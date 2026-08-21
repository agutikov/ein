//! The tokeniser — one function per Lark terminal, matched on demand.
//!
//! Lark parses ein-lang with **Earley + the dynamic lexer**, which does not
//! produce a token stream at all: at every position it offers every terminal
//! that matches there, and the parser explores. A plain maximal-munch lexer is
//! therefore *not* equivalent, and the difference is observable —
//!
//! ```text
//! (rulex (?a) :match X :assert Y)   →   (rule x (?a) :match X :assert Y)
//! ```
//!
//! — because the anonymous literal `"rule"` matches at the same position where
//! `SYMBOL` matches `rulex`, and only the split reading parses. Six more
//! literals behave the same way (`relation`, `step`, `branch-open`, …).
//!
//! So this module exposes **positional matchers**, not a stream: the parser
//! holds a [`Cursor`], asks for the terminal it wants at that cursor, and
//! restores the cursor when an alternative fails
//! ([`crate::parse`]). Each matcher is a pure function of `(&str, Cursor)`, so
//! the lexer allocates nothing per token and is trivially fuzzable.
//!
//! Terminal definitions are `grammar.lark`'s, verbatim — transcribed to EBNF
//! at [`00_ebnf.md` §1](../../../../docs/kernel/ir/03-ein-lang/00_ebnf.md)
//! when that file went (S1a.10.5), which is the spec this scanner answers to; the
//! comments below record only where the Rust differs in *form* from the
//! regex.

/// A position in the source: byte offset for slicing, 1-based line and column
/// **in characters** for `Loc` and for error rendering (Lark counts
/// characters, not bytes).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cursor {
    pub pos: usize,
    pub line: u32,
    pub col: u32,
    /// Characters consumed so far — `pos_in_stream` in Lark's terms, which
    /// indexes a Python `str`.
    pub chars: usize,
}

impl Cursor {
    pub const START: Cursor = Cursor {
        pos: 0,
        line: 1,
        col: 1,
        chars: 0,
    };

    fn bump(&mut self, ch: char) {
        self.pos += ch.len_utf8();
        self.chars += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

/// The terminals the parser asks for by name. The eighteen *string literals*
/// (`rule`, `not`, `step`, …) are not here: they are matched by
/// [`match_literal`], which is what makes the reserved-word split above
/// possible.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Term {
    LParen,
    RParen,
    Symbol,
    Var,
    Keyword,
    Wildcard,
    Eq,
    Range,
    Int,
    Str,
}

/// A matched terminal: the source span and the cursor just past it.
#[derive(Clone, Copy, Debug)]
pub struct Lexeme {
    /// Byte range of the raw token text.
    pub start: usize,
    pub end: usize,
    /// Where the token starts — this is the position a `Loc` records.
    pub at: Cursor,
    /// Where scanning resumes.
    pub next: Cursor,
}

impl Lexeme {
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }
}

/// The words the grammar spells as string literals. Each can be matched at a
/// position where `SYMBOL` also matches a *longer* identifier, and the parser
/// tries both readings — that is the whole reason this list is data.
///
/// `relation` is here but is **not** excluded from `SYMBOL` (rules match
/// `(relation ?R ?A ?B)` patterns), which is why `(relation R (T1 T2))`
/// parses as a fact and is rejected later by the loader.
pub const LITERALS: &[&str] = &[
    "relation",
    "rule",
    "hrule",
    "query",
    "config",
    "trace",
    "macro",
    "import",
    "not",
    "and",
    "or",
    "neq",
    "step",
    "branch-open",
    "branch-close",
    "branch-ref",
    "contradiction",
    "symmetry-class",
];

/// The eleven words `SYMBOL`'s negative lookahead rejects. `relation` is
/// deliberately absent; see [`LITERALS`].
const RESERVED: &[&str] = &[
    "not", "and", "or", "neq", "rule", "hrule", "query", "config", "trace", "macro", "import",
];

