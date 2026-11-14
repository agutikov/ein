//! **The one printer**: a value as Ein text the loader reads back to the same
//! value (M1e2 S1e2.3.8, P1e2.3's N5).
//!
//! A fact argument is a name (a [`crate::Symbol`]) or a nested fact. A name
//! prints **bare** where its text reads back as one name under the name rule
//! (M1f S1f.5.4's ruling, the parser's since S1f.5.5's swap): a run of
//! [name characters](is_name_char), not a bare `_`, not one of the thirteen
//! words that are never a name ([`LEXER_KEYWORDS`]) and not one of the
//! engine's three writeback names. So `1`, `07`, `-7`, `1..5`, `it's` and
//! `[a]` print bare, and `(p 1) (p 01)` print as two facts. Anything else
//! prints as a **quoted string**: a space, a parenthesis, `?`, `:`, `#`, a
//! non-ASCII letter, the empty name, a reserved word. Since S1f.5.5's swap a
//! string is no value a program can write (S1f.5.2's ruling), so a quoted
//! name is one the engine made (the loader's `#…` names) and does not load
//! back. A nested fact prints as `(rel arg …)` with its arguments printed the
//! same way.
//!
//! Until S1e2.3.8 every surface printed a name as its raw text, so the int
//! `1` and the name `"1"` printed alike, `(holds "x y")` printed as the
//! two-argument `(holds x y)`, and a `--dump-answer` state file did not load
//! back to its state. Since S1f.5.5's swap there is no int: a run of digits
//! is a name.
//!
//! The lexer is `ein-ir`'s and this crate is below it, so [`is_symbol`] is a
//! second statement of the name rule; `ein-ir`'s
//! `the_printers_symbol_is_the_lexers` holds the two to one answer.

use std::borrow::Cow;

/// The thirteen words that are never a name (`ein-ir`'s
/// `grammar::RESERVED_WORDS`, tokens of their own in `ein.l`): a name that is
/// one of them prints quoted. A longer run (`not-red`, `rule.x`, `neq-1`) is
/// the name it spells, a reserved word being a whole token. `open`, the
/// verdict, joined them at M1f S1f.5.23 (the user's ruling on Q-M1f.66): no
/// program can write a name `open`, and a form headed `open` is the verdict,
/// which [`head`] prints bare.
pub const LEXER_KEYWORDS: &[&str] = &[
    "not", "and", "or", "neq", "rule", "hrule", "query", "solve", "config", "trace", "macro",
    "import", "open",
];

/// A character of a name, under the name rule (M1f S1f.5.4's ruling, the
/// user's of 2026-11-07, `ein.l`'s `NAME`): ASCII letters and digits, the
/// connectors `_ - .`, the decorators `$ % ^ * + = < > ! & @ ~ /`, and, for
/// now, `' ` [ ] { }`. Nothing else: no `? : ; # | , \ " ( )`, no space, no
/// character outside ASCII.
pub fn is_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || matches!(
            ch,
            '_' | '-'
                | '.'
                | '$'
                | '%'
                | '^'
                | '*'
                | '+'
                | '='
                | '<'
                | '>'
                | '!'
                | '&'
                | '@'
                | '~'
                | '/'
                | '\''
                | '`'
                | '['
                | ']'
                | '{'
                | '}'
        )
}

