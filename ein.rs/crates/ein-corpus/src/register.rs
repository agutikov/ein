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

/// `/2` since M1e2 S1e2.1a.10 (2026-10-16): every row states its witness's R,
/// [`Region`]. `/3` since S1e2.1a.5's switch (2026-10-18): every row states
/// [`Now`], the region run that asks its question, and the walk's blocks are
/// the record of what the walk said. `/4` since M1e2 S1e2.2.5 (2026-11-04):
/// a right answer and a `now` can be a **refusal** ([`Right::refused`],
/// [`Now::refused`]), *forbid*'s, and a refused row states no [`Region`].
/// `/5` since M1e2 S1e2.2.8 (2026-11-04): a `now` records the relations its
/// run names outside the universe ([`Now::outside`], the summary's
/// `root.hypgen.outside`), and a right answer can state them
/// ([`Right::outside`]), W6's on Q-M1e2.3 (a). `/6` since M1e2 S1e2.2.11
/// (2026-11-04): a `now` whose `right` states a core records the core its run
/// reports ([`Now::core`], the summary's `region.core`), W10's.
pub const REGISTER_SCHEMA: &str = "ein-wrong-answers/6";

/// What a [`Now`] run's states are: all of R, or one of its lines.
pub const LINES: [&str; 3] = ["all", "minimal", "maximal"];

/// The two kinds of an empty `R` as `ein-summary/2` spells them, which
/// [`Now`] reads: every node dead, or every alive node owing.
pub const SUMMARY_EMPTY: [&str; 2] = ["dead", "owing"];

/// The routes that compute a [`Region`], as `ein solve` names them.
pub const ROUTES: [&str; 3] = ["route 1", "route 2", "state route"];

/// The two kinds of an empty `R`.
pub const EMPTY_KINDS: [&str; 2] = ["no-alive", "all-owe"];

/// What a row may say about itself.
pub const STATUSES: [&str; 3] = ["open", "fixed", "not-wrong"];

/// What gave a recorded [`Answer`]: the walk, which no command has run since
/// M1e2 S1e2.1a.5's switch, or a region run (W13's `wrong`, M1e2 S1e2.1a.7).
pub const ANSWERED_BY: [&str; 2] = ["walk", "region"];

/// The verdict words, as `--json-summary`'s `verdict.type` spells them.
pub const VERDICTS: [&str; 4] = ["Solution", "Ambiguity", "Contradiction", "Open"];

/// What the review (M1e2 S1e2.1.7) may rule on a row. `deferred` since
/// 2026-11-03, the user's ruling on W8: the row's reading waits on a followup,
/// and the row stays `open` with that followup as its `owner`.
pub const REVIEW_VERDICTS: [&str; 6] = [
    "confirmed",
    "right-answer-corrected",
    "not-wrong",
    "split",
    "merge",
    "deferred",
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
    /// What the walk answered. Since M1e2 S1e2.1a.5 no command runs the
    /// walk, so this, [`Self::control`] and [`Self::twin`] are the record of
    /// what it said, and [`Self::now`] is what is run. A row filed after the
    /// switch records a region run's wrong answer, and says so
    /// ([`Answer::by`], W13's).
    pub wrong: Answer,
    /// A run that gets it right today.
    #[serde(default)]
    pub control: Option<Answer>,
    /// Further runs the case page puts beside the wrong one.
    #[serde(default)]
    pub twin: Vec<Answer>,
    pub right: Right,
    /// The witness's region, as the library computes it. Every row states one,
    /// and `ein-infer/tests/register_region.rs` holds it to `explore`.
    #[serde(default)]
    pub region: Option<Region>,
    /// The region run that asks the row's question, and what it answers —
    /// M1e2 S1e2.1a.5. `ein-cli/tests/wrong_answers.rs` runs it.
    pub now: Now,
    /// Why an `open` row's [`Self::now`] is still not [`Self::right`]: what R
    /// shares with the case, or which reading the case waits on. Required
    /// exactly when the row is open.
    #[serde(default)]
    pub open_because: Option<String>,
    /// The user's verdict on the row (S1e2.1.7). Absent until it is reviewed.
    #[serde(default)]
    pub review: Option<Review>,
    /// What made a `fixed` row's [`Self::now`] answer [`Self::right`]: the
    /// commit, the work it was, and its date — M1e2 S1e2.1a.7, so a case page
    /// says what fixed it. Stated exactly when the row is fixed.
    #[serde(default)]
    pub fixed: Option<Fixed>,
}

