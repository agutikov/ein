//! The lattice's arithmetic — layer *k+1* candidates from the layer-*k*
//! frontier.
//!
//! The textbook Apriori prefix-join, filtered against the alive set and the
//! learned no-good clauses (the downward-closure prune). Pure set arithmetic:
//! no KB inspection and no saturation, which makes it the easiest part of the
//! search layer to port and the easiest to get subtly wrong in *ordering*.
//!
//! ### The comparator is the whole risk
//!
//! ein.py sorts `(relation_name, args)` tuples — by **content**. `FactId`
//! order here is interning order, which is an artefact of what the loader
//! happened to see first, so sorting by it would produce a different layer-1
//! order and therefore a different traversal on every puzzle.
//! [`Terms::cmp_fact_semantic`] is the comparator that agrees with Python's,
//! and it exists for this call site
//! ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §3b).
//!
//! Where the two *cannot* agree is a mixed-type argument: Python raises
//! `TypeError` comparing a `str` to an `int`, and `Value` is totally ordered
//! by construction. That is [D2](../../../../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
//! with `examples/ein-bugs/mixed-type-hypothesis.ein` pinning both halves.
//!
//! ### Where no-goods are *not* consulted
//!
//! [`filter_candidate`] runs at layer-generation time, so a clause emitted
//! part-way through a layer does not prune the rest of that layer. The
//! asymmetry is deliberate and load-bearing downstream: it is what makes
//! [design/08](../../../../plans/m1a_rust/design/08_parallelism.md) §2's case 1
//! free.

use ein_core::{FactId, Kb, Terms};
use rustc_hash::FxHashSet;

use crate::hypgen::ScoreError;

/// A canonically-ordered, deduped set of `FactId`s — the lattice's node
/// identity.
pub type CanonicalSetId = Vec<FactId>;

/// Sort + dedup into a [`CanonicalSetId`].
pub fn canonicalise(terms: &Terms, elements: impl IntoIterator<Item = FactId>) -> CanonicalSetId {
    let mut out: Vec<FactId> = elements.into_iter().collect();
    out.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));
    // ein.py is `tuple(sorted(set(...)))`, so the dedup is by *identity* and
    // the sort by content; interning makes those the same question, and
    // `dedup` after the sort is the same answer.
    out.dedup();
    out
}

