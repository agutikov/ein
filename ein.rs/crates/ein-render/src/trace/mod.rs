//! The trace: its AST, a derivation's steps, the relevance prune, and the
//! region trace's markdown.
//!
//! The walk's answer table, which lived beside it in `answer.rs`, went with
//! the walk at M1e2 S1e2.1c.5; the read-out is [`crate::read_out`].

pub mod ast;
pub mod linearize;
pub mod region;
pub mod relevance;
pub mod render;

pub use ast::{FactRef, RefArg, TraceStep, fact_ref, parse_trace_steps, step_to_ir, trace_to_ir};
pub use linearize::{steps_of, steps_of_with};
pub use relevance::relevant_firings;
