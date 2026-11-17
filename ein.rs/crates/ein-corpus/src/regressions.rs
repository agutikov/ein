//! The regressions — `corpus/regressions.toml` — and the format of any file of
//! programs held to a recorded answer.
//!
//! One row per program and run that was once answered wrong: the answer the
//! engine gives today, `[case.now]`, and the answer the definitions give,
//! `[case.right]`, as data, so a run that stops giving the right answer fails
//! the gate. The field reference is the file's own header. This module reads
//! a file of the format and holds the invariants that need no engine run;
//! `ein-cli/tests/held/` runs a row's `[case.now]` through the binary and
//! compares what it answers, and recomputes a row's `[case.region]` with the
//! library; and the half of a row that states `id_order` is
//! `ein-render/tests/id_order_invariance.rs`'s.
//!
//! **One format, two readings** ([`Held`]). The project's file holds every
//! row to its right answer: `[case.now]` answers `[case.right]` on every field
//! `[case.right]` states ([`Regressions::not_right`] is empty). A file of
//! answers known to be wrong uses the same rows the other way: no row's
//! `[case.now]` answers its `[case.right]` by the method
//! ([`Regressions::right_by_the_method`] is empty), so that a run that starts
//! answering right is news. `ein-cli`'s example `held_answers` reads either,
//! given its path.
//!
//! **Not `:expect`.** An expectation is a claim about the program's answer,
//! and it can say what the answer is but not what it was: a row says both,
//! and it can say what `:expect` cannot, a core, and the lines a command
//! prints. (An answer whose goal extent is empty a claim states as any other,
//! Q-M1e.13.)

use serde::Deserialize;
use std::path::Path;

/// The project's file, repo-root-relative: beside `corpus.toml`, because its
/// witnesses are corpus programs.
pub const REGRESSIONS: &str = "corpus/regressions.toml";

/// The format's schema, `ein-regressions/N`.
pub const SCHEMA: &str = "ein-regressions/1";

/// What a file's rows are held to, beside what their runs answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    /// Every row's `[case.now]` answers its `[case.right]`: regressions.
    Right,
    /// No row's `[case.now]` answers its `[case.right]` by the method: known
    /// wrong answers, each still answered wrong.
    Open,
}

/// A file of rows.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Regressions {
    pub schema: String,
    #[serde(default)]
    pub case: Vec<Row>,
}

/// One program, one run, and what it answers.
#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    /// The row's name, stable, unique in its file.
    pub id: String,
    /// The wrong answer the row guards against.
    pub title: String,
    /// `default`, or what the run needs beside the witness: a lever
    /// (`lever: …`), a bound the read-out qualifies.
    pub settings: String,
    /// The decisions the right answer rests on: ids, each a file under
    /// `docs/decisions/`.
    #[serde(default)]
    pub decisions: Vec<String>,
    /// The program, repo-root-relative.
    pub witness: String,
    /// Required for a witness under [`OUTSIDE_THE_CORPUS`] in the project's
    /// file: why it cannot be a corpus entry.
    #[serde(default)]
    pub not_in_corpus: Option<String>,
    /// An `id_order_invariance` op and seed under which the answer must not
    /// move.
    #[serde(default)]
    pub id_order: Option<IdOrder>,
    pub right: Right,
    /// The witness's region, as the library computes it; absent where the
    /// run or the right answer is a refusal, or the run ends in an error.
    #[serde(default)]
    pub region: Option<Region>,
    /// The run that asks the row's question, and what it answers.
    pub now: Now,
    /// Keys this format does not read, kept whole: a file other than the
    /// project's may carry its own beside a row's. The project's file carries
    /// none ([`Held::Right`]'s [`Regressions::defects`]).
    #[serde(flatten)]
    pub other: toml::Table,
}

/// A core's status as a row spells it: `none`, `within-budget`, or `cut-`
/// and one or more of the caps, in order — `ein_infer::explain::CoreStatus::word`'s
/// words for a core a search took, and `none`. Not `not-kept`, which no run
/// writes: a row records the summary's `region.core_status`, root's, which a
/// run always takes.
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

/// A decision id, `Q-M<milestone>.<n>`.
fn is_decision_id(q: &str) -> bool {
    let Some(rest) = q.strip_prefix("Q-M") else {
        return false;
    };
    let Some((m, n)) = rest.rsplit_once('.') else {
        return false;
    };
    !m.is_empty()
        && m.bytes().all(|b| b.is_ascii_alphanumeric())
        && !n.is_empty()
        && n.bytes().all(|b| b.is_ascii_digit())
}

