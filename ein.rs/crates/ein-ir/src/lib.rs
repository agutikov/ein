//! `ein-ir` — the parser, AST, dumper, macros, imports, embedded stdlib.
//!
//! The parser is generated from the grammar, `src/ein.l` and `src/ein.y`
//! ([`grammar`]), and it is the only one.
//!
//! This crate owns the engine's only filesystem access — the read of a
//! program's text ([`read`], every command's and every imported module's),
//! import resolution and the stdlib — which is what keeps any policy on what
//! the engine may read a single seam.
//!
//! The frontend is a **pure function** (source → forms) with no engine state
//! ([03-design/04-ir-frontend.md § 1](../../../../docs/ein/ein.rs/03-design/04-ir-frontend.md#1-a-pure-function-of-its-inputs)): every
//! artefact it produces is text a golden file pins, so it can be checked byte
//! for byte in isolation.
//!
//! [`stdlib`] is specified by
//! [03-design/02-shared-assets.md](../../../../docs/ein/ein.rs/03-design/02-shared-assets.md).

#![forbid(unsafe_code)]

pub mod ast;
pub mod claim;
pub mod dump;
pub mod expect;
pub mod from_ir;
pub mod grammar;
pub mod imports;
pub mod macros;
pub mod read;
pub mod stdlib;

pub use ast::{ArgSpan, Ast, FileId, Loc, Node, NodeId, SymId, loc_repr, node_repr};
pub use dump::{dump_canonical, dump_compact, dump_pretty};
pub use from_ir::{
    KbLoadError, LoadOptions, MAX_MATCH_VARIABLES, WHY_REQUIRED_OF, load, load_file,
    load_file_with, load_query, load_query_with, load_with, pattern_relations, written_in,
    written_in_conclusions,
};
pub use grammar::{ParseError, SyntaxError, parse};
pub use imports::{LoadError, Resolver, resolve_imports};
pub use macros::{Macro, MacroError, collect_macros, expand_macros};
