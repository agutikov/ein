//! The pair layer, measured: what one candidate and two decide at root, over
//! the corpus, in process, beside [`state_reach`](state_reach.rs) and
//! [`route1_reach`](route1_reach.rs).
//!
//! Every corpus file and register witness that loads (or the `FILE`s given),
//! through [`ein_infer::structure::pair_layer`] at a budget: one JSON line per
//! program, and the lists themselves, as rendered text, in
//! `<out>/<program>/pairs.json`, where `<program>` is the file's path from
//! the repo root without its `.ein`.
//!
//! ```sh
//! cargo run --release -p ein-infer --example pair_layer -- \
//!     [--facts F] [--jobs J] [--routes] [--out DIR] [FILE…] > rows.jsonl
//! ```
//!
//! `--facts` is the budget, in facts beyond root summed over the layer's
//! fixpoints, and defaults to the routes' default; `--jobs` is the node
//! function's fan-out, and no line depends on it but in `wall`. `--out`
//! defaults to `build/structure` under the repo root. With no `FILE` it
//! sweeps `corpus_files()` plus the register's witnesses.
//!
//! A line's names: `n`; `monotone`, the static verdict, and under a flagged
//! program `domain`, the sentence that says what the layer is there;
//! `licences`, those that decided a pair without its node; `at_root`,
//! `dead_alone`, `implications`, `equivalence_classes` (and
//! `equivalent_candidates`, the candidates in them); `pairs`, by what decided
//! each (`all`, `dead_member`, `implied`, `saturated`, `left`) and by what a
//! saturated pair is (`excluded`, `interacting`); `open_by_class`, the pairs
//! neither licence decides counted once per pair of single states, which is
//! what L3 read over the equivalence classes would leave of them (not read
//! while `singles_left` is not 0); `in_excluded`, the candidates in at least one
//! excluded pair, and the two shares, `share_excluded` of all pairs and
//! `share_in_excluded` of all candidates; `against_l1` and `against_l3`, a
//! flagged program's pairs the licences would have read wrongly;
//! `saturations` and `facts`, the layer's work in the read-out's units, root's
//! node counted as the routes count it; `stopped` and `singles_left`; `wall`,
//! root's saturation included; `lists`, the file written.
//!
//! `--routes` asks the same program, loaded again each time, at the same
//! budget, a describe, a `--single` and a `--maximal`, and adds each one's
//! `saturations`, `facts`, `wall` and whether it finished under `routes`;
//! and `searches`, whether the program's describe saturates more than root at
//! the determinism sweeps' budget and cube limit, as
//! `ein-render/tests/corpus_ops`' `searches` reads it.

use std::path::{Path, PathBuf};
use std::time::Instant;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Stopped, Which};
use ein_infer::region::Cores;
use ein_infer::structure::{self, PairLayer};
use ein_ir::{Ast, load, parse};
use serde_json::{Value, json};

/// The determinism sweeps' budget and cube limit, `corpus_ops`' `SWEEP_BUDGET`
/// and `SWEEP_LIMIT`: what *searches* is read at.
const SWEEP_BUDGET: u64 = 1 << 14;
const SWEEP_LIMIT: u32 = 10;

fn loaded(path: &Path) -> Option<(ein_core::Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn stopped(s: &Option<Stopped>) -> Value {
    match s {
        None => Value::Null,
        Some(Stopped::Budget { spent, projected }) => {
            json!({"why": "budget", "spent": spent, "projected": projected})
        }
        Some(Stopped::Limit { n, limit }) => json!({"why": "limit", "n": n, "limit": limit}),
        Some(Stopped::Deadline) => json!({"why": "deadline"}),
        Some(Stopped::NoSearch) => json!({"why": "no-search"}),
    }
}

/// One question about `R`, asked of the program loaded again: its work, and
/// whether it finished.
fn route(path: &Path, mode: Mode, facts: u64, jobs: usize) -> Value {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return Value::Null;
    };
    let opts = explore::Options {
        budget: facts,
        jobs,
        cores: Cores::READ_OUT,
        nogoods: false,
        ..explore::Options::new(mode)
    };
    let t = Instant::now();
    match explore::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(a) => json!({
            "saturations": a.saturations,
            "facts": a.facts,
            "finished": a.stopped.is_none(),
            "exhausted": a.exhausted,
            "verdict": a.verdict.map(|v| v.word()),
            "stopped": stopped(&a.stopped),
            "wall": t.elapsed().as_secs_f64(),
        }),
        Err(e) => json!({"error": e.to_string()}),
    }
}

/// `corpus_ops`' `searches`, by the same call.
fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return false;
    };
    let mut opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    opts.budget = SWEEP_BUDGET;
    opts.limit = SWEEP_LIMIT;
    opts.jobs = 1;
    ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
        .is_ok_and(|r| r.answer.saturations > 1)
}

