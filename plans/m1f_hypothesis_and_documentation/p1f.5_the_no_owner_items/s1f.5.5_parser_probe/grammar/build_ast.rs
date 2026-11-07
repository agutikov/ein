//! The builder `ein.y`'s actions call: one call per action, each making the
//! node today's hand-written parser makes for the same text, so that the
//! loader reads the generated parser's tree unchanged.
//!
//! Probe code for S1f.5.5 step 2 (T1f.5.5.2). It is compiled twice: by the
//! probe's `c_ein`, against `ein-ir` read-only, and by a snapshot's `ein-ir`
//! (whose `lib.rs` says `extern crate self as ein_ir;`), so that a whole `ein`
//! binary can run on the generated parser. Its conventions are today's
//! (`parse.rs`): a declarator, `and`, `or`, `not` and `neq` are built with a
//! synthetic head and no `Loc`; any other list takes its head's `Loc`; a
//! top-level fact has none; `()` is `@empty` and a parameter list `@params`.
//!
//! Two leaves change, by the rulings this grammar carries: a run of digits is
//! a name, `Atom("01")` and not `Int(1)` (Q-M1e2.24), except as `:priority`'s
//! value, which stays `Int` until S1f.5.15 deletes the keyword; and a fact's
//! `:using` lists its premises under the synthetic head `@list`, where today a
//! list's head was the first element, which the loader then dropped.

use ein_ir::ast::{Ast, FileId, Loc, Node, NodeId};
use lrlex::{DefaultLexeme, DefaultLexerTypes};
use lrpar::{Lexeme, NonStreamingLexer};
use std::cell::RefCell;

pub type Id = NodeId;
type Tok = Result<DefaultLexeme<u32>, DefaultLexeme<u32>>;
type Lx<'a, 'i> = &'a dyn NonStreamingLexer<'i, DefaultLexerTypes<u32>>;

/// The state the actions share: the arena being built and the file's id.
pub struct B {
    pub ast: Ast,
    pub file: FileId,
}

impl B {
    pub fn new(mut ast: Ast, filename: Option<&str>) -> RefCell<B> {
        let file = ast.intern_file(filename);
        RefCell::new(B { ast, file })
    }
}

fn lexeme(t: Tok) -> DefaultLexeme<u32> {
    match t {
        Ok(l) | Err(l) => l,
    }
}

fn loc(p: &RefCell<B>, lx: Lx, l: DefaultLexeme<u32>) -> Option<Loc> {
    let ((line, col), _) = lx.line_col(l.span());
    Some(Loc { file: p.borrow().file, line: line as u32, col: col as u32 })
}

// ── Lists of ids ──────────────────────────────────────────────────────
pub fn none() -> Vec<Id> {
    Vec::new()
}
pub fn one(x: Id) -> Vec<Id> {
    vec![x]
}
pub fn push(mut v: Vec<Id>, x: Id) -> Vec<Id> {
    v.push(x);
    v
}
pub fn cat(mut v: Vec<Id>, w: Vec<Id>) -> Vec<Id> {
    v.extend(w);
    v
}

// ── Leaves ────────────────────────────────────────────────────────────
/// A name, at its position. A token recovery inserted has no text.
pub fn name(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, lx, l);
    let text = lx.span_str(l.span()).to_string();
    p.borrow_mut().ast.atom(&text, at)
}

/// `?name`, stored without its `?`.
pub fn var(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, lx, l);
    let text = lx.span_str(l.span());
    let mut b = p.borrow_mut();
    let s = b.ast.intern(text.get(1..).unwrap_or(""));
    b.ast.push(Node::Var(s), at)
}

/// A string, stored unescaped. The lexer let through only the escapes
/// S1f.5.2 names: `\n \t \r \" \\` and `\u{…}`.
pub fn text(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, lx, l);
    let raw = lx.span_str(l.span());
    let body = unescape(raw.get(1..raw.len().saturating_sub(1)).unwrap_or(""));
    let mut b = p.borrow_mut();
    let s = b.ast.intern(&body);
    b.ast.push(Node::Str(s), at)
}

/// `:priority`'s digits, as today's `Int`: the loader reads a rule's order
/// from it, and S1f.5.15 deletes the keyword.
pub fn count(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, lx, l);
    let digits = lx.span_str(l.span());
    let canon = ein_ir::ast::canonical_int(digits);
    let mut b = p.borrow_mut();
    let s = b.ast.intern(&canon);
    b.ast.push(Node::Int(s), at)
}

fn unescape(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut it = body.chars().peekable();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('u') => {
                let hex: String = it.by_ref().skip(1).take_while(|&h| h != '}').collect();
                // An out-of-range code point is T1f.5.5.3's to refuse with a line.
                out.push(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32).unwrap_or('\u{FFFD}'));
            }
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

// ── Forms ─────────────────────────────────────────────────────────────
/// `:key value`; the pair takes its key's position.
pub fn pair(p: &RefCell<B>, lx: Lx, key: Tok, value: Id) -> Id {
    let l = lexeme(key);
    let at = loc(p, lx, l);
    let text = lx.span_str(l.span());
    let mut b = p.borrow_mut();
    let s = b.ast.intern(text.get(1..).unwrap_or(""));
    let k = b.ast.push(Node::Keyword(s), at);
    b.ast.push(Node::KwPair { key: k, value }, at)
}

/// A form with a synthetic head and no position: a declarator, `and`, `or`,
/// `not`, `neq`.
pub fn named(p: &RefCell<B>, word: &str, items: Vec<Id>) -> Id {
    p.borrow_mut().ast.sform_named(word, &items, None)
}

/// `(head item…)` inside another form: it takes its head's position.
pub fn list(p: &RefCell<B>, head: Id, items: Vec<Id>) -> Id {
    let mut b = p.borrow_mut();
    let at = b.ast.loc(head);
    b.ast.sform(head, &items, at)
}

/// A top-level fact: its arguments, then its clauses, and no position.
pub fn top(p: &RefCell<B>, head: Id, args: Vec<Id>, clauses: Vec<Id>) -> Id {
    p.borrow_mut().ast.sform(head, &cat(args, clauses), None)
}

/// `()`.
pub fn empty(p: &RefCell<B>) -> Id {
    named(p, "@empty", none())
}

/// A rule's or a macro's parameter list.
pub fn params(p: &RefCell<B>, vars: Vec<Id>) -> Id {
    named(p, "@params", vars)
}

/// A fact's `:using`: its premises, each a fact.
pub fn premises(p: &RefCell<B>, facts: Vec<Id>) -> Id {
    named(p, "@list", facts)
}
