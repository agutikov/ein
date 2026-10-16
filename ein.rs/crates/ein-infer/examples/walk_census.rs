//! The walk's last census — M1e2
//! [S1e2.1a.10](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.10_the_register_under_r.md)
//! T1e2.1a.10.1, taken before S1e2.1a.5's switch removes the last path to the
//! walk.
//!
//! [`walk_vs_region`](walk_vs_region.rs) ran the walk six ways of its own
//! choosing. This runs it the ways the repo **declares**: every `solve` run
//! and every `levers` cell `corpus/corpus.toml` lists for an entry, its `test`
//! run as the `solve -e` walk `ein test` makes, and every run a
//! `corpus/wrong_answers.toml` row records, on every program whose R is
//! known (route 1 under [`known::BUDGET`] visits, or the oracle at `n ≤ 16`
//! for a flagged one).
//!
//! **A cell's answer is the CLI's**: `$EIN_BIN` (default
//! `ein.rs/target/release/ein`) run as the corpus runs it
//! ([`ein_corpus::plan::argv`]), its `--json-summary` read back. Each recorded
//! state is put on R — by its closure on a monotone program, and on a flagged
//! one by its positive facts beyond root's against the oracle's states. The
//! walk's question is R's maximal line (`walk_and_region.md` § 3), so a cell
//! **agrees** when:
//!
//! - every recorded model is on the maximal line, and there are as many as
//!   the run asked for (`-n N`: `min(N, |max|)`; `-e`: all of them);
//! - the verdict's kind matches R: a model when R is non-empty, `Open` when
//!   R is empty and an alive node owes, `Contradiction` otherwise;
//! - with R empty, the open states are R's maximal owing nodes.
//!
//! A cell that agrees where R holds more than its maximal line is reason 1,
//! *a different question*, and is not a defect. A cell that disagrees is
//! attributed by re-running the walk in-process: with no cut (`-e -m n`, no
//! `-E`) it is reason 2, *the cuts*; with no cut and nothing written (no
//! forced positive, kill cache or singleton writeback) reason 3, *a
//! different node*; still disagreeing, reason 5 on a program that states an
//! obligation and reason 4, *`complete` is approximate*, otherwise. The
//! in-process run with the cell's own flags is held to the CLI's answer, so
//! the attribution is about the run the CLI made.
//!
//! One JSON line per cell on stdout; the census on stderr.
//!
//! ```sh
//! cargo build --release -p ein-cli
//! cargo run --release -p ein-infer --example walk_census -- [--jobs J] [FILE…] > cells.jsonl
//! ```

