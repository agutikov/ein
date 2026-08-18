//! The symbol table — every name in a program as a `u32`.
//!
//! Relation names, object names, rule names, variable names, keyword names,
//! and every `String` / `Range` / `Var` literal that reaches a fact argument
//! intern here
//! ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §2). What
//! ein.py holds as a `str` object per occurrence — 50 B each, shared only when
//! CPython happened to intern it, which for a `"House-1"` read from a file it
//! has not — becomes a 4-byte [`Symbol`], and every identity comparison
//! becomes an integer compare.
//!
//! Two rules the rest of the port depends on:
//!
//! - **Ids are assignment-ordered.** [`Symbol`] deliberately has no `Ord`:
//!   its numeric order is first-seen order, and using it as a sort key would
//!   make the output depend on the order the *loader* happened to walk the
//!   file. Observable sorts go through [`Interner::rank`]
//!   ([design/08](../../../../plans/m1a_rust/design/08_parallelism.md) §1).
//! - **Interning is not belief.** This table says a name exists; whether a
//!   proposition built out of it holds is a per-KB bit
//!   ([`crate::facts`]).

use rustc_hash::FxHashMap;
use std::sync::OnceLock;

/// How many distinct ids one interned space can hold.
///
/// [`crate::Value`] packs a 2-bit tag beside a 30-bit payload, so a symbol, an
/// int-pool entry and a fact all share the same ceiling. Reaching it needs
/// ≥ 4 GB of symbol text; the check exists so that hitting it is an error
/// somebody can read rather than a silent wrap into another value's identity.
pub const CAPACITY: u32 = 1 << 30;

/// The 30-bit payload is full.
///
/// A puzzle that reaches this is a research finding, not a crash — so it is a
/// [`Result`] at the three sites that assign ids, and not a panic.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overflow {
    Symbols,
    Ints,
    Facts,
}

impl std::fmt::Display for Overflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            Overflow::Symbols => "distinct symbols",
            Overflow::Ints => "distinct integer literals",
            Overflow::Facts => "distinct facts",
        };
        write!(f, "too many {what} — the limit is {CAPACITY}")
    }
}

impl std::error::Error for Overflow {}

/// An interned string. 4 bytes, `Copy`, and **not** `Ord` — see the module
/// docs.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Symbol(pub u32);

/// Text arena + span table + lookup, with a lazily-built lexicographic rank.
///
/// One per process (per `Engine` in the server —
/// [design/09](../../../../plans/m1a_rust/design/09_server_mode.md) § Interner
/// scope).
#[derive(Default, Debug)]
pub struct Interner {
    /// All text, one allocation family.
    arena: String,
    /// `Symbol` → `(start, len)` in [`Interner::arena`].
    spans: Vec<(u32, u32)>,
    /// The reverse lookup. It owns a *second* copy of each string, which
    /// design/03 §2 does not: the map it sketches is keyed by `&'arena str`,
    /// and a self-referential borrow needs `unsafe`, which this crate forbids.
    /// The duplicate is ~1.5 KB on `zebra2` and the alternative is a
    /// hand-rolled table for no measurable gain — [`crate::facts`] pays that
    /// price where it buys something.
    lookup: FxHashMap<Box<str>, Symbol>,
    /// `Symbol` → position in the lexicographically sorted symbol list.
    /// Rebuilt on first use after any growth; see [`Interner::rank`].
    rank: OnceLock<Vec<u32>>,
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Assign `s` a [`Symbol`], or return the one it already has.
    pub fn intern(&mut self, s: &str) -> Result<Symbol, Overflow> {
        if let Some(&id) = self.lookup.get(s) {
            return Ok(id);
        }
        if self.spans.len() as u32 >= CAPACITY {
            return Err(Overflow::Symbols);
        }
        let start = self.arena.len() as u32;
        self.arena.push_str(s);
        let id = Symbol(self.spans.len() as u32);
        self.spans.push((start, s.len() as u32));
        self.lookup.insert(s.into(), id);
        // The symbol list grew, so the rank table describes a shorter list
        // than the one that exists. Drop it; the next sort rebuilds it.
        self.rank.take();
        Ok(id)
    }

    /// The [`Symbol`] `s` already has, if any. Interning is a `&mut` operation
    /// and most read paths do not have one.
    pub fn get(&self, s: &str) -> Option<Symbol> {
        self.lookup.get(s).copied()
    }

