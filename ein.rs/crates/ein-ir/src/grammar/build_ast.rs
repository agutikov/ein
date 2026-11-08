//! The builder `ein.y`'s actions call: one call per action, each making the
//! node the hand-written parser makes for the same text, so that the loader
//! reads the generated parser's tree unchanged.
//!
//! S1f.5.5 step 2's builder (T1f.5.5.2), made real at T1f.5.5.3. Its
//! conventions are the hand-written parser's (`parse.rs`): a declarator,
//! `and`, `or`, `not` and `neq` are built with a synthetic head and no `Loc`;
//! any other list takes its head's `Loc`; a top-level fact has none; `()` is
//! `@empty` and a parameter list `@params`.
//!
//! Two leaves differ, by the rulings this grammar carries: a run of digits is
//! a name, `Atom("01")` and not `Int(1)` (Q-M1e2.24, S1f.5.4's ruling), so the
//! generated parser builds no `Int` at all, since `:priority`, whose value was
//! the one place digits stayed one, is not in the grammar (the user,
//! 2026-11-08: S1f.5.15 deletes it before the swap); and a fact's `:using`
//! lists its premises under the synthetic head `@list`, where the hand-written
//! parser took the list's first element as its head, which the loader then
//! dropped (the paper grammar's default 2). A macro's `:why`, which the
//! hand-written parser refuses, is a key pair after its body.
//!
//! A position is a `Loc` as the hand-written lexer counts one: lines split at
//! `\n` alone, columns in characters, both 1-based ([`Lines`]).

use std::cell::RefCell;

use lrlex::{DefaultLexeme, DefaultLexerTypes};
use lrpar::{Lexeme, NonStreamingLexer};

use crate::ast::{Ast, FileId, Loc, Node, NodeId};

pub(super) type Id = NodeId;
type Tok = Result<DefaultLexeme<u32>, DefaultLexeme<u32>>;
type Lx<'a, 'i> = &'a dyn NonStreamingLexer<'i, DefaultLexerTypes<u32>>;

/// Where each line of a source starts, so that a byte offset becomes the
/// hand-written lexer's line and column in O(log n): `lrlex`'s own
/// `line_col` walks the line from its start on every call.
pub(super) struct Lines {
    /// The byte offset of each line's first character.
    starts: Vec<usize>,
    /// For a source that is not all ASCII, the characters before each byte
    /// offset; empty for an ASCII one, where a byte is a character.
    chars_before: Vec<u32>,
}

impl Lines {
    pub(super) fn new(text: &str) -> Lines {
        let mut starts = vec![0];
        starts.extend(text.match_indices('\n').map(|(i, _)| i + 1));
        let mut chars_before = Vec::new();
        if !text.is_ascii() {
            // Only a character's first byte is ever asked for: a token starts
            // on one.
            chars_before = vec![0; text.len() + 1];
            let mut n = 0;
            for (i, _) in text.char_indices() {
                chars_before[i] = n;
                n += 1;
            }
            chars_before[text.len()] = n;
        }
        Lines {
            starts,
            chars_before,
        }
    }

    /// The 1-based line and column of byte offset `at`.
    pub(super) fn line_col(&self, at: usize) -> (u32, u32) {
        let line = self.starts.partition_point(|&s| s <= at) - 1;
        let start = self.starts[line];
        let col = if self.chars_before.is_empty() {
            at - start
        } else {
            (self.chars_before[at] - self.chars_before[start]) as usize
        };
        (line as u32 + 1, col as u32 + 1)
    }
}

/// The state the actions share: the arena being built, the file's id, and
/// where the lines start.
pub(super) struct B {
    pub(super) ast: Ast,
    pub(super) file: FileId,
    pub(super) lines: Lines,
}

impl B {
    pub(super) fn new(mut ast: Ast, text: &str, filename: Option<&str>) -> RefCell<B> {
        let file = ast.intern_file(filename);
        RefCell::new(B {
            ast,
            file,
            lines: Lines::new(text),
        })
    }
}

/// A token as an action receives it: `Err` for one recovery inserted, whose
/// span is empty.
fn lexeme(t: Tok) -> DefaultLexeme<u32> {
    match t {
        Ok(l) | Err(l) => l,
    }
}

fn loc(p: &RefCell<B>, l: DefaultLexeme<u32>) -> Option<Loc> {
    let b = p.borrow();
    let (line, col) = b.lines.line_col(l.span().start());
    Some(Loc {
        file: b.file,
        line,
        col,
    })
}

// ── Lists of ids ──────────────────────────────────────────────────────

pub(super) fn none() -> Vec<Id> {
    Vec::new()
}

pub(super) fn one(x: Id) -> Vec<Id> {
    vec![x]
}