#[path = "shared/known.rs"]
mod known;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use ein_corpus::repo_root;
use ein_infer::Events;
use ein_infer::solve::{OnBudget, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use known::{Known, State, beyond, closure_of, known, loaded};
use rustc_hash::FxHashMap;
use serde_json::{Value, json};

/// The in-process walk's budget for an uncut re-run.
const ENTERINGS: u64 = 1 << 21;

/// The flags region modes take: a run carrying one is not the walk.
const REGION_FLAGS: [&str; 4] = ["--minimal", "--maximal", "--single", "--region-budget"];

/// One declared run on one file.
struct Cell {
    path: String,
    run: String,
    env: Vec<String>,
    /// The register answers this cell is: `<row id> <wrong|control|twin n>`.
    rows: Vec<String>,
}

/// What a run's flags ask the walk for.
#[derive(Clone, Debug, Default)]
struct Flags {
    stop_after: Option<u64>,
    max_set_size: Option<u32>,
    max_enterings: Option<u64>,
    no_lookahead: bool,
    no_kill_cache: bool,
    sanity: bool,
    order: Option<String>,
    /// `EIN_TRAVERSAL=tree`: the CLI's answer only.
    tree: bool,
}

fn flags(run: &str, env: &[String]) -> Flags {
    let mut f = Flags {
        stop_after: Some(1),
        ..Flags::default()
    };
    let toks: Vec<&str> = run.split_whitespace().collect();
    assert_eq!(toks.first(), Some(&"solve"), "{run}");
    let mut i = 1;
    let num = |i: usize| -> u64 {
        toks.get(i)
            .and_then(|t| t.parse().ok())
            .unwrap_or_else(|| panic!("{run}: a number after {}", toks[i - 1]))
    };
    while i < toks.len() {
        match toks[i] {
            "-e" => f.stop_after = None,
            "-n" => {
                f.stop_after = Some(num(i + 1));
                i += 1;
            }
            "-m" => {
                f.max_set_size = Some(num(i + 1) as u32);
                i += 1;
            }
            "-E" => {
                f.max_enterings = Some(num(i + 1));
                i += 1;
            }
            "-L" => f.no_lookahead = true,
            "-K" => f.no_kill_cache = true,
            "-y" => f.sanity = true,
            "-o" => {
                f.order = toks.get(i + 1).map(|s| s.to_string());
                i += 1;
            }
            // Read-outs and artefacts: the answer does not depend on them.
            "-p" | "-s" => {}
            "--trace" | "--dump-states" | "--dump-region" => i += 1,
            t => panic!("{run}: `{t}` is not mapped — add it before trusting the census"),
        }
        i += 1;
    }
    for e in env {
        match e.as_str() {
            "EIN_TRAVERSAL=tree" => f.tree = true,
            e => panic!("{run}: env `{e}` is not mapped"),
        }
    }
    f
}

/// A walk's answer, by closure and by text.
#[derive(Default)]
struct Got {
    word: String,
    exhausted: bool,
    models: Vec<State>,
    open: Vec<State>,
    /// Each model's facts, by text.
    model_facts: Vec<Vec<String>>,
    /// In-process only: the closure of every node the walk narrated as a
    /// `solution` — judged complete — whether or not it was then recorded.
    narrated: BTreeSet<State>,
    /// In-process only: the closure of every node it entered and found alive.
    entered: BTreeSet<State>,
    /// In-process only: how many obligation rules the program states.
    declared: usize,
}

/// Collects the closures of the enterings the walk finds alive, and of those
/// it calls solutions.
struct Narrated<'a> {
    index: FxHashMap<&'a str, u32>,
    seen: BTreeSet<State>,
    alive: BTreeSet<State>,
}

impl ein_infer::solve::Dumper for Narrated<'_> {
    fn entering(
        &mut self,
        _layer: u32,
        _commitment: &[ein_core::FactId],
        terms: &ein_core::Terms,
        outcome: &str,
        info: &ein_infer::solve::EnteringInfo<'_>,
    ) {
        let Some(kb) = info.kb else {
            return;
        };
        if outcome == "solution" || outcome == "alive" {
            let c = closure_of(kb, terms, &self.index);
            if outcome == "solution" {
                self.seen.insert(c.clone());
            }
            self.alive.insert(c);
        }
    }
}

