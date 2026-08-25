//! `ein-infer` — Compile, match, saturate, world, search, verdict, explain.
//!
//! The engine proper. It lands over
//! [P1a.3](../../../../docs/history/m1a_rust/README.md#p1a3--deductive-core) (compile →
//! match → saturate → the NAF boundary) and
//! [P1a.4](../../../../docs/history/m1a_rust/README.md#p1a4--search-layer) (the
//! hypothesis loop above it).
//!
//! Everything here is a **port**, not a redesign: `ein.py` is the oracle, so
//! the shape of a plan, the order of a firing sequence and the text of an
//! error are all fixed by what the Python implementation does
//! ([design/01](../../../../docs/history/m1a_rust/design/01_parity_contract.md)). What
//! is free is the encoding, and that is where the whole win lives —
//! [design/05](../../../../docs/history/m1a_rust/design/05_matcher.md) §1 attributes 46 %
//! of an exhaustive solve's self time to unification the data model made
//! impossible to do quickly.

#![forbid(unsafe_code)]

pub mod apriori;
/// What this build has compiled in — `ein --version`'s feature line.
pub mod build;
pub mod canon;
pub mod closed;
pub mod commitment;
pub mod compile;
pub mod contradiction;
pub mod engine;
pub mod events;
pub mod expect;
pub mod explain;
pub mod firing;
/// The T1a.6.9.2 verification instrument — off unless the build asked for it.
#[cfg(feature = "fork-delta")]
pub mod fork_audit;
pub mod hrule;
pub mod hypgen;
pub mod lookahead;
pub mod match_;
pub mod mt19937;
pub mod naf_deps;
pub mod nogoods;
pub mod obligations;
pub mod plan;
pub mod predicates;
pub mod sanity;
pub mod saturator;
pub mod shape;
pub mod solve;
/// The S1a.7.0 measurement instrument — off unless the build asked for it.
#[cfg(feature = "spec-audit")]
pub mod spec_audit;
pub mod verdict;

pub use apriori::{
    CanonicalSetId, Filter, apriori_prefix_join, canonicalise, filter_candidate, filter_reason,
    generate_layer, layer_1, order_candidates,
};
pub use canon::{state_digest, state_key};
pub use closed::{emit_closed, producible_relations};
pub use commitment::{CommitmentSetResult, Kind, try_commitment_set};
pub use compile::{
    CompileError, PlanKey, PlanMemo, SharedMemo, activators_for, asserted_relation, compile_rule,
    naf_relation_refs, negated_relation, plan_key,
};
pub use contradiction::{Contradiction, contradicts, detect, has_contradiction};
pub use engine::Engine;
pub use events::{Events, Level};
pub use expect::{Outcome, Report, check};
pub use explain::{
    Explanation, ExplanationBudget, explain, minimal_contradiction_frontier,
    smallest_contradiction_frontier,
};
pub use firing::{ActivatorId, BindingKey, Env, FireError, Firing, build_fact, fire};
pub use hrule::Hrules;
pub use hypgen::{
    CLOSED, Drop, HypGenStats, ScoreError, Skip, candidate_objects, complete, consistent, generate,
    is_solution_node, open_hypotheses, score_hypothesis,
};
pub use lookahead::Lookahead;
pub use match_::{Emit, Match, Matcher};
pub use naf_deps::{NafDep, compute_naf_map, derived_naf_warnings};
pub use nogoods::emit_nogood;
pub use obligations::{Owed, Owes};
pub use plan::{
    Disjunct, GuardArg, GuardArgKind, MAX_REGS, NafGuard, Plan, PlanId, Probe, ProbeSrc, Reg,
    RelStep, Slot, Span, Step,
};
pub use predicates::Pred;
pub use sanity::{SanityError, check_commutativity};
pub use saturator::{SaturateError, Saturator, Session, Snapshot};
pub use shape::{
    commit_shape, explain_shape, hyp_shape, hyp_shape_with, lattice_shape, match_shape, naf_map,
    plan_shape, plan_shape_with, saturate_events, solve_shape,
};
pub use solve::{
    DeadCommitment, Dumper, EnteringInfo, LatticeProof, LatticeStats, LayerCensus, MonotonicStats,
    NoDumper, OnBudget, SolveError, SolveOptions, Solved, solve,
};
pub use verdict::{Answer, Solution, Verdict, goal_bindings, query_value};
