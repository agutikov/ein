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
//! **Since M1e2 S1e2.2.19 a `now` can record `N`'s cores** (schema /8, W19):
//! a dead node's core is not in the summary, whose `region.core` is root's,
//! so the run also writes `--dump-answer`, and its `nogoods.json` is read.
//!
//! **Since M1e2 S1e2.2.23 a `now` can record the status beside root's core**
//! (schema /9, W23): the summary's `region.core_status`, and where `right`
//! states it with a `qualifier`, the read-out's `core search` row.
//!
//! **Since M1e2 S1e2.2.25 a `now` can record how many members of R hold a
//! stale fact** (schema /10, W8's rows past the oracle's limit): the
//! summary's `monotonicity.state`. It can also compare `N` on the sets it
//! lists alone (`nogoods_listed`, `W10/second-preparation`'s `zebra2.ein`,
//! whose `N` is 366 sets).
//!
//! **Since M1e2 S1e2.2.26 a `now` can record the members it met and does not
//! list** (schema /11, W27's rows): the summary's states with `listed =
//! false`, what a line mode past `--region-limit` leaves to `--dump-answer`.
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

use ein_corpus::register::{Nogood, Now, Row, Stale, sorted_nogoods};
use ein_corpus::{Scratch, plan, register, repo_root};
use serde_json::Value as J;

/// What one run left behind: its exit code, stdout, stderr, and the summary.
struct Got {
    code: i32,
    stdout: String,
    stderr: String,
    summary: Option<J>,
    /// `--dump-answer`'s `nogoods.json`, where the row records `N`'s cores.
    nogoods: Option<J>,
}

fn run(row: &Row, dir: &Path) -> Got {
    let mut argv = plan::argv(&row.now.run, &row.witness, dir);
    // `-H` lists H, whose relations are what a state's `(model …)` names.
    argv.push("-H".to_string());
    // A dead node's core is the answer directory's (M1e2 S1e2.2.19, W19).
    let answer = dir.join("answer");
    if row.now.nogoods.is_some() {
        argv.push("--dump-answer".to_string());
        argv.push(answer.display().to_string());
    }
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(&argv)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|e| panic!("{}: {}: {e}", row.id, row.witness));
    let summary = std::fs::read_to_string(dir.join("summary.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("summary.json parses"));
    let nogoods = std::fs::read_to_string(answer.join("nogoods.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("nogoods.json parses"));
    Got {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
        nogoods,
    }
}

/// `nogoods.json`'s sets and cores, sorted as the register compares them.
fn nogoods_of(j: &J) -> Vec<Nogood> {
    let strs = |x: &J| -> Vec<String> {
        x.as_array()
            .map(|v| {
                v.iter()
                    .filter_map(|s| s.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    };
    let n: Vec<Nogood> = j
        .as_array()
        .map(|v| {
            v.iter()
                .map(|x| Nogood {
                    set: strs(&x["set"]),
                    core: strs(&x["core"]),
                })
                .collect()
        })
        .unwrap_or_default();
    sorted_nogoods(&n)
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
    // The core's status, where the row records one (M1e2 S1e2.2.23, W23):
    // `none` where the summary states none.
    let core_status = row.now.core_status.as_ref().map(|_| {
        region
            .get("core_status")
            .and_then(J::as_str)
            .unwrap_or("none")
            .to_string()
    });
    // N's cores, where the row records them (M1e2 S1e2.2.19, W19); on the
    // sets the row lists alone where it says so (schema /10, M1e2
    // S1e2.2.25), each of which the run must report.
    let nogoods = match (&row.now.nogoods, &got.nogoods) {
        (None, _) => None,
        (Some(want), Some(j)) if row.now.nogoods_listed => {
            let all = nogoods_of(j);
            let mut kept = Vec::new();
            for w in sorted_nogoods(want) {
                match all.iter().find(|g| g.set == w.set) {
                    Some(g) => kept.push(g.clone()),
                    None => return Err(format!("N has no member {:?}", w.set)),
                }
            }
            Some(sorted_nogoods(&kept))
        }
        (Some(_), Some(j)) => Some(nogoods_of(j)),
        (Some(_), None) => return Err("no nogoods.json was written".to_string()),
    };
    // Stale members of R, where the row records them (schema /10, M1e2
    // S1e2.2.25): the summary's `monotonicity.state`, its `stale` entries of
    // class `R` and its `nodes`.
    let stale = row.now.stale.as_ref().map(|_| {
        let st = &s["monotonicity"]["state"];
        Stale {
            in_r: st["stale"]
                .as_array()
                .map_or(0, |v| v.iter().filter(|x| x["class"] == "R").count() as u64),
            nodes: st["nodes"].as_u64().unwrap_or(0),
        }
    });
    // The members met and not listed, where the row records them (schema
    // /11, M1e2 S1e2.2.26, W27): the summary's states with `listed = false`.
    let others = row.now.others.map(|_| {
        region["states"].as_array().map_or(0, |v| {
            v.iter().filter(|x| x["listed"] == false).count() as u64
        })
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
        nogoods,
        core_status,
        stale,
        nogoods_listed: row.now.nogoods_listed,
        others,
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
    field(
        "nogoods",
        format!("{:?}", want.nogoods.as_deref().map(sorted_nogoods)),
        format!("{:?}", got.nogoods),
    );
    field(
        "core_status",
        format!("{:?}", want.core_status),
        format!("{:?}", got.core_status),
    );
    field(
        "stale",
        format!("{:?}", want.stale),
        format!("{:?}", got.stale),
    );
    field(
        "others",
        format!("{:?}", want.others),
        format!("{:?}", got.others),
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
                    // the sentence too (M1e2 S1e2.2.8, W6): the `outside` row,
                    // wherever `now` records that `outside` — an open row
                    // whose read-out names nothing yet (M1e2 S1e2.2.22,
                    // `W22/kw-neq-in-goal`) is open on that field.
                    if row.right.outside.as_ref() == Some(&row.now.outside)
                        && let Some(q) = &row.right.qualifier
                        && !got.stdout.contains(&format!("    outside         {q}\n"))
                    {
                        d.push(format!("the read-out does not print {q:?}"));
                    }
                    // A right that states a core's status states the read-out's
                    // `core search` row too (M1e2 S1e2.2.23, W23), wherever
                    // `now` records that status.
                    if row.right.core_status.is_some()
                        && row.right.core_status == row.now.core_status
                        && let Some(q) = &row.right.qualifier
                        && !got.stdout.contains(&format!("  core search       {q}\n"))
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