fn closure_of_text(facts: &[String], index: &FxHashMap<&str, u32>) -> State {
    let mut v: Vec<u32> = facts
        .iter()
        .filter_map(|f| index.get(f.as_str()).copied())
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

/// The CLI's answer to a cell, from its `--json-summary`.
fn cli(cell: &Cell, k: &Known, bin: &Path, out: &Path) -> Result<Got, String> {
    let _ = std::fs::remove_dir_all(out);
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let argv = ein_corpus::plan::argv(&cell.run, &cell.path, out);
    let mut cmd = std::process::Command::new(bin);
    cmd.args(&argv).current_dir(repo_root());
    for e in &cell.env {
        let (name, value) = e.split_once('=').expect("NAME=value");
        cmd.env(name, value);
    }
    let status = cmd.output().map_err(|e| e.to_string())?.status;
    let text = std::fs::read_to_string(out.join("summary.json"))
        .map_err(|e| format!("exit {:?}, no summary: {e}", status.code()))?;
    let s: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let v = &s["verdict"];
    let index: FxHashMap<&str, u32> = k
        .candidates
        .iter()
        .enumerate()
        .map(|(i, t)| (t.as_str(), i as u32))
        .collect();
    let facts = |x: &Value| -> Vec<String> {
        x["facts"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|f| f.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    };
    let list = |key: &str| -> Vec<Vec<String>> {
        v[key]
            .as_array()
            .map(|a| a.iter().map(facts).collect())
            .unwrap_or_default()
    };
    let model_facts = list("solutions");
    let open = list("open_states");
    Ok(Got {
        word: v["type"].as_str().unwrap_or("?").to_string(),
        exhausted: v["exhausted"].as_bool().unwrap_or(false),
        models: model_facts
            .iter()
            .map(|m| closure_of_text(m, &index))
            .collect(),
        open: open.iter().map(|m| closure_of_text(m, &index)).collect(),
        model_facts,
        narrated: BTreeSet::new(),
        entered: BTreeSet::new(),
        declared: 0,
    })
}

/// Which of the walk's writes and cuts an in-process run keeps.
#[derive(Clone, Copy, PartialEq)]
enum Variant {
    /// The cell's own flags.
    Own,
    /// No `-n`, `-m` or `-E`.
    Uncut,
    /// And no forced positive, kill cache or singleton writeback.
    Unwritten,
}

fn in_process(path: &Path, k: &Known, f: &Flags, v: Variant) -> Got {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return Got {
            word: "load-error".into(),
            ..Got::default()
        };
    };
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    cfg.enable_pre_branch_lookahead &= !f.no_lookahead;
    cfg.enable_lookahead_kill_cache &= !f.no_kill_cache;
    cfg.lattice_sanity_check |= f.sanity;
    if let Some(o) = &f.order {
        cfg.lattice_order = o.clone();
    }
    let n = k.candidates.len().max(1) as u32;
    let (stop_after, max_set_size, max_enterings, on_budget) = match v {
        Variant::Own => (
            f.stop_after,
            f.max_set_size.unwrap_or(5),
            f.max_enterings,
            OnBudget::Raise,
        ),
        _ => (None, n, Some(ENTERINGS), OnBudget::Verdict),
    };
    if v == Variant::Unwritten {
        cfg.enable_forced_positive = false;
        cfg.enable_lookahead_kill_cache = false;
        cfg.enable_singleton_writeback = false;
    }
    let opts = SolveOptions {
        stop_after,
        max_set_size,
        config: Some(cfg),
        max_enterings,
        on_budget,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let mut narrated = Narrated {
        index: k
            .candidates
            .iter()
            .enumerate()
            .map(|(i, t)| (t.as_str(), i as u32))
            .collect(),
        seen: BTreeSet::new(),
        alive: BTreeSet::new(),
    };
    let solved = match solve(&mut kb, &mut terms, &ast, &mut events, &mut narrated, &opts) {
        Ok(s) => s,
        Err(ein_infer::solve::SolveError::Budget { .. }) => {
            return Got {
                word: "Aborted".into(),
                ..Got::default()
            };
        }
        Err(e) => {
            return Got {
                word: format!("error: {e}"),
                ..Got::default()
            };
        }
    };
    let index: FxHashMap<&str, u32> = k
        .candidates
        .iter()
        .enumerate()
        .map(|(i, t)| (t.as_str(), i as u32))
        .collect();
    let render = |kb: &ein_core::Kb| -> Vec<String> {
        kb.facts()
            .map(|f| ein_infer::events::sexpr(&terms, f))
            .collect()
    };
    match &solved.answer {
        Answer::Aborted { .. } => Got {
            word: "Aborted".into(),
            ..Got::default()
        },
        Answer::Verdict(verdict) => {
            let models = verdict.models();
            Got {
                word: verdict.as_str().to_string(),
                exhausted: solved.stats.exhausted,
                models: models
                    .iter()
                    .map(|s| closure_of(&s.kb, &terms, &index))
                    .collect(),
                open: match verdict {
                    Verdict::Open { states, .. } => states
                        .iter()
                        .map(|s| closure_of(&s.kb, &terms, &index))
                        .collect(),
                    _ => Vec::new(),
                },
                model_facts: models.iter().map(|s| render(&s.kb)).collect(),
                narrated: narrated.seen,
                entered: narrated.alive,
                declared: solved.owes.declared,
            }
        }
    }
}

/// A walk's answer against R's maximal line.
#[derive(Default, Debug)]
struct Judged {
    agree: bool,
    /// Models the run asked for that it did not record.
    max_missing: usize,
    /// Recorded models that are not states of R (C1).
    not_in_r: usize,
    /// Recorded models of R off its maximal line.
    in_r_not_max: usize,
    open_missing: usize,
    open_extra: usize,
    /// The verdict's kind is not R's.
    word: bool,
    /// Judged by the oracle's states rather than by closure.
    by_state: bool,
}

fn kind(word: &str) -> &str {
    match word {
        "Solution" | "Ambiguity" => "model",
        w => w,
    }
}

fn judge(k: &Known, got: &Got, stop: Option<u64>) -> Judged {
    let expected_word = if !k.r.is_empty() {
        "model"
    } else if !k.open_max.is_empty() {
        "Open"
    } else {
        "Contradiction"
    };
    let (in_max, not_in_r, in_r_not_max, by_state) = match (&k.r_facts, &k.max_facts, &k.root_facts)
    {
        (Some(r), Some(max), Some(root)) => {
            let st: BTreeSet<Vec<String>> =
                got.model_facts.iter().map(|m| beyond(m, root)).collect();
            (
                st.intersection(max).count(),
                st.difference(r).count(),
                st.iter()
                    .filter(|s| r.contains(*s) && !max.contains(*s))
                    .count(),
                true,
            )
        }
        _ => {
            let st: BTreeSet<&State> = got.models.iter().collect();
            (
                st.iter().filter(|s| k.max.contains(**s)).count(),
                st.iter().filter(|s| !k.r.contains(**s)).count(),
                st.iter()
                    .filter(|s| k.r.contains(**s) && !k.max.contains(**s))
                    .count(),
                false,
            )
        }
    };
    let max = k.max.len();
    let wanted = stop.map_or(max, |n| max.min(n as usize));
    let open: BTreeSet<&State> = got.open.iter().collect();
    let (open_missing, open_extra) = if k.r.is_empty() {
        (
            k.open_max.iter().filter(|s| !open.contains(s)).count(),
            open.iter().filter(|s| !k.open_max.contains(**s)).count(),
        )
    } else {
        (0, 0)
    };
    let mut j = Judged {
        max_missing: wanted.saturating_sub(in_max),
        not_in_r,
        in_r_not_max,
        open_missing,
        open_extra,
        word: kind(&got.word) != expected_word,
        by_state,
        ..Judged::default()
    };
    j.agree = j.max_missing == 0
        && j.not_in_r == 0
        && j.in_r_not_max == 0
        && j.open_missing == 0
        && j.open_extra == 0
        && !j.word;
    j
}

fn judged_json(j: &Judged) -> Value {
    json!({
        "agree": j.agree,
        "max_missing": j.max_missing,
        "not_in_r": j.not_in_r,
        "in_r_not_max": j.in_r_not_max,
        "open_missing": j.open_missing,
        "open_extra": j.open_extra,
        "word": j.word,
        "by_state": j.by_state,
    })
}

/// Same answer: the verdict, `exhausted`, and the recorded states.
fn same(a: &Got, b: &Got) -> bool {
    let set = |v: &[State]| v.iter().cloned().collect::<BTreeSet<State>>();
    a.word == b.word
        && a.exhausted == b.exhausted
        && set(&a.models) == set(&b.models)
        && set(&a.open) == set(&b.open)
}

fn census(path_rel: &str, cells: &[&Cell], bin: &Path, out: &Path) -> Vec<Value> {
    let path = repo_root().join(path_rel);
    let Some(k) = known(&path, 1, true) else {
        return cells
            .iter()
            .map(|c| json!({"path": c.path, "run": c.run, "env": c.env, "rows": c.rows, "known": false}))
            .collect();
    };
    let mut rows = Vec::new();
    for (i, cell) in cells.iter().enumerate() {
        let f = flags(&cell.run, &cell.env);
        let dir = out.join(format!("{}", i));
        let got = match cli(cell, &k, bin, &dir) {
            Ok(g) => g,
            Err(e) => {
                rows.push(json!({"path": cell.path, "run": cell.run, "env": cell.env, "rows": cell.rows, "known": true, "cli_error": e}));
                continue;
            }
        };
        let j = judge(&k, &got, f.stop_after);
        let own = (!f.tree).then(|| in_process(&path, &k, &f, Variant::Own));
        let cli_matches = own.as_ref().map(|o| same(o, &got));
        let declared = own.as_ref().map_or(0, |o| o.declared);
        let n = k.candidates.len();
        let cut = f.stop_after.is_some()
            || f.max_set_size.unwrap_or(5) < n as u32
            || f.max_enterings.is_some();
        let mut reason: Value = Value::Null;
        let mut attribution = serde_json::Map::new();
        if got.word == "Aborted" {
            reason = json!("budget");
        } else if !j.agree && !f.tree {
            let uncut = in_process(&path, &k, &f, Variant::Uncut);
            let ju = judge(&k, &uncut, None);
            attribution.insert("uncut".into(), judged_json(&ju));
            reason = if cut && ju.agree {
                json!(2)
            } else {
                let unwritten = in_process(&path, &k, &f, Variant::Unwritten);
                let jw = judge(&k, &unwritten, None);
                attribution.insert("unwritten".into(), judged_json(&jw));
                // The walk judged a missing maximal state complete and did not
                // keep it: `record_node` re-saturated the node over the
                // `(not h)` the completeness check wrote into it, and its own
                // rules refuted it (M1e `CO-M1`, Q-M1e.7 ruling A). That is a
                // different node, by a write no flag turns off.
                let recorded: BTreeSet<&State> = unwritten.models.iter().collect();
                let refused: Vec<&State> = k
                    .max
                    .iter()
                    .filter(|s| !recorded.contains(s) && unwritten.narrated.contains(*s))
                    .collect();
                attribution.insert("refused_at_record".into(), json!(refused.len()));
                // A missing maximal state the walk never entered alive: on a
                // flagged program, the walk's prune of every superset of a
                // dead node, which the static check does not license there.
                let never: usize = k
                    .max
                    .iter()
                    .filter(|s| !recorded.contains(s) && !unwritten.entered.contains(*s))
                    .count();
                attribution.insert("missing_never_entered".into(), json!(never));
                if jw.agree {
                    json!(3)
                } else if !refused.is_empty() {
                    json!("3r")
                } else if !k.monotone && never > 0 {
                    json!("prune")
                } else if declared > 0 {
                    json!(5)
                } else {
                    json!(4)
                }
            };
        } else if !j.agree {
            reason = json!("tree");
        } else if k.r != k.max {
            reason = json!(1);
        }
        rows.push(json!({
            "path": cell.path,
            "run": cell.run,
            "env": cell.env,
            "rows": cell.rows,
            "known": true,
            "n": n,
            "route": k.route,
            "monotone": k.monotone,
            "declared": declared,
            "k_s": k.r.len(),
            "k_c": k.k_c,
            "visited": k.visited,
            "max": k.max.len(),
            "open_max": k.open_max.len(),
            "walk": {
                "verdict": got.word,
                "k": got.models.len(),
                "open": got.open.len(),
                "exhausted": got.exhausted,
            },
            "judged": judged_json(&j),
            "cut": cut,
            "cli_matches_in_process": cli_matches,
            "reason": reason,
            "attribution": attribution,
        }));
    }
    rows
}

fn main() {
    let mut jobs = 1usize;
    let mut only: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            _ => only.push(a),
        }
    }
    let root = repo_root();
    let bin = std::env::var_os("EIN_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("ein.rs/target/release/ein"));
    assert!(bin.exists(), "{}: build it first", bin.display());
    let out = root.join("ein.rs/target/walk_census");

    // Every declared walk cell, keyed by (file, run, env).
    let mut cells: BTreeMap<(String, String, Vec<String>), Cell> = BTreeMap::new();
    let mut add = |path: &str, run: &str, env: &[String], row: Option<String>| {
        if !run.starts_with("solve") || REGION_FLAGS.iter().any(|f| run.contains(f)) {
            return;
        }
        let c = cells
            .entry((path.to_string(), run.to_string(), env.to_vec()))
            .or_insert_with(|| Cell {
                path: path.to_string(),
                run: run.to_string(),
                env: env.to_vec(),
                rows: Vec::new(),
            });
        if let Some(r) = row {
            c.rows.push(r);
        }
    };
    for e in &ein_corpus::corpus().entry {
        for run in e.all_runs() {
            // `ein test` walks each query that states an `:expect` with no
            // stop and `-m 5` (`ein-cli/src/test.rs`), which is `solve -e`.
            let run = if run == "test" {
                "solve -e".to_string()
            } else {
                run
            };
            add(&e.path, &run, &[], None);
        }
    }
    for row in ein_corpus::register().case {
        for (label, a) in row.answers() {
            add(
                row.witness_of(a),
                &a.run,
                &a.env,
                Some(format!("{} {label}", row.id)),
            );
        }
    }
    let mut by_path: BTreeMap<String, Vec<&Cell>> = BTreeMap::new();
    for c in cells.values() {
        if only.is_empty() || only.iter().any(|o| c.path.contains(o.as_str())) {
            by_path.entry(c.path.clone()).or_default().push(c);
        }
    }
    let paths: Vec<(&String, &Vec<&Cell>)> = by_path.iter().collect();
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Value>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some((path, cells)) = paths.get(i) else {
                        break;
                    };
                    let dir = out.join(format!("p{i}"));
                    let t = std::time::Instant::now();
                    let rows = census(path, cells, &bin, &dir);
                    eprintln!(
                        "# {}/{} {path} {:.1} s",
                        i + 1,
                        paths.len(),
                        t.elapsed().as_secs_f64()
                    );
                    results.lock().unwrap().extend(rows);
                }
            });
        }
    });
    let mut rows = results.into_inner().unwrap();
    rows.sort_by(|a, b| {
        (a["path"].as_str(), a["run"].as_str()).cmp(&(b["path"].as_str(), b["run"].as_str()))
    });
    for r in &rows {
        println!("{r}");
    }
    report(&rows);
}

