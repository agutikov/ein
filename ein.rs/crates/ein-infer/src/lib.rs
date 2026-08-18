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

pub mod apriori;
pub mod closed;
pub mod compile;
pub mod contradiction;
pub mod engine;
pub mod events;
pub mod firing;
pub mod hrule;
pub mod hypgen;
pub mod lookahead;
pub mod match_;
pub mod naf_deps;
pub mod nogoods;
pub mod plan;
pub mod predicates;
pub mod saturator;
pub mod shape;

pub use apriori::{
    CanonicalSetId, apriori_prefix_join, canonicalise, filter_candidate, generate_layer, layer_1,
    order_candidates,
};
pub use closed::{emit_closed, producible_relations};
pub use compile::{
    CompileError, PlanKey, PlanMemo, activators_for, asserted_relation, compile_rule,
    naf_relation_refs, negated_relation, plan_key,
};
pub use contradiction::{Contradiction, contradicts, detect, has_contradiction};
pub use engine::Engine;
pub use events::{Events, Level};
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
pub use plan::{
    Disjunct, GuardArg, GuardArgKind, MAX_REGS, NafGuard, Plan, PlanId, Probe, ProbeSrc, Reg,
    RelStep, Slot, Span, Step,
};
pub use predicates::Pred;
pub use saturator::{SaturateError, Saturator, Session};
pub use shape::{
    hyp_shape, hyp_shape_with, lattice_shape, match_shape, naf_map, plan_shape, plan_shape_with,
    saturate_events,
};
