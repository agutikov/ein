//! The border route measured against the state route: `price_list
//! --borders`.
//!
//! For every corpus program and register witness that searches, whose static
//! check calls it monotone and whose root is alive, describe and `--count`
//! are each asked twice at the default budget, by the state route and by the
//! border route (`ein_infer::explore::describe_by`), with the options `ein
//! solve` gives them (describe takes `N`, `--count` does not; no core). Per
//! cell: the run's saturations and facts beyond root, whether it was
//! stopped, and on the border route the work of each border and the borders'
//! sizes. Where both routes finish, every field of the two descriptions that
//! is `R`'s or around it is compared, and the fields that differ are named.
//! Where the state route finishes, its walk gives the global borders' sizes:
//! the maximal alive states, the maximal owing states, `N`; and on every
//! program within the oracle's limit the cube gives them too, and its
//! description is compared with the border route's as well.
//!
//! It writes `borders/borders.jsonl` (a line a program) and
//! `borders/borders.md` (the tables and the verdicts) under `--out` (default
//! `build/structure`), and prints the verdicts. The rule is the stage's
//! (`s1f.10.17_describe_by_borders.md` § The rule for *go*): *go* where the
//! total over the programs is lower in saturations and in facts beyond root
//! and no program's cell is higher by more than one tenth in either; a
//! stopped cell counts with what it spent.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use ein_corpus::repo_root;
use ein_infer::described::Described;
use ein_infer::explore::{self, Answer, Ask, Mode, Route, Stopped, Which};
use ein_infer::region;
use serde_json::{Value, json};

use super::prices;

/// The programs read most closely.
const NAMED: [&str; 6] = [
    "examples/beyond_arith/send_more_money.ein",
    "examples/beyond_arith/send_more_money_written.ein",
    "examples/zebra2-minus-15.ein",
    "examples/zebra2-minus-15-obligations.ein",
    "examples/zebra2.ein",
    "examples/zebra.ein",
];

const ORACLE_LIMIT: u32 = 16;

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

