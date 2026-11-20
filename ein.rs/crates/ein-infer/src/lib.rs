//! `ein-infer` — Compile, match, saturate, world, search, verdict, explain.
//!
//! The engine proper. It lands over
//! [01-architecture.md § 3.3](../../../../docs/ein/ein.rs/01-architecture.md#33-ein-infer) (compile →
//! match → saturate → the NAF boundary) and
//! [03-design/07-search.md](../../../../docs/ein/ein.rs/03-design/07-search.md) (the
//! hypothesis loop above it). The search is the region's: every question
//! about `R` is [`explore`]'s, answered by a route over nodes that are root
//! with a subset of H committed and saturated ([`region`]). The generator
//! ([`hypgen`]) builds H by its ladder, config-free, once at root.
//!
//! The shape of a plan, the order of a firing sequence and the text of an
//! error are all observable, and fixed:
//! [03-design/01-determinism-and-order.md](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md)
//! states what a run's output depends on, and
//! [07-diagnostics.md](../../../../docs/ein/user/04-reference/07-diagnostics.md) every
//! refusal's text.
//! What is free is the encoding, and that is where the cost lives —
//! [03-design/05-matcher.md § 1](../../../../docs/ein/ein.rs/03-design/05-matcher.md#1-the-costs-it-is-shaped-against) attributes 46 %
//! of an exhaustive solve's self time to unification.

#![forbid(unsafe_code)]

pub mod build;
pub mod canon;
pub mod claim;
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
pub mod extendable;
pub mod firing;
pub mod hrule;
pub mod hypgen;
pub mod invariant;
pub mod known_dead;
pub mod match_;
pub mod mechanisms;
pub mod monotone;
pub mod mt19937;
pub mod naf_deps;
pub mod oblgen;
pub mod obligations;
pub mod open;
pub mod order;
pub mod plan;
pub mod predicates;
pub mod read_out;
pub mod region;
pub mod saturator;
pub mod scope;
pub mod shape;
pub mod slice;
pub mod states;
pub mod strata;
pub mod structure;
pub mod transversal;
pub(crate) mod tree;
/// What this build has compiled in — `ein --version`'s feature line.
pub mod tree_classes;
pub mod unheld;
pub mod verdict;

pub use canon::{state_digest, state_key};
pub use compile::{
    CompileError, PlanKey, PlanMemo, SharedMemo, activators_for, asserted_relation,
    binds_a_relation_to_a_fact, compile_rule, naf_relation_refs, plan_key, take_inert_activators,
};
pub use contradiction::{Contradiction, Pairs, detect, has_contradiction};
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
pub use hypgen::{CLOSED, Drop, HypGenStats, Skip, candidate_objects};
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
    explain_shape, match_shape, naf_map, plan_shape, plan_shape_with, saturate_events,
};
pub use verdict::{goal_bindings, goal_written, query_value, solve_value};
