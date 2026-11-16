//! The price list: what each question about `R` costs on each corpus
//! program that searches, the part of it the structure of `H` explains,
//! what each object of the structure costs, and what each way of spending
//! it would have added and removed, beside [`pair_layer`](pair_layer.rs).
//! It measures; no route reads any of it.
//!
//! ```sh
//! cargo run --release -p ein-infer --example price_list -- \
//!     [--out DIR] [--no-nodes] [--all] [--deaths-alone] [FILE…]
//! ```
//!
//! `--deaths-alone` asks every question with the routes' memory of what is
//! dead learning from deaths alone (`ein_infer::known_dead::from_deaths_alone`):
//! the work without what a state denies, and each way priced on those nodes.
//! Without it the routes read the denials, and the `denials` way prices what
//! is left of them: the nodes a denial would still remove were the set asked
//! about first closed under the closures the run holds.
//!
//! `--orders [--out DIR] [--only NAME,…] [FILE…]` measures the orders of the
//! obligation tree instead ([`shared/orders.rs`](shared/orders.rs)): each
//! order asked the questions the tree answers on every program, a file an
//! order under `orders/`, `routes` being the order the routes take;
//! `--orders-report [--out DIR]` prints their tables, each against the
//! tree's order of before by the rule below, read two ways.
//!
//! `--borders [--out DIR] [FILE…]` measures describe and `--count` by the
//! border route against the state route instead
//! ([`shared/borders.rs`](shared/borders.rs)): each question by both routes,
//! what each border cost and how large it is, the two descriptions compared
//! where both finish and against the cube within its limit, and the verdicts
//! by the rule below.
//!
//! Every corpus file and register witness that loads and **searches**, or
//! the `FILE`s given (`--all` keeps those that do not search too). For each
//! the structure is read once ([`ein_infer::structure::theory_coupled`], and
//! root's pair layer again node by node), and each of the six questions
//! `ein solve` asks, describe, `--count`, `--single`, `-n 1`, `--minimal`
//! and `--maximal`, is asked twice at the default budget: plain, for its
//! work and its wall, and watched, for its nodes, which
//! [`shared/prices.rs`](shared/prices.rs) splits and prices. A cell the
//! budget stops is priced on what it visited and says it stopped.
//!
//! It writes, under `--out` (default `build/structure` under the repo
//! root):
//!
//! - `prices/prices.jsonl`, one line a program: `structure`, what each
//!   object cost, and `cells`, per question the `work`, the `split` (each
//!   part `[saturations, facts]`), the `detail` counts, each of the `ways`
//!   (`added`, `by` object, `removed`, `with`) and, under `--single` and
//!   `-n 1`, the `ceiling`;
//! - `prices/verdicts.json`, per question and way the corpus totals with
//!   and without the way, the programs whose cell is higher by more than
//!   one tenth, and *go* or *no* in saturations and in facts;
//! - `prices/prices.md`, both as tables, every program in them;
//! - `<program>/prices.json` and `prices.md`, one program's line and its
//!   tables, and `<program>/prices/<question>.nodes.jsonl`, every node the
//!   watched run visited with its part (not with `--no-nodes`).
//!
//! A way is *go* for a question when the corpus total with the way's cost
//! added is lower and no program's cell is higher by more than one tenth,
//! in saturations and in facts beyond root; a stopped cell counts with what
//! it had spent. Progress is a line a program on stderr, and the verdicts'
//! table is printed.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use ein_corpus::repo_root;
use ein_infer::explore::Stopped;
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "shared/prices.rs"]
mod prices;
// What *a program searches* means, by the determinism sweeps' one
// definition.
#[allow(dead_code)]
#[path = "../../ein-render/tests/corpus_ops/searches.rs"]
mod searching;

// The orders of the obligation tree, measured: `--orders`.
#[path = "shared/orders.rs"]
mod orders;

// The border route against the state route: `--borders`.
#[path = "shared/borders.rs"]
mod borders;

use prices::{Cell, PARTS, Read, WAYS};

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

fn searches(path: &Path) -> bool {
    let Some((mut kb, mut terms, ast)) = prices::loaded(path) else {
        return false;
    };
    searching::searches(&ast, &mut terms, &mut kb)
}

/// What each object of the structure cost on one program.
fn structure_row(r: &Read) -> Value {
    let l = &r.theory.root;
    let halves = r.layer_work();
    let small = match &r.theory.small {
        Ok(s) => json!({
            "saturations": s.work.saturations,
            "facts": s.work.facts,
            "pairs_saturated": s.pairs.saturated,
            "excluded": s.excluded.len(),
            "dead_alone": s.dead_alone.len(),
        }),
        Err(e) => json!({"error": e}),
    };
    let cover = r.cover.as_ref().map(|c| {
        json!({
            "groups": c.groups.len(),
            "pairs_of_the_theory": c.excluded.len(),
            "steps": c.steps,
            "cut": c.cut.is_some(),
            "saturations": 0,
        })
    });
    let owed = r.exact.as_ref().map(|x| {
        json!({
            "instances": x.owed.table().len(),
            "licensed": r.licensed.len(),
            "owed_groups": x.owed_groups().len(),
            "groups_nothing_owes": x.unowed().len(),
            "unlicensed": x.unlicensed().len(),
            "grids": x.grids.grids.len(),
            "saturations": 0,
        })
    });
    let coupling = r.coupling.as_ref().map(|c| {
        let three = |e: &[ein_infer::structure::Edge]| e.iter().filter(|e| e.arity() >= 3).count();
        json!({
            "questions": c.questions.len(),
            "edges": c.edges.len(),
            "edges_of_three_or_more": three(&c.edges),
            "one_step_edges": c.one_step.as_ref().map(Vec::len),
            "one_step_edges_of_three_or_more": c.one_step.as_deref().map(three),
            "components": c.components.len(),
            "conflict_components": c.conflict_components.len(),
            "node": c.node.map(|w| json!({
                "saturations": w.saturations,
                "facts": w.facts,
                "applications": w.applications,
            })),
            "node_stopped": c.node_stopped.map(|s| s.as_str()),
        })
    });
    json!({
        "root_layer": {
            "saturations": l.work.saturations,
            "facts": l.work.facts,
            "singles": {"saturations": halves[0][0], "facts": halves[0][1]},
            "pairs": {"saturations": halves[1][0], "facts": halves[1][1]},
            "dead_alone": l.dead_alone.len(),
            "excluded": l.excluded.len(),
            "complete": l.complete(),
            "read_node_by_node": r.layer_same,
            "wall": r.wall_layer,
        },
        "small_kb": small,
        "cover": cover,
        "owed": owed,
        "coupling": coupling,
        "wall": r.wall,
    })
}