    /// The text behind a symbol.
    pub fn text(&self, sym: Symbol) -> &str {
        let (start, len) = self.spans[sym.0 as usize];
        &self.arena[start as usize..(start + len) as usize]
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    /// `sym`'s position in the lexicographically sorted symbol list.
    ///
    /// This is what an observable sort by name uses: ein.py's `sorted(names)`
    /// compares Unicode code points and Rust's `Ord for str` compares UTF-8
    /// bytes, which agree for all inputs
    /// ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §1),
    /// so ranking once turns every later name-sort into a `u32` sort.
    ///
    /// Cheap to maintain because the symbol table is effectively frozen after
    /// load: rules cannot fabricate atoms, so saturation and search add facts
    /// but no symbols. On `zebra2` that is one sort of ~150 strings, once.
    pub fn rank(&self, sym: Symbol) -> u32 {
        self.ranks()[sym.0 as usize]
    }

    /// The whole rank table, built on demand.
    pub fn ranks(&self) -> &[u32] {
        self.rank.get_or_init(|| {
            let mut order: Vec<u32> = (0..self.spans.len() as u32).collect();
            order.sort_by(|&a, &b| self.text(Symbol(a)).cmp(self.text(Symbol(b))));
            let mut rank = vec![0u32; order.len()];
            for (position, sym) in order.into_iter().enumerate() {
                rank[sym as usize] = position as u32;
            }
            rank
        })
    }

    /// Compare two symbols the way `sorted()` would compare their names.
    pub fn cmp_text(&self, a: Symbol, b: Symbol) -> std::cmp::Ordering {
        self.rank(a).cmp(&self.rank(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interning_is_injective_and_stable() {
        let mut i = Interner::new();
        let a = i.intern("House-1").expect("room");
        let b = i.intern("House-2").expect("room");
        assert_ne!(a, b);
        assert_eq!(i.intern("House-1").expect("room"), a);
        assert_eq!(i.text(a), "House-1");
        assert_eq!(i.text(b), "House-2");
        assert_eq!(i.get("House-2"), Some(b));
        assert_eq!(i.get("House-3"), None);
    }

    #[test]
    fn the_arena_holds_one_copy_and_spans_index_it() {
        let mut i = Interner::new();
        for s in ["a", "bb", "ccc"] {
            i.intern(s).expect("room");
        }
        assert_eq!(i.arena, "abbccc");
        assert_eq!(i.len(), 3);
    }

    #[test]
    fn rank_is_lexicographic_while_ids_are_first_seen() {
        let mut i = Interner::new();
        let zebra = i.intern("zebra").expect("room");
        let apple = i.intern("apple").expect("room");
        // Assignment order says zebra first; the rank table says otherwise,
        // and the rank table is the one an observable sort may use.
        assert!(zebra.0 < apple.0);
        assert_eq!(i.rank(apple), 0);
        assert_eq!(i.rank(zebra), 1);
        assert_eq!(i.cmp_text(apple, zebra), std::cmp::Ordering::Less);
    }

    #[test]
    fn growth_invalidates_the_rank_table() {
        let mut i = Interner::new();
        let b = i.intern("b").expect("room");
        assert_eq!(i.rank(b), 0);
        let a = i.intern("a").expect("room");
        // Were the table stale, `b` would still rank 0 and `a` would index
        // past the end.
        assert_eq!(i.rank(a), 0);
        assert_eq!(i.rank(b), 1);
    }

    #[test]
    fn rank_orders_by_code_point_not_by_locale() {
        let mut i = Interner::new();
        let mut syms = Vec::new();
        // Deliberately mixed: ASCII case, a combining sequence, non-BMP.
        for s in ["Z", "a", "A", "z", "Åsa", "Zebra", "😀", "é", "e\u{301}"] {
            syms.push((s, i.intern(s).expect("room")));
        }
        let mut by_rank = syms.clone();
        by_rank.sort_by_key(|&(_, sym)| i.rank(sym));
        let mut by_text: Vec<_> = syms.iter().map(|&(s, _)| s).collect();
        by_text.sort();
        assert_eq!(by_rank.iter().map(|&(s, _)| s).collect::<Vec<_>>(), by_text);
    }
}
