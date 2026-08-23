//! `ein-ir` — lexer, parser, AST, dumper, macros, imports, embedded stdlib.
//!
//! This crate owns the engine's only filesystem access — import resolution and
//! the stdlib — which is what keeps any later policy on what the engine may
//! read a single seam.
//!
//! The frontend is a **pure function** (source → forms) with no engine state,
//! which is why it is the first phase of the port
//! ([P1a.1](../../../../docs/history/m1a_rust/README.md#p1a1--ir-frontend)): every
//! artefact it produces is text a golden file already pins, so it can be
//! brought to byte parity in isolation.
//!
//! [`stdlib`] landed at
//! [S1a.0.3](../../../../docs/history/m1a_rust/README.md#s1a03--shared-stdlib-and-examples).

#![forbid(unsafe_code)]

pub mod ast;
pub mod dump;
pub mod from_ir;
pub mod imports;
pub mod lex;
pub mod macros;
pub mod parse;
pub mod stdlib;

pub use ast::{ArgSpan, Ast, FileId, Loc, Node, NodeId, SymId, loc_repr, node_repr};
pub use dump::{dump_canonical, dump_compact, dump_pretty};
pub use from_ir::{KbLoadError, load, load_file};
pub use imports::{LoadError, Resolver, resolve_and_minimize, resolve_imports};
pub use macros::{Macro, MacroError, collect_macros, expand_macros};
pub use parse::{ParseError, parse};
