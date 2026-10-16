//! The walk against the region — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md),
//! the user's question of 2026-10-16: *why do the walk and the oracle differ?*
//!
//! For every program whose region is known — route 1 at a budget, or the
//! oracle at `n ≤ 16` where the static check flags it — the walk is run six
//! ways and each recorded state is put on the region by its **closure**, the
//! candidates its KB holds. In a monotone program a state is its closure.
//!
//! | variant | what it is |
//! |---|---|
//! | `solve` | today's `ein solve`: `-n 1`, `-m 5`, the program's config |
//! | `e` | `ein solve -e` |
//! | `e_m` | `-e` with `-m n`: no depth cap |
//! | `e_m_f` | `e_m` with `enable-forced-positive` off: no root promotion |
//! | `e_m_f_k` | `e_m_f` with the lookahead kill cache off (`-K`): no `(not h)` it writes |
//! | `e_m_f_k_w` | `e_m_f_k` with the singleton writeback off: no `(not h)` a dead singleton writes |
//!
//! A disagreement that disappears at a variant is attributed to what that
//! variant removed. One JSON line per program.
//!
//! ```sh
//! cargo run --release -p ein-infer --example walk_vs_region -- [--jobs J] [FILE…] > rows.jsonl
//! ```

#[path = "shared/known.rs"]
mod known;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_corpus::repo_root;
use ein_infer::Events;
use ein_infer::region;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use known::{Known, State, closure_of, known, loaded};
use rustc_hash::FxHashMap;
use serde_json::{Value, json};

/// The walk's, per variant.
const ENTERINGS: u64 = 1 << 21;

fn run_walk(path: &Path, k: &Known, variant: &str, jobs: usize) -> Value {
    let Some((mut kb, mut terms, ast)) = loaded(path) else {
        return json!(null);
    };
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    let (stop_after, max_set_size) = match variant {
        "solve" => (Some(1), 5),
        "e" => (None, 5),
        _ => (None, k.candidates.len().max(1) as u32),
    };
    if variant.starts_with("e_m_f") {
        cfg.enable_forced_positive = false;
    }
    if variant.starts_with("e_m_f_k") {
        cfg.enable_lookahead_kill_cache = false;
    }
    if variant == "e_m_f_k_w" {
        cfg.enable_singleton_writeback = false;
    }
    let opts = SolveOptions {
        store_lattice: true,
        stop_after,
        max_set_size,
        config: Some(cfg),
        max_enterings: Some(ENTERINGS),
        on_budget: OnBudget::Verdict,
        jobs,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let solved = match solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) {
        Ok(s) => s,
        Err(e) => return json!({"error": e.to_string()}),
    };
    let index: FxHashMap<&str, u32> = k
        .candidates
        .iter()
        .enumerate()
        .map(|(i, t)| (t.as_str(), i as u32))
        .collect();
    let (word, models, open): (&str, BTreeSet<State>, BTreeSet<State>) = match &solved.answer {
        Answer::Aborted { .. } => ("Aborted", BTreeSet::new(), BTreeSet::new()),
        Answer::Verdict(v) => {
            let models = v
                .models()
                .into_iter()
                .map(|s| closure_of(&s.kb, &terms, &index))
                .collect();
            let open = match v {
                Verdict::Open { states, .. } => states
                    .iter()
                    .map(|s| closure_of(&s.kb, &terms, &index))
                    .collect(),
                _ => BTreeSet::new(),
            };
            (v.as_str(), models, open)
        }
    };
    if std::env::var_os("WVR_SHOW").is_some() && variant == "e" {
        let name = |st: &State| -> String {
            let v: Vec<&str> = st
                .iter()
                .map(|&b| k.candidates[b as usize].as_str())
                .collect();
            format!("{{{}}}", v.join(" "))
        };
        for (what, set) in [
            ("max", &k.max),
            ("min", &k.min),
            ("walk", &models),
            ("walk-open", &open),
        ] {
            for st in set.iter() {
                eprintln!("  {what:9} {}", name(st));
            }
        }
    }
    // Each entering against R's status of the cube node it stood on: the
    // commitment plus every promotion before it.
    let (mut walk_dead_r_alive, mut walk_alive_r_dead, mut off_cube) = (0u64, 0u64, 0u64);
    if let Some(proof) = &solved.proof {
        for (i, e) in proof.entered.iter().enumerate() {
            let mut st: Vec<u32> = Vec::new();
            let mut ok = true;
            let promoted = proof
                .promotions
                .iter()
                .filter(|p| p.after_enterings <= i)
                .map(|p| p.fact);
            for f in e.commitment.iter().copied().chain(promoted) {
                match index.get(ein_infer::events::sexpr(&terms, f).as_str()) {
                    Some(&b) => st.push(b),
                    None => ok = false,
                }
            }
            if !ok {
                off_cube += 1;
                continue;
            }
            st.sort_unstable();
            st.dedup();
            let r_dead = k.status.get(&st).is_none_or(|&s| s & region::DEAD != 0);
            let w_dead = e.kind != ein_infer::Kind::Alive;
            if w_dead && !r_dead {
                walk_dead_r_alive += 1;
            }
            if !w_dead && r_dead {
                walk_alive_r_dead += 1;
            }
        }
    }
    let missing = k.max.difference(&models).count();
    let not_in_r = models.difference(&k.r).count();
    let in_r_not_max = models
        .iter()
        .filter(|m| k.r.contains(*m) && !k.max.contains(*m))
        .count();
    let open_missing = k.open_max.difference(&open).count();
    let open_extra = open.difference(&k.open_max).count();
    json!({
        "verdict": word,
        "k": models.len(),
        "open": open.len(),
        "exhausted": solved.stats.exhausted,
        "enterings": solved.stats.base.enterings_total,
        "forced_positives": solved.stats.base.forced_positives,
        "alive0": solved.universe.alive0.len(),
        "proposed": solved.universe.proposed.len(),
        "max_equal": missing == 0 && not_in_r == 0 && in_r_not_max == 0,
        "max_missing": missing,
        "not_in_r": not_in_r,
        "in_r_not_max": in_r_not_max,
        "min_equal": models == k.min,
        "walk_dead_r_alive": walk_dead_r_alive,
        "walk_alive_r_dead": walk_alive_r_dead,
        "off_cube": off_cube,
        "open_equal": open_missing == 0 && open_extra == 0,
        "open_missing": open_missing,
        "open_extra": open_extra,
    })
}

fn main() {
    let mut jobs = 1usize;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
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
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some(k) = known(&path, jobs, false) else {
            continue;
        };
        let mut walks = serde_json::Map::new();
        for v in ["solve", "e", "e_m", "e_m_f", "e_m_f_k", "e_m_f_k_w"] {
            walks.insert(v.to_string(), run_walk(&path, &k, v, jobs));
        }
        println!(
            "{}",
            json!({
                "path": rel,
                "n": k.candidates.len(),
                "route": k.route,
                "monotone": k.monotone,
                "visited": k.visited,
                "alive": k.alive,
                "alive_states": k.alive_states,
                "k_c": k.k_c,
                "k_s": k.r.len(),
                "max": k.max.len(),
                "min": k.min.len(),
                "open_max": k.open_max.len(),
                "empty_in_r": k.r.contains(&Vec::new()),
                "walks": walks,
            })
        );
    }
}
