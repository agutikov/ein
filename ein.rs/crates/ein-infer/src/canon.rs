//! Canonical state identity — the lattice search's dedup key.
//!
//! Two commitment-set branches that saturate to the same closed KB collapse to
//! one lattice node, which is what makes the search space a DAG rather than a
//! tree.
//!
//! **The guarantee: identity is the canonical representation
//! itself, never a hash of it.** [`state_key`] returns the sorted fact list and
//! every identity site keys on that list directly, so correctness does not
//! depend on hash quality — a collision costs a comparison, not a wrong answer.
//! [`state_digest`] exists only for display and is *never* identity.
//!
//! For **identity** any total order is equivalent
//! ([03-design/01-determinism-and-order.md § 5](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#5-where-order-does-not-matter)),
//! so this sorts by `FactId` — a `u32` sort and a `memcmp` equality instead of
//! building a string per fact. The `repr` order is needed only where the key
//! is *displayed* (`--dump-states` sorts nodes by it), and that is
//! [03-design/01-determinism-and-order.md § 4.3](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#43-output-bytes)'s.

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
    // determinism-ok: identity order — a canonical *key*, compared for
    // equality and never rendered. Any total order gives the same
    // equivalence classes; what the order has to be is a function of the
    // input, which it is, because ids are assigned at load and the search
    // assigns none (`docs/ein/ein.rs/03-design/01-determinism-and-order.md` § 2.1).
    // The dumper re-sorts by repr before printing — `dump::snapshot`'s
    // `repr_sorted`, and the paragraph above.
    out.sort_unstable();
    out.into_boxed_slice()
}

/// The first of each canonical state — a `dedup` for anything that carries a
/// [`Kb`].
///
/// **The one implementation of *same model***, and the reason it is one:
/// three sites deduplicated a model list by [`state_key`] independently — the
/// solution table counted `k` that way, `expect` compared claims that way
/// (its doc comment said *keyed the way `answer.rs` counts `k`*, which is a
/// parallel copy admitting to being one), and the key table filtered that way
/// inline. Parallel copies of one definition drift apart, and where the
/// copies are counts the drift is two surfaces disagreeing on `k`.
///
/// Quadratic in the number of items and linear in a KB's facts per item. Both
/// are `k`-sized here — the corpus's largest model set is 32 — so a linear
/// scan beats a map that would have to own its keys.
pub fn distinct_by_state<'a, T, F>(items: impl IntoIterator<Item = &'a T>, kb_of: F) -> Vec<&'a T>
where
    T: 'a,
    F: Fn(&'a T) -> &'a Kb,
{
    let mut keys: Vec<Box<[FactId]>> = Vec::new();
    let mut out: Vec<&'a T> = Vec::new();
    for it in items {
        let key = state_key(kb_of(it));
        // determinism-ok: membership only — a linear scan over keys that are
        // themselves canonical, so nothing about the order of `keys` reaches
        // the output.
        if keys.contains(&key) {
            continue;
        }
        keys.push(key);
        out.push(it);
    }
    out
}

/// A hash of a [`state_key`] — **display and logging only**.
///
/// Never use a digest as identity: distinct states may share one
/// ([03-design/01-determinism-and-order.md § 8](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#8-hashes-that-reach-the-output)).
pub fn state_digest(key: &[FactId]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = rustc_hash::FxHasher::default();
    key.hash(&mut h);
    h.finish()
}