/// Python's `\s`: Unicode `White_Space` **plus** U+001C–U+001F, which the
/// standard property does not include but `re` does.
fn is_py_space(ch: char) -> bool {
    ch.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&ch)
}

/// Python's `\w`, as `\b` uses it.
fn is_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// The ASCII half of [`is_py_space`], as a byte test: `\t\n\v\f\r`, space,
/// and the U+001C–U+001F `re` adds. Everything outside it — U+00A0 and the
/// rest of `White_Space` — falls back to the `char` path, which is why this
/// may be a fast path rather than a redefinition.
const fn is_ascii_py_space(b: u8) -> bool {
    matches!(b, b' ' | 0x09..=0x0d | 0x1c..=0x1f)
}

fn char_at(src: &str, pos: usize) -> Option<char> {
    src[pos..].chars().next()
}

/// Advance `c` to the byte offset `end`, which must be a `char` boundary.
///
/// A [`Cursor`] counts **characters** and lines, so this is still one step per
/// character — but the step is a byte compare rather than a UTF-8 decode, and
/// that is what a parse was spending its time on: `skip_trivia` was 26.3 % of a
/// `parse/zebra2` profile and `match_term` 13.5 %, most of it
/// `src[pos..].chars().next()` over runs that are ASCII (T1a.6.5.1).
///
/// Vectorising it instead — `is_ascii()`, then `rposition` for the last
/// newline — was **+10 to +14 %**: the spans reaching here are one space and a
/// two-character indent, where three passes lose to one loop. The one run long
/// enough to pay for a bulk path is a line comment, and [`skip_trivia_from`]
/// gives it one.
#[inline]
fn advance_to(src: &str, mut c: Cursor, end: usize) -> Cursor {
    let bytes = src.as_bytes();
    while c.pos < end {
        let b = bytes[c.pos];
        if b < 0x80 {
            c.pos += 1;
            c.chars += 1;
            if b == b'\n' {
                c.line += 1;
                c.col = 1;
            } else {
                c.col += 1;
            }
        } else {
            let ch = char_at(src, c.pos).expect("inside the span");
            c.bump(ch);
        }
    }
    c
}

/// Advance over `%ignore`d input: whitespace, `;…` to end of line, and
/// `#|…|#`.
///
/// An **unterminated** `#|` is not trivia: `BLOCK_COMMENT` requires the
/// closing `|#`, so the terminal does not match and the `#` becomes the
/// position at which nothing can be scanned. Leaving the cursor on it is what
/// makes the error land where Lark's lands.
#[inline]
pub fn skip_trivia(src: &str, c: Cursor) -> Cursor {
    // Every alternative the parser tries asks for a terminal at the *same*
    // position, so most calls have nothing to skip and this is one byte
    // compare inlined into the caller — `skip_trivia` was 13-26 % of a parse
    // profile largely as call overhead (T1a.6.5.1).
    match src.as_bytes().get(c.pos) {
        Some(&b) if is_trivia_start(b) => skip_trivia_from(src, c),
        _ => c,
    }
}

/// Could a token of trivia start with this byte? `#` only opens one as `#|`
/// and a non-ASCII byte only when it decodes to `White_Space`; both are the
/// slow path's business, and a false positive there costs nothing.
const fn is_trivia_start(b: u8) -> bool {
    is_ascii_py_space(b) || b == b';' || b == b'#' || b >= 0x80
}

