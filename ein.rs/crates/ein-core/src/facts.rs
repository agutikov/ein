//! The fact store — interned rows, and the number every proposition gets.
//!
//! ein.py's `Fact` is `(relation_name: str, args: tuple[str | int | Fact,
//! ...])` with identity on exactly those two fields, which makes every
//! identity comparison a tuple compare recursing into `str.__eq__` and every
//! hash a tuple hash over string hashes. Here a proposition is a `u32`
//! ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §4):
//!
//! | ein.py | ein.rs |
//! |---|---|
//! | `Fact.__eq__` → tuple compare → per-arg `str.__eq__` | `FactId == FactId` |
//! | `Fact.__hash__` → tuple hash | the `FactId` *is* the hash |
//! | `kb._fact_by_id(rel, args)` — an O(deg) scan of the extent | [`FactStore::probe`], O(1) |
//! | a nested `Fact` arg is an unregistered object | a nested arg is a `FactId` like any other |
//!
//! **Interning is not belief.** [`FactStore::intern`] says "this proposition
//! has this number"; it does not say the proposition holds. Belief is a
//! per-KB bit, which is what lets a hypothesis fork intern freely without the
//! parent ever seeing the proposition — and what makes an O(1) fork correct
//! rather than merely cheap.

use crate::intern::{CAPACITY, Overflow, Symbol};
use crate::value::Value;
use rustc_hash::FxHasher;
use std::hash::Hasher;

/// A proposition's number. Dense, so it indexes a `Vec` or a bitset directly.
///
/// `Ord` is derived and is **assignment order** — first-interned first. That
/// is a legitimate sort key only where any total order is equivalent (a
/// `state_key`'s sorted vector, a no-good clause's canonical form —
/// [design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §6),
/// never where a reader sees the result.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FactId(pub u32);

/// One row: which relation, and the arguments themselves when there are at
/// most [`INLINE_ARGS`] of them.
///
/// 20 bytes, and reading a fact's arguments is **one** load rather than two
/// dependent ones — which is what T1a.6.2.6 turned out to be about. The store
/// is 18 KB for an exhaustive `zebra` and lives in L1 entirely, so this was
/// never a cache-footprint question ([baseline.md
/// §13](../../../../docs/history/m1a_rust/measurements/baseline.md)); it is a
/// question about the *dependency chain* on the matcher's hot path, where
/// every candidate resolves two argument lists — the premise's and, for a
/// nested pattern, the fact inside it — before it can be rejected.
///
/// A fact of arity > [`INLINE_ARGS`] keeps its arguments in
/// [`FactStore::args`] as before. Two covers **96.7 %** of an exhaustive
/// `zebra`'s facts and **83.5 %** of `zebra2`'s, measured rather than guessed
/// (`examples/layout_shape.rs`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Row {
    pub rel: Symbol,
    /// Where the arguments start in the arena — meaningful only when
    /// `arity > INLINE_ARGS`.
    args_at: u32,
    arity: u16,
    _pad: u16,
    /// The arguments, for the common case. Junk past `arity`.
    inline: [Value; INLINE_ARGS],
}

/// How many arguments a row holds without reaching for the arena.
///
/// The arity histogram is bimodal at 1 and 2 — `(not <fact>)` and every
/// binary relation — and the tail past 2 is 3.3 % of `zebra` and 16.5 % of
/// `zebra2`. Three would buy another 1.1 % of `zebra` for 4 more bytes a row;
/// two is where the mass is.
pub const INLINE_ARGS: usize = 2;

