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

pub mod derivation;
pub mod dot_util;

pub use derivation::{derivation_dag_to_dot, fact_dot_id};
pub use dot_util::{esc, fact_key, hashed_id, quote};
