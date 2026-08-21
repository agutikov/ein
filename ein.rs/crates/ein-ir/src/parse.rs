//! Recursive descent over the grammar
//! ([`01_grammar.md` §3](../../../../docs/kernel/ir/03-ein-lang/01_grammar.md),
//! `grammar.lark` until S1a.10.5), with the two ambiguities Earley
//! resolves implicitly resolved explicitly.
//!
//! **Backtracking, not lookahead.** Every production begins `"(" LITERAL`, and
//! the literal can also be the prefix of a longer `SYMBOL` (§[`crate::lex`]),
//! so the head of a form does not determine its production: `(rulex …)` is a
//! `rule_decl` named `x` when the rest of the form fits one, and an ordinary
//! fact named `rulex` when it does not. The parser therefore tries the
//! alternatives **in grammar order** and takes the first that consumes the
//! whole `( … )` — which is what Lark's ambiguity resolution does, and is
//! sound here because every alternative ends at the same closing paren, so a
//! choice never changes what the *rest* of the file sees.
//!
//! **Error position.** ein.py reports Lark's, and Lark reports the furthest
//! point its Earley columns reached — so the parser tracks the furthest
//! cursor at which an expected terminal failed to match and reports that, not
//! the failure of the last alternative tried. Reaching the end of the input
//! is `UnexpectedEOF`, which Lark stamps `-1:-1` with `pos_in_stream = -1`;
//! [`get_context`] reproduces the caret that quirk produces
//! ([design/04](../../../../plans/m1a_rust/design/04_ir_frontend.md) §4,
//! Q-M1a.3).

use crate::ast::{Ast, FileId, Loc, Node, NodeId, canonical_int};
use crate::lex::{
    Cursor, Lexeme, Term, match_literal, match_term, skip_trivia, unescape_string_body,
};

/// A syntax error, rendered exactly as `ir/parser.py` renders Lark's:
/// `{file}:{line}:{col}: unexpected input` and then `get_context`'s two lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub file: String,
    /// `-1` at end of input — Lark's `UnexpectedEOF`.
    pub line: i64,
    pub col: i64,
    pub context: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}: unexpected input\n{}",
            self.file, self.line, self.col, self.context
        )
    }
}

impl std::error::Error for ParseError {}

/// Parse `text` into top-level forms, appending to `ast`.
///
/// `filename` is recorded in every `Loc` and in the error message; `None`
/// becomes `<string>`, as `parse(text)` does in ein.py.
pub fn parse(ast: &mut Ast, text: &str, filename: Option<&str>) -> Result<Vec<NodeId>, ParseError> {
    ein_core::counters::bump(|k| {
        k.parse_call += 1;
        k.parse_bytes += text.len() as u64;
    });
    let file = ast.intern_file(filename);
    let mut p = Parser {
        src: text,
        ast,
        file,
        scratch: Vec::new(),
        furthest: Cursor::START,
    };
    let mut forms = Vec::new();
    let mut c = Cursor::START;
    loop {
        c = skip_trivia(text, c);
        if c.pos == text.len() {
            return Ok(forms);
        }
        match p.form(c) {
            Some((id, next)) => {
                forms.push(id);
                c = next;
            }
            None => {
                // Not `c`: an inner alternative may have got further before it
                // died, and that further point is what Lark names.
                p.note_fail(c);
                return Err(p.error(filename));
            }
        }
    }
}

/// Lark's `UnexpectedInput.get_context`, character-for-character.
///
/// Two details are load-bearing and neither is incidental: the window is
/// **40 characters** either side, so a long line is *truncated* around the
/// caret; and `pos` is `-1` for an EOF error, where Python's negative slicing
/// then renders the last line of the file with the caret one past its end.
pub fn get_context(text: &str, pos: Option<usize>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let (before, after): (Vec<char>, Vec<char>) = match pos {
        Some(pos) => {
            let start = pos.saturating_sub(40);
            let end = (pos + 40).min(n);
            (
                chars[start..pos.min(n)].to_vec(),
                chars[pos.min(n)..end].to_vec(),
            )
        }
        None => {
            // `text[0:-1]` and `text[-1:39]`, exactly.
            let before = if n == 0 {
                Vec::new()
            } else {
                chars[..n - 1].to_vec()
            };
            let after = if n >= 1 && n - 1 < 39 {
                chars[n - 1..39.min(n)].to_vec()
            } else {
                Vec::new()
            };
            (before, after)
        }
    };
    let before: String = {
        let s: String = before.into_iter().collect();
        match s.rfind('\n') {
            Some(i) => s[i + 1..].to_string(),
            None => s,
        }
    };
    let after: String = {
        let s: String = after.into_iter().collect();
        match s.find('\n') {
            Some(i) => s[..i].to_string(),
            None => s,
        }
    };
    let pad = " ".repeat(expandtabs_len(&before));
    format!("{before}{after}\n{pad}^\n")
}

