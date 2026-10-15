//! The wrong-answer register — `corpus/wrong_answers.toml`.
//!
//! One row per **witness** of every known wrong answer, with the answer the
//! engine gives today and the one it should give, as data. M1e2
//! [S1e2.1.1](../../../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.1_the_case_register.md)
//! wrote it because the cases lived in five registers that did not know about
//! each other — question entries, fixture headers, `:expect` clauses stating
//! today's answer on purpose, probe scripts and a milestone table — and a
//! renderer needs one list while a fix stage needs one place to say *done*.
//!
//! The format, the field reference and the rule for a witness that also
//! carries an `:expect` are the file's own header. This module reads it and
//! holds the invariants that need no engine run; what each row *answers* is
//! `ein-cli/tests/wrong_answers.rs`'s, through the binary, and the id-order
//! half of a row is `ein-render/tests/id_order_invariance.rs`'s.
//!
//! **Not `:expect`.** An expectation is a claim about the program's answer, so
//! one stating the *right* answer of an open case would fail `ein test` with no
//! pointer to its owner, and one stating the wrong answer — as
//! `branching/15`+`16` do — pins the defect and says nothing about what the
//! answer should be. The register says both, and it can say what `:expect`
//! cannot: a model with an empty goal extent (Q-M1e.13), a core, a lever
//! read through an environment variable.

use serde::Deserialize;
use std::path::Path;

/// The register's path, repo-root-relative — beside `corpus.toml` because it
/// is a corpus reader's table, and not under `plans/` because a plan is deleted
/// when its milestone closes and a register guarding fixed cases has to
/// outlive it.
pub const REGISTER: &str = "corpus/wrong_answers.toml";

pub const REGISTER_SCHEMA: &str = "ein-wrong-answers/1";

/// What a row may say about itself.
pub const STATUSES: [&str; 3] = ["open", "fixed", "not-wrong"];

/// The verdict words, as `--json-summary`'s `verdict.type` spells them.
pub const VERDICTS: [&str; 4] = ["Solution", "Ambiguity", "Contradiction", "Open"];

/// What the user's review (M1e2 S1e2.1.7) may rule on a row.
pub const REVIEW_VERDICTS: [&str; 5] = [
    "confirmed",
    "right-answer-corrected",
    "not-wrong",
    "split",
    "merge",
];

/// What a `look_at` pointer may name — the pages of a review-bundle run
/// directory. A pointer is one of these words, optionally followed by an
/// argument: `layer 2`, `entering p(A)`, `model 3`, `events writeback`,
/// `trace step 12`.
pub const LOOK_AT: [&str; 11] = [
    "root",
    "root-writes",
    "timeline",
    "layer",
    "entering",
    "refuted",
    "trace",
    "answer",
    "model",
    "summary",
    "events",
];

/// The one directory outside the corpus a witness may live in: minimised inputs
/// that violate a property the corpus sweeps assert, and so cannot be corpus
/// entries until they are fixed (`corpus/fuzz_findings/README.md`).
pub const OUTSIDE_THE_CORPUS: &str = "corpus/fuzz_findings/";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Register {
    pub schema: String,
    #[serde(default)]
    pub case: Vec<Row>,
}

/// One witness of one case.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    /// `<case>/<witness>` — stable, and the review bundle's directory name.
    pub id: String,
    /// The inventory's id: `W1`…`W12`, `C1`.
    pub case: String,
    pub title: String,
    pub class: String,
    /// `default`, or the lever the wrong answer needs.
    pub settings: String,
    /// The P1e2.2 stage that fixes it.
    pub owner: String,
    /// The ids that carried the case before the register.
    #[serde(default)]
    pub question: Vec<String>,
    pub witness: String,
    /// Required for a witness under [`OUTSIDE_THE_CORPUS`]: why it cannot be a
    /// corpus entry.
    #[serde(default)]
    pub not_in_corpus: Option<String>,
    /// `open`, `fixed`, `not-wrong`, or `merged:<id>`.
    pub status: String,
    #[serde(default)]
    pub note: Option<String>,
    /// Where in a run the case goes wrong — the review bundle's links into
    /// the run directories (M1e2 S1e2.1.6). Written once, here, so the fixing
    /// stage reads the same pointers the reviewer followed. The vocabulary is
    /// [`LOOK_AT`].
    #[serde(default)]
    pub look_at: Vec<String>,
    /// What the engine answers today. For a `fixed` row, what it *used* to
    /// answer.
    pub wrong: Answer,
    /// A run that gets it right today.
    #[serde(default)]
    pub control: Option<Answer>,
    /// Further runs the case page puts beside the wrong one.
    #[serde(default)]
    pub twin: Vec<Answer>,
    pub right: Right,
    /// The user's verdict on the row (S1e2.1.7). Absent until it is reviewed.
    #[serde(default)]
    pub review: Option<Review>,
}