/// Rows, a flat argument arena, and the lookup that makes interning
/// injective.
#[derive(Default, Debug)]
pub struct FactStore {
    rows: Vec<Row>,
    args: Vec<Value>,
    /// Open addressing over `FactId + 1` (`0` is the empty slot), sized to a
    /// power of two, linear probing, no deletions — because the store is
    /// append-only, which is the same property the layered KB rests on.
    ///
    /// Hand-rolled rather than an `FxHashMap<(Symbol, Box<[Value]>), FactId>`
    /// for one reason: that map's keys would hold a second copy of every
    /// argument list, roughly doubling the store, and the key it wants to
    /// hold — a slice of [`FactStore::args`] — cannot be spelled without
    /// `unsafe`. Probing compares against the arena instead, so the store
    /// holds exactly one copy of everything.
    table: Vec<u32>,
}

impl FactStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// The number this `(rel, args)` proposition already has, if any.
    ///
    /// Computes the key without materialising a row, which is what lets the
    /// hypothesis generator reject a candidate before creating anything
    /// ([design/07](../../../../docs/history/m1a_rust/design/07_search_layer.md) §2).
    pub fn probe(&self, rel: Symbol, args: &[Value]) -> Option<FactId> {
        crate::counters::bump(|c| c.fact_probe += 1);
        if self.table.is_empty() {
            return None;
        }
        self.find(hash_row(rel, args), rel, args).ok()
    }

    /// The number this `(rel, args)` proposition has, assigning one if it is
    /// new.
    pub fn intern(&mut self, rel: Symbol, args: &[Value]) -> Result<FactId, Overflow> {
        crate::counters::bump(|c| c.fact_intern += 1);
        if (self.rows.len() + 1) * 4 >= self.table.len() * 3 {
            self.grow();
        }
        let hash = hash_row(rel, args);
        let slot = match self.find(hash, rel, args) {
            Ok(existing) => return Ok(existing),
            Err(slot) => slot,
        };
        if self.rows.len() as u32 >= CAPACITY {
            return Err(Overflow::Facts);
        }
        crate::counters::bump(|c| c.fact_new += 1);
        let id = FactId(self.rows.len() as u32);
        let mut row = Row {
            rel,
            args_at: self.args.len() as u32,
            arity: u16::try_from(args.len()).expect("a fact's arity fits u16"),
            _pad: 0,
            inline: [Value::UNBOUND; INLINE_ARGS],
        };
        if args.len() <= INLINE_ARGS {
            row.inline[..args.len()].copy_from_slice(args);
        } else {
            self.args.extend_from_slice(args);
        }
        self.rows.push(row);
        self.table[slot] = id.0 + 1;
        Ok(id)
    }

    pub fn rel(&self, id: FactId) -> Symbol {
        crate::counters::bump(|c| c.fact_read += 1);
        self.rows[id.0 as usize].rel
    }

    pub fn args(&self, id: FactId) -> &[Value] {
        crate::counters::bump(|c| c.fact_read += 1);
        self.args_of(&self.rows[id.0 as usize])
    }

    pub fn arity(&self, id: FactId) -> usize {
        self.rows[id.0 as usize].arity as usize
    }

    pub fn get(&self, id: FactId) -> (Symbol, &[Value]) {
        crate::counters::bump(|c| c.fact_read += 1);
        let row = &self.rows[id.0 as usize];
        (row.rel, self.args_of(row))
    }

    /// The row itself, for a caller that reads the relation, decides, and only
    /// then wants the arguments.
    ///
    /// The matcher's nested-pattern step is that caller and it is 25 M calls
    /// on an exhaustive `zebra`: `get` would resolve an argument list that
    /// **79 %** of an exhaustive `zebra2`'s candidates never read, and asking
    /// [`FactStore::rel`] first and [`FactStore::args`] second loads the row
    /// twice — measured at −8.2 % on one puzzle and +0 % on the other, each
    /// way round (T1a.6.2.2). One load, then the branch.
    pub fn row(&self, id: FactId) -> &Row {
        crate::counters::bump(|c| c.fact_read += 1);
        &self.rows[id.0 as usize]
    }

    /// A row's arguments — inline when it has at most [`INLINE_ARGS`] of them.
    pub fn args_of<'a>(&'a self, row: &'a Row) -> &'a [Value] {
        let arity = row.arity as usize;
        if arity <= INLINE_ARGS {
            &row.inline[..arity]
        } else {
            let at = row.args_at as usize;
            &self.args[at..at + arity]
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Bytes held by the two arenas — the figure design/03 §4 budgets
    /// (~8 KB for a saturated `zebra2`). Excludes the lookup table, which is
    /// an index over them rather than part of the store's contents.
    pub fn footprint(&self) -> usize {
        self.rows.len() * size_of::<Row>() + self.args.len() * size_of::<Value>()
    }

    /// How many facts hold their arguments inline — the acceptance number for
    /// [`INLINE_ARGS`], so the choice stays a measurement.
    pub fn inline_share(&self) -> (usize, usize) {
        (
            self.rows
                .iter()
                .filter(|r| r.arity as usize <= INLINE_ARGS)
                .count(),
            self.rows.len(),
        )
    }

    /// `Ok(id)` when the proposition is present; `Err(slot)` names the empty
    /// slot it would go in.
    fn find(&self, hash: u64, rel: Symbol, args: &[Value]) -> Result<FactId, usize> {
        let mask = self.table.len() - 1;
        let mut slot = (hash as usize) & mask;
        loop {
            match self.table[slot] {
                0 => return Err(slot),
                entry => {
                    let id = FactId(entry - 1);
                    if self.rel(id) == rel && self.args(id) == args {
                        return Ok(id);
                    }
                }
            }
            slot = (slot + 1) & mask;
        }
    }

    fn grow(&mut self) {
        let capacity = (self.table.len() * 2).max(64);
        self.table = vec![0; capacity];
        let mask = capacity - 1;
        for i in 0..self.rows.len() {
            let id = FactId(i as u32);
            let mut slot = (hash_row(self.rel(id), self.args(id)) as usize) & mask;
            while self.table[slot] != 0 {
                slot = (slot + 1) & mask;
            }
            self.table[slot] = id.0 + 1;
        }
    }
}

fn hash_row(rel: Symbol, args: &[Value]) -> u64 {
    let mut h = FxHasher::default();
    h.write_u32(rel.0);
    h.write_u32(args.len() as u32);
    for a in args {
        h.write_u32(a.bits());
    }
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intern::Interner;
    use crate::value::{IntPool, Value};

    fn v(n: u32) -> Value {
        Value::sym(Symbol(n))
    }

    #[test]
    fn a_row_is_twenty_bytes_and_holds_two_arguments() {
        assert_eq!(size_of::<Row>(), 20);
        // The claim the size is spent on: a binary fact's arguments come back
        // without touching the arena at all.
        let mut s = FactStore::new();
        let rel = Symbol(0);
        let binary = s.intern(rel, &[v(1), v(2)]).expect("room");
        let wide = s.intern(rel, &[v(1), v(2), v(3)]).expect("room");
        assert_eq!(s.args(binary), &[v(1), v(2)]);
        assert_eq!(s.args(wide), &[v(1), v(2), v(3)]);
        assert_eq!(s.args.len(), 3, "only the wide fact reached the arena");
        assert_eq!(s.inline_share(), (1, 2));
    }

    #[test]
    fn interning_is_injective_and_stable() {
        let mut s = FactStore::new();
        let rel = Symbol(0);
        let a = s.intern(rel, &[v(1), v(2)]).expect("room");
        let b = s.intern(rel, &[v(2), v(1)]).expect("room");
        assert_ne!(a, b, "argument order is part of identity");
        assert_eq!(s.intern(rel, &[v(1), v(2)]).expect("room"), a);
        assert_eq!(s.probe(rel, &[v(1), v(2)]), Some(a));
        assert_eq!(s.probe(rel, &[v(1), v(3)]), None);
        assert_eq!(s.probe(Symbol(1), &[v(1), v(2)]), None);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn arity_is_part_of_identity() {
        let mut s = FactStore::new();
        let rel = Symbol(0);
        let unary = s.intern(rel, &[v(1)]).expect("room");
        let binary = s.intern(rel, &[v(1), v(2)]).expect("room");
        let nullary = s.intern(rel, &[]).expect("room");
        assert_ne!(unary, binary);
        assert_ne!(unary, nullary);
        assert_eq!(s.args(nullary), &[] as &[Value]);
        assert_eq!(s.args(unary), &[v(1)]);
    }

    #[test]
    fn a_probe_on_an_empty_store_does_not_divide_by_the_mask() {
        let s = FactStore::new();
        assert_eq!(s.probe(Symbol(0), &[v(1)]), None);
    }

    #[test]
    fn nested_identity_cascades_the_way_fact_eq_does() {
        let mut s = FactStore::new();
        let (co_located, not) = (Symbol(0), Symbol(1));
        let inner = s.intern(co_located, &[v(10), v(11)]).expect("room");
        let outer = s.intern(not, &[Value::fact(inner)]).expect("room");
        // Re-deriving the inner proposition from scratch must reach the same
        // outer fact — this is `Fact.__eq__` recursing into nested facts.
        let inner_again = s.intern(co_located, &[v(10), v(11)]).expect("room");
        assert_eq!(inner_again, inner);
        assert_eq!(s.probe(not, &[Value::fact(inner_again)]), Some(outer));
        // A *different* inner proposition is a different outer one.
        let other = s.intern(co_located, &[v(10), v(12)]).expect("room");
        assert_eq!(s.probe(not, &[Value::fact(other)]), None);
    }

    #[test]
    fn interning_a_nested_fact_says_nothing_about_belief() {
        // ein.py's nested `Fact` args are "unregistered" objects — they are
        // not in `kb.facts`. Here they get an id like any other proposition,
        // and the store holds no belief state at all to distinguish them:
        // that bit lives in the KB, which is what makes an O(1) fork sound.
        let mut s = FactStore::new();
        let inner = s.intern(Symbol(0), &[v(1)]).expect("room");
        let outer = s.intern(Symbol(1), &[Value::fact(inner)]).expect("room");
        assert_eq!(s.len(), 2);
        assert!(inner != outer);
        assert_eq!(s.args(outer), &[Value::fact(inner)]);
    }

    #[test]
    fn growth_preserves_every_identity() {
        let mut s = FactStore::new();
        let rel = Symbol(0);
        let ids: Vec<_> = (0..1000)
            .map(|i| s.intern(rel, &[v(i), v(i * 2)]).expect("room"))
            .collect();
        assert_eq!(s.len(), 1000);
        for (i, id) in ids.iter().enumerate() {
            let i = i as u32;
            assert_eq!(s.probe(rel, &[v(i), v(i * 2)]), Some(*id));
            assert_eq!(s.args(*id), &[v(i), v(i * 2)]);
        }
    }

    #[test]
    fn a_zebra_sized_store_fits_the_memory_budget() {
        // design/03 §4: 381 facts of mean arity ≈ 2.2 is ~8 KB, contiguous,
        // against ~60–80 KB of scattered Python object graph.
        let mut interner = Interner::new();
        let mut ints = IntPool::new();
        let mut s = FactStore::new();
        let rel = interner.intern("co-located").expect("room");
        for i in 0..381u32 {
            let a = Value::sym(interner.intern(&format!("House-{i}")).expect("room"));
            let b = Value::int(ints.intern(&i.to_string()).expect("room"));
            // 2 args for four facts in five, 3 for the fifth — mean 2.2.
            if i % 5 == 4 {
                s.intern(rel, &[a, b, a]).expect("room");
            } else {
                s.intern(rel, &[a, b]).expect("room");
            }
        }
        assert_eq!(s.len(), 381);
        assert!(
            s.footprint() <= 10_000,
            "381 facts + args took {} bytes",
            s.footprint()
        );
    }
}
