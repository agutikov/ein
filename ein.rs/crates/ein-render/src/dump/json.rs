//! A `json.dumps`-shaped writer.
//!
//! The dumps are compared byte for byte, so what matters is not "valid JSON"
//! but *CPython's* JSON: key order is a document property (insertion order,
//! or sorted under `sort_keys=True`), `indent=2` changes the separators as
//! well as the whitespace, and a float renders through `repr` rather than
//! through a format string.
//!
//! `serde_json` cannot express the first of those without the `preserve_order`
//! feature and an `indexmap` behind it, and would still need the float and the
//! separator rules bolted on. Two hundred lines here buys exactness and no
//! dependency — the same trade [`ein_core::pyrepr`] made for `repr()`.

/// A JSON value, with objects as ordered key-value lists.
#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    /// An integer, as its canonical decimal text.
    Int(i64),
    /// An integer too wide for `i64`, as its canonical decimal text. The IR's
    /// `INT` is unbounded and CPython writes a big integer as digits, so a
    /// writer that fell back to a string here would differ from `json.dumps`.
    BigInt(String),
    Float(f64),
    Str(String),
    Array(Vec<Json>),
    /// Insertion-ordered; [`dumps_indent_sorted`] can sort it.
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }

    pub fn str(s: impl Into<String>) -> Json {
        Json::Str(s.into())
    }

    pub fn int(n: impl Into<i64>) -> Json {
        Json::Int(n.into())
    }
}

/// `json.dumps(value)` — compact, insertion order, `', '` / `': '`.
pub fn dumps(value: &Json) -> String {
    let mut out = String::new();
    write(&mut out, value, None, 0, false);
    out
}

/// `json.dumps(value, indent=2, sort_keys=True)`.
pub fn dumps_indent_sorted(value: &Json) -> String {
    let mut out = String::new();
    write(&mut out, value, Some(2), 0, true);
    out
}

/// `json.dumps(value, indent=2)` — indented, insertion order.
pub fn dumps_indent(value: &Json) -> String {
    let mut out = String::new();
    write(&mut out, value, Some(2), 0, false);
    out
}

fn write(out: &mut String, value: &Json, indent: Option<usize>, depth: usize, sort: bool) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Int(n) => out.push_str(&n.to_string()),
        Json::BigInt(text) => out.push_str(text),
        Json::Float(f) => out.push_str(&float_repr(*f)),
        Json::Str(s) => write_str(out, s),
        Json::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                    if indent.is_none() {
                        out.push(' ');
                    }
                }
                newline_indent(out, indent, depth + 1);
                write(out, item, indent, depth + 1, sort);
            }
            newline_indent(out, indent, depth);
            out.push(']');
        }
        Json::Object(pairs) => {
            if pairs.is_empty() {
                out.push_str("{}");
                return;
            }
            let mut pairs: Vec<&(String, Json)> = pairs.iter().collect();
            if sort {
                pairs.sort_by(|a, b| a.0.cmp(&b.0));
            }
            out.push('{');
            for (i, (k, v)) in pairs.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                    if indent.is_none() {
                        out.push(' ');
                    }
                }
                newline_indent(out, indent, depth + 1);
                write_str(out, k);
                out.push_str(": ");
                write(out, v, indent, depth + 1, sort);
            }
            newline_indent(out, indent, depth);
            out.push('}');
        }
    }
}

/// `float.__repr__` — the shortest decimal that round-trips, which is what
/// Rust's `{}` already produces, plus the `.0` CPython keeps on an integral
/// value.
///
/// The two implementations part company where CPython switches to exponent
/// notation (|x| ≥ 1e16, or 0 < |x| < 1e-4) and Rust does not. Every float
/// that reaches this writer is a `round(_, 3)` of a clock reading in seconds
/// or milliseconds, so neither bound is in range — and those fields are on the
/// [normalisation list](../../../../../docs/history/m1a_rust/design/01_parity_contract.md) §5
/// anyway, because ein.py is not stable there either.
fn float_repr(f: f64) -> String {
    if f.is_nan() {
        return "NaN".to_string();
    }
    if f.is_infinite() {
        return if f > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let mut s = format!("{f}");
    if !s.contains(['.', 'e', 'E']) {
        s.push_str(".0");
    }
    s
}

fn newline_indent(out: &mut String, indent: Option<usize>, depth: usize) {
    if let Some(n) = indent {
        out.push('\n');
        out.push_str(&" ".repeat(n * depth));
    }
}

/// CPython's `py_encode_basestring_ascii` — the `ensure_ascii=True` default.
///
/// Control characters take their short escape where one exists and `\uXXXX`
/// otherwise; every non-ASCII character becomes `\uXXXX`, with a surrogate
/// pair above the BMP.
fn write_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) < 0x7f => out.push(c),
            c => {
                let cp = c as u32;
                if cp <= 0xFFFF {
                    out.push_str(&format!("\\u{cp:04x}"));
                } else {
                    let v = cp - 0x10000;
                    out.push_str(&format!("\\u{:04x}", 0xD800 + (v >> 10)));
                    out.push_str(&format!("\\u{:04x}", 0xDC00 + (v & 0x3FF)));
                }
            }
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_separators_are_pythons() {
        let v = Json::obj(vec![
            ("b", Json::int(1)),
            (
                "a",
                Json::Array(vec![Json::str("x"), Json::Bool(true), Json::Null]),
            ),
        ]);
        assert_eq!(dumps(&v), r#"{"b": 1, "a": ["x", true, null]}"#);
    }

    #[test]
    fn indent_two_drops_the_space_and_sorts_when_asked() {
        let v = Json::obj(vec![("b", Json::int(1)), ("a", Json::int(2))]);
        assert_eq!(dumps_indent(&v), "{\n  \"b\": 1,\n  \"a\": 2\n}");
        assert_eq!(dumps_indent_sorted(&v), "{\n  \"a\": 2,\n  \"b\": 1\n}");
        assert_eq!(dumps_indent(&Json::Array(vec![])), "[]");
        assert_eq!(dumps_indent(&Json::Object(vec![])), "{}");
    }

    #[test]
    fn a_float_keeps_its_point() {
        assert_eq!(dumps(&Json::Float(1.0)), "1.0");
        assert_eq!(dumps(&Json::Float(0.125)), "0.125");
        assert_eq!(dumps(&Json::Float(-0.0)), "-0.0");
    }

    #[test]
    fn non_ascii_is_escaped_as_cpython_escapes_it() {
        // `ensure_ascii=True` is CPython's default, and the two `ein.py` call
        // sites that overrode it are accounted for: `_events.py` did, and
        // `ein-infer`'s event writer reproduces that (`events.rs`); `_summary.py`
        // did, and `ein-cli`'s `summary::write` deliberately does not — M1e
        // S1e.4.8, `MA-L3`, where the reason is written. Everything else took
        // the default, and this writer is what those callers get.
        assert_eq!(dumps(&Json::str("é")), r#""\u00e9""#);
        assert_eq!(dumps(&Json::str("⊥")), r#""\u22a5""#);
        assert_eq!(dumps(&Json::str("𝄞")), r#""\ud834\udd1e""#);
        assert_eq!(dumps(&Json::str("a\tb\"c\\d\n")), r#""a\tb\"c\\d\n""#);
    }
}