/// One review verdict, as the user gave it.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    /// One of [`REVIEW_VERDICTS`].
    pub verdict: String,
    /// `YYYY-MM-DD`.
    pub date: String,
    /// The commit the review bundle the reviewer read was built at.
    pub bundle: String,
    /// The reviewer's sentence, where there was one.
    #[serde(default)]
    pub note: Option<String>,
}

impl Row {
    /// Every run the row records an answer for, labelled the way a failure
    /// message names it: `wrong`, `control`, `twin 1`, …
    pub fn answers(&self) -> Vec<(String, &Answer)> {
        let mut out = vec![("wrong".to_string(), &self.wrong)];
        if let Some(c) = &self.control {
            out.push(("control".to_string(), c));
        }
        for (i, t) in self.twin.iter().enumerate() {
            out.push((format!("twin {}", i + 1), t));
        }
        out
    }

    /// The file an answer was taken on.
    pub fn witness_of<'a>(&'a self, a: &'a Answer) -> &'a str {
        a.witness.as_deref().unwrap_or(&self.witness)
    }
}

/// An answer the engine gives to one run.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    /// Another file — a control or a twin — when not the row's witness.
    #[serde(default)]
    pub witness: Option<String>,
    /// A corpus run name ([`crate::plan::argv`]).
    pub run: String,
    /// `NAME=value`, set for the run.
    #[serde(default)]
    pub env: Vec<String>,
    /// What differs from the wrong run, in one line.
    #[serde(default)]
    pub why: Option<String>,
    #[serde(default)]
    pub exit: i32,
    pub verdict: String,
    pub k: usize,
    pub exhausted: bool,
    #[serde(default)]
    pub core: Option<Vec<String>>,
    #[serde(default)]
    pub closes: Vec<String>,
    #[serde(default)]
    pub models: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub open: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub owes: Option<Vec<u64>>,
    /// In a build with debug assertions the run aborts with this text on
    /// stderr; every other field is the release build's.
    #[serde(default)]
    pub debug_panic: Option<String>,
    #[serde(default)]
    pub id_order: Option<IdOrder>,
}

/// An `id_order_invariance` op and the seed under which its output moves.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdOrder {
    pub op: String,
    pub seed: i64,
}

/// The answer the definitions give. Every field it omits is not claimed.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Right {
    /// The ruling a contested case's answer assumes.
    #[serde(default)]
    pub reading: Option<String>,
    /// From what the answer was derived.
    pub derived: String,
    #[serde(default)]
    pub verdict: Option<String>,
    #[serde(default)]
    pub k: Option<usize>,
    #[serde(default)]
    pub exhausted: Option<bool>,
    #[serde(default)]
    pub core: Option<Vec<String>>,
    #[serde(default)]
    pub closes: Vec<String>,
    #[serde(default)]
    pub models: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub open: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub owes: Option<Vec<u64>>,
    /// A sentence the right read-out would add.
    #[serde(default)]
    pub qualifier: Option<String>,
    /// The answer under each other reading of a contested case.
    #[serde(default)]
    pub alternative: Vec<Right>,
}