/// `tuple` ordering over two canonical sets: element-wise, then by length.
pub fn cmp_set(terms: &Terms, a: &[FactId], b: &[FactId]) -> std::cmp::Ordering {
    for (&x, &y) in a.iter().zip(b.iter()) {
        let ord = terms.cmp_fact_semantic(x, y);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}

/// Textbook Apriori-gen prefix-join: for each pair `(s, t)` agreeing on their
/// first `|s|-1` elements with `s[-1] < t[-1]`, emit `s[:-1] + (s[-1], t[-1])`.
///
/// Each layer-(k+1) candidate of size `|s|+1` is yielded exactly once.
///
/// The **break** on the first prefix mismatch is a cost win and *only* a cost
/// win, which is worth saying because
/// [S1a.4.3](../../../../plans/m1a_rust/p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md)
/// calls it "load-bearing for both cost and order". It cannot move the order:
/// every set in a layer has the same size, so sorting by the full tuple sorts
/// by the prefix as its primary key, and once a prefix differs no later entry
/// can match it again. Replacing the `break` with a `continue` produces
/// byte-identical output on all 65 files — checked, not assumed. It is kept
/// because ein.py has it and because the scan is quadratic without it.
pub fn apriori_prefix_join(terms: &Terms, a_prev: &[CanonicalSetId]) -> Vec<CanonicalSetId> {
    let mut sorted_prev: Vec<&CanonicalSetId> = a_prev.iter().collect();
    sorted_prev.sort_by(|a, b| cmp_set(terms, a, b));
    let mut out = Vec::new();
    for (i, s) in sorted_prev.iter().enumerate() {
        if s.is_empty() {
            continue;
        }
        let prefix = &s[..s.len() - 1];
        for t in &sorted_prev[i + 1..] {
            if t.is_empty() || &t[..t.len() - 1] != prefix {
                break;
            }
            let (s_last, t_last) = (s[s.len() - 1], t[t.len() - 1]);
            if terms.cmp_fact_semantic(s_last, t_last) == std::cmp::Ordering::Less {
                let mut c = prefix.to_vec();
                c.push(s_last);
                c.push(t_last);
                out.push(c);
            }
        }
    }
    out
}

/// True iff `candidate` should be explored.
///
/// Dropped when any element has left `alive` — which covers the single-element
/// negatives the singleton-death writeback wrote since `a_prev` was computed —
/// or when any learned clause is a subset of it, which covers the multi-element
/// conditional deaths whose clauses propagated up from earlier layers.
///
/// The "every (k−1)-subset ∈ `a_prev`" condition holds by
/// [`apriori_prefix_join`]'s construction and is deliberately not re-verified.
pub fn filter_candidate(
    candidate: &[FactId],
    alive: &FxHashSet<FactId>,
    nogoods: &ein_core::Nogoods,
) -> bool {
    if !candidate.iter().all(|h| alive.contains(h)) {
        return false;
    }
    let mut set: Vec<FactId> = candidate.to_vec();
    // determinism-ok: identity order as `is_subset`'s precondition — the
    // predicate is a set question, so the order decides nothing and only has
    // to agree with the one the clauses were normalised under (`nogoods`).
    set.sort_unstable();
    !nogoods.iter().any(|clause| is_subset(clause, &set))
}

/// `clause ⊆ set`, both sorted by `FactId` — a merge walk rather than a hash
/// set per candidate. Storage order is free here because this is a *subset*
/// question, not an ordering one (design/07 §6).
pub fn is_subset(clause: &[FactId], set: &[FactId]) -> bool {
    let mut it = set.iter();
    clause.iter().all(|c| it.any(|s| s == c))
}

/// Prefix-join then per-candidate filter — the survivors in the join's
/// emission order.
pub fn generate_layer(
    terms: &Terms,
    a_prev: &[CanonicalSetId],
    alive: &FxHashSet<FactId>,
    nogoods: &ein_core::Nogoods,
) -> Vec<CanonicalSetId> {
    apriori_prefix_join(terms, a_prev)
        .into_iter()
        .filter(|c| filter_candidate(c, alive, nogoods))
        .collect()
}

/// Every singleton from `alive`, sorted — the BFS entry point.
///
/// Equivalent to what [`apriori_prefix_join`] would produce from
/// `A_0 = {()}`, but explicit, and the one place the whole traversal order is
/// decided.
pub fn layer_1(terms: &Terms, alive: &FxHashSet<FactId>) -> Vec<CanonicalSetId> {
    // `alive` is a `frozenset` in ein.py too, and `layer_1` is where it sorts.
    // determinism-ok: sorted by content on the next line, before any caller.
    let mut ids: Vec<FactId> = alive.iter().copied().collect();
    ids.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));
    ids.into_iter().map(|h| vec![h]).collect()
}

