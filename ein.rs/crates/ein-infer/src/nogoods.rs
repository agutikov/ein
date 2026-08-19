//! Path-condition no-good clause learning — the CDCL analogue.
//!
//! Every dead commitment's path condition becomes a learned clause on the
//! root. A prospective commitment set that is a superset of one is filtered
//! **pre-fork** by [`crate::apriori::filter_candidate`] — no fork, no
//! saturation, no clause of its own.
//!
//! **Subsumption keeps the store minimal on emit.** A new clause is dropped if
//! any stored clause is a subset of it (the stored one is stronger and the new
//! one adds nothing); otherwise every stored strict superset is removed and
//! the new clause goes in.
//!
//! `min_size` defaults to **2**, preserving the split where size-1 clauses are
//! the singleton-death `(not h)` writeback's domain. The set-indexed engines
//! pass **1**, because `filter_candidate`'s subset check runs against the
//! store before `alive` is recomputed, so a layer-1 singleton death has to
//! land here to prune within the same layer.
//!
//! ### Storage
//!
//! A clause is a `Box<[FactId]>` sorted by **id**, not by content. ein.py's is
//! a `frozenset`, so the only questions asked of it are membership and subset
//! — and for those any total order does, which is why this one does not need
//! [`ein_core::Terms::cmp_fact_semantic`]. The one place order escapes is the
//! `nogood` event, and that sorts the rendered s-expressions.
//!
//! design/07 §4 also specifies a `u64` bitmask fast path for an alive set of
//! ≤ 64. It is **not here**: its stated payoff is the
//! `enable_singleton_writeback=false` regime, where exhaustive `zebra2` grows
//! from 101 enterings to 3 336+, and nothing can run an exhaustive solve until
//! [S1a.4.5](../../../../plans/m1a_rust/p1a.4_search_layer/s1a.4.5_solve_loop.md).
//! Landing a second representation before the measurement that justifies it is
//! the mistake Win B already made once
//! ([Q-M1a.17](../../../../plans/m1a_rust/open_questions.md)), so it carries
//! its trigger condition into
//! [P1a.6](../../../../plans/m1a_rust/p1a.6_performance/README.md) instead.
//!
//! **The trigger was pulled and the answer was no**
//! ([S1a.6.4](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)
//! T1a.6.4.4). That regime is 3 831 enterings and 2.38 s — and **354 clauses**,
//! because the subsumption above is what keeps the two numbers apart. The whole
//! apriori/no-good machinery is **0.3 %** of it (`filter_candidate` 0.3 %,
//! `is_subset` and this module 0.0 %) against 60.2 % for the NAF boundary. So
//! the sorted `Box<[FactId]>` is the only representation, and carrying the
//! trigger rather than the code was the right call twice over.

use ein_core::{FactId, Kb, Terms};

use crate::apriori::is_subset;
use crate::events::Events;

/// Insert `clause` with subsumption; true iff a new clause was added.
///
/// Gated by `enable_path_nogoods` at the call site: with it off the store
/// stays empty and `filter_candidate`'s subset check is a no-op, so subsumed
/// dead commitments are re-explored — a different entering count, which is why
/// the lever is a T1 observable rather than a tuning knob.
pub fn emit_nogood(
    kb: &Kb,
    terms: &Terms,
    events: &mut Events,
    clause: &[FactId],
    min_size: usize,
) -> bool {
    let mut sorted: Vec<FactId> = clause.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() < min_size {
        return false;
    }
    let store = kb.nogoods();
    let mut store = store.write().expect("the no-good store");

    // Subsumed by an existing stronger clause?
    if store.iter().any(|c| is_subset(c, &sorted)) {
        if events.on() {
            let rendered = clause_repr(terms, &sorted);
            events.emit("nogood", |l| {
                l.owned_strs("clause", rendered);
                l.bool("emitted", false);
                l.bool("subsumed", true);
            });
        }
        return false;
    }

    // Remove the ones this clause subsumes.
    let doomed: Vec<Box<[FactId]>> = store
        .iter()
        .filter(|c| is_subset(&sorted, c))
        .map(|c| c.to_vec().into_boxed_slice())
        .collect();
    let removed = doomed.len();
    for c in doomed {
        store.remove(&c);
    }
    store.insert(sorted.clone().into_boxed_slice());
    if events.on() {
        let rendered = clause_repr(terms, &sorted);
        events.emit("nogood", |l| {
            l.owned_strs("clause", rendered);
            l.bool("emitted", true);
            l.bool("subsumed", false);
            l.num("removed", removed as i64);
        });
    }
    true
}

/// A clause as **sorted** s-expressions — it is a set, so any order but a
/// sorted one would leak the store's iteration into the event stream.
pub fn clause_repr(terms: &Terms, clause: &[FactId]) -> Vec<String> {
    let mut out: Vec<String> = clause
        .iter()
        .map(|&f| crate::events::sexpr(terms, f))
        .collect();
    out.sort();
    out
}
