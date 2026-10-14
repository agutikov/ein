//! Integer and float coercion, for the two kinds of `(config …)` value that
//! are numbers.
//!
//! A flag's value may arrive as an atom's *name* or a string's body and still
//! coerce. Both functions accept a little more than Rust's `FromStr` does:
//!
//! - surrounding whitespace, trimmed (`str::trim` — Unicode `White_Space`);
//! - `_` digit separators, each of which must sit **between two ASCII
//!   digits** — `1_000` and `1_0.5` coerce, `_1`, `1_`, `1__0`, `1_.0` and
//!   `1._0` do not.
//!
//! Rejecting those would turn a puzzle that loads into one that does not.

/// An integer, for the widths a `SolverConfig` field can hold.
///
/// `None` is a malformed value, which the caller turns into the load message.
/// A literal wider than `i64` is also `None`: the fields are seeds and
/// counters, and no puzzle has one.
pub fn parse_int(text: &str) -> Option<i64> {
    let cleaned = strip_separators(text.trim())?;
    cleaned.parse::<i64>().ok()
}

/// A float.
///
/// `inf` / `infinity` / `nan` (case-insensitively, sign included), a bare `1.`
/// or `.5`, and an exponent are all accepted — by `f64::from_str` itself, so
/// whitespace and separators are all this adds.
pub fn parse_float(text: &str) -> Option<f64> {
    let cleaned = strip_separators(text.trim())?;
    cleaned.parse::<f64>().ok()
}

/// Remove `_` separators, rejecting any that does not sit **between** two
/// digits.
fn strip_separators(text: &str) -> Option<String> {
    if !text.contains('_') {
        return Some(text.to_string());
    }
    let bytes = text.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b != b'_' {
            continue;
        }
        let before = i.checked_sub(1).map(|j| bytes[j]);
        let after = bytes.get(i + 1).copied();
        if !before.is_some_and(|c| c.is_ascii_digit()) || !after.is_some_and(|c| c.is_ascii_digit())
        {
            return None;
        }
    }
    Some(text.replace('_', ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coercion_accepts_more_than_from_str_does() {
        assert_eq!(parse_int("  -42  "), Some(-42));
        assert_eq!(parse_int("1_000"), Some(1000));
        assert_eq!(parse_int("+7"), Some(7));
        assert_eq!(parse_float(" 1_0.5 "), Some(10.5));
        assert_eq!(parse_float("1e3"), Some(1000.0));
        assert_eq!(parse_float(".5"), Some(0.5));
        assert_eq!(parse_float("1."), Some(1.0));
        assert!(parse_float("-Infinity").is_some_and(|v| v.is_infinite() && v < 0.0));
        assert!(parse_float("nan").is_some_and(f64::is_nan));
    }

    #[test]
    fn a_misplaced_separator_is_rejected() {
        for bad in ["_1", "1_", "1__0", "1_.0", "1._0"] {
            assert_eq!(parse_int(bad), None, "parse_int({bad:?})");
            assert_eq!(parse_float(bad), None, "parse_float({bad:?})");
        }
        assert_eq!(parse_int("lex"), None);
        assert_eq!(parse_int("1.5"), None, "an int does not truncate a decimal");
        assert_eq!(parse_float("lex"), None);
    }
}
