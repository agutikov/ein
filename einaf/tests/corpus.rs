//! **The corpus as the test of the feedback object.** Every entry of
//! `corpus/corpus.toml` that is not `slow`, through the bounded call:
//!
//! - asked every question `einaf::kernel` accepts, one load a program, and
//!   each outcome held to what the corpus says of the program: its group, the
//!   exit `ein solve` gives it, the claims it states, and the engine's own
//!   static check;
//! - every object written as JSON and read back as itself;
//! - and, under the questions its entry declares, **what the object says
//!   held to what the binary says of the same run**: `ein solve`'s
//!   `--json-summary`, from the binary the gate built (`$EIN_BIN`, or
//!   `ein.rs/target/gate/ein`). The library and the binary are two routes to
//!   one answer, and the test is that they agree.
//!
//! Properties are asserted, and no value of an answer is banked; of a
//! flagged program only that it is flagged, and where.
//!
//! The sweep runs on the engine's worker pool (`ein_corpus::pool`,
//! `EIN_TEST_JOBS` workers), one unit a program, once for both tests.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Instant;

use einaf::kernel::{Bounds, Feedback, Outcome, Part, Question, Source, call};
use serde_json::Value as J;

/// A diagnostic as both sides write it: kind, severity, message, place.
type Said = (String, String, String, Option<(String, u64, u64)>);
/// A state as both sides write it: its facts, its lines, whether it is listed.
type StateRow = (Vec<String>, Vec<String>, bool);
/// A query's class as both sides write it: its facts, its rows, its states.
type ClassRow = (Vec<String>, Vec<BTreeMap<String, String>>, Vec<usize>);

/// What one program gave.
struct Found {
    path: String,
    group: String,
    /// Every question asked, at the default bounds, and its object.
    objects: Vec<(Question, Feedback)>,
    /// Each question the entry declares, with the bounds it declares, and
    /// what the object and the binary's summary disagree on.
    compared: Vec<(String, Vec<String>)>,
    /// The parts the compared objects held, by name: what the comparison
    /// read, so that it is not vacuous.
    held: BTreeMap<&'static str, usize>,
    /// An object that did not come back as itself through JSON.
    round_trip: Vec<String>,
}

