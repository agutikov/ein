//! The trace: its AST, a derivation's steps, the relevance prune, and the
//! region trace's markdown. The read-out it begins with is
//! [`crate::read_out`].

pub mod ast;
pub mod linearize;
pub mod region;
pub mod relevance;
pub mod render;

pub use ast::{FactRef, RefArg, TraceStep, fact_ref};
pub use linearize::{steps_of, steps_of_with};
pub use relevance::relevant_firings;
