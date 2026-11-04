//! M1e2 S1e2.1.1 — **the wrong-answer register, as a test.**
//!
//! `corpus/wrong_answers.toml` holds one row per witness of every known wrong
//! answer: what the engine says and what it should say. This file runs what a
//! row records and holds the engine to it, so that the register is a test from
//! the day it exists.
//!
//! **Since M1e2 S1e2.1a.5's switch what is run is each row's `[case.now]`**:
//! the region run that asks the row's question (`solve --maximal` for a
//! model-set row), compared field for field with the `--json-summary` it
//! writes (`ein-summary/2`). The walk's `wrong`, `control` and `twin` blocks
//! are the record of what the walk said, and no command runs the walk. What
//! a row's status means is data, held without a run by `ein-corpus`'s
//! `the_register_is_well_formed`:
//!
//! - a **fixed** row's `now` answers its `right`, every field `right` states;
//! - an **open** row says why its `now` is still not `right` — what R shares
//!   with the case, or the reading the case waits on.
//!
//! **Since M1e2 S1e2.2.5 a `now` can be a refusal** (*forbid*): the run exits
//! 1, its stderr's first line is the recorded one, and it writes no summary.
//!
//! So this file's one job is that `now` is what the binary answers. A row
//! whose run changes answer is news either way: a fixed row breaking, or an
//! open one changing, which is a fix nobody intended or a new wrong answer.
//!
//! **Through the binary, as a process**, as `corpus_cli.rs` is: a row is a
//! claim about what `ein <run>` answers.
//!
//! ```text
//! cargo test -p ein-cli --test wrong_answers
//! ```

use std::path::Path;
use std::process::Command;

use ein_corpus::register::{Now, Row};
use ein_corpus::{Scratch, plan, register, repo_root};
use serde_json::Value as J;

/// What one run left behind: its exit code, stdout, stderr, and the summary.
struct Got {
    code: i32,
    stdout: String,
    stderr: String,
    summary: Option<J>,
}