/// Does `s`, written bare, read back as one name? A run of
/// [name characters](is_name_char) that is not a bare `_`, not one of the
/// thirteen words that are never a name ([`LEXER_KEYWORDS`]), and not one of
/// the engine's three writeback names (`ein.l`'s `WRITEBACK`, T1f.5.5.11). The
/// ten words a position gives a meaning to (`relation`, `model`, …) are
/// names wherever a name goes, so they print bare.
pub fn is_symbol(s: &str) -> bool {
    !s.is_empty()
        && s != "_"
        && s.chars().all(is_name_char)
        && !LEXER_KEYWORDS.contains(&s)
        && ![
            crate::terms::FORCED_POSITIVE,
            crate::terms::LOOKAHEAD_DIES,
            crate::terms::MONOTONIC_UNCONDITIONAL,
        ]
        .contains(&s)
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

/// The first character of every name the parser, the loader and the engine
/// make up for something a program writes no name for ([`INTERNAL_NAMES`]).
///
/// `#` is no name character in either lexer: the name rule's list leaves it
/// out (the user, 2026-11-07: *"`;`,`#`,`|` - for comments, also not allowed
/// in names"*), the generated lexer reads a `#` that does not open a `#|`
/// comment as part of a token that is not a name, and the hand-written lexer
/// starts no token with it. None of these names is followed by `|`, so none
/// opens a comment either. Until M1f S1f.5.5's T1f.5.5.8 (2026-11-08) they were
/// spelled with `@` and `<…>`, which the name rule makes name characters, so
/// that `(p ())` and `(p (@empty))` would have loaded as one fact.
pub const INTERNAL_MARK: char = '#';

/// The relation name the loader gives a nested form whose head is not an atom
/// (`(r (?x a))`, `(r (_ a))`): it is no name text can spell, and `_` in head
/// position loads as it, so a nested fact with this head prints `(_ …)`.
pub const NESTED_HEAD: &str = "#nested";

/// The relation name the loader gives the empty form `()`, which prints as
/// `()`.
pub const EMPTY_HEAD: &str = "#empty";

/// The head the parser gives a rule's or a macro's parameter list, which the
/// dumper prints without a head and the loader reads as no relation.
pub const PARAMS_HEAD: &str = "#params";

/// The head the generated parser gives a fact's `:using` list, which the
/// dumper prints without a head and the loader does not read.
pub const LIST_HEAD: &str = "#list";

/// Every name the parser, the loader and the engine make up for something a
/// program writes no name for, each starting with [`INTERNAL_MARK`], so no
/// program can spell one (`ein-ir/tests/generated_parser.rs`,
/// `no_name_the_loader_makes_up_is_a_name`).
pub const INTERNAL_NAMES: [&str; 6] = [
    EMPTY_HEAD,
    NESTED_HEAD,
    PARAMS_HEAD,
    LIST_HEAD,
    crate::terms::QUERY_RULE,
    crate::terms::WRITTEN_STEP,
];

/// Is `name` one the parser, the loader or the engine made up
/// ([`INTERNAL_NAMES`]), rather than one a program wrote?
pub fn is_internal(name: &str) -> bool {
    name.starts_with(INTERNAL_MARK)
}

/// A fact's head as Ein text: a relation is a `SYMBOL`, a kernel form's head
/// (`not`, `and`, `or`, `neq`, `=`, and `open`, the verdict, since M1f
/// S1f.5.23) reads back as that form, and the two names
/// the loader gives a nested form with no atom head print as what loads them
/// ([`NESTED_HEAD`], [`EMPTY_HEAD`]). Any other text is quoted, which does not
/// parse in head position: no corpus state holds one
/// (`ein-cli/tests/answer_round_trip.rs`).
pub fn head(rel: &str) -> Cow<'_, str> {
    match rel {
        "not" | "and" | "or" | "neq" | "=" | "open" => Cow::Borrowed(rel),
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

    /// Restated at S1f.5.5's swap (T1f.5.5.4): the name rule, so a run of
    /// digits, `1..5`, `it's` and a bracket print bare, and `(p 1)` and
    /// `(p 01)` print apart.
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
            "1",
            "01",
            "-7",
            "1..5",
            "not-red",
            "rule.x",
            "_x",
            "it's",
            "[a]",
            "{}",
            "$x",
            "a/b",
        ] {
            assert_eq!(symbol(s), s, "{s}");
        }
        for (s, want) in [
            ("x y", "\"x y\""),
            ("", "\"\""),
            ("not", "\"not\""),
            ("open", "\"open\""),
            ("?x", "\"?x\""),
            ("_", "\"_\""),
            (":kw", "\":kw\""),
            ("a:b", "\"a:b\""),
            ("a#b", "\"a#b\""),
            ("Åsa", "\"Åsa\""),
            ("a\"b", "\"a\\\"b\""),
            ("back\\slash", "\"back\\\\slash\""),
            ("line\nbreak", "\"line\\nbreak\""),
            ("#|", "\"#|\""),
            ("<forced-positive>", "\"<forced-positive>\""),
        ] {
            assert_eq!(symbol(s), want, "{s:?}");
        }
    }

    #[test]
    fn a_form_prints_its_head_as_what_loads_it() {
        assert_eq!(form("p", &[]), "(p)");
        assert_eq!(form("p", &["A".into()]), "(p A)");
        assert_eq!(form("not", &["(p A)".into()]), "(not (p A))");
        assert_eq!(form("open", &["likes".into()]), "(open likes)");
        assert_eq!(form("open", &[]), "(open)");
        assert_eq!(form(NESTED_HEAD, &["a".into()]), "(_ a)");
        assert_eq!(form(EMPTY_HEAD, &[]), "()");
    }

    #[test]
    fn every_internal_name_carries_the_mark_and_prints_quoted() {
        for name in INTERNAL_NAMES {
            assert!(is_internal(name), "{name}");
            assert_eq!(symbol(name), quote(name), "{name}");
        }
        for written in ["@empty", "<nested>", "@params", "@list", "<query>", "empty"] {
            assert!(!is_internal(written), "{written}");
        }
    }
}
