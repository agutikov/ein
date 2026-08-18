//! `ein-ir` — lexer, parser, AST, dumper, macros, imports, embedded stdlib.
//!
//! This crate owns the engine's only filesystem access — import resolution and
//! the stdlib — which is what makes `--sandbox`
//! ([design/09](../../../plans/m1a_rust/design/09_server_mode.md)) a single
//! seam later.
//!
//! The frontend is a **pure function** (source → forms) with no engine state,
//! which is why it is the first phase of the port
//! ([P1a.1](../../../plans/m1a_rust/p1a.1_ir_frontend/README.md)): every
//! artefact it produces is text a golden file already pins, so it can be
//! brought to byte parity in isolation.
//!
//! [`stdlib`] landed at
//! [S1a.0.3](../../../plans/m1a_rust/p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md).

#![forbid(unsafe_code)]

pub mod ast;
pub mod dump;
pub mod lex;
pub mod parse;
pub mod stdlib;

pub use ast::{ArgSpan, Ast, FileId, Loc, Node, NodeId, SymId};
pub use dump::{dump_canonical, dump_compact, dump_pretty};
pub use parse::{ParseError, parse};
