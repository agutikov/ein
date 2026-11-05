//! How far route 1 reaches — M1e2
//! [S1e2.1a.9](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
//! T1e2.1a.9.1, in process.
//!
//! Every corpus file and register witness that loads, through
//! [`ein_infer::downset`] (route 1) at a budget, and through the oracle too
//! where `n ≤ 16` and `--oracle` asks: one JSON line per program, with `n`,
//! the static verdict, what route 1 visited per layer, whether it declined
//! and why, `k_c`, `k_s`, the lines, and the wall clock of each route. The
//! route the default would take is the last field: `1` (route 1 described
//! `R`), `2` (the oracle, `n ≤ 16`), or `3` (neither: partial).
//!
//! ```sh
//! cargo run --release -p ein-infer --example route1_reach -- \
//!     [--budget N] [--facts F] [--jobs J] [--oracle] [FILE…] > rows.jsonl
//! ```
//!
//! `--budget` is in **visited commitments**, default `2¹⁶`, the unit
//! `route1_reach.md`'s sweeps were taken in. `--facts` adds the ruled unit,
//! facts beyond root (`downset::DEFAULT_BUDGET`), and is off unless given.
//!
//! With no `FILE` it sweeps `corpus_files()` plus the register's witnesses.
//! `route1_reach.md`'s tables were read off the rows; no script reads them.
//!
//! A row's `layers` are `[visited, dead, owing, in R, facts, wall]` per layer.
//! Since M1e2 S1e2.1c.1 (`alive_set_reach.md`) a row also carries, read off
//! the class each visited point already has and from the same run:
//! `layer_states`, per layer `[alive states, states in R, alive states first
//! met]`; `alive_states`, the distinct alive states visited; `r_states_met`,
//! the distinct states of R visited (`k_s` when the route finished);
//! `extensions`, the alive states' distinct one-up extensions;
//! `description.generators`, every state's minimal generators summed; and
//! `top`, ⊤'s status, which the route saturates once when the budget stops
//! it. None of it is read by a route.

use std::path::{Path, PathBuf};
use std::time::Instant;

use rustc_hash::FxHashSet;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::downset::{self, Declined};
use ein_infer::region::{self, Outcome};
use ein_ir::{Ast, load, parse};
use serde_json::json;

