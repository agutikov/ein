//! M1e2 S1e2.1.2 — **`ein solve --dump-answer DIR`, held to what it says.**
//!
//! The flag writes the answer as files: the program, root, one loadable file
//! per recorded model or open state with the commitment that reached it, every
//! refuted commitment, and `answer.json`. Its page is
//! [`docs/kernel/inference/answer_dump.md`](../../../../docs/kernel/inference/answer_dump.md),
//! and these are the claims on it that a test can hold:
//!
//! - `answer.json` **is** `--json-summary`, byte for byte, on every arm;
//! - every state file **loads**, holds exactly the recorded state, and is a
//!   **fixpoint** of its own program — saturating it derives no fact and no
//!   `(false)`. A recorded state that is not a fixpoint is the class M1e's
//!   `CO-M1` fixed, and this makes it observable per file;
//! - `models/NN` is stdout's *model N/k*;
//! - the flag **moves no answer**: stdout, stderr, the exit code, the summary
//!   and the event stream are the same with it and without;
//! - a second dump into the same directory leaves nothing stale; a `.einb`
//!   is refused; an aborted run still says what it is.
//!
//! ```text
//! cargo test -p ein-cli --test answer_dump
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test answer_dump   # + zebra2-minus-15 -e -m 3, 32 models
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_core::Terms;
use ein_corpus::{register, repo_root};
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

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ein-answer-dump-{}-{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch");
    dir
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

/// One state file, loaded and saturated in-process: `None` when it is a
/// fixpoint holding exactly `recorded` facts, else what is wrong with it.
///
/// `may_be_false` is for `root.ein` alone: a root that is contradictory before
/// any hypothesis is recorded holding `(false)`, and holding it again is the
/// fixpoint. A model or an open state that saturates to `(false)` is a state
/// the engine's own rules refute.
fn not_a_fixpoint(path: &Path, recorded: usize, may_be_false: bool) -> Option<String> {
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
    let saturated = kb.n_facts();
    if !may_be_false && ein_infer::contradiction::has_contradiction(&kb, &terms) {
        return Some("saturates to (false)".to_string());
    }
    if loaded != recorded {
        return Some(format!(
            "loads {loaded} facts where the state recorded {recorded}"
        ));
    }
    if saturated != loaded {
        return Some(format!(
            "derives {} facts it did not hold — not a fixpoint",
            saturated - loaded
        ));
    }
    None
}

/// Every `.ein` the directory holds against the fact count its record says.
fn check_states(dir: &Path, label: &str, bad: &mut Vec<String>) -> usize {
    let index = json(&dir.join("index.json"));
    let mut n = 0;
    for key in ["models", "open"] {
        for stem in index[key].as_array().into_iter().flatten() {
            let stem = stem.as_str().expect("a file stem");
            let rec = json(&dir.join(format!("{stem}.json")));
            let recorded = rec["facts"].as_array().expect("facts").len();
            if let Some(why) = not_a_fixpoint(&dir.join(format!("{stem}.ein")), recorded, false) {
                bad.push(format!("{label}: {stem}.ein {why}"));
            }
            n += 1;
        }
    }
    let root = dir.join("root.ein");
    let initial = index["root_initial_facts"].as_u64().unwrap_or(0) as usize;
    if let Some(why) = not_a_fixpoint(&root, initial, true) {
        bad.push(format!("{label}: root.ein {why}"));
    }
    n + 1
}

/// The programs the per-commit checks run on: one per verdict word, the
/// obligations rung, an hrule, a blind rung and the puzzle.
const CASES: [(&str, &[&str]); 6] = [
    ("examples/lattice/02_genuine_3set_death.ein", &["-e"]),
    ("examples/branching/16_lookahead_two_step_off.ein", &["-e"]),
    ("examples/ein-bugs/wrong-answers/tree-dead.ein", &["-e"]),
    ("examples/ein-bugs/wrong-answers/owed-seats.ein", &["-e"]),
    ("examples/branching/15_lookahead_two_step_on.ein", &["-e"]),
    ("examples/zebra2.ein", &["-e"]),
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
    }
    let _ = std::fs::remove_dir_all(&root);
}

