//! The parser: S1f.5.5's grammar, `src/ein.l` and `src/ein.y` in grmtools'
//! notation, compiled by `build.rs` (M1f S1f.5.5). Since the swap,
//! T1f.5.5.4, it is the only parser: [`crate::parse()`] is [`parse`], and the
//! hand-written lexer and parser, `lex.rs` and `parse.rs`, are gone.
//!
//! **What it builds is the tree the loader reads**, node for node and
//! [`Loc`](crate::Loc) for `Loc`, through one builder call per grammar action
//! (`grammar/build_ast.rs`): the tree the hand-written parser built, where the
//! two languages agree, and where they differ, by a ruling of this phase,
//! each attributed file by file before the swap (T1f.5.5.9,
//! `plans/…/s1f.5.5_attribution.md`).
//!
//! **A refused file reports every error recovery reaches** ([`ParseError`]),
//! each with its true position, what was found there, its notes, and at most
//! three of the repairs CPCT+ ranked, the first being the one the parse went
//! on with, then a count of the rest. Where a place takes a name, the ten
//! words a position gives a meaning to ([`POSITION_WORDS`]) are names there
//! too, so the repairs that differ only in which of them, or which name, they
//! insert are one repair, *Insert a name* (the user's ruling of 2026-11-08 on
//! the review stop's item 6). Recovery is deterministic: `lrpar` is the
//! patched copy under `ein.rs/vendor/lrpar` (F11), which ranks equal repairs
//! in the order its search found them and stops at a count of search steps,
//! [`RECOVERY_STEP_BUDGET`], rather than at a wall-clock time; a report cut
//! there says so after its last error. A file that parses pays nothing for
//! recovery: the recoverer is built at the first error. **A refused file stays
//! refused**: the repairs are advice, and no repaired program is loaded.
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
use ein_core::config::RETIRED;

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

/// The thirteen words that are never a name: tokens of their own in `ein.l`,
/// each a declarator, a kernel connective, or `open`, the verdict, which is
/// written only as a rule's whole `:assert`, `(open)` or `(open ?R)` (`ein.y`'s
/// `Assert`; M1f S1f.5.23, the user's ruling on Q-M1f.66: a position word
/// until then). `ein_core::text::LEXER_KEYWORDS` is the printer's copy, which
/// `the_reserved_words_are_the_printers` holds to this one.
pub const RESERVED_WORDS: [&str; 13] = [
    "not", "and", "or", "neq", "rule", "hrule", "query", "solve", "config", "trace", "macro",
    "import", "open",
];

/// The ten words a position gives a meaning to (the paper grammar's
/// default 1): tokens of their own in `ein.l`, and names wherever a name goes
/// (`ein.y`'s `Name`), so `(p model)` is a fact and `(relation step T)` a
/// declaration. `open` was the eleventh until M1f S1f.5.23.
pub const POSITION_WORDS: [&str; 10] = [
    "relation", "step", "eq", "absent", "false", "true", "model", "region", "merged", "owing",
];

/// What a parse error found at an `open` says (M1f S1f.5.23, the user's
/// ruling on Q-M1f.66, *"Refuse it"*): the one place it is written, and that
/// it names nothing.
pub const OPEN_NOTE: &str = "open is reserved: it is the verdict a rule's :assert states, written as the \
     whole :assert, (open) or (open ?R), and nowhere else, so it names no relation and no value";

/// Where a character that is not ASCII may be written, as the line that
/// refuses one anywhere else says it (M1f S1f.5.22, the user's ruling of
/// 2026-11-10, *"Unicode is not allowed anywhere else than inside string
/// literals, and the error must say it explicitely"*, and Q-M1f.65 (b): a
/// comment may hold Unicode). [`not_ascii`] writes the line.
pub const UNICODE_ALLOWED: &str = "Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment";