fn skip_trivia_from(src: &str, mut c: Cursor) -> Cursor {
    let bytes = src.as_bytes();
    loop {
        let Some(&b) = bytes.get(c.pos) else {
            return c;
        };
        if is_ascii_py_space(b) {
            c.pos += 1;
            c.chars += 1;
            if b == b'\n' {
                c.line += 1;
                c.col = 1;
            } else {
                c.col += 1;
            }
            continue;
        }
        if b == b';' {
            // `str::find` of a `char` is `memchr` — the line-comment scan the
            // task asks for, without a dependency for it. A comment is the one
            // long run in the file, and it cannot contain a newline, so an
            // ASCII one is three additions rather than forty.
            let end = src[c.pos..].find('\n').map_or(src.len(), |rel| c.pos + rel);
            let span = &src[c.pos..end];
            if span.is_ascii() {
                c.pos = end;
                c.chars += span.len();
                c.col += span.len() as u32;
            } else {
                c = advance_to(src, c, end);
            }
            continue;
        }
        if b == b'#' && src[c.pos..].starts_with("#|") {
            let Some(rel) = src[c.pos + 2..].find("|#") else {
                return c;
            };
            c = advance_to(src, c, c.pos + 2 + rel + 2);
            continue;
        }
        // Non-ASCII: still possibly `White_Space` (U+00A0 and friends), which
        // only a decode can tell.
        if b >= 0x80 {
            let ch = char_at(src, c.pos).expect("a char boundary");
            if is_py_space(ch) {
                c.bump(ch);
                continue;
            }
        }
        return c;
    }
}

/// Match one of [`LITERALS`] at `c` (after trivia). A plain string compare —
/// Lark's `PatternStr` carries no word boundary, which is precisely why
/// `rulex` can be read as `rule` + `x`.
pub fn match_literal(src: &str, c: Cursor, word: &str) -> Option<Lexeme> {
    ein_core::counters::bump(|k| k.lex_match += 1);
    let at = skip_trivia(src, c);
    if !src[at.pos..].starts_with(word) {
        return None;
    }
    let next = advance_to(src, at, at.pos + word.len());
    Some(Lexeme {
        start: at.pos,
        end: next.pos,
        at,
        next,
    })
}

/// Match `term` at `c` (after trivia), or `None`.
pub fn match_term(src: &str, c: Cursor, term: Term) -> Option<Lexeme> {
    ein_core::counters::bump(|k| {
        k.lex_match += 1;
        k.lex_symbol += u64::from(matches!(term, Term::Symbol));
    });
    let at = skip_trivia(src, c);
    let rest = &src[at.pos..];
    let len = match term {
        Term::LParen => rest.starts_with('(').then_some(1),
        Term::RParen => rest.starts_with(')').then_some(1),
        Term::Eq => rest.starts_with('=').then_some(1),
        Term::Wildcard => match_wildcard(rest),
        Term::Symbol => match_symbol(rest),
        Term::Var => match_var(rest),
        Term::Keyword => match_keyword(rest),
        Term::Range => match_range(rest),
        Term::Int => match_int(rest),
        Term::Str => match_string(rest),
    }?;
    let next = advance_to(src, at, at.pos + len);
    Some(Lexeme {
        start: at.pos,
        end: next.pos,
        at,
        next,
    })
}

/// `WILDCARD: /_(?![A-Za-z0-9_])/` — `_` only when standalone, so a leading
/// `__` of a dunder atom is never two wildcards. Note the lookahead is ASCII
/// (`[A-Za-z0-9_]`), not `\w`.
fn match_wildcard(rest: &str) -> Option<usize> {
    if !rest.starts_with('_') {
        return None;
    }
    match rest[1..].chars().next() {
        Some(ch) if ch.is_ascii_alphanumeric() || ch == '_' => None,
        _ => Some(1),
    }
}

/// `SYMBOL: /(?!(?:not|and|…|import)\b)(?:__)?[A-Za-z][A-Za-z0-9_*.-]*/`
///
/// The lookahead is **start-anchored**, so `std.rule` is one atom while
/// `rule.x` is not a `SYMBOL` at all (the literal `rule` matches there
/// instead, and the `.x` that follows scans as nothing — which is how
/// `(rule.x A)` becomes a parse error).
fn match_symbol(rest: &str) -> Option<usize> {
    for word in RESERVED {
        if let Some(after) = rest.strip_prefix(*word) {
            // `\b` — a boundary, i.e. the next character is not a word
            // character (end of input counts).
            if after.chars().next().is_none_or(|ch| !is_word(ch)) {
                return None;
            }
        }
    }
    let body = rest.strip_prefix("__").unwrap_or(rest);
    let dunder = rest.len() - body.len();
    let mut chars = body.char_indices();
    match chars.next() {
        Some((_, ch)) if ch.is_ascii_alphabetic() => {}
        _ => return None,
    }
    let mut len = 1;
    for (i, ch) in chars {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '*' | '.' | '-') {
            len = i + ch.len_utf8();
        } else {
            break;
        }
    }
    Some(dunder + len)
}