fn pair(x: [u64; 2]) -> Value {
    json!({"saturations": x[0], "facts": x[1]})
}

fn cell_row(c: &Cell) -> Value {
    let run = match &c.run {
        Ok(run) => run,
        Err(e) => return json!({"flags": c.flags, "error": e}),
    };
    let base = [run.saturations, run.facts];
    let split: BTreeMap<&str, [u64; 2]> = PARTS
        .iter()
        .zip(c.priced.split)
        .map(|(p, x)| (*p, x))
        .collect();
    let ways: BTreeMap<&str, Value> = c
        .priced
        .ways
        .iter()
        .map(|(name, d)| {
            let by: BTreeMap<&str, Value> = d.by.iter().map(|(k, v)| (*k, pair(*v))).collect();
            (
                *name,
                json!({
                    "applies": d.applies,
                    "exact": d.exact,
                    "added": pair(d.added),
                    "by": by,
                    "removed": pair(d.removed),
                    "with": {
                        "saturations": d.with(base[0], 0),
                        "facts": if d.no_facts { Value::Null } else { json!(d.with(base[1], 1)) },
                    },
                    "counts": d.counts,
                    "note": d.note,
                }),
            )
        })
        .collect();
    json!({
        "flags": c.flags,
        "route": format!("{:?}", run.route),
        "work": pair(base),
        "wall": run.wall,
        "wall_watched": c.wall_watched,
        "finished": run.stopped.is_none(),
        "exhausted": run.exhausted,
        "verdict": run.verdict,
        "stopped": stopped(&run.stopped),
        "states_listed": run.states.len(),
        "same_work_watched": c.same_work,
        "texts_no_candidate_has": c.unmapped,
        "split": split,
        "detail": c.priced.detail,
        "ways": ways,
        "ceiling": c.priced.ceiling.as_ref().map(|x| json!({
            "leaves": x.leaves,
            "floor": x.floor,
            "decided": x.decided,
        })),
    })
}

/// One program, kept for the tables: its row, and what the verdicts read.
struct Program {
    rel: String,
    row: Value,
    cells: Vec<Cell>,
}

fn nodes_file(dir: &Path, r: &Read, c: &Cell) {
    let mut text = String::new();
    for (x, part) in c.nodes.iter().zip(&c.priced.parts) {
        let kind = match x.kind {
            prices::Kind::Dead => "dead",
            prices::Kind::Owing => "owing",
            prices::Kind::InR => "in-r",
        };
        let names =
            |s: &[u32]| -> Vec<&str> { s.iter().map(|&b| &r.texts[b as usize][..]).collect() };
        let line = json!({
            "commitment": names(&x.set),
            "kind": kind,
            "owes": x.owes,
            "facts": x.facts,
            "part": PARTS[*part as usize],
            "core": x.core.as_deref().map(names),
        });
        let _ = writeln!(text, "{line}");
    }
    let at = dir.join("prices");
    std::fs::create_dir_all(&at).expect("the program's prices directory");
    std::fs::write(at.join(format!("{}.nodes.jsonl", c.key)), text).expect("the nodes");
}

// ── Tables ──────────────────────────────────────────────────────────────

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

fn pct(x: u64, of: u64) -> String {
    if of == 0 {
        "–".to_string()
    } else {
        format!("{:.1} %", 100.0 * x as f64 / of as f64)
    }
}

