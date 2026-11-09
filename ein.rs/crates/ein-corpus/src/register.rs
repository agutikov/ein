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
/// reports ([`Now::core`], the summary's `region.core`), W10's. `/7` since
/// M1e2 S1e2.2.18 (2026-11-04): a recorded answer and a `now` can be a run
/// that **ended in an error** before it answered ([`Answer::error`],
/// [`Now::error`]), exit 1 with no verdict, W18's `str-in-assert`; such a
/// `now` is never `right`, and states no [`Region`]. `/8` since M1e2
/// S1e2.2.19 (2026-11-04): a `now` whose `right` states the cores of `N`'s
/// members records the cores its run reports ([`Now::nogoods`],
/// `--dump-answer`'s `nogoods.json`), W19's: a dead node's core, where
/// [`Now::core`] is root's. `/9` since M1e2 S1e2.2.23 (2026-11-05): a `now`
/// whose `right` states a core's status records the status its run reports
/// beside root's core ([`Now::core_status`], the summary's
/// `region.core_status`), W23's: whether the core search was cut at its
/// budget. `/10` since M1e2 S1e2.2.25 (2026-11-05): a `now` can record how
/// many members of R hold a stale fact ([`Now::stale`], the summary's
/// `monotonicity.state`) where `right` states it ([`Right::stale`]), W8's rows
/// past the oracle's limit; a region can be a **stopped** description
/// ([`Region::stopped`]), its counts lower bounds and its maximal line not
/// known; `now` can compare `N` on the members it lists alone
/// ([`Now::nogoods_listed`], a program whose `N` is hundreds of sets); and a
/// `deferred` row can be owned by another milestone's stage (S1f.10.6) as
/// well as by a followup. `/11` since M1e2 S1e2.2.26 (2026-11-05): a `now`
/// records how many members of R it met and counts without listing them
/// ([`Now::others`], the summary's states with `listed = false`) where
/// `right` states it ([`Right::others`]), W27's: a line mode past
/// `--region-limit`, whose listed states are the line members it certifies
/// and make no verdict. `/12` since M1e2 S1e2.3.8 (2026-11-06): a `now`
/// records the outcome of the program's solve-form claim ([`Now::claim`],
/// the summary's `claim.outcome`) and each query's class count and claim
/// outcome ([`Now::queries`], the summary's `queries`) where `right` states
/// them, W28's: a claim and a query's classes compared printed text, in which
/// the int `1` and the name `"1"` were one. A run whose recorded claim is
/// `failed` exits 1, as a refuted claim does ([`Now::refutes`]). `/13` since
/// M1f S1f.5.0 (2026-11-08): where `right` is a **refusal**, a `now` may record
/// the core and the claims its run reports ([`Now::core`], [`Now::claim`],
/// [`Now::queries`]), what the refusal replaces, W29's
/// (`W29/core-two-ways`'s core, a bare `_` in a rule; `W29/goal-arg`'s and
/// `W32/kwpair-in-goal`'s query); a refused `now` records none of them.
/// `/14` since M1f S1f.5.15 (2026-11-08): a `now` records the rules root's
/// `refuted by` names ([`Now::refuted_by`], the summary's `region.refuted_by`)
/// where `right` states them ([`Right::refuted_by`]), W36's: under
/// `:record-alternative-justifications false` the row names other rules
/// beside the same core, which the config principle does not exempt.
/// `/15` since M1f S1f.5.5 (2026-11-08): a `now` records the sentences the
/// read-out prints for a query's `:goal-text` ([`Now::goal_text`], its `“…”`
/// lines, which the summary does not carry) where `right` states them
/// ([`Right::goal_text`]), W37's: a template reference that names a variable
/// with a character other than a letter, a digit, `_` or `-` is printed as
/// written. `/16` since M1f S1f.5.5 (T1f.5.5.13, 2026-11-09): a `now` can be
/// a **command's** run, any `ein` run that is not a solve ([`Now::is_command`]),
/// which writes no summary: its answer is a refusal or an error, as a solve's
/// can be, or that it answered ([`Now::answered`], [`Right::answered`]), exit 0
/// with nothing on stderr; a `now` can read the witness through a `.einb`
/// ([`Now::container`]), saved by `ein kb save` first, and then answers what the
/// same run answers on the witness; and a recorded answer can be a command's
/// ([`Answer::by`] `command`, with [`Answer::refused`]), W40's and W41's: one
/// command refused a program another answered. `/17` since M1f S1f.5.6
/// (T1f.5.6.5, 2026-11-09): a `now` records lines the run prints, each found
/// by the label it begins with, on stdout or in `--dump-answer`'s
/// `answer.txt` ([`Now::printed`], [`Printed`]), where `right` states them
/// ([`Right::printed`]), W42's: a label as long as its column glued to its
/// value, so `class 100 of 277` and `28 states` read as `class 100 of 27728
/// states`.
pub const REGISTER_SCHEMA: &str = "ein-wrong-answers/17";

/// Where a [`Printed`] line is read (schema /17): the run's stdout, or the
/// read-out `--dump-answer DIR` writes uncut, `DIR/answer.txt`.
pub const PRINTED_FILES: [&str; 2] = ["stdout", "answer.txt"];

/// A line a run prints — schema `/17` (M1f S1f.5.6, W42): the first line of
/// [`Self::file`] that begins, past its indent, with [`Self::label`], as
/// printed, its trailing spaces aside.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Printed {
    /// One of [`PRINTED_FILES`].
    pub file: String,
    /// What the line begins with, past its indent: a row's label, or `-H`'s
    /// relation.
    pub label: String,
    /// The whole line.
    pub line: String,
}

/// What a claim's check came to, as the summary spells it (schema /12):
/// `none` where the program states no such claim.
pub const CLAIM_OUTCOMES: [&str; 4] = ["held", "failed", "not-checked", "none"];

/// What a [`Now`] run's states are: all of R, or one of its lines.
pub const LINES: [&str; 3] = ["all", "minimal", "maximal"];

/// The two kinds of an empty `R` as `ein-summary/2` spells them, which
/// [`Now`] reads: every node dead, or every alive node owing.
pub const SUMMARY_EMPTY: [&str; 2] = ["dead", "owing"];

/// The routes that compute a [`Region`], as `ein solve` names them.
pub const ROUTES: [&str; 3] = ["route 1", "route 2", "state route"];

/// Why a stopped [`Region`] stopped (schema /10): past `--region-limit` on a
/// program the static check flags, or at `--region-budget`.
pub const STOPS: [&str; 2] = ["limit", "budget"];

/// The two kinds of an empty `R`.
pub const EMPTY_KINDS: [&str; 2] = ["no-alive", "all-owe"];

/// What a row may say about itself.
pub const STATUSES: [&str; 3] = ["open", "fixed", "not-wrong"];

/// What gave a recorded [`Answer`]: the walk, which no command has run since
/// M1e2 S1e2.1a.5's switch, a region run (W13's `wrong`, M1e2 S1e2.1a.7), or
/// since schema /16 a command's run that is not a solve (W40's and W41's
/// `wrong`, M1f T1f.5.5.13).
pub const ANSWERED_BY: [&str; 3] = ["walk", "region", "command"];

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

/// A core's status as the register spells it (schema /9): `none`,
/// `within-budget`, or `cut-` and one or more of the caps, in order —
/// `ein_infer::explain::CoreStatus::word`'s words for a core a search took,
/// and `none`. Not `not-kept`, M1e2 S1e2.3.9's word for a member of N a run
/// past `--region-limit` printed with no core, retired by M1f S1f.5.12: a
/// row records the summary's `region.core_status`, root's, which a run
/// always takes.
fn is_core_status(s: &str) -> bool {
    if s == "none" || s == "within-budget" {
        return true;
    }
    let Some(caps) = s.strip_prefix("cut-") else {
        return false;
    };
    let order = ["environments", "rounds", "facts"];
    let mut at = 0;
    for c in caps.split('-') {
        match order[at..].iter().position(|o| *o == c) {
            Some(i) => at += i + 1,
            None => return false,
        }
    }
    true
}

