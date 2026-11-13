//! `ein-infer` — Compile, match, saturate, world, search, verdict, explain.
//!
//! The engine proper. It lands over
//! [P1a.3](../../../../docs/history/m1a_rust/README.md#p1a3--deductive-core) (compile →
//! match → saturate → the NAF boundary) and
//! [P1a.4](../../../../docs/history/m1a_rust/README.md#p1a4--search-layer) (the
//! hypothesis loop above it). The search is the region's since M1e2: every
//! question about `R` is [`explore`]'s, answered by a route. P1a.4's
//! hypothesis loop, the walk, had no command since S1e2.1a.5's switch and was
//! deleted at S1e2.1c.5 on the user's ruling of 2026-11-03; its generator
//! ([`hypgen`]) stays, because H is built by its ladder and two summary blocks
//! still call it.
//!
//! The shape of a plan, the order of a firing sequence and the text of an
//! error are all observable, and fixed: where no other page states one, it is
//! [`docs/kernel/defined_behaviour.md`](../../../../docs/kernel/defined_behaviour.md)'s.
//! What is free is the encoding, and that is where the cost lives —
//! [design/05](../../../../docs/history/m1a_rust/design/05_matcher.md) §1 attributes 46 %
//! of an exhaustive solve's self time to unification.

#![forbid(unsafe_code)]

/// What this build has compiled in — `ein --version`'s feature line.
pub mod build;
pub mod canon;
pub mod claim;
pub mod closed;
pub mod compile;
pub mod contradiction;
pub mod count;
pub mod described;
pub mod detector;
pub mod downset;
pub mod engine;
pub mod error;
pub mod events;
pub mod expect;
pub mod explain;
pub mod explore;
pub mod firing;
pub mod forbid;
pub mod hrule;
pub mod hypgen;
pub mod invariant;
pub mod lookahead;
pub mod match_;
pub mod monotone;
pub mod mt19937;
pub mod naf_deps;
pub mod oblgen;
pub mod obligations;
pub mod plan;
pub mod predicates;
pub mod read_out;
pub mod region;
pub mod saturator;
pub mod scope;
pub mod shape;
pub mod states;
pub mod strata;
pub mod transversal;
pub(crate) mod tree;
pub mod types_as_objects;
pub mod verdict;

pub use canon::{state_digest, state_key};
pub use closed::{emit_closed, producible_relations};
pub use compile::{
    CompileError, PlanKey, PlanMemo, SharedMemo, activators_for, asserted_relation, compile_rule,
    naf_relation_refs, negated_relation, plan_key,
};
pub use contradiction::{Contradiction, contradicts, detect, has_contradiction};
pub use engine::Engine;
pub use error::SolveError;
pub use events::{Events, Level};
pub use expect::{Outcome, Report};
pub use explain::{
    Explanation, ExplanationBudget, explain, minimal_contradiction_frontier,
    smallest_contradiction_frontier,
};
pub use firing::{
    ActivatorId, BindingKey, Env, FireError, Firing, build_conclusion, build_fact, fire,
};
pub use hrule::Hrules;
pub use hypgen::{
    CLOSED, Drop, HypGenStats, ScoreError, Skip, candidate_objects, complete, consistent, generate,
    is_solution_node, open_hypotheses, score_hypothesis,
};
pub use lookahead::Lookahead;
pub use match_::{Emit, Match, Matcher};
pub use naf_deps::{NafDep, compute_naf_map, derived_naf_warnings};
pub use obligations::{Owed, Owes};
pub use plan::{
    Disjunct, GuardArg, GuardArgKind, MAX_REGS, NafGuard, Plan, PlanId, Probe, ProbeSrc, Reg,
    RelStep, Slot, Span, Step,
};
pub use predicates::Pred;
pub use saturator::{SaturateError, Saturator, Session, Snapshot};
pub use shape::{
    explain_shape, hyp_shape, hyp_shape_with, match_shape, naf_map, plan_shape, plan_shape_with,
    saturate_events,
};
pub use verdict::{goal_bindings, goal_written, query_value, solve_value};
