//! `ein-infer` — Compile, match, saturate, world, search, verdict, explain.
//!
//! The engine proper. It lands over
//! [P1a.3](../../../plans/m1a_rust/p1a.3_deductive_core/README.md) (compile →
//! match → saturate → the NAF boundary) and
//! [P1a.4](../../../plans/m1a_rust/p1a.4_search_layer/README.md) (the
//! hypothesis loop above it).
//!
//! Everything here is a **port**, not a redesign: `ein.py` is the oracle, so
//! the shape of a plan, the order of a firing sequence and the text of an
//! error are all fixed by what the Python implementation does
//! ([design/01](../../../plans/m1a_rust/design/01_parity_contract.md)). What
//! is free is the encoding, and that is where the whole win lives —
//! [design/05](../../../plans/m1a_rust/design/05_matcher.md) §1 attributes 46 %
//! of an exhaustive solve's self time to unification the data model made
//! impossible to do quickly.

#![forbid(unsafe_code)]

pub mod compile;
pub mod contradiction;
pub mod engine;
pub mod events;
pub mod firing;
pub mod match_;
pub mod plan;
pub mod predicates;
pub mod saturator;
pub mod shape;

pub use compile::{
    CompileError, PlanKey, PlanMemo, activators_for, asserted_relation, compile_rule,
    naf_relation_refs, negated_relation, plan_key,
};
pub use contradiction::{Contradiction, contradicts, detect, has_contradiction};
pub use engine::Engine;
pub use events::{Events, Level};
pub use firing::{ActivatorId, BindingKey, Env, FireError, Firing, build_fact, fire};
pub use match_::{Emit, Match, Matcher};
pub use plan::{
    Disjunct, GuardArg, GuardArgKind, MAX_REGS, NafGuard, Plan, PlanId, Probe, ProbeSrc, Reg,
    RelStep, Slot, Span, Step,
};
pub use predicates::Pred;
pub use saturator::{SaturateError, Saturator, Session};
pub use shape::{match_shape, plan_shape, plan_shape_with, saturate_events};