/// A followup's id, `F<n>`, as `plans/followups/` names them.
fn is_followup(o: &str) -> bool {
    o.strip_prefix('F')
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// Another milestone's stage, `S1f.10.6` (schema /10, M1e2 S1e2.2.25): a
/// stage id whose milestone is not M1e2's, which a `deferred` row may wait on
/// as it waits on a followup (W26's, Q-M1e2.16 (5)).
fn is_other_milestone_stage(o: &str) -> bool {
    let parts: Vec<&str> = o.split('.').collect();
    parts.len() == 3
        && parts[0].len() > 1
        && parts[0].starts_with('S')
        && parts[0][1..].bytes().all(|b| b.is_ascii_alphanumeric())
        && parts[0] != "S1e2"
        && parts[1..]
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
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
    /// --maximal` for a model-set row; or since schema /16 a command's run
    /// that is not a solve, `saturate --dump` or `render rules`
    /// ([`Self::is_command`]).
    pub run: String,
    /// One of [`LINES`]: which part of R its states are. Empty on a
    /// command's run (schema /16), which lists no state.
    #[serde(default)]
    pub line: String,
    /// The run reads the witness through a `.einb` (schema /16, M1f
    /// T1f.5.5.13, W40): `ein kb save WITNESS DIR/<stem>.einb` first, then
    /// the run on the container.
    #[serde(default)]
    pub container: bool,
    /// A command's run answered (schema /16, W40, W41): exit 0, with nothing
    /// on stderr, and with [`Self::container`] its stdout is the same run's on
    /// the witness, every number a line times masked and the lines that say
    /// what the run read (`input:`, its size, `parse:`, `kb load:`, `open:`)
    /// aside. A command's `now` is a refusal, an error or this.
    #[serde(default)]
    pub answered: bool,
    /// The run refuses the program — M1e2 S1e2.2.5, *forbid*: exit 1, and
    /// this is the first line of its stderr. A refused run lists nothing, so
    /// the fields below keep their defaults.
    #[serde(default)]
    pub refused: Option<String>,
    /// The run ended in an error before it answered — M1e2 S1e2.2.18 (W18):
    /// exit 1, and this is the first line of its stderr. Not a refusal: the
    /// program was accepted and a firing stopped the run, so the library
    /// returns a saturation error where a refusal returns
    /// `SolveError::Refused`. Such a run lists nothing, and is never
    /// [`Right`]. (Not the summary's `region.stopped`, which says why a
    /// search that answered stopped.)
    #[serde(default)]
    pub error: Option<String>,
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
    /// Each member of `N` the run reports and its core — `--dump-answer`'s
    /// `nogoods.json` (M1e2 S1e2.2.19, W19), each set and core sorted, the
    /// list sorted by set. Recorded exactly where [`Right::nogoods`] is
    /// stated: [`Self::core`] is root's, and a dead node's core is not.
    #[serde(default)]
    pub nogoods: Option<Vec<Nogood>>,
    /// The status the run reports beside root's core — the summary's
    /// `region.core_status` (M1e2 S1e2.2.23, W23): `within-budget`, `cut-`
    /// and the caps its search stopped at, or `none` where the summary
    /// states none (a `null`, or a run before S1e2.2.23, which wrote no such
    /// field). Recorded exactly where [`Right::core_status`] is stated.
    #[serde(default)]
    pub core_status: Option<String>,
    /// How many members of R the run met hold a stale fact, and of how many
    /// nodes — the summary's `monotonicity.state` (schema /10, M1e2
    /// S1e2.2.25): its `stale` entries of class `R`, and its `nodes`.
    /// Recorded exactly where [`Right::stale`] is stated (W8's rows past the
    /// oracle's limit, whose states are not listed).
    #[serde(default)]
    pub stale: Option<Stale>,
    /// [`Self::nogoods`] lists some members of `N`, and the run's `N` is
    /// compared on those sets alone, each of which it must report (schema
    /// /10, M1e2 S1e2.2.25): `zebra2.ein`'s `N` is 366 sets.
    #[serde(default)]
    pub nogoods_listed: bool,
    /// How many members of R the run met and counts without listing them —
    /// the summary's states with `listed = false` (schema /11, M1e2
    /// S1e2.2.26): what a line mode past `--region-limit` leaves to
    /// `--dump-answer`. Recorded exactly where [`Right::others`] is stated
    /// (W27's rows).
    #[serde(default)]
    pub others: Option<u64>,
    /// The outcome of the program's `(solve … :expect …)` claim — the
    /// summary's `claim.outcome`, one of [`CLAIM_OUTCOMES`], `none` where the
    /// summary states none (schema /12, M1e2 S1e2.3.8, W28). Recorded exactly
    /// where [`Right::claim`] is stated.
    #[serde(default)]
    pub claim: Option<String>,
    /// Each query's projection classes and claim, in program order — the
    /// summary's `queries` (schema /12, M1e2 S1e2.3.8, W28). Recorded
    /// exactly where [`Right::queries`] is stated.
    #[serde(default)]
    pub queries: Option<Vec<QueryAnswer>>,
    /// The rules root's `refuted by` row names, in the order it prints them —
    /// the summary's `region.refuted_by` (schema /14, M1f S1f.5.15, W36),
    /// empty where the summary states none. Recorded exactly where
    /// [`Right::refuted_by`] is stated.
    #[serde(default)]
    pub refuted_by: Option<Vec<String>>,
    /// Each sentence the read-out prints for a query's `:goal-text`, one per
    /// row of a projection class, in the order printed (schema /15, M1f
    /// S1f.5.5, W37): the read-out's `“…”` lines, since the summary carries
    /// the rows and not the sentences. Recorded exactly where
    /// [`Right::goal_text`] is stated.
    #[serde(default)]
    pub goal_text: Option<Vec<String>>,
    /// Lines the run prints, each found by its label (schema /17, M1f
    /// S1f.5.6, W42): on stdout, or in `--dump-answer`'s `answer.txt`, which
    /// the run then writes. Recorded exactly where [`Right::printed`] is
    /// stated.
    #[serde(default)]
    pub printed: Option<Vec<Printed>>,
}

/// One query's answer — schema `/12` (M1e2 S1e2.3.8, W28): how many
/// projection classes it has over the states it asks about, and what its
/// `:expect` came to.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryAnswer {
    /// The summary's `queries[i].classes`, counted.
    pub classes: u64,
    /// The summary's `queries[i].expect.outcome`, one of [`CLAIM_OUTCOMES`],
    /// `none` where the query states no claim.
    pub expect: String,
}

/// Members of R holding a stale fact, of the nodes a run computed — schema
/// `/10` (M1e2 S1e2.2.25).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stale {
    /// Members of R whose own fixpoint falsifies a fact's `absent` guard.
    pub in_r: u64,
    /// The nodes the run computed.
    pub nodes: u64,
}

/// A minimal dead set and the core its run reports for it — schema `/8`
/// (M1e2 S1e2.2.19, W19): one entry of `--dump-answer`'s `nogoods.json`,
/// its `refuted_by` left out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nogood {
    /// The commitment, sorted.
    pub set: Vec<String>,
    /// Its core, sorted.
    pub core: Vec<String>,
}

/// `nogoods` as the register compares them: each set and core sorted, and
/// the list sorted.
pub fn sorted_nogoods(n: &[Nogood]) -> Vec<Nogood> {
    let mut v: Vec<Nogood> = n
        .iter()
        .map(|x| {
            let (mut set, mut core) = (x.set.clone(), x.core.clone());
            set.sort();
            core.sort();
            Nogood { set, core }
        })
        .collect();
    v.sort();
    v
}

impl Now {
    /// Is the run a command's that is not a solve (schema /16)? Then it writes
    /// no summary, and its answer is a refusal, an error or
    /// [`Self::answered`].
    pub fn is_command(&self) -> bool {
        !self.run.starts_with("solve")
    }

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
    /// Does the run refute a claim the program states? Then it exits 1, as
    /// `ein solve` does on a refuted claim (schema /12, W28's fixed rows).
    pub fn refutes(&self) -> bool {
        self.claim.as_deref() == Some("failed")
            || self.queries.iter().flatten().any(|q| q.expect == "failed")
    }

