//! **The one printer**: a value as Ein text the loader reads back to the same
//! value (M1e2 S1e2.3.8, P1e2.3's N5).
//!
//! A fact argument is a name (a [`crate::Symbol`]), an int or a nested fact,
//! and the loader makes a name of every textual argument: an atom, a string,
//! a variable written in a fact (`?x` is the name `?x`) and a range (`1..5`
//! is the name `1..5`) all intern as the symbol they spell
//! ([`crate::Terms::value_text`]). So a name prints **bare** where its text
//! lexes back as one `SYMBOL` and is no reserved word, and as a **quoted
//! string** everywhere else: a space, a parenthesis, a quote, a leading digit,
//! `?`, `:` or `_`, a range's shape, a non-ASCII letter, the empty name, and
//! every word `SYMBOL` refuses ([`LEXER_KEYWORDS`] at a word boundary,
//! `00_ebnf.md` § 1). An int prints bare, as its canonical decimal. A nested
//! fact prints as `(rel arg …)` with its arguments printed the same way.
//!
//! Until S1e2.3.8 every surface printed a name as its raw text, so the int
//! `1` and the name `"1"` printed alike, `(holds "x y")` printed as the
//! two-argument `(holds x y)`, and a `--dump-answer` state file did not load
//! back to its state.
//!
//! The lexer is `ein-ir`'s and this crate is below it, so [`is_symbol`] is a
//! second statement of `SYMBOL`; `ein-ir`'s
//! `the_printers_symbol_is_the_lexers` holds the two to one answer.

use std::borrow::Cow;

/// The twelve words `SYMBOL`'s negative lookahead rejects (`00_ebnf.md` § 1,
/// `LEXER_KEYWORDS`): a name that is one of them, or starts with one followed
/// by a non-word character (`not-red`, `rule.x`), is no `SYMBOL` and prints
/// quoted. `ein-ir`'s lexer reads this list.
pub const LEXER_KEYWORDS: &[&str] = &[
    "not", "and", "or", "neq", "rule", "hrule", "query", "solve", "config", "trace", "macro",
    "import",
];

/// A word character, as the word boundary after a reserved word tests it:
/// `_`, Unicode `Alphabetic`, or a numeric general category — `00_ebnf.md`
/// § 1, `WordChar`.
pub fn is_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Does `s`, written bare, lex as exactly one `SYMBOL` that spells `s`?
/// `SYMBOL ::= ( '__' )? [A-Za-z] [A-Za-z0-9_*.-]*`, refused where the text
/// starts with one of [`LEXER_KEYWORDS`] at a word boundary.
pub fn is_symbol(s: &str) -> bool {
    for word in LEXER_KEYWORDS {
        if let Some(after) = s.strip_prefix(*word)
            && after.chars().next().is_none_or(|ch| !is_word(ch))
        {
            return false;
        }
    }
    let body = s.strip_prefix("__").unwrap_or(s);
    let mut chars = body.chars();
    match chars.next() {
        Some(ch) if ch.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '*' | '.' | '-'))
}

/// `s` as a `STRING` literal: `\` `"` newline, tab and carriage return
/// escaped (`\\` `\"` `\n` `\t` `\r`), every other character as itself. The
/// lexer's unescape is the inverse (`00_ebnf.md` § 1, fact 4), and since a
/// newline is escaped a printed value is always one line.
pub fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// A name as Ein text: bare where it [lexes as itself](is_symbol), else
/// [quoted](quote).
pub fn symbol(s: &str) -> Cow<'_, str> {
    if is_symbol(s) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(quote(s))
    }
}

/// The relation name the loader gives a nested form whose head is not an atom
/// (`(r (?x a))`, `(r (_ a))`): it is no name text can spell, and `_` in head
/// position loads as it, so a nested fact with this head prints `(_ …)`.
pub const NESTED_HEAD: &str = "<nested>";

/// The relation name the loader gives the empty form `()`, which prints as
/// `()`.
pub const EMPTY_HEAD: &str = "@empty";

/// A fact's head as Ein text: a relation is a `SYMBOL`, a kernel form's head
/// (`not`, `and`, `or`, `neq`, `=`) reads back as that form, and the two names
/// the loader gives a nested form with no atom head print as what loads them
/// ([`NESTED_HEAD`], [`EMPTY_HEAD`]). Any other text is quoted, which does not
/// parse in head position: no corpus state holds one
/// (`ein-cli/tests/answer_round_trip.rs`).
pub fn head(rel: &str) -> Cow<'_, str> {
    match rel {
        "not" | "and" | "or" | "neq" | "=" => Cow::Borrowed(rel),
        NESTED_HEAD => Cow::Borrowed("_"),
        _ => symbol(rel),
    }
}

/// `(head arg …)`, or `(head)` with no arguments, or `()` for the empty form.
pub fn form(rel: &str, args: &[String]) -> String {
    if rel == EMPTY_HEAD && args.is_empty() {
        return "()".to_string();
    }
    if args.is_empty() {
        format!("({})", head(rel))
    } else {
        format!("({} {})", head(rel), args.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_is_bare_only_where_it_lexes_as_itself() {
        for s in [
            "A",
            "House-1",
            "a.b",
            "neqw",
            "rulex",
            "__dunder__",
            "x*",
            "relation",
            "open",
        ] {
            assert_eq!(symbol(s), s, "{s}");
        }
        for (s, want) in [
            ("1", "\"1\""),
            ("x y", "\"x y\""),
            ("", "\"\""),
            ("not", "\"not\""),
            ("not-red", "\"not-red\""),
            ("rule.x", "\"rule.x\""),
            ("?x", "\"?x\""),
            ("_", "\"_\""),
            ("_x", "\"_x\""),
            (":kw", "\":kw\""),
            ("1..5", "\"1..5\""),
            ("Åsa", "\"Åsa\""),
            ("a\"b", "\"a\\\"b\""),
            ("back\\slash", "\"back\\\\slash\""),
            ("line\nbreak", "\"line\\nbreak\""),
            ("#|", "\"#|\""),
        ] {
            assert_eq!(symbol(s), want, "{s:?}");
        }
    }

    #[test]
    fn a_form_prints_its_head_as_what_loads_it() {
        assert_eq!(form("p", &[]), "(p)");
        assert_eq!(form("p", &["A".into()]), "(p A)");
        assert_eq!(form("not", &["(p A)".into()]), "(not (p A))");
        assert_eq!(form(NESTED_HEAD, &["a".into()]), "(_ a)");
        assert_eq!(form(EMPTY_HEAD, &[]), "()");
    }
}