/// `len(s.expandtabs())` — tab stops every 8 columns.
fn expandtabs_len(s: &str) -> usize {
    let mut col = 0;
    for ch in s.chars() {
        if ch == '\t' {
            col += 8 - (col % 8);
        } else {
            col += 1;
        }
    }
    col
}

/// 1-based line and column of character `i`.
fn line_col(chars: &[char], i: usize) -> (u32, u32) {
    let (mut line, mut col) = (1u32, 1u32);
    for &ch in &chars[..i.min(chars.len())] {
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// Where the ignored terminal starting at `i` ends, if one does — `\s+`,
/// `;[^\n]*`, or `#\|[\s\S]*?\|#` (non-greedy, so the *first* `|#`).
fn trivia_end(chars: &[char], i: usize) -> Option<usize> {
    match chars[i] {
        ch if is_py_space(ch) => {
            let mut e = i;
            while e < chars.len() && is_py_space(chars[e]) {
                e += 1;
            }
            Some(e)
        }
        ';' => {
            let mut e = i;
            while e < chars.len() && chars[e] != '\n' {
                e += 1;
            }
            Some(e)
        }
        '#' if chars.get(i + 1) == Some(&'|') => {
            let mut e = i + 2;
            while e + 1 < chars.len() {
                if chars[e] == '|' && chars[e + 1] == '#' {
                    return Some(e + 2);
                }
                e += 1;
            }
            None
        }
        _ => None,
    }
}

fn is_py_space(ch: char) -> bool {
    ch.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&ch)
}

/// Where Lark *reports* the failure, given the position `p` at which the parse
/// actually ran out of viable items. Usually `p` — but not always, and the
/// difference is neither cosmetic nor rare enough to ignore.
///
/// `xearley.py`'s scanner advances one **character** at a time and raises
/// `UnexpectedCharacters(i)` only when the Earley set, the scan buffer *and*
/// the `delayed_matches` dict are all empty. `delayed_matches` is a
/// `defaultdict(list)` keyed by a match's end position, and the `%ignore`
/// pass writes `delayed_matches[m.end()].extend(to_scan)` for **every**
/// position at which whitespace or a comment matches — including positions
/// no live item is looking at, and including an empty `to_scan`, which still
/// *creates* the key. A dict holding one empty list is truthy, so those
/// phantom keys hold the error back until the scanner walks past them:
///
/// ```text
/// (y";"{      → 1:6   ·  the `{`
/// (y";"{?     → 1:7   ·  the `;` inside the string matched `;[^\n]*`, whose
///                        end is one character further along
/// ```
///
/// So the reported position is the first `i >= p` at which no pending key
/// remains — and since keys are deleted in increasing order, "no pending key"
/// is just "the furthest end seen so far is at most `i + 1`". Running off the
/// end without dying is `UnexpectedEOF`, which is `None` here.
///
/// Copied rather than corrected: the harness diffs stderr, so a better message
/// is a T3 failure (Q-M1a.3 — reproduce exactly, then improve both together).
fn death_position(chars: &[char], p: usize) -> Option<usize> {
    let mut furthest_key = 0usize;
    for i in 0..chars.len() {
        if let Some(e) = trivia_end(chars, i) {
            furthest_key = furthest_key.max(e);
        }
        if i >= p && furthest_key <= i + 1 {
            return Some(i);
        }
    }
    None
}

struct Parser<'a> {
    src: &'a str,
    ast: &'a mut Ast,
    file: FileId,
    /// Argument stack. Productions push onto it from a mark and flush the
    /// slice into the arena on success, so a failed alternative leaves no
    /// half-built span behind.
    scratch: Vec<NodeId>,
    furthest: Cursor,
}

/// What a production returns: the node it built and where to resume.
type Parsed = Option<(NodeId, Cursor)>;

impl<'a> Parser<'a> {
    // ── Failure tracking ───────────────────────────────────────────

    fn note_fail(&mut self, c: Cursor) {
        let at = skip_trivia(self.src, c);
        if at.pos > self.furthest.pos {
            self.furthest = at;
        }
    }