    pub fn against(&self, right: &Right) -> Vec<String> {
        let mut out = Vec::new();
        // An error is no answer at all, so it is never the right one (W18).
        if self.error.is_some() {
            out.push(format!(
                "right: {}, now: an error",
                if right.refused {
                    "a refusal"
                } else {
                    "an answer"
                }
            ));
            return out;
        }
        // A command's answer (schema /16) is that it answered: no verdict,
        // `k` or model, which its run does not write.
        match (right.answered, self.answered) {
            (true, true) => return out,
            (true, false) => {
                out.push(format!(
                    "right: the command answers, now: {}",
                    if self.refused.is_some() {
                        "a refusal"
                    } else {
                        "no answer"
                    }
                ));
                return out;
            }
            (false, true) => {
                out.push(format!(
                    "right: {}, now: the command answers",
                    if right.refused {
                        "a refusal"
                    } else {
                        "a solve's answer"
                    }
                ));
                return out;
            }
            (false, false) => {}
        }
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
        // The rules `refuted by` names (schema /14, M1f S1f.5.15, W36).
        if let Some(w) = &right.refuted_by
            && Some(w) != self.refuted_by.as_ref()
        {
            out.push(format!(
                "refuted by: right {w:?}, now {:?}",
                self.refuted_by
            ));
        }
        // A query's sentences (schema /15, M1f S1f.5.5, W37), in order.
        if let Some(w) = &right.goal_text
            && Some(w) != self.goal_text.as_ref()
        {
            out.push(format!("goal text: right {w:?}, now {:?}", self.goal_text));
        }
        // The lines printed (schema /17, M1f S1f.5.6, W42).
        if let Some(w) = &right.printed
            && Some(w) != self.printed.as_ref()
        {
            out.push(format!("printed: right {w:?}, now {:?}", self.printed));
        }
        // The core's status (schema /9, M1e2 S1e2.2.23, W23).
        if let Some(s) = &right.core_status
            && Some(s) != self.core_status.as_ref()
        {
            out.push(format!(
                "core status: right {s:?}, now {:?}",
                self.core_status
            ));
        }
        // The claims and the classes (schema /12, M1e2 S1e2.3.8, W28).
        if let Some(c) = &right.claim
            && Some(c) != self.claim.as_ref()
        {
            out.push(format!("claim: right {c:?}, now {:?}", self.claim));
        }
        if let Some(q) = &right.queries
            && Some(q) != self.queries.as_ref()
        {
            out.push(format!("queries: right {q:?}, now {:?}", self.queries));
        }
        // The members met and not listed (schema /11, M1e2 S1e2.2.26).
        if let Some(o) = right.others
            && Some(o) != self.others
        {
            out.push(format!(
                "members met and not listed: right {o}, now {:?}",
                self.others
            ));
        }
        // Stale members of R (schema /10, M1e2 S1e2.2.25).
        if let Some(w) = right.stale
            && Some(w) != self.stale.as_ref().map(|s| s.in_r)
        {
            out.push(format!(
                "stale members of R: right {w}, now {:?}",
                self.stale.as_ref().map(|s| s.in_r)
            ));
        }
        // N's cores (schema /8, M1e2 S1e2.2.19, W19).
        if let Some(n) = &right.nogoods {
            let want = sorted_nogoods(n);
            let got = self.nogoods.as_deref().map(sorted_nogoods);
            if Some(&want) != got.as_ref() {
                out.push(format!("nogoods: right {want:?}, now {got:?}"));
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
    /// [`Self::why`] says how it was taken, and so does a command's (schema
    /// /16), whose [`Self::run`] is not a solve.
    #[serde(default)]
    pub by: Option<String>,
    /// The run read the witness through a `.einb`, as [`Now::container`]
    /// (schema /16). A record.
    #[serde(default)]
    pub container: bool,
    /// The run refused the program (schema /16, a command's): `exit` is 1 and
    /// this is the first line of its stderr, and `verdict`, `k` and
    /// `exhausted` are not stated. A record (W41's `wrong`).
    #[serde(default)]
    pub refused: Option<String>,
    /// `NAME=value`, set for the run.
    #[serde(default)]
    pub env: Vec<String>,
    /// What differs from the wrong run, in one line.
    #[serde(default)]
    pub why: Option<String>,
    #[serde(default)]
    pub exit: i32,
    /// The run ended in an error before it answered — M1e2 S1e2.2.18 (W18):
    /// `exit` is 1 and this is the first line of its stderr, and `verdict`,
    /// `k` and `exhausted` are not stated.
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub verdict: String,
    #[serde(default)]
    pub k: usize,
    #[serde(default)]
    pub exhausted: bool,
    #[serde(default)]
    pub core: Option<Vec<String>>,
    /// Each member of `N` and its core, as the run reported them — schema
    /// `/8` (M1e2 S1e2.2.19, W19's `wrong`).
    #[serde(default)]
    pub nogoods: Option<Vec<Nogood>>,
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
    /// which M1e2 S1e2.2.7 fixed, and W16's order pair's, which M1e2
    /// S1e2.2.16 fixed; no test skips a row on it since.
    #[serde(default)]
    pub debug_panic: Option<String>,
    #[serde(default)]
    pub id_order: Option<IdOrder>,
    /// The solve-form claim's outcome, as [`Now::claim`] — schema `/12`
    /// (M1e2 S1e2.3.8, W28's `wrong`). A record.
    #[serde(default)]
    pub claim: Option<String>,
    /// Each query's classes and claim outcome, as [`Now::queries`] — schema
    /// `/12` (W28's `wrong`). A record.
    #[serde(default)]
    pub queries: Option<Vec<QueryAnswer>>,
    /// The rules root's `refuted by` names, as [`Now::refuted_by`] — schema
    /// `/14` (M1f S1f.5.15, W36's `wrong`). A record.
    #[serde(default)]
    pub refuted_by: Option<Vec<String>>,
    /// The sentences the read-out printed for a query's `:goal-text`, as
    /// [`Now::goal_text`] — schema `/15` (M1f S1f.5.5, W37's `wrong`). A
    /// record.
    #[serde(default)]
    pub goal_text: Option<Vec<String>>,
    /// The lines printed, as [`Now::printed`] — schema `/17` (M1f S1f.5.6,
    /// W42's `wrong`). A record.
    #[serde(default)]
    pub printed: Option<Vec<Printed>>,
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
    /// One of [`STOPS`] where the description stopped before R was exhausted
    /// (schema /10, M1e2 S1e2.2.25): `k_s` and `k_c` are then the lower
    /// bounds it reached, `minimal` the line members it certified, and
    /// `maximal` is empty, not known (W8's rows past the oracle's limit).
    #[serde(default)]
    pub stopped: Option<String>,
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
    /// The answer is that a command's run **answers** (schema /16, M1f
    /// T1f.5.5.13, W40, W41): a program another command answers is answered
    /// by this one too, exit 0, which [`Now::answered`] records. It states no
    /// verdict, `k` or model.
    #[serde(default)]
    pub answered: bool,
    #[serde(default)]
    pub verdict: Option<String>,
    #[serde(default)]
    pub k: Option<usize>,
    #[serde(default)]
    pub exhausted: Option<bool>,
    #[serde(default)]
    pub core: Option<Vec<String>>,
    /// Each member of `N` and the core the definitions give it — schema `/8`
    /// (M1e2 S1e2.2.19, W19): the facts its refutation rests on, which name
    /// the commitment.
    #[serde(default)]
    pub nogoods: Option<Vec<Nogood>>,
    /// The status the right read-out reports beside root's core — schema
    /// `/9` (M1e2 S1e2.2.23, W23): `cut-` and the caps a search stopped at,
    /// where the user ruled that a cut core says so (Q-M1e2.16).
    #[serde(default)]
    pub core_status: Option<String>,
    /// How many members of R hold a stale fact under the reading — schema
    /// `/10` (M1e2 S1e2.2.25): compared with [`Now::stale`]'s `in_r`.
    #[serde(default)]
    pub stale: Option<u64>,
    /// How many members of R the right read-out meets and counts without
    /// listing them — schema `/11` (M1e2 S1e2.2.26, W27): a line mode past
    /// `--region-limit` lists the line members it certifies, [`Self::k`] of
    /// them, which make no verdict, so a right that states `others` states
    /// `k` and `exhausted` and no verdict, and `exhausted = false` where it
    /// meets a member it does not list. A minimal question where ∅ is in R
    /// is decided at root, past the limit too (M1f S1f.5.6), so it meets
    /// none: `others = 0` and `exhausted = true`. Compared with
    /// [`Now::others`].
    #[serde(default)]
    pub others: Option<u64>,
    /// The outcome the program's solve-form claim should come to — schema
    /// `/12` (M1e2 S1e2.3.8, W28). Compared with [`Now::claim`].
    #[serde(default)]
    pub claim: Option<String>,
    /// Each query's classes and claim outcome, in program order, as the
    /// definitions give them — schema `/12` (M1e2 S1e2.3.8, W28). Compared
    /// with [`Now::queries`].
    #[serde(default)]
    pub queries: Option<Vec<QueryAnswer>>,
    /// The rules root's `refuted by` should name, in order — schema `/14`
    /// (M1f S1f.5.15, W36). Compared with [`Now::refuted_by`].
    #[serde(default)]
    pub refuted_by: Option<Vec<String>>,
    /// The sentences the read-out should print for a query's `:goal-text`,
    /// in order, each template's references filled — schema `/15` (M1f
    /// S1f.5.5, W37). Compared with [`Now::goal_text`].
    #[serde(default)]
    pub goal_text: Option<Vec<String>>,
    /// The lines the run should print, each found by its label — schema
    /// `/17` (M1f S1f.5.6, W42). Compared with [`Now::printed`].
    #[serde(default)]
    pub printed: Option<Vec<Printed>>,
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
    /// `wrong_answers.rs` holds the run's stdout to; on one that states
    /// [`Self::core_status`], its `core search` row (W23).
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
            // A line past the limit makes no verdict (schema /11, W27): its
            // right states `k` and the members it does not list instead.
            if r.status == "fixed"
                && !r.right.refused
                && !r.right.answered
                && (r.right.k.is_none() || (r.right.verdict.is_none() && r.right.others.is_none()))
            {
                out.push(at("a fixed row states the right verdict and k".to_string()));
            }
            // Since M1f S1f.5.6 a line decided at root meets no member it does
            // not list, and is exhausted: `others = 0`.
            if let Some(o) = r.right.others
                && (r.right.verdict.is_some()
                    || r.right.exhausted.is_none()
                    || (o > 0 && r.right.exhausted != Some(false)))
            {
                out.push(at(
                    "a right that states `others` is a line mode's: no verdict, `exhausted` \
                     stated, and false where it meets a member it does not list"
                        .to_string(),
                ));
            }
            // What `now` records beside `right` (schema /13, M1f S1f.5.0):
            // a field `right` states, or, where `right` is a refusal, the core
            // and the claims today's answer reports, which the refusal
            // replaces (W29's).
            let recorded = |now: bool, right: bool| now == right || (r.right.refused && !right);
            // The claims and the classes (schema /12, M1e2 S1e2.3.8).
            if !recorded(r.now.claim.is_some(), r.right.claim.is_some())
                || !recorded(r.now.queries.is_some(), r.right.queries.is_some())
            {
                out.push(at(
                    "[now] records a claim and the queries exactly when [right] states them, \
                     or beside a refusal"
                        .to_string(),
                ));
            }
            let outcomes = r
                .now
                .claim
                .iter()
                .chain(r.right.claim.iter())
                .chain(r.now.queries.iter().flatten().map(|q| &q.expect))
                .chain(r.right.queries.iter().flatten().map(|q| &q.expect));
            for o in outcomes {
                if !CLAIM_OUTCOMES.contains(&o.as_str()) {
                    out.push(at(format!(
                        "claim outcome {o:?} is not one of {CLAIM_OUTCOMES:?}"
                    )));
                }
            }
            // The rules `refuted by` names (schema /14, M1f S1f.5.15).
            if r.now.refuted_by.is_some() != r.right.refuted_by.is_some() {
                out.push(at(
                    "[now] records what `refuted by` names exactly when [right] states it"
                        .to_string(),
                ));
            }
            // A query's sentences (schema /15, M1f S1f.5.5).
            if r.now.goal_text.is_some() != r.right.goal_text.is_some() {
                out.push(at(
                    "[now] records a query's sentences exactly when [right] states them"
                        .to_string(),
                ));
            }
            // The lines printed (schema /17, M1f S1f.5.6): exactly when
            // [right] states them, label for label, each read where it can be.
            if r.now.printed.is_some() != r.right.printed.is_some() {
                out.push(at(
                    "[now] records the lines printed exactly when [right] states them".to_string(),
                ));
            }
            let labels = |p: &Option<Vec<Printed>>| {
                p.iter()
                    .flatten()
                    .map(|x| (x.file.clone(), x.label.clone()))
                    .collect::<Vec<_>>()
            };
            if labels(&r.now.printed) != labels(&r.right.printed) {
                out.push(at(
                    "[now] and [right] print the same labels in the same files".to_string(),
                ));
            }
            for p in r.now.printed.iter().chain(r.right.printed.iter()).flatten() {
                if !PRINTED_FILES.contains(&p.file.as_str()) {
                    out.push(at(format!(
                        "printed file {:?} is not one of {PRINTED_FILES:?}",
                        p.file
                    )));
                }
                if p.label.is_empty() || !p.line.trim_start().starts_with(&p.label) {
                    out.push(at(format!(
                        "printed line {:?} does not begin with its label {:?}",
                        p.line, p.label
                    )));
                }
            }
            if r.right.refuted_by.is_some() && r.right.core.is_none() {
                out.push(at(
                    "[right] states what `refuted by` names beside its core".to_string()
                ));
            }
            // The members met and not listed (schema /11, M1e2 S1e2.2.26).
            if r.now.others.is_some() != r.right.others.is_some() {
                out.push(at(
                    "[now] records the members it does not list exactly when [right] states them"
                        .to_string(),
                ));
            }
            if r.right.refused && (r.right.verdict.is_some() || r.right.k.is_some()) {
                out.push(at("a refusal states no verdict and no k".to_string()));
            }
            if r.now.refused.is_some()
                && (r.now.k != 0
                    || r.now.exhausted
                    || r.now.empty.is_some()
                    || !r.now.models.is_empty()
                    || !r.now.relations.is_empty()
                    || r.now.core.is_some()
                    || r.now.refuted_by.is_some()
                    || r.now.goal_text.is_some()
                    || r.now.printed.is_some()
                    || r.now.claim.is_some()
                    || r.now.queries.is_some())
            {
                out.push(at("a refused [now] lists nothing".to_string()));
            }
            // A command's refusal is its own (schema /16): the library answers
            // the witness, whose region the row states.
            if r.now.refused.is_some() && r.region.is_some() && !r.now.is_command() {
                out.push(at(
                    "a refused [now] has no [case.region]: the library refuses it too".to_string(),
                ));
            }
            if r.now.error.is_some() {
                if r.now.refused.is_some() {
                    out.push(at("[now] is a refusal or an error, not both".to_string()));
                }
                if r.now.k != 0
                    || r.now.exhausted
                    || r.now.empty.is_some()
                    || !r.now.models.is_empty()
                    || !r.now.relations.is_empty()
                {
                    out.push(at("an error [now] lists nothing".to_string()));
                }
                if r.region.is_some() && !r.now.is_command() {
                    out.push(at(
                        "an error [now] has no [case.region]: the library stops too".to_string(),
                    ));
                }
            }
            if (r.status == "open") != r.open_because.is_some() {
                out.push(at(
                    "`open_because` is stated exactly when the row is open".to_string()
                ));
            }
            // A command's run (schema /16) writes no summary: its `now` is a
            // refusal, an error or that it answered, and states nothing a
            // summary would.
            if r.now.is_command() {
                let ends = [
                    r.now.refused.is_some(),
                    r.now.error.is_some(),
                    r.now.answered,
                ]
                .into_iter()
                .filter(|b| *b)
                .count();
                if ends != 1 {
                    out.push(at(format!(
                        "[now] run {:?} is a command's (schema /16): its answer is exactly one of \
                         a refusal, an error or `answered`",
                        r.now.run
                    )));
                }
                if !r.now.line.is_empty()
                    || r.now.k != 0
                    || r.now.exhausted
                    || r.now.empty.is_some()
                    || !r.now.models.is_empty()
                    || !r.now.relations.is_empty()
                    || !r.now.outside.is_empty()
                    || r.now.core.is_some()
                    || r.now.nogoods.is_some()
                    || r.now.nogoods_listed
                    || r.now.core_status.is_some()
                    || r.now.stale.is_some()
                    || r.now.others.is_some()
                    || r.now.claim.is_some()
                    || r.now.queries.is_some()
                    || r.now.refuted_by.is_some()
                    || r.now.goal_text.is_some()
                    || r.now.printed.is_some()
                {
                    out.push(at(
                        "a command's [now] (schema /16) writes no summary, and states nothing one \
                         would"
                            .to_string(),
                    ));
                }
                if !r.right.refused && !r.right.answered {
                    out.push(at(
                        "a command's [right] (schema /16) is a refusal or `answered`".to_string(),
                    ));
                }
            } else if r.now.answered || r.right.answered {
                out.push(at(
                    "`answered` is a command's (schema /16): a solve's answer is its summary"
                        .to_string(),
                ));
            }
            if r.right.answered
                && (r.right.refused || r.right.verdict.is_some() || r.right.k.is_some())
            {
                out.push(at(
                    "an `answered` [right] states no refusal, verdict or k".to_string()
                ));
            }
            if !recorded(r.now.core.is_some(), r.right.core.is_some()) {
                out.push(at(
                    "[now] records a core exactly when [right] states one, or beside a refusal"
                        .to_string(),
                ));
            }
            if let Some(c) = &r.now.core
                && c.windows(2).any(|w| w[0] > w[1])
            {
                out.push(at("[now] core is sorted".to_string()));
            }
            // The core's status (schema /9, M1e2 S1e2.2.23).
            if r.now.core_status.is_some() != r.right.core_status.is_some() {
                out.push(at(
                    "[now] records a core's status exactly when [right] states one".to_string(),
                ));
            }
            if r.right.core_status.is_some() && r.right.core.is_none() {
                out.push(at(
                    "[right] states a core's status beside its core".to_string()
                ));
            }
            for s in [&r.now.core_status, &r.right.core_status]
                .into_iter()
                .flatten()
            {
                if !is_core_status(s) {
                    out.push(at(format!(
                        "core status {s:?} is not none, within-budget or cut-<caps>"
                    )));
                }
            }
            // Stale members of R (schema /10, M1e2 S1e2.2.25).
            if r.now.stale.is_some() != r.right.stale.is_some() {
                out.push(at(
                    "[now] records stale members of R exactly when [right] states them".to_string(),
                ));
            }
            if r.now.nogoods_listed && r.now.nogoods.as_ref().is_none_or(Vec::is_empty) {
                out.push(at(
                    "[now] nogoods_listed compares N on the sets it lists, and lists none"
                        .to_string(),
                ));
            }
            // N's cores (schema /8, M1e2 S1e2.2.19).
            if r.now.nogoods.is_some() != r.right.nogoods.is_some() {
                out.push(at(
                    "[now] records N's cores exactly when [right] states them".to_string(),
                ));
            }
            if let Some(n) = &r.now.nogoods
                && sorted_nogoods(n) != *n
            {
                out.push(at(
                    "[now] nogoods are sorted: each set and core, and the list by set".to_string(),
                ));
            }
            if !r.now.is_command() && !LINES.contains(&r.now.line.as_str()) {
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
                && (r.status != "open"
                    || !(is_followup(&r.owner) || is_other_milestone_stage(&r.owner)))
            {
                out.push(at(format!(
                    "a `deferred` review is an open row owned by the followup it waits on \
                     (F<n>) or by another milestone's stage (S1f.10.6); this one is {:?}, \
                     owned by {:?}",
                    r.status, r.owner
                )));
            }
            for (label, a) in r.answers() {
                let at = |m: String| format!("{} [{label}]: {m}", r.id);
                let command = a.by.as_deref() == Some("command");
                if command == a.run.starts_with("solve") {
                    out.push(at(format!(
                        "run {:?}: a recorded run is a solve, or a command's that is not one \
                         with `by = \"command\"` (schema /16)",
                        a.run
                    )));
                }
                if command && a.error.is_none() && a.refused.is_none() {
                    out.push(at(
                        "a command's recorded answer (schema /16) is its refusal or its error"
                            .to_string(),
                    ));
                }
                if a.refused.is_some() {
                    if !command || a.error.is_some() {
                        out.push(at(
                            "a recorded refusal is a command's (schema /16), and not an error too"
                                .to_string(),
                        ));
                    }
                    if a.exit != 1 || !a.verdict.is_empty() || a.k != 0 || a.exhausted {
                        out.push(at(
                            "a recorded refusal exits 1 and states no verdict or k".to_string()
                        ));
                    }
                } else if a.error.is_some() {
                    // A run that ended in an error before it answered (M1e2
                    // S1e2.2.18).
                    if a.exit != 1
                        || !a.verdict.is_empty()
                        || a.k != 0
                        || a.exhausted
                        || a.models.is_some()
                        || a.open.is_some()
                        || a.core.is_some()
                    {
                        out.push(at(
                            "a run that ends in an error exits 1 and states no verdict, k, \
                             model or core"
                                .to_string(),
                        ));
                    }
                } else if !VERDICTS.contains(&a.verdict.as_str()) {
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
                    Some("region" | "command") if a.why.is_none() => out.push(at(
                        "an answer a region or a command's run gave says how it was taken (`why`)"
                            .to_string(),
                    )),
                    Some("region" | "command") => {}
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
                // A refused row states no region (M1e2 S1e2.2.5), nor one
                // whose run ends in an error (S1e2.2.18): checked above. A
                // command's row states its witness's (schema /16).
                None if (r.now.refused.is_some() || r.now.error.is_some())
                    && !r.now.is_command() => {}
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
                    if let Some(w) = &g.stopped {
                        if !STOPS.contains(&w.as_str()) {
                            out.push(format!(
                                "{} [region]: stopped {w:?} is not one of {STOPS:?}",
                                r.id
                            ));
                        }
                        if !g.maximal.is_empty() {
                            out.push(format!(
                                "{} [region]: a stopped region's maximal line is not known",
                                r.id
                            ));
                        }
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

    /// **A core's status is a word the engine writes** (schema /9, M1e2
    /// S1e2.2.23): `none`, `within-budget`, or `cut-` and the caps in
    /// `explain::Cut`'s order.
    #[test]
    fn a_core_status_is_a_word_the_engine_writes() {
        for ok in [
            "none",
            "within-budget",
            "cut-environments",
            "cut-rounds",
            "cut-facts",
            "cut-environments-rounds",
            "cut-environments-rounds-facts",
        ] {
            assert!(is_core_status(ok), "{ok}");
        }
        for bad in [
            "cut",
            "cut-",
            "cut-rounds-environments",
            "cut-environments-environments",
            "within",
            "",
        ] {
            assert!(!is_core_status(bad), "{bad}");
        }
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
                // An answer under a permuted interner, which a run of the
                // binary cannot be asked for: `id_order_invariance` holds
                // that half (M1e2 S1e2.2.12, `W11/fork-delta-order`).
                (
                    r.wrong.id_order.is_some(),
                    "an answer under a permuted interner",
                ),
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
        // Another milestone's stage may own a deferred row (schema /10, M1e2
        // S1e2.2.25: W26's, S1f.10.6); a stage of M1e2's may not.
        assert!(is_other_milestone_stage("S1f.10.6"));
        for bad in ["S1e2.2.6", "S1f.10", "F21", "S1f.10.x", "S.1.2"] {
            assert!(!is_other_milestone_stage(bad), "{bad}");
        }
    }

    /// A row as it stood before M1f S1f.5.5's swap (T1f.5.5.4) restated it,
    /// for the schema tests whose live example the swap turned into a refusal:
    /// W28's claims and classes, and W29's core and query beside a refusal,
    /// each recorded at `977829bf` with its long strings cut; and W37's open
    /// `now`, recorded at `410a3388`, before M1f S1f.5.4's T1f.5.4.5 fixed it.
    fn as_it_stood(row: &str) -> Row {
        let reg: Register = toml::from_str(&format!("schema = \"{REGISTER_SCHEMA}\"\n{row}"))
            .expect("the row as it stood parses");
        reg.case.into_iter().next().expect("one row")
    }

    /// `W28/claim-int-name-solve` as it stood at `977829bf`, before S1f.5.5's swap restated it.
    const W28_CLAIM_SOLVE: &str = r##"
[[case]]
id       = "W28/claim-int-name-solve"
case     = "W28"
title    = "a claim and a query's projection classes compare printed text, which printed …"
class    = "claim"
settings = "default"
owner    = "S1e2.3.8"
question = ["Q-M1e2.20"]
witness  = "examples/ein-bugs/wrong-answers/claim-int-name-solve.ein"
status   = "fixed"
look_at  = ["answer", "summary"]
note     = "Filed by S1e2.3.8 (T1e2.3.8.2, 2026-11-06), beside W28/claim-int-name: the same …"

[case.fixed]
commit = "712f23fd"
by     = "S1e2.3.8 (T1e2.3.8.3): the one printer, a value as Ein text the loader reads back to the same value, so a claim and a query's classes, which compare its text, compare values (Q-M1e2.20 (a))"
date   = "2026-11-06"

[case.wrong]
run       = "solve"
by        = "region"
why       = "`ein solve FILE` and `ein test -v FILE`, release and debug, at 86227f9a …"
verdict   = "Solution"
k         = 1
exhausted = true
claim     = "held"

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
derived   = "Q-M1e2.20 (a), the user's ruling of 2026-11-06: a claim compares values. R is …"
verdict   = "Solution"
k         = 1
exhausted = true
claim     = "failed"

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 1
models    = [
  [],
]
claim     = "failed"
"##;

    /// `W28/classes-int-name` as it stood at `977829bf`, before S1f.5.5's swap restated it.
    const W28_CLASSES: &str = r##"
[[case]]
id       = "W28/classes-int-name"
case     = "W28"
title    = "a claim and a query's projection classes compare printed text, which printed …"
class    = "claim"
settings = "default"
owner    = "S1e2.3.8"
question = ["Q-M1e2.20"]
witness  = "examples/ein-bugs/wrong-answers/classes-int-name.ein"
status   = "fixed"
look_at  = ["answer", "summary"]
note     = "Filed by S1e2.3.8 (T1e2.3.8.2, 2026-11-06), beside W28's claims: the same text …"

[case.fixed]
commit = "712f23fd"
by     = "S1e2.3.8 (T1e2.3.8.3): the one printer, a value as Ein text the loader reads back to the same value, so a claim and a query's classes, which compare its text, compare values (Q-M1e2.20 (a))"
date   = "2026-11-06"

[case.wrong]
run       = "solve"
by        = "region"
why       = "`ein solve FILE`, release and debug, at 86227f9a (2026-11-06): R the eight …"
verdict   = "Ambiguity"
k         = 8
exhausted = true
queries   = [{ classes = 2, expect = "none" }]

[case.region]
route    = "state route"
monotone = true
k_s      = 8
k_c      = 8
minimal  = [[]]
maximal  = [["(pick \"1\")", "(pick A)", "(pick B)"]]

[case.right]
derived   = "Q-M1e2.20 (a), the user's ruling of 2026-11-06: classes are keyed by value. A …"
verdict   = "Ambiguity"
k         = 8
exhausted = true
queries   = [{ classes = 4, expect = "none" }]

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 8
relations = ["pick"]
models    = [
  [],
  ["(pick \"1\")"],
  ["(pick \"1\")", "(pick A)"],
  ["(pick \"1\")", "(pick A)", "(pick B)"],
  ["(pick \"1\")", "(pick B)"],
  ["(pick A)"],
  ["(pick A)", "(pick B)"],
  ["(pick B)"],
]
queries   = [{ classes = 4, expect = "none" }]
"##;

    /// `W29/core-two-ways` as it stood at `977829bf`, before S1f.5.5's swap restated it.
    const W29_CORE: &str = r##"
[[case]]
id       = "W29/core-two-ways"
case     = "W29"
title    = "a bare `_` loads in silence: dropped from a fact or an activator, and read in a …"
class    = "core"
settings = "default"
owner    = "S1f.5.1"
question = ["Q-M1e2.22"]
witness  = "examples/ein-bugs/wrong-answers/underscore-core-two-ways.ein"
status   = "open"
open_because = "A bare `_` in a rule's pattern matches any value and binds nothing (W20's …"
look_at  = ["refuted", "summary"]
note     = "Filed by S1f.5.0 (T1f.5.0.2, 2026-11-08) from P1f.5's F3, S1f.5.1's case, the …"

[case.wrong]
run       = "solve"
by        = "region"
why       = "`ein solve` on the witness, release and debug, at ec004a48 (2026-11-08; the …"
verdict   = "Contradiction"
k         = 0
exhausted = true
core      = ["(big b0)", "(big2 b0)", "(big3 b0)", "(c x)"]

[[case.twin]]
witness   = "examples/ein-bugs/wrong-answers/underscore-core-two-ways-twin.ein"
run       = "solve"
by        = "region"
why       = "the slot written `?w0`, a variable the rule reads nowhere else, at ec004a48 …"
verdict   = "Contradiction"
k         = 0
exhausted = true
core      = ["(c x)", "(small s0)"]

[case.region]
route    = "state route"
monotone = true
k_s      = 0
k_c      = 0
empty    = "no-alive"
minimal  = []
maximal  = []

[case.right]
reading   = "Q-M1e2.22, the user's ruling of 2026-11-06: a bare `_` is reserved and refused, …"
refused   = true
derived   = "A bare `_` names nothing (Q-M1e2.22), so the rule is refused at load, with a …"

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 0
empty     = "dead"
relations = ["big", "big2", "big3", "c", "hit", "holds", "small"]
models    = []
core      = ["(big b0)", "(big2 b0)", "(big3 b0)", "(c x)"]
"##;

    /// `W29/goal-arg` as it stood at `977829bf`, before S1f.5.5's swap restated it.
    const W29_GOAL: &str = r##"
[[case]]
id       = "W29/goal-arg"
case     = "W29"
title    = "a bare `_` loads in silence: dropped from a fact or an activator, and read in a …"
class    = "claim"
settings = "default"
owner    = "S1f.5.1"
question = ["Q-M1e2.22"]
witness  = "examples/ein-bugs/wrong-answers/underscore-goal-arg.ein"
status   = "open"
open_because = "A query's goal is compiled as a rule's pattern, where a bare `_` matches any …"
look_at  = ["answer", "summary"]
note     = "Filed by S1f.5.0 (T1f.5.0.2, 2026-11-08) from S1f.5.1's case (the goal row), on …"

[case.wrong]
run       = "solve"
by        = "region"
why       = "`ein solve` on the witness, release and debug, at ec004a48 (2026-11-08; the …"
verdict   = "Solution"
k         = 1
exhausted = true
queries   = [{ classes = 1, expect = "none" }]

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
reading   = "Q-M1e2.22, the user's ruling of 2026-11-06: a bare `_` is reserved and refused, …"
refused   = true
derived   = "A bare `_` names nothing, in a goal as anywhere else (Q-M1e2.22), so the query …"

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 1
relations = []
models    = [
  [],
]
queries   = [{ classes = 1, expect = "none" }]
"##;

    /// **A run that ends in an error**, schema /7 (M1e2 S1e2.2.18):
    /// `W18/str-in-assert`'s `wrong` ended in one before it answered, exit 1,
    /// and states no verdict. Such a `now` lists nothing, states no region, is
    /// not also a refusal, and is never `right`, so a fixed row whose `now`
    /// ends in an error is a defect. The row's `right` is a refusal since M1f
    /// S1f.5.5's swap (S1f.5.2's ruling: a string is no value), so the error is
    /// held against a refusal; it was held against an answer until then.
    #[test]
    fn a_run_that_ends_in_an_error_is_no_answer() {
        let row = register()
            .case
            .into_iter()
            .find(|r| r.wrong.error.is_some())
            .expect("W18/str-in-assert's wrong ended in an error");
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let stated = "a run that ends in an error exits 1 and states no verdict";
        let mut r = row.clone();
        r.wrong.verdict = "Solution".to_string();
        assert!(defects(&r).iter().any(|d| d.contains(stated)));
        let mut r = row.clone();
        r.wrong.exit = 0;
        assert!(defects(&r).iter().any(|d| d.contains(stated)));

        // A `now` that ends in an error: never right, lists nothing, states
        // no region, and is not also a refusal.
        let mut r = row.clone();
        r.now = Now {
            run: "solve --minimal".to_string(),
            line: "minimal".to_string(),
            container: false,
            answered: false,
            refused: None,
            error: Some("a line".to_string()),
            exhausted: false,
            k: 0,
            empty: None,
            relations: Vec::new(),
            models: Vec::new(),
            outside: Vec::new(),
            core: None,
            nogoods: None,
            core_status: None,
            stale: None,
            nogoods_listed: false,
            others: None,
            claim: None,
            queries: None,
            refuted_by: None,
            goal_text: None,
            printed: None,
        };
        r.region = None;
        assert_eq!(r.now.against(&r.right), ["right: a refusal, now: an error"]);
        r.status = "fixed".to_string();
        r.open_because = None;
        r.fixed = Some(Fixed {
            commit: "0000000".to_string(),
            by: "a stage".to_string(),
            date: "2026-11-04".to_string(),
        });
        assert!(defects(&r).iter().any(|d| {
            d.contains("fixed, and [now] is not [right]: right: a refusal, now: an error")
        }));
        let mut s = r.clone();
        s.now.k = 1;
        assert!(
            defects(&s)
                .iter()
                .any(|d| d.contains("an error [now] lists nothing"))
        );
        let mut s = r.clone();
        s.now.refused = Some("a line".to_string());
        assert!(
            defects(&s)
                .iter()
                .any(|d| d.contains("a refusal or an error, not both"))
        );
    }

    /// **N's cores**, schema /8 (M1e2 S1e2.2.19, W19): a `now` records them
    /// exactly where `right` states them, sorted, and a fixed row's must be
    /// `right`'s, set for set.
    #[test]
    fn a_dead_nodes_core_is_compared_set_for_set() {
        let row = register()
            .case
            .into_iter()
            .find(|r| r.now.nogoods.is_some())
            .expect("W19's rows record N's cores");
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let mut r = row.clone();
        r.right.nogoods = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("exactly when [right] states them"))
        );
        let mut r = row.clone();
        r.now.nogoods = Some(vec![Nogood {
            set: vec!["(z a)".to_string(), "(a a)".to_string()],
            core: Vec::new(),
        }]);
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] nogoods are sorted"))
        );
        // The comparison: the right core against the run's, set for set.
        let mut r = row.clone();
        let right = r.right.nogoods.clone().expect("stated");
        r.now.nogoods = Some(right.clone());
        assert!(r.now.against(&r.right).is_empty());
        let mut other = right;
        other[0].core = vec!["(elsewhere)".to_string()];
        r.now.nogoods = Some(other);
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("nogoods: "))
        );
    }

    /// **Schema /10** (M1e2 S1e2.2.25): a stale count is compared where
    /// `right` states one, a stopped region states no maximal line, and a
    /// `now` that compares N on the sets it lists lists some.
    #[test]
    fn a_stale_count_a_stopped_region_and_a_listed_n() {
        let reg = register();
        let row = reg
            .case
            .iter()
            .find(|r| r.now.stale.is_some())
            .expect("W8's rows past the limit record a stale count")
            .clone();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        // The count is compared with right's.
        let mut r = row.clone();
        r.right.stale = r.now.stale.as_ref().map(|s| s.in_r);
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .all(|d| !d.starts_with("stale"))
        );
        r.right.stale = Some(0);
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("stale"))
        );
        // Stated on one side only is a defect.
        let mut r = row.clone();
        r.right.stale = None;
        assert!(defects(&r).iter().any(|d| d.contains("stale members of R")));
        // A stopped region's maximal line is not known, and its word is one
        // of STOPS.
        let mut r = row.clone();
        let g = r.region.as_mut().expect("a stopped region");
        assert_eq!(g.stopped.as_deref(), Some("limit"));
        g.maximal = vec![Vec::new()];
        assert!(defects(&r).iter().any(|d| d.contains("not known")));
        let g = r.region.as_mut().expect("a stopped region");
        g.maximal = Vec::new();
        g.stopped = Some("tired".to_string());
        assert!(defects(&r).iter().any(|d| d.contains("is not one of")));
        // A listed N lists some members.
        let listed = reg
            .case
            .iter()
            .find(|r| r.now.nogoods_listed)
            .expect("W10/second-preparation compares N on one member")
            .clone();
        assert!(defects(&listed).is_empty(), "{:?}", defects(&listed));
        let mut r = listed;
        r.now.nogoods = Some(Vec::new());
        assert!(defects(&r).iter().any(|d| d.contains("nogoods_listed")));
    }

    /// **Schema /11** (M1e2 S1e2.2.26, W27): the members a line mode past the
    /// limit meets and does not list are compared where `right` states them,
    /// and such a right states `k`, `exhausted = false` and no verdict; since
    /// M1f S1f.5.6 a minimal line decided at root meets none, `others = 0`,
    /// and is exhausted.
    #[test]
    fn a_line_past_the_limit_counts_what_it_does_not_list() {
        let reg = register();
        let row = reg
            .case
            .iter()
            .find(|r| r.right.others.is_some_and(|o| o > 0))
            .expect("W27's maximal rows state the members met and not listed")
            .clone();
        let decided = reg
            .case
            .iter()
            .find(|r| r.right.others == Some(0))
            .expect("W27's minimal rows are decided at root (M1f S1f.5.6)")
            .clone();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        // The count is compared with right's.
        let mut r = row.clone();
        r.now.others = r.right.others;
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .all(|d| !d.starts_with("members met"))
        );
        r.now.others = r.right.others.map(|o| o + 1);
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("members met"))
        );
        // Stated on one side only is a defect.
        let mut r = row.clone();
        r.now.others = None;
        assert!(defects(&r).iter().any(|d| d.contains("does not list")));
        // Such a right makes no verdict and is not exhausted.
        let mut r = row.clone();
        r.right.verdict = Some("Solution".to_string());
        assert!(defects(&r).iter().any(|d| d.contains("no verdict")));
        let mut r = row;
        r.right.exhausted = Some(true);
        assert!(defects(&r).iter().any(|d| d.contains("no verdict")));
        // A line decided at root meets none, and is exhausted (M1f S1f.5.6);
        // it still makes no verdict.
        assert!(defects(&decided).is_empty(), "{:?}", defects(&decided));
        assert_eq!(decided.right.exhausted, Some(true));
        let mut r = decided;
        r.right.verdict = Some("Solution".to_string());
        assert!(defects(&r).iter().any(|d| d.contains("no verdict")));
    }

    /// **Schema /12** (M1e2 S1e2.3.8, W28): a claim's outcome and a query's
    /// classes are compared where `right` states them, recorded on both sides
    /// or neither, in the summary's words; and a run whose claim is `failed`
    /// refutes it, which is an exit 1.
    #[test]
    fn a_claim_and_a_querys_classes_are_answers() {
        let reg = register();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        // No live row states a claim or the classes since M1f S1f.5.5's swap
        // restated W28's to the refusal, so the rows as they stood.
        assert!(
            reg.case
                .iter()
                .all(|r| r.right.claim.is_none() && r.right.queries.is_none()),
            "a live row states a claim or the classes again: use it"
        );
        let solve = as_it_stood(W28_CLAIM_SOLVE);
        let query = as_it_stood(W28_CLASSES);
        for row in [&solve, &query] {
            assert!(defects(row).is_empty(), "{:?}", defects(row));
        }
        // The outcome is compared with right's.
        let mut r = solve.clone();
        r.now.claim = r.right.claim.clone();
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .all(|d| !d.starts_with("claim"))
        );
        r.now.claim = Some("not-checked".to_string());
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("claim"))
        );
        // So is each query's count of classes.
        let mut r = query.clone();
        r.now.queries = r.right.queries.clone();
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .all(|d| !d.starts_with("queries"))
        );
        r.now.queries.as_mut().expect("stated")[0].classes += 1;
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("queries"))
        );
        // Stated on one side only, or in a word the summary has not, is a defect.
        let mut r = query.clone();
        r.now.queries = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] records a claim"))
        );
        let mut r = solve.clone();
        r.right.claim = Some("holds".to_string());
        assert!(defects(&r).iter().any(|d| d.contains("claim outcome")));
        // A failed claim is a refutation: the run exits 1.
        let mut r = solve;
        r.now.claim = Some("failed".to_string());
        assert!(r.now.refutes());
        r.now.claim = Some("held".to_string());
        assert!(!r.now.refutes());
    }

    /// **Schema /13** (M1f S1f.5.0, W29 and W32): beside a `right` that is a
    /// refusal, `now` may record the core and the claims today's answer
    /// reports, which the refusal replaces; anywhere else a core or a claim is
    /// recorded exactly where `right` states one, and a refused `now` records
    /// none.
    #[test]
    fn beside_a_refusal_now_records_what_today_reports() {
        let reg = register();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        // No live row records an answer beside a refusal since M1f S1f.5.5's
        // swap fixed W29's and W32's, each `now` the parse refusal, so the rows
        // as they stood.
        assert!(
            reg.case
                .iter()
                .all(|r| !(r.right.refused && (r.now.core.is_some() || r.now.queries.is_some()))),
            "a live row records an answer beside a refusal: use it"
        );
        let core = as_it_stood(W29_CORE);
        let query = as_it_stood(W29_GOAL);
        for row in [&core, &query] {
            assert!(defects(row).is_empty(), "{:?}", defects(row));
            // An answer, where `right` is a refusal: open, by the method.
            assert!(!row.now.against(&row.right).is_empty());
        }
        // Beside a right that is an answer, the same `now` is a defect.
        let mut r = core.clone();
        r.right.refused = false;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] records a core"))
        );
        let mut r = query.clone();
        r.right.refused = false;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] records a claim"))
        );
        // A refused `now` records neither.
        let mut r = core;
        r.now.refused = Some("kb load error: …".to_string());
        r.now.k = 0;
        r.now.exhausted = false;
        r.now.empty = None;
        r.now.models.clear();
        r.now.relations.clear();
        r.region = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("a refused [now] lists nothing"))
        );
        r.now.core = None;
        assert!(defects(&r).is_empty(), "{:?}", defects(&r));
    }

    /// **What `refuted by` names is an answer**, schema /14 (M1f S1f.5.15,
    /// W36): the rules root's row names, recorded exactly where `right`
    /// states them and compared in the order printed. W36's `now` names
    /// `forced, occupied` beside the core whose right names `no-room`, so it
    /// is open by the method; the same `now` naming `no-room` answers it.
    #[test]
    fn what_refuted_by_names_is_an_answer() {
        let reg = register();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        let row = reg
            .case
            .iter()
            .find(|r| r.right.refuted_by.is_some())
            .expect("W36 states what refuted by names")
            .clone();
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let differs = row.now.against(&row.right);
        assert!(
            differs.len() == 1 && differs[0].starts_with("refuted by: "),
            "{differs:?}"
        );
        let mut r = row.clone();
        r.now.refuted_by = r.right.refuted_by.clone();
        assert!(r.now.against(&r.right).is_empty());
        // The order is the read-out's, so a permutation is another answer.
        if let Some(v) = r.now.refuted_by.as_mut() {
            v.push("another".to_string());
            v.reverse();
        }
        assert!(!r.now.against(&r.right).is_empty());
        // Recorded exactly where `right` states it, and beside a core.
        let mut r = row.clone();
        r.right.refuted_by = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] records what `refuted by` names"))
        );
        let mut r = row;
        r.right.core = None;
        r.now.core = None;
        assert!(defects(&r).iter().any(|d| d.contains("beside its core")));
    }

    /// `W37/template-var-star` as it stood at `410a3388`, before M1f S1f.5.4's
    /// T1f.5.4.5 fixed it: `open`, its `now` the template as written.
    const W37_OPEN: &str = r##"