fn loaded(path: &Path) -> Option<(ein_core::Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn main() {
    let mut budget: u64 = 1 << 16;
    let mut facts = u64::MAX;
    let mut jobs = 1usize;
    let mut oracle = false;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--budget" => {
                budget = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--budget N")
            }
            "--facts" => facts = args.next().and_then(|v| v.parse().ok()).expect("--facts F"),
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            "--oracle" => oracle = true,
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
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let t = Instant::now();
        let opts = downset::Options {
            budget: facts,
            max_visits: budget,
            jobs,
            deadline: None,
            cores: true,
        };
        let d = match downset::compute(&mut kb, &mut terms, &ast, &opts) {
            Ok(d) => d,
            Err(e) => {
                println!("{}", json!({"path": rel, "error": e.to_string()}));
                continue;
            }
        };
        let wall1 = t.elapsed().as_secs_f64();
        let n = d.n();
        let alive = d
            .points
            .iter()
            .filter(|p| p.status & region::DEAD == 0)
            .count();
        let layers: Vec<_> = d
            .layers
            .iter()
            .zip(&d.layer_wall)
            .map(|(l, w)| json!([l.visited, l.dead, l.owing, l.r, l.facts, w.as_secs_f64()]))
            .collect();
        // The states each layer met, read off the class every point carries
        // (the first visited point with the same fixpoint): distinct states
        // among the layer's alive points, among its members of R, and the
        // alive states first met in it. Points are in visit order, a layer
        // at a time, so a layer is a slice.
        let mut start = 0usize;
        let mut seen_alive = 0u64;
        let mut seen_r = 0u64;
        let layer_states: Vec<_> = d
            .layers
            .iter()
            .map(|l| {
                let end = start + l.visited as usize;
                let (mut alive_c, mut r_c) = (FxHashSet::default(), FxHashSet::default());
                let mut new = 0u64;
                for (i, p) in d.points[start..end].iter().enumerate() {
                    if p.status & region::DEAD != 0 {
                        continue;
                    }
                    alive_c.insert(p.class);
                    if p.in_r() {
                        r_c.insert(p.class);
                    }
                    if p.class as usize == start + i {
                        new += 1;
                        seen_alive += 1;
                        if p.in_r() {
                            seen_r += 1;
                        }
                    }
                }
                start = end;
                json!([alive_c.len(), r_c.len(), new])
            })
            .collect();
        // The alive states' one-up extensions, distinct: each alive state's
        // candidate set (its first point's closure) plus one candidate it does
        // not hold. S1e2.1c.2's state route saturates exactly these, beside
        // root, so this counts its work where `walk_and_region.md` § 6 bounded
        // it by alive states × n.
        let mut extensions: FxHashSet<Box<[u32]>> = FxHashSet::default();
        for (i, p) in d.points.iter().enumerate() {
            if p.status & region::DEAD != 0 || p.class as usize != i {
                continue;
            }
            for b in 0..n as u32 {
                if p.closure.binary_search(&b).is_err() {
                    let mut e = p.closure.to_vec();
                    e.insert(e.partition_point(|&x| x < b), b);
                    extensions.insert(e.into_boxed_slice());
                }
            }
        }
        let top = d.top.as_ref().map(|t| {
            json!({
                "dead": t.status & region::DEAD != 0,
                "owing": t.status & region::OWING != 0,
                "owes": t.owes,
                "facts": d.top_facts,
            })
        });
        let facts: u64 = d.layers.iter().map(|l| l.facts).sum();
        let declined = d.declined.as_ref().map(|x| match x {
            Declined::Budget { layer, next, .. } => {
                json!({"why": "budget", "layer": layer, "next": next})
            }
            other => json!({"why": other.as_str()}),
        });
        let desc = d.description.as_ref().map(|x| {
            json!({
                "k_c": x.k_c, "k_s": x.k_s,
                "minimal": x.minimal.len(), "maximal": x.maximal.len(),
                "nogoods": x.nogoods.len(), "components": x.components.len(),
                "subcubes": x.subcubes.len(), "merged": x.merged.len(),
                "empty_in_r": x.minimal.iter().any(|&i| d.points[i as usize].set.is_empty()),
                // Every state's minimal generators, summed: a merged state's
                // list, and one for each state a single commitment reaches.
                "generators": x.merged.iter().map(|m| m.generators.len() as u64).sum::<u64>()
                    + (x.k_s - x.merged.len() as u64),
            })
        });
        let mut wall2 = None;
        let mut oracle_r = None;
        if oracle && n <= region::DEFAULT_LIMIT as usize {
            let (mut kb2, mut terms2, ast2) = loaded(&path).expect("loaded once");
            let t2 = Instant::now();
            let o = region::Options {
                jobs,
                ..region::Options::default()
            };
            if let Ok(Outcome::Region(r)) =
                region::compute(&mut kb2, &mut terms2, &ast2, &o, &mut |_| {})
            {
                wall2 = Some(t2.elapsed().as_secs_f64());
                oracle_r = Some(r.topology.size_r);
            }
        }
        let route = if d.description.is_some() {
            1
        } else if n <= region::DEFAULT_LIMIT as usize {
            2
        } else {
            3
        };
        println!(
            "{}",
            json!({
                "path": rel,
                "n": n,
                "monotone": d.monotonicity.is_monotone(),
                "sites": d.monotonicity.sites.len(),
                "root_dead": d.root_dead,
                "visited": d.points.len(),
                "alive": alive,
                "alive_states": seen_alive,
                "r_states_met": seen_r,
                "extensions": extensions.len(),
                "facts": facts,
                "layers": layers,
                "layer_states": layer_states,
                "declined": declined,
                "top": top,
                "description": desc,
                "wall_route1": wall1,
                "wall_root": d.cost.root.as_secs_f64(),
                "wall_nodes": d.cost.nodes.as_secs_f64(),
                "wall_oracle": wall2,
                "oracle_r": oracle_r,
                "route": route,
            })
        );
    }
}