/// The line a character that is not ASCII is refused with, outside a string
/// literal and a comment (M1f S1f.5.22): the character, quoted, and its code
/// point, then where Unicode is allowed ([`UNICODE_ALLOWED`]).
///
/// ```text
/// 'é' (U+00E9) is not ASCII: Unicode is allowed only inside a string literal (after :why, :source or :goal-text) or in a comment
/// ```
///
/// A character a reader cannot see where it stands, a space, a control, a
/// zero-width or a bidirectional format character, a combining mark or a
/// variation selector, is quoted as the escape a string writes it with,
/// `'\u{a0}' (U+00A0)`, so that the line shows what it names and one error
/// stays one line ([`shown_char`], Q-M1f.92).
pub fn not_ascii(c: char) -> String {
    format!(
        "'{}' (U+{:04X}) is not ASCII: {UNICODE_ALLOWED}",
        shown_char(c),
        c as u32
    )
}

/// A character as [`not_ascii`] quotes it: itself, or where a reader cannot
/// see it, the `\u{…}` escape a string writes it with.
pub fn shown_char(c: char) -> String {
    let invisible = c.is_whitespace()
        || c.is_control()
        || matches!(c as u32,
            0x00AD | 0x034F | 0x061C | 0x115F | 0x1160 | 0x17B4 | 0x17B5 | 0x3164 | 0xFEFF | 0xFFA0
            | 0x0300..=0x036F
            | 0x180B..=0x180F
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x200B..=0x200F
            | 0x202A..=0x202E
            | 0x2060..=0x206F
            | 0x20D0..=0x20FF
            | 0xFE00..=0xFE0F
            | 0xFE20..=0xFE2F
            | 0xFFF9..=0xFFFB
            | 0x1D173..=0x1D17A
            | 0xE0000..=0xE007F
            | 0xE0100..=0xE01EF);
    if invisible {
        format!("\\u{{{:x}}}", c as u32)
    } else {
        c.to_string()
    }
}

/// The `(query …)` keywords that choose the lattice, which belong on
/// `(solve …)` (M1e2 S1e2.1a.4): a query that writes one is refused at the
/// keyword, with a note saying where it goes (the paper grammar's default 5).
const LATTICE_KEYWORDS: [&str; 4] = [
    ":hrules",
    ":hypothesis-relations",
    ":no-hypothesis",
    ":no-search",
];

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
    /// What the error means, where a ruling of this phase says more than the
    /// token does: a bare `_` (S1f.5.1), a string as a value (S1f.5.2), a
    /// query's lattice keyword and a retired `(config …)` flag (the paper
    /// grammar's default 5).
    pub notes: Vec<String>,
    /// The first [`REPAIRS_SHOWN`] repairs recovery ranked, each a sequence of
    /// steps, synonyms folded into one ([`crate::grammar`]); the first is the
    /// one the parse went on with.
    pub repairs: Vec<String>,
    /// How many ranked repairs the report leaves out, synonyms folded.
    pub more_repairs: usize,
}

/// A refused file: every error recovery reached, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub file: String,
    pub errors: Vec<SyntaxError>,
    /// Recovery stopped at [`RECOVERY_STEP_BUDGET`] at the last error, so the
    /// file may hold errors past it that the report does not list.
    pub cut: bool,
    /// The search steps recovery took over the parse, at most
    /// [`RECOVERY_STEP_BUDGET`].
    pub recovery_steps: u64,
}

impl ParseError {
    /// The first error's line and column: where the file stops being Ein.
    pub fn position(&self) -> (u32, u32) {
        self.errors.first().map_or((0, 0), |e| (e.line, e.col))
    }
}

impl fmt::Display for ParseError {
    /// One line an error, `file:line:col: what`, then its notes and repairs
    /// indented under it, and, where recovery was cut, a last line saying so.
    /// No trailing newline.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = Vec::new();
        for e in &self.errors {
            lines.push(format!("{}:{}:{}: {}", self.file, e.line, e.col, e.what));
            for n in &e.notes {
                lines.push(format!("  note: {n}"));
            }
            for r in &e.repairs {
                lines.push(format!("  repair: {r}"));
            }
            if e.more_repairs > 0 {
                let s = if e.more_repairs == 1 { "" } else { "s" };
                lines.push(format!("  and {} more repair{s}", e.more_repairs));
            }
        }
        if self.cut
            && let Some(last) = self.errors.last()
        {
            lines.push(format!(
                "{}:{}:{}: recovery stopped here, at its budget of {} steps: \
                 errors after this one are not reported",
                self.file, last.line, last.col, RECOVERY_STEP_BUDGET
            ));
        }
        f.write_str(&lines.join("\n"))
    }
}

