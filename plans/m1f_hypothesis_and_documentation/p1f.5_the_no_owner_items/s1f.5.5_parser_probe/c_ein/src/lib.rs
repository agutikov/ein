//! S1f.5.5 step 2's prototype front end: the final grammar, building today's
//! `ein_ir::Ast` through `grammar/build_ast.rs`, so the real loader reads it.
#![deny(unsafe_code)]
use ein_ir::{Ast, NodeId};
use lrpar::{LexError, Lexeme, NonStreamingLexer};

#[path = "../../grammar/build_ast.rs"]
pub mod b;
lrlex::lrlex_mod!("ein.l");
lrpar::lrpar_mod!("ein.y");

/// One refusal: its line and column, and the report recovery gave.
#[derive(Debug, Clone)]
pub struct Refusal {
    pub line: usize,
    pub col: usize,
    pub text: String,
}

/// Parse `text` into `ast`. On a refusal, every error recovery reached.
pub fn parse(ast: Ast, text: &str, filename: Option<&str>) -> (Ast, Result<Vec<NodeId>, Vec<Refusal>>) {
    let def = ein_l::lexerdef();
    let lexer = def.lexer(text);
    let cell = b::B::new(ast, filename);
    let (res, errs) = ein_y::parse(&lexer, &cell);
    let ast = cell.into_inner().ast;
    if errs.is_empty() {
        return (ast, Ok(res.expect("no error, so a tree")));
    }
    let out = errs
        .iter()
        .map(|e| {
            let span = match e {
                lrpar::LexParseError::LexError(l) => l.span(),
                lrpar::LexParseError::ParseError(p) => p.lexeme().span(),
            };
            let ((line, col), _) = lexer.line_col(span);
            Refusal { line, col, text: e.pp(&lexer, &ein_y::token_epp) }
        })
        .collect();
    (ast, Err(out))
}