/// `VAR: /\?[A-Za-z][A-Za-z0-9_*-]*/` — no `.`, a deliberate asymmetry with
/// `SYMBOL` (module names are atoms, never variables).
fn match_var(rest: &str) -> Option<usize> {
    let body = rest.strip_prefix('?')?;
    let mut chars = body.char_indices();
    match chars.next() {
        Some((_, ch)) if ch.is_ascii_alphabetic() => {}
        _ => return None,
    }
    let mut len = 1;
    for (i, ch) in chars {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '*' | '-') {
            len = i + ch.len_utf8();
        } else {
            break;
        }
    }
    Some(1 + len)
}

/// `KEYWORD: /:[a-z][A-Za-z0-9_-]*/` — lower-case first character.
fn match_keyword(rest: &str) -> Option<usize> {
    let body = rest.strip_prefix(':')?;
    let mut chars = body.char_indices();
    match chars.next() {
        Some((_, ch)) if ch.is_ascii_lowercase() => {}
        _ => return None,
    }
    let mut len = 1;
    for (i, ch) in chars {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-') {
            len = i + ch.len_utf8();
        } else {
            break;
        }
    }
    Some(1 + len)
}

/// `RANGE: /[0-9]+\.\.([0-9]+|\*)/`. Digit-anchored, so it never collides with
/// a dotted `SYMBOL`; tried before `INT` because both start on a digit.
fn match_range(rest: &str) -> Option<usize> {
    let low = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    if low == 0 || !rest[low..].starts_with("..") {
        return None;
    }
    let after = &rest[low + 2..];
    if after.starts_with('*') {
        return Some(low + 3);
    }
    let high = after
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(after.len());
    (high > 0).then_some(low + 2 + high)
}

/// `INT: /-?[0-9]+/`.
fn match_int(rest: &str) -> Option<usize> {
    let neg = usize::from(rest.starts_with('-'));
    let digits = &rest[neg..];
    let n = digits
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(digits.len());
    (n > 0).then_some(neg + n)
}

/// `STRING: /"([^"\\]|\\.)*"/`.
///
/// The `.` of `\\.` is Python's, which does **not** match a newline — so a raw
/// newline inside a string is fine (`[^"\\]` takes it) but a backslash
/// immediately before one is not, and the terminal fails.
fn match_string(rest: &str) -> Option<usize> {
    let mut it = rest.char_indices();
    match it.next() {
        Some((_, '"')) => {}
        _ => return None,
    }
    while let Some((i, ch)) = it.next() {
        match ch {
            '"' => return Some(i + 1),
            '\\' => match it.next() {
                Some((_, '\n')) | None => return None,
                Some(_) => {}
            },
            _ => {}
        }
    }
    None
}

