//! **A fact that stands as a value**, written by cases that are data: a
//! relation's full name, a count of arguments, and the words a fact of it is
//! written as, `{1}` its first argument, each argument formatted in its
//! turn. One file a theory, beside this module under `cases/`; the first is
//! the word theory's quantities ([`WORDS`]). A fact with no case is not
//! taken, and prints as written: no words are invented for a relation the
//! cases do not name, which is Ein's own rule for a fact.

use super::{Format, Formatters, Names, Term};

/// The word theory's quantities.
pub const WORDS: &str = include_str!("cases/words.txt");

/// **One case**: the relation's full name, its count, its words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    pub relation: String,
    pub count: usize,
    pub words: String,
}

/// **A theory's cases**, read off its file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cases {
    pub theory: String,
    pub cases: Vec<Case>,
}

impl Cases {
    /// **Read a file of cases**: a case a line, `RELATION COUNT WORDS`; a
    /// line that begins with `#` and a blank line are none. A line that is
    /// no case is an error naming it.
    pub fn parse(theory: &str, text: &str) -> Result<Cases, String> {
        let mut cases = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let mut it = l.splitn(2, char::is_whitespace);
            let relation = it.next().unwrap_or_default().to_string();
            let rest = it.next().unwrap_or_default().trim_start();
            let mut it = rest.splitn(2, char::is_whitespace);
            let count: usize = it
                .next()
                .and_then(|c| c.parse().ok())
                .ok_or_else(|| format!("{theory}, line {}: no count of arguments", i + 1))?;
            let words = it.next().unwrap_or_default().trim().to_string();
            if relation.is_empty() || words.is_empty() {
                return Err(format!("{theory}, line {}: no relation or no words", i + 1));
            }
            for k in references(&words) {
                if k == 0 || k > count {
                    return Err(format!(
                        "{theory}, line {}: {{{k}}} is no argument of {relation}, of {count}",
                        i + 1
                    ));
                }
            }
            cases.push(Case {
                relation,
                count,
                words,
            });
        }
        Ok(Cases {
            theory: theory.to_string(),
            cases,
        })
    }

    /// The word theory's cases.
    pub fn words() -> Cases {
        Cases::parse("words", WORDS).expect("the word theory's cases read")
    }

    /// The case of a relation at a count.
    pub fn case(&self, relation: &str, count: usize) -> Option<&Case> {
        self.cases
            .iter()
            .find(|c| c.relation == relation && c.count == count)
    }
}

/// The argument numbers a case's words name, `{k}`.
fn references(words: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut rest = words;
    while let Some(i) = rest.find('{') {
        rest = &rest[i + 1..];
        if let Some(j) = rest.find('}')
            && let Ok(k) = rest[..j].parse::<usize>()
        {
            out.push(k);
        }
    }
    out
}

impl Format for Cases {
    fn kind(&self) -> &str {
        &self.theory
    }

    fn format(&self, term: &Term, names: &Names, all: &Formatters) -> Option<String> {
        let head = term.head()?;
        let args = term.args();
        let case = self.case(&names.full(head), args.len())?;
        // One pass over the words, so that an argument's own text is never
        // read as a reference.
        let mut out = String::new();
        let mut rest = case.words.as_str();
        while let Some(i) = rest.find('{') {
            out.push_str(&rest[..i]);
            let after = &rest[i + 1..];
            match after.find('}').and_then(|j| {
                let k: usize = after[..j].parse().ok()?;
                Some((j, args.get(k.checked_sub(1)?)?))
            }) {
                Some((j, a)) => {
                    out.push_str(&all.term_or_written(a, names));
                    rest = &after[j + 1..];
                }
                None => {
                    out.push('{');
                    rest = after;
                }
            }
        }
        out.push_str(rest);
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Formatters, Names};
    use super::*;

    fn names() -> Names {
        let mut n = Names::new();
        n.import(
            "words",
            &["amount".to_string(), "together".to_string()],
            None,
        );
        n.import("std.math.decimal", &["n".to_string()], None);
        n.import("std.math.arith", &["z".to_string()], None);
        n
    }

    #[test]
    fn the_word_theorys_cases_read_and_name_its_quantities() {
        let c = Cases::words();
        for r in [
            "amount",
            "together",
            "apart",
            "capacity",
            "needed",
            "over",
            "number-of",
            "each-of",
            "rest-of",
        ] {
            assert!(
                c.cases.iter().any(|k| k.relation == format!("words.{r}")),
                "{r}"
            );
        }
        assert!(Cases::parse("t", "p 1 {2}").is_err());
        assert!(Cases::parse("t", "p x words").is_err());
    }

    #[test]
    fn a_quantity_is_written_in_its_cases_words_its_arguments_formatted() {
        let all = Formatters::standard();
        let v = |t: &str| all.value(t, &names()).text;
        assert_eq!(v("(amount Bob apples t1)"), "how many apples Bob has at t1");
        assert_eq!(
            v("(words.amount Alice apples now)"),
            "how many apples Alice has at now"
        );
        assert_eq!(
            v("(words.needed boxes apples (together Bob Alice apples now))"),
            "how many boxes are needed for [how many apples Bob and Alice have together at now]"
        );
        // A fact with no case, and a count no case has, print as written.
        assert_eq!(v("(likes Bob Stew)"), "(likes Bob Stew)");
        assert_eq!(v("(amount Bob apples)"), "(amount Bob apples)");
        // The relation named by its full name alone: a program's own
        // `amount`, where no import lists it, is not the theory's.
        let none = Names::new();
        assert_eq!(
            all.value("(amount Bob apples t1)", &none).text,
            "(amount Bob apples t1)"
        );
    }
}