impl std::error::Error for ParseError {}

type Def = LRNonStreamingLexerDef<DefaultLexerTypes<u32>>;

/// The lexer, built once: its rules are regular expressions, compiled when
/// it is made. Read here for its rules; a parse lexes with [`LEXER`].
fn lexerdef() -> &'static Def {
    static DEF: OnceLock<Def> = OnceLock::new();
    DEF.get_or_init(ein_l::lexerdef)
}

thread_local! {
    /// This thread's copy of [`lexerdef`], which every lex goes through
    /// (T1f.5.5.12). A clone shares the compiled regular expressions and has
    /// its own pool of their search caches, and a pool hands its cache out
    /// cheaply only to the first thread that used it: every other thread
    /// takes a lock per search, and a fresh cache where the lock is taken.
    /// lrlex searches with every rule at every position, so a thread lexing
    /// through another thread's pool lexed 1.8 times slower alone and 3.1
    /// times slower beside seven others (the T12 record), which is every
    /// parse of a test thread but the first.
    static LEXER: Def = lexerdef().clone();
}

/// Parse `text` into its top-level forms, appending them to `ast`.
///
/// `filename` is recorded in every `Loc` and in the report; `None` becomes
/// `<string>`. On a refusal nothing the parse built stays in the arena, and
/// the report holds every error recovery reached.
pub fn parse(ast: &mut Ast, text: &str, filename: Option<&str>) -> Result<Vec<NodeId>, ParseError> {
    ein_core::counters::bump(|k| {
        k.parse_call += 1;
        k.parse_bytes += text.len() as u64;
    });
    LEXER.with(|def| parse_with(def, ast, text, filename))
}

