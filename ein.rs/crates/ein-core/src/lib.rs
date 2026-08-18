//! `ein-core` — the data model.
//!
//! No I/O and no engine: interning, `Value` / `FactId` as integers, row
//! storage, the layered COW KB ([design/03](../../../plans/m1a_rust/design/03_data_model.md)),
//! and the `python_repr` compatibility renderer
//! ([design/02](../../../plans/m1a_rust/design/02_determinism_and_order.md) §7).
//! Everything depends on this crate; it depends on nothing.
//!
//! The data model lands at
//! [P1a.2](../../../plans/m1a_rust/p1a.2_kb_core/README.md); the two
//! compatibility renderers land early, at
//! [S1a.1.2](../../../plans/m1a_rust/p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md),
//! because they are trivial to write and expensive to discover missing at
//! [P1a.5](../../../plans/m1a_rust/p1a.5_presentation/README.md).

#![forbid(unsafe_code)]

pub mod facts;
pub mod intern;
mod printable;
pub mod pyfmt;
pub mod pyrepr;
pub mod terms;
pub mod value;

pub use facts::{FactId, FactStore, Row};
pub use intern::{CAPACITY, Interner, Overflow, Symbol};
pub use terms::Terms;
pub use value::{IntId, IntPool, Tag, Value};
