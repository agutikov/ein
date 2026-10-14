//! `repr` — the textual form of a value, for the four shapes a fact argument
//! can take: `str`, `int`, `tuple` and `Fact`.
//!
//! The canonical state key sorts by it, the explanation tie-breaks order by
//! it and the DOT labels print it
//! ([design/02](../../../../docs/history/m1a_rust/design/02_determinism_and_order.md) §7),
//! so its bytes are observable. The format:
//!
//! - **str** — quoted in `'`, unless the body contains a `'` and no `"`, in
//!   which case in `"` so that nothing needs escaping. The chosen quote and
//!   `\` are backslash-escaped; tab, newline and carriage return are `\t`
//!   `\n` `\r`; any other C0 control and DEL is `\xNN`; any other
//!   non-printable code point — general category `Cc` `Cf` `Cs` `Co` `Cn`
//!   `Zl` `Zp` `Zs`, ASCII space excepted (`printable.rs`) — is `\xNN` below
//!   U+0100, `\uNNNN` below U+10000 and `\UNNNNNNNN` above, lower-case hex;
//!   every other character is written as itself.
//! - **int** — canonical decimal text ([`canonical_int`]).
//! - **tuple** — `(a, b)`, items separated by `, `; one item is `(a,)` and
//!   none is `()`.
//! - **Fact** — `Fact(relation_name='p', args=('A',))`: the relation name as
//!   a `str` and the args as a tuple, a nested fact recursively.
//!
//! Ints are carried as their **canonical decimal text**, not as an `i64`:
//! `INT: /-?[0-9]+/` accepts any width, so a fixed-width parse would reject
//! literals that load.

use crate::printable::is_printable;

/// The value shapes reachable from a fact argument. `Fact` is spelled out
/// rather than referenced because [`crate`]'s fact store lands in
/// [P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core); the renderer
/// only ever needed the relation name and the args.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReprValue {
    Str(String),
    /// Canonical decimal text, sign included.
    Int(String),
    Tuple(Vec<ReprValue>),
    Fact {
        relation_name: String,
        args: Vec<ReprValue>,
    },
}

/// The `repr` of `value`.
pub fn repr(value: &ReprValue) -> String {
    let mut out = String::new();
    write_repr(&mut out, value);
    out
}

fn write_repr(out: &mut String, value: &ReprValue) {
    match value {
        ReprValue::Str(s) => out.push_str(&repr_str(s)),
        ReprValue::Int(i) => out.push_str(i),
        ReprValue::Tuple(items) => {
            out.push('(');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_repr(out, item);
            }
            // `(a,)` — the trailing comma that distinguishes a 1-tuple from a
            // parenthesised expression, and `()` for the empty one.
            if items.len() == 1 {
                out.push(',');
            }
            out.push(')');
        }
        ReprValue::Fact {
            relation_name,
            args,
        } => {
            out.push_str("Fact(relation_name=");
            out.push_str(&repr_str(relation_name));
            out.push_str(", args=");
            write_repr(out, &ReprValue::Tuple(args.clone()));
            out.push(')');
        }
    }
}

/// The `repr` of a `str`.
///
/// The quote is `'` unless the body contains a `'` and no `"`, in which case
/// it switches to `"` to avoid escaping — so `"it's"` reprs as `"it's"` but a
/// string with both quotes reprs as `'both \' and "'`.
pub fn repr_str(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for ch in s.chars() {
        match ch {
            _ if ch == quote || ch == '\\' => {
                out.push('\\');
                out.push(ch);
            }
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ if (ch as u32) < 0x20 || ch as u32 == 0x7f => {
                out.push_str(&format!("\\x{:02x}", ch as u32));
            }
            _ if (ch as u32) < 0x7f => out.push(ch),
            _ if is_printable(ch) => out.push(ch),
            _ => {
                let cp = ch as u32;
                if cp < 0x100 {
                    out.push_str(&format!("\\x{cp:02x}"));
                } else if cp < 0x10000 {
                    out.push_str(&format!("\\u{cp:04x}"));
                } else {
                    out.push_str(&format!("\\U{cp:08x}"));
                }
            }
        }
    }
    out.push(quote);
    out
}

