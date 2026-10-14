//! M1e2 S1e2.1.1 — **the wrong-answer register, as a test.**
//!
//! `corpus/wrong_answers.toml` holds one row per witness of every known wrong
//! answer: what the engine says today and what it should say. This file runs
//! every run a row records and holds the engine to it, so that the register is
//! a test from the day it exists:
//!
//! - an **open** row must still answer `wrong`, and its controls and twins what
//!   they record — an open case that changes answer is a fix nobody intended,
//!   or a new wrong answer, and either one is news;
//! - a **fixed** row's wrong run must answer `right`, every field `right`
//!   states;
//! - a **not-wrong** row must still answer `wrong`, which the review ruled is
//!   right.
//!
//! **Through the binary, as a process** — the stage said in-process, and named
//! `corpus_cli.rs` as the model, which is a process sweep. A row is a claim
//! about what `ein <run>` answers, and two of them need what only a process
//! has: `EIN_TRAVERSAL=tree` is read from the environment by `ein-infer` at
//! solve time, which an in-process test could set only for every test in the
//! binary at once, and W7's debug abort is a `panic!` that would end this test
//! rather than be observed by it. What is compared is the `--json-summary` the
//! run writes, which is the same object stdout prints and the one the review
//! bundle reads.
//!
//! The id-order half of a W11 row is not here: it needs a permuted interner,
//! which `ein-render/tests/id_order_invariance.rs` owns and this crate cannot
//! reach, so that file reads the same rows for that half.
//!
//! ```text
//! cargo test -p ein-cli --test wrong_answers
//! ```

use std::path::Path;
use std::process::Command;

use ein_corpus::register::{Answer, Right, Row, project};
use ein_corpus::{plan, register, repo_root};
use serde_json::Value as J;

/// What one run left behind.
struct Got {
    code: i32,
    stderr: String,
    summary: Option<J>,
}