impl Register {
    pub fn load(path: &Path) -> Result<Register, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let reg: Register =
            toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if reg.schema != REGISTER_SCHEMA {
            return Err(format!(
                "{}: schema {:?}, expected {REGISTER_SCHEMA:?}",
                path.display(),
                reg.schema
            ));
        }
        Ok(reg)
    }

    /// Every way a row can be malformed without running anything, one message
    /// per defect, each naming the row.
    pub fn defects(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut seen = std::collections::BTreeMap::<&str, &Row>::new();
        for r in &self.case {
            let at = |m: String| format!("{}: {m}", r.id);
            if !r.id.starts_with(&format!("{}/", r.case)) {
                out.push(at(format!("id does not start with its case {:?}", r.case)));
            }
            if let Some(prev) = seen.insert(r.id.as_str(), r) {
                out.push(at(format!("id repeated (also {:?})", prev.witness)));
            }
            let merged = r.status.strip_prefix("merged:");
            if !STATUSES.contains(&r.status.as_str()) && merged.is_none_or(str::is_empty) {
                out.push(at(format!(
                    "status {:?} is not one of {STATUSES:?} or merged:<id>",
                    r.status
                )));
            }
            if r.witness.starts_with(OUTSIDE_THE_CORPUS) != r.not_in_corpus.is_some() {
                out.push(at(format!(
                    "`not_in_corpus` is required exactly for a witness under {OUTSIDE_THE_CORPUS}"
                )));
            }
            for p in &r.look_at {
                let word = p.split_whitespace().next().unwrap_or("");
                if !LOOK_AT.contains(&word) {
                    out.push(at(format!("look_at {p:?} is not one of {LOOK_AT:?}")));
                }
            }
            if r.status == "fixed" && (r.right.verdict.is_none() || r.right.k.is_none()) {
                out.push(at("a fixed row states the right verdict and k".to_string()));
            }
            if let Some(rv) = &r.review {
                if !REVIEW_VERDICTS.contains(&rv.verdict.as_str()) {
                    out.push(at(format!(
                        "review verdict {:?} is not one of {REVIEW_VERDICTS:?}",
                        rv.verdict
                    )));
                }
                let d = rv.date.as_bytes();
                if d.len() != 10
                    || d[4] != b'-'
                    || d[7] != b'-'
                    || !d
                        .iter()
                        .enumerate()
                        .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
                {
                    out.push(at(format!("review date {:?} is not YYYY-MM-DD", rv.date)));
                }
                if rv.bundle.len() < 7 || !rv.bundle.bytes().all(|c| c.is_ascii_hexdigit()) {
                    out.push(at(format!("review bundle {:?} is not a commit", rv.bundle)));
                }
            }
            if (r.status == "not-wrong")
                != r.review.as_ref().is_some_and(|v| v.verdict == "not-wrong")
            {
                out.push(at(
                    "status `not-wrong` and review verdict `not-wrong` go together".to_string(),
                ));
            }
            for (label, a) in r.answers() {
                let at = |m: String| format!("{} [{label}]: {m}", r.id);
                if !a.run.starts_with("solve") {
                    out.push(at(format!(
                        "run {:?} is not a solve — the register reads --json-summary",
                        a.run
                    )));
                }
                if !VERDICTS.contains(&a.verdict.as_str()) {
                    out.push(at(format!(
                        "verdict {:?} is not one of {VERDICTS:?}",
                        a.verdict
                    )));
                }
                if a.verdict == "Contradiction" && a.core.is_none() {
                    out.push(at("a Contradiction states its core".to_string()));
                }
                if (a.models.is_some() || a.open.is_some()) && a.closes.is_empty() {
                    out.push(at(
                        "models are compared over `closes`, which is empty".to_string()
                    ));
                }
                if let Some(m) = &a.models
                    && m.len() != a.k
                {
                    out.push(at(format!("{} models listed for k = {}", m.len(), a.k)));
                }
                if label != "wrong" && a.why.is_none() {
                    out.push(at("a control or twin says what differs (`why`)".to_string()));
                }
                if label != "wrong" && a.id_order.is_some() {
                    out.push(at("`id_order` belongs on the wrong run".to_string()));
                }
            }
            for right in std::iter::once(&r.right).chain(&r.right.alternative) {
                if let Some(v) = &right.verdict
                    && !VERDICTS.contains(&v.as_str())
                {
                    out.push(format!(
                        "{} [right]: verdict {v:?} is not one of {VERDICTS:?}",
                        r.id
                    ));
                }
                if (right.models.is_some() || right.open.is_some()) && right.closes.is_empty() {
                    out.push(format!(
                        "{} [right]: models are compared over `closes`, which is empty",
                        r.id
                    ));
                }
                if let (Some(m), Some(k)) = (&right.models, right.k)
                    && m.len() != k
                {
                    out.push(format!(
                        "{} [right]: {} models listed for k = {k}",
                        r.id,
                        m.len()
                    ));
                }
            }
            if r.right.alternative.iter().any(|a| a.reading.is_none()) {
                out.push(format!(
                    "{} [right]: an alternative names its reading",
                    r.id
                ));
            }
            if !r.right.alternative.is_empty() && r.right.reading.is_none() {
                out.push(format!(
                    "{} [right]: a contested answer names the reading it assumes",
                    r.id
                ));
            }
        }
        // The rows of one case describe one case.
        let mut by_case = std::collections::BTreeMap::<&str, (&str, &str)>::new();
        for r in &self.case {
            let first = *by_case
                .entry(r.case.as_str())
                .or_insert((r.title.as_str(), r.owner.as_str()));
            if first != (r.title.as_str(), r.owner.as_str()) {
                out.push(format!(
                    "{}: title or owner differs from the case's first row",
                    r.id
                ));
            }
        }
        out
    }
}