impl Regressions {
    /// Read a file of the format.
    pub fn load(path: &Path) -> Result<Regressions, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let file: Regressions =
            toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if file.schema != SCHEMA {
            return Err(format!(
                "{}: schema {:?}, expected {SCHEMA:?}",
                path.display(),
                file.schema
            ));
        }
        Ok(file)
    }

    /// Every row whose `[case.now]` does not answer its `[case.right]`, one
    /// message per field: what [`Held::Right`] holds empty.
    pub fn not_right(&self) -> Vec<String> {
        let mut out = Vec::new();
        for r in &self.case {
            for d in r.now.against(&r.right) {
                out.push(format!("{}: [now] is not [right]: {d}", r.id));
            }
        }
        out
    }

    /// Every row whose `[case.now]` answers its `[case.right]` on every field
    /// the format records, where nothing the run cannot show keeps it open:
    /// what [`Held::Open`] holds empty. Such a row is answered right, by
    /// construction, unless its reading is contested (a `[case.right]` with
    /// alternatives keeps its row and its question). A `[case.right]` that
    /// states what `[case.now]` does not record keeps the row open on that
    /// field, and the row's run shows it, not this comparison: a refusal,
    /// which states no verdict; a qualifier recorded as no `outside`; facts
    /// over a relation no candidate is over, since a state's `(model …)` names
    /// H's relations only; or an answer under a permuted interner, which a
    /// run of the binary cannot be asked for. A core is recorded
    /// ([`Now::core`]), so it is compared, not exempt.
    pub fn right_by_the_method(&self) -> Vec<String> {
        let mut out = Vec::new();
        for r in &self.case {
            let differs = r.now.against(&r.right);
            let beyond_h = r.right.models.is_some()
                && r.right.closes.iter().any(|c| !r.now.relations.contains(c));
            let unrecorded = r.right.verdict.is_none()
                || (r.right.qualifier.is_some() && r.right.outside.is_none())
                || beyond_h
                || r.id_order.is_some();
            let contested = !r.right.alternative.is_empty();
            if differs.is_empty() && !unrecorded && !contested {
                out.push(format!(
                    "{}: [now] answers [right] on every field the format records, and nothing \
                     the run cannot show keeps it open: it is answered right",
                    r.id
                ));
            }
        }
        out
    }

    /// Every way a row can be malformed without running anything, one message
    /// per defect, each naming the row.
    pub fn defects(&self, held: Held) -> Vec<String> {
        let mut out = Vec::new();
        let mut seen = std::collections::BTreeMap::<&str, &Row>::new();
        for r in &self.case {
            let at = |m: String| format!("{}: {m}", r.id);
            if r.id.is_empty() {
                out.push(at("a row has a name".to_string()));
            }
            if let Some(prev) = seen.insert(r.id.as_str(), r) {
                out.push(at(format!("id repeated (also {:?})", prev.witness)));
            }
            for q in &r.decisions {
                if !is_decision_id(q) {
                    out.push(at(format!(
                        "decision {q:?} is not a decision id, Q-M<m>.<n>"
                    )));
                }
            }
            if held == Held::Right {
                if !r.other.is_empty() {
                    let keys: Vec<&String> = r.other.keys().collect();
                    out.push(at(format!("keys the format does not read: {keys:?}")));
                }
                if r.witness.starts_with(OUTSIDE_THE_CORPUS) != r.not_in_corpus.is_some() {
                    out.push(at(format!(
                        "`not_in_corpus` is required exactly for a witness under \
                         {OUTSIDE_THE_CORPUS}"
                    )));
                }
                // A line past the limit makes no verdict: its right states
                // `k` and the members it does not list instead.
                if !r.right.refused
                    && !r.right.answered
                    && r.right.signal.is_none()
                    && (r.right.k.is_none()
                        || (r.right.verdict.is_none() && r.right.others.is_none()))
                {
                    out.push(at("a regression states the right verdict and k".to_string()));
                }
            }
            // A line decided at root meets no member it does not list, and
            // is exhausted: `others = 0`.
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
            // What `now` records beside `right`: a field `right` states, or,
            // where `right` is a refusal, the core and the claims today's
            // answer reports, which the refusal replaces.
            let recorded = |now: bool, right: bool| now == right || (r.right.refused && !right);
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
            if r.now.refuted_by.is_some() != r.right.refuted_by.is_some() {
                out.push(at(
                    "[now] records what `refuted by` names exactly when [right] states it"
                        .to_string(),
                ));
            }
            if r.now.goal_text.is_some() != r.right.goal_text.is_some() {
                out.push(at(
                    "[now] records a query's sentences exactly when [right] states them"
                        .to_string(),
                ));
            }
            // The lines printed: exactly when [right] states them, label for
            // label, each read where it can be.
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
                if !printed_file(&p.file) {
                    out.push(at(format!(
                        "printed file {:?} is not one of {PRINTED_FILES:?} or a state file, \
                         models/NN.ein",
                        p.file
                    )));
                }
                // An empty line says the file has no line at the label.
                if p.label.is_empty()
                    || (!p.line.is_empty() && !p.line.trim_start().starts_with(&p.label))
                {
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
            // A command's refusal is its own, and so is a solve's usage
            // refusal, exit 2: the library answers the witness, whose region
            // the row states.
            if let Some(e) = r.now.exit
                && (e != 2 || r.now.refused.is_none() || r.now.is_command())
            {
                out.push(at(format!(
                    "[now] exit {e}: a solve's refusal exits 1, or 2 where it refuses what the run \
                     was asked to write"
                )));
            }
            if r.now.refused.is_some()
                && r.region.is_some()
                && !r.now.is_command()
                && r.now.exit != Some(2)
            {
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
            // A command's run writes no summary: its `now` is a refusal, an
            // error or that it answered, and states nothing a summary would.
            if r.now.is_command() {
                let ends = [
                    r.now.refused.is_some(),
                    r.now.error.is_some(),
                    r.now.answered,
                    r.now.signal.is_some(),
                    r.now.panicked.is_some(),
                ]
                .into_iter()
                .filter(|b| *b)
                .count();
                if ends != 1 {
                    out.push(at(format!(
                        "[now] run {:?} is a command's: its answer is exactly one of a refusal, \
                         an error or `answered`, or with its stdout closed a signal or a panic",
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
                        "a command's [now] writes no summary, and states nothing one would"
                            .to_string(),
                    ));
                }
                if !r.right.refused && !r.right.answered && r.right.signal.is_none() {
                    out.push(at(
                        "a command's [right] is a refusal or `answered`, or a signal".to_string(),
                    ));
                }
            } else if r.now.answered || r.right.answered {
                out.push(at(
                    "`answered` is a command's: a solve's answer is its summary".to_string(),
                ));
            }
            // A run whose stdout is closed: its answer is how it ended, a
            // signal or a panic, and a right that is a signal is such a run's.
            if let Some(o) = &r.now.stdout
                && o != STDOUT_CLOSED
            {
                out.push(at(format!("[now] stdout {o:?} is not {STDOUT_CLOSED:?}")));
            }
            if r.now.stdout_closed() != (r.now.signal.is_some() || r.now.panicked.is_some()) {
                out.push(at(
                    "[now] ends by a signal or a panic exactly when its stdout is closed"
                        .to_string(),
                ));
            }
            if r.now.stdout_closed() && (r.now.container || r.now.exit.is_some()) {
                out.push(at(
                    "a [now] whose stdout is closed reads the witness and states no exit: a \
                     panic's is 101, and a signal ends a run with none"
                        .to_string(),
                ));
            }
            if let Some(n) = r.now.signal
                && n != SIGPIPE
            {
                out.push(at(format!(
                    "[now] signal {n}: a closed stdout ends a run by SIGPIPE, {SIGPIPE}"
                )));
            }
            if r.right.signal.is_some() != r.now.stdout_closed() {
                out.push(at(
                    "[right] states a signal exactly when [now]'s stdout is closed".to_string(),
                ));
            }
            if let Some(n) = r.right.signal
                && (n != SIGPIPE
                    || r.right.refused
                    || r.right.answered
                    || r.right.verdict.is_some()
                    || r.right.k.is_some())
            {
                out.push(at(format!(
                    "a [right] that is a signal is SIGPIPE's, {SIGPIPE}, and states no refusal, \
                     `answered`, verdict or k; this one is {n}"
                )));
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
            for right in std::iter::once(&r.right).chain(&r.right.alternative) {
                if let Some(v) = &right.verdict
                    && !VERDICTS.contains(&v.as_str())
                {
                    out.push(at(format!(
                        "[right]: verdict {v:?} is not one of {VERDICTS:?}"
                    )));
                }
                if (right.models.is_some() || right.open.is_some()) && right.closes.is_empty() {
                    out.push(at(
                        "[right]: models are compared over `closes`, which is empty".to_string(),
                    ));
                }
                if let (Some(m), Some(k)) = (&right.models, right.k)
                    && m.len() != k
                {
                    out.push(at(format!(
                        "[right]: {} models listed for k = {k}",
                        m.len()
                    )));
                }
            }
            match &r.region {
                // A refused row states no region, nor one whose run ends in
                // an error: checked above. A command's row states its
                // witness's.
                None if (r.now.refused.is_some() || r.now.error.is_some())
                    && !r.now.is_command() => {}
                // A program refused at load has no R, so a row whose right
                // answer is a refusal states none, whatever its `now` runs.
                None if r.right.refused => {}
                None => out.push(at("no [case.region]".to_string())),
                Some(g) => {
                    if !ROUTES.contains(&g.route.as_str()) {
                        out.push(at(format!(
                            "[region]: route {:?} is not one of {ROUTES:?}",
                            g.route
                        )));
                    }
                    let empty = g.k_s == 0;
                    if empty != g.empty.is_some() {
                        out.push(at(
                            "[region]: `empty` is stated exactly when k_s = 0".to_string()
                        ));
                    }
                    if let Some(e) = &g.empty
                        && !EMPTY_KINDS.contains(&e.as_str())
                    {
                        out.push(at(format!(
                            "[region]: empty {e:?} is not one of {EMPTY_KINDS:?}"
                        )));
                    }
                    if let Some(w) = &g.stopped {
                        if !STOPS.contains(&w.as_str()) {
                            out.push(at(format!(
                                "[region]: stopped {w:?} is not one of {STOPS:?}"
                            )));
                        }
                        if !g.maximal.is_empty() {
                            out.push(at("[region]: a stopped region's maximal line is not known"
                                .to_string()));
                        }
                    }
                    if empty != (g.minimal.is_empty() && g.maximal.is_empty()) || g.k_c < g.k_s {
                        out.push(at("[region]: the lines, k_s and k_c disagree".to_string()));
                    }
                }
            }
            if r.right.alternative.iter().any(|a| a.reading.is_none()) {
                out.push(at("[right]: an alternative names its reading".to_string()));
            }
            if !r.right.alternative.is_empty() && r.right.reading.is_none() {
                out.push(at(
                    "[right]: a contested answer names the reading it assumes".to_string(),
                ));
            }
        }
        out
    }
}

/// What a run's stdout is when it is not read in full ([`Now::stdout`]):
/// `closed`, a pipe whose reader closed before the run started.
pub const STDOUT_CLOSED: &str = "closed";

/// The one signal a row records a run ended by ([`Now::signal`],
/// [`Right::signal`]): `SIGPIPE`, which the kernel sends a
/// process that writes to a pipe no reader holds, 13 on every Unix the
/// binary ships for.
pub const SIGPIPE: i32 = 13;

/// Where a [`Printed`] line is read: the run's stdout, or the
/// read-out `--dump-answer DIR` writes uncut, `DIR/answer.txt`.
pub const PRINTED_FILES: [&str; 2] = ["stdout", "answer.txt"];

/// Whether a [`Printed`] line can be read in `file`: one of
/// [`PRINTED_FILES`], or a state file `--dump-answer DIR`
/// writes, `models/NN.ein`.
pub fn printed_file(file: &str) -> bool {
    PRINTED_FILES.contains(&file)
        || file
            .strip_prefix("models/")
            .and_then(|f| f.strip_suffix(".ein"))
            .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// A line a run prints: the first line of
/// [`Self::file`] that begins, past its indent, with [`Self::label`], as
/// printed, its trailing spaces aside.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Printed {
    /// One [`printed_file`] accepts.
    pub file: String,
    /// What the line begins with, past its indent: a row's label, or `-H`'s
    /// relation.
    pub label: String,
    /// The whole line; or empty, where no line of the file
    /// begins with the label.
    pub line: String,
}

/// What a claim's check came to, as the summary spells it:
/// `none` where the program states no such claim.
pub const CLAIM_OUTCOMES: [&str; 4] = ["held", "failed", "not-checked", "none"];

/// What a [`Now`] run's states are: all of R, or one of its lines.
pub const LINES: [&str; 3] = ["all", "minimal", "maximal"];

/// The two kinds of an empty `R` as `ein-summary/2` spells them, which
/// [`Now`] reads: every node dead, or every alive node owing.
pub const SUMMARY_EMPTY: [&str; 2] = ["dead", "owing"];

/// The routes that compute a [`Region`], as `ein solve` names them.
pub const ROUTES: [&str; 4] = ["route 1", "route 2", "state route", "tree and classes"];

/// Why a stopped [`Region`] stopped: past `--region-limit` on a
/// program the static check flags, or at `--region-budget`.
pub const STOPS: [&str; 2] = ["limit", "budget"];

/// The two kinds of an empty `R`.
pub const EMPTY_KINDS: [&str; 2] = ["no-alive", "all-owe"];

/// The verdict words, as `--json-summary`'s `verdict.type` spells them.
pub const VERDICTS: [&str; 4] = ["Solution", "Ambiguity", "Contradiction", "Open"];

/// The one directory outside the corpus a witness may live in: minimised inputs
/// that violate a property the corpus sweeps assert, and so cannot be corpus
/// entries until they are fixed (`corpus/fuzz_findings/README.md`).
pub const OUTSIDE_THE_CORPUS: &str = "corpus/fuzz_findings/";

/// What a region run answers: `ein-summary/2`'s fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Now {
    /// A corpus run name: the mode that asks the row's question, `solve
    /// --maximal` for a model-set row; or a command's run that
    /// is not a solve, `saturate --dump` or `render rules`
    /// ([`Self::is_command`]).
    pub run: String,
    /// One of [`LINES`]: which part of R its states are. Empty on a
    /// command's run, which lists no state.
    #[serde(default)]
    pub line: String,
    /// The run reads the witness through a `.einb`:
    /// `ein kb save WITNESS DIR/<stem>.einb` first, then
    /// the run on the container.
    #[serde(default)]
    pub container: bool,
    /// A command's run answered: exit 0, with nothing
    /// on stderr, and with [`Self::container`] its stdout is the same run's on
    /// the witness, every number a line times masked and the lines that say
    /// what the run read (`input:`, its size, `parse:`, `kb load:`, `open:`)
    /// aside. A command's `now` is a refusal, an error or this.
    #[serde(default)]
    pub answered: bool,
    /// The run refuses the program — *forbid*: exit 1, and
    /// this is the first line of its stderr. A refused run lists nothing, so
    /// the fields below keep their defaults.
    #[serde(default)]
    pub refused: Option<String>,
    /// The run ended in an error before it answered: exit 1, and this
    /// is the first line of its stderr. Not a refusal: the
    /// program was accepted and a firing stopped the run, so the library
    /// returns a saturation error where a refusal returns
    /// `SolveError::Refused`. Such a run lists nothing, and is never
    /// [`Right`]. (Not the summary's `region.stopped`, which says why a
    /// search that answered stopped.)
    #[serde(default)]
    pub error: Option<String>,
    /// The exit code a [`Self::refused`] run ends with, where it is not 1:
    /// `2`, a usage refusal of what a solve's
    /// run was asked to write, the library answering the program, so the row
    /// states its [`Region`].
    #[serde(default)]
    pub exit: Option<i32>,
    /// The run's stdout is not read in full:
    /// [`STDOUT_CLOSED`], a pipe whose reader closed before the run started.
    /// Such a run is a command's ([`Self::is_command`]), whatever its
    /// subcommand, and its answer is how it ended: [`Self::signal`] or
    /// [`Self::panicked`], exactly one.
    #[serde(default)]
    pub stdout: Option<String>,
    /// The signal that ended a run whose stdout is closed, with
    /// nothing on stderr: [`SIGPIPE`], as `cat` ends (Q-M1f.68).
    #[serde(default)]
    pub signal: Option<i32>,
    /// A run whose stdout is closed panicked:
    /// exit 101, and this is the panic's message, the line of stderr after
    /// `thread 'main' (…) panicked at …:`, whose thread id and source position
    /// are the toolchain's.
    #[serde(default)]
    pub panicked: Option<String>,
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
    /// summary's `root.hypgen.outside`: what the program can
    /// conclude or ask about and the rung that built H proposes no fact of.
    #[serde(default)]
    pub outside: Vec<String>,
    /// The unsat core the run reports, sorted — the summary's `region.core`.
    /// Recorded exactly where [`Right::core`] is stated.
    #[serde(default)]
    pub core: Option<Vec<String>>,
    /// Each member of `N` the run reports and its core — `--dump-answer`'s
    /// `nogoods.json`, each set and core sorted, the
    /// list sorted by set. Recorded exactly where [`Right::nogoods`] is
    /// stated: [`Self::core`] is root's, and a dead node's core is not.
    #[serde(default)]
    pub nogoods: Option<Vec<Nogood>>,
    /// The status the run reports beside root's core — the summary's
    /// `region.core_status`: `within-budget`, `cut-`
    /// and the caps its search stopped at, or `none` where the summary
    /// states none (a `null`, or no such field). Recorded exactly where
    /// [`Right::core_status`] is stated.
    #[serde(default)]
    pub core_status: Option<String>,
    /// How many members of R the run met hold a stale fact, and of how many
    /// nodes — the summary's `monotonicity.state`: its `stale`
    /// entries of class `R`, and its `nodes`.
    /// Recorded exactly where [`Right::stale`] is stated.
    #[serde(default)]
    pub stale: Option<Stale>,
    /// [`Self::nogoods`] lists some members of `N`, and the run's `N` is
    /// compared on those sets alone, each of which it must report: for a
    /// program whose `N` is hundreds of sets.
    #[serde(default)]
    pub nogoods_listed: bool,
    /// How many members of R the run met and counts without listing them —
    /// the summary's states with `listed = false`: what a line
    /// mode past `--region-limit` leaves to
    /// `--dump-answer`. Recorded exactly where [`Right::others`] is stated.
    #[serde(default)]
    pub others: Option<u64>,
    /// The outcome of the program's `(solve … :expect …)` claim — the
    /// summary's `claim.outcome`, one of [`CLAIM_OUTCOMES`], `none` where the
    /// summary states none. Recorded exactly
    /// where [`Right::claim`] is stated.
    #[serde(default)]
    pub claim: Option<String>,
    /// Each query's projection classes and claim, in program order — the
    /// summary's `queries`. Recorded
    /// exactly where [`Right::queries`] is stated.
    #[serde(default)]
    pub queries: Option<Vec<QueryAnswer>>,
    /// The rules root's `refuted by` row names, in the order it prints them —
    /// the summary's `region.refuted_by`,
    /// empty where the summary states none. Recorded exactly where
    /// [`Right::refuted_by`] is stated.
    #[serde(default)]
    pub refuted_by: Option<Vec<String>>,
    /// Each sentence the read-out prints for a query's `:goal-text`, one per
    /// row of a projection class, in the order printed:
    /// the read-out's `“…”` lines, since the summary carries
    /// the rows and not the sentences. Recorded exactly where
    /// [`Right::goal_text`] is stated.
    #[serde(default)]
    pub goal_text: Option<Vec<String>>,
    /// Lines the run prints, each found by its label: on
    /// stdout, or in `--dump-answer`'s `answer.txt`, which
    /// the run then writes. Recorded exactly where [`Right::printed`] is
    /// stated.
    #[serde(default)]
    pub printed: Option<Vec<Printed>>,
}

/// One query's answer: how many
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

/// Members of R holding a stale fact, of the nodes a run computed.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stale {
    /// Members of R whose own fixpoint falsifies a fact's `absent` guard.
    pub in_r: u64,
    /// The nodes the run computed.
    pub nodes: u64,
}

/// A minimal dead set and the core its run reports for it: one entry of `--dump-answer`'s `nogoods.json`,
/// its `refuted_by` left out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nogood {
    /// The commitment, sorted.
    pub set: Vec<String>,
    /// Its core, sorted.
    pub core: Vec<String>,
}

