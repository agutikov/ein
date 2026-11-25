//! **A row's run, through the binary.**
//!
//! A file of the `ein_corpus::regressions` format records, for each row, what
//! `ein <run>` answers on its witness, `[case.now]`. [`hold`] runs it,
//! through the binary as a process, and compares what it answers with what
//! the row records, field for field with the `--json-summary` it writes
//! (`ein-summary/2`). So a row whose run changes answer is news either way: a
//! row answered right breaking, or one answered wrong changing, which is a
//! fix nobody intended or a new wrong answer.
//!
//! - **A refusal** exits 1, its stderr's first line is the recorded one, and
//!   its summary has no answer: every answer key `null`, and its diagnostics
//!   the refusal's findings (Q-M2.9); a run that ends in an engine's error
//!   before it answered writes none. A solve's usage refusal exits 2.
//! - **N's cores**: a dead node's core is not in the summary, whose
//!   `region.core` is root's, so the run also writes `--dump-answer`, and its
//!   `nogoods.json` is read.
//! - **A core's status**: the summary's `region.core_status`, and where
//!   `right` states it with a `qualifier`, the read-out's `core search` row.
//! - **A stale count**: the summary's `monotonicity.state`. `N` can be
//!   compared on the sets a row lists alone (`nogoods_listed`).
//! - **The members a line mode met and does not list**: the summary's states
//!   with `listed = false`.
//! - **A claim's outcome and a query's classes**: the summary's
//!   `claim.outcome`, and each query's class count and `expect.outcome`. A
//!   row whose recorded claim is `failed` expects the run's exit 1.
//! - **What root's `refuted by` names**: the summary's `region.refuted_by`, in
//!   the order the read-out prints it.
//! - **What a query's `:goal-text` prints**: the read-out's `“…”` sentences,
//!   read off stdout, since the summary carries the rows and not the
//!   sentences.
//! - **A command's run that is not a solve** writes no summary: its answer is
//!   a refusal, an error or that it answered, exit 0 with nothing on stderr.
//!   A `container` row saves the witness with `ein kb save` first and runs on
//!   the `.einb`; one that answered prints what the same run prints on the
//!   witness itself, every number a line times masked and the lines that say
//!   what the run read aside. `{FILE}` in a recorded line is the path the
//!   run was handed, and `{out}` the run's scratch directory.
//! - **A line the run prints**: the first line that begins, past its indent,
//!   with a label, on stdout, in `--dump-answer`'s `answer.txt` or in a state
//!   file it writes.
//! - **A run with its stdout closed**: a pipe whose reader was dropped before
//!   the run was spawned, so every write the run makes meets no reader,
//!   whatever the timing. Its answer is how it ended: by a signal, `SIGPIPE`,
//!   with nothing on stderr, or by a panic, exit 101, whose message is the
//!   line after `thread 'main' (…) panicked at …:`.
//!
//! A row's `[case.region]` is [`regions`]'s, through the library.
//!
//! `tests/regressions.rs` holds the project's file by both, the binary the one
//! Cargo built; the example `held_answers` holds a file given on its command
//! line, with the binary given beside it.

#![allow(dead_code)]

pub mod regions;

use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

use ein_corpus::regressions::{Nogood, Now, Printed, QueryAnswer, Row, Stale, sorted_nogoods};
use ein_corpus::{plan, repo_root};
use serde_json::Value as J;

/// What one run left behind: its exit code, stdout, stderr, and the summary.
struct Got {
    /// The path the run was handed: the witness, or the container saved from
    /// it.
    file: String,
    code: i32,
    /// The signal that ended the run, where one did.
    signal: Option<i32>,
    stdout: String,
    stderr: String,
    summary: Option<J>,
    /// `--dump-answer`'s `nogoods.json`, where the row records `N`'s cores.
    nogoods: Option<J>,
    /// `--dump-answer`'s directory, where the row records a line of its
    /// `answer.txt` or of a state file it writes.
    answer: std::path::PathBuf,
}

