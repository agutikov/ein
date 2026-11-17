//! Describe's two routes measured against each other: `price_list
//! --describe-routes`.
//!
//! For every corpus program and regression witness that searches, whose static
//! check calls it monotone and whose root is alive, describe and `--count`
//! are each asked twice at the default budget, as `ein solve` asks them:
//! with `--describe-route tree`, the default, which takes the tree and the
//! classes where root owes an instance a licence takes and the state route
//! elsewhere, and with `--describe-route states` (describe takes `N`,
//! `--count` does not, no core). Per cell: the route taken, its saturations
//! and facts beyond root, whether it stopped and in which part, what the
//! tree and the classes found of each part, and **the forecast against what
//! was spent**: each step a route takes is projected before it is taken, its
//! nodes times the facts per node so far, which is the projection its budget
//! check reads ([`ein_infer::explore`]'s `afford`, the state route's before
//! each layer), and the step's facts are read off the run's watch. Where both
//! finish, the two descriptions are compared field by field where both
//! computed the field (`Described::differences`), and on every program within
//! `--region-limit` each against the cube's.
//!
//! It writes `describe/routes.jsonl` (a line a program), `describe/verdicts.json`
//! and `describe/routes.md` under `--out` (default `build/structure`), and
//! prints the tables: the work of the default against the state route per
//! question with every program higher by more than one tenth named, and the
//! forecasts' accuracy.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use ein_corpus::repo_root;
use ein_infer::described::Described;
use ein_infer::events::{Events, Stepped, Watch};
use ein_infer::explore::{self, Answer, Ask, DescribeRoute, Mode, Route, Stopped, Which};
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

/// Every step a run took: `(nodes, facts)`.
struct Steps(Arc<Mutex<Vec<(u64, u64)>>>);

impl Watch for Steps {
    fn step(&mut self, s: &Stepped) {
        if s.visited > 0 {
            self.0
                .lock()
                .expect("one thread")
                .push((s.visited, s.facts));
        }
    }
}

/// The forecast of each step, as the budget check projects it before the
/// step, against the facts the step spent: `(projected, spent)` per step.
fn forecasts(steps: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let (mut sats, mut facts) = (0u64, 0u64);
    steps
        .iter()
        .map(|&(nodes, spent)| {
            let per_node = (facts / sats.max(1)).max(1);
            let projected = nodes.saturating_mul(per_node);
            sats += nodes;
            facts += spent;
            (projected, spent)
        })
        .collect()
}

/// A run's answer, its wall, and its steps.
type Asked = (Answer, f64, Vec<(u64, u64)>);

/// One question, as `ein solve` asks it under `route`, timed and watched.
fn ask(path: &Path, route: DescribeRoute, nogoods: bool) -> Result<Asked, String> {
    let (mut kb, mut terms, ast) = prices::loaded(path).ok_or("does not load")?;
    let opts = explore::Options {
        cores: region::Cores::READ_OUT,
        nogoods,
        describe_route: route,
        ..explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        })
    };
    let kept = Arc::new(Mutex::new(Vec::new()));
    let mut events = Events::off().watched(Box::new(Steps(kept.clone())));
    let t = Instant::now();
    let a = explore::run_with(&mut kb, &mut terms, &ast, &opts, &mut events)
        .map_err(|e| e.to_string())?;
    let wall = t.elapsed().as_secs_f64();
    drop(events);
    let steps = std::mem::take(&mut *kept.lock().expect("the run is over"));
    Ok((a, wall, steps))
}

/// The cube's description, less what only the cube computes.
fn of_cube(path: &Path) -> Option<Described> {
    let (mut kb, mut terms, ast) = prices::loaded(path)?;
    let o = region::Options {
        limit: region::DEFAULT_LIMIT,
        keep_states: false,
        jobs: 1,
        deadline: None,
        cores: region::Cores::READ_OUT,
    };
    match region::compute(&mut kb, &mut terms, &ast, &o, &mut |_| {}).ok()? {
        region::Outcome::Region(r) => Some(ein_infer::described::of_cube(&r)),
        _ => None,
    }
}

fn route_word(r: Route) -> &'static str {
    match r {
        Route::States => "states",
        Route::TreeClasses => "tree",
        Route::DownSet => "1",
        Route::Cube => "2",
        Route::Tree => "obligation tree",
        Route::Root => "root",
    }
}