/// `nogoods` as a row compares them: each set and core sorted, and
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
    /// Is the run a command's that is not a solve, or one whose
    /// stdout is closed? Then it writes no summary, and its
    /// answer is a refusal, an error or [`Self::answered`], or how a closed
    /// stdout ended it.
    pub fn is_command(&self) -> bool {
        !self.run.starts_with("solve") || self.stdout_closed()
    }

    /// Does the run write to a pipe whose reader closed before it started?
    /// Then it is a command's run whatever its subcommand, and
    /// its answer is a signal or a panic.
    pub fn stdout_closed(&self) -> bool {
        self.stdout.as_deref() == Some(STDOUT_CLOSED)
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

    /// Does the run refute a claim the program states? Then it exits 1, as
    /// `ein solve` does on a refuted claim.
    pub fn refutes(&self) -> bool {
        self.claim.as_deref() == Some("failed")
            || self.queries.iter().flatten().any(|q| q.expect == "failed")
    }

    /// Where it disagrees with `right`, field by field: the verdict, `k`,
    /// `exhausted`, and the models over `closes` ∩ [`Self::relations`]. A
    /// relation `closes` names that no candidate is over must hold the same
    /// facts in every model `right` lists, or the comparison could not see
    /// it. Empty where the run answers `right` on every field `right`
    /// states.
    pub fn against(&self, right: &Right) -> Vec<String> {
        let mut out = Vec::new();
        // An error is no answer at all, so it is never the right one.
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
        // A run whose stdout is closed answers by how it ended:
        // a signal, with nothing on stderr, or a panic.
        if let Some(s) = right.signal {
            match (self.signal, &self.panicked) {
                (Some(n), _) if n == s => {}
                (Some(n), _) => out.push(format!("right: ended by signal {s}, now: by signal {n}")),
                (None, Some(m)) => out.push(format!(
                    "right: ended by signal {s}, now: a panic, exit 101: {m}"
                )),
                (None, None) => out.push(format!("right: ended by signal {s}, now: no signal")),
            }
            return out;
        }
        if self.signal.is_some() || self.panicked.is_some() {
            out.push("right: an answer, now: a run a closed stdout ended".to_string());
            return out;
        }
        // A command's answer is that it answered: no verdict,
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
        // The rules `refuted by` names.
        if let Some(w) = &right.refuted_by
            && Some(w) != self.refuted_by.as_ref()
        {
            out.push(format!(
                "refuted by: right {w:?}, now {:?}",
                self.refuted_by
            ));
        }
        // A query's sentences, in order.
        if let Some(w) = &right.goal_text
            && Some(w) != self.goal_text.as_ref()
        {
            out.push(format!("goal text: right {w:?}, now {:?}", self.goal_text));
        }
        // The lines printed.
        if let Some(w) = &right.printed
            && Some(w) != self.printed.as_ref()
        {
            out.push(format!("printed: right {w:?}, now {:?}", self.printed));
        }
        // The core's status.
        if let Some(s) = &right.core_status
            && Some(s) != self.core_status.as_ref()
        {
            out.push(format!(
                "core status: right {s:?}, now {:?}",
                self.core_status
            ));
        }
        // The claims and the classes.
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
        // The members met and not listed.
        if let Some(o) = right.others
            && Some(o) != self.others
        {
            out.push(format!(
                "members met and not listed: right {o}, now {:?}",
                self.others
            ));
        }
        // Stale members of R.
        if let Some(w) = right.stale
            && Some(w) != self.stale.as_ref().map(|s| s.in_r)
        {
            out.push(format!(
                "stale members of R: right {w}, now {:?}",
                self.stale.as_ref().map(|s| s.in_r)
            ));
        }
        // N's cores.
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

/// The witness's `R`: what `ein solve` describes, in its
/// own vocabulary: the lines are **commitments**, each the candidates it
/// holds, sorted. A model projected over `closes` is [`Right`]'s
/// vocabulary.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    /// One of [`ROUTES`].
    pub route: String,
    /// The static check's verdict. A flagged program's `R` is the cube's,
    /// every node a fork of root, an assumption the engine answers under and
    /// claims nothing beyond (Q-M1e2.2).
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
    /// One of [`STOPS`] where the description stopped before R was
    /// exhausted: `k_s` and `k_c` are then the lower
    /// bounds it reached, `minimal` the line members it certified, and
    /// `maximal` is empty, not known.
    #[serde(default)]
    pub stopped: Option<String>,
}

/// An `id_order_invariance` op and a seed under which a permuted interner
/// once moved the answer: the run must not move under it.
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
    #[serde(default)]
    pub derived: Option<String>,
    /// The answer is a **refusal** — *forbid*: the program is
    /// refused before any question about a commitment, a load error's exit.
    /// A refusal states no verdict, `k` or model.
    #[serde(default)]
    pub refused: bool,
    /// The answer is that a command's run **answers**: a program another command answers is answered
    /// by this one too, exit 0, which [`Now::answered`] records. It states no
    /// verdict, `k` or model.
    #[serde(default)]
    pub answered: bool,
    /// The answer is that a run whose stdout is closed ends by this signal,
    /// with nothing on stderr: [`SIGPIPE`],
    /// as `cat` ends (Q-M1f.68). It states no
    /// refusal, verdict, `k` or model. Compared with [`Now::signal`].
    #[serde(default)]
    pub signal: Option<i32>,
    #[serde(default)]
    pub verdict: Option<String>,
    #[serde(default)]
    pub k: Option<usize>,
    #[serde(default)]
    pub exhausted: Option<bool>,
    #[serde(default)]
    pub core: Option<Vec<String>>,
    /// Each member of `N` and the core the definitions give it: the facts its refutation rests on, which name
    /// the commitment.
    #[serde(default)]
    pub nogoods: Option<Vec<Nogood>>,
    /// The status the right read-out reports beside root's core: `cut-` and the caps a search stopped at,
    /// where a cut core says so.
    #[serde(default)]
    pub core_status: Option<String>,
    /// How many members of R hold a stale fact under the reading: compared with [`Now::stale`]'s `in_r`.
    #[serde(default)]
    pub stale: Option<u64>,
    /// How many members of R the right read-out meets and counts without
    /// listing them: a line mode past
    /// `--region-limit` lists the line members it certifies, [`Self::k`] of
    /// them, which make no verdict, so a right that states `others` states
    /// `k` and `exhausted` and no verdict, and `exhausted = false` where it
    /// meets a member it does not list. A minimal question where ∅ is in R
    /// is decided at root, past the limit too, so it meets
    /// none: `others = 0` and `exhausted = true`. Compared with
    /// [`Now::others`].
    #[serde(default)]
    pub others: Option<u64>,
    /// The outcome the program's solve-form claim should come to. Compared with [`Now::claim`].
    #[serde(default)]
    pub claim: Option<String>,
    /// Each query's classes and claim outcome, in program order, as the
    /// definitions give them. Compared
    /// with [`Now::queries`].
    #[serde(default)]
    pub queries: Option<Vec<QueryAnswer>>,
    /// The rules root's `refuted by` should name, in order. Compared with [`Now::refuted_by`].
    #[serde(default)]
    pub refuted_by: Option<Vec<String>>,
    /// The sentences the read-out should print for a query's `:goal-text`,
    /// in order, each template's references filled.
    /// Compared with [`Now::goal_text`].
    #[serde(default)]
    pub goal_text: Option<Vec<String>>,
    /// The lines the run should print, each found by its label. Compared with [`Now::printed`].
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
    /// A sentence the right read-out would add. On a row that states
    /// [`Self::outside`], the read-out's `outside` row; on one that states
    /// [`Self::core_status`], its `core search` row. Where `[case.now]`
    /// answers that field, the run's stdout is held to print it.
    #[serde(default)]
    pub qualifier: Option<String>,
    /// The relations the right read-out names outside the universe, sorted —
    /// Q-M1e2.3: R is over H, and the read-out says what H leaves out.
    #[serde(default)]
    pub outside: Option<Vec<String>>,
    /// The answer under each other reading of a contested case.
    #[serde(default)]
    pub alternative: Vec<Right>,
}

