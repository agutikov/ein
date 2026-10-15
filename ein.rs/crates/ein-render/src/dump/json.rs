//! The dumps' JSON writer.
//!
//! The dumps are compared byte for byte, so what matters is not "valid JSON"
//! but one exact spelling of it: key order is a document property (insertion
//! order, or sorted when asked), indenting changes the separators as well as
//! the whitespace — `", "` / `": "` compact, `","` / `": "` indented — a
//! float is the shortest decimal that round-trips rather than a format
//! string, and everything outside printable ASCII is a `\uXXXX` escape.
//!
//! `serde_json` cannot express the first of those without the `preserve_order`
//! feature and an `indexmap` behind it, and would still need the float and the
//! separator rules bolted on. Two hundred lines here buys exactness and no
//! dependency — the same trade [`ein_core::repr`] made for a value's printed
//! form.

/// A JSON value, with objects as ordered key-value lists.
#[derive(Clone, Debug)]
pub enum Json {
    Null,
    Bool(bool),
    /// An integer, as its canonical decimal text.
    Int(i64),
    /// An integer too wide for `i64`, as its canonical decimal text. The IR's
    /// `INT` is unbounded and a big integer is written as digits, so a writer
    /// that fell back to a string here would change its JSON type.
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

/// Compact, insertion order, `", "` / `": "`.
pub fn dumps(value: &Json) -> String {
    let mut out = String::new();
    write(&mut out, value, None, 0, false);
    out
}

/// Indented by two, keys sorted.
pub fn dumps_indent_sorted(value: &Json) -> String {
    let mut out = String::new();
    write(&mut out, value, Some(2), 0, true);
    out
}

/// Indented by two, insertion order.
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

/// The shortest decimal that round-trips, which is what Rust's `{}` already
/// produces, plus a `.0` on an integral value.
///
/// `{}` never switches to exponent notation, so a very large or very small
/// value is written out in full. Every float that reaches this writer is a
/// clock reading in seconds or milliseconds rounded to three decimals, so
/// neither extreme is in range — and those fields are blanked before any
/// comparison anyway, because no two runs agree on a clock.
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

/// A string literal, ASCII only.
///
/// Control characters take their short escape where one exists and `\uXXXX`
/// otherwise; every character outside printable ASCII becomes `\uXXXX`, with
/// a surrogate pair above the BMP.
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

// ── reading ─────────────────────────────────────────────────────────

impl Json {
    /// `self[key]` of an object, or `None`.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(pairs) => pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// An array's items; `&[]` for anything else, `null` included.
    pub fn items(&self) -> &[Json] {
        match self {
            Json::Array(v) => v,
            _ => &[],
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }
}

/// Parse one JSON document — what `ein render region` reads a dump back
/// with (M1e2 S1e2.1a.2). Every JSON text is accepted, not only this writer's
/// spelling: an integer that fits `i64` is [`Json::Int`], a wider one
/// [`Json::BigInt`], anything with a point or an exponent [`Json::Float`].
pub fn parse(text: &str) -> Result<Json, String> {
    let mut p = Parser {
        s: text.as_bytes(),
        i: 0,
    };
    p.ws();
    let v = p.value(0)?;
    p.ws();
    if p.i != p.s.len() {
        return Err(p.err("trailing characters"));
    }
    Ok(v)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

/// Nesting past this is refused rather than recursed into.
const MAX_DEPTH: usize = 512;

impl Parser<'_> {
    fn err(&self, what: &str) -> String {
        format!("JSON: {what} at byte {}", self.i)
    }

    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\t' | b'\n' | b'\r') {
            self.i += 1;
        }
    }

    fn eat(&mut self, lit: &str) -> Result<(), String> {
        if self.s[self.i..].starts_with(lit.as_bytes()) {
            self.i += lit.len();
            Ok(())
        } else {
            Err(self.err(&format!("expected `{lit}`")))
        }
    }

