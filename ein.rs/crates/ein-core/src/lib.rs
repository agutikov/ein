//! `ein-core` — the data model.
//!
//! No I/O and no engine: interning, `Value` / `FactId` as integers, row
//! storage, the layered COW KB ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md)),
//! and the `python_repr` compatibility renderer
//! ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §7).
//! Everything depends on this crate; it depends on nothing.
//!
//! The data model lands at
//! [P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core); the two
//! compatibility renderers land early, at
//! [S1a.1.2](../../../../docs/history/m1a_rust/README.md#s1a12--ast-arena-compatibility-renderers-dumper),
//! because they are trivial to write and expensive to discover missing at
//! [P1a.5](../../../../docs/history/m1a_rust/README.md#p1a5--presentation-and-cli).

#![forbid(unsafe_code)]

pub mod bitset;
pub mod config;
pub mod counters;
pub mod entities;
pub mod facts;
pub mod intern;
pub mod kb;
mod printable;
pub mod program;
pub mod prov;
pub mod pyfmt;
pub mod pynum;
pub mod pyrepr;
pub mod shape;
pub mod terms;
pub mod value;
pub mod walks;
pub mod why;

pub use bitset::BitSet;
pub use config::SolverConfig;
pub use counters::Counters;
pub use entities::{ExprRef, Loc, Macro, NameCategory, Pattern, Query, Registry, Relation, Rule};
pub use facts::{FactId, FactStore, Row};
pub use intern::{CAPACITY, Interner, Overflow, Symbol};
pub use kb::{Added, EqClasses, FactView, Kb, Layer, NameEntry, Nogoods, SlotKey};
pub use program::Program;
pub use prov::{NafArg, NafRef, Prov, ProvArena, ProvId, ProvKind, Region};
pub use pynum::{python_float, python_int};
pub use shape::shape;
pub use terms::{
    Kernel, Kernel as KernelSymbols, Lent, PREDICATES, RESERVED, STRUCTURAL, Table, Terms,
    is_predicate, is_reserved,
};
pub use value::{IntId, IntPool, Tag, Value};
pub use walks::{
    DerivationDag, Justifications, build_derivation_dag, detect_provenance_cycles, unsat_core,
    walk_premises,
};
pub use why::render_why;
