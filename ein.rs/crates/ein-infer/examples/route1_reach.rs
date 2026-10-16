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
//! `utils/route1_reach.py` turns the rows into `route1_reach.md`'s tables.

use std::path::{Path, PathBuf};
use std::time::Instant;

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
                "layers": layers,
                "declined": declined,
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
