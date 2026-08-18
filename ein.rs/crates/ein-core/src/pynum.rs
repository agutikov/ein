//! CPython's `int(s)` and `float(s)`, for the two coercions `(config …)`
//! inherits.
//!
//! `SolverConfig.from_kw_pairs` calls the builtins on whatever the IR node
//! carried, so a flag's value may arrive as an atom's *name* or a string's
//! body and still coerce. The builtins accept a little more than Rust's
//! `FromStr` does — surrounding whitespace, and `_` separators between digits
//! (PEP 515) — and a port that quietly rejected those would turn a puzzle that
//! loads into one that does not.

/// `int(text)`, for the widths a `SolverConfig` field can hold.
///
/// `None` is CPython's `ValueError`, which the caller turns into the load
/// message. A literal wider than `i64` is also `None`, where CPython would
/// keep it: the fields are seeds and counters, no puzzle has one, and the
/// place a huge seed would matter is CPython's Mersenne seeding, which is
/// Q-M1a.5's problem and not this function's.
pub fn python_int(text: &str) -> Option<i64> {
    let cleaned = strip_separators(text.trim())?;
    cleaned.parse::<i64>().ok()
}

/// `float(text)`.
///
/// Rust's `f64::from_str` already matches CPython on `inf` / `infinity` /
/// `nan` (case-insensitively, sign included), on a bare `1.` or `.5`, and on
/// exponents; whitespace and separators are the whole difference.
pub fn python_float(text: &str) -> Option<f64> {
    let cleaned = strip_separators(text.trim())?;
    cleaned.parse::<f64>().ok()
}

/// Remove PEP 515 `_` separators, rejecting the placements CPython rejects:
/// one may only sit **between** two digits.
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
    fn the_builtins_accept_more_than_from_str_does() {
        assert_eq!(python_int("  -42  "), Some(-42));
        assert_eq!(python_int("1_000"), Some(1000));
        assert_eq!(python_int("+7"), Some(7));
        assert_eq!(python_float(" 1_0.5 "), Some(10.5));
        assert_eq!(python_float("1e3"), Some(1000.0));
        assert_eq!(python_float(".5"), Some(0.5));
        assert_eq!(python_float("1."), Some(1.0));
        assert!(python_float("-Infinity").is_some_and(|v| v.is_infinite() && v < 0.0));
        assert!(python_float("nan").is_some_and(f64::is_nan));
    }

    #[test]
    fn a_misplaced_separator_is_a_value_error_there_and_here() {
        for bad in ["_1", "1_", "1__0", "1_.0", "1._0"] {
            assert_eq!(python_int(bad), None, "int({bad:?})");
            assert_eq!(python_float(bad), None, "float({bad:?})");
        }
        assert_eq!(python_int("lex"), None);
        assert_eq!(python_int("1.5"), None, "int() does not truncate a decimal");
        assert_eq!(python_float("lex"), None);
    }
}
