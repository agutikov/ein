//! **The answer**: what `einaf from-text` prints of a program the kernel
//! judged, read off the feedback objects of the questions asked of it, and
//! the answer query's values formatted ([`crate::format`]).
//!
//! **The text is chosen by the answer query's classes, not by the kernel's
//! word alone** ([the answer's page](../../docs/einaf/08-from-text.md#3-the-answer)):
//! where R has a state and the answer query one class with a row over a
//! complete set, each row through its `:goal-text`, its values formatted, a
//! line each, which is the program's own sentence; where it has two classes
//! or more, or one with no row, and where the outcome gives no state to read,
//! one of EinAf's fixed texts ([`Case::fixed`]), chosen by the kernel's
//! values and never written by the model. Two of them are answers, the same
//! strings every arm may give; the others are lines of the command, scored
//! as no answer.
//!
//! **The answer query** is the program's one query with a `:goal-text`; with
//! two, the first is read (the contract's check reports the second).
//!
//! **A stand-in for the answer's comparison** ([`score`]): whether the
//! answer states what a gold short answer states, the values it binds, by a
//! minimal reader; [S2.5.3]'s is the one every arm will be scored by.

use serde::{Deserialize, Serialize};

use crate::format::{Formatters, Names};
use crate::kernel::{Feedback, Outcome, Part, QueryPart, Question, Vocabulary};

/// **The statements leave the answer open**: an answer, the same string
/// every arm may give and the gold short answer of an *ambiguous* instance.
pub const UNDETERMINED: &str = "The statements do not determine the answer.";
/// **The statements have no model**: an answer, and the gold short answer
/// of an *unsatisfiable* instance.
pub const CONTRADICT: &str = "The statements contradict each other.";
/// R is empty and every alive node owes: no answer.
pub const INCOMPLETE: &str = "No answer: the program is incomplete.";
/// A bound stopped the run before its question was decided: no answer.
pub const NOT_DECIDED: &str = "No answer: not decided.";
/// The program is not monotone, and nothing is claimed of its answer.
pub const FLAGGED: &str = "No answer is claimed: the program is not monotone.";
/// The kernel refused the program, or no program was written.
pub const REFUSED: &str = "No answer: the program was refused.";
/// The program states no query with a `:goal-text`.
pub const NO_QUERY: &str = "No answer: the program states no answer query.";

/// **Which row of the answer's table a run is in.**
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Case {
    /// A state in R, the answer query one class with a row, over a complete
    /// set: its rows, the program's sentence.
    Lines,
    /// A state in R, and two classes or more.
    Several,
    /// A state in R, and one class with no row.
    NoRow,
    Inconsistent,
    Incomplete,
    NotDecided,
    Flagged,
    Invalid,
    /// R has a state, and the program states no answer query.
    NoQuery,
}

impl Case {
    /// The fixed text, where the case has one: every case but [`Case::Lines`].
    pub fn fixed(self) -> Option<&'static str> {
        match self {
            Case::Lines => None,
            Case::Several | Case::NoRow => Some(UNDETERMINED),
            Case::Inconsistent => Some(CONTRADICT),
            Case::Incomplete => Some(INCOMPLETE),
            Case::NotDecided => Some(NOT_DECIDED),
            Case::Flagged => Some(FLAGGED),
            Case::Invalid => Some(REFUSED),
            Case::NoQuery => Some(NO_QUERY),
        }
    }

    /// Whether what is printed is an answer: the program's lines, or one of
    /// the two fixed texts that are answers.
    pub fn is_answer(self) -> bool {
        matches!(
            self,
            Case::Lines | Case::Several | Case::NoRow | Case::Inconsistent
        )
    }

    pub fn word(self) -> &'static str {
        match self {
            Case::Lines => "lines",
            Case::Several => "several",
            Case::NoRow => "no-row",
            Case::Inconsistent => "inconsistent",
            Case::Incomplete => "incomplete",
            Case::NotDecided => "not-decided",
            Case::Flagged => "flagged",
            Case::Invalid => "invalid",
            Case::NoQuery => "no-query",
        }
    }
}

