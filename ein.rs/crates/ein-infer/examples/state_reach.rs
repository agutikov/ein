//! How far the state route reaches — M1e2
//! [S1e2.1c.2](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.2_the_lines_by_duality.md),
//! in process, beside [`route1_reach`](route1_reach.rs).
//!
//! Every corpus file and register witness that loads (or the `FILE`s given),
//! through [`ein_infer::states`] at a budget: one JSON line per program with
//! the same names `route1_reach` uses where they mean the same thing — `n`,
//! the static verdict, `visited` (saturations), `facts` (beyond root, summed),
//! `layers` (`[visited, dead, owing, in R, facts, wall]` per layer), `declined`
//! and `top` — plus what is the state route's own: `alive_states`, `r_states`,
//! `pruned` (extensions dead by L1, not saturated), `known` (extensions that
//! were a state already, by L3), `layer_size` and `complete_up_to`.
//!
//! `--route1` runs route 1 on the same program in the same process, at the
//! same budget, and adds its `visited`, `facts` and wall under `route1`, so the
//! two routes compare line for line.
//!
//! ```sh
//! cargo run --release -p ein-infer --example state_reach -- \
//!     [--budget N] [--facts F] [--jobs J] [--route1] [FILE…] > rows.jsonl
//! ```
//!
//! `--budget` is in visited commitments and `--facts` in facts beyond root,
//! both off unless given, as in `route1_reach`. With no `FILE` it sweeps
//! `corpus_files()` plus the register's witnesses. `alive_set_reach.md`'s
//! second half was read off the rows; no script reads them.

use std::path::{Path, PathBuf};
use std::time::Instant;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::downset::{self, Declined};
use ein_infer::states;
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

fn declined(d: &Option<Declined>) -> serde_json::Value {
    d.as_ref().map_or(serde_json::Value::Null, |x| match x {
        Declined::Budget {
            layer,
            next,
            spent,
            projected,
        } => {
            json!({"why": "budget", "layer": layer, "next": next, "spent": spent, "projected": projected})
        }
        other => json!({"why": other.as_str()}),
    })
}

fn main() {
    let mut budget = u64::MAX;
    let mut facts = u64::MAX;
    let mut jobs = 1usize;
    let mut route1 = false;
    let mut by_size = 0usize;
    let mut duality = false;
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
            "--route1" => route1 = true,
            "--duality" => duality = true,
            "--by-size" => {
                by_size = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .expect("--by-size K")
            }
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
    let opts = downset::Options {
        budget: facts,
        max_visits: budget,
        jobs,
        deadline: None,
        cores: true,
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
        let s = match states::compute(&mut kb, &mut terms, &ast, &opts) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", json!({"path": rel, "error": e.to_string()}));
                continue;
            }
        };
        let wall = t.elapsed().as_secs_f64();
        let t_desc = Instant::now();
        let desc = match &s.declined {
            Some(Declined::NotMonotone | Declined::Contradicted { .. }) => None,
            _ => states::describe(&s).ok(),
        };
        let wall_desc = t_desc.elapsed().as_secs_f64();
        let desc = desc.map(|d| {
            let merged: Vec<u32> = d
                .r
                .iter()
                .copied()
                .filter(|&i| d.commitments[i as usize].is_some_and(|c| c > 1))
                .collect();
            json!({
                "exhausted": d.exhausted,
                "cube": d.cube,
                "k_c": d.k_c,
                "k_s": d.r.len(),
                "minimal": d.minimal.len(),
                "minimal_states": d.minimal.iter().map(|x| x.1).collect::<std::collections::BTreeSet<_>>().len(),
                "least": d.minimal.iter().map(|x| x.0.len()).min(),
                "maximal": d.maximal.len(),
                "nogoods": d.nogoods.as_ref().map(Vec::len),
                "components": d.components,
                "subcubes": d.subcubes.as_ref().map(Vec::len),
                "merged": merged.len(),
                "generators": d.r.iter().map(|&i| d.generators[i as usize].len() as u64).sum::<u64>(),
                "frontier": d.frontier.len(),
                "maximal_alive": s.states.iter().filter(|x| x.maximal()).count(),
                "r_by_size": r_by_size(&s, &d, by_size),
                "duality": duality.then(|| dual_check(&s, &d)),
            })
        });
        let layers: Vec<_> = s
            .layers
            .iter()
            .zip(&s.layer_wall)
            .map(|(l, w)| json!([l.visited, l.dead, l.owing, l.r, l.facts, w.as_secs_f64()]))
            .collect();
        let r1 = route1
            .then(|| {
                let (mut kb, mut terms, ast) = loaded(&path)?;
                let t = Instant::now();
                let d = downset::compute(&mut kb, &mut terms, &ast, &opts).ok()?;
                let wall = t.elapsed().as_secs_f64();
                Some(json!({
                    "visited": d.points.len() as u64 + u64::from(d.top.is_some()),
                    "facts": d.layers.iter().map(|l| l.facts).sum::<u64>() + d.top_facts,
                    "declined": declined(&d.declined),
                    "wall": wall,
                }))
            })
            .flatten();
        println!(
            "{}",
            json!({
                "path": rel,
                "n": s.n(),
                "monotone": s.monotonicity.is_monotone(),
                "root_dead": s.root.dead,
                "visited": s.saturations(),
                "facts": s.facts(),
                "alive_states": s.states.len(),
                "r_states": s.states.iter().filter(|x| x.in_r()).count(),
                "dead_saturated": s.dead.len(),
                "pruned": s.pruned,
                "known": s.known,
                "layers": layers,
                "layer_size": s.layer_size,
                "complete_up_to": s.complete_up_to,
                "declined": declined(&s.declined),
                "top": s.top.as_ref().map(|t| json!({"dead": t.dead, "owes": t.owes, "facts": t.facts})),
                "description": desc,
                "wall": wall,
                "wall_description": wall_desc,
                "wall_root": s.cost.root.as_secs_f64(),
                "wall_nodes": s.cost.nodes.as_secs_f64(),
                "route1": r1,
            })
        );
    }
}

