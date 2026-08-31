//! The symbol table — every name in a program as a `u32`.
//!
//! Relation names, object names, rule names, variable names, keyword names,
//! and every `String` / `Range` / `Var` literal that reaches a fact argument
//! intern here
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §2). What
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
//!   ([design/08](../../../../docs/history/m1a_rust/design/08_parallelism.md) §1).
//! - **Interning is not belief.** This table says a name exists; whether a
//!   proposition built out of it holds is a per-KB bit
//!   ([`crate::facts`]).

use rustc_hash::FxHashMap;
use std::sync::OnceLock;

/// How many distinct ids one interned space can hold.
///
/// [`crate::Value`] packs a 2-bit tag beside a 30-bit payload, so a symbol, an
/// int-pool entry and a fact all share the same ceiling. The check exists so
/// that hitting it is an error somebody can read rather than a silent wrap
/// into another value's identity.
///
/// **This is one of two limits, and it is not the one that binds first.** A
/// symbol's text and a fact's overflow arguments live in arenas addressed by
/// `u32`, so [`ARENA_CAPACITY`] caps the *bytes* where this caps the *count* —
/// and they are independent, because nothing bounds a symbol's length. This
/// comment used to say reaching the id ceiling "needs ≥ 4 GB of symbol text",
/// which holds only at a mean symbol length of exactly 4 bytes; measured
/// through the `.einb` symbol-table header
/// (`ein kb save <file> x.einb`, 2026-09-01) the corpus runs 7.55 B/symbol on
/// `examples/branching/06_lookahead_on.ein` and 18.93 on `examples/zebra2.ein`,
/// where the arena fills at **21 %** of this ceiling. Both are guarded, at the
/// one site each arena can grow (M1e S1e.4.1, `CO-L1`).
pub const CAPACITY: u32 = 1 << 30;

/// How many slots one arena addressed by a `u32` offset can hold.
///
/// The [`Interner`]'s text (slots are **bytes**) and the
/// [`crate::facts::FactStore`]'s overflow argument vector (slots are
/// **[`crate::Value`]s**) are both indexed by a `u32` start, so an arena that
/// passed this would wrap a start into another value's identity — the silent
/// failure [`CAPACITY`]'s own comment says this module exists to prevent, in
/// the exact shape it warns about. Unreachable at corpus scale (`zebra2`'s
/// text arena is 3 692 bytes) and guarded anyway, because the alternative to a
/// readable error here is not a crash, it is a wrong answer.
pub const ARENA_CAPACITY: usize = 1 << 32;

/// Room in an arena for `more` slots, or the overflow that says which arena.
///
/// One helper for the two growth sites, so the bound is stated once. The
/// `checked_add` matters as much as the comparison: `len + more` is itself a
/// wrap on a host whose `usize` is 32 bits.
pub(crate) fn arena_room(len: usize, more: usize, which: Overflow) -> Result<(), Overflow> {
    match len.checked_add(more) {
        Some(end) if end <= ARENA_CAPACITY => Ok(()),
        _ => Err(which),
    }
}