/// The census, for the page.
fn report(rows: &[Value]) {
    let known: Vec<&Value> = rows.iter().filter(|r| r["known"] == true).collect();
    let programs = |v: &[&Value]| {
        v.iter()
            .filter_map(|r| r["path"].as_str())
            .collect::<BTreeSet<_>>()
            .len()
    };
    let all: Vec<&Value> = rows.iter().collect();
    eprintln!(
        "cells {} on {} programs; R known for {} cells on {} programs ({} monotone, {} flagged)",
        rows.len(),
        programs(&all),
        known.len(),
        programs(&known),
        programs(
            &known
                .iter()
                .copied()
                .filter(|r| r["monotone"] == true)
                .collect::<Vec<_>>()
        ),
        programs(
            &known
                .iter()
                .copied()
                .filter(|r| r["monotone"] == false)
                .collect::<Vec<_>>()
        ),
    );
    let errors = known
        .iter()
        .filter(|r| r.get("cli_error").is_some())
        .count();
    let mismatched = known
        .iter()
        .filter(|r| r["cli_matches_in_process"] == false)
        .count();
    eprintln!("cli errors {errors}; CLI ≠ in-process with the cell's flags: {mismatched}");
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for r in &known {
        if r.get("cli_error").is_some() {
            continue;
        }
        let key = match (&r["judged"]["agree"], &r["reason"]) {
            (Value::Bool(true), Value::Null) => "agree, R is its maximal line".to_string(),
            (Value::Bool(true), _) => "agree, reason 1 (R holds more)".to_string(),
            (_, reason) => format!("disagree, reason {reason}"),
        };
        *by.entry(key).or_default() += 1;
    }
    for (k, n) in &by {
        eprintln!("  {n:5}  {k}");
    }
    let c1 = known
        .iter()
        .filter(|r| r["judged"]["not_in_r"].as_u64().unwrap_or(0) > 0)
        .count();
    eprintln!("cells recording a model outside R (C1): {c1}");
    eprintln!("\ndisagreements:");
    for r in &known {
        if r["judged"]["agree"] != false {
            continue;
        }
        let j = &r["judged"];
        let w = &r["walk"];
        eprintln!(
            "  [{}] {} `{}`{} n={} route {} | walk {} k={} open={} ex={} | R k_s={} max={} open_max={} | missing {} not-in-R {} off-max {} open -{}/+{}{} | rows {}",
            r["reason"],
            r["path"].as_str().unwrap_or(""),
            r["run"].as_str().unwrap_or(""),
            r["env"]
                .as_array()
                .filter(|a| !a.is_empty())
                .map(|a| format!(" {a:?}"))
                .unwrap_or_default(),
            r["n"],
            r["route"],
            w["verdict"].as_str().unwrap_or(""),
            w["k"],
            w["open"],
            w["exhausted"],
            r["k_s"],
            r["max"],
            r["open_max"],
            j["max_missing"],
            j["not_in_r"],
            j["in_r_not_max"],
            j["open_missing"],
            j["open_extra"],
            if j["word"] == true { " word" } else { "" },
            r["rows"],
        );
    }
}