/// One program's tables.
fn program_md(p: &Program) -> String {
    let mut s = String::new();
    let row = &p.row;
    let _ = writeln!(s, "# `{}`: what each question costs\n", p.rel);
    let _ = writeln!(
        s,
        "{} candidates; the static check calls it {}.\n",
        row["n"],
        if row["monotone"].as_bool() == Some(true) {
            "monotone"
        } else {
            "**flagged**: no way of spending is applied, and none is paid for"
        }
    );
    let st = &row["structure"];
    let _ = writeln!(s, "## What each object costs\n");
    let _ = writeln!(s, "| object | saturations | facts beyond root | |");
    let _ = writeln!(s, "|---|---:|---:|---|");
    let l = &st["root_layer"];
    let _ = writeln!(
        s,
        "| root's pair layer, whole | {} | {} | {} dead alone, {} pairs excluded |",
        l["saturations"], l["facts"], l["dead_alone"], l["excluded"]
    );
    let _ = writeln!(
        s,
        "| … root's node and the singles | {} | {} | |",
        l["singles"]["saturations"], l["singles"]["facts"]
    );
    let _ = writeln!(
        s,
        "| … the pairs | {} | {} | |",
        l["pairs"]["saturations"], l["pairs"]["facts"]
    );
    let k = &st["small_kb"];
    if k["error"].is_null() {
        let _ = writeln!(
            s,
            "| the small KB's pair layer | {} | {} | in its own root's terms; {} pairs excluded |",
            k["saturations"], k["facts"], k["excluded"]
        );
    } else {
        let _ = writeln!(
            s,
            "| the small KB's pair layer | | | not read: {} |",
            k["error"]
        );
    }
    if !st["cover"].is_null() {
        let c = &st["cover"];
        let _ = writeln!(
            s,
            "| the cover, beside the small KB's layer | 0 | 0 | {} groups, {} calls of the enumeration |",
            c["groups"], c["steps"]
        );
    }
    if !st["owed"].is_null() {
        let o = &st["owed"];
        let _ = writeln!(
            s,
            "| what root owes, beside the cover | 0 | 0 | {} instances, {} licensed, {} owed groups, {} grids |",
            o["instances"], o["licensed"], o["owed_groups"], o["grids"]
        );
    }
    if !st["coupling"].is_null() {
        let c = &st["coupling"];
        let _ = writeln!(
            s,
            "| the coupling's whole node, beside both layers | {} | {} | {} applications; {} edges, {} one step back; {} components |",
            c["node"]["saturations"],
            c["node"]["facts"],
            c["node"]["applications"],
            c["edges"],
            c["one_step_edges"],
            c["components"]
        );
    }
    let _ = writeln!(s, "\n## The work, and its split\n");
    let _ = writeln!(
        s,
        "Saturations a part. *dead otherwise* is `dead_edge` with `dead_other`.\n"
    );
    let _ = writeln!(
        s,
        "| question | route | saturations | facts | | in R | owing, an owed group | owing otherwise | dead, a pair of the theory | dead, a pair of the puzzle | dead, a candidate dead alone | dead otherwise, inside one edge | dead otherwise | watched run the same |"
    );
    let _ = writeln!(
        s,
        "|---|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---|"
    );
    for c in &p.cells {
        let x = &row["cells"][c.key];
        if !x["error"].is_null() {
            let _ = writeln!(s, "| `{}` | error: {} |", c.flags, x["error"]);
            continue;
        }
        let part = |name: &str| u(&x["split"][name][0]);
        let _ = writeln!(
            s,
            "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            c.flags,
            x["route"].as_str().unwrap_or(""),
            x["work"]["saturations"],
            x["work"]["facts"],
            if x["finished"].as_bool() == Some(true) {
                x["verdict"].as_str().unwrap_or("").to_string()
            } else {
                format!(
                    "**stopped**: {}",
                    x["stopped"]["why"].as_str().unwrap_or("")
                )
            },
            part("in_r"),
            part("owing_owed_group"),
            part("owing_other"),
            part("dead_theory"),
            part("dead_puzzle"),
            part("dead_single"),
            part("dead_edge"),
            part("dead_other"),
            if x["same_work_watched"].as_bool() == Some(true) {
                "yes"
            } else {
                "**no**"
            },
        );
    }
    for c in &p.cells {
        let x = &row["cells"][c.key];
        if !x["error"].is_null() {
            continue;
        }
        let _ = writeln!(s, "\n## `{}`: each way of spending\n", c.flags);
        let base = [u(&x["work"]["saturations"]), u(&x["work"]["facts"])];
        let _ = writeln!(
            s,
            "| way | added: saturations | facts | by object | removed: saturations | facts | with it: saturations | facts | |"
        );
        let _ = writeln!(s, "|---|---:|---:|---|---:|---:|---:|---:|---|");
        for w in WAYS {
            let d = &x["ways"][w];
            if d["applies"].as_bool() != Some(true) {
                let _ = writeln!(
                    s,
                    "| `{w}` | | | not applied | | | {} | {} | |",
                    base[0], base[1]
                );
                continue;
            }
            let by: Vec<String> = d["by"]
                .as_object()
                .map(|o| {
                    o.iter()
                        .filter(|(_, v)| u(&v["saturations"]) > 0)
                        .map(|(k, v)| format!("{k} {}", v["saturations"]))
                        .collect()
                })
                .unwrap_or_default();
            let _ = writeln!(
                s,
                "| `{w}` | {} | {} | {} | {} | {} | {} | {} | {} |",
                d["added"]["saturations"],
                d["added"]["facts"],
                by.join("; "),
                d["removed"]["saturations"],
                d["removed"]["facts"],
                d["with"]["saturations"],
                if d["with"]["facts"].is_null() {
                    "not read".to_string()
                } else {
                    d["with"]["facts"].to_string()
                },
                if d["exact"].as_bool() == Some(true) {
                    ""
                } else {
                    "an estimate"
                },
            );
        }
        let detail: Vec<String> = x["detail"]
            .as_object()
            .map(|o| o.iter().map(|(k, v)| format!("`{k}` {v}")).collect())
            .unwrap_or_default();
        if !detail.is_empty() {
            let _ = writeln!(s, "\nBeside the split: {}.", detail.join(", "));
        }
        if !x["ceiling"].is_null() {
            let _ = writeln!(
                s,
                "\nThe leaves found: {}; root's node and one node a candidate of their commitments, {}.",
                x["ceiling"]["leaves"], x["ceiling"]["floor"]
            );
        }
    }
    s
}

/// A program whose cell is higher with a way by more than one tenth: the
/// program, its cell, the cell with the way, the object that added the
/// most, and whether the cell stopped.
type Loser = (String, u64, u64, String, bool);