    fn error(&self, filename: Option<&str>) -> ParseError {
        let chars: Vec<char> = self.src.chars().collect();
        let (line, col, pos) = match death_position(&chars, self.furthest.chars) {
            Some(i) => {
                let (line, col) = line_col(&chars, i);
                (line as i64, col as i64, Some(i))
            }
            None => (-1, -1, None),
        };
        let context = get_context(self.src, pos);
        ParseError {
            file: filename.unwrap_or("<string>").to_string(),
            line,
            col,
            context: context.trim_end_matches('\n').to_string(),
        }
    }

    // ── Terminals ──────────────────────────────────────────────────

    fn eat(&mut self, c: Cursor, term: Term) -> Option<Lexeme> {
        match match_term(self.src, c, term) {
            Some(l) => Some(l),
            None => {
                self.note_fail(c);
                None
            }
        }
    }

    fn eat_lit(&mut self, c: Cursor, word: &str) -> Option<Lexeme> {
        match match_literal(self.src, c, word) {
            Some(l) => Some(l),
            None => {
                self.note_fail(c);
                None
            }
        }
    }

    /// Peek without recording a failure — used where the grammar's `*` /
    /// alternation makes "not this one" an ordinary outcome.
    fn peek(&self, c: Cursor, term: Term) -> bool {
        match_term(self.src, c, term).is_some()
    }

    fn loc(&self, l: &Lexeme) -> Option<Loc> {
        Some(Loc {
            file: self.file,
            line: l.at.line,
            col: l.at.col,
        })
    }

    // ── Leaf builders ──────────────────────────────────────────────

    fn symbol(&mut self, c: Cursor) -> Parsed {
        let l = self.eat(c, Term::Symbol)?;
        let loc = self.loc(&l);
        let id = self.ast.atom(l.text(self.src), loc);
        Some((id, l.next))
    }

    fn var(&mut self, c: Cursor) -> Parsed {
        let l = self.eat(c, Term::Var)?;
        let loc = self.loc(&l);
        let name = self.ast.intern(&l.text(self.src)[1..]);
        Some((self.ast.push(Node::Var(name), loc), l.next))
    }

    /// A `?value` leaf, or `None` when nothing here is one. `KEYWORD` is
    /// deliberately absent: excluding it from `?value` is what enforces the
    /// `:kw value` alternation.
    fn leaf_value(&mut self, c: Cursor) -> Parsed {
        if let Some((id, next)) = self.symbol(c) {
            return Some((id, next));
        }
        if let Some((id, next)) = self.var(c) {
            return Some((id, next));
        }
        if let Some(l) = match_term(self.src, c, Term::Wildcard) {
            let loc = self.loc(&l);
            return Some((self.ast.push(Node::Wildcard, loc), l.next));
        }
        // RANGE before INT: both are digit-anchored, and the INT reading of
        // `1..5` always dies on the `..` that follows it.
        if let Some(l) = match_term(self.src, c, Term::Range) {
            let loc = self.loc(&l);
            let text = l.text(self.src);
            let (low_s, high_s) = text.split_once("..").expect("RANGE has ..");
            let low = self.ast.intern(&canonical_int(low_s));
            let high = (high_s != "*").then(|| self.ast.intern(&canonical_int(high_s)));
            return Some((self.ast.push(Node::Range { low, high }, loc), l.next));
        }
        if let Some(l) = match_term(self.src, c, Term::Int) {
            let loc = self.loc(&l);
            let v = self.ast.intern(&canonical_int(l.text(self.src)));
            return Some((self.ast.push(Node::Int(v), loc), l.next));
        }
        if let Some(l) = match_term(self.src, c, Term::Str) {
            let loc = self.loc(&l);
            let raw = l.text(self.src);
            let body = unescape_string_body(&raw[1..raw.len() - 1]);
            let v = self.ast.intern(&body);
            return Some((self.ast.push(Node::Str(v), loc), l.next));
        }
        self.note_fail(c);
        None
    }

    fn kw_pair(&mut self, c: Cursor) -> Parsed {
        let l = self.eat(c, Term::Keyword)?;
        let loc = self.loc(&l);
        let name = self.ast.intern(&l.text(self.src)[1..]);
        let key = self.ast.push(Node::Keyword(name), loc);
        let (value, next) = self.value(l.next)?;
        // `KwPair.loc` is the *key's* loc — `ast.py::kw_pair`.
        Some((self.ast.push(Node::KwPair { key, value }, loc), next))
    }