/// Why an id could not be assigned.
///
/// Three of the seven are the 30-bit payload filling up and three more are an
/// arena or a field beside it. A puzzle that reaches one is a research
/// finding, not a crash — so it is a [`Result`] at every site that assigns an
/// id or writes an offset, and not a panic. **That was a claim rather than a
/// fact until M1e S1e.4.1** (`CO-L1`): [`SymbolText`](Overflow::SymbolText),
/// [`FactArgs`](Overflow::FactArgs) and [`FactArity`](Overflow::FactArity)
/// are the three the sentence above was already promising, and the last of
/// them was a live `expect` that took the process down with exit 101 on a
/// 65 536-argument fact.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overflow {
    Symbols,
    Ints,
    Facts,
    /// The text arena passed [`ARENA_CAPACITY`], so a span start would wrap.
    SymbolText,
    /// The fact store's overflow argument arena passed [`ARENA_CAPACITY`].
    FactArgs,
    /// A fact had more arguments than [`crate::facts::Row`]'s `u16` arity
    /// field can count.
    FactArity,
    /// **Not a capacity condition.** The intern tables are shared with a
    /// worker, so nobody may grow them — [`crate::Terms::share`].
    ///
    /// This is the whole of what a worker cannot do, and it is why
    /// [P1a.7](../../../../docs/history/m1a_rust/README.md#p1a7--parallelism) needs
    /// no lock on the interner or the fact store: the tables are read by `&`
    /// and grown only where nothing else holds them. An entering that hits it
    /// hands itself back and is re-run on the committing thread — which
    /// [shared_state.md §2a](../../../../docs/history/m1a_rust/measurements/shared_state.md#2a-and-a-total-is-the-wrong-shape-of-number-for-it)
    /// measured at **zero** enterings on four of six workloads and 7 of 111 on
    /// the worst, all of them in the head of a layer.
    Shared,
}

impl std::fmt::Display for Overflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self {
            Overflow::Symbols => "distinct symbols",
            Overflow::Ints => "distinct integer literals",
            Overflow::Facts => "distinct facts",
            Overflow::SymbolText => {
                return write!(
                    f,
                    "too much symbol text — the limit is {ARENA_CAPACITY} bytes"
                );
            }
            Overflow::FactArgs => {
                return write!(
                    f,
                    "too many stored fact arguments — the limit is {ARENA_CAPACITY}"
                );
            }
            Overflow::FactArity => {
                return write!(f, "a fact takes at most {} arguments", u16::MAX);
            }
            Overflow::Shared => {
                return f.write_str(
                    "the intern tables are shared with a worker and cannot \
                     grow — this entering has to be re-run on the committing \
                     thread",
                );
            }
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
/// One per process. `.einb`
/// ([design/10](../../../../docs/history/m1a_rust/design/10_binary_format.md) §3) is
/// the only thing that crosses interner boundaries, and it remaps on open.
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
        // The arena bound, and it is the one that binds first — see
        // [`CAPACITY`]. On the miss path only: the hit above returns before
        // it, and the symbol table is effectively frozen after load, so this
        // runs ~195 times on `zebra2` and never during a search.
        arena_room(self.arena.len(), s.len(), Overflow::SymbolText)?;
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
    /// ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §1),
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

    /// **The arena bound is the byte bound, and it is not the id bound.**
    ///
    /// A 4 GiB arena is not allocatable in a unit test, so the boundary is
    /// asserted where it is *computed* rather than where it is reached. The
    /// last assertion is the one that holds [`CAPACITY`]'s corrected comment:
    /// at the corpus's own mean symbol length the byte bound arrives first, so
    /// a future edit restoring *"reaching the id ceiling needs ≥ 4 GB of
    /// symbol text"* fails here rather than merely reading oddly. M1e
    /// S1e.4.1, `CO-L1`.
    #[test]
    fn the_arena_bound_is_the_byte_bound_and_not_the_id_bound() {
        assert_eq!(
            arena_room(ARENA_CAPACITY - 4, 4, Overflow::SymbolText),
            Ok(())
        );
        assert_eq!(
            arena_room(ARENA_CAPACITY - 4, 5, Overflow::SymbolText),
            Err(Overflow::SymbolText)
        );
        // The add itself, not only the comparison: a 32-bit `usize` wraps here.
        assert_eq!(
            arena_room(usize::MAX, 1, Overflow::FactArgs),
            Err(Overflow::FactArgs)
        );
        // 7.55 B/symbol is the corpus's *shortest* mean (`branching/06`), so
        // even there the arena fills at 53 % of the id ceiling — measured
        // 2026-09-01 through `ein kb save`'s symbol-table header.
        assert!(
            ARENA_CAPACITY / 7 < CAPACITY as usize,
            "the id ceiling would bind first, and the doc comment says it does not"
        );
    }
}