fn parse_with(
    def: &Def,
    ast: &mut Ast,
    text: &str,
    filename: Option<&str>,
) -> Result<Vec<NodeId>, ParseError> {
    let file = filename.unwrap_or("<string>").to_string();
    let lexer = def.lexer(text);
    let lines = b::Lines::new(text);
    if let Some(at) = past_the_nesting_limit(&lexer) {
        let (line, col) = lines.line_col(at);
        return Err(ParseError {
            file,
            errors: vec![SyntaxError {
                line,
                col,
                what: format!(
                    "this `(` nests deeper than the limit of {NESTING_LIMIT} parentheses"
                ),
                notes: Vec::new(),
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

/// Does the lexer read `text` as exactly one `NAME`, spelling all of it? The
/// name rule as the lexer applies it: a reserved word (`rule`) is a token of
/// its own and no `NAME`, and so is a run with a character the rule leaves
/// out. The printer's statement of it, `ein_core::text::is_symbol`, is held
/// to this one (`the_printers_symbol_is_the_lexers`), and every name the
/// loader makes up is held not to be one (`ein_core::text::INTERNAL_NAMES`,
/// T1f.5.5.8).
pub fn is_one_name(text: &str) -> bool {
    LEXER.with(|def| {
        let lexer = def.lexer(text);
        let lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>> = &lexer;
        let mut tokens = lexer.iter();
        let (Some(Ok(l)), None) = (tokens.next(), tokens.next()) else {
            return false;
        };
        l.span().start() == 0
            && l.span().len() == text.len()
            && rule_name(l.tok_id()) == Some("NAME")
    })
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
) -> ParseError {
    let mut out = Vec::new();
    let (mut cut, mut steps) = (false, 0);
    for e in errors {
        out.push(match e {
            LexParseError::LexError(l) => {
                let mut at = l.span().start();
                let mut chars = text[at..].chars();
                let first = chars.next();
                // `#` and a character that is not ASCII: no rule starts at the
                // `#` (BAD takes `#` before an ASCII character alone), and the
                // character is what the line is about.
                if first == Some('#') && chars.next().is_some_and(|c| !c.is_ascii()) {
                    at += 1;
                }
                let (line, col) = lines.line_col(at);
                let what = match text[at..].chars().next() {
                    // M1f S1f.5.22: no rule but a string's and the skip rule's
                    // comments matches a character that is not ASCII.
                    Some(c) if !c.is_ascii() => not_ascii(c),
                    c => format!(
                        "no token starts with {}",
                        shown(&c.map(String::from).unwrap_or_default())
                    ),
                };
                SyntaxError {
                    line,
                    col,
                    what,
                    notes: Vec::new(),
                    repairs: Vec::new(),
                    more_repairs: 0,
                }
            }
            LexParseError::ParseError(p) => {
                cut = p.recovery_cut();
                steps = p.recovery_steps();
                let l = p.lexeme();
                // The end of the input is where the text ends: `lrpar` makes
                // its end-of-input lexeme at the end of the last token, so
                // `(p a` and a newline reported `1:5` where the input ends at
                // `2:1` (M1f S1f.5.5, T1f.5.5.13).
                let at = match token_display(l.tok_id()) {
                    None => text.len(),
                    Some(_) => l.span().start(),
                };
                let (line, col) = lines.line_col(at);
                let found = lexer.span_str(l.span());
                let what = match token_display(l.tok_id()) {
                    None => "unexpected end of input".to_string(),
                    Some((name, description)) => {
                        if description {
                            format!("unexpected {}: {name}", shown(found))
                        } else {
                            format!("unexpected {}", shown(found))
                        }
                    }
                };
                let ranked = folded(lexer, p.repairs());
                SyntaxError {
                    line,
                    col,
                    what,
                    notes: notes(lexer, lines, l, found),
                    repairs: ranked.iter().take(REPAIRS_SHOWN).cloned().collect(),
                    more_repairs: ranked.len().saturating_sub(REPAIRS_SHOWN),
                }
            }
        });
    }
    ParseError {
        file,
        errors: out,
        cut,
        recovery_steps: steps,
    }
}

/// A lexer rule's name, by its token id, which `build.rs` set to the
/// parser's: `NAME`, `TEXT`, `UNDERSCORE`, or the token's own text (`:why`,
/// `(`, `model`); `None` for the end of the input, which no rule makes.
fn rule_name(tok: u32) -> Option<&'static str> {
    lexerdef()
        .iter_rules()
        .find(|r| r.tok_id() == Some(tok))?
        .name()
}

/// What a ruling of this phase says about the token an error found, beyond
/// the token itself: the review stop's item 3 (S1f.5.1's and S1f.5.2's
/// sentences), the paper grammar's default 5 (the loader's two lines a
/// keyword's position no longer reaches), on a string with an escape it does
/// not read, each such escape and where it is (S1f.5.2's escapes, T1f.5.2.5),
/// at an `open`, where the verdict is written (S1f.5.23, [`OPEN_NOTE`]), and,
/// since M1f S1f.7.7, S1f.7.6 and S1f.7.10, at a word the value rule keeps out
/// of a value: an `absent` that opens a form ([`ABSENT_NOTE`]), an `eq` or a
/// `neq` that opens a form inside another, and an argument written after
/// `(false` ([`FALSE_NOTE`]).
fn notes(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    lines: &b::Lines,
    at: &lrlex::DefaultLexeme<u32>,
    found: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    match rule_name(at.tok_id()) {
        Some("UNDERSCORE" | "VARUNDERSCORE") => out.push(format!(
            "{found} is reserved and names nothing: where any value will do, write a variable \
             used once"
        )),
        Some("TEXT") => out.push(format!(
            "{} is a string, and a string is natural-language text, written only after :why, \
             :source or :goal-text: write a name bare",
            cut_at_40(found)
        )),
        Some("BADESCAPE") => {
            let body = found
                .strip_prefix('"')
                .and_then(|f| f.strip_suffix('"'))
                .unwrap_or("");
            for (offset, escape) in b::bad_escapes(body) {
                let (line, col) = lines.line_col(at.span().start() + 1 + offset);
                out.push(format!(
                    "{}, at {line}:{col}, is no escape a string reads: a string's escapes are \
                     \\n, \\t, \\r, \\\", \\\\ and \\u{{…}}, one to six hex digits naming a \
                     Unicode scalar value, 0 to 10FFFF and not D800 to DFFF",
                    cut_at_40(&escape)
                ));
            }
        }
        Some(k) if LATTICE_KEYWORDS.contains(&k) && top_form(lexer, at) == Some("query") => {
            out.push(format!(
                "{k} chooses the lattice and belongs on (solve …) since M1e2 S1e2.1a.4 — one \
                 program, one lattice"
            ));
        }
        Some(")") => out.extend(empty_connective(lexer, at)),
        // A value after `(solve :no-search …)`, the one keyword that takes none
        // (M1f S1f.6.36).
        _ if !found.starts_with(':')
            && found != ")"
            && written_before(lexer, at, 1) == [":no-search"] =>
        {
            out.push(
                ":no-search is a flag and takes no value: it states that the lattice is root \
                 alone"
                    .to_string(),
            )
        }
        Some("open") => out.push(OPEN_NOTE.to_string()),
        Some("absent") if written_before(lexer, at, 1) == ["("] => {
            out.push(ABSENT_NOTE.to_string())
        }
        Some(k @ ("eq" | "neq")) if nested_head(lexer, at) => out.push(format!(
            "{k} is computed and never stored, so ({k} a b) is no value: write it as a \
                 premise or a conclusion of its own, where it is checked, and never inside \
                 another fact or pattern"
        )),
        Some(k)
            if !k.starts_with(':')
                && k != ")"
                && written_before(lexer, at, 2) == ["(", "false"] =>
        {
            out.push(FALSE_NOTE.to_string())
        }
        _ if found.starts_with(':') && top_form(lexer, at) == Some("config") => {
            if let Some((flag, stage, why)) = RETIRED.iter().find(|(f, _, _)| *f == &found[1..]) {
                out.push(format!(
                    "config flag :{flag} was retired at M1e2 {stage}: {why}"
                ));
            }
        }
        _ => {}
    }
    out
}

/// What an `absent` that is no premise's connective is told (M1f S1f.7.7): a
/// fact, a value or a conclusion headed `absent` is refused, and the author who
/// wrote one where a negation was meant is pointed at `(not …)`.
const ABSENT_NOTE: &str = "absent is negation as failure, a test a premise makes of a state, \
                           (absent <premise>) in a :match or a :goal, so no fact, value or \
                           conclusion is of it: the stored negation of a fact is (not …)";

/// What an argument written after `false` is told (M1f S1f.7.6).
const FALSE_NOTE: &str = "false takes no argument: (false) says that the state is impossible, \
                          and names nothing";

/// Is the lexeme `at` the head of a form written inside another form?
fn nested_head(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    at: &lrlex::DefaultLexeme<u32>,
) -> bool {
    let (mut depth, mut last) = (0usize, "");
    for l in lexer.iter().flatten() {
        if l.span().start() >= at.span().start() {
            break;
        }
        last = lexer.span_str(l.span());
        match last {
            "(" => depth += 1,
            ")" => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    last == "(" && depth >= 2
}

/// The last `n` tokens written before the lexeme `at`, as written; fewer where
/// fewer were written.
fn written_before<'i>(
    lexer: &dyn NonStreamingLexer<'i, DefaultLexerTypes<u32>>,
    at: &lrlex::DefaultLexeme<u32>,
    n: usize,
) -> Vec<&'i str> {
    let before: Vec<&str> = lexer
        .iter()
        .flatten()
        .take_while(|l| l.span().start() < at.span().start())
        .map(|l| lexer.span_str(l.span()))
        .collect();
    before[before.len().saturating_sub(n)..].to_vec()
}

/// `(and)` and `(or)`, which the grammar refuses, `and` and `or` each taking
/// one premise or more (M1f S1f.5.9, the user's ruling of 2026-11-07 on F12:
/// *"reject (and) while it is the same as () and `and` should have
/// arguments"*): where the `)` closes one, the note names the rule and the
/// clause it is written in, and says that the empty conjunction a `:match`
/// holds, or the empty conclusion an `:assert` holds, is written `()`.
fn empty_connective(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    at: &lrlex::DefaultLexeme<u32>,
) -> Option<String> {
    let before: Vec<&str> = lexer
        .iter()
        .flatten()
        .take_while(|l| l.span().start() < at.span().start())
        .map(|l| lexer.span_str(l.span()))
        .collect();
    let [.., "(", word] = before.as_slice() else {
        return None;
    };
    let word = *word;
    if word != "and" && word != "or" {
        return None;
    }
    // The top-level form, the name a rule binds, and the clause written at
    // its own depth last.
    let (mut depth, mut head, mut name, mut clause) = (0usize, None, None, None);
    for (i, &t) in before.iter().enumerate() {
        match t {
            "(" => {
                depth += 1;
                if depth == 1 {
                    head = before.get(i + 1).copied();
                    name = before.get(i + 2).copied();
                    clause = None;
                }
            }
            ")" => depth = depth.saturating_sub(1),
            _ if depth == 1 && t.starts_with(':') => clause = Some(t),
            _ => {}
        }
    }
    let place = match (head, name, clause) {
        (Some(h @ ("rule" | "hrule")), Some(n), Some(c)) => format!("{h} '{n}''s {c}"),
        (Some(h @ ("query" | "solve")), _, Some(c)) => format!("the {h}'s {c}"),
        (Some("macro"), Some(n), _) => format!("macro '{n}''s template"),
        _ => "this form".to_string(),
    };
    // `()` stands for the whole clause only: `(and)` written as the clause's
    // value, not inside another connective.
    let whole = matches!(before.as_slice(), [.., c, "(", _] if Some(*c) == clause);
    Some(match (word, head, clause, whole) {
        ("and", Some("rule" | "hrule"), Some(":match"), true) => format!(
            "{place} writes (and), the empty conjunction, whose one spelling is () — and takes \
             one premise or more"
        ),
        ("and", Some("rule" | "hrule"), Some(":assert"), true) => format!(
            "{place} writes (and), the empty conclusion, whose one spelling is () — and takes \
             one conclusion or more"
        ),
        ("and", Some("rule" | "hrule"), Some(":match"), false) => {
            format!("{place} writes (and) — and takes one premise or more")
        }
        ("and", Some("rule" | "hrule"), Some(":assert"), false) => {
            format!("{place} writes (and) — and takes one conclusion or more")
        }
        ("and", ..) => format!("{place} writes (and) — and takes one argument or more"),
        _ => format!("{place} writes (or) — or takes one disjunct or more"),
    })
}

/// The head of the top-level form the lexeme `at` stands in, as written:
/// `query`, `config`, a fact's relation; `None` at the top level.
fn top_form<'i>(
    lexer: &dyn NonStreamingLexer<'i, DefaultLexerTypes<u32>>,
    at: &lrlex::DefaultLexeme<u32>,
) -> Option<&'i str> {
    let (mut depth, mut head, mut opened) = (0usize, None, false);
    for l in lexer.iter().flatten() {
        if l.span().start() >= at.span().start() {
            break;
        }
        let t = lexer.span_str(l.span());
        if opened && depth == 1 {
            head = Some(t);
        }
        opened = false;
        match t {
            "(" => {
                depth += 1;
                opened = depth == 1;
            }
            ")" => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    head = None;
                }
            }
            _ => {}
        }
    }
    if depth == 0 { None } else { head }
}

/// How a report names a token: its `%epp` where that differs from its name,
/// so a description (`("a name", true)`), and otherwise its name (`(":why",
/// false)`, `(")", false)`); `None` for the end of the input.
fn token_display(tok: u32) -> Option<(&'static str, bool)> {
    let name = rule_name(tok)?;
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

/// The ranked repairs as a report lists them, best first, with synonyms
/// folded (the review stop's item 6): where a place takes a name, each of the
/// [`POSITION_WORDS`] is a name there too, so repairs that differ only in which
/// of them, or which name, they insert are one repair, *Insert a name*, listed
/// at the rank of the first of them. A place takes a name where a repair
/// inserts one, or where four of the words or more are repairs there: every
/// production of `ein.y` that takes four of them takes `NAME` too, and the
/// places that take fewer and no name (a flag's `true` or `false`, a claim's
/// `model`, `merged` or `owing`, a solve's `region`) keep their words, each
/// listed as itself. CPCT+ often ranks the words and not the name where both
/// are repairs, which is what this folds.
fn folded(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    ranked: &[Vec<ParseRepair<lrlex::DefaultLexeme<u32>, u32>>],
) -> Vec<String> {
    let written: Vec<(String, String)> = ranked
        .iter()
        .map(|r| (repair(lexer, r, false), repair(lexer, r, true)))
        .collect();
    let is_a_name = |key: &String| {
        let synonyms = written.iter().filter(|(_, k)| k == key).count();
        written.iter().any(|(as_is, _)| as_is == key) || synonyms >= 4
    };
    let mut out: Vec<String> = Vec::new();
    for (as_is, as_name) in &written {
        let shown = if as_is != as_name && is_a_name(as_name) {
            as_name
        } else {
            as_is
        };
        if !out.contains(shown) {
            out.push(shown.clone());
        }
    }
    out
}

/// One ranked repair, step by step: `Insert` a token, `Delete` or `Shift` one
/// of the file's; with `fold`, each of the [`POSITION_WORDS`] it inserts
/// written as the name it also is.
fn repair(
    lexer: &dyn NonStreamingLexer<'_, DefaultLexerTypes<u32>>,
    steps: &[ParseRepair<lrlex::DefaultLexeme<u32>, u32>],
    fold: bool,
) -> String {
    steps
        .iter()
        .map(|s| match s {
            ParseRepair::Insert(t) => {
                let t = t.as_storaget();
                if fold && rule_name(t).is_some_and(|n| POSITION_WORDS.contains(&n)) {
                    "Insert a name".to_string()
                } else {
                    format!("Insert {}", token(t))
                }
            }
            ParseRepair::Delete(l) => format!("Delete {}", shown(lexer.span_str(l.span()))),
            ParseRepair::Shift(l) => format!("Shift {}", shown(lexer.span_str(l.span()))),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A token's text cut at 40 characters, a control character escaped so that
/// one error stays one line.
fn cut_at_40(text: &str) -> String {
    let mut out = String::new();
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
    out
}

/// A token's text as a report quotes it: in backticks, [cut](cut_at_40).
fn shown(text: &str) -> String {
    format!("`{}`", cut_at_40(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The printer's list of the words that are never a name is the
    /// lexer's.
    #[test]
    fn the_reserved_words_are_the_printers() {
        assert_eq!(ein_core::text::LEXER_KEYWORDS, RESERVED_WORDS);
        for w in RESERVED_WORDS {
            assert!(!is_one_name(w), "{w}");
            assert!(is_one_name(&format!("{w}-1")), "{w}-1");
        }
        for w in POSITION_WORDS {
            assert!(!is_one_name(w), "{w} is a token of its own");
        }
    }

    /// M1e2 S1e2.3.8, restated at the swap (T1f.5.5.4): the one printer
    /// prints a name bare exactly where this lexer reads the bare text back
    /// as one `NAME` spelling it, or as one of the [`POSITION_WORDS`], which
    /// are names wherever a name goes ([`ein_core::text::is_symbol`]), over
    /// every reserved and position word, each followed and preceded by every
    /// character class the name rule tests, and every ASCII character in
    /// every position of a short name.
    #[test]
    fn the_printers_symbol_is_the_lexers() {
        let mut names: Vec<String> = Vec::new();
        let tails = [
            "", "x", "X", "1", "_", "-", ".", "*", "-1", "?", ":", " ", "é", "١", "(", "\"", "#",
            "|", "'", "[", "}",
        ];
        for w in RESERVED_WORDS.iter().chain(&POSITION_WORDS) {
            for t in tails {
                names.push(format!("{w}{t}"));
                names.push(format!("__{w}{t}"));
                names.push(format!("{t}{w}"));
            }
        }
        for a in (0u8..128).map(char::from) {
            for b in ["", "a", "1", "-", "_"] {
                names.push(format!("{a}{b}"));
                names.push(format!("a{a}{b}"));
                names.push(format!("__{a}{b}"));
            }
        }
        names.extend(
            [
                "",
                "Åsa",
                "aÅ",
                "1..5",
                "1..*",
                "?x",
                "_",
                "__",
                "___x",
                "7",
                "07",
                "-7",
                "it's",
                "f'",
                "[a]",
                "{}",
                "a#b",
                "#empty",
                "<forced-positive>",
                "<forced-positive>x",
            ]
            .map(String::from),
        );
        let mut bare = 0;
        for n in &names {
            let reads_back = is_one_name(n) || POSITION_WORDS.contains(&n.as_str());
            assert_eq!(ein_core::text::is_symbol(n), reads_back, "{n:?}");
            bare += usize::from(reads_back);
        }
        assert!(
            bare > 100 && bare < names.len() - 100,
            "{bare} of {}",
            names.len()
        );
    }
}
