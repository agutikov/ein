//! The generated parser: S1f.5.5's grammar, `src/ein.l` and `src/ein.y` in
//! grmtools' notation, compiled by `build.rs` (M1f S1f.5.5, T1f.5.5.3).
//!
//! **It sits beside the hand-written parser and is not the default path.**
//! Every command reads its program through [`crate::parse()`], which is the
//! hand-written parser unless `ein-ir` is built with its `generated-parser`
//! feature; this module is reached from that hook, from its own tests, and
//! from T1f.5.5.9's comparison. The swap (T1f.5.5.4) makes it the only parser.
//!
//! **What it builds is the hand-written parser's tree**, node for node and
//! [`Loc`](crate::Loc) for `Loc`, through one builder call per grammar action
//! (`grammar/build_ast.rs`), so the loader reads it unchanged. Where the two
//! languages differ, they differ by a ruling of this phase, which T1f.5.5.9
//! attributes file by file; `ein-ir/tests/generated_parser.rs` banks every
//! corpus file where the trees are not the same.
//!
//! **A refused file reports every error recovery reaches** ([`Refusal`]), each
//! with its position, what was found there, and at most three of the repairs
//! CPCT+ ranked, the first being the one the parse went on with, then a count
//! of the rest. Recovery is deterministic: `lrpar` is the
//! patched copy under `ein.rs/vendor/lrpar` (F11), which ranks equal repairs
//! in the order its search found them and stops at a count of search steps,
//! [`RECOVERY_STEP_BUDGET`], rather than at a wall-clock time; a report cut
//! there says so after its last error. A file that parses pays nothing for
//! recovery: the recoverer is built at the first error.
//!
//! **A program nested past [`NESTING_LIMIT`] is refused before it is parsed**
//! (F10). The LR parser survives any depth, but macro expansion, the loader
//! and the dumper recurse over the tree it builds.

use std::fmt;
use std::sync::OnceLock;

use cfgrammar::TIdx;
use lrlex::{DefaultLexerTypes, LRNonStreamingLexerDef, LexerDef};
use lrpar::{LexError, LexParseError, Lexeme, NonStreamingLexer, ParseRepair};

use crate::ast::{Ast, NodeId};

mod build_ast;
use build_ast as b;

lrlex::lrlex_mod!("ein.l");
lrpar::lrpar_mod!("ein.y");

/// How deep a program may nest its parentheses: a top-level form is depth 1.
/// One past it is refused at the parse, with a line (F10). The stage's
/// default, 1 000, checked on a debug build in a test thread's stack through
/// the dumper, macro expansion and the loader
/// (`ein-ir/tests/generated_parser.rs`).
pub const NESTING_LIMIT: usize = 1000;

/// The search steps error recovery may take over one parse, `lrpar`'s patched
/// constant (F11), re-exported so that a report can name it.
pub use lrpar::parser::RECOVERY_STEP_BUDGET;

/// How many of an error's ranked repairs a report lists before it counts the
/// rest.
pub const REPAIRS_SHOWN: usize = 3;

/// One error of a refused file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    /// 1-based, as a [`Loc`](crate::Loc) counts: lines at `\n`, columns in
    /// characters.
    pub line: u32,
    pub col: u32,
    /// What is wrong there: `unexpected …`, a character no token starts
    /// with, or the nesting limit.
    pub what: String,
    /// The first [`REPAIRS_SHOWN`] repairs recovery ranked, each a sequence of
    /// steps; the first is the one the parse went on with.
    pub repairs: Vec<String>,
    /// How many ranked repairs the report leaves out.
    pub more_repairs: usize,
}