/// The relation a fact string is over: `(p A)` → `p`, `(not (p A))` → `not`.
pub fn relation_of(fact: &str) -> &str {
    let body = fact.strip_prefix('(').unwrap_or(fact);
    body.split([' ', ')']).next().unwrap_or("")
}

/// A recorded state's facts over `closes`, sorted — a row's spelling of a
/// model.
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
    use crate::{corpus, regressions, repo_root};

    /// A row given as text, in a file of one row.
    fn row(text: &str) -> Row {
        let file: Regressions = toml::from_str(&format!("schema = \"{SCHEMA}\"\n{text}"))
            .unwrap_or_else(|e| panic!("the row parses: {e}"));
        file.case.into_iter().next().expect("one row")
    }

    fn defects(r: &Row, held: Held) -> Vec<String> {
        Regressions {
            schema: SCHEMA.to_string(),
            case: vec![r.clone()],
        }
        .defects(held)
    }

    fn named(name: &str) -> Row {
        regressions()
            .case
            .into_iter()
            .find(|r| r.id == name)
            .unwrap_or_else(|| panic!("{name}: no such row in {REGRESSIONS}"))
    }

    /// The smallest row: a solve whose one state is root.
    const ONE: &str = r##"
[[case]]
id       = "one"
title    = "a row"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
verdict   = "Solution"
k         = 1
exhausted = true

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 1
models    = [
  [],
]
"##;

    /// A run whose claim is refuted where the right answer holds it.
    const CLAIM: &str = r##"