/// One way's verdict for one question.
struct Verdict {
    /// `[saturations, facts]`: the corpus total, and with the way.
    base: [u64; 2],
    with: [u64; 2],
    /// The same over the cells no budget stopped.
    base_finished: [u64; 2],
    with_finished: [u64; 2],
    added: [u64; 2],
    removed: [u64; 2],
    by: BTreeMap<String, [u64; 2]>,
    /// The programs whose cell is higher by more than one tenth, a unit.
    losers: [Vec<Loser>; 2],
    /// Programs whose cell is lower, a unit.
    winners: [usize; 2],
    applied: usize,
    /// Facts are not read for the way on some cell.
    no_facts: bool,
    estimate: bool,
    /// A cost of the way is not counted.
    bound: bool,
    /// Where the way reads a core at each death it keeps, which is a search
    /// and no saturation: the cells' plain walls summed, and an estimate of
    /// them with the way, a kept saturation at the plain run's wall a
    /// saturation and a core at the watched run's extra wall a death; and
    /// the programs whose cell of 10 ms or more is slower so by more than
    /// one tenth, each with its two walls.
    wall: Option<[f64; 2]>,
    wall_losers: Vec<(String, f64, f64)>,
}

impl Verdict {
    fn go(&self, unit: usize) -> Option<bool> {
        if unit == 1 && self.no_facts {
            return None;
        }
        Some(self.with[unit] < self.base[unit] && self.losers[unit].is_empty())
    }

    fn go_finished(&self, unit: usize) -> Option<bool> {
        if unit == 1 && self.no_facts {
            return None;
        }
        Some(
            self.with_finished[unit] < self.base_finished[unit]
                && self.losers[unit].iter().all(|l| l.4),
        )
    }

    /// The object whose cost is the most of what the way added.
    fn dearest(&self, unit: usize) -> Option<&str> {
        self.by
            .iter()
            .filter(|(_, v)| v[unit] > 0)
            .max_by_key(|(_, v)| v[unit])
            .map(|(k, _)| k.as_str())
    }
}

fn verdict(programs: &[Program], key: &str, way: &str) -> Verdict {
    let mut v = Verdict {
        base: [0; 2],
        with: [0; 2],
        base_finished: [0; 2],
        with_finished: [0; 2],
        added: [0; 2],
        removed: [0; 2],
        by: BTreeMap::new(),
        losers: [Vec::new(), Vec::new()],
        winners: [0; 2],
        applied: 0,
        no_facts: false,
        estimate: false,
        bound: false,
        wall: None,
        wall_losers: Vec::new(),
    };
    for p in programs {
        let Some(c) = p.cells.iter().find(|c| c.key == key) else {
            continue;
        };
        let Ok(run) = &c.run else {
            continue;
        };
        let base = [run.saturations, run.facts];
        let finished = run.stopped.is_none();
        let d = c
            .priced
            .ways
            .iter()
            .find(|(w, _)| *w == way)
            .map(|(_, d)| d);
        let d = d.filter(|d| d.applies);
        if let Some(d) = d {
            v.applied += 1;
            v.no_facts |= d.no_facts;
            v.estimate |= !d.exact;
            v.bound |= d.bound;
            for (k, x) in &d.by {
                let e = v.by.entry(k.to_string()).or_default();
                e[0] += x[0];
                e[1] += x[1];
            }
        }
        if way.starts_with("death-core") {
            let wall = v.wall.get_or_insert([0.0; 2]);
            let with = match d {
                None => run.wall,
                Some(d) => {
                    let dead = c.priced.detail.get("dead").copied().unwrap_or(0);
                    let read = d.counts.get("cores read").copied().unwrap_or(0);
                    let kept = d.with(run.saturations, 0) as f64 / run.saturations.max(1) as f64;
                    let cores = (c.wall_watched - run.wall).max(0.0);
                    run.wall * kept + cores * read as f64 / dead.max(1) as f64
                }
            };
            wall[0] += run.wall;
            wall[1] += with;
            if run.wall >= 0.01 && with > run.wall * 1.1 {
                v.wall_losers.push((p.rel.clone(), run.wall, with));
            }
        }
        for unit in 0..2 {
            let with = d.map_or(base[unit], |d| d.with(base[unit], unit));
            v.base[unit] += base[unit];
            v.with[unit] += with;
            if finished {
                v.base_finished[unit] += base[unit];
                v.with_finished[unit] += with;
            }
            if let Some(d) = d {
                v.added[unit] += d.added[unit];
                v.removed[unit] += d.removed[unit];
                if with < base[unit] {
                    v.winners[unit] += 1;
                }
                // Higher by more than one tenth.
                if with > base[unit] && 10 * (with - base[unit]) > base[unit] {
                    let dearest =
                        d.by.iter()
                            .max_by_key(|(_, x)| x[unit])
                            .map_or(String::new(), |(k, _)| k.to_string());
                    v.losers[unit].push((p.rel.clone(), base[unit], with, dearest, !finished));
                }
            }
        }
    }
    v
}

fn word(go: Option<bool>) -> &'static str {
    match go {
        Some(true) => "**go**",
        Some(false) => "no",
        None => "not read",
    }
}

fn verdict_json(v: &Verdict) -> Value {
    let losers = |unit: usize| -> Vec<Value> {
        v.losers[unit]
            .iter()
            .map(|(p, base, with, dearest, stopped)| {
                json!({"program": p, "cell": base, "with": with, "dearest": dearest, "stopped": stopped})
            })
            .collect()
    };
    let unit = |i: usize| {
        json!({
            "total": v.base[i],
            "with": v.with[i],
            "added": v.added[i],
            "removed": v.removed[i],
            "total_finished": v.base_finished[i],
            "with_finished": v.with_finished[i],
            "go": v.go(i),
            "go_over_finished_cells": v.go_finished(i),
            "lower_on": v.winners[i],
            "losers": losers(i),
            "dearest": v.dearest(i),
        })
    };
    let go = match (v.go(0), v.go(1)) {
        (Some(a), Some(b)) => json!(a && b),
        (Some(false), None) => json!(false),
        _ => Value::Null,
    };
    let by: BTreeMap<&str, Value> = v.by.iter().map(|(k, x)| (k.as_str(), pair(*x))).collect();
    json!({
        "applied_on": v.applied,
        "estimate": v.estimate,
        "bound": v.bound,
        "wall": v.wall.map(|w| json!({
            "plain": w[0],
            "with_estimated": w[1],
            "slower_by_more_than_a_tenth": v.wall_losers.iter().map(|(p, a, b)| {
                json!({"program": p, "wall": a, "with_estimated": b})
            }).collect::<Vec<_>>(),
        })),
        "go": go,
        "saturations": unit(0),
        "facts": unit(1),
        "by": by,
    })
}