pub(super) fn push(mut v: Vec<Id>, x: Id) -> Vec<Id> {
    v.push(x);
    v
}

pub(super) fn cat(mut v: Vec<Id>, w: Vec<Id>) -> Vec<Id> {
    v.extend(w);
    v
}

// ── Leaves ────────────────────────────────────────────────────────────

/// A name, at its position. A token recovery inserted has no text.
pub(super) fn name(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, l);
    let text = lx.span_str(l.span());
    p.borrow_mut().ast.atom(text, at)
}

/// `?name`, stored without its `?`.
pub(super) fn var(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, l);
    let text = lx.span_str(l.span());
    let mut b = p.borrow_mut();
    let s = b.ast.intern(text.get(1..).unwrap_or(""));
    b.ast.push(Node::Var(s), at)
}

/// A string, stored unescaped. The lexer let through only the escapes
/// S1f.5.2 names: `\n \t \r \" \\` and `\u{…}`.
pub(super) fn text(p: &RefCell<B>, lx: Lx, t: Tok) -> Id {
    let l = lexeme(t);
    let at = loc(p, l);
    let raw = lx.span_str(l.span());
    let body = unescape(raw.get(1..raw.len().saturating_sub(1)).unwrap_or(""));
    let mut b = p.borrow_mut();
    let s = b.ast.intern(&body);
    b.ast.push(Node::Str(s), at)
}

/// A string's body with its escapes read. `\u{…}` beyond Unicode's range is
/// U+FFFD here; refusing it with a line is S1f.5.2's, at the swap.
fn unescape(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut it = body.chars();
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
                out.push(
                    u32::from_str_radix(&hex, 16)
                        .ok()
                        .and_then(char::from_u32)
                        .unwrap_or('\u{FFFD}'),
                );
            }
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

// ── Forms ─────────────────────────────────────────────────────────────

/// `:key value`; the pair takes its key's position.
pub(super) fn pair(p: &RefCell<B>, lx: Lx, key: Tok, value: Id) -> Id {
    let l = lexeme(key);
    let at = loc(p, l);
    let text = lx.span_str(l.span());
    let mut b = p.borrow_mut();
    let s = b.ast.intern(text.get(1..).unwrap_or(""));
    let k = b.ast.push(Node::Keyword(s), at);
    b.ast.push(Node::KwPair { key: k, value }, at)
}

/// A form with a synthetic head and no position: a declarator, `and`, `or`,
/// `not`, `neq`.
pub(super) fn named(p: &RefCell<B>, word: &str, items: Vec<Id>) -> Id {
    p.borrow_mut().ast.sform_named(word, &items, None)
}

/// `(head item…)` inside another form: it takes its head's position.
pub(super) fn list(p: &RefCell<B>, head: Id, items: Vec<Id>) -> Id {
    let mut b = p.borrow_mut();
    let at = b.ast.loc(head);
    b.ast.sform(head, &items, at)
}

/// A top-level fact: its arguments, then its clauses, and no position.
pub(super) fn top(p: &RefCell<B>, head: Id, args: Vec<Id>, clauses: Vec<Id>) -> Id {
    p.borrow_mut().ast.sform(head, &cat(args, clauses), None)
}

/// `()`.
pub(super) fn empty(p: &RefCell<B>) -> Id {
    named(p, "@empty", none())
}

/// A rule's or a macro's parameter list.
pub(super) fn params(p: &RefCell<B>, vars: Vec<Id>) -> Id {
    named(p, "@params", vars)
}

/// A fact's `:using`: its premises, each a fact.
pub(super) fn premises(p: &RefCell<B>, facts: Vec<Id>) -> Id {
    named(p, "@list", facts)
}

#[cfg(test)]
mod tests {
    use super::Lines;

    /// The hand-written lexer's count: `\n` alone ends a line, a column is a
    /// character, and both start at 1.
    #[test]
    fn a_position_is_counted_as_the_hand_written_lexer_counts_it() {
        let ascii = Lines::new("(a b)\n  (c)\r\n(d)");
        assert_eq!(ascii.line_col(0), (1, 1));
        assert_eq!(ascii.line_col(3), (1, 4));
        assert_eq!(ascii.line_col(8), (2, 3));
        assert_eq!(ascii.line_col(13), (3, 1));
        let text = "(p \"Жук\" a)\n(é b)";
        let wide = Lines::new(text);
        // `a` is the 10th character of line 1, at byte 12.
        let a = text.find(" a").unwrap() + 1;
        assert_eq!((a, wide.line_col(a)), (12, (1, 10)));
        // `b` is the 4th character of line 2.
        let b = text.find(" b").unwrap() + 1;
        assert_eq!(wide.line_col(b), (2, 4));
    }
}
