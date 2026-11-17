//! `ein-core` — the data model.
//!
//! No I/O and no engine: interning, `Value` / `FactId` as integers, row
//! storage, the layered COW KB ([03-design/03-data-model.md](../../../../docs/ein/ein.rs/03-design/03-data-model.md)),
//! and the `repr` / `numfmt` value renderers
//! ([03-design/01-determinism-and-order.md § 7](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#7-the-value-renderer)).
//! Everything depends on this crate; it depends on nothing.
//!
//! The data model lands at
//! [03-design/03-data-model.md](../../../../docs/ein/ein.rs/03-design/03-data-model.md); the two
//! renderers land early, at
//! [03-design/01-determinism-and-order.md § 7](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#7-the-value-renderer),
//! because they are trivial to write and expensive to discover missing at
//! [01-architecture.md § 3.4](../../../../docs/ein/ein.rs/01-architecture.md#34-ein-einb-ein-render-ein-cli).

#![forbid(unsafe_code)]

pub mod bitset;
pub mod config;
pub mod counters;
pub mod entities;
pub mod facts;
pub mod intern;
pub mod kb;
pub mod numfmt;
pub mod numparse;
mod printable;
pub mod program;
pub mod prov;
pub mod repr;
pub mod shape;
pub mod terms;
pub mod text;
pub mod value;
pub mod walks;
pub mod why;

pub use bitset::BitSet;
pub use config::SolverConfig;
pub use counters::Counters;
pub use entities::{
    ExprRef, Loc, Macro, NameCategory, Pattern, Query, Registry, Relation, Rule, Solve,
};
pub use facts::{FactId, FactStore, Row};
pub use intern::{CAPACITY, Interner, Overflow, Symbol};
pub use kb::{Added, EqClasses, FactView, Kb, Layer, NameEntry, SlotKey};
pub use numparse::{parse_float, parse_int};
pub use program::{Program, Sources};
pub use prov::{NafArg, NafRef, Prov, ProvArena, ProvId, ProvKind, Region};
pub use shape::shape;
pub use terms::{
    Kernel, Kernel as KernelSymbols, Lent, PREDICATES, RESERVED, STRUCTURAL, Table, Terms,
    is_predicate, is_reserved,
};
pub use value::{Tag, Value};
pub use walks::{
    DerivationDag, Justifications, build_derivation_dag, detect_provenance_cycles, unsat_core,
    walk_premises,
};
pub use why::render_why;
