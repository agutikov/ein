//! Python's `repr()`, for the four shapes ein.py's observable output leans on.
//!
//! Several T3 sites sort or print `repr()` of a Python value —
//! `canon.state_key`'s `key=repr`, the explanation tie-breaks, the DOT
//! labels. ein.rs has no Python, so it needs a faithful renderer for exactly
//! `str`, `int`, `tuple` and `Fact`
//! ([design/02](../../../../plans/m1a_rust/design/02_determinism_and_order.md) §7).
//!
//! The alternative — rewriting those `key=repr` sorts in ein.py as explicit
//! comparators — was considered and rejected: it re-baselines existing goldens
//! and edits M1 code for the port's convenience, which the milestone's
//! non-goals forbid.
//!
//! Ints are carried as their **canonical decimal text**, not as an `i64`:
//! Python's integers are unbounded and `INT: /-?[0-9]+/` accepts any width, so
//! a fixed-width parse would reject values ein.py prints fine.

use crate::printable::is_printable;

/// The value shapes reachable from a fact argument. `Fact` is spelled out
/// rather than referenced because [`crate`]'s fact store lands in
/// [P1a.2](../../../../plans/m1a_rust/p1a.2_kb_core/README.md); the renderer
/// only ever needed the two fields that survive `repr=False`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyValue {
    Str(String),
    /// Canonical decimal text, sign included.
    Int(String),
    Tuple(Vec<PyValue>),
    Fact {
        relation_name: String,
        args: Vec<PyValue>,
    },
}

/// `repr(value)`.
pub fn repr(value: &PyValue) -> String {
    let mut out = String::new();
    write_repr(&mut out, value);
    out
}

fn write_repr(out: &mut String, value: &PyValue) {
    match value {
        PyValue::Str(s) => out.push_str(&repr_str(s)),
        PyValue::Int(i) => out.push_str(i),
        PyValue::Tuple(items) => {
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
        PyValue::Fact {
            relation_name,
            args,
        } => {
            out.push_str("Fact(relation_name=");
            out.push_str(&repr_str(relation_name));
            out.push_str(", args=");
            write_repr(out, &PyValue::Tuple(args.clone()));
            out.push(')');
        }
    }
}

/// `repr(s)` for a `str`, following CPython's `unicode_repr`.
///
/// The quote is `'` unless the body contains a `'` and no `"`, in which case
/// CPython switches to `"` to avoid escaping — so `"it's"` reprs as `"it's"`
/// but a string with both quotes reprs as `'both \' and "'`.
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

/// Canonical decimal text for an integer — `str(int(text))`.
///
/// `007` → `7`, `-007` → `-7`, `-0` → `0`. Done on the digits rather than
/// through an integer type because `INT: /-?[0-9]+/` accepts any width and
/// Python's `int` is unbounded; an `i64` parse would reject inputs ein.py
/// accepts.
///
/// Lives here rather than beside the lexer because it is the same question
/// [`PyValue::Int`] answers — what Python would have printed — and because
/// [P1a.2](../../../../plans/m1a_rust/p1a.2_kb_core/README.md)'s int pool
/// stores exactly this form
/// ([design/03](../../../../plans/m1a_rust/design/03_data_model.md) §3).
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

    fn s(x: &str) -> PyValue {
        PyValue::Str(x.to_string())
    }

    #[test]
    fn the_quote_switches_only_to_avoid_escaping() {
        assert_eq!(repr_str("a"), "'a'");
        assert_eq!(repr_str("it's"), "\"it's\"");
        assert_eq!(repr_str("say \"hi\""), "'say \"hi\"'");
        assert_eq!(repr_str("both ' and \""), "'both \\' and \"'");
    }

    #[test]
    fn escapes_follow_cpython_not_rust() {
        assert_eq!(repr_str("a\tb\nc\rd"), "'a\\tb\\nc\\rd'");
        assert_eq!(repr_str("\u{0}\u{1f}\u{7f}"), "'\\x00\\x1f\\x7f'");
        assert_eq!(repr_str("\u{a0}"), "'\\xa0'");
        assert_eq!(repr_str("\u{200b}"), "'\\u200b'");
        assert_eq!(repr_str("Åsa"), "'Åsa'");
        assert_eq!(repr_str("😀"), "'😀'");
        assert_eq!(repr_str("back\\slash"), "'back\\\\slash'");
    }

    #[test]
    fn canonical_int_matches_python_int_then_str() {
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
        assert_eq!(repr(&PyValue::Tuple(vec![])), "()");
        assert_eq!(repr(&PyValue::Tuple(vec![s("a")])), "('a',)");
        assert_eq!(repr(&PyValue::Tuple(vec![s("a"), s("b")])), "('a', 'b')");
        assert_eq!(
            repr(&PyValue::Tuple(vec![PyValue::Int("-7".into()), s("b")])),
            "(-7, 'b')"
        );
    }

    #[test]
    fn a_fact_reprs_as_its_dataclass_does() {
        let inner = PyValue::Fact {
            relation_name: "co-located".into(),
            args: vec![s("Norwegian"), s("House-2")],
        };
        assert_eq!(
            repr(&inner),
            "Fact(relation_name='co-located', args=('Norwegian', 'House-2'))"
        );
        // A nested fact is an ordinary arg — the relational-node duality.
        let outer = PyValue::Fact {
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