    // ── `?value` ───────────────────────────────────────────────────

    fn value(&mut self, c: Cursor) -> Parsed {
        if !self.peek(c, Term::LParen) {
            return self.leaf_value(c);
        }
        let open = self.eat(c, Term::LParen)?;
        let after = open.next;
        // Grammar order: not | and | or | neq | generic_list. The reserved
        // forms come first, and they win where both readings parse —
        // `(a (notx))` is `(a (not x))`, not a list headed `notx`.
        for alt in [
            Parser::alt_not,
            Parser::alt_and,
            Parser::alt_or,
            Parser::alt_neq,
            Parser::alt_generic_list,
        ] {
            if let Some(r) = self.attempt(alt, after) {
                return Some(r);
            }
        }
        None
    }

    /// Run one alternative from `c`, rolling both arenas back if it fails.
    fn attempt(&mut self, alt: fn(&mut Self, Cursor) -> Parsed, c: Cursor) -> Parsed {
        let mark = self.ast.mark();
        let scratch = self.scratch.len();
        match alt(self, c) {
            Some(r) => Some(r),
            None => {
                self.ast.rollback(mark);
                self.scratch.truncate(scratch);
                None
            }
        }
    }

    // ── Repetition helpers ─────────────────────────────────────────

    /// `kw_pair*` — pushed onto the scratch stack, stopping at the first
    /// token that does not open one.
    fn kw_pairs(&mut self, mut c: Cursor) -> Cursor {
        while self.peek(c, Term::Keyword) {
            let mark = self.ast.mark();
            let scratch = self.scratch.len();
            match self.kw_pair(c) {
                Some((id, next)) => {
                    self.scratch.push(id);
                    c = next;
                }
                None => {
                    self.ast.rollback(mark);
                    self.scratch.truncate(scratch);
                    break;
                }
            }
        }
        c
    }

    /// `value*` — stops at a `)` or a `KEYWORD`, which are the only two things
    /// that legally follow a value list.
    fn values(&mut self, mut c: Cursor) -> Cursor {
        loop {
            if self.peek(c, Term::RParen) || self.peek(c, Term::Keyword) {
                return c;
            }
            let mark = self.ast.mark();
            let scratch = self.scratch.len();
            match self.value(c) {
                Some((id, next)) => {
                    self.scratch.push(id);
                    c = next;
                }
                None => {
                    self.ast.rollback(mark);
                    self.scratch.truncate(scratch);
                    return c;
                }
            }
        }
    }

    /// Flush the scratch stack from `mark` into the arena as one form.
    fn finish(&mut self, mark: usize, head: NodeId, loc: Option<Loc>) -> NodeId {
        let id = self.ast.sform(head, &self.scratch[mark..], loc);
        self.scratch.truncate(mark);
        id
    }

    fn finish_named(&mut self, mark: usize, name: &str, loc: Option<Loc>) -> NodeId {
        let head = self.ast.atom(name, None);
        self.finish(mark, head, loc)
    }

    // ── Value-position alternatives ────────────────────────────────