[[case]]
id       = "W37/template-var-star"
case     = "W37"
title    = "a template cannot name a variable whose name holds a character other than …"
class    = "read-out"
settings = "default"
owner    = "S1f.5.4"
question = ["S1f.5.5 T9 review item 7"]
witness  = "examples/ein-bugs/wrong-answers/template-var-star.ein"
status   = "open"
open_because = "`ein_core::why::render_why` reads a reference as `{?`, one letter or digit, …"
look_at  = ["answer", "trace"]
note     = "Filed by S1f.5.5 (T1f.5.5.10, 2026-11-08) from its T9 review's § 11, …"

[case.wrong]
run       = "solve"
by        = "region"
why       = "`ein solve` on the witness, release, at a50cf081 (2026-11-08): …"
verdict   = "Solution"
k         = 1
exhausted = true
closes    = ["q"]
models    = [["(q a)"]]
goal_text = ["{?n*} is a q"]

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
reading   = "a template reference names any variable the program can write (Q31's `{?var}` notation: …"
verdict   = "Solution"
k         = 1
exhausted = true
closes    = ["q"]
models    = [["(q a)"]]
goal_text = ["a is a q"]
derived   = "`reach` fires on `(p a)` with `?n*` bound to `a`, …"

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 1
relations = []
models    = [
  [],
]
goal_text = ["{?n*} is a q"]
"##;

    /// **A line a run prints is an answer**, schema /17 (M1f S1f.5.6, W42):
    /// recorded exactly where `right` states lines, the same labels in the
    /// same files, each file one the test reads and each line beginning with
    /// its label; compared whole, so a label glued to its value is another
    /// answer than the label and the value apart.
    #[test]
    fn a_line_a_run_prints_is_an_answer() {
        let reg = register();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        let rows: Vec<&Row> = reg
            .case
            .iter()
            .filter(|r| r.right.printed.is_some())
            .collect();
        assert!(rows.len() >= 2, "W42's rows state the lines printed");
        for row in rows {
            assert!(defects(row).is_empty(), "{:?}", defects(row));
            let mut r = row.clone();
            r.now.printed = r.right.printed.clone();
            assert!(
                r.now
                    .against(&r.right)
                    .iter()
                    .all(|d| !d.starts_with("printed")),
                "{}",
                r.id
            );
            // Glue the label to its value: another answer.
            if let Some(p) = r.now.printed.as_mut().and_then(|v| v.first_mut()) {
                p.line = p.line.replacen(&format!("{} ", p.label), &p.label, 1);
            }
            assert!(
                r.now
                    .against(&r.right)
                    .iter()
                    .any(|d| d.starts_with("printed")),
                "{}",
                r.id
            );
            // Recorded exactly where `right` states it, label for label.
            let mut r = row.clone();
            r.right.printed = None;
            assert!(defects(&r).iter().any(|d| d.contains("the lines printed")));
            let mut r = row.clone();
            if let Some(p) = r.now.printed.as_mut().and_then(|v| v.first_mut()) {
                p.label = "another".to_string();
            }
            assert!(defects(&r).iter().any(|d| d.contains("the same labels")));
            let mut r = row.clone();
            if let Some(p) = r.right.printed.as_mut().and_then(|v| v.first_mut()) {
                p.file = "answer.json".to_string();
            }
            assert!(defects(&r).iter().any(|d| d.contains("is not one of")));
        }
    }

    /// **What a query's `:goal-text` prints is an answer**, schema /15 (M1f
    /// S1f.5.5, W37): the read-out's sentences, recorded exactly where `right`
    /// states them and compared in the order printed. W37's `now` printed the
    /// template as written where its right fills the reference, so it was open
    /// by the method (the row as it stood at `410a3388`); the same `now`
    /// printing the filled sentence answers it, as the live row does since M1f
    /// S1f.5.4's T1f.5.4.5.
    #[test]
    fn what_a_goal_text_prints_is_an_answer() {
        let reg = register();
        let defects = |r: &Row| {
            Register {
                schema: REGISTER_SCHEMA.to_string(),
                case: vec![r.clone()],
            }
            .defects()
        };
        let live = reg
            .case
            .iter()
            .find(|r| r.right.goal_text.is_some())
            .expect("W37 states what its goal text prints");
        assert!(defects(live).is_empty(), "{:?}", defects(live));
        assert!(live.now.against(&live.right).is_empty(), "W37 is fixed");
        let row = as_it_stood(W37_OPEN);
        assert!(defects(&row).is_empty(), "{:?}", defects(&row));
        let differs = row.now.against(&row.right);
        assert!(
            differs.len() == 1 && differs[0].starts_with("goal text: "),
            "{differs:?}"
        );
        let mut r = row.clone();
        r.now.goal_text = r.right.goal_text.clone();
        assert!(r.now.against(&r.right).is_empty());
        // The order is the read-out's, so a permutation is another answer.
        if let Some(v) = r.now.goal_text.as_mut() {
            v.push("another".to_string());
            v.reverse();
        }
        assert!(!r.now.against(&r.right).is_empty());
        // Recorded exactly where `right` states it.
        let mut r = row.clone();
        r.right.goal_text = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("[now] records a query's sentences"))
        );
        // A refused `now` prints no sentence.
        let mut r = row;
        r.now.refused = Some("a line".to_string());
        r.now.k = 0;
        r.now.exhausted = false;
        r.now.models = Vec::new();
        r.region = None;
        assert!(
            defects(&r)
                .iter()
                .any(|d| d.contains("a refused [now] lists nothing"))
        );
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