/// The binary the gate built.
fn ein_bin() -> PathBuf {
    let bin = std::env::var_os("EIN_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| ein_corpus::repo_root().join("ein.rs/target/gate/ein"));
    assert!(
        bin.is_file(),
        "{}: no `ein` binary to compare with; the gate builds it with \
         `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace`, \
         or `cargo build --manifest-path ein.rs/Cargo.toml --profile gate -p ein-cli`, \
         and `$EIN_BIN` names another",
        bin.display()
    );
    bin
}

/// The questions an entry declares, each with its bounds: its `solve` runs,
/// read as the question and the bounds they ask, the flags that only print
/// left out; `--no-search` for an entry that declares no `solve` run.
fn declared(runs: &[String]) -> Vec<(Question, Bounds, Vec<String>)> {
    let mut out: Vec<(Question, Bounds, Vec<String>)> = Vec::new();
    let mut any = false;
    for run in runs {
        let words: Vec<&str> = run.split_whitespace().collect();
        if words.first() != Some(&"solve") {
            continue;
        }
        any = true;
        let (mut single, mut part, mut count) = (false, None, false);
        let mut bounds = Bounds::default();
        let mut extra: Vec<String> = Vec::new();
        let mut skip = false;
        let mut it = words[1..].iter();
        while let Some(&w) = it.next() {
            match w {
                "--single" => single = true,
                "--minimal" => part = Some(Part::Minimal),
                "--maximal" => part = Some(Part::Maximal),
                "--count" => count = true,
                "--region-limit" => {
                    let v = it.next().expect("a limit");
                    bounds.limit = v.parse().expect("a number");
                    extra.extend(["--region-limit".to_string(), v.to_string()]);
                }
                "--region-budget" => {
                    let v = it.next().expect("a budget");
                    bounds.budget = v.parse().expect("a number");
                    extra.extend(["--region-budget".to_string(), v.to_string()]);
                }
                // Flags that print more and ask the same question.
                "-p" | "-s" | "-H" => {}
                "--trace" | "--dump-lattice" => {
                    it.next();
                }
                // `-n N`, a question EinAf does not ask.
                _ => skip = true,
            }
        }
        if skip {
            continue;
        }
        let q = match (single, count, part) {
            (true, _, p) => Question::Single(p.unwrap_or(Part::All)),
            (_, true, _) => Question::Count,
            (_, _, Some(p)) => Question::Line(p),
            _ => Question::Describe,
        };
        if !out.iter().any(|(q2, b2, _)| *q2 == q && *b2 == bounds) {
            out.push((q, bounds, extra));
        }
    }
    if !any {
        out.push((Question::NoSearch, Bounds::default(), Vec::new()));
    }
    out
}

fn sweep() -> &'static [Found] {
    static SWEEP: OnceLock<Vec<Found>> = OnceLock::new();
    SWEEP.get_or_init(|| {
        let t = Instant::now();
        let root = ein_corpus::repo_root();
        let bin = ein_bin();
        let tmp = ein_corpus::Scratch::new("einaf-corpus");
        let corpus = ein_corpus::corpus();
        let entries: Vec<_> = corpus.entry.iter().filter(|e| !e.slow).cloned().collect();
        let found = ein_corpus::pool::map(
            "einaf_feedback",
            &entries.iter().enumerate().collect::<Vec<_>>(),
            |(_, e)| (e.path.clone(), String::new()),
            |(i, e)| {
                let path = root.join(&e.path);
                let objects: Vec<(Question, Feedback)> = Question::ALL
                    .into_iter()
                    .zip(call(Source::File(&path), &Question::ALL, Bounds::default()))
                    .collect();
                let mut round_trip = Vec::new();
                for (q, f) in &objects {
                    match Feedback::from_json(&f.to_json()) {
                        Ok(back) if back == *f => {}
                        Ok(_) => round_trip.push(format!("{}: read back as another", q.flags())),
                        Err(e) => round_trip.push(format!("{}: {e}", q.flags())),
                    }
                }
                let mut compared = Vec::new();
                let mut held: BTreeMap<&'static str, usize> = BTreeMap::new();
                for (k, (q, bounds, extra)) in declared(&e.runs).into_iter().enumerate() {
                    let f = match objects.iter().find(|(q2, _)| *q2 == q) {
                        Some((_, f)) if bounds == Bounds::default() => f.clone(),
                        _ => call(Source::File(&path), &[q], bounds)
                            .pop()
                            .expect("an object"),
                    };
                    let summary = tmp.join(format!("{i:04}-{k}.json"));
                    // The binary is handed the path the kernel is, so a line
                    // that names the program or one of its modules names it
                    // the same way in both.
                    let mut args: Vec<String> =
                        vec!["solve".to_string(), path.display().to_string()];
                    args.extend(q.argv().iter().map(|s| s.to_string()));
                    args.extend(extra.iter().cloned());
                    args.extend(["--json-summary".to_string(), summary.display().to_string()]);
                    let out = Command::new(&bin)
                        .args(&args)
                        .current_dir(&root)
                        .output()
                        .expect("ein runs");
                    let code = out.status.code().unwrap_or(-1);
                    let s: Option<J> = std::fs::read_to_string(&summary)
                        .ok()
                        .and_then(|t| serde_json::from_str(&t).ok());
                    let label = format!("{} {}", q.flags(), extra.join(" "));
                    for (part, present) in [
                        ("states", f.states.as_ref().is_some_and(|v| !v.is_empty())),
                        (
                            "classes",
                            f.queries.iter().flatten().any(|q| !q.classes.is_empty()),
                        ),
                        (
                            "claim",
                            f.claim.as_ref().is_some_and(|c| c.outcome.is_some()),
                        ),
                        ("core", f.core.is_some()),
                        (
                            "frontier",
                            f.owed.as_ref().is_some_and(|o| o.frontier.is_some()),
                        ),
                        ("diagnostics", !f.diagnostics.is_empty()),
                        ("flagged", f.flagged.is_some()),
                    ] {
                        *held.entry(part).or_default() += usize::from(present);
                    }
                    compared.push((label, agree(&f, q, code, s.as_ref())));
                }
                Found {
                    path: e.path.clone(),
                    group: e.group.clone(),
                    objects,
                    compared,
                    held,
                    round_trip,
                }
            },
        );
        eprintln!(
            "the sweep: {} programs, {:.1} s",
            found.len(),
            t.elapsed().as_secs_f64()
        );
        found
    })
}