/// The relation a fact string is over: `(p A)` → `p`, `(not (p A))` → `not`.
pub fn relation_of(fact: &str) -> &str {
    let body = fact.strip_prefix('(').unwrap_or(fact);
    body.split([' ', ')']).next().unwrap_or("")
}

/// A recorded state's facts over `closes`, sorted — the register's spelling of
/// a model.
pub fn project<'a>(facts: impl IntoIterator<Item = &'a str>, closes: &[String]) -> Vec<String> {
    let mut out: Vec<String> = facts
        .into_iter()
        .filter(|f| closes.iter().any(|c| c == relation_of(f)))
        .map(str::to_string)
        .collect();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{corpus, register, repo_root};

    #[test]
    fn the_register_is_well_formed() {
        let reg = register();
        assert!(!reg.case.is_empty(), "{REGISTER} has no rows");
        let defects = reg.defects();
        assert!(
            defects.is_empty(),
            "{REGISTER}:\n  {}",
            defects.join("\n  ")
        );
    }

    /// A row's witness — and every control's and twin's — is a file the
    /// corpus knows. The register adds no new kind of file: it adds the answer
    /// a corpus program should have.
    #[test]
    fn every_witness_is_a_file_the_corpus_knows() {
        let repo = repo_root();
        let corpus = corpus();
        let mut bad = Vec::new();
        for r in &register().case {
            for (label, a) in r.answers() {
                let w = r.witness_of(a);
                if !repo.join(w).is_file() {
                    bad.push(format!("{} [{label}]: {w} does not exist", r.id));
                } else if !w.starts_with(OUTSIDE_THE_CORPUS)
                    && !corpus.entry.iter().any(|e| e.path == w)
                {
                    bad.push(format!("{} [{label}]: {w} has no corpus.toml entry", r.id));
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    #[test]
    fn a_fact_is_over_its_head() {
        assert_eq!(relation_of("(p A)"), "p");
        assert_eq!(relation_of("(noted 1 1)"), "noted");
        assert_eq!(relation_of("(note edge (src Y))"), "note");
        assert_eq!(relation_of("(not (p A))"), "not");
        assert_eq!(relation_of("(clash)"), "clash");
        let closes = ["p".to_string()];
        assert_eq!(
            project(["(q A)", "(p B)", "(not (p A))", "(p A)"], &closes),
            ["(p A)", "(p B)"]
        );
    }
}