/// Within-layer ordering.
///
/// `"lex"` is canonical-tuple order: deterministic, uninformed, and the
/// shipping default because the regression baselines were recorded under it.
/// `"score-sum"` sums [`crate::hypgen::score_hypothesis`] over the set,
/// descending, tie-broken by the canonical tuple — so what it actually does
/// depends on `hypgen_scoring`, and under `"most-constrained"` every score is
/// `0.0` and it collapses to `lex`.
///
/// An element not yet in the KB is scored anyway: ein.py builds a synthetic
/// `Fact` for it, because the scorer reads only the relation and the args.
/// Here it needs no synthesis — a `FactId` exists whether or not any KB holds
/// the proposition true.
/// Takes the candidates **by value**, because it reorders them and cloning
/// them to do it is not free: `features/01 -e` spends 26 ms per solve copying
/// a layer's sets so that the sort has somewhere to put them, and a layer
/// arrives here in [`apriori_prefix_join`]'s emission order — which is already
/// `cmp_set` order, so `"lex"` is a linear scan over an owned vector rather
/// than a sort over a copy.
pub fn order_candidates(
    kb: &Kb,
    terms: &Terms,
    candidates: Vec<CanonicalSetId>,
    mode: &str,
) -> Result<Vec<CanonicalSetId>, ScoreError> {
    let mut out: Vec<CanonicalSetId> = candidates;
    match mode {
        "lex" => {
            out.sort_by(|a, b| cmp_set(terms, a, b));
            Ok(out)
        }
        "score-sum" => {
            let mut scored: Vec<(f64, CanonicalSetId)> = Vec::with_capacity(out.len());
            for c in out {
                let mut sum = 0.0;
                for &h in &c {
                    sum += crate::hypgen::score_hypothesis(kb, terms, h)?;
                }
                scored.push((sum, c));
            }
            // `sorted(key=(-score, c))`, **stable**. `total_cmp` is a total
            // order over the float and agrees with `<` on everything a score
            // can be; the tuple key makes the sort total anyway.
            scored.sort_by(|(sa, ca), (sb, cb)| {
                sb.total_cmp(sa).then_with(|| cmp_set(terms, ca, cb))
            });
            Ok(scored.into_iter().map(|(_, c)| c).collect())
        }
        other => Err(ScoreError::Unknown(format!(
            "unknown lattice_order mode: {} (expected 'lex' or 'score-sum')",
            ein_core::pyrepr::repr_str(other)
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ein_core::Value;

    fn ids(terms: &mut Terms, names: &[&str]) -> Vec<FactId> {
        names
            .iter()
            .map(|n| {
                let r = terms.intern_text("r").expect("room");
                let a = terms.intern_text(n).expect("room");
                terms.intern_fact(r, &[Value::sym(a)]).expect("room")
            })
            .collect()
    }

    /// The join emits each size-(k+1) set once, from the shared prefix, and
    /// **breaks** rather than scanning on: `{a,b}` and `{a,c}` join, `{b,c}`
    /// shares no prefix with them and contributes nothing.
    #[test]
    fn the_prefix_join_emits_each_superset_once() {
        let mut terms = Terms::new();
        let f = ids(&mut terms, &["a", "b", "c"]);
        let (a, b, c) = (f[0], f[1], f[2]);
        let prev = vec![vec![a, b], vec![a, c], vec![b, c]];
        let out = apriori_prefix_join(&terms, &prev);
        assert_eq!(out, vec![vec![a, b, c]]);
    }

    /// Singletons are ordered by **content**, not by `FactId`. Interning `zeta`
    /// first gives it the lower id and the higher name — so an id-ordered
    /// `layer_1` and a content-ordered one disagree, which is the bug this
    /// comparator exists to prevent.
    #[test]
    fn layer_1_orders_by_content_not_by_interning_order() {
        let mut terms = Terms::new();
        let f = ids(&mut terms, &["zeta", "alpha"]);
        assert!(
            f[0].0 < f[1].0,
            "zeta interned first, so it has the lower id"
        );
        let alive: FxHashSet<FactId> = f.iter().copied().collect();
        assert_eq!(layer_1(&terms, &alive), vec![vec![f[1]], vec![f[0]]]);
    }

    #[test]
    fn an_unknown_order_mode_reports_ein_pys_text() {
        let mut terms = Terms::new();
        let kb = Kb::new(ein_core::Program::new());
        let _ = ids(&mut terms, &["a"]);
        assert_eq!(
            order_candidates(&kb, &terms, Vec::new(), "nonsense"),
            Err(ScoreError::Unknown(
                "unknown lattice_order mode: 'nonsense' (expected 'lex' or \
                 'score-sum')"
                    .to_string()
            ))
        );
    }
}
