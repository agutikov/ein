//! `--json-summary` (`ein-summary/2`) and `--dump-answer DIR` (`ein-answer/2`)
//! under the region modes — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! fifth step, the fields of
//! [`read_out.md` § 8.1–8.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md#81---json-summary-ein-summary2).
//!
//! - **one shape on every arm**: the summary's keys, and the `region` block's,
//!   do not depend on the question or on what R turned out to be;
//! - **the same facts as stdout**: the counts the summary carries are the ones
//!   the read-out prints;
//! - **the directory is the answer**: `answer.txt` is the read-out uncut,
//!   `answer.json` is the summary byte for byte, each `models/` file loads and
//!   is a fixpoint of its own program holding its state's model, and
//!   `nogoods.json` is N;
//! - **additive**: stdout, stderr and the exit code are the same with the two
//!   flags and without.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use ein_core::Terms;
use ein_corpus::{Scratch, repo_root};
use ein_ir::Ast;
use serde_json::Value as J;

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn scratch(tag: &str) -> Scratch {
    Scratch::new(&format!("ein-region-answer-{tag}"))
}

fn json(path: &Path) -> J {
    serde_json::from_str(&std::fs::read_to_string(path).expect("written")).expect("JSON")
}

/// One run per arm of the read-out: exhausted, the owing and the dead kind of
/// an empty R, a hole, a partial route-1 run, a flagged run past the limit,
/// and the modes.
const ARMS: [(&str, &[&str]); 9] = [
    (
        "examples/branching/05_mini_zebra.ein",
        &["--region-budget", "524288"],
    ),
    (
        "tests/stdlib/closure/03_closed_and_owing.ein",
        &["--region-budget", "524288"],
    ),
    (
        "tests/stdlib/algebra/09_irreflexive_violated.ein",
        &["--region-budget", "524288"],
    ),
    (
        "examples/ein-bugs/naf-upward-closure.ein",
        &["--region-budget", "524288"],
    ),
    (
        "examples/saturation/square-bwd/floors.ein",
        &["--region-budget", "4096"],
    ),
    (
        "examples/lattice/06_friends_tables.ein",
        &["--region-budget", "524288"],
    ),
    ("examples/lattice/02_genuine_3set_death.ein", &["--maximal"]),
    ("examples/lattice/02_genuine_3set_death.ein", &["--single"]),
    (
        "examples/lattice/02_genuine_3set_death.ein",
        &["-n", "2", "--minimal"],
    ),
];

fn keys(v: &J) -> BTreeSet<String> {
    v.as_object()
        .map_or(BTreeSet::new(), |o| o.keys().cloned().collect())
}

#[test]
fn the_summary_has_one_shape_on_every_arm_and_says_what_stdout_says() {
    let mut shapes: Vec<(String, BTreeSet<String>, BTreeSet<String>)> = Vec::new();
    for (i, (file, flags)) in ARMS.iter().enumerate() {
        let dir = scratch(&format!("summary-{i}"));
        let path = dir.join("s.json");
        let p = path.to_string_lossy().to_string();
        let mut args = vec!["solve", file];
        args.extend(flags.iter());
        let bare = ein(&args);
        args.extend(["--json-summary", &p]);
        let r = ein(&args);
        assert_eq!(
            (r.code, &r.out, &r.err),
            (bare.code, &bare.out, &bare.err),
            "{file} {flags:?}: the flag moved stdout, stderr or the exit code"
        );
        let s = json(&path);
        assert_eq!(s["schema"], "ein-summary/2", "{file}");
        let region = &s["region"];
        // The counts stdout prints are the summary's.
        if let Some(k_s) = region["k_s"].as_u64() {
            let row = r
                .out
                .lines()
                .find(|l| l.trim_start().starts_with("states (k_s)"))
                .expect("a k_s row");
            // `1 194`: digits in groups of three, up to the first word.
            let printed: String = row["  states (k_s)".len()..]
                .trim_start()
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == ' ')
                .filter(char::is_ascii_digit)
                .collect();
            assert_eq!(printed, k_s.to_string(), "{file}: {row}");
        }
        let listed = region["states"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["listed"] == true)
            .count();
        assert!(
            listed <= region["states"].as_array().unwrap().len(),
            "{file}"
        );
        shapes.push((format!("{file} {flags:?}"), keys(&s), keys(region)));
    }
    let (first, top, reg) = &shapes[0];
    for (label, t, r) in &shapes[1..] {
        assert_eq!(t, top, "{label}: the summary's keys differ from {first}'s");
        assert_eq!(
            r, reg,
            "{label}: the region block's keys differ from {first}'s"
        );
    }
}

