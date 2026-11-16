//! `ein-ir` — the parser, AST, dumper, macros, imports, embedded stdlib.
//!
//! The parser is generated from the grammar, `src/ein.l` and `src/ein.y`
//! ([`grammar`], M1f S1f.5.5); since the swap (T1f.5.5.4) it is the only one.
//!
//! This crate owns the engine's only filesystem access — the read of a
//! program's text ([`read`], since M1f S1f.5.22 every command's and every
//! imported module's), import resolution and the stdlib — which is what keeps
//! any later policy on what the engine may read a single seam.
//!
//! The frontend is a **pure function** (source → forms) with no engine state,
//! which is why it was M1a's first phase
//! ([design/ir-frontend.md § 1](../../../../docs/ein/ein.rs/design/ir-frontend.md#1-a-pure-function-of-its-inputs)): every
//! artefact it produces is text a golden file pins, so it can be checked byte
//! for byte in isolation.
//!
//! [`stdlib`] landed at
//! [design/shared-assets.md](../../../../docs/ein/ein.rs/design/shared-assets.md).

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
