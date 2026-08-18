//! `ein-render` — DOT renderers, the markdown trace, the solution table.
//!
//! Everything that formats lives here, so the T3 surface is one crate and can
//! be diffed as a unit
//! ([design/12](../../../plans/m1a_rust/design/12_toolchain_and_layout.md) §1).
//! The bulk lands at
//! [P1a.5](../../../plans/m1a_rust/p1a.5_presentation/README.md); the
//! derivation DAG comes early, with the provenance it renders
//! ([S1a.2.4](../../../plans/m1a_rust/p1a.2_kb_core/s1a.2.4_provenance.md)),
//! because a walk whose output nobody can read is a walk nobody can check.

#![forbid(unsafe_code)]

pub mod answer;
pub mod builder;
pub mod constraints;
pub mod derivation;
pub mod dot_util;
pub mod ir_dot;
pub mod kb_dot;
pub mod lattice_dag;
pub mod palette;
pub mod rules;
pub mod shape;
pub mod slice;
pub mod trace;
pub mod why;

pub use answer::{render_answer, render_solution_table};
pub use constraints::render_constraints;
pub use derivation::{derivation_dag_to_dot, fact_dot_id};
pub use dot_util::{esc, fact_key, fact_label, hashed_id, multiline, quote, value_label};
pub use ir_dot::{DotOpts, TraceView, to_dot as ir_to_dot, to_dot_form};
pub use kb_dot::{ColourBy, KbDotOpts, to_dot as kb_to_dot};
pub use lattice_dag::{LatticeView, render_lattice};
pub use palette::{PALETTE, hash_color};
pub use rules::{RuleMode, render_rule_form, render_rules_forms};
pub use shape::dot_shape;
pub use slice::{render_slice, render_solution, render_state};
pub use trace::{
    LinearizeOpts, Mode, Reductio, Trace, TraceStep, linearize, parse_trace_steps, render_markdown,
    trace_to_ir,
};
pub use why::render_why;