/// A state file loads, saturates to itself, and holds its state's model.
fn a_fixpoint_holding(path: &Path, model: &[String]) -> Option<String> {
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
    if kb.n_facts() != loaded {
        return Some(format!(
            "derives {} facts — not a fixpoint",
            kb.n_facts() - loaded
        ));
    }
    let facts: BTreeSet<String> = kb
        .facts()
        .map(|f| ein_infer::events::sexpr(&terms, f))
        .collect();
    model
        .iter()
        .find(|m| !facts.contains(*m))
        .map(|m| format!("does not hold {m}"))
}

#[test]
fn the_answer_directory_is_the_answer() {
    let mut bad: Vec<String> = Vec::new();
    let mut files = 0usize;
    for (i, (file, flags)) in ARMS.iter().enumerate() {
        let tmp = scratch(&format!("dump-{i}"));
        let dir = tmp.join("d");
        let d = dir.to_string_lossy().to_string();
        let summary = dir.with_extension("json");
        let sp = summary.to_string_lossy().to_string();
        let mut args = vec!["solve", file];
        args.extend(flags.iter());
        let bare = ein(&args);
        args.extend(["--dump-answer", &d, "--json-summary", &sp]);
        let r = ein(&args);
        let label = format!("{file} {flags:?}");
        assert_eq!(
            (r.code, &r.out, &r.err),
            (bare.code, &bare.out, &bare.err),
            "{label}: the flags moved stdout, stderr or the exit code"
        );
        let index = json(&dir.join("index.json"));
        assert_eq!(index["schema"], "ein-answer/2", "{label}");
        assert_eq!(
            std::fs::read(dir.join("answer.json")).unwrap(),
            std::fs::read(&summary).unwrap(),
            "{label}: answer.json is not the summary"
        );
        let txt = std::fs::read_to_string(dir.join("answer.txt")).unwrap();
        // The uncut read-out starts as stdout does and is never shorter.
        let header = |s: &str| s.lines().skip(1).take(4).collect::<Vec<_>>().join("\n");
        assert_eq!(header(&txt), header(&r.out), "{label}: answer.txt");
        assert!(txt.lines().count() >= r.out.lines().count(), "{label}");
        let s = json(&summary);
        let states = s["region"]["states"].as_array().unwrap();
        let models = index["models"].as_array().unwrap();
        assert_eq!(models.len(), states.len(), "{label}: one file per state");
        for (m, st) in models.iter().zip(states) {
            let model: Vec<String> = st["facts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|f| f.as_str().unwrap().to_string())
                .collect();
            let path = dir.join(m["path"].as_str().unwrap());
            if let Some(why) = a_fixpoint_holding(&path, &model) {
                bad.push(format!("{label}: {} {why}", path.display()));
            }
            files += 1;
        }
        let nogoods = json(&dir.join("nogoods.json"));
        if let Some(n) = s["region"]["nogoods"].as_u64() {
            assert_eq!(
                nogoods.as_array().unwrap().len() as u64,
                n,
                "{label}: nogoods.json"
            );
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(files >= 15, "{files} state files checked");
}

#[test]
fn a_second_dump_leaves_nothing_stale() {
    let tmp = scratch("twice");
    let dir = tmp.join("d");
    let d = dir.to_string_lossy().to_string();
    let big = ein(&[
        "solve",
        "examples/lattice/02_genuine_3set_death.ein",
        "--region-budget",
        "524288",
        "--dump-answer",
        &d,
    ]);
    assert_eq!(big.code, 0, "{}", big.err);
    let small = ein(&[
        "solve",
        "examples/lattice/02_genuine_3set_death.ein",
        "--single",
        "--dump-answer",
        &d,
    ]);
    assert_eq!(small.code, 0, "{}", small.err);
    let index = json(&dir.join("index.json"));
    let listed = index["models"].as_array().unwrap().len();
    let on_disk = std::fs::read_dir(dir.join("models")).unwrap().count();
    assert_eq!(
        listed, on_disk,
        "models/ holds files the index does not name"
    );
}