fn run(row: &Row, a: &Answer, dir: &Path) -> Got {
    let witness = row.witness_of(a);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(plan::argv(&a.run, witness, dir))
        .current_dir(repo_root());
    for kv in &a.env {
        let (k, v) = kv
            .split_once('=')
            .unwrap_or_else(|| panic!("{}: env {kv:?} is not NAME=value", row.id));
        cmd.env(k, v);
    }
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("{}: {witness}: {e}", row.id));
    let summary = std::fs::read_to_string(dir.join("summary.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("summary.json parses"));
    Got {
        code: out.status.code().unwrap_or(-1),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
    }
}

/// The recorded states of one kind (`solutions` or `open_states`), projected
/// onto `closes` and sorted — the register's spelling, compared as a multiset.
fn states(s: &J, kind: &str, closes: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = s["verdict"][kind]
        .as_array()
        .map(|v| {
            v.iter()
                .map(|m| {
                    let facts = m["facts"].as_array().expect("facts");
                    project(facts.iter().map(|f| f.as_str().expect("fact")), closes)
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

fn sorted(v: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut v: Vec<Vec<String>> = v
        .iter()
        .map(|m| {
            let mut m = m.clone();
            m.sort();
            m
        })
        .collect();
    v.sort();
    v
}

/// The fields every answer and every right answer can state, read off a run.
/// `want` fields left `None` are not compared.
struct Claim<'a> {
    verdict: Option<&'a str>,
    k: Option<usize>,
    exhausted: Option<bool>,
    core: Option<&'a [String]>,
    closes: &'a [String],
    models: Option<&'a [Vec<String>]>,
    open: Option<&'a [Vec<String>]>,
    owes: Option<&'a [u64]>,
}

impl<'a> From<&'a Answer> for Claim<'a> {
    fn from(a: &'a Answer) -> Self {
        Claim {
            verdict: Some(&a.verdict),
            k: Some(a.k),
            exhausted: Some(a.exhausted),
            core: a.core.as_deref(),
            closes: &a.closes,
            models: a.models.as_deref(),
            open: a.open.as_deref(),
            owes: a.owes.as_deref(),
        }
    }
}

impl<'a> From<&'a Right> for Claim<'a> {
    fn from(r: &'a Right) -> Self {
        Claim {
            verdict: r.verdict.as_deref(),
            k: r.k,
            exhausted: r.exhausted,
            core: r.core.as_deref(),
            closes: &r.closes,
            models: r.models.as_deref(),
            open: r.open.as_deref(),
            owes: r.owes.as_deref(),
        }
    }
}

/// Every field where the run disagrees with the claim, as `field: recorded …,
/// got …`.
fn differences(want: &Claim, s: &J) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = |name: &str, recorded: String, got: String| {
        if recorded != got {
            out.push(format!("{name}: recorded {recorded}, got {got}"));
        }
    };
    let v = &s["verdict"];
    if let Some(w) = want.verdict {
        field(
            "verdict",
            w.to_string(),
            v["type"].as_str().unwrap_or("?").to_string(),
        );
    }
    if let Some(w) = want.k {
        field("k", w.to_string(), v["k"].to_string());
    }
    if let Some(w) = want.exhausted {
        field("exhausted", w.to_string(), v["exhausted"].to_string());
    }
    if let Some(w) = want.core {
        let mut got: Vec<String> = v["unsat_core"]
            .as_array()
            .map(|c| {
                c.iter()
                    .map(|f| f.as_str().unwrap_or("?").to_string())
                    .collect()
            })
            .unwrap_or_default();
        got.sort();
        let mut w = w.to_vec();
        w.sort();
        field("core", format!("{w:?}"), format!("{got:?}"));
    }
    if let Some(w) = want.models {
        field(
            "models",
            format!("{:?}", sorted(w)),
            format!("{:?}", states(s, "solutions", want.closes)),
        );
    }
    if let Some(w) = want.open {
        field(
            "open states",
            format!("{:?}", sorted(w)),
            format!("{:?}", states(s, "open_states", want.closes)),
        );
    }
    if let Some(w) = want.owes {
        let got: Vec<u64> = s["owes"]["models"]
            .as_array()
            .map(|m| m.iter().map(|o| o["total"].as_u64().unwrap_or(0)).collect())
            .unwrap_or_default();
        field("owes", format!("{w:?}"), format!("{got:?}"));
    }
    out
}

/// One run against one claim, with the exit code and the debug abort.
fn check(want: &Claim, exit: i32, debug_panic: Option<&str>, got: &Got) -> Vec<String> {
    if let Some(text) = debug_panic.filter(|_| cfg!(debug_assertions)) {
        return if got.code == 101 && got.stderr.contains(text) {
            Vec::new()
        } else {
            vec![format!(
                "recorded a debug-build abort ({text:?}), got exit {} — stderr: {}",
                got.code,
                got.stderr
                    .lines()
                    .find(|l| !l.is_empty())
                    .unwrap_or("<empty>")
            )]
        };
    }
    let mut out = Vec::new();
    if got.code != exit {
        out.push(format!(
            "exit: recorded {exit}, got {} — stderr: {}",
            got.code,
            got.stderr
                .lines()
                .find(|l| !l.is_empty())
                .unwrap_or("<empty>")
        ));
    }
    match &got.summary {
        Some(s) => out.extend(differences(want, s)),
        None => out.push("no summary.json was written".to_string()),
    }
    out
}

#[test]
fn every_row_answers_what_the_register_records() {
    let reg = register();
    let scratch = std::env::temp_dir().join(format!("ein-wrong-answers-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let mut bad = Vec::new();
    let mut runs = 0usize;

    for row in &reg.case {
        if row.status.starts_with("merged:") {
            continue;
        }
        for (label, a) in row.answers() {
            let dir = scratch.join(plan::slug(&row.id)).join(plan::slug(&label));
            std::fs::create_dir_all(&dir).expect("scratch dir");
            let got = run(row, a, &dir);
            runs += 1;
            // A fixed row's wrong run is held to the right answer; everything
            // else to what it records.
            let (want, exit, debug_panic, against) = if row.status == "fixed" && label == "wrong" {
                (Claim::from(&row.right), 0, None, "right")
            } else {
                (Claim::from(a), a.exit, a.debug_panic.as_deref(), "recorded")
            };
            let diffs = check(&want, exit, debug_panic, &got);
            if diffs.is_empty() {
                continue;
            }
            let why = match (row.status.as_str(), label.as_str()) {
                ("open", "wrong") => {
                    "an OPEN case changed answer — a fix nobody intended, or a new wrong \
                     answer; either way the row moves in the same commit"
                }
                ("fixed", "wrong") => "a FIXED case no longer answers right",
                ("not-wrong", "wrong") => "the answer the review ruled right has changed",
                _ => "a control or twin changed answer — the case page would now be wrong",
            };
            bad.push(format!(
                "{} [{label}] ein {} {}{} — {against} answer not given; {why}:\n    {}",
                row.id,
                a.run,
                row.witness_of(a),
                if a.env.is_empty() {
                    String::new()
                } else {
                    format!(" (with {})", a.env.join(" "))
                },
                diffs.join("\n    ")
            ));
        }
    }
    let _ = std::fs::remove_dir_all(&scratch);
    assert!(runs > 0, "the register recorded no runs");
    assert!(
        bad.is_empty(),
        "{} of {runs} register runs disagree with corpus/wrong_answers.toml:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
}