[[case]]
id       = "claim"
title    = "a claim compares printed text"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
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

    /// A dead root whose core is today's answer, where the right answer is a
    /// refusal.
    const CORE_BESIDE_A_REFUSAL: &str = r##"
[[case]]
id       = "core-beside-a-refusal"
title    = "a bare `_` loads in silence"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 0
k_c      = 0
empty    = "no-alive"
minimal  = []
maximal  = []

[case.right]
refused   = true

[case.now]
run       = "solve"
line      = "all"
exhausted = true
k         = 0
empty     = "dead"
relations = ["big", "c", "hit"]
models    = []
core      = ["(big b0)", "(c x)"]
"##;

    /// A query's classes as today's answer, where the right answer is a
    /// refusal.
    const QUERY_BESIDE_A_REFUSAL: &str = r##"
[[case]]
id       = "query-beside-a-refusal"
title    = "a bare `_` loads in silence"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
refused   = true

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

    /// A query's `:goal-text` printed as written where the right answer fills
    /// its reference.
    const GOAL_TEXT_AS_WRITTEN: &str = r##"
[[case]]
id       = "goal-text-as-written"
title    = "a template cannot name a variable whose name holds a `*`"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 1
k_c      = 1
minimal  = [[]]
maximal  = [[]]

[case.right]
verdict   = "Solution"
k         = 1
exhausted = true
closes    = ["q"]
models    = [["(q a)"]]
goal_text = ["a is a q"]

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

    /// A flagged program past the limit: a stopped region, a stale count,
    /// and `N` compared on the one set it lists.
    const PAST_THE_LIMIT: &str = r##"