fn run(row: &Row, dir: &Path) -> Got {
    let mut argv = plan::argv(&row.now.run, &row.witness, dir);
    // `-H` lists H, whose relations are what a state's `(model …)` names.
    argv.push("-H".to_string());
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(&argv)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|e| panic!("{}: {}: {e}", row.id, row.witness));
    let summary = std::fs::read_to_string(dir.join("summary.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("summary.json parses"));
    Got {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
    }
}

/// The relations `-H` lists, sorted.
fn relations(stdout: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut on = false;
    for l in stdout.lines() {
        if l.starts_with("H — the universe") {
            on = true;
            continue;
        }
        if !on {
            continue;
        }
        if let Some(rest) = l.strip_prefix("  ")
            && !rest.starts_with(' ')
        {
            let mut words = rest.split_whitespace();
            match (words.next(), words.next()) {
                (Some(rel), Some(n)) if n.parse::<u64>().is_ok() => out.push(rel.to_string()),
                _ => on = false,
            }
        } else if !l.starts_with("    ") {
            on = false;
        }
    }
    out.sort();
    out.dedup();
    out
}

/// What the summary says, in `[case.now]`'s fields.
fn now_of(row: &Row, got: &Got) -> Result<Now, String> {
    let s = got.summary.as_ref().ok_or("no summary.json was written")?;
    let region = &s["region"];
    let states: Vec<&J> = region["states"]
        .as_array()
        .map(|v| v.iter().filter(|x| x["listed"] == true).collect())
        .unwrap_or_default();
    let facts = |x: &J| -> Vec<String> {
        let mut v: Vec<String> = x["facts"]
            .as_array()
            .map(|f| {
                f.iter()
                    .filter_map(|f| f.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        v.sort();
        v
    };
    let empty = region["empty"].as_str().map(str::to_string);
    let mut models: Vec<Vec<String>> = if states.is_empty() && empty.as_deref() == Some("owing") {
        region["frontier"]
            .as_array()
            .map(|f| f.iter().map(facts).collect())
            .unwrap_or_default()
    } else {
        states.iter().map(|x| facts(x)).collect()
    };
    models.sort();
    let asked = &s["asked"];
    let line = asked["which"].as_str().unwrap_or("?").to_string();
    // What the universe leaves out (M1e2 S1e2.2.8): on every run.
    let mut outside: Vec<String> = s["root"]["hypgen"]["outside"]
        .as_array()
        .ok_or("no root.hypgen.outside")?
        .iter()
        .filter_map(|o| o["relation"].as_str().map(str::to_string))
        .collect();
    outside.sort();
    // The core, where the row records one (M1e2 S1e2.2.11, W10).
    let core = row.now.core.as_ref().map(|_| {
        let mut c: Vec<String> = region["core"]
            .as_array()
            .map(|v| {
                v.iter()
                    .filter_map(|x| x.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        c.sort();
        c
    });
    Ok(Now {
        run: row.now.run.clone(),
        line,
        refused: None,
        error: None,
        exhausted: region["exhausted"] == true,
        k: states.len(),
        empty,
        relations: relations(&got.stdout),
        models,
        outside,
        core,
    })
}

/// Every field where the run disagrees with the row's `[case.now]`.
fn differences(want: &Now, got: &Now) -> Vec<String> {
    let mut out = Vec::new();
    let mut field = |name: &str, recorded: String, now: String| {
        if recorded != now {
            out.push(format!("{name}: recorded {recorded}, got {now}"));
        }
    };
    field("line", want.line.clone(), got.line.clone());
    field(
        "exhausted",
        want.exhausted.to_string(),
        got.exhausted.to_string(),
    );
    field("k", want.k.to_string(), got.k.to_string());
    field(
        "empty",
        format!("{:?}", want.empty),
        format!("{:?}", got.empty),
    );
    field(
        "relations",
        format!("{:?}", want.relations),
        format!("{:?}", got.relations),
    );
    let mut w = want.models.clone();
    w.iter_mut().for_each(|m| m.sort());
    w.sort();
    field("models", format!("{w:?}"), format!("{:?}", got.models));
    field(
        "outside",
        format!("{:?}", want.outside),
        format!("{:?}", got.outside),
    );
    field(
        "core",
        format!("{:?}", want.core),
        format!("{:?}", got.core),
    );
    out
}

#[test]
fn every_row_answers_what_the_register_records() {
    let reg = register();
    let scratch = Scratch::new("ein-wrong-answers");
    let mut bad = Vec::new();
    let mut runs = 0usize;

    for row in &reg.case {
        if row.status.starts_with("merged:") {
            continue;
        }
        let dir = scratch.join(plan::slug(&row.id));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let got = run(row, &dir);
        runs += 1;
        let first = got
            .stderr
            .lines()
            .find(|l| !l.is_empty())
            .unwrap_or("<empty>");
        // A refusal (M1e2 S1e2.2.5, *forbid*) is exit 1 and its first line,
        // and no summary; so is a run that ended in an error before it
        // answered (M1e2 S1e2.2.18, W18).
        let ended = match (&row.now.refused, &row.now.error) {
            (Some(want), _) => Some(("refused", "a refusal", want)),
            (None, Some(want)) => Some(("error", "an error", want)),
            (None, None) => None,
        };
        let diffs = if let Some((field, what, want)) = ended {
            let mut d = Vec::new();
            if got.code != 1 {
                d.push(format!("exit: recorded 1, {what}, got {}", got.code));
            }
            if first != want {
                d.push(format!("{field}: recorded {want:?}, got {first:?}"));
            }
            if got.summary.is_some() {
                d.push(format!("{what} wrote a summary"));
            }
            d
        } else if got.code != 0 {
            vec![format!(
                "exit: recorded 0, got {} — stderr: {first}",
                got.code
            )]
        } else {
            match now_of(row, &got) {
                Ok(now) => {
                    let mut d = differences(&row.now, &now);
                    // A right that names what the universe leaves out states
                    // the sentence too (M1e2 S1e2.2.8, W6): the `outside` row.
                    if row.right.outside.is_some()
                        && let Some(q) = &row.right.qualifier
                        && !got.stdout.contains(&format!("    outside         {q}\n"))
                    {
                        d.push(format!("the read-out does not print {q:?}"));
                    }
                    d
                }
                Err(e) => vec![e],
            }
        };
        if diffs.is_empty() {
            continue;
        }
        let why = match row.status.as_str() {
            "fixed" => "a FIXED row no longer answers what it records, which is its right answer",
            "open" => {
                "an OPEN row changed answer — a fix nobody intended, or a new wrong answer; \
                 either way the row moves in the same commit"
            }
            _ => "the answer the review ruled right has changed",
        };
        bad.push(format!(
            "{} ein {} {} — {why}:\n    {}",
            row.id,
            row.now.run,
            row.witness,
            diffs.join("\n    ")
        ));
    }
    assert!(runs > 0, "the register recorded no runs");
    assert!(
        bad.is_empty(),
        "{} of {runs} register runs disagree with corpus/wrong_answers.toml:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
}