/// What fixed a row.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixed {
    /// The commit, abbreviated.
    pub commit: String,
    /// The stage or task, as its commit names it.
    pub by: String,
    /// `YYYY-MM-DD`.
    pub date: String,
}

fn is_date(d: &str) -> bool {
    let d = d.as_bytes();
    d.len() == 10
        && d[4] == b'-'
        && d[7] == b'-'
        && d.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

fn is_commit(c: &str) -> bool {
    c.len() >= 7 && c.bytes().all(|b| b.is_ascii_hexdigit())
}

/// A followup's id, `F<n>`, as `plans/followups/` names them.
fn is_followup(o: &str) -> bool {
    o.strip_prefix('F')
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
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

/// What a region run answers — M1e2 S1e2.1a.5, `ein-summary/2`'s fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Now {
    /// A corpus run name: the mode that asks the row's question, `solve
    /// --maximal` for a model-set row.
    pub run: String,
    /// One of [`LINES`]: which part of R its states are.
    pub line: String,
    /// The run refuses the program — M1e2 S1e2.2.5, *forbid*: exit 1, and
    /// this is the first line of its stderr. A refused run lists nothing, so
    /// the fields below keep their defaults.
    #[serde(default)]
    pub refused: Option<String>,
    #[serde(default)]
    pub exhausted: bool,
    /// The states it lists.
    #[serde(default)]
    pub k: usize,
    /// One of [`SUMMARY_EMPTY`], when the run established R empty.
    #[serde(default)]
    pub empty: Option<String>,
    /// The candidates' relations: what a state's `(model …)` names.
    #[serde(default)]
    pub relations: Vec<String>,
    /// Each listed state's `(model …)`, sorted; for an owing empty R, its
    /// frontier's, which are the open states.
    #[serde(default)]
    pub models: Vec<Vec<String>>,
    /// The relations the run names outside the universe, sorted — the
    /// summary's `root.hypgen.outside` (M1e2 S1e2.2.8): what the program can
    /// conclude or ask about and the rung that built H proposes no fact of.
    #[serde(default)]
    pub outside: Vec<String>,
    /// The unsat core the run reports, sorted — the summary's `region.core`
    /// (M1e2 S1e2.2.11). Recorded exactly where [`Right::core`] is stated,
    /// which is a core row's: what W10 is about.
    #[serde(default)]
    pub core: Option<Vec<String>>,
}

impl Now {
    /// The verdict word its states make: `Solution` for one, `Ambiguity` for
    /// more, and for none `Contradiction` or `Open` by the kind of empty R.
    pub fn verdict(&self) -> &'static str {
        match (self.k, self.empty.as_deref()) {
            (1, _) => "Solution",
            (0, Some("owing")) => "Open",
            (0, _) => "Contradiction",
            _ => "Ambiguity",
        }
    }

    /// Where it disagrees with `right`, field by field: the verdict, `k`,
    /// `exhausted`, and the models over `closes` ∩ [`Self::relations`]. A
    /// relation `closes` names that no candidate is over must hold the same
    /// facts in every model `right` lists, or the comparison could not see
    /// it.
    pub fn against(&self, right: &Right) -> Vec<String> {
        let mut out = Vec::new();
        // A refusal is the whole answer: it states no verdict, `k` or model.
        match (right.refused, self.refused.is_some()) {
            (true, true) => return out,
            (true, false) => {
                out.push("right: a refusal, now: an answer".to_string());
                return out;
            }
            (false, true) => {
                out.push("right: an answer, now: a refusal".to_string());
                return out;
            }
            (false, false) => {}
        }
        if let Some(v) = &right.verdict
            && v != self.verdict()
        {
            out.push(format!("verdict: right {v}, now {}", self.verdict()));
        }
        if let Some(k) = right.k
            && k != self.k
        {
            out.push(format!("k: right {k}, now {}", self.k));
        }
        if let Some(e) = right.exhausted
            && e != self.exhausted
        {
            out.push(format!("exhausted: right {e}, now {}", self.exhausted));
        }
        if let Some(o) = &right.outside
            && o != &self.outside
        {
            out.push(format!(
                "outside the universe: right {o:?}, now {:?}",
                self.outside
            ));
        }
        if let Some(c) = &right.core {
            let mut want = c.clone();
            want.sort();
            if Some(&want) != self.core.as_ref() {
                out.push(format!("core: right {want:?}, now {:?}", self.core));
            }
        }
        let over: Vec<String> = right
            .closes
            .iter()
            .filter(|c| self.relations.contains(c))
            .cloned()
            .collect();
        let sorted = |m: &[Vec<String>]| -> Vec<Vec<String>> {
            let mut v: Vec<Vec<String>> = m
                .iter()
                .map(|x| project(x.iter().map(String::as_str), &over))
                .collect();
            v.sort();
            v
        };
        // An owing empty R lists its frontier, which `right` calls its open
        // states; every other run lists its states, which are models.
        let owing = self.k == 0 && self.empty.as_deref() == Some("owing");
        let states: &[Vec<String>] = if owing { &[] } else { &self.models };
        if let Some(models) = &right.models {
            if sorted(models) != sorted(states) {
                out.push(format!(
                    "models over {over:?}: right {:?}, now {:?}",
                    sorted(models),
                    sorted(states)
                ));
            }
            let unseen: Vec<String> = right
                .closes
                .iter()
                .filter(|c| !self.relations.contains(c))
                .cloned()
                .collect();
            let per_model: Vec<Vec<String>> = models
                .iter()
                .map(|m| project(m.iter().map(String::as_str), &unseen))
                .collect();
            if per_model.windows(2).any(|w| w[0] != w[1]) {
                out.push(format!(
                    "{unseen:?} hold no candidate, and right's models differ over them"
                ));
            }
        }
        if let Some(open) = &right.open {
            let want = sorted(open);
            let got = if owing {
                sorted(&self.models)
            } else {
                Vec::new()
            };
            if want != got {
                out.push(format!("open states: right {want:?}, now {got:?}"));
            }
        }
        out
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
    /// One of [`ANSWERED_BY`]; the walk when absent. A region run's
    /// [`Self::why`] says how it was taken.
    #[serde(default)]
    pub by: Option<String>,
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
    /// In a build with debug assertions the run aborted with this text on
    /// stderr; every other field is the release build's. A record: W7's,
    /// which M1e2 S1e2.2.7 fixed, and no test skips a row on it since.
    #[serde(default)]
    pub debug_panic: Option<String>,
    #[serde(default)]
    pub id_order: Option<IdOrder>,
}

/// The witness's `R` — M1e2 S1e2.1a.10. What `ein solve` describes, in its
/// own vocabulary: the lines are **commitments**, each the candidates it
/// holds, sorted. The walk's vocabulary, models projected over `closes`, stays
/// in [`Answer`] and [`Right`].
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    /// One of [`ROUTES`].
    pub route: String,
    /// The static check's verdict. A flagged program's `R` is the cube's,
    /// every node a fork of root; what a node should be is F21's D2.
    pub monotone: bool,
    /// `|R|` in states.
    pub k_s: u64,
    /// `|R|` in commitments.
    pub k_c: u64,
    /// One of [`EMPTY_KINDS`], exactly when `k_s = 0`.
    #[serde(default)]
    pub empty: Option<String>,
    pub minimal: Vec<Vec<String>>,
    pub maximal: Vec<Vec<String>>,
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
    /// The answer is a **refusal** — M1e2 S1e2.2.5, *forbid*: the program is
    /// refused before any question about a commitment, a load error's exit.
    /// A refusal states no verdict, `k` or model.
    #[serde(default)]
    pub refused: bool,
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
    /// A sentence the right read-out would add. On a fixed row that states
    /// [`Self::outside`], the read-out's `outside` row, which
    /// `wrong_answers.rs` holds the run's stdout to.
    #[serde(default)]
    pub qualifier: Option<String>,
    /// The relations the right read-out names outside the universe, sorted —
    /// M1e2 S1e2.2.8, Q-M1e2.3 (a): R is over H, and the read-out says what
    /// H leaves out.
    #[serde(default)]
    pub outside: Option<Vec<String>>,
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
            if r.status == "fixed"
                && !r.right.refused
                && (r.right.verdict.is_none() || r.right.k.is_none())
            {
                out.push(at("a fixed row states the right verdict and k".to_string()));
            }
            if r.right.refused && (r.right.verdict.is_some() || r.right.k.is_some()) {
                out.push(at("a refusal states no verdict and no k".to_string()));
            }
            if r.now.refused.is_some()
                && (r.now.k != 0
                    || r.now.exhausted
                    || r.now.empty.is_some()
                    || !r.now.models.is_empty()
                    || !r.now.relations.is_empty())
            {
                out.push(at("a refused [now] lists nothing".to_string()));
            }
            if r.now.refused.is_some() && r.region.is_some() {
                out.push(at(
                    "a refused [now] has no [case.region]: the library refuses it too".to_string(),
                ));
            }
            if (r.status == "open") != r.open_because.is_some() {
                out.push(at(
                    "`open_because` is stated exactly when the row is open".to_string()
                ));
            }
            if !r.now.run.starts_with("solve") {
                out.push(at(format!(
                    "[now] run {:?} is not a solve — the register reads --json-summary",
                    r.now.run
                )));
            }
            if r.now.core.is_some() != r.right.core.is_some() {
                out.push(at(
                    "[now] records a core exactly when [right] states one".to_string()
                ));
            }
            if let Some(c) = &r.now.core
                && c.windows(2).any(|w| w[0] > w[1])
            {
                out.push(at("[now] core is sorted".to_string()));
            }
            if !LINES.contains(&r.now.line.as_str()) {
                out.push(at(format!(
                    "[now] line {:?} is not one of {LINES:?}",
                    r.now.line
                )));
            }
            if r.now.models.len() != r.now.k
                && !(r.now.k == 0 && r.now.empty.as_deref() == Some("owing"))
            {
                out.push(at(format!(
                    "[now] {} models listed for k = {}",
                    r.now.models.len(),
                    r.now.k
                )));
            }
            if let Some(e) = &r.now.empty
                && !SUMMARY_EMPTY.contains(&e.as_str())
            {
                out.push(at(format!(
                    "[now] empty {e:?} is not one of {SUMMARY_EMPTY:?}"
                )));
            }
            // A fixed row is fixed because its region run answers `right`:
            // the comparison is data, so it holds without a run, and
            // `wrong_answers.rs` holds the run to `now`.
            if r.status == "fixed" {
                for d in r.now.against(&r.right) {
                    out.push(at(format!("fixed, and [now] is not [right]: {d}")));
                }
            }
            if let Some(rv) = &r.review {
                if !REVIEW_VERDICTS.contains(&rv.verdict.as_str()) {
                    out.push(at(format!(
                        "review verdict {:?} is not one of {REVIEW_VERDICTS:?}",
                        rv.verdict
                    )));
                }
                if !is_date(&rv.date) {
                    out.push(at(format!("review date {:?} is not YYYY-MM-DD", rv.date)));
                }
                if !is_commit(&rv.bundle) {
                    out.push(at(format!("review bundle {:?} is not a commit", rv.bundle)));
                }
            }
            if (r.status == "fixed") != r.fixed.is_some() {
                out.push(at(
                    "`[case.fixed]` is stated exactly when the row is fixed".to_string()
                ));
            }
            if let Some(f) = &r.fixed {
                if !is_commit(&f.commit) {
                    out.push(at(format!("[fixed] commit {:?} is not a commit", f.commit)));
                }
                if !is_date(&f.date) {
                    out.push(at(format!("[fixed] date {:?} is not YYYY-MM-DD", f.date)));
                }
            }
            if (r.status == "not-wrong")
                != r.review.as_ref().is_some_and(|v| v.verdict == "not-wrong")
            {
                out.push(at(
                    "status `not-wrong` and review verdict `not-wrong` go together".to_string(),
                ));
            }
            if r.review.as_ref().is_some_and(|v| v.verdict == "deferred")
                && (r.status != "open" || !is_followup(&r.owner))
            {
                out.push(at(format!(
                    "a `deferred` review is an open row owned by the followup it waits on \
                     (F<n>); this one is {:?}, owned by {:?}",
                    r.status, r.owner
                )));
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
                match a.by.as_deref() {
                    None | Some("walk") => {}
                    Some("region") if a.why.is_none() => out.push(at(
                        "an answer a region run gave says how it was taken (`why`)".to_string(),
                    )),
                    Some("region") => {}
                    Some(b) => out.push(at(format!("by {b:?} is not one of {ANSWERED_BY:?}"))),
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
            match &r.region {
                // A refused row states no region (M1e2 S1e2.2.5): checked above.
                None if r.now.refused.is_some() => {}
                None => out.push(format!("{}: no [case.region]", r.id)),
                Some(g) => {
                    if !ROUTES.contains(&g.route.as_str()) {
                        out.push(format!(
                            "{} [region]: route {:?} is not one of {ROUTES:?}",
                            r.id, g.route
                        ));
                    }
                    let empty = g.k_s == 0;
                    if empty != g.empty.is_some() {
                        out.push(format!(
                            "{} [region]: `empty` is stated exactly when k_s = 0",
                            r.id
                        ));
                    }
                    if let Some(e) = &g.empty
                        && !EMPTY_KINDS.contains(&e.as_str())
                    {
                        out.push(format!(
                            "{} [region]: empty {e:?} is not one of {EMPTY_KINDS:?}",
                            r.id
                        ));
                    }
                    if empty != (g.minimal.is_empty() && g.maximal.is_empty()) || g.k_c < g.k_s {
                        out.push(format!(
                            "{} [region]: the lines, k_s and k_c disagree",
                            r.id
                        ));
                    }
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
        // The rows of one case describe one case: one title, and one owner
        // per status, since a fixed row names the stage that fixed it and an
        // open one the stage that will. A row the review re-opened can move
        // to another stage (M1e2 S1e2.1.7: `W1/alive-empty-interlayer`'s
        // refusal was S1e2.2.5's while W1's fixed rows stayed S1e2.1a.5's),
        // and so fixed rows share an owner per commit that fixed them (M1e2
        // S1e2.2.5 fixed two W1 rows beside the switch's twelve).
        let mut by_case = std::collections::BTreeMap::<&str, &str>::new();
        let mut by_status = std::collections::BTreeMap::<(&str, &str, Option<&str>), &str>::new();
        for r in &self.case {
            let title = *by_case.entry(r.case.as_str()).or_insert(r.title.as_str());
            let owner = *by_status
                .entry((
                    r.case.as_str(),
                    r.status.as_str(),
                    r.fixed.as_ref().map(|f| f.commit.as_str()),
                ))
                .or_insert(r.owner.as_str());
            if (title, owner) != (r.title.as_str(), r.owner.as_str()) {
                out.push(format!(
                    "{}: title differs from the case's first row, or owner from the first \
                     row of its case and status (and, for a fixed row, its fixing commit)",
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

    /// **What fixed a row, and what gave its `wrong`**, M1e2 S1e2.1a.7: a
    /// fixed row states `[case.fixed]` and no other row does, and a recorded
    /// answer a region run gave says how it was taken.
    #[test]
    fn a_fixed_row_says_what_fixed_it() {
        let text = std::fs::read_to_string(repo_root().join(REGISTER)).expect("the register");
        let reg: Register = toml::from_str(&text).expect("parses");
        let mut row = reg
            .case
            .iter()
            // The first *fixed* one: W10/alt-cap-shared, filed open by M1e2
            // S1e2.2.11, comes before W13 and is a region run's too.
            .find(|r| r.wrong.by.as_deref() == Some("region") && r.status == "fixed")
            .expect("W13's wrong is a region run's")
            .clone();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let fixed = row.fixed.take();
        assert!(defects(&row).iter().any(|d| d.contains("[case.fixed]")));
        row.fixed = fixed;
        row.wrong.why = None;
        assert!(
            defects(&row)
                .iter()
                .any(|d| d.contains("says how it was taken"))
        );
        row.wrong.by = Some("oracle".to_string());
        assert!(defects(&row).iter().any(|d| d.contains("is not one of")));
    }

    /// **An open row is open by the method**, M1e2 S1e2.1c.5's step 2,
    /// questions (i) and (iii), held without a run. A row whose `[case.now]`
    /// answers its `right` on every field the register records closes by
    /// construction, unless its reading is contested: a contested row keeps
    /// its stage and its question, and the revisit (S1e2.1.7) reads it. A
    /// `right` that states what `[case.now]` does not record keeps the row
    /// open on that field, and the row's run shows it, not this comparison:
    /// a refusal, which states no verdict; a qualifier the register
    /// does not record as `outside` (M1e2 S1e2.2.8); or facts over
    /// a relation no candidate is over, since a state's `(model …)` names H's
    /// relations only (W7's `noted`, W8's `p`). Taken 2026-11-03: no row
    /// closed, and W12 is the one contested row (i) holds for. A core has
    /// been recorded since M1e2 S1e2.2.11 ([`Now::core`]), so it is compared,
    /// not exempt.
    #[test]
    fn an_open_row_is_open_by_the_method() {
        let mut bad = Vec::new();
        for r in &register().case {
            if r.status != "open" {
                continue;
            }
            let differs = r.now.against(&r.right);
            // A state's `(model …)` names the candidates' relations only, so
            // facts `right` states over any other relation are not in `now`.
            let beyond_h = r.right.models.is_some()
                && r.right.closes.iter().any(|c| !r.now.relations.contains(c));
            let unrecorded: Vec<&str> = [
                (r.right.verdict.is_none(), "a refusal or no answer"),
                (
                    r.right.qualifier.is_some() && r.right.outside.is_none(),
                    "a qualifier",
                ),
                (beyond_h, "facts over a relation no candidate is over"),
            ]
            .into_iter()
            .filter_map(|(on, what)| on.then_some(what))
            .collect();
            let contested = r.class.contains("contested");
            if differs.is_empty() && unrecorded.is_empty() && !contested {
                bad.push(format!(
                    "{}: open, uncontested, and [now] answers [right] on every field the \
                     register records — it closes by construction (S1e2.1c.5's method): \
                     flip it to fixed with the test that holds it",
                    r.id
                ));
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    /// **A deferred row**, M1e2 S1e2.1.7: the user deferred W8's reading to
    /// F21 on 2026-11-03, so its rows stay open and the followup owns them.
    /// A `deferred` review on a row that is not open, or that a stage owns, is
    /// a defect.
    #[test]
    fn a_deferred_row_is_open_and_owned_by_its_followup() {
        let mut row = register()
            .case
            .into_iter()
            .find(|r| r.review.as_ref().is_some_and(|v| v.verdict == "deferred"))
            .expect("W8's rows are deferred");
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let owner = std::mem::replace(&mut row.owner, "S1e2.2.6".to_string());
        assert!(defects(&row).iter().any(|d| d.contains("`deferred`")));
        row.owner = owner;
        row.status = "not-wrong".to_string();
        assert!(defects(&row).iter().any(|d| d.contains("`deferred`")));
        assert!(is_followup("F21") && !is_followup("F") && !is_followup("S1e2.2.6"));
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