[[case]]
id       = "past-the-limit"
title    = "a positive derived under absent survives the commitment that falsifies its guard"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "route 2"
monotone = false
k_s      = 7
k_c      = 9
stopped  = "limit"
minimal  = [[]]
maximal  = []

[case.right]
k         = 7
exhausted = false
stale     = 0
nogoods   = [{ set = ["(p a)"], core = ["(p a)", "(seed a)"] }]

[case.now]
run       = "solve"
line      = "all"
exhausted = false
k         = 7
models    = [[], [], [], [], [], [], []]
stale     = { in_r = 5, nodes = 9 }
nogoods   = [{ set = ["(p a)"], core = ["(p a)", "(seed a)"] }]
nogoods_listed = true
"##;

    /// A dead root whose `refuted by` names other rules than the right
    /// answer's, beside the same core.
    const REFUTED_BY: &str = r##"
[[case]]
id       = "refuted-by"
title    = "a flag moves `refuted by` beside an unmoved core"
settings = "default"
witness  = "examples/branching/01_saturate_only.ein"

[case.region]
route    = "state route"
monotone = true
k_s      = 0
k_c      = 0
empty    = "no-alive"
minimal  = []
maximal  = []

[case.right]
verdict    = "Contradiction"
k          = 0
exhausted  = true
core       = ["(g1 a)", "(g2 a)"]
refuted_by = ["k1"]