fn cell(a: &Answer, wall: f64, steps: &[(u64, u64)]) -> Value {
    let d = a.described.as_deref();
    let f = forecasts(steps);
    let mut v = json!({
        "route": route_word(a.route),
        "saturations": a.saturations,
        "facts": a.facts,
        "exhausted": a.exhausted,
        "stopped": stopped(&a.stopped),
        "k_s": a.counts.map(|c| c.k_s),
        "k_c": a.counts.map(|c| c.k_c),
        "n_not_computed": d.is_some_and(|d| d.nogoods_skipped),
        "frontier_not_computed": d.is_some_and(|d| d.frontier_skipped),
        "steps": f,
        "wall": wall,
    });
    if let Some(t) = a.tree_work.as_deref() {
        v["tree"] = json!({
            "work": {"states": t.work[0], "singles": t.work[1], "classes": t.work[2]},
            "stopped_in": t.stopped_in.map(|x| format!("{x:?}")),
            "states": t.states,
            "classes": t.classes,
            "below": t.below,
            "generators": t.generators,
            "classes_read": t.classes_read,
        });
    }
    v
}

/// One program: both routes, both questions.
fn program(path: &Path, rel: &str) -> Value {
    let mut row = json!({"path": rel});
    let mut descriptions: BTreeMap<(&str, &str), Described> = BTreeMap::new();
    for (key, nogoods) in [("describe", true), ("count", false)] {
        for (side, route) in [
            ("default", DescribeRoute::Tree),
            ("states", DescribeRoute::States),
        ] {
            match ask(path, route, nogoods) {
                Ok((a, wall, steps)) => {
                    if !a.monotonicity.is_monotone() || a.root_dead {
                        return json!({"path": rel, "skipped": "flagged or dead root"});
                    }
                    row["n"] = json!(a.candidates.len());
                    row["root_owes"] = json!(a.root_owes);
                    row[key][side] = cell(&a, wall, &steps);
                    if let Some(d) = a.described {
                        descriptions.insert((key, side), *d);
                    }
                }
                Err(e) => row[key][side] = json!({"error": e}),
            }
        }
    }
    for key in ["describe", "count"] {
        if let (Some(x), Some(y)) = (
            descriptions.get(&(key, "default")),
            descriptions.get(&(key, "states")),
        ) && x.exhausted
            && y.exhausted
        {
            row["differ"][key] = json!(x.differences(y));
        }
    }
    let n = row["n"].as_u64().unwrap_or(u64::MAX);
    if n <= u64::from(region::DEFAULT_LIMIT)
        && let Some(cube) = of_cube(path)
    {
        for side in ["default", "states"] {
            if let Some(d) = descriptions.get(&("describe", side))
                && d.exhausted
            {
                row["against_cube"][side] = json!(cube.differences(d));
            }
        }
    }
    row
}

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

/// The rule's reading for one question: totals both ways in both units, and
/// every program whose default cell is higher by more than a tenth.
fn verdict(rows: &[Value], key: &str) -> Value {
    let mut tot = [[0u64; 2]; 2];
    let mut losers: Vec<Value> = Vec::new();
    let mut lower = 0u64;
    let mut changed = 0u64;
    for r in rows {
        let c = &r[key];
        if c["default"]["saturations"].is_null() || c["states"]["saturations"].is_null() {
            continue;
        }
        let s = [u(&c["states"]["saturations"]), u(&c["states"]["facts"])];
        let b = [u(&c["default"]["saturations"]), u(&c["default"]["facts"])];
        for k in 0..2 {
            tot[0][k] += s[k];
            tot[1][k] += b[k];
        }
        if c["default"]["route"] == "tree" {
            changed += 1;
        }
        if (0..2).any(|k| b[k] * 10 > s[k] * 11 || (s[k] == 0 && b[k] > 0)) {
            losers.push(json!({
                "path": r["path"],
                "states": s,
                "default": b,
                "stopped": [c["states"]["exhausted"] == false, c["default"]["exhausted"] == false],
            }));
        }
        if b[0] < s[0] {
            lower += 1;
        }
    }
    json!({
        "total_states": tot[0],
        "total_default": tot[1],
        "programs_on_the_tree": changed,
        "programs_lower": lower,
        "losers": losers,
    })
}

