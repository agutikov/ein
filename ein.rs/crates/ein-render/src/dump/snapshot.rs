//! The lattice content snapshot — `ein.py`'s
//! `inference/monotonic/snapshot.py`.
//!
//! Projects a completed lattice solve into a value that is *invariant* under
//! within-layer traversal-order permutations: two solves of the same puzzle at
//! the same `max_set_size` under different `lattice_order_seed`s must produce
//! equal snapshots. If they do not, an order leak has crept into the loop and
//! the lattice's "set determines kb" invariant is degraded at the engine level.
//!
//! The snapshot is **result-level**: it keys on the post-saturation *states*
//! reached (solutions, deads, nodes), not on the commitment *paths* or the
//! learned clauses — both of which are legitimately order- and
//! orientation-sensitive once symmetric pairs are no longer canonicalised by
//! the kernel.
//!
//! `nodes_by_state_key` is empty for the same reason
//! [`super::lattice`]'s `kb_index/` never materialises: `solve` writes no
//! `SetNode`s. The field is here because the snapshot's *identity* is defined
//! to include it, and a shuffle harness comparing two snapshots must compare
//! the same shape either way.

use ein_core::pyrepr::{PyValue, repr};
use ein_core::{FactId, Kb, Tag, Terms, Value};
use ein_infer::canon::state_key;
use ein_infer::solve::LatticeProof;
use ein_infer::verdict::Answer;

/// A content-addressed lattice projection — `LatticeSnapshotV1`.
///
/// Every field is a sorted, deduplicated vector, so structural equality is
/// enough to compare two snapshots without bespoke logic.
/// One entry of [`LatticeSnapshot::nodes_by_state_key`]: a state key, the
/// union of every label observed for it, and the union of the per-`SetNode`
/// verdicts.
pub type StateNode = (Box<[FactId]>, Vec<Vec<FactId>>, Vec<String>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LatticeSnapshot {
    /// One entry per distinct state key observed.
    pub nodes_by_state_key: Vec<StateNode>,
    /// `state_key(root_kb)` at termination — it carries the accumulated
    /// singleton-death `(not h)` writebacks and the forced-positive
    /// promotions, the only root writes during the search.
    pub root_state_key: Box<[FactId]>,
    pub verdict_kind: String,
    /// The distinct satisfying *model states*, keyed by post-saturation state
    /// rather than by commitment path — so the two orientations of a
    /// symmetric pair count once.
    pub solutions: Vec<Box<[FactId]>>,
    /// The distinct refuted states, keyed the same way.
    pub deads: Vec<Box<[FactId]>>,
    /// The surviving size-N frontier when the depth cap was the terminator.
    pub alive_at_end: Vec<Vec<FactId>>,
}

/// `repr` of one canonical fact — `(relation_name, hashable_args)`, with a
/// nested fact recursing into *that* shape rather than into a `Fact`.
///
/// The nesting matters: `canon._hashable_args` lowers a nested `Fact` to its
/// own `(rel, args)` tuple, so ein.py's `repr` of a state-key element spells
/// `('not', (('color-loc', ('Blue', 'House-1')),))` where the *commitment*
/// repr — over raw `Fact.args` — spells `Fact(relation_name=…)`. Two shapes,
/// two builders, and the sort order differs between them.
fn canon_fact(terms: &Terms, id: FactId) -> PyValue {
    let (rel, args) = terms.fact(id);
    PyValue::Tuple(vec![
        PyValue::Str(terms.sym(rel).to_string()),
        PyValue::Tuple(args.iter().map(|a| canon_value(terms, *a)).collect()),
    ])
}

fn canon_value(terms: &Terms, v: Value) -> PyValue {
    match v.tag() {
        Tag::Fact => canon_fact(terms, v.as_fact().expect("tagged Fact")),
        _ => terms.py_value(v),
    }
}

/// `repr` of a whole canonical state key — the sort key for a *snapshot's*
/// cells, where a proof's cells use [`crate::lattice_dag::commitment_repr`].
pub fn canon_key_repr(terms: &Terms, key: &[FactId]) -> String {
    repr(&PyValue::Tuple(
        key.iter().map(|f| canon_fact(terms, *f)).collect(),
    ))
}

/// A state key in `repr` order.
///
/// [`ein_infer::canon::state_key`] sorts by `FactId` — a `u32` sort and a
/// `memcmp` instead of a string per fact — because for *identity* any total
/// order does. ein.py sorts by `repr`, and where the key is **displayed** that
/// is the order a reader sees, so the re-sort lands here: at the one place a
/// state key becomes output. `canon.rs` says as much, and names this phase.
fn repr_sorted(terms: &Terms, key: &[FactId]) -> Box<[FactId]> {
    let mut keyed: Vec<(String, FactId)> = key
        .iter()
        .map(|f| (repr(&canon_fact(terms, *f)), *f))
        .collect();
    keyed.sort();
    keyed.into_iter().map(|(_, f)| f).collect()
}

/// Project a completed lattice solve into a [`LatticeSnapshot`].
///
/// `root_kb` is the KB at termination — the solver's root after the call
/// returns.
pub fn lattice_snapshot(
    answer: &Answer,
    proof: &LatticeProof,
    root_kb: &Kb,
    terms: &Terms,
) -> LatticeSnapshot {
    let mut solutions: Vec<Box<[FactId]>> = proof
        .solutions
        .iter()
        .map(|s| repr_sorted(terms, &state_key(&s.kb)))
        .collect();
    solutions.sort();
    solutions.dedup();
    let mut deads: Vec<Box<[FactId]>> = proof
        .dead_commitments
        .iter()
        .map(|d| repr_sorted(terms, &d.state_key))
        .collect();
    deads.sort();
    deads.dedup();
    let mut alive_at_end: Vec<Vec<FactId>> = proof.alive_at_end.clone();
    alive_at_end.sort();
    alive_at_end.dedup();
    LatticeSnapshot {
        // Empty by construction — see the module docs.
        nodes_by_state_key: Vec::new(),
        root_state_key: repr_sorted(terms, &state_key(root_kb)),
        verdict_kind: answer.as_str().to_string(),
        solutions,
        deads,
        alive_at_end,
    }
}
