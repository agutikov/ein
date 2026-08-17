//! `ein-core` — the data model.
//!
//! No I/O and no engine: interning, `Value` / `FactId` as integers, row
//! storage, the layered COW KB ([design/03](../../../plans/m1a_rust/design/03_data_model.md)),
//! and the `python_repr` compatibility renderer
//! ([design/02](../../../plans/m1a_rust/design/02_determinism_and_order.md) §7).
//! Everything depends on this crate; it depends on nothing.
//!
//! Empty until [P1a.2](../../../plans/m1a_rust/p1a.2_kb_core/README.md).

#![forbid(unsafe_code)]