    fn alt_not(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "not")?;
        let mark = self.scratch.len();
        let (v, mut c) = self.value(lit.next)?;
        self.scratch.push(v);
        c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "not", None), close.next))
    }

    fn alt_and(&mut self, c: Cursor) -> Parsed {
        self.alt_and_or(c, "and")
    }

    fn alt_or(&mut self, c: Cursor) -> Parsed {
        self.alt_and_or(c, "or")
    }

    /// `"(" ("and"|"or") value+ kw_pair* ")"`.
    fn alt_and_or(&mut self, c: Cursor, word: &str) -> Parsed {
        let lit = self.eat_lit(c, word)?;
        let mark = self.scratch.len();
        let (first, c) = self.value(lit.next)?;
        self.scratch.push(first);
        let c = self.values(c);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, word, None), close.next))
    }

    /// `"(" "neq" value value ")"` — no trailing kw-pairs, unlike `not`.
    fn alt_neq(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "neq")?;
        let mark = self.scratch.len();
        let (a, c) = self.value(lit.next)?;
        self.scratch.push(a);
        let (b, c) = self.value(c)?;
        self.scratch.push(b);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "neq", None), close.next))
    }

    /// `generic_list: "(" ")" | "(" list_head list_item* ")"`.
    ///
    /// The empty case gets the synthetic `@empty` head the dumper prints as
    /// `()`; the non-empty case is the **one** production that gives its form
    /// a `Loc` — its head's.
    fn alt_generic_list(&mut self, c: Cursor) -> Parsed {
        if let Some(close) = match_term(self.src, c, Term::RParen) {
            let mark = self.scratch.len();
            return Some((self.finish_named(mark, "@empty", None), close.next));
        }
        let (head, mut c) = self.list_head(c)?;
        let head_loc = self.ast.loc(head);
        let mark = self.scratch.len();
        loop {
            if self.peek(c, Term::RParen) {
                break;
            }
            let item = if self.peek(c, Term::Keyword) {
                self.kw_pair(c)
            } else {
                self.value(c)
            };
            match item {
                Some((id, next)) => {
                    self.scratch.push(id);
                    c = next;
                }
                None => {
                    self.scratch.truncate(mark);
                    return None;
                }
            }
        }
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish(mark, head, head_loc), close.next))
    }

    /// `?list_head: SYMBOL | VAR | WILDCARD | EQ`.
    fn list_head(&mut self, c: Cursor) -> Parsed {
        if let Some(r) = self.attempt(Parser::symbol, c) {
            return Some(r);
        }
        if let Some(r) = self.attempt(Parser::var, c) {
            return Some(r);
        }
        if let Some(l) = match_term(self.src, c, Term::Wildcard) {
            let loc = self.loc(&l);
            return Some((self.ast.push(Node::Wildcard, loc), l.next));
        }
        if let Some(l) = match_term(self.src, c, Term::Eq) {
            let loc = self.loc(&l);
            return Some((self.ast.atom("=", loc), l.next));
        }
        self.note_fail(c);
        None
    }

    // ── Top-level forms ────────────────────────────────────────────

    fn form(&mut self, c: Cursor) -> Parsed {
        let open = self.eat(c, Term::LParen)?;
        let after = open.next;
        // `?form`'s alternation, in grammar order, with `fact_form` inlined as
        // its three: eq_fact | not_form | generic_fact.
        for alt in [
            Parser::alt_relation_decl,
            Parser::alt_rule_decl,
            Parser::alt_hrule_decl,
            Parser::alt_query,
            Parser::alt_config,
            Parser::alt_trace,
            Parser::alt_macro_decl,
            Parser::alt_import,
            Parser::alt_eq_fact,
            Parser::alt_not,
            Parser::alt_generic_fact,
        ] {
            if let Some(r) = self.attempt(alt, after) {
                return Some(r);
            }
        }
        None
    }

    /// `relation_decl: "(" "relation" SYMBOL SYMBOL* kw_pair* ")"`.
    ///
    /// The signature may be empty (S1.22.4: a bare `(relation R)`), but the
    /// name may not — `(relation)` falls through to `generic_fact` and is
    /// rejected by the loader, not here.
    fn alt_relation_decl(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "relation")?;
        let mark = self.scratch.len();
        let (name, mut c) = self.symbol(lit.next)?;
        self.scratch.push(name);
        while let Some(r) = self.attempt(Parser::symbol, c) {
            self.scratch.push(r.0);
            c = r.1;
        }
        c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "relation", None), close.next))
    }

    fn alt_rule_decl(&mut self, c: Cursor) -> Parsed {
        self.alt_rule_like(c, "rule")
    }

    fn alt_hrule_decl(&mut self, c: Cursor) -> Parsed {
        self.alt_rule_like(c, "hrule")
    }

    /// `rule_decl: "(" "rule" SYMBOL rule_params kw_pair+ ")"` — and `hrule`,
    /// whose shape is identical and whose only difference is where the loader
    /// files it.
    fn alt_rule_like(&mut self, c: Cursor, word: &str) -> Parsed {
        let lit = self.eat_lit(c, word)?;
        let mark = self.scratch.len();
        let (name, c) = self.symbol(lit.next)?;
        self.scratch.push(name);
        let (params, c) = self.params(c, false)?;
        self.scratch.push(params);
        let (first, c) = self.kw_pair(c)?;
        self.scratch.push(first);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, word, None), close.next))
    }

    /// `rule_params: "(" VAR* ")"` / `macro_params: "(" VAR+ ")"`.
    fn params(&mut self, c: Cursor, at_least_one: bool) -> Parsed {
        let open = self.eat(c, Term::LParen)?;
        let mut c = open.next;
        let mark = self.scratch.len();
        let mut n = 0;
        while let Some(r) = self.attempt(Parser::var, c) {
            self.scratch.push(r.0);
            c = r.1;
            n += 1;
        }
        if at_least_one && n == 0 {
            self.scratch.truncate(mark);
            return None;
        }
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "@params", None), close.next))
    }

    fn alt_query(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "query")?;
        let mark = self.scratch.len();
        // `kw_pair+` — `(query)` is a parse error, unlike `(config)`.
        let (first, c) = self.kw_pair(lit.next)?;
        self.scratch.push(first);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "query", None), close.next))
    }

    fn alt_config(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "config")?;
        let mark = self.scratch.len();
        let c = self.kw_pairs(lit.next);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "config", None), close.next))
    }

    /// `trace_form: "(" "trace" trace_event* ")"`. The loader ignores
    /// `(trace …)` entirely, but the parser must accept it — a trace is
    /// ein-lang the engine itself writes.
    fn alt_trace(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "trace")?;
        let mark = self.scratch.len();
        let mut c = lit.next;
        while !self.peek(c, Term::RParen) {
            match self.attempt(Parser::trace_event, c) {
                Some((id, next)) => {
                    self.scratch.push(id);
                    c = next;
                }
                None => {
                    self.scratch.truncate(mark);
                    return None;
                }
            }
        }
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "trace", None), close.next))
    }

    fn trace_event(&mut self, c: Cursor) -> Parsed {
        let open = self.eat(c, Term::LParen)?;
        let c = open.next;
        for (word, kws) in [
            ("step", true),
            ("branch-open", true),
            ("branch-close", true),
            ("branch-ref", false),
            ("contradiction", true),
            ("symmetry-class", true),
        ] {
            let mark = self.ast.mark();
            let scratch = self.scratch.len();
            if let Some(r) = self.trace_event_of(c, word, kws) {
                return Some(r);
            }
            self.ast.rollback(mark);
            self.scratch.truncate(scratch);
        }
        None
    }

    fn trace_event_of(&mut self, c: Cursor, word: &str, kws: bool) -> Parsed {
        let lit = self.eat_lit(c, word)?;
        let mark = self.scratch.len();
        let (name, mut c) = self.symbol(lit.next)?;
        self.scratch.push(name);
        if kws {
            c = self.kw_pairs(c);
        }
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, word, None), close.next))
    }

    /// `macro_decl: "(" "macro" SYMBOL macro_params value ")"` — the params
    /// list is `VAR+`, so `(macro m () B)` is a parse error.
    fn alt_macro_decl(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "macro")?;
        let mark = self.scratch.len();
        let (name, c) = self.symbol(lit.next)?;
        self.scratch.push(name);
        let (params, c) = self.params(c, true)?;
        self.scratch.push(params);
        let (body, c) = self.value(c)?;
        self.scratch.push(body);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "macro", None), close.next))
    }

    /// `import_form: "(" "import" SYMBOL kw_pair* ")"` — the dotted module
    /// name is one `SYMBOL`; resolution to a file is the loader's job.
    fn alt_import(&mut self, c: Cursor) -> Parsed {
        let lit = self.eat_lit(c, "import")?;
        let mark = self.scratch.len();
        let (name, c) = self.symbol(lit.next)?;
        self.scratch.push(name);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish_named(mark, "import", None), close.next))
    }

    /// `eq_fact: "(" EQ value value kw_pair* ")"`. The head is the `=` atom
    /// *with* its position — `EQ` is a named terminal so it survives token
    /// filtering and reaches the AST.
    fn alt_eq_fact(&mut self, c: Cursor) -> Parsed {
        let l = self.eat(c, Term::Eq)?;
        let loc = self.loc(&l);
        let head = self.ast.atom("=", loc);
        let mark = self.scratch.len();
        let (a, c) = self.value(l.next)?;
        self.scratch.push(a);
        let (b, c) = self.value(c)?;
        self.scratch.push(b);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish(mark, head, None), close.next))
    }

    /// `generic_fact: "(" SYMBOL value* kw_pair* ")"` — the flat default, and
    /// the reason a typo'd declarator is a *parse* error rather than a silent
    /// fact: the reserved words are excluded from `SYMBOL`.
    fn alt_generic_fact(&mut self, c: Cursor) -> Parsed {
        let (head, c) = self.symbol(c)?;
        let mark = self.scratch.len();
        let c = self.values(c);
        let c = self.kw_pairs(c);
        let close = self.eat(c, Term::RParen)?;
        Some((self.finish(mark, head, None), close.next))
    }
}
