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

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::Events;
use ein_infer::downset;
use ein_infer::region::{self, Outcome};
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load, parse};
use rustc_hash::FxHashMap;
use serde_json::{Value, json};

/// Route 1's budget here: enough for `zebra.ein` (270 912 visits).
const BUDGET: u64 = 1 << 20;
/// The walk's, per variant.
const ENTERINGS: u64 = 1 << 21;

type State = Vec<u32>;

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// R and its neighbours, as sets of closures.
#[derive(Default)]
struct Known {
    route: u8,
    monotone: bool,
    candidates: Vec<String>,
    k_c: u64,
    alive: u64,
    /// Every state of R.
    r: BTreeSet<State>,
    /// The states of R's maximal line.
    max: BTreeSet<State>,
    min: BTreeSet<State>,
    /// Alive, owing, and no alive commitment one fact up: what the walk can
    /// record as an open state.
    open_max: BTreeSet<State>,
    alive_states: u64,
    visited: u64,
    /// The status of every commitment the route decided; any other is dead by
    /// the theorem (route 1) or does not occur (route 2 holds them all).
    status: FxHashMap<State, u8>,
}

fn known(path: &Path, jobs: usize) -> Option<Known> {
    let (mut kb, mut terms, ast) = loaded(path)?;
    let d = downset::compute(
        &mut kb,
        &mut terms,
        &ast,
        &downset::Options {
            budget: u64::MAX,
            max_visits: BUDGET,
            jobs,
        },
    )
    .ok()?;
    let candidates: Vec<String> = d.candidates.iter().map(|c| c.text.clone()).collect();
    if let Some(desc) = &d.description {
        let mut k = Known {
            route: 1,
            monotone: true,
            candidates,
            k_c: desc.k_c,
            visited: d.points.len() as u64,
            ..Known::default()
        };
        k.status = d
            .points
            .iter()
            .map(|p| (p.set.to_vec(), p.status))
            .collect();
        let index: FxHashMap<&[u32], usize> = d
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (&p.set[..], i))
            .collect();
        let alive = |s: &[u32]| {
            index
                .get(s)
                .is_some_and(|&i| d.points[i].status & region::DEAD == 0)
        };
        let mut alive_states: BTreeSet<State> = BTreeSet::new();
        for p in &d.points {
            if p.status & region::DEAD != 0 {
                continue;
            }
            k.alive += 1;
            alive_states.insert(p.closure.to_vec());
            if p.in_r() {
                k.r.insert(p.closure.to_vec());
            } else {
                let up = (0..k.candidates.len() as u32)
                    .filter(|b| p.set.binary_search(b).is_err())
                    .any(|b| {
                        let mut s = p.set.to_vec();
                        let at = s.partition_point(|&x| x < b);
                        s.insert(at, b);
                        alive(&s)
                    });
                if !up {
                    k.open_max.insert(p.closure.to_vec());
                }
            }
        }
        k.alive_states = alive_states.len() as u64;
        for &i in &desc.maximal {
            k.max.insert(d.points[i as usize].closure.to_vec());
        }
        for &i in &desc.minimal {
            k.min.insert(d.points[i as usize].closure.to_vec());
        }
        return Some(k);
    }
    if d.n() > region::DEFAULT_LIMIT as usize || d.monotonicity.is_monotone() {
        return None;
    }
    let (mut kb, mut terms, ast) = loaded(path)?;
    let opts = region::Options {
        jobs,
        ..region::Options::default()
    };
    let Ok(Outcome::Region(r)) = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |_| {})
    else {
        return None;
    };
    let n = r.n();
    let bits = |m: u64| -> State { (0..n as u32).filter(|b| m >> b & 1 == 1).collect() };
    let mut k = Known {
        route: 2,
        monotone: false,
        candidates,
        k_c: r.topology.size_r,
        visited: 1 << n,
        ..Known::default()
    };
    let mut alive_states: BTreeSet<State> = BTreeSet::new();
    for m in 0..(1u64 << n) {
        let st = r.status[m as usize];
        k.status.insert(bits(m), st);
        if st & region::DEAD != 0 {
            continue;
        }
        k.alive += 1;
        let closure = bits(r.nodes[m as usize].closure);
        alive_states.insert(closure.clone());
        if st == 0 {
            k.r.insert(closure);
        } else {
            let up = (0..n)
                .any(|b| m >> b & 1 == 0 && r.status[(m | 1 << b) as usize] & region::DEAD == 0);
            if !up {
                k.open_max.insert(closure);
            }
        }
    }
    k.alive_states = alive_states.len() as u64;
    for &m in &r.topology.maximal {
        k.max.insert(bits(r.nodes[m as usize].closure));
    }
    for &m in &r.topology.minimal {
        k.min.insert(bits(r.nodes[m as usize].closure));
    }
    Some(k)
}

/// The candidates a recorded KB holds, by text.
fn closure_of(kb: &Kb, terms: &Terms, index: &FxHashMap<&str, u32>) -> State {
    let mut v: Vec<u32> = kb
        .facts()
        .filter_map(|f| {
            index
                .get(ein_infer::events::sexpr(terms, f).as_str())
                .copied()
        })
        .collect();
    v.sort_unstable();
    v.dedup();
    v
}

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
        let Some(k) = known(&path, jobs) else {
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