fn run(ein: &Path, row: &Row, dir: &Path) -> Got {
    // A container row reads the witness saved as a `.einb`.
    let file = if row.now.container {
        let stem = Path::new(&row.witness)
            .file_stem()
            .expect("a witness is a file")
            .to_string_lossy()
            .into_owned();
        let einb = dir.join(format!("{stem}.einb"));
        let saved = Command::new(ein)
            .args(["kb", "save", &row.witness])
            .arg(&einb)
            .current_dir(repo_root())
            .output()
            .unwrap_or_else(|e| panic!("{}: kb save: {e}", row.id));
        assert!(
            saved.status.success(),
            "{}: `ein kb save {}` failed: {}",
            row.id,
            row.witness,
            String::from_utf8_lossy(&saved.stderr)
        );
        einb.display().to_string()
    } else {
        row.witness.clone()
    };
    let mut argv = plan::argv(&row.now.run, &file, dir);
    // `-H` lists H, whose relations are what a state's `(model …)` names; a
    // command's run lists no state.
    if !row.now.is_command() {
        argv.push("-H".to_string());
    }
    // A dead node's core is the answer directory's, and so is
    // `answer.txt`.
    let answer = dir.join("answer");
    let reads_answer = row.now.printed.iter().flatten().any(|p| p.file != "stdout");
    if row.now.nogoods.is_some() || reads_answer {
        argv.push("--dump-answer".to_string());
        argv.push(answer.display().to_string());
    }
    // A row that records `N` reads it off a describe that computes it: the
    // state route, which `--describe-route states` asks, describe's default
    // computing no `N` where its tree branches. A mode's run takes no route
    // option.
    let describes = argv.first().map(String::as_str) == Some("solve")
        && !argv.iter().any(|a| {
            matches!(
                a.as_str(),
                "-n" | "--solutions"
                    | "--single"
                    | "--minimal"
                    | "--maximal"
                    | "--monotone"
                    | "--no-search"
            )
        });
    if row.now.nogoods.is_some() && describes {
        argv.push("--describe-route".to_string());
        argv.push("states".to_string());
    }
    let mut cmd = Command::new(ein);
    cmd.args(&argv).current_dir(repo_root());
    // A closed stdout: a pipe whose reader is dropped before the
    // run is spawned, so every write the run makes is one after its reader
    // has gone, whatever the timing.
    if row.now.stdout_closed() {
        let (reader, writer) = std::io::pipe().expect("a pipe");
        drop(reader);
        cmd.stdout(Stdio::from(writer));
    }
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("{}: {}: {e}", row.id, row.witness));
    // `cmd` holds the pipe's writer until it is dropped; the run has ended.
    drop(cmd);
    let summary = std::fs::read_to_string(dir.join("summary.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("summary.json parses"));
    let nogoods = std::fs::read_to_string(answer.join("nogoods.json"))
        .ok()
        .map(|s| serde_json::from_str(&s).expect("nogoods.json parses"));
    Got {
        file,
        code: out.status.code().unwrap_or(-1),
        signal: signal_of(out.status),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
        nogoods,
        answer,
    }
}

/// The signal that ended a run, where one did.
#[cfg(unix)]
fn signal_of(status: ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.signal()
}

#[cfg(not(unix))]
fn signal_of(_: ExitStatus) -> Option<i32> {
    None
}

/// A panic's message: the line of stderr after `thread 'main'
/// (…) panicked at …:`, whose thread id and source position are the
/// toolchain's.
fn panic_message(stderr: &str) -> Option<&str> {
    let mut lines = stderr.lines();
    lines.find(|l| l.contains(" panicked at "))?;
    lines.next()
}

/// How a run whose stdout is closed ended:
/// by the signal `now` records, with nothing on stderr, or by the panic it
/// records, exit 101.
fn closed(row: &Row, got: &Got) -> Vec<String> {
    let mut d = Vec::new();
    let ended = match (got.signal, got.code) {
        (Some(n), _) => format!("signal {n}"),
        (None, c) => format!("exit {c}"),
    };
    match (row.now.signal, &row.now.panicked) {
        (Some(want), _) => {
            if got.signal != Some(want) {
                d.push(format!(
                    "recorded: ended by signal {want}, got {ended} — stderr: {}",
                    got.stderr
                        .lines()
                        .find(|l| !l.is_empty())
                        .unwrap_or("<empty>")
                ));
            }
            if !got.stderr.is_empty() {
                d.push(format!("the run wrote stderr: {:?}", got.stderr));
            }
        }
        (None, Some(want)) => {
            if got.code != 101 || got.signal.is_some() {
                d.push(format!("recorded: a panic, exit 101, got {ended}"));
            }
            let said = panic_message(&got.stderr);
            if said != Some(want.as_str()) {
                d.push(format!("panicked: recorded {want:?}, got {said:?}"));
            }
        }
        (None, None) => d.push("a [now] whose stdout is closed records how it ended".to_string()),
    }
    d
}

/// A command's stdout as two runs of it are compared: the lines
/// that say what the run read (`input:`, its size, `parse:`, `kb load:`,
/// `open:`) aside, every number a line times (`… ms`) masked, and runs of
/// spaces one, since a masked number's width is the format's.
fn comparable(stdout: &str) -> String {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let t = line.trim_start();
        let size = t.ends_with(" chars") || t.ends_with(" bytes, a .einb container");
        if ["input:", "parse:", "kb load:", "open:"]
            .iter()
            .any(|p| t.starts_with(p))
            || (size && t.starts_with(|c: char| c.is_ascii_digit()))
        {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let masked: Vec<&str> = words
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let timed = words
                    .get(i + 1)
                    .is_some_and(|n| matches!(*n, "ms" | "s" | "s)"));
                let number = w
                    .trim_start_matches("Δ=")
                    .trim_start_matches('(')
                    .parse::<f64>()
                    .is_ok();
                if timed && number { "#" } else { w }
            })
            .collect();
        out.push(masked.join(" "));
    }
    out.join("\n")
}

