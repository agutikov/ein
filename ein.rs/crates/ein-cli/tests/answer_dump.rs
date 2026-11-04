//! M1e2 S1e2.1.2 — **`ein solve --dump-answer DIR`, held to what it says**,
//! since S1e2.1a.5's switch the region read-out's directory, `ein-answer/2`.
//!
//! The flag writes the answer as files: the read-out, the summary, one
//! loadable program per state the run reached, the owing frontier of an empty
//! R, the minimal dead sets, and `index.json` naming each. Its page is
//! [`docs/kernel/inference/answer_dump.md`](../../../../docs/kernel/inference/answer_dump.md),
//! and these are the claims on it that a test can hold:
//!
//! - `answer.json` **is** `--json-summary`, and `answer.txt` stdout, byte for
//!   byte, on every question;
//! - every state file **loads**, holds exactly its state's candidates, and is
//!   a **fixpoint** of its own program — saturating it derives no fact and no
//!   `(false)`. A recorded state that is not a fixpoint is the class M1e's
//!   `CO-M1` fixed, and this makes it observable per file;
//! - `models/NN` is stdout's *state N of k*;
//! - the flag **moves no answer**: stdout, stderr, the exit code, the summary
//!   and the event stream are the same with it and without;
//! - a second dump into the same directory leaves nothing stale; a `.einb`
//!   is refused; a stopped run still says what it is.
//!
//! ```text
//! cargo test -p ein-cli --test answer_dump
//! ```

use std::path::Path;
use std::process::Command;