    fn value(&mut self, depth: usize) -> Result<Json, String> {
        if depth > MAX_DEPTH {
            return Err(self.err("nesting too deep"));
        }
        match self.s.get(self.i) {
            None => Err(self.err("unexpected end")),
            Some(b'n') => self.eat("null").map(|_| Json::Null),
            Some(b't') => self.eat("true").map(|_| Json::Bool(true)),
            Some(b'f') => self.eat("false").map(|_| Json::Bool(false)),
            Some(b'"') => self.string().map(Json::Str),
            Some(b'[') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::Array(out));
                }
                loop {
                    self.ws();
                    out.push(self.value(depth + 1)?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Json::Array(out));
                        }
                        _ => return Err(self.err("expected `,` or `]`")),
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Object(out));
                }
                loop {
                    self.ws();
                    if self.s.get(self.i) != Some(&b'"') {
                        return Err(self.err("expected a key"));
                    }
                    let k = self.string()?;
                    self.ws();
                    self.eat(":")?;
                    self.ws();
                    out.push((k, self.value(depth + 1)?));
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Json::Object(out));
                        }
                        _ => return Err(self.err("expected `,` or `}`")),
                    }
                }
            }
            Some(c) if *c == b'-' || c.is_ascii_digit() => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.i;
        if self.s.get(self.i) == Some(&b'-') {
            self.i += 1;
        }
        let mut float = false;
        while let Some(&c) = self.s.get(self.i) {
            match c {
                b'0'..=b'9' => {}
                b'.' | b'e' | b'E' | b'+' | b'-' => float = true,
                _ => break,
            }
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).expect("ASCII");
        if float {
            return text
                .parse::<f64>()
                .map(Json::Float)
                .map_err(|_| self.err("bad number"));
        }
        if text == "-" || text.is_empty() {
            return Err(self.err("bad number"));
        }
        Ok(match text.parse::<i64>() {
            Ok(n) => Json::Int(n),
            Err(_) => Json::BigInt(text.to_string()),
        })
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let h = self
            .s
            .get(self.i..self.i + 4)
            .and_then(|b| std::str::from_utf8(b).ok())
            .and_then(|t| u32::from_str_radix(t, 16).ok())
            .ok_or_else(|| self.err("bad \\u escape"))?;
        self.i += 4;
        Ok(h)
    }

    fn string(&mut self) -> Result<String, String> {
        self.i += 1; // the opening quote
        let mut out = String::new();
        loop {
            let start = self.i;
            while let Some(&c) = self.s.get(self.i) {
                if c == b'"' || c == b'\\' {
                    break;
                }
                self.i += 1;
            }
            out.push_str(
                std::str::from_utf8(&self.s[start..self.i]).map_err(|_| self.err("not UTF-8"))?,
            );
            match self.s.get(self.i) {
                None => return Err(self.err("unterminated string")),
                Some(b'"') => {
                    self.i += 1;
                    return Ok(out);
                }
                Some(_) => {
                    self.i += 1;
                    let e = *self
                        .s
                        .get(self.i)
                        .ok_or_else(|| self.err("unterminated escape"))?;
                    self.i += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hi = self.hex4()?;
                            let cp = if (0xD800..0xDC00).contains(&hi) {
                                self.eat("\\u")?;
                                let lo = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(self.err("unpaired surrogate"));
                                }
                                0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
                            } else {
                                hi
                            };
                            out.push(char::from_u32(cp).ok_or_else(|| self.err("bad code point"))?);
                        }
                        _ => return Err(self.err("bad escape")),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What this writer writes, this reader reads back to the same text.
    #[test]
    fn a_document_round_trips_through_parse() {
        let v = Json::obj(vec![
            ("s", Json::str("a\tb\"c\\d\né⊥𝄞")),
            ("n", Json::int(-42)),
            ("big", Json::BigInt("123456789012345678901234567890".into())),
            ("f", Json::Float(0.125)),
            (
                "a",
                Json::Array(vec![Json::Null, Json::Bool(true), Json::Object(vec![])]),
            ),
        ]);
        for text in [dumps(&v), dumps_indent(&v)] {
            let back = parse(&text).expect("parses");
            assert_eq!(dumps(&back), dumps(&v));
        }
        assert_eq!(
            parse("{\"k\": [1, 2]}")
                .unwrap()
                .get("k")
                .unwrap()
                .items()
                .len(),
            2
        );
        assert!(parse("[1,]").is_err());
        assert!(parse("{\"a\" 1}").is_err());
        assert!(parse("1 2").is_err());
        assert!(parse("\"\\ud834\"").is_err());
    }

    #[test]
    fn compact_separators_carry_a_space() {
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
    fn non_ascii_is_escaped_with_a_surrogate_pair_above_the_bmp() {
        // One writer does not take this escaping: `ein-infer`'s event writer
        // (`events.rs`) writes non-ASCII as it is. `ein-cli`'s
        // `summary::write` escapes, deliberately — M1e S1e.4.8, `MA-L3`, where
        // the reason is written. Everything else is written by this one.
        assert_eq!(dumps(&Json::str("é")), r#""\u00e9""#);
        assert_eq!(dumps(&Json::str("⊥")), r#""\u22a5""#);
        assert_eq!(dumps(&Json::str("𝄞")), r#""\ud834\udd1e""#);
        assert_eq!(dumps(&Json::str("a\tb\"c\\d\n")), r#""a\tb\"c\\d\n""#);
    }
}
