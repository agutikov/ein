//! `ein-render` — DOT renderers, the markdown trace, the read-out.
//!
//! Everything that formats lives here, so every rendered surface is one crate
//! and can be diffed as a unit
//! ([03-design/10-toolchain-and-layout.md § 1](../../../../docs/ein/ein.rs/03-design/10-toolchain-and-layout.md#1-the-workspace)).
//! The bulk lands at
//! [01-architecture.md § 3.4](../../../../docs/ein/ein.rs/01-architecture.md#34-ein-einb-ein-render-ein-cli); the
//! derivation DAG comes early, with the provenance it renders
//! ([03-design/03-data-model.md § 8.4](../../../../docs/ein/ein.rs/03-design/03-data-model.md#84-derivation-walks)),
//! because a walk whose output nobody can read is a walk nobody can check.

#![forbid(unsafe_code)]

pub mod builder;
pub mod constraints;
pub mod derivation;
pub mod dot_util;
pub mod dump;
pub mod ir_dot;
pub mod kb_dot;
pub mod models;
pub mod palette;
pub mod read_out;
pub mod region_doc;
pub mod region_dot;
pub mod rules;
pub mod sentence;
pub mod shape;
pub mod slice;
pub mod structure;
pub mod trace;

pub use constraints::render_constraints;
pub use derivation::{derivation_dag_to_dot, fact_dot_id};
pub use dot_util::{esc, fact_key, fact_label, hashed_id, multiline, quote, value_label};
/// `:why` substitution — [`ein_core::render_why`], re-exported.
///
/// It lives in `ein-core` so the obligation pass can render an `owe`'s
/// sentence from `ein-infer`. Within this crate a rule's `:why` is filled by
/// [`sentence::rule_sentence`] alone, and a relation's by
/// [`sentence::statement`].
pub use ein_core::render_why;
pub use ir_dot::{DotOpts, TraceView, to_dot as ir_to_dot, to_dot_form};
pub use kb_dot::{ColourBy, KbDotOpts, to_dot as kb_to_dot};
pub use palette::{PALETTE, hash_color};
pub use rules::{RuleMode, render_rule_form, render_rules_forms};
pub use sentence::{FactSentence, Origin, fact_sentence, rule_sentence};
pub use shape::dot_shape;
pub use slice::{render_slice, render_solution, render_state};
pub use trace::{TraceStep, steps_of, steps_of_with};