/// A refused file: every error recovery reached, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub file: String,
    pub errors: Vec<SyntaxError>,
    /// Recovery stopped at [`RECOVERY_STEP_BUDGET`] at the last error, so the
    /// file may hold errors past it that the report does not list.
    pub cut: bool,
    /// The search steps recovery took over the parse, at most
    /// [`RECOVERY_STEP_BUDGET`].
    pub recovery_steps: u64,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.errors {
            writeln!(f, "{}:{}:{}: {}", self.file, e.line, e.col, e.what)?;
            for r in &e.repairs {
                writeln!(f, "  repair: {r}")?;
            }
            if e.more_repairs > 0 {
                let s = if e.more_repairs == 1 { "" } else { "s" };
                writeln!(f, "  and {} more repair{s}", e.more_repairs)?;
            }
        }
        if self.cut
            && let Some(last) = self.errors.last()
        {
            writeln!(
                f,
                "{}:{}:{}: recovery stopped here, at its budget of {} steps: \
                 errors after this one are not reported",
                self.file, last.line, last.col, RECOVERY_STEP_BUDGET
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for Refusal {}

/// The lexer, built once: its rules are regular expressions, compiled when
/// it is made.
fn lexerdef() -> &'static LRNonStreamingLexerDef<DefaultLexerTypes<u32>> {
    static DEF: OnceLock<LRNonStreamingLexerDef<DefaultLexerTypes<u32>>> = OnceLock::new();
    DEF.get_or_init(ein_l::lexerdef)
}

/// Parse `text` with the generated parser, appending its top-level forms to
/// `ast`, as [`crate::parse()`] does with the hand-written one.
///
/// `filename` is recorded in every `Loc` and in the report; `None` becomes
/// `<string>`. On a refusal nothing the parse built stays in the arena.
pub fn parse(ast: &mut Ast, text: &str, filename: Option<&str>) -> Result<Vec<NodeId>, Refusal> {
    let file = filename.unwrap_or("<string>").to_string();
    let lexer = lexerdef().lexer(text);
    let lines = b::Lines::new(text);
    if let Some(at) = past_the_nesting_limit(&lexer) {
        let (line, col) = lines.line_col(at);
        return Err(Refusal {
            file,
            errors: vec![SyntaxError {
                line,
                col,
                what: format!(
                    "this `(` nests deeper than the limit of {NESTING_LIMIT} parentheses"
                ),
                repairs: Vec::new(),
                more_repairs: 0,
            }],
            cut: false,
            recovery_steps: 0,
        });
    }
    let mark = ast.mark();
    let cell = b::B::new(std::mem::take(ast), text, filename);
    let (forms, errors) = ein_y::parse(&lexer, &cell);
    *ast = cell.into_inner().ast;
    if errors.is_empty() {
        return Ok(forms.expect("a parse with no error built its forms"));
    }
    ast.rollback(mark);
    Err(report(file, text, &lexer, &lines, &errors))
}

/// Does the generated lexer read `text` as exactly one `NAME`, spelling all of
/// it? The name rule as the lexer applies it: a reserved word (`rule`) is a
/// token of its own and no `NAME`, and so is a run with a character the rule
/// leaves out. `ein-ir/tests/generated_parser.rs` holds every name the loader
/// makes up to it (`ein_core::text::INTERNAL_NAMES`, T1f.5.5.8).
pub fn is_one_name(text: &str) -> bool {
    let lexer = lexerdef().lexer(text);
    let lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>> = &lexer;
    let mut tokens = lexer.iter();
    let (Some(Ok(l)), None) = (tokens.next(), tokens.next()) else {
        return false;
    };
    l.span().start() == 0
        && l.span().len() == text.len()
        && lexerdef()
            .iter_rules()
            .find(|r| r.tok_id() == Some(l.tok_id()))
            .and_then(|r| r.name())
            == Some("NAME")
}

/// The byte offset of the first `(` that opens one level past
/// [`NESTING_LIMIT`], if any does. A lexing error ends nothing here: the
/// parse reports it.
fn past_the_nesting_limit(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
) -> Option<usize> {
    let mut depth = 0usize;
    for l in lexer.iter().flatten() {
        match lexer.span_str(l.span()) {
            "(" => {
                depth += 1;
                if depth > NESTING_LIMIT {
                    return Some(l.span().start());
                }
            }
            ")" => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    None
}

type Error = LexParseError<u32, DefaultLexerTypes<u32>>;

fn report(
    file: String,
    text: &str,
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    lines: &b::Lines,
    errors: &[Error],
) -> Refusal {
    let mut out = Vec::new();
    let (mut cut, mut steps) = (false, 0);
    for e in errors {
        out.push(match e {
            LexParseError::LexError(l) => {
                let at = l.span().start();
                let (line, col) = lines.line_col(at);
                let ch: String = text[at..].chars().take(1).collect();
                SyntaxError {
                    line,
                    col,
                    what: format!("no token starts with {}", shown(&ch)),
                    repairs: Vec::new(),
                    more_repairs: 0,
                }
            }
            LexParseError::ParseError(p) => {
                cut = p.recovery_cut();
                steps = p.recovery_steps();
                let l = p.lexeme();
                let (line, col) = lines.line_col(l.span().start());
                let what = match token_display(l.tok_id()) {
                    None => "unexpected end of input".to_string(),
                    Some((name, description)) => {
                        let found = shown(lexer.span_str(l.span()));
                        if description {
                            format!("unexpected {found}: {name}")
                        } else {
                            format!("unexpected {found}")
                        }
                    }
                };
                let ranked = p.repairs();
                SyntaxError {
                    line,
                    col,
                    what,
                    repairs: ranked
                        .iter()
                        .take(REPAIRS_SHOWN)
                        .map(|r| repair(lexer, r))
                        .collect(),
                    more_repairs: ranked.len().saturating_sub(REPAIRS_SHOWN),
                }
            }
        });
    }
    Refusal {
        file,
        errors: out,
        cut,
        recovery_steps: steps,
    }
}

/// How a report names a token: its `%epp` where that differs from its name,
/// so a description (`("a name", true)`), and otherwise its name (`(":why",
/// false)`, `(")", false)`); `None` for the end of the input, which no lexer
/// rule makes. A token's name is its lexer rule's, whose id `build.rs` set to
/// the parser's.
fn token_display(tok: u32) -> Option<(&'static str, bool)> {
    let name = lexerdef()
        .iter_rules()
        .find(|r| r.tok_id() == Some(tok))?
        .name()?;
    Some(match ein_y::token_epp(TIdx(tok)) {
        Some(epp) if epp != name => (epp, true),
        _ => (name, false),
    })
}

/// A token as a report names it: its own name in backticks (`` `:why` ``,
/// `` `)` ``), or its description (`a name`), or `end of input`.
fn token(tok: u32) -> String {
    match token_display(tok) {
        None => "end of input".to_string(),
        Some((name, true)) => name.to_string(),
        Some((name, false)) => format!("`{name}`"),
    }
}

/// One ranked repair, step by step: `Insert` a token, `Delete` or `Shift` one
/// of the file's.
fn repair(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    steps: &[ParseRepair<lrlex::DefaultLexeme<u32>, u32>],
) -> String {
    steps
        .iter()
        .map(|s| match s {
            ParseRepair::Insert(t) => format!("Insert {}", token(t.as_storaget())),
            ParseRepair::Delete(l) => format!("Delete {}", shown(lexer.span_str(l.span()))),
            ParseRepair::Shift(l) => format!("Shift {}", shown(lexer.span_str(l.span()))),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A token's text as a report quotes it: in backticks, a control character
/// escaped so that one error stays one line, and cut at 40 characters.
fn shown(text: &str) -> String {
    let mut out = String::from("`");
    for (n, c) in text.chars().enumerate() {
        if n == 40 {
            out.push('…');
            break;
        }
        match c {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('`');
    out
}

/// The hand-written parser's signature over the generated parser: the
/// `generated-parser` feature's hook in [`crate::parse()`]. The first error's
/// position is the [`ParseError`](crate::ParseError)'s, and the whole report
/// is its context.
pub(crate) fn parse_as_hand_written(
    ast: &mut Ast,
    text: &str,
    filename: Option<&str>,
) -> Result<Vec<NodeId>, crate::ParseError> {
    parse(ast, text, filename).map_err(|r| {
        let (line, col) = r
            .errors
            .first()
            .map_or((-1, -1), |e| (i64::from(e.line), i64::from(e.col)));
        crate::ParseError {
            file: r.file.clone(),
            line,
            col,
            context: r.to_string().trim_end().to_string(),
        }
    })
}