/// What a command's `now` answers when it answered: exit 0, an
/// empty stderr, and with `container` the same run's stdout on the witness.
fn answered(ein: &Path, row: &Row, dir: &Path, got: &Got) -> Vec<String> {
    let mut d = Vec::new();
    if got.code != 0 {
        d.push(format!(
            "exit: recorded 0, the command answers, got {} — stderr: {}",
            got.code,
            got.stderr.lines().next().unwrap_or("<empty>")
        ));
        return d;
    }
    if !got.stderr.trim().is_empty() {
        d.push(format!(
            "the command answered and wrote stderr: {}",
            got.stderr
        ));
    }
    if row.now.container {
        let there = dir.join("witness");
        std::fs::create_dir_all(&there).expect("scratch dir");
        let out = Command::new(ein)
            .args(plan::argv(&row.now.run, &row.witness, &there))
            .current_dir(repo_root())
            .output()
            .unwrap_or_else(|e| panic!("{}: {}: {e}", row.id, row.witness));
        let want = comparable(&String::from_utf8_lossy(&out.stdout));
        let have = comparable(&got.stdout);
        if want != have {
            let at = want
                .lines()
                .zip(have.lines())
                .position(|(a, b)| a != b)
                .map_or_else(
                    || "their lengths".to_string(),
                    |i| format!("line {}", i + 1),
                );
            d.push(format!(
                "the container's stdout is not the witness's ({at} differs)"
            ));
        }
    }
    d
}

/// `nogoods.json`'s sets and cores, sorted as a row compares them.
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

/// The sentences the read-out prints for a query's `:goal-text`, in order —
/// its `“…”` lines, which nothing else on the
/// read-out writes.
fn goal_text(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|l| {
            l.trim_start()
                .strip_prefix('“')
                .and_then(|r| r.strip_suffix('”'))
                .map(str::to_string)
        })
        .collect()
}