/// The forecasts' accuracy over every step of a route's cells: the steps,
/// the facts spent against the facts projected in all, and the share of
/// steps that spent more than twice or less than half their projection.
fn accuracy(rows: &[Value], route: &str) -> Value {
    let (mut steps, mut projected, mut spent, mut over, mut under) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ratios: Vec<f64> = Vec::new();
    for r in rows {
        for key in ["describe", "count"] {
            for side in ["default", "states"] {
                let c = &r[key][side];
                if c["route"] != route {
                    continue;
                }
                for s in c["steps"].as_array().into_iter().flatten() {
                    let (p, f) = (u(&s[0]), u(&s[1]));
                    steps += 1;
                    projected += p;
                    spent += f;
                    over += u64::from(f > 2 * p);
                    under += u64::from(2 * f < p);
                    ratios.push(f as f64 / p.max(1) as f64);
                }
            }
        }
    }
    ratios.sort_by(|a, b| a.total_cmp(b));
    let at = |q: f64| {
        ratios
            .get(((ratios.len() as f64 - 1.0) * q).round() as usize)
            .copied()
            .unwrap_or(0.0)
    };
    json!({
        "steps": steps,
        "projected": projected,
        "spent": spent,
        "spent_over_twice": over,
        "spent_under_half": under,
        "ratio_median": at(0.5),
        "ratio_p10": at(0.1),
        "ratio_p90": at(0.9),
        "ratio_max": ratios.last().copied().unwrap_or(0.0),
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
        for row in ein_corpus::regressions().case {
            files.push(repo_root().join(&row.witness));
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
    let rows: Vec<Value> = ein_corpus::pool::map(
        "describe-routes",
        &files,
        |(_, rel)| (rel.clone(), "describe-routes".to_string()),
        |(path, rel)| {
            let v = program(path, rel);
            eprintln!("{rel}");
            v
        },
    );
    let at = out.join("describe");
    std::fs::create_dir_all(&at).expect("the describe directory");
    let mut lines = String::new();
    for r in &rows {
        let _ = writeln!(lines, "{r}");
    }
    std::fs::write(at.join("routes.jsonl"), lines).expect("routes.jsonl");
    let compared: Vec<Value> = rows
        .iter()
        .filter(|r| r.get("skipped").is_none())
        .cloned()
        .collect();
    let verdicts = json!({
        "programs": compared.len(),
        "describe": verdict(&compared, "describe"),
        "count": verdict(&compared, "count"),
        "forecast": {
            "states": accuracy(&compared, "states"),
            "tree": accuracy(&compared, "tree"),
        },
    });
    std::fs::write(
        at.join("verdicts.json"),
        serde_json::to_string_pretty(&verdicts).expect("JSON") + "\n",
    )
    .expect("verdicts.json");
    let md = report(&compared, &verdicts);
    std::fs::write(at.join("routes.md"), &md).expect("routes.md");
    print!("{md}");
}

fn report(rows: &[Value], verdicts: &Value) -> String {
    let mut md = String::new();
    let _ = writeln!(md, "# Describe's two routes\n\n{} programs.\n", rows.len());
    for key in ["describe", "count"] {
        let v = &verdicts[key];
        let _ = writeln!(
            md,
            "## {key}\n\n- the state route: {} (saturations, facts)\n- the default: {}\n- programs on the tree and the classes: {}\n- programs lower in saturations: {}\n- higher by more than a tenth: {}\n",
            v["total_states"],
            v["total_default"],
            v["programs_on_the_tree"],
            v["programs_lower"],
            v["losers"].as_array().map_or(0, Vec::len),
        );
        let _ = writeln!(
            md,
            "| higher | the state route | the default | stopped (states, default) |\n|---|---:|---:|---|"
        );
        for l in v["losers"].as_array().into_iter().flatten() {
            let _ = writeln!(
                md,
                "| `{}` | {} · {} | {} · {} | {} |",
                l["path"].as_str().unwrap_or(""),
                l["states"][0],
                l["states"][1],
                l["default"][0],
                l["default"][1],
                l["stopped"],
            );
        }
        let _ = writeln!(md);
    }
    let _ = writeln!(
        md,
        "## The forecasts\n\n```json\n{}\n```\n",
        serde_json::to_string_pretty(&verdicts["forecast"]).unwrap_or_default()
    );
    let _ = writeln!(md, "## The named programs\n");
    for name in NAMED {
        let Some(r) = rows.iter().find(|r| r["path"] == name) else {
            continue;
        };
        let mut r = r.clone();
        for key in ["describe", "count"] {
            for side in ["default", "states"] {
                if let Some(o) = r[key][side].as_object_mut() {
                    o.remove("steps");
                }
            }
        }
        let _ = writeln!(
            md,
            "### `{name}`\n\n```json\n{}\n```\n",
            serde_json::to_string_pretty(&r).unwrap_or_default()
        );
    }
    let _ = writeln!(md, "## Fields that differ\n");
    for r in rows {
        let bad = |v: &Value| {
            v.as_object().is_some_and(|o| {
                o.values()
                    .any(|x| x.as_array().is_some_and(|a| !a.is_empty()))
            })
        };
        if bad(&r["differ"]) || bad(&r["against_cube"]) {
            let _ = writeln!(
                md,
                "- `{}`: routes {}, cube {}",
                r["path"].as_str().unwrap_or(""),
                r["differ"],
                r["against_cube"]
            );
        }
    }
    md
}
