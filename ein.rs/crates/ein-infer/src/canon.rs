//! Canonical state identity — the lattice search's dedup key.
//!
//! Two commitment-set branches that saturate to the same closed KB collapse to
//! one lattice node, which is what makes the search space a DAG rather than a
//! tree.
//!
//! **The guarantee (P1.21 R1): identity is the canonical representation
//! itself, never a hash of it.** [`state_key`] returns the sorted fact list and
//! every identity site keys on that list directly, so correctness does not
//! depend on hash quality — a collision costs a comparison, not a wrong answer.
//! [`state_digest`] exists only for display and is *never* identity.
//!
//! ein.py sorts with `key=repr`, because a `FactId` tuple whose args mix `str`
//! and `int` is not orderable and `repr` is a total order over anything. For
//! **identity** any total order is equivalent
//! ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §6),
//! so this sorts by `FactId` — a `u32` sort and a `memcmp` equality instead of
//! building a string per fact. The `repr` order is still needed where the key
//! is *displayed* (`--dump-states` sorts nodes by it), and that is
//! [P1a.5](../../../../docs/history/m1a_rust/README.md#p1a5--presentation-and-cli)'s.

use ein_core::{FactId, Kb};

/// The order-insensitive canonical key of a KB's propositional facts.
///
/// Keys **only** the facts: the ontology, the rules and the query are constant
/// across branches, and the per-branch trace is deliberately the *dedup
/// target* — different proof paths reaching the same closed KB should collapse
/// to one node. Provenance is excluded for the same reason fact identity
/// excludes it everywhere else in the KB.
///
/// Argument order *inside* a fact is preserved — `(right-of A B)` and
/// `(right-of B A)` are different facts — because only the outer list is
/// sorted.
pub fn state_key(kb: &Kb) -> Box<[FactId]> {
    let mut out: Vec<FactId> = kb.facts().collect();
    // determinism-ok: identity order, and this is class A of the T1a.7.1.5
    // audit — a canonical *key*, compared for equality and never rendered.
    // Any total order gives the same equivalence classes (design/02 §6); what
    // the order has to be is a function of the input, which it is, because
    // ids are assigned at load and the search assigns none (T1a.7.1.1/.2).
    // The dumper re-sorts by repr before printing — `dump::snapshot`'s
    // `repr_sorted`, and the paragraph above.
    out.sort_unstable();
    out.into_boxed_slice()
}

/// A hash of a [`state_key`] — **display and logging only**.
///
/// Never use a digest as identity: distinct states may share one. ein.py's is
/// CPython's `hash(tuple)`, which is not stable across runs, so the two
/// implementations do not agree on the number and are not asked to
/// ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §8).
pub fn state_digest(key: &[FactId]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = rustc_hash::FxHasher::default();
    key.hash(&mut h);
    h.finish()
}