/// The lists, as rendered text.
fn lists(rel: &str, l: &PairLayer) -> Value {
    let one = |v: &[u32]| -> Vec<&str> { v.iter().map(|&b| l.text(b)).collect() };
    let two = |v: &[(u32, u32)]| -> Vec<[&str; 2]> {
        v.iter().map(|&(a, b)| [l.text(a), l.text(b)]).collect()
    };
    json!({
        "program": rel,
        "n": l.n(),
        "monotone": l.monotone,
        "domain": l.domain(),
        "root_dead": l.root_dead,
        "complete": l.complete(),
        "stopped": stopped(&l.stopped),
        "singles_left": l.singles_left,
        "pairs": {
            "all": l.pairs.all,
            "dead_member": l.pairs.dead_member,
            "implied": l.pairs.implied,
            "saturated": l.pairs.saturated,
            "left": l.pairs.left,
        },
        "work": {"saturations": l.work.saturations, "facts": l.work.facts},
        "candidates": l.candidates.iter().map(|c| c.text.as_str()).collect::<Vec<_>>(),
        "at_root": one(&l.at_root),
        "dead_alone": one(&l.dead_alone),
        "implies": two(&l.implies),
        "equivalent": l.equivalent.iter().map(|c| one(c)).collect::<Vec<_>>(),
        "excluded": two(&l.excluded),
        "interacting": two(&l.interacting),
        "against_l1": two(&l.against_l1),
        "against_l3": l
            .against_l3
            .iter()
            .map(|&(a, b, dead)| json!({"pair": [l.text(a), l.text(b)], "dead": dead}))
            .collect::<Vec<_>>(),
    })
}

fn main() {
    let mut facts = ein_infer::downset::DEFAULT_BUDGET;
    let mut jobs = 1usize;
    let mut routes = false;
    let mut out = repo_root().join("build/structure");
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--facts" => facts = args.next().and_then(|v| v.parse().ok()).expect("--facts F"),
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--routes" => routes = true,
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = ein_corpus::corpus_files();
        for row in ein_corpus::register().case {
            for (_, a) in row.answers() {
                files.push(repo_root().join(row.witness_of(a)));
            }
        }
        files.sort();
        files.dedup();
    }
    let opts = structure::Options {
        budget: facts,
        jobs,
        deadline: None,
    };
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let t = Instant::now();
        let l = match structure::pair_layer(&mut kb, &mut terms, &ast, &opts) {
            Ok(l) => l,
            Err(e) => {
                println!("{}", json!({"path": rel, "error": e.to_string()}));
                continue;
            }
        };
        let wall = t.elapsed().as_secs_f64();
        drop((kb, terms, ast));

        // The lists, in a file of the program's own.
        let dir = out.join(rel.trim_start_matches('/').trim_end_matches(".ein"));
        let file = dir.join("pairs.json");
        std::fs::create_dir_all(&dir).expect("the program's directory");
        let text = serde_json::to_string_pretty(&lists(&rel, &l)).expect("JSON");
        std::fs::write(&file, text + "\n").expect("pairs.json");

        // The pairs neither licence decides, counted once per pair of single
        // states: two equivalent candidates have one state, so by L3 their
        // pairs with a third are one node.
        let open_by_class = {
            let n = l.n();
            let mut class: Vec<u32> = (0..n as u32).collect();
            for c in &l.equivalent {
                for &b in c {
                    class[b as usize] = c[0];
                }
            }
            let mut closed = vec![false; n];
            for &b in l.dead_alone.iter().chain(&l.at_root) {
                closed[b as usize] = true;
            }
            let implied: std::collections::HashSet<(u32, u32)> =
                l.implies.iter().copied().collect();
            let mut pairs: std::collections::HashSet<(u32, u32)> = Default::default();
            for a in 0..n as u32 {
                for b in a + 1..n as u32 {
                    if closed[a as usize]
                        || closed[b as usize]
                        || implied.contains(&(a, b))
                        || implied.contains(&(b, a))
                    {
                        continue;
                    }
                    let (x, y) = (class[a as usize], class[b as usize]);
                    pairs.insert((x.min(y), x.max(y)));
                }
            }
            pairs.len()
        };
        let n = l.n() as f64;
        let in_excluded = l.in_an_excluded_pair();
        let share = |x: f64, of: f64| if of > 0.0 { json!(x / of) } else { Value::Null };
        let mut row = json!({
            "path": rel,
            "n": l.n(),
            "monotone": l.monotone,
            "domain": l.domain(),
            "root_dead": l.root_dead,
            "licences": l.licences.iter().map(|x| x.label()).collect::<Vec<_>>(),
            "at_root": l.at_root.len(),
            "dead_alone": l.dead_alone.len(),
            "singles_left": l.singles_left,
            "implications": l.implies.len(),
            "equivalence_classes": l.equivalent.len(),
            "equivalent_candidates": l.equivalent.iter().map(Vec::len).sum::<usize>(),
            "pairs": {
                "all": l.pairs.all,
                "dead_member": l.pairs.dead_member,
                "implied": l.pairs.implied,
                "saturated": l.pairs.saturated,
                "excluded": l.excluded.len(),
                "interacting": l.interacting.len(),
                "left": l.pairs.left,
            },
            "open_by_class": open_by_class,
            "in_excluded": in_excluded,
            "share_excluded": share(l.excluded.len() as f64, l.pairs.all as f64),
            "share_in_excluded": share(in_excluded as f64, n),
            "against_l1": l.against_l1.len(),
            "against_l3": l.against_l3.len(),
            "saturations": l.work.saturations,
            "facts": l.work.facts,
            "stopped": stopped(&l.stopped),
            "wall": wall,
            "lists": file.strip_prefix(repo_root()).unwrap_or(&file).to_string_lossy(),
        });
        if routes {
            let ask = |which, ask| route(&path, Mode { which, ask }, facts, jobs);
            row["searches"] = json!(searches(&path));
            row["routes"] = json!({
                "describe": ask(Which::All, Ask::Describe),
                "single": ask(Which::All, Ask::Single),
                "maximal": ask(Which::Maximal, Ask::Describe),
            });
        }
        println!("{row}");
    }
}