[case.now]
run        = "solve"
line       = "all"
exhausted  = true
k          = 0
empty      = "dead"
models     = []
core       = ["(g1 a)", "(g2 a)"]
refuted_by = ["v", "w"]
"##;

    #[test]
    fn the_regressions_are_well_formed_and_every_row_answers_right() {
        let file = regressions();
        assert!(!file.case.is_empty(), "{REGRESSIONS} has no rows");
        let mut bad = file.defects(Held::Right);
        bad.extend(file.not_right());
        assert!(bad.is_empty(), "{REGRESSIONS}:\n  {}", bad.join("\n  "));
    }

    /// **A row of the project's file carries no key the format does not
    /// read**; a file of another reader may, and keeps them whole.
    #[test]
    fn a_key_the_format_does_not_read_is_kept_and_refused_as_a_regression() {
        let r = row(&ONE.replace("witness  = ", "owner    = \"someone\"\nwitness  = "));
        assert_eq!(
            r.other.get("owner").and_then(toml::Value::as_str),
            Some("someone")
        );
        assert!(
            defects(&r, Held::Open).is_empty(),
            "{:?}",
            defects(&r, Held::Open)
        );
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("keys the format does not read"))
        );
        assert!(defects(&row(ONE), Held::Right).is_empty());
    }

    /// **A core's status is a word the engine writes**: `none`,
    /// `within-budget`, or `cut-` and the caps in `explain::Cut`'s order.
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

    /// A row's witness is a file the corpus knows. The file adds no new kind
    /// of program: it adds the answer a corpus program should have.
    #[test]
    fn every_witness_is_a_file_the_corpus_knows() {
        let repo = repo_root();
        let corpus = corpus();
        let mut bad = Vec::new();
        for r in &regressions().case {
            let w = &r.witness;
            if !repo.join(w).is_file() {
                bad.push(format!("{}: {w} does not exist", r.id));
            } else if !w.starts_with(OUTSIDE_THE_CORPUS)
                && !corpus.entry.iter().any(|e| &e.path == w)
            {
                bad.push(format!("{}: {w} has no corpus.toml entry", r.id));
            }
            for q in &r.decisions {
                let file = std::fs::read_dir(repo.join("docs/decisions"))
                    .into_iter()
                    .flatten()
                    .flatten()
                    .filter(|d| d.path().is_dir())
                    .flat_map(|d| std::fs::read_dir(d.path()).into_iter().flatten().flatten())
                    .any(|f| {
                        f.file_name()
                            .to_string_lossy()
                            .starts_with(&format!("{q}-"))
                    });
                if !file {
                    bad.push(format!(
                        "{}: decision {q} has no file under docs/decisions/",
                        r.id
                    ));
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    /// **A wrong answer is open by the method**, held without a run: a row
    /// whose `[case.now]` answers its `[case.right]` on every field the
    /// format records is answered right, unless the run cannot show what
    /// keeps it open, or its reading is contested.
    #[test]
    fn a_wrong_answer_is_open_by_the_method() {
        let file = |r: &Row| Regressions {
            schema: SCHEMA.to_string(),
            case: vec![r.clone()],
        };
        // Answered right: flagged under `Open`, and clean under `Right`.
        let right = row(ONE);
        assert_eq!(file(&right).right_by_the_method().len(), 1);
        assert!(file(&right).not_right().is_empty());
        // Answered wrong: the other way round.
        let mut wrong = right.clone();
        wrong.now.k = 2;
        wrong.now.models.push(vec!["(p a)".to_string()]);
        assert!(file(&wrong).right_by_the_method().is_empty());
        assert!(!file(&wrong).not_right().is_empty());
        // What the run cannot show keeps a row open: a right that is a
        // refusal or states no verdict, a qualifier with no `outside`, facts
        // over a relation no candidate is over, an answer under a permuted
        // interner.
        let mut r = right.clone();
        r.right.verdict = None;
        assert!(file(&r).right_by_the_method().is_empty());
        let mut r = right.clone();
        r.right.qualifier = Some("a line".to_string());
        assert!(file(&r).right_by_the_method().is_empty());
        let mut r = right.clone();
        r.right.closes = vec!["noted".to_string()];
        r.right.models = Some(vec![vec!["(noted a)".to_string()]]);
        assert!(file(&r).right_by_the_method().is_empty());
        let mut r = right.clone();
        r.id_order = Some(IdOrder {
            op: "solve[default]".to_string(),
            seed: 1,
        });
        assert!(file(&r).right_by_the_method().is_empty());
        // So does a contested reading.
        let mut r = right;
        r.right.reading = Some("a reading".to_string());
        let mut alt = r.right.clone();
        alt.reading = Some("another".to_string());
        r.right.alternative = vec![alt];
        assert!(file(&r).right_by_the_method().is_empty());
    }

    /// **A run that ends in an error is no answer**: such a `now` lists
    /// nothing, states no region, is not also a refusal, and is never
    /// `right`.
    #[test]
    fn a_run_that_ends_in_an_error_is_no_answer() {
        let mut r = row(ONE);
        r.right.refused = true;
        r.right.verdict = None;
        r.right.k = None;
        r.right.exhausted = None;
        r.now = row(ONE).now;
        r.now.run = "solve --minimal".to_string();
        r.now.line = "minimal".to_string();
        r.now.error = Some("a line".to_string());
        r.now.exhausted = false;
        r.now.k = 0;
        r.now.models.clear();
        r.region = None;
        assert!(
            defects(&r, Held::Open).is_empty(),
            "{:?}",
            defects(&r, Held::Open)
        );
        assert_eq!(r.now.against(&r.right), ["right: a refusal, now: an error"]);
        let mut s = r.clone();
        s.now.k = 1;
        assert!(
            defects(&s, Held::Open)
                .iter()
                .any(|d| d.contains("an error [now] lists nothing"))
        );
        let mut s = r.clone();
        s.now.refused = Some("a line".to_string());
        assert!(
            defects(&s, Held::Open)
                .iter()
                .any(|d| d.contains("a refusal or an error, not both"))
        );
        let mut s = r;
        s.region = row(ONE).region;
        assert!(
            defects(&s, Held::Open)
                .iter()
                .any(|d| d.contains("an error [now] has no [case.region]"))
        );
    }

    /// **N's cores**: a `now` records them exactly where `right` states
    /// them, sorted, and they are compared with `right`'s set for set.
    #[test]
    fn a_dead_nodes_core_is_compared_set_for_set() {
        let row = regressions()
            .case
            .into_iter()
            .find(|r| r.now.nogoods.is_some())
            .expect("a regression records N's cores");
        assert!(
            defects(&row, Held::Right).is_empty(),
            "{:?}",
            defects(&row, Held::Right)
        );
        let mut r = row.clone();
        r.right.nogoods = None;
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("exactly when [right] states them"))
        );
        let mut r = row.clone();
        r.now.nogoods = Some(vec![Nogood {
            set: vec!["(z a)".to_string(), "(a a)".to_string()],
            core: Vec::new(),
        }]);
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("[now] nogoods are sorted"))
        );
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

    /// A stale count is compared where `right` states one, a stopped region
    /// states no maximal line, and a `now` that compares N on the sets it
    /// lists lists some.
    #[test]
    fn a_stale_count_a_stopped_region_and_a_listed_n() {
        let row = row(PAST_THE_LIMIT);
        assert!(
            defects(&row, Held::Open).is_empty(),
            "{:?}",
            defects(&row, Held::Open)
        );
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
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("stale members of R"))
        );
        // A stopped region's maximal line is not known, and its word is one
        // of STOPS.
        let mut r = row.clone();
        let g = r.region.as_mut().expect("a stopped region");
        g.maximal = vec![Vec::new()];
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("not known"))
        );
        let g = r.region.as_mut().expect("a stopped region");
        g.maximal = Vec::new();
        g.stopped = Some("tired".to_string());
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("is not one of"))
        );
        // A listed N lists some members.
        let mut r = row;
        r.now.nogoods = Some(Vec::new());
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("nogoods_listed"))
        );
    }

    /// The members a line mode past the limit meets and does not list are
    /// compared where `right` states them, and such a right states `k`,
    /// `exhausted = false` and no verdict; a minimal line decided at root
    /// meets none, `others = 0`, and is exhausted.
    #[test]
    fn a_line_past_the_limit_counts_what_it_does_not_list() {
        let file = regressions();
        let row = file
            .case
            .iter()
            .find(|r| r.right.others.is_some_and(|o| o > 0))
            .expect("a maximal line past the limit states the members met and not listed")
            .clone();
        let decided = file
            .case
            .iter()
            .find(|r| r.right.others == Some(0))
            .expect("a minimal line past the limit is decided at root")
            .clone();
        assert!(
            defects(&row, Held::Right).is_empty(),
            "{:?}",
            defects(&row, Held::Right)
        );
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
        let mut r = row.clone();
        r.now.others = None;
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("does not list"))
        );
        let mut r = row.clone();
        r.right.verdict = Some("Solution".to_string());
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("no verdict"))
        );
        let mut r = row;
        r.right.exhausted = Some(true);
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("no verdict"))
        );
        assert!(
            defects(&decided, Held::Right).is_empty(),
            "{:?}",
            defects(&decided, Held::Right)
        );
        assert_eq!(decided.right.exhausted, Some(true));
        let mut r = decided;
        r.right.verdict = Some("Solution".to_string());
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("no verdict"))
        );
    }

    /// A claim's outcome and a query's classes are compared where `right`
    /// states them, recorded on both sides or neither, in the summary's
    /// words; and a run whose claim is `failed` refutes it, which is an exit
    /// 1.
    #[test]
    fn a_claim_and_a_querys_classes_are_answers() {
        let solve = row(CLAIM);
        let query = regressions()
            .case
            .into_iter()
            .find(|r| r.right.queries.is_some() && r.now.queries.is_some())
            .expect("a regression states a query's classes");
        for (r, held) in [(&solve, Held::Right), (&query, Held::Right)] {
            assert!(defects(r, held).is_empty(), "{:?}", defects(r, held));
        }
        let mut r = solve.clone();
        r.now.claim = Some("not-checked".to_string());
        assert!(
            r.now
                .against(&r.right)
                .iter()
                .any(|d| d.starts_with("claim"))
        );
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
        let mut r = query;
        r.now.queries = None;
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("[now] records a claim"))
        );
        let mut r = solve.clone();
        r.right.claim = Some("holds".to_string());
        assert!(
            defects(&r, Held::Right)
                .iter()
                .any(|d| d.contains("claim outcome"))
        );
        // A failed claim is a refutation: the run exits 1.
        let mut r = solve;
        assert!(r.now.refutes());
        r.now.claim = Some("held".to_string());
        assert!(!r.now.refutes());
    }

    /// Beside a `right` that is a refusal, `now` may record the core and the
    /// claims today's answer reports, which the refusal replaces; anywhere
    /// else a core or a claim is recorded exactly where `right` states one,
    /// and a refused `now` records none.
    #[test]
    fn beside_a_refusal_now_records_what_today_reports() {
        let core = row(CORE_BESIDE_A_REFUSAL);
        let query = row(QUERY_BESIDE_A_REFUSAL);
        for r in [&core, &query] {
            assert!(
                defects(r, Held::Open).is_empty(),
                "{:?}",
                defects(r, Held::Open)
            );
            // An answer, where `right` is a refusal: open, by the method.
            assert!(!r.now.against(&r.right).is_empty());
        }
        let mut r = core.clone();
        r.right.refused = false;
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("[now] records a core"))
        );
        let mut r = query;
        r.right.refused = false;
        assert!(
            defects(&r, Held::Open)
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
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("a refused [now] lists nothing"))
        );
        r.now.core = None;
        assert!(
            defects(&r, Held::Open).is_empty(),
            "{:?}",
            defects(&r, Held::Open)
        );
    }

    /// **What `refuted by` names is an answer**: the rules root's row names,
    /// recorded exactly where `right` states them and compared in the order
    /// printed.
    #[test]
    fn what_refuted_by_names_is_an_answer() {
        let row = row(REFUTED_BY);
        assert!(
            defects(&row, Held::Open).is_empty(),
            "{:?}",
            defects(&row, Held::Open)
        );
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
        let mut r = row.clone();
        r.right.refuted_by = None;
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("[now] records what `refuted by` names"))
        );
        let mut r = row;
        r.right.core = None;
        r.now.core = None;
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("beside its core"))
        );
        // A regression states one too, and answers it.
        let live = regressions()
            .case
            .into_iter()
            .find(|r| r.right.refuted_by.is_some())
            .expect("a regression states what `refuted by` names");
        assert!(live.now.against(&live.right).is_empty());
    }

    /// **A line a run prints is an answer**: recorded exactly where `right`
    /// states lines, the same labels in the same files, each file one the
    /// test reads and each line beginning with its label; compared whole, so
    /// a label glued to its value is another answer than the label and the
    /// value apart.
    #[test]
    fn a_line_a_run_prints_is_an_answer() {
        let file = regressions();
        let rows: Vec<&Row> = file
            .case
            .iter()
            .filter(|r| r.right.printed.is_some())
            .collect();
        assert!(rows.len() >= 2, "regressions state the lines printed");
        for row in rows {
            assert!(
                defects(row, Held::Right).is_empty(),
                "{:?}",
                defects(row, Held::Right)
            );
            let mut r = row.clone();
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
            let mut r = row.clone();
            r.right.printed = None;
            assert!(
                defects(&r, Held::Right)
                    .iter()
                    .any(|d| d.contains("the lines printed"))
            );
            let mut r = row.clone();
            if let Some(p) = r.now.printed.as_mut().and_then(|v| v.first_mut()) {
                p.label = "another".to_string();
            }
            assert!(
                defects(&r, Held::Right)
                    .iter()
                    .any(|d| d.contains("the same labels"))
            );
            let mut r = row.clone();
            if let Some(p) = r.right.printed.as_mut().and_then(|v| v.first_mut()) {
                p.file = "answer.json".to_string();
            }
            assert!(
                defects(&r, Held::Right)
                    .iter()
                    .any(|d| d.contains("is not one of"))
            );
        }
    }

    /// **A solve's usage refusal exits 2**: `ein solve X.einb --dump-answer
    /// DIR` refusing the container while the library answers the program is
    /// a well-formed `now`, its region stated; the exit is 2 and nothing
    /// else, on a solve and on nothing else.
    #[test]
    fn a_solves_usage_refusal_exits_two() {
        let row = named("einb-dump-answer");
        assert!(row.now.container && row.region.is_some());
        let mut r = row.clone();
        r.right.nogoods = None;
        r.now = Now {
            refused: Some("error: --dump-answer {out}/answer: a .einb container …".to_string()),
            exit: Some(2),
            line: "all".to_string(),
            k: 0,
            exhausted: false,
            empty: None,
            relations: Vec::new(),
            models: Vec::new(),
            nogoods: None,
            ..row.now.clone()
        };
        assert!(
            defects(&r, Held::Open).is_empty(),
            "{:?}",
            defects(&r, Held::Open)
        );
        // Exit 1, the default, is *forbid*'s, which states no region.
        let mut s = r.clone();
        s.now.exit = None;
        assert!(
            defects(&s, Held::Open)
                .iter()
                .any(|d| d.contains("a refused [now] has no [case.region]"))
        );
        for (exit, refused) in [(3, true), (2, false)] {
            let mut s = r.clone();
            s.now.exit = Some(exit);
            if !refused {
                s.now.refused = None;
            }
            assert!(
                defects(&s, Held::Open)
                    .iter()
                    .any(|d| d.contains("a solve's refusal exits 1, or 2")),
                "exit {exit}, refused {refused}"
            );
        }
    }

    /// **What a query's `:goal-text` prints is an answer**: the read-out's
    /// sentences, recorded exactly where `right` states them and compared in
    /// the order printed.
    #[test]
    fn what_a_goal_text_prints_is_an_answer() {
        let live = regressions()
            .case
            .into_iter()
            .find(|r| r.right.goal_text.is_some())
            .expect("a regression states what its goal text prints");
        assert!(live.now.against(&live.right).is_empty());
        let row = row(GOAL_TEXT_AS_WRITTEN);
        assert!(
            defects(&row, Held::Open).is_empty(),
            "{:?}",
            defects(&row, Held::Open)
        );
        let differs = row.now.against(&row.right);
        assert!(
            differs.len() == 1 && differs[0].starts_with("goal text: "),
            "{differs:?}"
        );
        let mut r = row.clone();
        r.now.goal_text = r.right.goal_text.clone();
        assert!(r.now.against(&r.right).is_empty());
        if let Some(v) = r.now.goal_text.as_mut() {
            v.push("another".to_string());
            v.reverse();
        }
        assert!(!r.now.against(&r.right).is_empty());
        let mut r = row.clone();
        r.right.goal_text = None;
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("[now] records a query's sentences"))
        );
        let mut r = row;
        r.now.refused = Some("a line".to_string());
        r.now.k = 0;
        r.now.exhausted = false;
        r.now.models = Vec::new();
        r.region = None;
        assert!(
            defects(&r, Held::Open)
                .iter()
                .any(|d| d.contains("a refused [now] lists nothing"))
        );
    }

    /// **A run whose stdout is closed answers by how it ended**: a signal
    /// with nothing on stderr, or a panic, exit 101, exactly one; such a run
    /// is a command's whatever its subcommand; and a right that is a signal
    /// is such a run's, SIGPIPE's.
    #[test]
    fn a_closed_stdout_ends_a_run_by_a_signal_or_a_panic() {
        let row = named("closed-stdout/solve");
        assert!(
            defects(&row, Held::Right).is_empty(),
            "{:?}",
            defects(&row, Held::Right)
        );
        assert!(row.now.stdout_closed() && row.now.is_command());
        assert_eq!(row.right.signal, Some(SIGPIPE));
        assert!(row.now.against(&row.right).is_empty());
        // The run panicking: [now] is not [right].
        let panic = "failed printing to stdout: Broken pipe (os error 32)";
        let mut open = row.clone();
        open.now.signal = None;
        open.now.panicked = Some(panic.to_string());
        assert!(
            defects(&open, Held::Open).is_empty(),
            "{:?}",
            defects(&open, Held::Open)
        );
        assert_eq!(
            open.now.against(&open.right),
            [format!(
                "right: ended by signal 13, now: a panic, exit 101: {panic}"
            )]
        );
        // A closed stdout states how it ended, once.
        let mut s = row.clone();
        s.now.signal = None;
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("exactly when its stdout is closed"))
        );
        let mut s = row.clone();
        s.now.panicked = Some(panic.to_string());
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("exactly one of"))
        );
        // Another signal, or another word for the stdout, is none of this.
        let mut s = row.clone();
        s.now.signal = Some(9);
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("ends a run by SIGPIPE"))
        );
        let mut s = row.clone();
        s.now.stdout = Some("half-read".to_string());
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("is not \"closed\""))
        );
        // A signal is the right of a closed stdout alone, and states nothing
        // else.
        let mut s = row.clone();
        s.right.k = Some(1);
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("a [right] that is a signal"))
        );
        let mut s = row;
        s.now.stdout = None;
        s.now.signal = None;
        assert!(
            defects(&s, Held::Right)
                .iter()
                .any(|d| d.contains("[right] states a signal exactly"))
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