/// **Every recorded state is a fixpoint of its own program** — over every
/// run the register records, and the puzzle.
///
/// The register is the list that matters here: its rows are the runs a
/// reviewer will open, and several of them are records the engine got wrong.
/// *Wrong* is allowed. A recorded state that its own rules would extend or
/// refute is not, and is what this would report.
#[test]
fn every_recorded_state_is_a_fixpoint_of_its_own_program() {
    let root = scratch("fixpoint");
    let mut bad = Vec::new();
    let mut states = 0;
    let mut runs: Vec<(String, String, Vec<String>)> = Vec::new();
    for row in &register().case {
        for (_, a) in row.answers() {
            // A debug build aborts on W7's pair; that is the row's own claim.
            if a.debug_panic.is_some() && cfg!(debug_assertions) {
                continue;
            }
            runs.push((row.witness_of(a).to_string(), a.run.clone(), a.env.clone()));
        }
    }
    runs.push(("examples/zebra2.ein".into(), "solve -e".into(), Vec::new()));
    if std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1") {
        runs.push((
            "examples/zebra2-minus-15.ein".into(),
            "solve -e -m 3".into(),
            Vec::new(),
        ));
    }
    runs.sort();
    runs.dedup();
    for (i, (file, run, env)) in runs.iter().enumerate() {
        let dir = root.join(format!("r{i}"));
        let flags: Vec<&str> = run.split_whitespace().skip(1).collect();
        let r = dump(file, &flags, env, &dir);
        assert!(
            r.code == 0 || r.code == 1,
            "{file} {run}: exit {}\n{}",
            r.code,
            r.err
        );
        states += check_states(&dir, &format!("{file} :: {run}"), &mut bad);
    }
    let _ = std::fs::remove_dir_all(&root);
    assert!(states > runs.len(), "no recorded state was checked");
    assert!(
        bad.is_empty(),
        "{} state files:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **`models/NN` is stdout's model N/k**, and the files are the answer.
///
/// `owed-seats.ein` has three models whose facts differ, so a numbering that
/// drifted from the read-out's would put one model's `-P` block against
/// another model's file and fail.
#[test]
fn the_files_are_the_answer_in_the_read_outs_order() {
    let dir = scratch("order").join("d");
    let file = "examples/ein-bugs/wrong-answers/owed-seats.ein";
    let r = dump(file, &["-e", "-P"], &[], &dir);
    assert_eq!(r.code, 0, "{}", r.err);
    let index = json(&dir.join("index.json"));
    let summary = json(&dir.with_extension("json"));
    let models: Vec<&str> = index["models"]
        .as_array()
        .expect("models")
        .iter()
        .map(|m| m.as_str().expect("stem"))
        .collect();
    assert_eq!(
        models.len() as u64,
        summary["verdict"]["k"].as_u64().unwrap()
    );
    // The set of models is the summary's set.
    let mut ours: Vec<J> = models
        .iter()
        .map(|m| json(&dir.join(format!("{m}.json")))["facts"].clone())
        .collect();
    let mut theirs: Vec<J> = summary["verdict"]["solutions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["facts"].clone())
        .collect();
    ours.sort_by_key(|v| v.to_string());
    theirs.sort_by_key(|v| v.to_string());
    assert_eq!(ours, theirs, "the files hold another model set");
    // And file N is block N of `-P`.
    let blocks: Vec<Vec<String>> = r
        .out
        .split("── solution ")
        .skip(1)
        .map(|b| {
            b.lines()
                .skip(1)
                .take_while(|l| l.starts_with("  (") || l.starts_with("final-state"))
                .filter(|l| l.starts_with("  ("))
                .map(|l| l.trim().to_string())
                .collect()
        })
        .collect();
    assert_eq!(blocks.len(), models.len(), "{}", r.out);
    for (n, (block, stem)) in blocks.iter().zip(&models).enumerate() {
        let facts = json(&dir.join(format!("{stem}.json")))["facts"].clone();
        for f in block {
            assert!(
                facts.as_array().unwrap().iter().any(|x| x == f.as_str()),
                "stdout's solution {} prints {f}, which {stem}.json does not hold",
                n + 1
            );
        }
    }
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
}

/// **The flag moves no answer.** Stdout, stderr, the exit code, the summary
/// and the event stream, with the flag and without, on three programs — one
/// of them under the tree traversal, whose proof has no layers.
#[test]
fn the_flag_moves_no_answer() {
    let root = scratch("additive");
    let cases: [(&str, &[&str], &[&str]); 3] = [
        ("examples/zebra2.ein", &["-e"], &[]),
        ("examples/ein-bugs/wrong-answers/forced-2.ein", &["-e"], &[]),
        (
            "examples/zebra2-obligations.ein",
            &["-e"],
            &["EIN_TRAVERSAL=tree"],
        ),
    ];
    for (i, (file, flags, env)) in cases.iter().enumerate() {
        let env: Vec<String> = env.iter().map(|s| s.to_string()).collect();
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
            let r = ein(&args, &env);
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
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_second_dump_into_one_directory_leaves_nothing_stale() {
    let dir = scratch("stale").join("d");
    let r = dump(
        "examples/lattice/02_genuine_3set_death.ein",
        &["-e"],
        &[],
        &dir,
    );
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(dir.join("models/03.ein").is_file());
    let r = dump(
        "examples/branching/15_lookahead_two_step_on.ein",
        &["-e"],
        &[],
        &dir,
    );
    assert_eq!(r.code, 0, "{}", r.err);
    let left: Vec<String> = std::fs::read_dir(dir.join("models"))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(left.len(), 2, "one model, two files — got {left:?}");
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
}

#[test]
fn an_aborted_run_still_says_what_it_is() {
    let dir = scratch("abort").join("d");
    let r = dump("examples/zebra2.ein", &["-E", "1"], &[], &dir);
    assert_eq!(r.code, 2, "a budget cut exits 2\n{}", r.err);
    let index = json(&dir.join("index.json"));
    assert_eq!(index["verdict"], "Aborted");
    assert!(index["reason"].as_str().unwrap().contains("max-enterings"));
    let a = std::fs::read(dir.join("answer.json")).expect("answer.json");
    let s = std::fs::read(dir.with_extension("json")).expect("summary");
    assert!(
        a == s,
        "the aborted answer.json is not --json-summary's bytes"
    );
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
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
    let _ = std::fs::remove_dir_all(&root);
}

/// **Every `stats` counter a summary carries is defined on the page.** The
/// review bundle quotes `answer_dump.md` § The run summary beside each number,
/// so a counter without a row would be printed with no meaning — and a row
/// for a counter no summary carries is a definition of nothing.
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
    let dir = scratch("counters").join("d");
    let r = dump("examples/zebra2.ein", &["-e"], &[], &dir);
    assert_eq!(r.code, 0, "{}", r.err);
    let summary = json(&dir.with_extension("json"));
    let keys: std::collections::BTreeSet<String> = summary["stats"]
        .as_object()
        .expect("stats")
        .keys()
        .cloned()
        .collect();
    assert_eq!(
        keys, rows,
        "the summary's counters and the page's rows differ"
    );
    let _ = std::fs::remove_dir_all(dir.parent().unwrap());
}