/// **One row of the answer query**, through its `:goal-text`: the kernel's
/// line, and the line with its values formatted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Line {
    /// The kernel's line, as it filled the template.
    pub kernel: String,
    /// The template filled again with the formatted values.
    pub formatted: String,
    /// Each value of the row, as written and as printed, with the formatter
    /// that wrote it, `null` where it prints as written.
    pub values: Vec<ValueOf>,
}

/// A value of a row, by its variable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueOf {
    pub variable: String,
    pub written: String,
    pub printed: String,
    pub by: Option<String>,
}

/// **A class of the answer query**, printed as an alternative: the states
/// that give it and its lines.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alternative {
    pub states: Vec<usize>,
    pub lines: Vec<Line>,
}

/// **The answer's line of a run**: what is printed, why, and from what.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answer {
    pub case: Case,
    /// The question whose object chose the text, as `ein solve`'s flags.
    pub question: Option<String>,
    /// The outcome over all of R it was chosen by.
    pub outcome: Option<Outcome>,
    /// The answer query's `:goal-text`, where there is one.
    pub goal_text: Option<String>,
    /// [`Case::Lines`]: each row, the kernel's line and the formatted one.
    pub lines: Vec<Line>,
    /// The fixed text, every other case.
    pub fixed: Option<String>,
    /// Whose words the answer is: `program` (the `:goal-text`), or `einaf`
    /// (a fixed text).
    pub from: String,
    /// [`Case::Several`]: each class's lines.
    pub alternatives: Vec<Alternative>,
    /// The alternatives are every class of the part the query names, not
    /// the ones the states listed so far give.
    pub alternatives_complete: bool,
    /// What stands beneath a fixed text: the kernel's sentences of what is
    /// owed, what stopped the run, its warning and what it printed, its
    /// diagnostics.
    pub beneath: Vec<String>,
    /// How many values of the printed lines still print as a term.
    pub terms: usize,
    /// The formatters it was written with, in order.
    pub formatters: Vec<String>,
}