/// The commitments of `R` of each size `0 ..= k`, counted from the states:
/// every subset of an extended state of `R` that holds one of its minimal
/// generators, enumerated. It is what route 1's per-layer `in R` column
/// counts, read the other way, so the two routes compare layer by layer where
/// route 1 does not finish (`--by-size K`; off at 0).
fn r_by_size(s: &states::StateRoute, d: &states::Description, k: usize) -> Vec<u64> {
    let mut out = vec![0u64; if k == 0 { 0 } else { k + 1 }];
    for &i in &d.r {
        let st = &s.states[i as usize];
        let gens: Vec<u128> = d.generators[i as usize]
            .iter()
            .map(|g| {
                g.iter()
                    .map(|&b| st.closure.binary_search(&b).expect("in S"))
                    .fold(0u128, |m, j| m | 1 << j)
            })
            .collect();
        let m = st.closure.len();
        for size in 0..out.len().min(m + 1) {
            // Every `size`-subset of S, as a mask over S's positions.
            let mut c: Vec<usize> = (0..size).collect();
            loop {
                let mask = c.iter().fold(0u128, |x, &j| x | 1 << j);
                if gens.iter().any(|&g| g & !mask == 0) {
                    out[size] += 1;
                }
                let mut j = size;
                loop {
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                    if c[j] < m - size + j {
                        break;
                    }
                    if j == 0 {
                        j = usize::MAX;
                        break;
                    }
                }
                if size == 0 || j == usize::MAX || c[j] >= m - size + j {
                    break;
                }
                c[j] += 1;
                for l in j + 1..size {
                    c[l] = c[l - 1] + 1;
                }
            }
        }
    }
    out
}

/// The duality, the other way: the complements of `N`'s minimal transversals
/// are the maximal alive sets, so they must be the maximal alive states this
/// route read off by L4, and kept where discharged, its maximal line
/// (`--duality`). `[holds, |Tr(N)|, seconds]`.
fn dual_check(s: &states::StateRoute, d: &states::Description) -> serde_json::Value {
    use ein_infer::transversal::{Bits, minimal_transversals};
    let n = s.n();
    let Some(nogoods) = &d.nogoods else {
        return serde_json::Value::Null;
    };
    let t = Instant::now();
    let edges: Vec<Bits> = nogoods.iter().map(|g| Bits::of(n, g)).collect();
    let full = Bits::full(n);
    let mut from_n: Vec<Bits> = minimal_transversals(n, &edges)
        .iter()
        .map(|t| full.minus(t))
        .collect();
    let mut read: Vec<Bits> = s
        .states
        .iter()
        .filter(|x| x.maximal())
        .map(|x| x.bits.clone())
        .collect();
    from_n.sort();
    read.sort();
    let line: Vec<Bits> = d
        .maximal
        .iter()
        .map(|&i| s.states[i as usize].bits.clone())
        .collect();
    let kept = from_n
        .iter()
        .filter(|m| s.states.iter().any(|x| x.bits == **m && x.in_r()))
        .count();
    json!([
        from_n == read && kept == line.len(),
        from_n.len(),
        t.elapsed().as_secs_f64()
    ])
}