/// The first line of `text` that begins, past its indent, with `label`, its
/// trailing spaces aside; empty where none does.
fn printed_line(text: &str, label: &str) -> String {
    text.lines()
        .find(|l| l.trim_start().starts_with(label))
        .map(|l| l.trim_end().to_string())
        .unwrap_or_default()
}

/// Whether `s` is a summary with no answer that says why: every answer key
/// `null`, and an error among its diagnostics.
fn unanswered(s: &J) -> bool {
    [
        "region",
        "verdict",
        "queries",
        "claim",
        "work",
        "monotonicity",
        "leftover",
        "root",
    ]
    .iter()
    .all(|k| s[k].is_null())
        && s["diagnostics"]
            .as_array()
            .is_some_and(|d| d.iter().any(|x| x["severity"] == "error"))
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
    // What the universe leaves out (Q-M1e2.3): on every run.
    let mut outside: Vec<String> = s["root"]["hypgen"]["outside"]
        .as_array()
        .ok_or("no root.hypgen.outside")?
        .iter()
        .filter_map(|o| o["relation"].as_str().map(str::to_string))
        .collect();
    outside.sort();
    // The core, where the row records one.
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
    // The core's status, where the row records one:
    // `none` where the summary states none.
    let core_status = row.now.core_status.as_ref().map(|_| {
        region
            .get("core_status")
            .and_then(J::as_str)
            .unwrap_or("none")
            .to_string()
    });
    // N's cores, where the row records them; on the sets the row lists
    // alone where it says so, each of which the run must report.
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
    // Stale members of R, where the row records them: the summary's
    // `monotonicity.state`, its `stale` entries of
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
    // The members met and not listed, where the row records them: the
    // summary's states with `listed = false`.
    let others = row.now.others.map(|_| {
        region["states"].as_array().map_or(0, |v| {
            v.iter().filter(|x| x["listed"] == false).count() as u64
        })
    });
    // The solve form's claim and each query's classes and claim, where the
    // row records them.
    let outcome = |e: &J| e["outcome"].as_str().unwrap_or("none").to_string();
    let claim = row.now.claim.as_ref().map(|_| outcome(&s["claim"]));
    // What root's `refuted by` names, where the row records it: in the
    // order printed, empty where none is stated.
    let refuted_by = row.now.refuted_by.as_ref().map(|_| {
        region["refuted_by"]
            .as_array()
            .map(|v| {
                v.iter()
                    .filter_map(|x| x.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    });
    // What a query's `:goal-text` prints, where the row records it:
    // off stdout, in the order printed.
    let goal_text = row.now.goal_text.as_ref().map(|_| goal_text(&got.stdout));
    // The lines printed, where the row records them: each found by its
    // label, on stdout, in `answer.txt`, or in a state file, where an
    // empty line says none begins with it.
    let printed = match &row.now.printed {
        None => None,
        Some(want) => {
            let mut v = Vec::new();
            for p in want {
                let text = match p.file.as_str() {
                    "stdout" => got.stdout.clone(),
                    file => std::fs::read_to_string(got.answer.join(file))
                        .map_err(|_| format!("no {file} was written"))?,
                };
                v.push(Printed {
                    file: p.file.clone(),
                    label: p.label.clone(),
                    line: printed_line(&text, &p.label),
                });
            }
            Some(v)
        }
    };
    let queries = row.now.queries.as_ref().map(|_| {
        s["queries"]
            .as_array()
            .map(|v| {
                v.iter()
                    .map(|q| QueryAnswer {
                        classes: q["classes"].as_array().map_or(0, |c| c.len() as u64),
                        expect: outcome(&q["expect"]),
                    })
                    .collect()
            })
            .unwrap_or_default()
    });
    Ok(Now {
        run: row.now.run.clone(),
        line,
        container: row.now.container,
        answered: false,
        refused: None,
        error: None,
        exit: None,
        stdout: None,
        signal: None,
        panicked: None,
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
        claim,
        queries,
        refuted_by,
        goal_text,
        printed,
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
        "refuted_by",
        format!("{:?}", want.refuted_by),
        format!("{:?}", got.refuted_by),
    );
    field(
        "goal_text",
        format!("{:?}", want.goal_text),
        format!("{:?}", got.goal_text),
    );
    field(
        "printed",
        format!("{:?}", want.printed),
        format!("{:?}", got.printed),
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
    field(
        "claim",
        format!("{:?}", want.claim),
        format!("{:?}", got.claim),
    );
    field(
        "queries",
        format!("{:?}", want.queries),
        format!("{:?}", got.queries),
    );
    out
}

/// What holding a file's rows came to: how many runs, and every row whose
/// run answered other than its `[case.now]` records, one message each.
pub struct Outcome {
    pub runs: usize,
    pub bad: Vec<String>,
}

/// Run every row's `[case.now]` through the binary `ein`, from the
/// repository's root, each in a directory of its own under `scratch`, and
/// compare what it answers with what the row records.
pub fn hold(ein: &Path, rows: &[Row], scratch: &Path) -> Outcome {
    let mut bad = Vec::new();
    let mut runs = 0usize;
    for row in rows {
        let dir = scratch.join(plan::slug(&row.id));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let got = run(ein, row, &dir);
        runs += 1;
        let first = got
            .stderr
            .lines()
            .find(|l| !l.is_empty())
            .unwrap_or("<empty>");
        // A refusal is exit 1 and its first line, and a summary with no
        // answer that says why; a run that ended in an engine's error before
        // it answered writes none.
        let ended = match (&row.now.refused, &row.now.error) {
            (Some(want), _) => Some(("refused", "a refusal", want)),
            (None, Some(want)) => Some(("error", "an error", want)),
            (None, None) => None,
        };
        let diffs = if row.now.stdout_closed() {
            closed(row, &got)
        } else if let Some((field, what, want)) = ended {
            // `{FILE}` is the path the run was handed, and `{out}` the run's
            // scratch directory.
            let want = &want
                .replace("{FILE}", &got.file)
                .replace("{out}", &dir.display().to_string());
            let exit = row.now.exit.unwrap_or(1);
            let mut d = Vec::new();
            if got.code != exit {
                d.push(format!("exit: recorded {exit}, {what}, got {}", got.code));
            }
            if first != want {
                d.push(format!("{field}: recorded {want:?}, got {first:?}"));
            }
            match &got.summary {
                Some(s) if !unanswered(s) => {
                    d.push(format!("{what} wrote a summary with an answer"));
                }
                Some(s) if field == "error" => {
                    d.push(format!("{what} wrote a summary: {}", s["diagnostics"]));
                }
                _ => {}
            }
            d
        } else if row.now.is_command() {
            answered(ein, row, &dir, &got)
        } else if got.code != i32::from(row.now.refutes()) {
            // A refuted claim exits 1; every other answer 0.
            vec![format!(
                "exit: recorded {}, got {} — stderr: {first}",
                i32::from(row.now.refutes()),
                got.code
            )]
        } else {
            match now_of(row, &got) {
                Ok(now) => {
                    let mut d = differences(&row.now, &now);
                    // A right that names what the universe leaves out states
                    // the sentence too: the `outside` row, wherever `now`
                    // records that `outside`.
                    if row.right.outside.as_ref() == Some(&row.now.outside)
                        && let Some(q) = &row.right.qualifier
                        && !got.stdout.contains(&format!("    outside         {q}\n"))
                    {
                        d.push(format!("the read-out does not print {q:?}"));
                    }
                    // A right that states a core's status states the
                    // read-out's `core search` row too, wherever `now` records
                    // that status.
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
        let why = if row.now.against(&row.right).is_empty() {
            "a row whose recorded answer is the right one no longer gives it"
        } else {
            "a row whose recorded answer is wrong changed answer — a fix nobody intended, or a \
             new wrong answer; either way the row moves in the same commit"
        };
        bad.push(format!(
            "{} ein {} {} — {why}:\n    {}",
            row.id,
            row.now.run,
            row.witness,
            diffs.join("\n    ")
        ));
    }
    Outcome { runs, bad }
}