/// Canonical decimal text for an integer literal: leading zeros dropped, and
/// a `-` only in front of a non-zero value.
///
/// `007` → `7`, `-007` → `-7`, `-0` → `0`. Done on the digits rather than
/// through an integer type because `INT: /-?[0-9]+/` accepts any width; an
/// `i64` parse would reject literals that load.
///
/// Lives here rather than beside the lexer because it is the text
/// [`ReprValue::Int`] carries, and because
/// [P1a.2](../../../../docs/history/m1a_rust/README.md#p1a2--kb-core)'s int pool
/// stores exactly this form
/// ([design/03](../../../../docs/history/m1a_rust/design/03_data_model.md) §3).
pub fn canonical_int(text: &str) -> String {
    let (neg, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let trimmed = digits.trim_start_matches('0');
    let body = if trimmed.is_empty() { "0" } else { trimmed };
    if neg && body != "0" {
        format!("-{body}")
    } else {
        body.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(x: &str) -> ReprValue {
        ReprValue::Str(x.to_string())
    }

    #[test]
    fn the_quote_switches_only_to_avoid_escaping() {
        assert_eq!(repr_str("a"), "'a'");
        assert_eq!(repr_str("it's"), "\"it's\"");
        assert_eq!(repr_str("say \"hi\""), "'say \"hi\"'");
        assert_eq!(repr_str("both ' and \""), "'both \\' and \"'");
    }

    #[test]
    fn escapes_follow_the_repr_table_not_rusts_debug() {
        assert_eq!(repr_str("a\tb\nc\rd"), "'a\\tb\\nc\\rd'");
        assert_eq!(repr_str("\u{0}\u{1f}\u{7f}"), "'\\x00\\x1f\\x7f'");
        assert_eq!(repr_str("\u{a0}"), "'\\xa0'");
        assert_eq!(repr_str("\u{200b}"), "'\\u200b'");
        assert_eq!(repr_str("Åsa"), "'Åsa'");
        assert_eq!(repr_str("😀"), "'😀'");
        assert_eq!(repr_str("back\\slash"), "'back\\\\slash'");
    }

    #[test]
    fn canonical_int_drops_leading_zeros_and_the_sign_of_zero() {
        for (input, want) in [
            ("0", "0"),
            ("007", "7"),
            ("-0", "0"),
            ("-007", "-7"),
            ("-000", "0"),
            ("12", "12"),
            ("-12", "-12"),
            // Wider than i64 — the case that rules out parsing to an integer.
            (
                "123456789012345678901234567890",
                "123456789012345678901234567890",
            ),
            (
                "00123456789012345678901234567890",
                "123456789012345678901234567890",
            ),
        ] {
            assert_eq!(canonical_int(input), want, "canonical_int({input:?})");
        }
    }

    #[test]
    fn a_one_tuple_keeps_its_comma() {
        assert_eq!(repr(&ReprValue::Tuple(vec![])), "()");
        assert_eq!(repr(&ReprValue::Tuple(vec![s("a")])), "('a',)");
        assert_eq!(repr(&ReprValue::Tuple(vec![s("a"), s("b")])), "('a', 'b')");
        assert_eq!(
            repr(&ReprValue::Tuple(vec![ReprValue::Int("-7".into()), s("b")])),
            "(-7, 'b')"
        );
    }

    #[test]
    fn a_fact_reprs_as_its_relation_name_and_args() {
        let inner = ReprValue::Fact {
            relation_name: "co-located".into(),
            args: vec![s("Norwegian"), s("House-2")],
        };
        assert_eq!(
            repr(&inner),
            "Fact(relation_name='co-located', args=('Norwegian', 'House-2'))"
        );
        // A nested fact is an ordinary arg — the relational-node duality.
        let outer = ReprValue::Fact {
            relation_name: "hypothesis".into(),
            args: vec![inner],
        };
        assert_eq!(
            repr(&outer),
            "Fact(relation_name='hypothesis', args=(Fact(relation_name='co-located', \
             args=('Norwegian', 'House-2')),))"
        );
    }
}