/// `STRING`'s body, unescaped with ein.py's **minimal** set: `\n`/`\t`/`\r`
/// map, and every other `\X` is `X`. There is no `\xNN`, no `\uNNNN`, no
/// octal — so `"\d"` is `d` and `"\\"` is a single backslash.
pub fn unescape_string_body(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            // A trailing lone backslash: ein.py's loop appends it as-is
            // (`i + 1 < len(body)` fails, so the `else` branch runs).
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str, term: Term) -> Option<&str> {
        match_term(src, Cursor::START, term).map(|l| l.text(src))
    }

    #[test]
    fn symbol_rejects_reserved_words_at_a_word_boundary() {
        // Excluded outright, or followed by a non-word character.
        for s in ["rule", "rule-x", "rule.x", "not", "import", "neq"] {
            assert_eq!(lex(s, Term::Symbol), None, "{s}");
        }
        // A word character after the reserved word kills the `\b`, so the
        // longer identifier is an ordinary atom.
        for s in ["rulex", "rule_x", "not_a", "neq_test", "importfoo.bar"] {
            assert_eq!(lex(s, Term::Symbol), Some(s), "{s}");
        }
        // Start-anchored: only a *leading* reserved word is excluded.
        assert_eq!(lex("std.rule", Term::Symbol), Some("std.rule"));
        assert_eq!(lex("__closed__", Term::Symbol), Some("__closed__"));
        assert_eq!(lex("__rule", Term::Symbol), Some("__rule"));
    }

    #[test]
    fn a_single_underscore_is_the_wildcard_and_a_dunder_is_not() {
        assert_eq!(lex("_", Term::Wildcard), Some("_"));
        assert_eq!(lex("_ x", Term::Wildcard), Some("_"));
        assert_eq!(lex("__closed__", Term::Wildcard), None);
        assert_eq!(lex("_x", Term::Wildcard), None);
        assert_eq!(lex("_x", Term::Symbol), None);
    }

    #[test]
    fn range_is_digit_anchored_and_beats_int() {
        assert_eq!(lex("1..5", Term::Range), Some("1..5"));
        assert_eq!(lex("12..*", Term::Range), Some("12..*"));
        assert_eq!(lex("1..", Term::Range), None);
        assert_eq!(lex("1..5", Term::Int), Some("1"));
        assert_eq!(lex("-5", Term::Int), Some("-5"));
        assert_eq!(lex("-", Term::Int), None);
    }

    #[test]
    fn strings_take_a_raw_newline_but_not_an_escaped_one() {
        assert_eq!(lex(r#""a\nb""#, Term::Str), Some(r#""a\nb""#));
        assert_eq!(lex("\"a\nb\"", Term::Str), Some("\"a\nb\""));
        assert_eq!(lex("\"a\\\nb\"", Term::Str), None);
        assert_eq!(lex(r#""unterminated"#, Term::Str), None);
    }

    #[test]
    fn unescape_is_the_minimal_set() {
        assert_eq!(unescape_string_body(r"a\nb"), "a\nb");
        assert_eq!(unescape_string_body(r"a\tb\rc"), "a\tb\rc");
        assert_eq!(unescape_string_body(r"\d"), "d");
        assert_eq!(unescape_string_body(r"\\"), "\\");
        assert_eq!(unescape_string_body(r#"\""#), "\"");
        assert_eq!(unescape_string_body(r"trailing\"), "trailing\\");
    }

    #[test]
    fn trivia_stops_at_an_unterminated_block_comment() {
        let src = "  ; line\n #| block |# x";
        let c = skip_trivia(src, Cursor::START);
        assert_eq!(&src[c.pos..], "x");
        let src = "  #| never closed";
        let c = skip_trivia(src, Cursor::START);
        assert_eq!(&src[c.pos..], "#| never closed");
    }

    #[test]
    fn positions_are_one_based_and_counted_in_characters() {
        let src = "; é\n  (a)";
        let l = match_term(src, Cursor::START, Term::LParen).expect("lparen");
        assert_eq!((l.at.line, l.at.col), (2, 3));
        assert_eq!(l.at.chars, 6);
    }

    #[test]
    fn a_literal_matches_a_prefix_of_a_longer_symbol() {
        // The whole reason literals are matched separately: both readings
        // exist at this position and the parser tries both.
        let src = "rulex";
        assert_eq!(lex(src, Term::Symbol), Some("rulex"));
        let l = match_literal(src, Cursor::START, "rule").expect("literal");
        assert_eq!(l.text(src), "rule");
        assert_eq!(&src[l.next.pos..], "x");
    }
}
