//! `:why` template substitution.
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
//! **A reference names every variable a program can write** (W37): after
//! `{?` it reads the name rule's characters ([`crate::text::is_name_char`])
//! up to the first `}`, so `{?isR*}`, the stdlib's, and every variable the
//! name rule allows (`?_x_`, `?a.b`, `?n*`) is substituted. `}` is a name
//! character too, so a variable holding one cannot be named, and the loader
//! refuses a template that writes such a reference
//! ([`unreachable_references`], Q-M1f.11).
//!
//! It lives here, in the crate that depends on nothing
//! ([architecture.md § 3.1](../../../../docs/ein/ein.rs/architecture.md#31-ein-core)),
//! rather than beside the renderers above the engine (the trace, the slice
//! and the answer), because the obligation pass is a reader *below* them: an
//! `owe` event carries the rendered sentence, and `--events` is emitted from
//! `ein-infer`, which `ein-render` depends on rather than the other way
//! round. `ein_render::render_why` names it.

/// Substitute `{?ref}` references against `bindings`, an association list of
/// `(name, rendered value)`.
///
/// A reference is `{?`, one or more of the name rule's characters
/// ([`crate::text::is_name_char`]) other than `}`, then `}`: the variable's
/// name as the program writes it after its `?`, up to the first `}`. Every
/// name character is ASCII, so the scan is over bytes.
pub fn render_why(template: &str, bindings: &[(String, String)]) -> String {
    let bytes = template.as_bytes();
    let mut out = String::with_capacity(template.len());
    let mut i = 0;
    while i < bytes.len() {
        if let Some(end) = reference_end(bytes, i) {
            let name = &template[i + 2..end];
            match bindings.iter().find(|(k, _)| k == name) {
                Some((_, v)) => out.push_str(v),
                None => out.push_str(&template[i..=end]),
            }
            i = end + 1;
            continue;
        }
        // Not a reference — copy one character and carry on.
        let ch = template[i..].chars().next().expect("in bounds");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

/// Where the reference starting at `i` ends: the index of its closing `}`, or
/// `None` where `i` starts no reference (not `{?`, no name character before a
/// `}`, or no `}`).
fn reference_end(bytes: &[u8], i: usize) -> Option<usize> {
    if bytes.get(i) != Some(&b'{') || bytes.get(i + 1) != Some(&b'?') {
        return None;
    }
    let start = i + 2;
    let mut j = start;
    while j < bytes.len() && bytes[j] != b'}' && crate::text::is_name_char(char::from(bytes[j])) {
        j += 1;
    }
    (j > start && bytes.get(j) == Some(&b'}')).then_some(j)
}

/// The variables of `vars` that `template` writes a reference to and no
/// reference can name: a name holding `}`, which a reference reads up to, so
/// `{?a}b}` is the reference `{?a}` and the text `b}` (Q-M1f.11). The loader
/// refuses each one, so a template never prints a sentence its author did not
/// write. `vars` are names without their `?`.
pub fn unreachable_references<'a>(
    template: &str,
    vars: impl IntoIterator<Item = &'a str>,
) -> Vec<&'a str> {
    let mut out: Vec<&str> = vars
        .into_iter()
        .filter(|v| v.contains('}') && template.contains(&format!("{{?{v}}}")))
        .collect();
    out.sort_unstable();
    out.dedup();
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

    /// **A reference names every variable the name rule spells** (W37),
    /// not only `{?` one letter or digit, then letters, digits, `_` and `-`:
    /// each name below is substituted.
    #[test]
    fn a_reference_reads_the_name_rules_characters_up_to_the_first_brace() {
        for name in [
            "isR*", "n*", "_x_", "a.b", "x'", "$x", "%", "a^b", "a/b", "<=", "!x", "@x", "~",
            "a+b", "a&b", "a=b", "`q`", "[a]", "{a", "1st", "-7", "07",
        ] {
            let template = format!("<{{?{name}}}>");
            assert_eq!(
                render_why(&template, &b(&[(name, "V")])),
                "<V>",
                "{template}"
            );
        }
        // Up to the first `}`: `{?a}b}` is the reference `{?a}` and `b}`.
        assert_eq!(render_why("{?a}b}", &b(&[("a", "V")])), "Vb}");
        // A character no name has ends no reference: the text stays.
        for kept in [
            "{?a b}",
            "{?a?b}",
            "{?a:b}",
            "{?a,b}",
            "{?a#b}",
            "{?a|b}",
            "{?Жук}",
            "{?a",
        ] {
            assert_eq!(
                render_why(kept, &b(&[("a b", "V"), ("a", "V"), ("Жук", "V")])),
                kept
            );
        }
    }

    #[test]
    fn a_reference_to_a_name_holding_a_brace_is_unreachable() {
        assert_eq!(
            unreachable_references("{?a}b} and {?c}", ["a}b", "c", "d}"]),
            ["a}b"]
        );
        assert!(unreachable_references("{?a} b}", ["a}b"]).is_empty());
        assert!(unreachable_references("{?isR*}", ["isR*"]).is_empty());
    }
}
