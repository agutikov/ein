//! `:why` template substitution — `ein.py`'s `inference/why.py`.
//!
//! Q31 picked `{?var}` notation: the `?` is part of the reference, identical
//! to the variable as it appears in `:match` / `:assert`. A bare `{x}` is a
//! literal and is left alone, so an author can put braces in a message.
//!
//! `(relation …)` `:why` templates reference argument slots *positionally* —
//! `{?1}` is the first argument, `{?2}` the second — which is why a reference
//! may start with a digit. Unbound references keep their reference text; the
//! trace renderer prefers a partial render over a hard failure.
//!
//! This is nominally
//! [S1a.5.2](../../../../docs/history/m1a_rust/README.md#s1a52--trace-and-answer-rendering)
//! T6, landed here because [`crate::slice`] labels every rule node with a
//! rendered `:why` and cannot be checked without one.

/// Substitute `{?ref}` references against `bindings`, an association list of
/// `(name, rendered value)`.
///
/// ein.py's regex is `\{\?([A-Za-z0-9][A-Za-z0-9_-]*)\}`, scanned here
/// directly: a reference is `{?`, one alphanumeric, then any run of
/// alphanumerics / `_` / `-`, then `}`.
pub fn render_why(template: &str, bindings: &[(String, String)]) -> String {
    let bytes = template.as_bytes();
    let mut out = String::with_capacity(template.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' && i + 2 < bytes.len() && bytes[i + 1] == b'?' {
            let start = i + 2;
            let mut j = start;
            // The first character must be alphanumeric; the rest may add
            // `_` and `-`.
            if j < bytes.len() && bytes[j].is_ascii_alphanumeric() {
                j += 1;
                while j < bytes.len()
                    && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_' || bytes[j] == b'-')
                {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b'}' {
                    let name = &template[start..j];
                    match bindings.iter().find(|(k, _)| k == name) {
                        Some((_, v)) => out.push_str(v),
                        None => out.push_str(&template[i..=j]),
                    }
                    i = j + 1;
                    continue;
                }
            }
        }
        // Not a reference — copy one character and carry on.
        let ch = template[i..].chars().next().expect("in bounds");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    #[test]
    fn a_bound_reference_is_substituted_and_an_unbound_one_is_kept() {
        let bindings = b(&[("rel", "co-located"), ("a", "Norwegian"), ("b", "House-1")]);
        assert_eq!(
            render_why("{?rel} is transitive: {?a} →{?rel}→ {?b}", &bindings),
            "co-located is transitive: Norwegian →co-located→ House-1"
        );
        assert_eq!(
            render_why("{?missing} stays", &bindings),
            "{?missing} stays"
        );
    }

    #[test]
    fn a_positional_slot_is_a_reference_and_a_bare_brace_is_not() {
        assert_eq!(
            render_why(
                "{?1} is drunk in {?2}",
                &b(&[("1", "Water"), ("2", "House-1")])
            ),
            "Water is drunk in House-1"
        );
        assert_eq!(
            render_why("{x} and {?} and {}", &b(&[("x", "no")])),
            "{x} and {?} and {}"
        );
    }
}