/// The corpus's tables.
fn corpus_md(programs: &[Program], verdicts: &BTreeMap<(usize, usize), Verdict>) -> String {
    let qs = prices::questions();
    let mut s = String::new();
    let _ = writeln!(s, "# The price list\n");
    let flagged = programs
        .iter()
        .filter(|p| p.row["monotone"].as_bool() != Some(true))
        .count();
    let _ = writeln!(
        s,
        "{} programs, {} of them flagged by the static check. Written by `price_list`; every \
         number is a count but a `wall`. A cell is one program under one question, at the \
         default budget.\n",
        programs.len(),
        flagged
    );

    // The work.
    let _ = writeln!(s, "## The work, per question\n");
    let _ = writeln!(
        s,
        "| question | cells | stopped | saturations | facts | in R | owing, an owed group | owing otherwise | dead, a pair of the theory | dead, a pair of the puzzle | dead, a candidate dead alone | dead otherwise, inside one edge | dead otherwise | not the watched run's work |"
    );
    let _ = writeln!(
        s,
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    );
    let mut facts_rows = String::new();
    for (key, flags, _) in qs {
        let (mut cells, mut stop, mut differ) = (0u64, 0u64, 0u64);
        let mut work = [0u64; 2];
        let mut split = [[0u64; 2]; 8];
        for p in programs {
            let Some(c) = p.cells.iter().find(|c| c.key == key) else {
                continue;
            };
            let Ok(run) = &c.run else { continue };
            cells += 1;
            stop += u64::from(run.stopped.is_some());
            differ += u64::from(!c.same_work);
            work[0] += run.saturations;
            work[1] += run.facts;
            for (a, b) in split.iter_mut().zip(c.priced.split) {
                a[0] += b[0];
                a[1] += b[1];
            }
        }
        let cols: Vec<String> = split
            .iter()
            .map(|x| format!("{} ({})", x[0], pct(x[0], work[0])))
            .collect();
        let _ = writeln!(
            s,
            "| `{flags}` | {cells} | {stop} | {} | {} | {} | {differ} |",
            work[0],
            work[1],
            cols.join(" | ")
        );
        let cols: Vec<String> = split
            .iter()
            .map(|x| format!("{} ({})", x[1], pct(x[1], work[1])))
            .collect();
        let _ = writeln!(
            facts_rows,
            "| `{flags}` | {} | {} |",
            work[1],
            cols.join(" | ")
        );
    }
    let _ = writeln!(s, "\nThe same split in facts beyond root:\n");
    let _ = writeln!(
        s,
        "| question | facts | in R | owing, an owed group | owing otherwise | dead, a pair of the theory | dead, a pair of the puzzle | dead, a candidate dead alone | dead otherwise, inside one edge | dead otherwise |"
    );
    let _ = writeln!(s, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    s.push_str(&facts_rows);

    // Beside the split.
    let _ = writeln!(s, "\n## Beside the split, per question\n");
    let mut names: Vec<String> = Vec::new();
    for p in programs {
        for c in &p.cells {
            for k in c.priced.detail.keys() {
                if !names.iter().any(|n| n == k) {
                    names.push(k.to_string());
                }
            }
        }
    }
    names.sort();
    let _ = writeln!(
        s,
        "| count | {} |",
        qs.iter()
            .map(|q| format!("`{}`", q.1))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    let _ = writeln!(s, "|---|{}", "---:|".repeat(qs.len()));
    for name in &names {
        let cols: Vec<String> = qs
            .iter()
            .map(|(key, _, _)| {
                programs
                    .iter()
                    .filter_map(|p| p.cells.iter().find(|c| c.key == *key))
                    .map(|c| c.priced.detail.get(name.as_str()).copied().unwrap_or(0))
                    .sum::<u64>()
                    .to_string()
            })
            .collect();
        let _ = writeln!(s, "| `{name}` | {} |", cols.join(" | "));
    }

    // The objects.
    let _ = writeln!(s, "\n## What each object costs, over the corpus\n");
    let mut tot: BTreeMap<&str, [u64; 2]> = BTreeMap::new();
    for p in programs {
        let st = &p.row["structure"];
        let mut add = |k: &'static str, v: &Value| {
            let e = tot.entry(k).or_default();
            e[0] += u(&v["saturations"]);
            e[1] += u(&v["facts"]);
        };
        add("1 root's pair layer, whole", &st["root_layer"]);
        add(
            "2 … root's node and the singles",
            &st["root_layer"]["singles"],
        );
        add("3 … the pairs", &st["root_layer"]["pairs"]);
        add("4 the small KB's pair layer", &st["small_kb"]);
        add("5 the coupling's whole node", &st["coupling"]["node"]);
    }
    let _ = writeln!(s, "| object | saturations | facts beyond root |");
    let _ = writeln!(s, "|---|---:|---:|");
    for (k, v) in &tot {
        let _ = writeln!(s, "| {} | {} | {} |", &k[2..], v[0], v[1]);
    }
    let _ = writeln!(
        s,
        "\nThe cover and what root owes saturate nothing beyond the layers they are read \
         beside; the small KB's facts are beyond its own root."
    );

    // The verdicts.
    let _ = writeln!(s, "\n## The verdicts\n");
    let _ = writeln!(
        s,
        "*go* where the corpus total with the way is lower and no program's cell is higher by \
         more than one tenth, in saturations and in facts; a stopped cell counts with what it \
         had spent. `lower / higher` are the programs whose cell is lower with the way and \
         those higher by more than a tenth, in saturations.\n"
    );
    let _ = writeln!(
        s,
        "| question | way | applied on | saturations | with the way | added | removed | lower / higher | in saturations | facts | with the way | in facts | **verdict** | the dearest object |"
    );
    let _ = writeln!(
        s,
        "|---|---|---:|---:|---:|---:|---:|---|---|---:|---:|---|---|---|"
    );
    for (qi, (_, flags, _)) in qs.iter().enumerate() {
        for (wi, way) in WAYS.iter().enumerate() {
            let v = &verdicts[&(qi, wi)];
            let both = match (v.go(0), v.go(1)) {
                (Some(true), Some(true)) if v.bound => "go, as a bound: a cost is not counted",
                (Some(true), Some(true)) if v.wall.is_some() => {
                    "go in the work's units; its cost is a core a death, below"
                }
                (Some(true), Some(true)) => "**go**",
                (Some(true), None) => "go in saturations; facts not read",
                _ => "no",
            };
            let _ = writeln!(
                s,
                "| `{flags}` | `{way}`{} | {} | {} | {} | {} | {} | {} / {} | {} | {} | {} | {} | {} | {} |",
                match (v.estimate, v.bound) {
                    (true, _) => " (an estimate)",
                    (_, true) => " (a bound)",
                    _ => "",
                },
                v.applied,
                v.base[0],
                v.with[0],
                v.added[0],
                v.removed[0],
                v.winners[0],
                v.losers[0].len(),
                word(v.go(0)),
                v.base[1],
                if v.no_facts {
                    "not read".to_string()
                } else {
                    v.with[1].to_string()
                },
                word(v.go(1)),
                both,
                v.dearest(0).unwrap_or("–"),
            );
        }
    }

    let _ = writeln!(s, "\n### What a core costs\n");
    let _ = writeln!(
        s,
        "`death-core-pair` and `death-core` add no saturation and read a core at every death \
         they keep, which is a search. In seconds, on the machine the table was taken on: the \
         cells' plain walls summed, and an estimate with the way, a kept saturation at the \
         plain run's wall a saturation and a core at the watched run's extra wall a death; \
         then the programs whose cell of 10 ms or more is slower so by more than one tenth.\n"
    );
    let _ = writeln!(
        s,
        "| question | way | wall | with the way, estimated | slower by more than a tenth |"
    );
    let _ = writeln!(s, "|---|---|---:|---:|---|");
    for (qi, (_, flags, _)) in qs.iter().enumerate() {
        for (wi, way) in WAYS.iter().enumerate() {
            let v = &verdicts[&(qi, wi)];
            let Some(w) = v.wall else { continue };
            let slower: Vec<String> = v
                .wall_losers
                .iter()
                .map(|(p, a, b)| format!("`{p}` {a:.3} → {b:.3}"))
                .collect();
            let _ = writeln!(
                s,
                "| `{flags}` | `{way}` | {:.2} | {:.2} | {}{} |",
                w[0],
                w[1],
                slower.len(),
                if slower.is_empty() {
                    String::new()
                } else {
                    format!(": {}", slower.join("; "))
                }
            );
        }
    }

    let _ = writeln!(s, "\n### Over the cells no budget stopped\n");
    let _ = writeln!(
        s,
        "| question | way | saturations | with the way | in saturations | facts | with the way | in facts |"
    );
    let _ = writeln!(s, "|---|---|---:|---:|---|---:|---:|---|");
    for (qi, (_, flags, _)) in qs.iter().enumerate() {
        for (wi, way) in WAYS.iter().enumerate() {
            let v = &verdicts[&(qi, wi)];
            let _ = writeln!(
                s,
                "| `{flags}` | `{way}` | {} | {} | {} | {} | {} | {} |",
                v.base_finished[0],
                v.with_finished[0],
                word(v.go_finished(0)),
                v.base_finished[1],
                if v.no_facts {
                    "not read".to_string()
                } else {
                    v.with_finished[1].to_string()
                },
                word(v.go_finished(1)),
            );
        }
    }

    let _ = writeln!(s, "\n### The losers, by question and way\n");
    let _ = writeln!(
        s,
        "Every program whose cell is higher by more than one tenth in saturations: the \
         program, its cell and the cell with the way, and the object that added the most.\n"
    );
    for (qi, (_, flags, _)) in qs.iter().enumerate() {
        for (wi, way) in WAYS.iter().enumerate() {
            let v = &verdicts[&(qi, wi)];
            if v.losers[0].is_empty() && v.losers[1].is_empty() {
                continue;
            }
            let list = |unit: usize| -> String {
                v.losers[unit]
                    .iter()
                    .map(|(p, base, with, dearest, stop)| {
                        format!(
                            "`{p}` {base} → {with}{}{}",
                            if dearest.is_empty() {
                                String::new()
                            } else {
                                format!(" ({dearest})")
                            },
                            if *stop { ", stopped" } else { "" }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            let _ = writeln!(
                s,
                "- `{flags}`, `{way}`: **{} in saturations**: {}",
                v.losers[0].len(),
                list(0)
            );
            let only: Vec<&Loser> = v.losers[1]
                .iter()
                .filter(|l| !v.losers[0].iter().any(|m| m.0 == l.0))
                .collect();
            if !only.is_empty() {
                let _ = writeln!(
                    s,
                    "  - and {} more in facts alone: {}",
                    only.len(),
                    only.iter()
                        .map(|(p, base, with, _, _)| format!("`{p}` {base} → {with}"))
                        .collect::<Vec<_>>()
                        .join("; ")
                );
            }
        }
    }

    // The two shares.
    let _ = writeln!(s, "\n## Alive and owing, per question\n");
    let _ = writeln!(
        s,
        "| question | saturations | alive and owing | of it, an owed group's instance | owing no licensed instance | built above a state owing no licensed instance |"
    );
    let _ = writeln!(s, "|---|---:|---:|---:|---:|---:|");
    for (key, flags, _) in qs {
        let mut work = 0u64;
        let (mut group, mut other, mut none, mut above) = (0u64, 0u64, 0u64, 0u64);
        for p in programs {
            let Some(c) = p.cells.iter().find(|c| c.key == key) else {
                continue;
            };
            let Ok(run) = &c.run else { continue };
            work += run.saturations;
            group += c.priced.part("owing_owed_group")[0];
            other += c.priced.part("owing_other")[0];
            none += c
                .priced
                .detail
                .get("owing_no_licensed_instance")
                .copied()
                .unwrap_or(0);
            above += c
                .priced
                .detail
                .get("above_a_state_owing_no_licensed_instance")
                .copied()
                .unwrap_or(0);
        }
        let _ = writeln!(
            s,
            "| `{flags}` | {work} | {} ({}) | {} | {} | {} |",
            group + other,
            pct(group + other, work),
            group,
            none,
            above
        );
    }

    let _ = writeln!(s, "\n## What a best order could not go under\n");
    let _ = writeln!(
        s,
        "Read off the leaves a run found: root's node and one node a candidate of their \
         commitments, the fewest nodes of any tree that reaches them. A run that lists no \
         state has no leaf to read.\n"
    );
    let _ = writeln!(
        s,
        "| question | cells | with a leaf | their saturations | the floor | above the floor | cells with no leaf | their saturations |"
    );
    let _ = writeln!(s, "|---|---:|---:|---:|---:|---:|---:|---:|");
    for (key, flags, _) in qs {
        if !matches!(key, "single" | "n1") {
            continue;
        }
        let (mut cells, mut with, mut sat, mut floor, mut without, mut sat_without) =
            (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        for p in programs {
            let Some(c) = p.cells.iter().find(|c| c.key == key) else {
                continue;
            };
            let (Ok(run), Some(x)) = (&c.run, &c.priced.ceiling) else {
                continue;
            };
            cells += 1;
            if x.leaves > 0 {
                with += 1;
                sat += run.saturations;
                floor += x.floor.min(run.saturations);
            } else {
                without += 1;
                sat_without += run.saturations;
            }
        }
        let _ = writeln!(
            s,
            "| `{flags}` | {cells} | {with} | {sat} | {floor} | {} ({}) | {without} | {sat_without} |",
            sat - floor,
            pct(sat - floor, sat)
        );
    }

    // Every program.
    for (key, flags, _) in qs {
        let _ = writeln!(s, "\n## Every program: `{flags}`\n");
        let _ = writeln!(
            s,
            "`m` monotone, `f` flagged; saturations a part; then the cell with each way, in \
             saturations, `·` where the way is not applied.\n"
        );
        let _ = writeln!(
            s,
            "| program | | n | saturations | facts | | in R | owing: group | owing: other | dead: theory | dead: puzzle | dead: single | dead: edge | dead: other | {} |",
            WAYS.iter()
                .map(|w| format!("`{w}`"))
                .collect::<Vec<_>>()
                .join(" | ")
        );
        let _ = writeln!(
            s,
            "|---|---|---:|---:|---:|---|{}",
            "---:|".repeat(8 + WAYS.len())
        );
        for p in programs {
            let Some(c) = p.cells.iter().find(|c| c.key == key) else {
                continue;
            };
            let m = if p.row["monotone"].as_bool() == Some(true) {
                "m"
            } else {
                "f"
            };
            let run = match &c.run {
                Ok(run) => run,
                Err(e) => {
                    let _ = writeln!(s, "| `{}` | {m} | {} | error: {e} |", p.rel, p.row["n"]);
                    continue;
                }
            };
            let parts: Vec<String> = c.priced.split.iter().map(|x| x[0].to_string()).collect();
            let ways: Vec<String> = c
                .priced
                .ways
                .iter()
                .map(|(_, d)| {
                    if d.applies {
                        d.with(run.saturations, 0).to_string()
                    } else {
                        "·".to_string()
                    }
                })
                .collect();
            let _ = writeln!(
                s,
                "| `{}` | {m} | {} | {} | {} | {} | {} | {} |",
                p.rel,
                p.row["n"],
                run.saturations,
                run.facts,
                match &run.stopped {
                    None => run.verdict.unwrap_or("").to_string(),
                    Some(_) => "stopped".to_string(),
                },
                parts.join(" | "),
                ways.join(" | ")
            );
        }
    }

    let _ = writeln!(s, "\n## Every program: what each object costs\n");
    let _ = writeln!(
        s,
        "Saturations and facts beyond root; the small KB's in its own root's terms.\n"
    );
    let _ = writeln!(
        s,
        "| program | | n | root's layer | facts | … singles | … pairs | the small KB's layer | facts | groups | owed groups | instances not licensed | edges | one step back | components | the node: facts | applications |"
    );
    let _ = writeln!(
        s,
        "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    );
    for p in programs {
        let st = &p.row["structure"];
        let m = if p.row["monotone"].as_bool() == Some(true) {
            "m"
        } else {
            "f"
        };
        let show = |v: &Value| {
            if v.is_null() {
                "–".to_string()
            } else {
                v.to_string()
            }
        };
        let _ = writeln!(
            s,
            "| `{}` | {m} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            p.rel,
            p.row["n"],
            st["root_layer"]["saturations"],
            st["root_layer"]["facts"],
            st["root_layer"]["singles"]["saturations"],
            st["root_layer"]["pairs"]["saturations"],
            show(&st["small_kb"]["saturations"]),
            show(&st["small_kb"]["facts"]),
            show(&st["cover"]["groups"]),
            show(&st["owed"]["owed_groups"]),
            show(&st["owed"]["unlicensed"]),
            show(&st["coupling"]["edges"]),
            show(&st["coupling"]["one_step_edges"]),
            show(&st["coupling"]["components"]),
            show(&st["coupling"]["node"]["facts"]),
            show(&st["coupling"]["node"]["applications"]),
        );
    }
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--borders") {
        borders::run(args.into_iter().filter(|a| a != "--borders").collect());
        return;
    }
    if let Some(mode) = args
        .iter()
        .find(|a| *a == "--orders" || *a == "--orders-report")
        .cloned()
    {
        let rest: Vec<String> = args.into_iter().filter(|a| *a != mode).collect();
        if mode == "--orders" {
            orders::run(rest);
        } else {
            orders::report(rest);
        }
        return;
    }
    let alone = args.iter().any(|a| a == "--deaths-alone");
    let rest: Vec<String> = args.into_iter().filter(|a| a != "--deaths-alone").collect();
    if alone {
        ein_infer::known_dead::from_deaths_alone(|| run(rest));
    } else {
        run(rest);
    }
}

fn run(args: Vec<String>) {
    let mut out = repo_root().join("build/structure");
    let mut nodes = true;
    let mut all = false;
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = args.into_iter();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--out" => out = PathBuf::from(args.next().expect("--out DIR")),
            "--no-nodes" => nodes = false,
            "--all" => all = true,
            _ => files.push(PathBuf::from(a)),
        }
    }
    let named = !files.is_empty();
    if !named {
        files = ein_corpus::corpus_files();
        for row in ein_corpus::register().case {
            for (_, a) in row.answers() {
                files.push(repo_root().join(row.witness_of(a)));
            }
        }
        files.sort();
        files.dedup();
    }
    let mut programs: Vec<Program> = Vec::new();
    for path in files {
        let path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .expect("a working directory")
                .join(path)
        };
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        if !all && !searches(&path) {
            continue;
        }
        let r = match Read::of(&path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{rel}: {e}");
                continue;
            }
        };
        let dir = out.join(rel.trim_start_matches('/').trim_end_matches(".ein"));
        std::fs::create_dir_all(&dir).expect("the program's directory");
        let mut cells: Vec<Cell> = Vec::new();
        let mut denials = prices::Denials::default();
        for (key, flags, mode) in prices::questions() {
            let mut c = prices::cell(&path, &r, key, flags, mode, &mut denials);
            if nodes {
                nodes_file(&dir, &r, &c);
            }
            // The nodes are in their file; the tables read the counts.
            c.nodes = Vec::new();
            c.priced.parts = Vec::new();
            cells.push(c);
        }
        let row = json!({
            "path": rel,
            "n": r.n,
            "monotone": r.monotone(),
            "candidates": r.texts,
            "structure": structure_row(&r),
            "denials_read": {"saturations": denials.saturations},
            "cells": cells.iter().map(|c| (c.key, cell_row(c))).collect::<BTreeMap<_, _>>(),
        });
        let p = Program { rel, row, cells };
        let text = serde_json::to_string_pretty(&p.row).expect("JSON");
        std::fs::write(dir.join("prices.json"), text + "\n").expect("prices.json");
        std::fs::write(dir.join("prices.md"), program_md(&p)).expect("prices.md");
        let sat: Vec<String> = p
            .cells
            .iter()
            .map(|c| match &c.run {
                Ok(run) => run.saturations.to_string(),
                Err(_) => "error".to_string(),
            })
            .collect();
        eprintln!("{}: {}", p.rel, sat.join(" "));
        programs.push(p);
    }

    let at = out.join("prices");
    std::fs::create_dir_all(&at).expect("the prices directory");
    let mut lines = String::new();
    for p in &programs {
        // The corpus's lines leave the candidates' texts to each program's
        // own file.
        let mut row = p.row.clone();
        row.as_object_mut().expect("an object").remove("candidates");
        let _ = writeln!(lines, "{row}");
    }
    std::fs::write(at.join("prices.jsonl"), lines).expect("prices.jsonl");
    let qs = prices::questions();
    let mut verdicts: BTreeMap<(usize, usize), Verdict> = BTreeMap::new();
    let mut table: BTreeMap<&str, BTreeMap<&str, Value>> = BTreeMap::new();
    for (qi, (key, _, _)) in qs.iter().enumerate() {
        for (wi, way) in WAYS.iter().enumerate() {
            let v = verdict(&programs, key, way);
            table.entry(key).or_default().insert(way, verdict_json(&v));
            verdicts.insert((qi, wi), v);
        }
    }
    let text = serde_json::to_string_pretty(&json!({
        "programs": programs.len(),
        "rule": "go when the corpus total with the way's cost added is lower and no program's \
                 cell is higher by more than one tenth, in saturations and in facts beyond root",
        "verdicts": table,
    }))
    .expect("JSON");
    std::fs::write(at.join("verdicts.json"), text + "\n").expect("verdicts.json");
    let md = corpus_md(&programs, &verdicts);
    std::fs::write(at.join("prices.md"), &md).expect("prices.md");
    // The verdicts' table, for the terminal.
    if let (Some(from), Some(to)) = (md.find("## The verdicts"), md.find("### Over the cells")) {
        print!("{}", &md[from..to]);
    }
}