/// One route's answer to one question, timed.
fn ask(path: &Path, route: Route, nogoods: bool) -> Result<(Answer, f64), String> {
    let (mut kb, mut terms, ast) = prices::loaded(path).ok_or("does not load")?;
    let opts = explore::Options {
        cores: region::Cores::READ_OUT,
        nogoods,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let t = Instant::now();
    let a =
        explore::describe_by(&mut kb, &mut terms, &ast, &opts, route).map_err(|e| e.to_string())?;
    Ok((a, t.elapsed().as_secs_f64()))
}

/// The fields of two descriptions that are `R`'s or around it and differ,
/// named. The run's own fields (the route, the work, how a stopped run is
/// partial, whether a node met owes) are not compared. Where both say `N`
/// was not taken, `N`'s members are the run's too.
pub fn differ(a: &Described, b: &Described) -> Vec<String> {
    let mut d: Vec<String> = Vec::new();
    let mut check = |name: &str, same: bool| {
        if !same {
            d.push(name.to_string());
        }
    };
    let texts =
        |x: &Described| -> Vec<String> { x.candidates.iter().map(|c| c.text.clone()).collect() };
    check("candidates", texts(a) == texts(b));
    check(
        "monotonicity",
        a.monotonicity.is_monotone() == b.monotonicity.is_monotone(),
    );
    check("exhausted", a.exhausted == b.exhausted);
    check("root", a.root == b.root);
    check("any_alive", a.any_alive == b.any_alive);
    check("k_s", a.k_s() == b.k_s());
    check(
        "states.closure",
        a.states
            .iter()
            .map(|s| &s.closure)
            .eq(b.states.iter().map(|s| &s.closure)),
    );
    check(
        "states.rep",
        a.states
            .iter()
            .map(|s| &s.rep)
            .eq(b.states.iter().map(|s| &s.rep)),
    );
    check(
        "states.commitments",
        a.states
            .iter()
            .map(|s| s.commitments)
            .eq(b.states.iter().map(|s| s.commitments)),
    );
    check(
        "states.generators",
        a.states
            .iter()
            .map(|s| &s.generators)
            .eq(b.states.iter().map(|s| &s.generators)),
    );
    check(
        "states.minimal",
        a.states
            .iter()
            .map(|s| s.minimal)
            .eq(b.states.iter().map(|s| s.minimal)),
    );
    check(
        "states.maximal",
        a.states
            .iter()
            .map(|s| s.maximal)
            .eq(b.states.iter().map(|s| s.maximal)),
    );
    check("k_c", a.k_c == b.k_c);
    check(
        "minimal_commitments",
        a.minimal_commitments == b.minimal_commitments,
    );
    check("nogoods_skipped", a.nogoods_skipped == b.nogoods_skipped);
    if !(a.nogoods_skipped && b.nogoods_skipped) {
        check("nogoods", a.nogoods == b.nogoods);
        check(
            "dead.set",
            a.dead
                .iter()
                .map(|x| &x.set)
                .eq(b.dead.iter().map(|x| &x.set)),
        );
        let cored = |x: &Described| x.dead.iter().all(|y| y.core_status.is_some());
        if cored(a) && cored(b) {
            check(
                "dead.core",
                a.dead
                    .iter()
                    .map(|x| (&x.core, &x.core_status, &x.refuted_by))
                    .eq(b
                        .dead
                        .iter()
                        .map(|x| (&x.core, &x.core_status, &x.refuted_by))),
            );
        }
    }
    check("dead_alone", a.dead_alone == b.dead_alone);
    check("dead_alone_set", a.dead_alone_set == b.dead_alone_set);
    check("dead_met", a.dead_met == b.dead_met);
    check(
        "shape.components",
        a.shape.as_ref().map(|s| &s.components) == b.shape.as_ref().map(|s| &s.components),
    );
    check(
        "shape.subcubes",
        a.shape.as_ref().map(|s| {
            s.subcubes.as_ref().map(|v| {
                let mut v = v.clone();
                v.sort_unstable();
                v
            })
        }) == b.shape.as_ref().map(|s| {
            s.subcubes.as_ref().map(|v| {
                let mut v = v.clone();
                v.sort_unstable();
                v
            })
        }),
    );
    check("frontier", a.frontier == b.frontier);
    d
}

/// The description less what only the cube has (`E`, the holes) and what
/// only the routes have: the cube's, made comparable.
fn of_cube(path: &Path) -> Option<Described> {
    let (mut kb, mut terms, ast) = prices::loaded(path)?;
    let o = region::Options {
        limit: ORACLE_LIMIT,
        keep_states: false,
        jobs: 1,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    match region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {}).ok()? {
        region::Outcome::Region(r) => {
            let mut d = ein_infer::described::of_cube(&r);
            if let Some(s) = d.shape.as_mut() {
                s.discharging = None;
                s.holes = None;
            }
            Some(d)
        }
        _ => None,
    }
}

/// The global borders' sizes, read off the cube: `N`, the maximal alive
/// sets, the maximal owing sets, and over `R`'s states the maximal closed
/// proper subsets and the minimal generators.
fn cube_sizes(path: &Path) -> Option<Value> {
    let (mut kb, mut terms, ast) = prices::loaded(path)?;
    let o = region::Options {
        limit: ORACLE_LIMIT,
        keep_states: false,
        jobs: 1,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    let region::Outcome::Region(r) =
        region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {}).ok()?
    else {
        return None;
    };
    let n = r.n();
    let size = 1u64 << n;
    let alive = |m: u64| r.status[m as usize] & region::DEAD == 0;
    let owing = |m: u64| alive(m) && r.status[m as usize] & region::OWING != 0;
    let up = |m: u64| {
        (0..n)
            .filter(move |&b| m >> b & 1 == 0)
            .map(move |b| m | 1 << b)
    };
    let maximal_alive = (0..size).filter(|&m| alive(m) && !up(m).any(alive)).count();
    let maximal_owing = (0..size).filter(|&m| owing(m) && !up(m).any(owing)).count();
    let closed = |m: u64| alive(m) && r.nodes[m as usize].closure == m;
    let states: Vec<u64> = (0..size)
        .filter(|&m| closed(m) && r.status[m as usize] == 0)
        .collect();
    let mut below = 0u64;
    let mut generators = 0u64;
    for &s in &states {
        let inside: Vec<u64> = (0..size)
            .filter(|&m| m != s && m & s == m && closed(m))
            .collect();
        below += inside
            .iter()
            .filter(|&&m| !inside.iter().any(|&x| x != m && x & m == m))
            .count() as u64;
        let members: Vec<u64> = (0..size)
            .filter(|&m| m & s == m && r.nodes[m as usize].closure == s)
            .collect();
        generators += members
            .iter()
            .filter(|&&m| !members.iter().any(|&x| x != m && x & m == x))
            .count() as u64;
    }
    Some(json!({
        "n": n,
        "states": states.len(),
        "below": below,
        "generators": generators,
        "nogoods": r.topology.nogoods.len(),
        "maximal_alive": maximal_alive,
        "maximal_owing": maximal_owing,
    }))
}

/// The same sizes off the state route's walk, where it finished.
fn walk_sizes(path: &Path) -> Option<Value> {
    let (mut kb, mut terms, ast) = prices::loaded(path)?;
    let opts = ein_infer::downset::Options {
        budget: ein_infer::downset::DEFAULT_BUDGET,
        max_visits: u64::MAX,
        jobs: 1,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    let s = ein_infer::states::compute(&mut kb, &mut terms, &ast, &opts).ok()?;
    if s.declined.is_some() {
        return None;
    }
    let d = ein_infer::states::describe(&s).ok()?;
    let maximal_alive = s.states.iter().filter(|x| x.maximal()).count();
    let maximal_owing = s
        .states
        .iter()
        .filter(|x| !x.in_r() && x.children.iter().all(|&c| s.states[c as usize].in_r()))
        .count();
    let mut parents: Vec<Vec<u32>> = vec![Vec::new(); s.states.len()];
    for (i, x) in s.states.iter().enumerate() {
        for &c in &x.children {
            parents[c as usize].push(i as u32);
        }
    }
    let below: u64 =
        d.r.iter()
            .map(|&i| {
                let st = &s.states[i as usize];
                let edges: Vec<ein_infer::transversal::Bits> = parents[i as usize]
                    .iter()
                    .map(|&p| st.bits.minus(&s.states[p as usize].bits))
                    .collect();
                ein_infer::transversal::minimal_edges(&edges).len() as u64
            })
            .sum();
    let generators: u64 =
        d.r.iter()
            .map(|&i| d.generators[i as usize].len() as u64)
            .sum();
    Some(json!({
        "alive_states": s.states.len(),
        "owing_states": s.states.iter().filter(|x| !x.in_r()).count(),
        "states": d.r.len(),
        "below": below,
        "generators": generators,
        "nogoods": d.nogoods.as_ref().map(Vec::len),
        "maximal_alive": maximal_alive,
        "maximal_owing": maximal_owing,
    }))
}

fn cell(a: &Answer, wall: f64) -> Value {
    let mut v = json!({
        "saturations": a.saturations,
        "facts": a.facts,
        "exhausted": a.exhausted,
        "stopped": stopped(&a.stopped),
        "k_s": a.counts.map(|c| c.k_s),
        "k_c": a.counts.map(|c| c.k_c),
        "wall": wall,
    });
    if let Some(b) = a.borders.as_deref() {
        v["borders"] = json!({
            "work": {
                "states": b.work[0],
                "classes": b.work[1],
                "singles": b.work[2],
                "global": b.work[3],
            },
            "stopped_in": b.stopped_in.map(|x| format!("{x:?}")),
            "states": b.states,
            "below": b.below,
            "generators": b.generators,
            "classes_read": b.classes_read,
            "nogoods": b.nogoods,
            "maximal_alive": b.maximal_alive,
            "global_read": b.global_read,
            "known": b.known,
            "rounds": b.rounds,
            "classes": b.classes,
            "global_met": b.global_met,
        });
    }
    v
}

/// One program: both routes, both questions, and the sizes.
fn program(path: &Path, rel: &str) -> Option<Value> {
    let (states_d, wall_sd) = match ask(path, Route::States, true) {
        Ok(x) => x,
        Err(e) => return Some(json!({"path": rel, "skipped": e})),
    };
    if states_d.root_dead {
        return Some(json!({"path": rel, "skipped": "root is dead"}));
    }
    let mut row = json!({
        "path": rel,
        "n": states_d.candidates.len(),
        "root_owes": states_d.root_owes,
    });
    let mut differs: BTreeMap<&str, Value> = BTreeMap::new();
    for (key, nogoods) in [("describe", true), ("count", false)] {
        let (s, ws) = if nogoods {
            (states_d.clone(), wall_sd)
        } else {
            match ask(path, Route::States, false) {
                Ok(x) => x,
                Err(e) => return Some(json!({"path": rel, "error": e})),
            }
        };
        let (b, wb) = match ask(path, Route::Borders, nogoods) {
            Ok(x) => x,
            Err(e) => {
                row[key] = json!({"states": cell(&s, ws), "borders": {"error": e}});
                continue;
            }
        };
        row[key] = json!({"states": cell(&s, ws), "borders": cell(&b, wb)});
        if let (Some(x), Some(y)) = (s.described.as_deref(), b.described.as_deref())
            && x.exhausted
            && y.exhausted
        {
            differs.insert(key, json!(differ(x, y)));
        }
    }
    row["differ"] = json!(differs);
    if states_d.candidates.len() <= ORACLE_LIMIT as usize {
        if let Some(cube) = of_cube(path) {
            // The border route's describe, held to the cube.
            if let Ok((b, _)) = ask(path, Route::Borders, true)
                && let Some(y) = b.described.as_deref()
                && y.exhausted
            {
                row["against_cube"] = json!(differ(&cube, y));
            }
        }
        row["cube"] = cube_sizes(path).unwrap_or(Value::Null);
    }
    row["walk"] = walk_sizes(path).unwrap_or(Value::Null);
    Some(row)
}

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

/// The verdict for one question: totals both ways in both units, and the
/// programs whose border cell is higher by more than a tenth.
fn verdict(rows: &[Value], key: &str) -> Value {
    let mut tot = [[0u64; 2]; 2];
    let mut losers: Vec<Value> = Vec::new();
    let mut winners = 0u64;
    let mut cells = 0u64;
    for r in rows {
        let c = &r[key];
        if c.is_null() || c["borders"]["saturations"].is_null() {
            continue;
        }
        cells += 1;
        let s = [u(&c["states"]["saturations"]), u(&c["states"]["facts"])];
        let b = [u(&c["borders"]["saturations"]), u(&c["borders"]["facts"])];
        for k in 0..2 {
            tot[0][k] += s[k];
            tot[1][k] += b[k];
        }
        let higher = (0..2).any(|k| b[k] * 10 > s[k] * 11 || (s[k] == 0 && b[k] > 0));
        if higher {
            losers.push(json!({
                "path": r["path"],
                "states": s,
                "borders": b,
                "stopped": [c["states"]["exhausted"] == false, c["borders"]["exhausted"] == false],
            }));
        }
        if b[0] < s[0] {
            winners += 1;
        }
    }
    let lower = tot[1][0] < tot[0][0] && tot[1][1] < tot[0][1];
    json!({
        "cells": cells,
        "total_states": tot[0],
        "total_borders": tot[1],
        "lower_on_the_corpus": lower,
        "programs_lower": winners,
        "losers": losers,
        "go": lower && losers.is_empty(),
    })
}

pub fn run(args: Vec<String>) {
    let mut out = repo_root().join("build/structure");
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = args.into_iter();
    while let Some(a) = args.next() {
        match a.as_str() {
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
    let files: Vec<(PathBuf, String)> = files
        .into_iter()
        .map(|path| {
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
            (path, rel)
        })
        .filter(|(path, _)| super::searches(path))
        .collect();
    let rows: Vec<Option<Value>> = ein_corpus::pool::map(
        "borders",
        &files,
        |(_, rel)| (rel.clone(), "borders".to_string()),
        |(path, rel)| {
            let v = program(path, rel);
            eprintln!("{rel}");
            v
        },
    );
    let rows: Vec<Value> = rows.into_iter().flatten().collect();
    let at = out.join("borders");
    std::fs::create_dir_all(&at).expect("the borders directory");
    let mut lines = String::new();
    for r in &rows {
        let _ = writeln!(lines, "{r}");
    }
    std::fs::write(at.join("borders.jsonl"), lines).expect("borders.jsonl");
    let compared: Vec<Value> = rows
        .iter()
        .filter(|r| r.get("skipped").is_none() && r.get("error").is_none())
        .cloned()
        .collect();
    let verdicts = json!({
        "programs": compared.len(),
        "describe": verdict(&compared, "describe"),
        "count": verdict(&compared, "count"),
    });
    std::fs::write(
        at.join("verdicts.json"),
        serde_json::to_string_pretty(&verdicts).expect("JSON") + "\n",
    )
    .expect("verdicts.json");
    let md = report(&compared, &verdicts);
    std::fs::write(at.join("borders.md"), &md).expect("borders.md");
    print!("{md}");
}

fn report(rows: &[Value], verdicts: &Value) -> String {
    let mut md = String::new();
    let _ = writeln!(md, "# The border route against the state route\n");
    let _ = writeln!(md, "{} programs.\n", rows.len());
    for key in ["describe", "count"] {
        let v = &verdicts[key];
        let _ = writeln!(
            md,
            "## {key}\n\n- cells: {}\n- total, the state route: {:?} (saturations, facts)\n- total, the border route: {:?}\n- lower on the corpus: {}\n- programs lower in saturations: {}\n- higher by more than a tenth: {}\n- **{}**\n",
            v["cells"],
            v["total_states"],
            v["total_borders"],
            v["lower_on_the_corpus"],
            v["programs_lower"],
            v["losers"].as_array().map_or(0, Vec::len),
            if v["go"] == true { "go" } else { "no" },
        );
        let _ = writeln!(
            md,
            "| loser | the state route | the border route | stopped (states, borders) |\n|---|---:|---:|---|"
        );
        for l in v["losers"].as_array().into_iter().flatten() {
            let _ = writeln!(
                md,
                "| `{}` | {} · {} | {} · {} | {} |",
                l["path"].as_str().unwrap_or(""),
                l["states"][0],
                l["states"][1],
                l["borders"][0],
                l["borders"][1],
                l["stopped"],
            );
        }
        let _ = writeln!(md);
    }
    let _ = writeln!(md, "## The named programs\n");
    for name in NAMED {
        let Some(r) = rows.iter().find(|r| r["path"] == name) else {
            continue;
        };
        let _ = writeln!(
            md,
            "### `{name}`\n\n```json\n{}\n```\n",
            serde_json::to_string_pretty(r).unwrap_or_default()
        );
    }
    let _ = writeln!(md, "## Fields that differ\n");
    for r in rows {
        let d = &r["differ"];
        let c = &r["against_cube"];
        let any = |v: &Value| {
            v.as_object().is_some_and(|o| {
                o.values()
                    .any(|x| x.as_array().is_some_and(|a| !a.is_empty()))
            }) || v.as_array().is_some_and(|a| !a.is_empty())
        };
        if any(d) || any(c) {
            let _ = writeln!(
                md,
                "- `{}`: routes {d}, cube {c}",
                r["path"].as_str().unwrap_or("")
            );
        }
    }
    md
}
