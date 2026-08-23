//! `ein-render` — DOT renderers, the markdown trace, the solution table.
//!
//! Everything that formats lives here, so the T3 surface is one crate and can
//! be diffed as a unit
//! ([design/12](../../../../docs/history/m1a_rust/design/12_toolchain_and_layout.md) §1).
//! The bulk lands at
//! [P1a.5](../../../../docs/history/m1a_rust/README.md#p1a5--presentation-and-cli); the
//! derivation DAG comes early, with the provenance it renders
//! ([S1a.2.4](../../../../docs/history/m1a_rust/README.md#s1a24--provenance-and-derivation-walks)),
//! because a walk whose output nobody can read is a walk nobody can check.

#![forbid(unsafe_code)]

pub mod answer;
pub mod builder;
pub mod constraints;
pub mod derivation;
pub mod dot_util;
pub mod dump;
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
pub use lattice_dag::{LatticeSource, LatticeView, render_lattice};
pub use palette::{PALETTE, hash_color};
pub use rules::{RuleMode, render_rule_form, render_rules_forms};
pub use shape::dot_shape;
pub use slice::{render_slice, render_solution, render_state};
pub use trace::{
    LinearizeOpts, Mode, Reductio, Trace, TraceStep, linearize, parse_trace_steps, render_markdown,
    trace_to_ir,
};
pub use why::render_why;