impl Answer {
    /// **The answer that is scored**: the formatted lines, a line each, or
    /// the fixed text.
    pub fn text(&self) -> String {
        match &self.fixed {
            Some(f) => f.clone(),
            None => self
                .lines
                .iter()
                .map(|l| l.formatted.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }

    /// The kernel's lines, unformatted, a line each.
    pub fn kernel_text(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.kernel.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// **What `einaf from-text` prints**: the answer, then, indented, what
    /// stands beneath it and the alternatives.
    pub fn print(&self) -> String {
        let mut s = self.text();
        s.push('\n');
        for b in &self.beneath {
            s.push_str(&format!("  {b}\n"));
        }
        if !self.alternatives.is_empty() {
            s.push_str(if self.alternatives_complete {
                "  The answer query's classes, each an alternative:\n"
            } else {
                "  The answer query's classes in the states found, each an alternative (R not listed whole):\n"
            });
            for (i, a) in self.alternatives.iter().enumerate() {
                let lines: Vec<&str> = a.lines.iter().map(|l| l.formatted.as_str()).collect();
                s.push_str(&format!(
                    "    {}. {}\n",
                    i + 1,
                    if lines.is_empty() {
                        "(no row)".to_string()
                    } else {
                        lines.join(" / ")
                    }
                ));
            }
        }
        s
    }
}

/// **How the program's printed names resolve**: its imports, read off its
/// vocabulary.
pub fn names(vocab: &Vocabulary) -> Names {
    let mut n = Names::new();
    for imp in &vocab.imports {
        n.import(
            &imp.module,
            imp.symbols.as_deref().unwrap_or(&[]),
            imp.alias.as_deref(),
        );
    }
    n
}

/// The answer query of an object: its first query with a `:goal-text`.
pub fn answer_query(f: &Feedback) -> Option<&QueryPart> {
    f.queries.as_ref()?.iter().find(|q| q.goal_text.is_some())
}

/// **Whether a description is needed**: `--single`'s object found R with a
/// state, and the answer query's one class with a row is over the states
/// one question listed and not over a complete set, so that it is a lower
/// bound: the description says whether R gives that class alone.
pub fn needs_description(single: &Feedback) -> bool {
    let answered = matches!(
        single.outcome_over(Part::All).map(|r| r.outcome),
        Some(Outcome::One | Outcome::Several)
    );
    answered
        && answer_query(single).is_some_and(|q| {
            !q.exact && q.classes.len() == 1 && q.classes[0].rows.iter().any(|_| true)
        })
}

/// A class's rows through the template, each formatted.
fn lines_of(
    template: &str,
    class: &crate::kernel::ClassPart,
    names: &Names,
    formatters: &Formatters,
) -> Vec<Line> {
    class
        .rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let pairs: Vec<(String, String)> =
                row.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            let (formatted, values) = formatters.fill(template, &pairs, names);
            Line {
                kernel: class.lines.get(i).cloned().unwrap_or_default(),
                formatted,
                values: pairs
                    .iter()
                    .zip(values)
                    .map(|((k, _), v)| ValueOf {
                        variable: k.clone(),
                        written: v.written,
                        printed: v.text,
                        by: v.by,
                    })
                    .collect(),
            }
        })
        .collect()
}

/// A diagnostic as a line beneath a fixed text: its kind, its place, its
/// message.
fn said(f: &crate::kernel::Finding) -> String {
    match &f.place {
        Some(p) if !f.message.contains(&format!("{}:{}", p.file, p.line)) => {
            format!(
                "{} at {}:{}:{}: {}",
                f.kind, p.file, p.line, p.col, f.message
            )
        }
        _ => format!("{}: {}", f.kind, f.message),
    }
}

/// **Read the answer** off the objects of the questions asked of the
/// program, in the order asked: the last that is not `--no-search` chooses
/// the text, and `--no-search`'s gives what root owes.
pub fn read(objects: &[Feedback], names: &Names, formatters: &Formatters) -> Answer {
    let deciding = objects
        .iter()
        .rev()
        .find(|f| f.asked.ask != "no-search")
        .or_else(|| objects.last());
    let root = objects.iter().find(|f| f.asked.ask == "no-search");
    let mut a = Answer {
        case: Case::Invalid,
        question: deciding.map(|f| f.asked.question.clone()),
        outcome: None,
        goal_text: None,
        lines: Vec::new(),
        fixed: None,
        from: "einaf".to_string(),
        alternatives: Vec::new(),
        alternatives_complete: false,
        beneath: Vec::new(),
        terms: 0,
        formatters: formatters.kinds(),
    };
    let Some(f) = deciding else {
        a.fixed = Some(REFUSED.to_string());
        a.beneath.push("no program was written".to_string());
        return a;
    };
    let outcome = f.outcome_over(Part::All).map(|r| r.outcome);
    a.outcome = outcome;
    let query = answer_query(f);
    a.goal_text = query.and_then(|q| q.goal_text.clone());
    a.case = match outcome {
        None => {
            if let Some(fault) = &f.fault {
                a.beneath.push(format!(
                    "the kernel failed ({}): {}",
                    fault.kind, fault.message
                ));
            }
            Case::Invalid
        }
        Some(Outcome::Invalid) => {
            a.beneath.extend(
                f.diagnostics
                    .iter()
                    .filter(|d| d.severity == "error")
                    .map(said),
            );
            Case::Invalid
        }
        Some(Outcome::Flagged) => {
            a.beneath.extend(
                f.diagnostics
                    .iter()
                    .filter(|d| d.severity == "warning")
                    .map(|d| d.message.clone()),
            );
            if let (Some(q), Some(t)) = (query, &a.goal_text) {
                for c in &q.classes {
                    for l in lines_of(t, c, names, formatters) {
                        a.beneath.push(format!("the kernel printed: {}", l.kernel));
                    }
                }
            }
            Case::Flagged
        }
        Some(Outcome::NotDecided) => {
            if let Some(stop) = f.readings.as_ref().and_then(|r| r.stopped.as_ref()) {
                a.beneath.push(match &stop.saturation {
                    Some(s) => format!("stopped by the bound {} in a saturation of {s}", stop.why),
                    None => format!("stopped by the bound {}", stop.why),
                });
            }
            a.beneath.extend(
                f.diagnostics
                    .iter()
                    .filter(|d| d.severity != "note")
                    .map(said),
            );
            Case::NotDecided
        }
        Some(Outcome::Inconsistent) => Case::Inconsistent,
        Some(Outcome::Incomplete) => {
            let owed = root
                .and_then(|r| r.owed.as_ref())
                .and_then(|o| o.root.clone())
                .or_else(|| f.owed.as_ref().and_then(|o| o.root.clone()))
                .unwrap_or_default();
            a.beneath.extend(owed);
            Case::Incomplete
        }
        Some(Outcome::One | Outcome::Several) => match (query, &a.goal_text) {
            (Some(q), Some(t)) => {
                let with_rows: Vec<&crate::kernel::ClassPart> =
                    q.classes.iter().filter(|c| !c.rows.is_empty()).collect();
                if q.classes.len() >= 2 {
                    a.alternatives = q
                        .classes
                        .iter()
                        .map(|c| Alternative {
                            states: c.states.clone(),
                            lines: lines_of(t, c, names, formatters),
                        })
                        .collect();
                    a.alternatives_complete = q.exact;
                    Case::Several
                } else if with_rows.len() == 1 && q.exact {
                    a.lines = lines_of(t, with_rows[0], names, formatters);
                    a.from = "program".to_string();
                    Case::Lines
                } else if with_rows.len() == 1 {
                    // A lower bound no description replaced: the run stopped
                    // before R was listed whole.
                    a.beneath.push(
                        "the answer query's classes are over the states found, not all of R"
                            .to_string(),
                    );
                    Case::NotDecided
                } else {
                    Case::NoRow
                }
            }
            _ => Case::NoQuery,
        },
    };
    a.terms = a
        .lines
        .iter()
        .flat_map(|l| &l.values)
        .filter(|v| v.by.is_none() && v.printed.starts_with('('))
        .count();
    a.fixed = a.case.fixed().map(str::to_string);
    a
}

/// **The questions the answer needs, after `--single`**: a description of
/// R where [`needs_description`] says the classes are a lower bound.
pub fn more_questions(single: &Feedback) -> Vec<Question> {
    if needs_description(single) {
        vec![Question::Describe]
    } else {
        Vec::new()
    }
}

/// **What a reader of the answer finds against a gold short answer.**
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scored {
    /// The gold short answer, as given.
    pub gold: String,
    /// `right`, `wrong`, or `no-answer` where what was printed is no answer.
    pub verdict: String,
    pub why: String,
}

/// **The stand-in reader of the answer's comparison**: does the answer state
/// the values the gold answer binds? A gold answer is `@undetermined` or
/// `@contradiction`, the fixed text of that name; or claims separated by
/// `|`, each a few words that one line of the answer must hold together,
/// case aside, a word a run of letters and digits. Where the gold states a
/// number, no line may state a number it does not. A fixed text that is an
/// answer is wrong against a gold of values, and lines against a fixed
/// text's gold.
pub fn score(answer: &Answer, gold: &str) -> Scored {
    let out = |verdict: &str, why: String| Scored {
        gold: gold.to_string(),
        verdict: verdict.to_string(),
        why,
    };
    if !answer.case.is_answer() {
        return out("no-answer", format!("{} is no answer", answer.case.word()));
    }
    let fixed = match gold.trim() {
        "@undetermined" => Some(UNDETERMINED),
        "@contradiction" => Some(CONTRADICT),
        _ => None,
    };
    if let Some(text) = fixed {
        return if answer.fixed.as_deref() == Some(text) {
            out("right", format!("the fixed text \"{text}\""))
        } else {
            out("wrong", format!("not the fixed text \"{text}\""))
        };
    }
    if let Some(f) = &answer.fixed {
        return out("wrong", format!("the fixed text \"{f}\" against values"));
    }
    let words = |s: &str| -> Vec<String> {
        s.split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(str::to_ascii_lowercase)
            .collect()
    };
    let lines: Vec<Vec<String>> = answer.lines.iter().map(|l| words(&l.formatted)).collect();
    let claims: Vec<Vec<String>> = gold
        .split('|')
        .map(words)
        .filter(|c| !c.is_empty())
        .collect();
    for c in &claims {
        if !lines.iter().any(|l| c.iter().all(|w| l.contains(w))) {
            return out("wrong", format!("no line states {}", c.join(" ")));
        }
    }
    let is_number = |w: &String| w.bytes().all(|b| b.is_ascii_digit());
    let gold_numbers: Vec<&String> = claims.iter().flatten().filter(|w| is_number(w)).collect();
    if !gold_numbers.is_empty() {
        for l in &lines {
            if let Some(n) = l.iter().find(|w| is_number(w) && !gold_numbers.contains(w)) {
                return out(
                    "wrong",
                    format!("a line states {n}, which the gold does not"),
                );
            }
        }
    }
    out("right", "every claim stated, no other number".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(texts: &[&str]) -> Answer {
        Answer {
            case: Case::Lines,
            question: None,
            outcome: Some(Outcome::One),
            goal_text: None,
            lines: texts
                .iter()
                .map(|t| Line {
                    kernel: t.to_string(),
                    formatted: t.to_string(),
                    values: Vec::new(),
                })
                .collect(),
            fixed: None,
            from: "program".to_string(),
            alternatives: Vec::new(),
            alternatives_complete: false,
            beneath: Vec::new(),
            terms: 0,
            formatters: Vec::new(),
        }
    }

    #[test]
    fn the_reader_reads_the_values_a_gold_answer_binds() {
        let v = |a: &Answer, g: &str| score(a, g).verdict;
        assert_eq!(v(&lines(&["Bob has 3 apples."]), "3"), "right");
        assert_eq!(
            v(&lines(&["Bob has 1 apples.", "Bob has 3 apples."]), "3"),
            "wrong"
        );
        assert_eq!(
            v(
                &lines(&["The Norwegian drinks water. The Japanese owns the zebra."]),
                "Norwegian water|Japanese zebra"
            ),
            "right"
        );
        assert_eq!(
            v(
                &lines(&[
                    "The Norwegian drinks water.",
                    "The Japanese owns the zebra."
                ]),
                "Norwegian water|Japanese zebra"
            ),
            "right"
        );
        assert_eq!(
            v(&lines(&["The Spaniard owns the zebra."]), "Japanese zebra"),
            "wrong"
        );
        assert_eq!(
            v(&lines(&["how many apples Bob has at t1: 3"]), "3"),
            "right"
        );
        let mut fixed = lines(&[]);
        fixed.case = Case::Several;
        fixed.fixed = Some(UNDETERMINED.to_string());
        assert_eq!(v(&fixed, "@undetermined"), "right");
        assert_eq!(v(&fixed, "3"), "wrong");
        assert_eq!(v(&lines(&["3"]), "@undetermined"), "wrong");
        fixed.case = Case::NotDecided;
        fixed.fixed = Some(NOT_DECIDED.to_string());
        assert_eq!(v(&fixed, "3"), "no-answer");
    }

    #[test]
    fn the_fixed_texts_are_two_answers_and_five_lines() {
        let all = [
            Case::Lines,
            Case::Several,
            Case::NoRow,
            Case::Inconsistent,
            Case::Incomplete,
            Case::NotDecided,
            Case::Flagged,
            Case::Invalid,
            Case::NoQuery,
        ];
        let answers: Vec<&str> = all
            .iter()
            .filter(|c| c.is_answer())
            .filter_map(|c| c.fixed())
            .collect();
        assert_eq!(answers, [UNDETERMINED, UNDETERMINED, CONTRADICT]);
        for c in all {
            assert_eq!(c.fixed().is_none(), c == Case::Lines, "{c:?}");
        }
    }
}