use ein_core::Terms;
use ein_corpus::{Scratch, register, repo_root};
use ein_ir::Ast;
use serde_json::Value as J;

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str], env: &[String]) -> Run {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.args(args).current_dir(repo_root());
    for kv in env {
        let (k, v) = kv.split_once('=').expect("NAME=value");
        cmd.env(k, v);
    }
    let out = cmd.output().expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn scratch(tag: &str) -> Scratch {
    Scratch::new(&format!("ein-answer-dump-{tag}"))
}

fn json(path: &Path) -> J {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// `ein solve <file> <flags…> --dump-answer <dir> --json-summary <dir>.json`.
fn dump(file: &str, flags: &[&str], env: &[String], dir: &Path) -> Run {
    let summary = dir.with_extension("json");
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    let (d, s) = (dir.to_string_lossy(), summary.to_string_lossy());
    args.extend(["--dump-answer", &d, "--json-summary", &s]);
    ein(&args, env)
}

/// A state file, loaded and saturated in-process: `None` when it is a
/// fixpoint, holds every candidate of `closure` and no other of
/// `candidates`, else what is wrong with it.
fn not_a_fixpoint(path: &Path, closure: &[String], candidates: &[String]) -> Option<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = match ein_ir::load_file(&mut ast, &mut terms, path) {
        Ok(kb) => kb,
        Err(e) => return Some(format!("does not load: {e}")),
    };
    let loaded = kb.n_facts();
    if let Err(e) = ein_infer::saturate_events(&ast, &mut terms, &mut kb) {
        return Some(format!("does not saturate: {e:?}"));
    }
    if ein_infer::contradiction::has_contradiction(&kb, &terms) {
        return Some("saturates to (false)".to_string());
    }
    let saturated = kb.n_facts();
    if saturated != loaded {
        return Some(format!(
            "derives {} facts it did not hold — not a fixpoint",
            saturated - loaded
        ));
    }
    let held: std::collections::BTreeSet<String> = kb
        .facts()
        .map(|f| ein_infer::events::sexpr(&terms, f))
        .collect();
    for c in candidates {
        if held.contains(c) != closure.contains(c) {
            return Some(format!(
                "{} {c}, which the state's closure {}",
                if held.contains(c) { "holds" } else { "lacks" },
                if held.contains(c) {
                    "does not list"
                } else {
                    "lists"
                }
            ));
        }
    }
    None
}

fn strings(v: &J) -> Vec<String> {
    v.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Every state file the directory holds against its state in the summary.
fn check_states(dir: &Path, label: &str, bad: &mut Vec<String>) -> usize {
    let index = json(&dir.join("index.json"));
    let summary = json(&dir.join("answer.json"));
    let states = summary["region"]["states"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut candidates: Vec<String> = states.iter().flat_map(|s| strings(&s["closure"])).collect();
    candidates.sort();
    candidates.dedup();
    let mut n = 0;
    for (i, entry) in index["models"].as_array().into_iter().flatten().enumerate() {
        let path = entry["path"].as_str().expect("a path");
        let closure = strings(&states[i]["closure"]);
        if let Some(why) = not_a_fixpoint(&dir.join(path), &closure, &candidates) {
            bad.push(format!("{label}: {path} {why}"));
        }
        n += 1;
    }
    for entry in index["owing"].as_array().into_iter().flatten() {
        let path = entry["path"].as_str().expect("a path");
        if let Some(why) = not_a_fixpoint(&dir.join(path), &[], &[]) {
            bad.push(format!("{label}: {path} {why}"));
        }
        n += 1;
    }
    n
}

/// The programs the per-commit checks run on, with a question each: all of R
/// on a program with a dead set and on an owing root, a line, a part, the
/// word, an empty R, and the puzzle's model.
const CASES: [(&str, &[&str]); 7] = [
    ("examples/lattice/02_genuine_3set_death.ein", &[]),
    ("examples/ein-bugs/wrong-answers/owed-seats.ein", &[]),
    (
        "examples/branching/16_lookahead_two_step_off.ein",
        &["--maximal"],
    ),
    (
        "examples/branching/15_lookahead_two_step_on.ein",
        &["-n", "2"],
    ),
    (
        "examples/ein-bugs/wrong-answers/tree-dead.ein",
        &["--single"],
    ),
    ("examples/ein-bugs/zebra2-bad.ein", &[]),
    ("examples/zebra2.ein", &["-n", "1"]),
];

#[test]
fn answer_json_is_the_json_summary_byte_for_byte() {
    let root = scratch("identity");
    for (i, (file, flags)) in CASES.iter().enumerate() {
        let dir = root.join(format!("c{i}"));
        let r = dump(file, flags, &[], &dir);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        let a = std::fs::read(dir.join("answer.json")).expect("answer.json");
        let s = std::fs::read(dir.with_extension("json")).expect("summary");
        assert!(a == s, "{file}: answer.json is not --json-summary's bytes");
        let t = std::fs::read_to_string(dir.join("answer.txt")).expect("answer.txt");
        assert_eq!(t, r.out, "{file}: answer.txt is not stdout");
    }
}

/// **Every state file is a fixpoint of its own program** — over every run
/// the register records and the cases above, the puzzle's model among them.
///
/// The register is the list that matters here: its rows are the runs a
/// reviewer will open, and several of them are records the engine got wrong.
/// *Wrong* is allowed. A written state that its own rules would extend or
/// refute is not, and is what this would report.
#[test]
fn every_recorded_state_is_a_fixpoint_of_its_own_program() {
    let root = scratch("fixpoint");
    let mut bad = Vec::new();
    let mut states = 0;
    let mut runs: Vec<(String, String)> = Vec::new();
    for row in &register().case {
        // A refused row (M1e2 S1e2.2.5) writes no state: `wrong_answers.rs`
        // holds its refusal.
        if row.status.starts_with("merged:") || row.now.refused.is_some() {
            continue;
        }
        runs.push((row.witness.clone(), row.now.run.clone()));
    }
    for (file, flags) in CASES {
        let mut run = vec!["solve"];
        run.extend_from_slice(flags);
        runs.push((file.to_string(), run.join(" ")));
    }
    runs.sort();
    runs.dedup();
    for (i, (file, run)) in runs.iter().enumerate() {
        let dir = root.join(format!("r{i}"));
        let flags: Vec<&str> = run.split_whitespace().skip(1).collect();
        let r = dump(file, &flags, &[], &dir);
        assert_eq!(r.code, 0, "{file} {run}: exit {}\n{}", r.code, r.err);
        states += check_states(&dir, &format!("{file} :: {run}"), &mut bad);
    }
    assert!(states > runs.len(), "no state file was checked");
    assert!(
        bad.is_empty(),
        "{} state files:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **`models/NN` is stdout's state N of k**, and the files are the answer.
///
/// `owed-seats.ein` has seven states whose facts differ, so a numbering that
/// drifted from the read-out's would put one state's `(model …)` against
/// another state's file and fail.
#[test]
fn the_files_are_the_answer_in_the_read_outs_order() {
    let tmp = scratch("order");
    let dir = tmp.join("d");
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let r = dump(file, &[], &[], &dir);
    assert_eq!(r.code, 0, "{}", r.err);
    let index = json(&dir.join("index.json"));
    let summary = json(&dir.with_extension("json"));
    let models = index["models"].as_array().expect("models");
    assert_eq!(
        models.len() as u64,
        summary["region"]["k_s"].as_u64().unwrap()
    );
    // Block N of the `(region …)` is file N.
    let lines: Vec<&str> = r.out.lines().map(str::trim_start).collect();
    let blocks: Vec<&str> = lines
        .windows(2)
        .filter(|w| w[0].starts_with(";; state ") && w[0].contains(" of "))
        .map(|w| w[1])
        .collect();
    assert_eq!(blocks.len(), models.len(), "{}", r.out);
    for (n, (block, entry)) in blocks.iter().zip(models).enumerate() {
        assert_eq!(entry["state"], n as u64 + 1);
        let text = std::fs::read_to_string(dir.join(entry["path"].as_str().unwrap())).unwrap();
        assert!(
            text.starts_with(&format!(";;; state {} of {}", n + 1, models.len())),
            "{text}"
        );
        let facts = block
            .trim_start_matches("(model")
            .trim_end_matches(')')
            .split(") (")
            .map(|f| format!("({})", f.trim().trim_matches(['(', ')'])));
        for f in facts.filter(|f| f != "()") {
            assert!(
                text.lines().any(|l| l.starts_with(&f[..f.len() - 1])),
                "stdout's state {} lists {f}, which its file does not hold",
                n + 1
            );
        }
    }
}

/// **The flag moves no answer.** Stdout, stderr, the exit code, the summary
/// and the event stream, with the flag and without, on three questions.
#[test]
fn the_flag_moves_no_answer() {
    let root = scratch("additive");
    let cases: [(&str, &[&str]); 3] = [
        ("examples/zebra2.ein", &["-n", "1"]),
        (
            "examples/ein-bugs/wrong-answers/forced-2.ein",
            &["--maximal"],
        ),
        ("examples/ein-bugs/wrong-answers/owed-seats.ein", &[]),
    ];
    for (i, (file, flags)) in cases.iter().enumerate() {
        let run = |with: bool| {
            let tag = if with { "with" } else { "without" };
            let s = root.join(format!("{i}-{tag}.json"));
            let e = root.join(format!("{i}-{tag}.jsonl"));
            let d = root.join(format!("{i}-dir"));
            let (s, e, d) = (
                s.to_string_lossy(),
                e.to_string_lossy(),
                d.to_string_lossy(),
            );
            let mut args = vec!["solve", file];
            args.extend_from_slice(flags);
            args.extend(["--json-summary", &s, "--events", &e]);
            if with {
                args.extend(["--dump-answer", &d]);
            }
            let r = ein(&args, &[]);
            let summary = std::fs::read_to_string(s.as_ref()).expect("summary");
            let events = std::fs::read_to_string(e.as_ref()).expect("events");
            (r, summary, events)
        };
        let (a, sa, ea) = run(false);
        let (b, sb, eb) = run(true);
        assert_eq!(a.code, b.code, "{file}: exit code moved");
        assert_eq!(a.out, b.out, "{file}: stdout moved");
        assert_eq!(a.err, b.err, "{file}: stderr moved");
        assert_eq!(sa, sb, "{file}: the summary moved");
        // The stream's `run` line names the argv; past it, nothing may move.
        let body = |s: &str| s.lines().skip(1).map(str::to_string).collect::<Vec<_>>();
        assert_eq!(body(&ea), body(&eb), "{file}: the event stream moved");
    }
}

#[test]
fn a_second_dump_into_one_directory_leaves_nothing_stale() {
    let tmp = scratch("stale");
    let dir = tmp.join("d");
    let r = dump(
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        &[],
        &[],
        &dir,
    );
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(dir.join("models/07.ein").is_file());
    let r = dump(
        "examples/branching/15_lookahead_two_step_on.ein",
        &["--maximal"],
        &[],
        &dir,
    );
    assert_eq!(r.code, 0, "{}", r.err);
    let left: Vec<String> = std::fs::read_dir(dir.join("models"))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    let k = json(&dir.join("index.json"))["models"]
        .as_array()
        .unwrap()
        .len();
    assert_eq!(
        left.len(),
        k,
        "{k} states, {} files — got {left:?}",
        left.len()
    );
}

/// **A stopped run still says what it is**: a budget stop is an answer, so
/// its directory is whole and its index says `exhausted false` and why; a
/// `-T` abort is not, so it exits 2 and the directory it made first stays
/// empty.
#[test]
fn a_stopped_run_still_says_what_it_is() {
    let tmp = scratch("abort");
    let dir = tmp.join("d");
    let r = dump("examples/zebra2.ein", &["--region-budget", "1"], &[], &dir);
    assert_eq!(r.code, 0, "a budget stop is an answer\n{}", r.err);
    let index = json(&dir.join("index.json"));
    assert_eq!(index["region"]["exhausted"], false);
    assert_eq!(index["region"]["stopped"]["why"], "budget");
    let a = std::fs::read(dir.join("answer.json")).expect("answer.json");
    let s = std::fs::read(dir.with_extension("json")).expect("summary");
    assert!(
        a == s,
        "the stopped answer.json is not --json-summary's bytes"
    );

    let t = dir.with_file_name("t");
    let r = dump("examples/zebra2.ein", &["-T", "0"], &[], &t);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(t.is_dir(), "the directory is made before the run");
    assert_eq!(
        std::fs::read_dir(&t).unwrap().count(),
        0,
        "an abort wrote files"
    );
}

#[test]
fn a_container_is_refused_because_it_carries_no_program() {
    let root = scratch("einb");
    let einb = root.join("z.einb");
    let r = ein(
        &["kb", "save", "examples/zebra2.ein", &einb.to_string_lossy()],
        &[],
    );
    assert_eq!(r.code, 0, "{}", r.err);
    let dir = root.join("d");
    let r = ein(
        &[
            "solve",
            &einb.to_string_lossy(),
            "--dump-answer",
            &dir.to_string_lossy(),
        ],
        &[],
    );
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(
        r.err.contains("--dump-answer") && r.err.contains(".einb"),
        "{}",
        r.err
    );
    assert!(r.out.is_empty(), "it ran anyway:\n{}", r.out);
}

/// **Every `work` counter and every `root` field a summary carries is
/// defined on the page.** The review bundle quotes `answer_dump.md` § The run
/// summary beside each number, so a counter without a row would be printed
/// with no meaning — and a row for a counter no summary carries is a
/// definition of nothing. A `root` field is a dotted row (`root.hypgen.raw`)
/// down to the sparse blocks, whose keys are data (M1e2 S1e2.3.1).
#[test]
fn every_summary_counter_is_defined_on_this_page() {
    let page = std::fs::read_to_string(repo_root().join("docs/kernel/inference/answer_dump.md"))
        .expect("the page");
    let section = page
        .split("## The run summary")
        .nth(1)
        .and_then(|s| s.split("\n## ").next())
        .expect("§ The run summary");
    let rows: std::collections::BTreeSet<String> = section
        .lines()
        .filter_map(|l| l.strip_prefix("| `"))
        .filter_map(|l| l.split('`').next())
        .map(str::to_string)
        .collect();
    let tmp = scratch("counters");
    let dir = tmp.join("d");
    let r = dump("examples/zebra2.ein", &["-n", "1"], &[], &dir);
    assert_eq!(r.code, 0, "{}", r.err);
    let summary = json(&dir.with_extension("json"));
    let mut keys: std::collections::BTreeSet<String> = summary["work"]
        .as_object()
        .expect("work")
        .keys()
        .cloned()
        .collect();
    fn fields(v: &serde_json::Value, at: &str, out: &mut std::collections::BTreeSet<String>) {
        const SPARSE: [&str; 3] = ["facts_by_relation", "filtered", "pre_candidate"];
        for (k, child) in v.as_object().expect("an object") {
            let name = format!("{at}.{k}");
            if child.is_object() && !SPARSE.contains(&k.as_str()) {
                fields(child, &name, out);
            } else {
                out.insert(name);
            }
        }
    }
    fields(&summary["root"], "root", &mut keys);
    assert!(
        keys.contains("root.hypgen.emitted") && keys.contains("root.saturator.naf_rounds"),
        "the root block was not walked: {keys:?}"
    );
    assert_eq!(
        keys, rows,
        "the summary's counters and fields and the page's rows differ"
    );
}