/// **What the object says against what the binary's summary says**, as a
/// list of disagreements.
fn agree(f: &Feedback, q: Question, code: i32, s: Option<&J>) -> Vec<String> {
    let mut bad = Vec::new();
    if let Some(fault) = &f.fault {
        return vec![format!("the kernel faulted: {fault:?}")];
    }
    let outcomes: BTreeSet<Outcome> = f.outcome.iter().flatten().map(|r| r.outcome).collect();
    let Some(s) = s else {
        return vec![format!("ein exits {code} and writes no summary")];
    };
    // The diagnostics, one for one: kind, severity, message and place, each
    // path as it is written, the program's as both were handed it and a
    // module's as its import resolves it from there.
    let theirs: Vec<Said> = s["diagnostics"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|d| {
            (
                d["kind"].as_str().unwrap_or("").to_string(),
                d["severity"].as_str().unwrap_or("").to_string(),
                d["message"].as_str().unwrap_or("").to_string(),
                (!d["place"].is_null()).then(|| {
                    (
                        d["place"]["file"].as_str().unwrap_or("").to_string(),
                        d["place"]["line"].as_u64().unwrap_or(0),
                        d["place"]["col"].as_u64().unwrap_or(0),
                    )
                }),
            )
        })
        .collect();
    let ours: Vec<Said> = f
        .diagnostics
        .iter()
        .map(|d| {
            (
                d.kind.clone(),
                d.severity.clone(),
                d.message.clone(),
                d.place
                    .as_ref()
                    .map(|p| (p.file.clone(), u64::from(p.line), u64::from(p.col))),
            )
        })
        .collect();

    // Invalid: a summary with no answer, and the refusal's findings.
    if outcomes.contains(&Outcome::Invalid) {
        if outcomes.len() != 1 {
            bad.push(format!("invalid beside {outcomes:?}"));
        }
        if code != 1 || !s["region"].is_null() || !s["monotonicity"].is_null() {
            bad.push(format!("invalid, and ein exits {code} with an answer"));
        }
        if ours != theirs {
            bad.push(format!("the findings: ours {ours:?}, theirs {theirs:?}"));
        }
        return bad;
    }
    if !s["monotonicity"].is_object() {
        return vec![format!(
            "ein exits {code} with no answer: {}",
            s["diagnostics"]
        )];
    }

    // Flagged: the binary says the program is not monotone, and where; of
    // its answer, nothing is compared.
    let monotone = s["monotonicity"]["monotone"].as_bool();
    if outcomes.contains(&Outcome::Flagged) {
        if outcomes.len() != 1 || monotone != Some(false) {
            bad.push(format!(
                "flagged: ours {outcomes:?}, ein's monotone {monotone:?}"
            ));
        }
        let sites = |v: &J| -> Vec<(String, String, String, u64)> {
            v.as_array()
                .into_iter()
                .flatten()
                .map(|x| {
                    (
                        x["rule"].as_str().unwrap_or("").to_string(),
                        x["kind"].as_str().unwrap_or("").to_string(),
                        x["relation"].as_str().unwrap_or("").to_string(),
                        x["depth"].as_u64().unwrap_or(0),
                    )
                })
                .collect()
        };
        let ours_sites: Vec<(String, String, String, u64)> = f
            .flagged
            .iter()
            .flat_map(|x| &x.sites)
            .map(|x| {
                (
                    x.rule.clone(),
                    x.kind.clone(),
                    x.relation.clone(),
                    u64::from(x.depth),
                )
            })
            .collect();
        if ours_sites != sites(&s["monotonicity"]["sites"]) {
            bad.push("the static check's sites".to_string());
        }
        return bad;
    }
    if monotone == Some(false) {
        bad.push(format!("ein says not monotone, ours {outcomes:?}"));
        return bad;
    }
    if ours != theirs {
        bad.push(format!("the diagnostics: ours {ours:?}, theirs {theirs:?}"));
    }

    // The outcome under each reading, read off the summary.
    let region = &s["region"];
    let theirs_outcome = |part: Part| -> Outcome {
        let empty = region["empty"].as_str();
        let counted = |k: u64| match k {
            1 => Outcome::One,
            _ => Outcome::Several,
        };
        match q {
            Question::NoSearch => {
                if s["root"]["dead"] == J::Bool(true) {
                    Outcome::Inconsistent
                } else {
                    Outcome::NotDecided
                }
            }
            Question::Single(_) => match s["verdict"]["word"].as_str() {
                Some("Solution") => Outcome::One,
                Some("Ambiguity") => Outcome::Several,
                Some("Contradiction") => Outcome::Inconsistent,
                Some("Open") => Outcome::Incomplete,
                _ => Outcome::NotDecided,
            },
            _ if region["exhausted"] != J::Bool(true) => Outcome::NotDecided,
            _ if empty == Some("dead") => Outcome::Inconsistent,
            _ if empty == Some("owing") => Outcome::Incomplete,
            Question::Line(_) => counted(region["states"].as_array().map_or(0, |v| v.len() as u64)),
            _ => counted(match part {
                Part::All => region["k_s"].as_u64().unwrap_or(0),
                Part::Minimal => region["minimal"]["states"].as_u64().unwrap_or(0),
                Part::Maximal => region["maximal"]["states"].as_u64().unwrap_or(0),
            }),
        }
    };
    for r in f.outcome.iter().flatten() {
        let t = theirs_outcome(r.part);
        if r.outcome != t {
            bad.push(format!(
                "{}: ours {:?} ({}), the summary's {t:?}",
                r.part.words(),
                r.outcome,
                r.why
            ));
        }
    }

    // The readings, the states, the queries, the claim, what is owed, the
    // core and the work, each against its key.
    let strs = |v: &J| -> Option<Vec<String>> {
        v.as_array().map(|a| {
            a.iter()
                .map(|x| x.as_str().unwrap_or("").to_string())
                .collect()
        })
    };
    if let Some(r) = &f.readings {
        if let Some(w) = &r.word
            && s["verdict"]["word"].as_str() != Some(w.word.as_str())
        {
            bad.push(format!("the word {w:?}, the summary's {}", s["verdict"]));
        }
        if matches!(q, Question::Describe | Question::Count) {
            for (ours, key) in [(r.k_s, &region["k_s"]), (r.k_c, &region["k_c"])] {
                if ours != key.as_u64() {
                    bad.push(format!("a count {ours:?}, the summary's {key}"));
                }
            }
            if r.minimal != region["minimal"]["states"].as_u64()
                || r.maximal != region["maximal"]["states"].as_u64()
            {
                bad.push("the lines' counts".to_string());
            }
        }
        if !region.is_null() && r.exhausted != region["exhausted"].as_bool() {
            bad.push(format!(
                "exhausted {:?}, the summary's {}",
                r.exhausted, region["exhausted"]
            ));
        }
    }
    if q != Question::NoSearch {
        let theirs_states: Option<Vec<StateRow>> = region["states"].as_array().map(|a| {
            a.iter()
                .map(|x| {
                    (
                        strs(&x["facts"]).unwrap_or_default(),
                        strs(&x["lines"]).unwrap_or_default(),
                        x["listed"].as_bool().unwrap_or(false),
                    )
                })
                .collect()
        });
        let ours_states: Option<Vec<StateRow>> = f.states.as_ref().map(|v| {
            v.iter()
                .map(|x| (x.facts.clone(), x.lines.clone(), x.listed))
                .collect()
        });
        if ours_states != theirs_states {
            bad.push("the states".to_string());
        }
    }
    let theirs_queries = s["queries"].as_array().cloned().unwrap_or_default();
    let ours_queries = f.queries.clone().unwrap_or_default();
    if ours_queries.len() != theirs_queries.len() {
        bad.push(format!(
            "{} queries, the summary's {}",
            ours_queries.len(),
            theirs_queries.len()
        ));
    }
    for (o, t) in ours_queries.iter().zip(&theirs_queries) {
        let classes: Vec<ClassRow> = t["classes"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| {
                (
                    strs(&c["facts"]).unwrap_or_default(),
                    c["rows"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|row| {
                            row.as_object()
                                .into_iter()
                                .flatten()
                                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                                .collect()
                        })
                        .collect(),
                    c["states"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|i| i.as_u64().unwrap_or(0) as usize)
                        .collect(),
                )
            })
            .collect();
        let ours: Vec<ClassRow> = o
            .classes
            .iter()
            .map(|c| (c.facts.clone(), c.rows.clone(), c.states.clone()))
            .collect();
        if t["goal"].as_str() != Some(o.goal.as_str())
            || t["over"].as_str() != Some(o.over.as_str())
            || t["exact"].as_bool() != Some(o.exact)
            || ours != classes
        {
            bad.push(format!("the query {}", o.goal));
        }
        let expect = |e: &J| (e["outcome"].as_str().map(str::to_string), strs(&e["lines"]));
        let ours_expect = o
            .expect
            .as_ref()
            .and_then(|e| e.outcome.clone().map(|x| (Some(x), Some(e.lines.clone()))));
        let theirs_expect = (!t["expect"].is_null()).then(|| expect(&t["expect"]));
        if ours_expect != theirs_expect {
            bad.push(format!(
                "the query's claim {ours_expect:?}, the summary's {theirs_expect:?}"
            ));
        }
    }
    let ours_claim = f
        .claim
        .as_ref()
        .and_then(|c| c.outcome.clone().map(|x| (x, c.lines.clone())));
    let theirs_claim = (!s["claim"].is_null()).then(|| {
        (
            s["claim"]["outcome"].as_str().unwrap_or("").to_string(),
            strs(&s["claim"]["lines"]).unwrap_or_default(),
        )
    });
    if ours_claim != theirs_claim {
        bad.push(format!(
            "the claim {ours_claim:?}, the summary's {theirs_claim:?}"
        ));
    }
    if !region.is_null() {
        let root_owed = f.owed.as_ref().and_then(|o| o.root.clone());
        if root_owed != strs(&region["root_owed"]) {
            bad.push("root's debts".to_string());
        }
        let frontier = f
            .owed
            .as_ref()
            .and_then(|o| o.frontier.clone())
            .map(|v| v.into_iter().map(|x| (x.facts, x.owed)).collect::<Vec<_>>());
        let theirs_frontier = region["frontier"].as_array().map(|a| {
            a.iter()
                .map(|x| {
                    (
                        strs(&x["facts"]).unwrap_or_default(),
                        strs(&x["owed"]).unwrap_or_default(),
                    )
                })
                .collect::<Vec<_>>()
        });
        if frontier != theirs_frontier {
            bad.push("the frontier".to_string());
        }
        let core = f.core.as_ref().map(|c| {
            (
                c.facts.clone(),
                c.sources.clone(),
                c.status.clone(),
                c.refuted_by.clone(),
            )
        });
        let theirs_core = (!region["core"].is_null()).then(|| {
            (
                strs(&region["core"]).unwrap_or_default(),
                region["core_sources"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .map(|x| x.as_str().map(str::to_string))
                    .collect::<Vec<_>>(),
                region["core_status"].as_str().unwrap_or("").to_string(),
                strs(&region["refuted_by"]).unwrap_or_default(),
            )
        });
        if core != theirs_core {
            bad.push(format!("the core {core:?}, the summary's {theirs_core:?}"));
        }
    }
    if f.work.saturations != s["work"]["saturations"].as_u64()
        || f.work.facts != s["work"]["facts"].as_u64()
    {
        bad.push(format!(
            "the work {:?} {:?}, the summary's {}",
            f.work.saturations, f.work.facts, s["work"]
        ));
    }
    bad
}

/// `ein solve FILE`'s exit for each corpus program that declares a bare
/// `solve` run, as the engine's own sweep banks it.
fn solve_exits() -> BTreeMap<String, i32> {
    let golden =
        ein_corpus::repo_root().join("ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt");
    let text = std::fs::read_to_string(&golden).expect("the corpus's exits");
    text.lines()
        .filter_map(|l| {
            let (code, rest) = l.split_once(char::is_whitespace)?;
            let path = rest.trim().strip_suffix(" :: solve")?;
            Some((path.to_string(), code.parse().ok()?))
        })
        .collect()
}

/// The outcomes a `(solve … :expect …)` allows over all of R, where it
/// holds.
fn allowed_by_solve_claim(shape: &str, states: Option<usize>) -> Option<BTreeSet<Outcome>> {
    use Outcome::*;
    Some(match (shape, states) {
        ("false", _) => [Inconsistent].into(),
        ("owing", _) => [Incomplete].into(),
        ("region", Some(0)) => [Inconsistent, Incomplete].into(),
        ("region", Some(1)) => [One].into(),
        ("region", _) => [Several].into(),
        _ => return None,
    })
}

/// The outcomes a query's `:expect` allows over its part, where it holds.
fn allowed_by_query_claim(classes: usize) -> BTreeSet<Outcome> {
    use Outcome::*;
    match classes {
        // A query's `(false)`: no class, which is an empty R of either kind.
        0 => [Inconsistent, Incomplete].into(),
        // One class: a state at least, and states can share a class.
        1 => [One, Several].into(),
        // Two classes need two states.
        _ => [Several].into(),
    }
}

#[test]
fn every_corpus_program_has_one_outcome_a_question_and_it_agrees_with_the_corpus() {
    let found = sweep();
    let exits = solve_exits();
    let mut bad: Vec<String> = Vec::new();
    let mut tally: BTreeMap<(String, Part), BTreeMap<Outcome, usize>> = BTreeMap::new();
    let mut claims_compared = 0usize;
    let mut claims_not_held = 0usize;
    let mut flagged_statically = 0usize;
    let mut refused = 0usize;

    for f in found {
        let of = |q: Question, p: Part| {
            f.objects
                .iter()
                .find(|(q2, _)| *q2 == q)
                .and_then(|(_, o)| o.outcome_over(p))
                .map(|r| r.outcome)
        };
        // One object a question, one outcome a reading, and no fault.
        for (q, o) in &f.objects {
            if let Some(fault) = &o.fault {
                bad.push(format!("{}: {} faulted: {fault:?}", f.path, q.flags()));
                continue;
            }
            let parts: Vec<Part> = o.outcome.iter().flatten().map(|r| r.part).collect();
            if parts != q.readings() {
                bad.push(format!("{}: {} reads {parts:?}", f.path, q.flags()));
            }
            for r in o.outcome.iter().flatten() {
                *tally
                    .entry((q.flags(), r.part))
                    .or_default()
                    .entry(r.outcome)
                    .or_default() += 1;
            }
        }
        let outcomes: BTreeSet<Outcome> = f
            .objects
            .iter()
            .flat_map(|(_, o)| o.outcome.iter().flatten().map(|r| r.outcome))
            .collect();
        let invalid = outcomes.contains(&Outcome::Invalid);
        refused += usize::from(invalid);
        // What the static check found: the sites of root alone.
        let statically = f
            .objects
            .iter()
            .find(|(q, _)| *q == Question::NoSearch)
            .and_then(|(_, o)| o.flagged.as_ref())
            .is_some_and(|x| !x.sites.is_empty());
        flagged_statically += usize::from(statically);

        // The group: a negative fixture is invalid under every question, a
        // parse-negative one with the parse's kinds alone; only a refused
        // load is invalid, and it is invalid under every question.
        let parse_kinds = ["parse", "not-ascii", "nesting-limit"];
        let kinds: BTreeSet<&str> = f
            .objects
            .iter()
            .flat_map(|(_, o)| o.diagnostics.iter())
            .filter(|_| invalid)
            .map(|d| d.kind.as_str())
            .collect();
        match f.group.as_str() {
            "parse-negative" | "load-negative"
                if outcomes != BTreeSet::from([Outcome::Invalid]) =>
            {
                bad.push(format!(
                    "{}: a {} entry reads {outcomes:?}",
                    f.path, f.group
                ));
            }
            "parse-negative" if !kinds.iter().all(|k| parse_kinds.contains(k)) => {
                bad.push(format!(
                    "{}: a parse-negative entry refused as {kinds:?}",
                    f.path
                ));
            }
            _ => {}
        }
        if invalid && outcomes.len() != 1 {
            bad.push(format!("{}: invalid, and reads {outcomes:?}", f.path));
        }

        // The engine's static check: a program it flags is flagged under
        // every question, and nothing else; one that reads flagged and is
        // not statically flagged was found so in flight, which the corpus
        // has no program of.
        if statically && outcomes != BTreeSet::from([Outcome::Flagged]) {
            bad.push(format!(
                "{}: the static check flags it, and it reads {outcomes:?}",
                f.path
            ));
        }
        if !statically && outcomes.contains(&Outcome::Flagged) {
            bad.push(format!(
                "{}: not flagged by the static check, and reads flagged (found in flight?)",
                f.path
            ));
        }

        // The exit `ein solve` gives it: invalid exits non-zero, and a
        // non-zero exit is invalid or a claim the run refuted.
        let describe = f
            .objects
            .iter()
            .find(|(q, _)| *q == Question::Describe)
            .map(|(_, o)| o);
        if let Some(&exit) = exits.get(&f.path) {
            let refuted = describe.is_some_and(|o| {
                o.claim.as_ref().and_then(|c| c.outcome.as_deref()) == Some("failed")
                    || o.queries.iter().flatten().any(|q| {
                        q.expect.as_ref().and_then(|e| e.outcome.as_deref()) == Some("failed")
                    })
            });
            if invalid && exit == 0 {
                bad.push(format!("{}: invalid, and `ein solve` exits 0", f.path));
            }
            if !invalid && exit != 0 && !refuted {
                bad.push(format!(
                    "{}: `ein solve` exits {exit}, and it is neither invalid nor refutes a claim",
                    f.path
                ));
            }
        }

        // Questions that read one part agree wherever both decided it: the
        // outcome is the program's, not the route's.
        let groups: [(Part, &[Question]); 3] = [
            (
                Part::All,
                &[
                    Question::Describe,
                    Question::Single(Part::All),
                    Question::Count,
                ],
            ),
            (
                Part::Minimal,
                &[
                    Question::Describe,
                    Question::Single(Part::Minimal),
                    Question::Line(Part::Minimal),
                ],
            ),
            (
                Part::Maximal,
                &[
                    Question::Describe,
                    Question::Single(Part::Maximal),
                    Question::Line(Part::Maximal),
                ],
            ),
        ];
        for (part, qs) in groups {
            let decided: BTreeSet<Outcome> = qs
                .iter()
                .filter_map(|&q| of(q, part))
                .filter(|o| *o != Outcome::NotDecided)
                .collect();
            if decided.len() > 1 {
                bad.push(format!(
                    "{}: {} the questions read {decided:?}",
                    f.path,
                    part.words()
                ));
            }
        }
        // R empty is R empty over every part, and `--no-search` says so of a
        // dead root alone.
        let read: Vec<Outcome> = f
            .objects
            .iter()
            .filter(|(q, _)| *q != Question::NoSearch)
            .flat_map(|(_, o)| o.outcome.iter().flatten().map(|r| r.outcome))
            .collect();
        let empty: BTreeSet<Outcome> = read
            .iter()
            .copied()
            .filter(|o| matches!(o, Outcome::Inconsistent | Outcome::Incomplete))
            .collect();
        let answers: BTreeSet<Outcome> = read.iter().copied().filter(|o| o.is_answer()).collect();
        if !empty.is_empty() && answers != empty {
            bad.push(format!(
                "{}: R is empty under one question and not another: {answers:?}",
                f.path
            ));
        }
        if of(Question::NoSearch, Part::All) == Some(Outcome::Inconsistent)
            && of(Question::Describe, Part::All) != Some(Outcome::Inconsistent)
        {
            bad.push(format!(
                "{}: --no-search reads a dead root, describe does not",
                f.path
            ));
        }

        // The claims the program states, where the kernel holds them on the
        // describing run: the outcome over the claim's part is one the
        // claim's shape allows. A flagged program is left out: nothing is
        // claimed of its answer.
        if outcomes.contains(&Outcome::Flagged) || invalid {
            continue;
        }
        let Some(d) = describe else { continue };
        if let Some(c) = &d.claim {
            if c.outcome.as_deref() == Some("held") {
                claims_compared += 1;
                let o = of(Question::Describe, Part::All).expect("describe's reading");
                if let Some(allowed) = allowed_by_solve_claim(&c.shape, c.states)
                    && o != Outcome::NotDecided
                    && !allowed.contains(&o)
                {
                    bad.push(format!(
                        "{}: (solve :expect …) {c:?} holds, and it reads {o:?}",
                        f.path
                    ));
                }
            } else {
                claims_not_held += 1;
            }
        }
        for q in d.queries.iter().flatten() {
            let Some(e) = &q.expect else { continue };
            if e.outcome.as_deref() != Some("held") {
                claims_not_held += 1;
                continue;
            }
            claims_compared += 1;
            let part = match q.over.as_str() {
                "minimal" => Part::Minimal,
                "maximal" => Part::Maximal,
                _ => Part::All,
            };
            let o = of(Question::Describe, part).expect("describe's reading");
            if o != Outcome::NotDecided && !allowed_by_query_claim(e.classes).contains(&o) {
                bad.push(format!(
                    "{}: (query :expect …) {e:?} holds, and it reads {o:?}",
                    f.path
                ));
            }
        }
    }

    eprintln!(
        "{} programs; {refused} refused; {flagged_statically} flagged by the static check; \
         {} with a bare `solve` exit compared; {claims_compared} claims compared, \
         {claims_not_held} not held on the describing run",
        found.len(),
        found.iter().filter(|f| exits.contains_key(&f.path)).count(),
    );
    for ((q, p), by) in &tally {
        let line: Vec<String> = Outcome::ALL
            .iter()
            .map(|o| format!("{} {}", o.word(), by.get(o).copied().unwrap_or(0)))
            .collect();
        eprintln!("  {q:<20} {:<22} {}", p.words(), line.join(" · "));
    }

    // Not vacuous: the sweep met every outcome, and compared claims.
    let met: BTreeSet<Outcome> = tally.values().flat_map(|by| by.keys().copied()).collect();
    assert_eq!(
        met,
        Outcome::ALL.into_iter().collect::<BTreeSet<_>>(),
        "the corpus sweep did not meet every outcome"
    );
    assert!(claims_compared > 0, "no claim was compared");
    assert!(found.len() > 800, "the sweep read {} programs", found.len());
    assert!(
        bad.is_empty(),
        "{} findings:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **Every object comes back as itself through JSON**, under every question.
#[test]
fn every_object_survives_a_round_trip_through_json() {
    let bad: Vec<String> = sweep()
        .iter()
        .flat_map(|f| f.round_trip.iter().map(move |e| format!("{}: {e}", f.path)))
        .collect();
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// **What the object says is what the binary says of the same run**, under
/// each question the entry declares: the outcome against the summary's
/// verdict, `region.empty`, `region.exhausted` and `monotonicity`; the
/// readings, the states, the queries' classes and claims, the program's
/// claim, what is owed, the core and the work against their keys; the
/// diagnostics against the summary's, one for one.
#[test]
fn every_object_says_what_the_binary_says_of_the_same_run() {
    let found = sweep();
    let mut runs = 0usize;
    let mut by_question: BTreeMap<String, usize> = BTreeMap::new();
    let mut bad: Vec<String> = Vec::new();
    for f in found {
        for (label, disagreements) in &f.compared {
            runs += 1;
            *by_question.entry(label.trim().to_string()).or_default() += 1;
            for d in disagreements {
                bad.push(format!("{} [{}]: {d}", f.path, label.trim()));
            }
        }
    }
    let mut held: BTreeMap<&'static str, usize> = BTreeMap::new();
    for f in found {
        for (k, n) in &f.held {
            *held.entry(k).or_default() += n;
        }
    }
    eprintln!(
        "{runs} runs compared with the binary, by question: {by_question:?}; \
         the objects that held each part: {held:?}"
    );
    assert!(runs > 900, "only {runs} runs were compared");
    for part in [
        "states",
        "classes",
        "claim",
        "core",
        "frontier",
        "diagnostics",
        "flagged",
    ] {
        assert!(
            held.get(part).copied().unwrap_or(0) > 0,
            "no object held {part}"
        );
    }
    assert!(
        bad.is_empty(),
        "{} disagreements:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// The sweep reads `corpus/corpus.toml`'s entries through the engine's own
/// manifest reader: a path the manifest names is a file.
#[test]
fn the_corpus_entries_are_files() {
    let root = ein_corpus::repo_root();
    for e in ein_corpus::corpus().entry {
        assert!(Path::new(&root.join(&e.path)).is_file(), "{}", e.path);
    }
}
