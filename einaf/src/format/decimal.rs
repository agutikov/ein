//! **The decimal numeral**, `std.math.decimal`'s: zero, `std.math.arith.z`,
//! or `(std.math.decimal.n PREFIX DIGIT)`, ten times the prefix plus the
//! digit, the digit one of the names `0` to `9` (Q-M1f.361). Written as its
//! digits, by value: high zeros are dropped, and zero is `0`. A term headed
//! like a numeral that is not one is no numeral, and this formatter does not
//! take it: it prints as written.

use super::{Format, Formatters, Names, Term};

/// The constructor's full name.
pub const N: &str = "std.math.decimal.n";
/// Zero's full name.
pub const Z: &str = "std.math.arith.z";

/// The formatter.
pub struct Decimal;

/// **A numeral's digits**, high zeros included, or `None` where the term is
/// no numeral.
pub fn digits(term: &Term, names: &Names) -> Option<String> {
    match term {
        Term::Name(z) if names.full(z) == Z => Some(String::new()),
        Term::Fact(parts) if parts.len() == 3 => {
            let Term::Name(head) = &parts[0] else {
                return None;
            };
            if names.full(head) != N {
                return None;
            }
            let Term::Name(d) = &parts[2] else {
                return None;
            };
            let digit = (d.len() == 1 && d.as_bytes()[0].is_ascii_digit()).then_some(d)?;
            Some(digits(&parts[1], names)? + digit)
        }
        _ => None,
    }
}

impl Format for Decimal {
    fn kind(&self) -> &str {
        "decimal"
    }

    fn format(&self, term: &Term, names: &Names, _: &Formatters) -> Option<String> {
        let d = digits(term, names)?;
        let value = d.trim_start_matches('0');
        Some(if value.is_empty() {
            "0".to_string()
        } else {
            value.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Formatters, Names, read};
    use super::*;

    fn names(form: &str) -> Names {
        let mut n = Names::new();
        match form {
            "selective" => {
                n.import("std.math.decimal", &["n".to_string()], None);
                n.import("std.math.arith", &["z".to_string()], None);
            }
            "alias" => {
                n.import("std.math.decimal", &[], Some("d"));
                n.import("std.math.arith", &[], Some("ar"));
            }
            _ => {}
        }
        n
    }

    fn f(text: &str, form: &str) -> Option<String> {
        Decimal.format(&read(text).unwrap(), &names(form), &Formatters::none())
    }

    #[test]
    fn a_numeral_is_its_digits_under_every_form_of_the_import() {
        assert_eq!(f("(n z 3)", "selective").as_deref(), Some("3"));
        assert_eq!(f("(d.n ar.z 3)", "alias").as_deref(), Some("3"));
        assert_eq!(
            f("(std.math.decimal.n std.math.arith.z 3)", "whole").as_deref(),
            Some("3")
        );
        assert_eq!(
            f("(n (n (n (n z 1) 7) 2) 8)", "selective").as_deref(),
            Some("1728")
        );
        assert_eq!(f("z", "selective").as_deref(), Some("0"));
        assert_eq!(f("(n (n z 0) 7)", "selective").as_deref(), Some("7"));
        assert_eq!(f("(n z 0)", "selective").as_deref(), Some("0"));
    }

    #[test]
    fn a_term_headed_like_a_numeral_that_is_not_one_is_not_taken() {
        // The program's own `n`, where no import lists it.
        assert_eq!(f("(n z 3)", "whole"), None);
        // A digit that is no digit, a prefix that is no numeral, a count off.
        assert_eq!(f("(n z 12)", "selective"), None);
        assert_eq!(f("(n Bob 1)", "selective"), None);
        assert_eq!(f("(n z 1 2)", "selective"), None);
        assert_eq!(f("Bob", "selective"), None);
    }
}
