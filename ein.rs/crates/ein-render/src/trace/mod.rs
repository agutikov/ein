//! The trace: its AST, the linearizer, the relevance prune, and the markdown.
//!
//! `ein.py`'s `ein/trace/` package, module for module. The answer table lives
//! beside it in [`crate::answer`], as it does there.

pub mod ast;
pub mod linearize;
pub mod relevance;
pub mod render;

pub use ast::{FactRef, RefArg, TraceStep, fact_ref, parse_trace_steps, step_to_ir, trace_to_ir};
pub use linearize::{LinearizeOpts, Reductio, Trace, linearize};
pub use relevance::relevant_firings;
pub use render::{Mode, render_markdown};
