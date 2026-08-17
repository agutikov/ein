//! `ein-ir` — lexer, parser, AST, dumper, macros, imports, embedded stdlib.
//!
//! This crate owns the engine's only filesystem access — import resolution and
//! the stdlib — which is what makes `--sandbox`
//! ([design/09](../../../plans/m1a_rust/design/09_server_mode.md)) a single
//! seam later.
//!
//! [`stdlib`] landed at
//! [S1a.0.3](../../../plans/m1a_rust/p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md);
//! the rest arrives with
//! [P1a.1](../../../plans/m1a_rust/p1a.1_ir_frontend/README.md).

#![forbid(unsafe_code)]

pub mod stdlib;
