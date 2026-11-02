//! The hole census — M1e2
//! [S1e2.1b.3](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.3_holes.md):
//! every hole of every flagged cube the oracle reaches, by kind and by cause,
//! each attributed to the site that made it, and R's shape around its holes.
//!
//! A **hole** is a node outside R with a node of R below it and one above
//! ([`region::Topology`]'s `holes`). Its **kind** is the status bit that puts
//! it out, `dead` before `owing`. Its **cause** and its **site** are
//! `shared/attribution.rs`'s, the rule `site_census` reads too. Per program it
//! prints:
//!
//! - every hole by kind and cause, and each minimal hole with its why, its
//!   cause, its site and its repair edges (a fact whose addition puts it in R);
//! - where the one-fact test (`local_max`, `local_min`) disagrees with the cube;
//! - R's components, its maximal sub-cubes, and its two lines set against its
//!   **convex pieces**: the maximal subsets of R that are convex (every node
//!   between two members is a member) and connected by one-fact edges. On a
//!   convex component the piece is the component. The pieces of a component
//!   with a hole between two of its members can overlap;
//! - each hole's status built from scratch (`utils/region.py`'s node: the
//!   program's text with the commitment written in as given facts), against
//!   the oracle's fork, node for node (`--no-scratch` skips it);
//! - with `--drafts`, each reporting option of F21's D4 drafted as read-out
//!   rows on the program: (a) R, its lines and its holes named, (b) R as
//!   convex pieces, (c) a qualifier, (d) a refusal. **Drafted**: no command
//!   prints them.
//!
//! ```sh
//! cargo run --release -p ein-infer --example hole_census                         # the flagged cubes within --region-limit 16
//! cargo run --release -p ein-infer --example hole_census -- --limit 19           # and lattice/06_friends_tables, n = 19
//! cargo run --release -p ein-infer --example hole_census -- --written            # the programs in shared/written.rs
//! cargo run --release -p ein-infer --example hole_census -- [--json] [--drafts] FILE…
//! ```
//!
//! With no `FILE` it sweeps `site_census`'s selection, `corpus_files()` plus
//! the register's witnesses, and keeps the programs the static check flags.
//! It writes nothing, and no route reads it.

#[allow(dead_code)]
#[path = "shared/attribution.rs"]
mod attribution;
#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;
#[allow(dead_code)]
#[path = "shared/written.rs"]
mod written;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use attribution::{Attributed, Cause, RootPlans, SiteText};
use ein_core::{Kb, Terms};
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::region::{self, DEAD, OWING, Outcome, Region};
use ein_infer::saturator::{Saturator, Session};
use ein_infer::{contradiction, obligations};
use ein_ir::{Ast, load, parse};
use serde_json::{Value as Json, json};

/// A program as text, with the directory its imports resolve from.
#[derive(Clone)]
struct Source {
    name: String,
    text: String,
    dir: Option<PathBuf>,
}

impl Source {
    fn of(path: &Path) -> Option<Source> {
        Some(Source {
            name: census::relative(path),
            text: std::fs::read_to_string(path).ok()?,
            dir: path.parent().map(Path::to_path_buf),
        })
    }

    /// Parse and load the text plus `extra`.
    fn load(&self, extra: &str) -> Option<(Kb, Terms, Ast)> {
        let text = if extra.is_empty() {
            self.text.clone()
        } else {
            format!("{}\n{extra}\n", self.text)
        };
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let forms = parse(&mut ast, &text, Some(&self.name)).ok()?;
        let kb = load(&mut ast, &mut terms, &forms, self.dir.as_deref()).ok()?;
        Some((kb, terms, ast))
    }
}

// ── The scratch route ───────────────────────────────────────────────

/// One node from scratch: the text plus the commitment as given facts,
/// saturated from nothing, read to its two bits.
fn scratch_status(src: &Source, commitment: &str) -> Option<u8> {
    let (mut kb, mut terms, ast) = src.load(commitment)?;
    let memo = SharedMemo::default();
    let mut events = Events::off();
    {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut events,
            memo: memo.clone(),
        };
        let mut sat = Saturator::new(&mut s).ok()?;
        sat.saturate(&mut s, None, &mut |_| {}).ok()?;
    }
    let dead = contradiction::has_contradiction(&kb, &terms);
    let owes = obligations::tally(&kb, &mut terms, &ast, &memo, &mut Events::off())
        .ok()?
        .total();
    Some((if dead { DEAD } else { 0 }) | (if owes > 0 { OWING } else { 0 }))
}

/// Every node of the cube from scratch, on `jobs` threads.
fn scratch_statuses(src: &Source, cands: &[String], jobs: usize) -> Vec<Option<u8>> {
    let size = 1usize << cands.len();
    let mut out: Vec<Option<u8>> = vec![None; size];
    let jobs = jobs.max(1).min(size);
    let chunk = size.div_ceil(jobs);
    std::thread::scope(|sc| {
        for (j, slot) in out.chunks_mut(chunk).enumerate() {
            sc.spawn(move || {
                for (k, r) in slot.iter_mut().enumerate() {
                    let mask = (j * chunk + k) as u64;
                    let text: Vec<&str> = (0..cands.len())
                        .filter(|b| mask >> b & 1 == 1)
                        .map(|b| cands[b].as_str())
                        .collect();
                    *r = scratch_status(src, &text.join(" "));
                }
            });
        }
    });
    out
}

// ── R's shape ───────────────────────────────────────────────────────

/// One convex piece: its members and its own two lines.
struct Piece {
    members: Vec<u64>,
    minimal: Vec<u64>,
    maximal: Vec<u64>,
}

fn piece_of(members: Vec<u64>) -> Piece {
    let minimal = members
        .iter()
        .copied()
        .filter(|&a| !members.iter().any(|&b| b != a && b & a == b))
        .collect();
    let maximal = members
        .iter()
        .copied()
        .filter(|&a| !members.iter().any(|&b| b != a && b & a == a))
        .collect();
    Piece {
        members,
        minimal,
        maximal,
    }
}

/// Every node between two members is a member.
fn convex(set: &BTreeSet<u64>) -> bool {
    for &a in set {
        for &b in set {
            if a == b || a & b != a {
                continue;
            }
            let free = b ^ a;
            let mut sub = free;
            loop {
                if !set.contains(&(a | sub)) {
                    return false;
                }
                if sub == 0 {
                    break;
                }
                sub = (sub - 1) & free;
            }
        }
    }
    true
}

/// Connected by one-fact edges inside the set.
fn connected(set: &BTreeSet<u64>) -> bool {
    let Some(&first) = set.iter().next() else {
        return true;
    };
    let mut seen = BTreeSet::from([first]);
    let mut stack = vec![first];
    while let Some(x) = stack.pop() {
        for &y in set {
            if (x ^ y).count_ones() == 1 && seen.insert(y) {
                stack.push(y);
            }
        }
    }
    seen.len() == set.len()
}

/// The most members a non-convex component may have for its pieces to be
/// enumerated: `2^k` subsets.
const PIECE_LIMIT: usize = 16;

/// R's convex pieces: per component, the component itself when no hole lies
/// between two of its members, else its maximal convex connected subsets.
/// `None` past [`PIECE_LIMIT`].
fn pieces(r: &Region) -> Option<Vec<Piece>> {
    let t = &r.topology;
    let mut out = Vec::new();
    for ci in 0..t.components.len() {
        let members: Vec<u64> = (0..r.status.len() as u64)
            .filter(|&m| t.component_of[m as usize] == ci as u32)
            .collect();
        let split = t
            .holes
            .iter()
            .any(|&h| members.iter().any(|&a| a & !h == 0) && members.iter().any(|&b| h & !b == 0));
        if !split {
            out.push(piece_of(members));
            continue;
        }
        let k = members.len();
        if k > PIECE_LIMIT {
            return None;
        }
        let mut good: Vec<u32> = Vec::new();
        for s in 1u32..(1 << k) {
            let set: BTreeSet<u64> = (0..k)
                .filter(|i| s >> i & 1 == 1)
                .map(|i| members[i])
                .collect();
            if connected(&set) && convex(&set) {
                good.push(s);
            }
        }
        for &s in &good {
            if good.iter().any(|&o| o != s && o & s == s) {
                continue;
            }
            out.push(piece_of(
                (0..k)
                    .filter(|i| s >> i & 1 == 1)
                    .map(|i| members[i])
                    .collect(),
            ));
        }
    }
    Some(out)
}

fn names(r: &Region, ms: &[u64]) -> Vec<String> {
    ms.iter().map(|&m| r.name(m)).collect()
}

// ── One program ─────────────────────────────────────────────────────

/// One hole, attributed.
struct Hole {
    mask: u64,
    kind: &'static str,
    also_owes: bool,
    why: String,
    attributed: Attributed,
}

fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// What one minimal hole's repair edges are: `+fact → node of R`.
fn repairs_of(r: &Region, h: u64) -> Vec<(String, String)> {
    r.topology
        .repairs
        .iter()
        .filter(|&&(x, _, _)| x == h)
        .map(|&(_, b, y)| (r.candidates[b].text.clone(), r.name(y)))
        .collect()
}

/// The four options of F21's D4, drafted as the rows each would print.
fn drafts(
    r: &Region,
    src: &Source,
    sites: &[SiteText],
    minimal: &[&Hole],
    holes: &[Hole],
    pieces: Option<&[Piece]>,
) -> Json {
    let n_sites = plural(sites.len(), "site", "sites");
    let (dead, owing) = (
        holes.iter().filter(|h| h.kind == "dead").count(),
        holes.iter().filter(|h| h.kind == "owing").count(),
    );
    let kinds = match (dead, owing) {
        (0, o) => format!("{o} owing"),
        (d, 0) => format!("{d} dead"),
        (d, o) => format!("{d} dead, {o} owing"),
    };
    let row = |label: &str, text: String| format!("  {label:<17} {text}");
    // Today's rows: the stub and up to four minimal holes.
    let mut today = vec![row(
        "lattice",
        format!(
            "not monotone: {n_sites} (the static check) — {}; assumes a monotone lattice for what a node is (P1e2.1b)",
            plural(holes.len(), "hole", "holes")
        ),
    )];
    for h in minimal.iter().take(4) {
        today.push(row(
            "  minimal hole",
            format!("{} — {}", r.name(h.mask), h.why),
        ));
    }
    // (a) R, its lines named as R's extremes, every minimal hole with its
    // cause, its site and its repair edges; dead and owing named apart.
    let mut a = vec![row(
        "lattice",
        format!(
            "not monotone: {n_sites} (the static check) — {} ({kinds}): the lines are R's own minimal and maximal states, and R is not every node between them",
            plural(holes.len(), "hole", "holes")
        ),
    )];
    for h in minimal {
        let causes: Vec<&str> = h.attributed.causes.iter().map(|c| c.words()).collect();
        a.push(row(
            &format!("  {} hole", h.kind),
            format!(
                "{} — {}; {}: {}",
                r.name(h.mask),
                h.why,
                causes.join(", "),
                h.attributed.sites.join("; ")
            ),
        ));
        let rep: Vec<String> = repairs_of(r, h.mask)
            .into_iter()
            .map(|(f, _)| format!("+{f}"))
            .collect();
        a.push(row("", format!("  repaired by {}", rep.join(", "))));
    }
    // (b) R as its convex pieces, each with its own two lines.
    let b = match pieces {
        Some(ps) => {
            let mut seen: BTreeMap<u64, usize> = BTreeMap::new();
            for p in ps {
                for &m in &p.members {
                    *seen.entry(m).or_default() += 1;
                }
            }
            let shared = seen.values().filter(|&&c| c > 1).count();
            let mut v = vec![row(
                "pieces",
                format!(
                    "{}, each described by its own two lines{}",
                    plural(ps.len(), "convex piece", "convex pieces"),
                    if shared > 0 {
                        format!(
                            "; {} in more than one",
                            plural(shared, "state lies", "states lie")
                        )
                    } else {
                        String::new()
                    }
                ),
            )];
            for (i, p) in ps.iter().enumerate() {
                v.push(row(
                    &format!("  piece {} of {}", i + 1, ps.len()),
                    format!(
                        "{} — minimal {} · maximal {}",
                        plural(p.members.len(), "state", "states"),
                        names(r, &p.minimal).join(", "),
                        names(r, &p.maximal).join(", ")
                    ),
                ));
            }
            v
        }
        None => vec![row(
            "pieces",
            "not computed: a component past the enumeration's bound".into(),
        )],
    };
    // (c) A qualifier: the stub reworded, the hole rows unchanged.
    let mut c = vec![row(
        "lattice",
        format!(
            "not monotone: {n_sites} (the static check) — {}: the two lines bound R and do not describe it; R is the listing",
            plural(holes.len(), "hole", "holes")
        ),
    )];
    c.extend(today.iter().skip(1).cloned());
    // (d) A refusal: a load error naming the sites.
    let d = vec![format!(
        "error: {}: not monotone, so R can have a hole — refused: {}",
        src.name,
        sites
            .iter()
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    )];
    json!({ "today": today, "a": a, "b": b, "c": c, "d_stderr": d })
}

/// The census of one program, or `None` when it does not load, does not
/// saturate, is monotone, or is past `limit`.
fn row(src: &Source, limit: u32, jobs: usize, scratch: bool, with_drafts: bool) -> Json {
    let skip = |why: &str| json!({ "path": src.name, "skipped": why });
    let Some((mut kb, mut terms, ast)) = src.load("") else {
        return skip("does not load");
    };
    let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
        return skip("does not saturate");
    };
    if root.monotonicity.is_monotone() {
        return skip("monotone");
    }
    let root_plans = RootPlans::of(&root.plans, &terms);

    let Some((mut kb_r, mut terms_r, ast_r)) = src.load("") else {
        return skip("does not load twice");
    };
    let opts = region::Options {
        limit,
        jobs,
        ..region::Options::default()
    };
    let t0 = Instant::now();
    let r = match region::compute(&mut kb_r, &mut terms_r, &ast_r, &opts, &mut |_| {}) {
        Ok(Outcome::Region(r)) => r,
        Ok(Outcome::Refused { n, .. }) => {
            return json!({ "path": src.name, "skipped": "past the limit", "n": n });
        }
        Err(e) => return json!({ "path": src.name, "skipped": format!("{e}") }),
    };
    let cube_s = t0.elapsed().as_secs_f64();
    let t = &r.topology;
    let sites = attribution::site_texts(&r.monotonicity.sites, &terms_r);
    let mut nodes =
        region::Nodes::new(&mut kb_r, &mut terms_r, &ast_r, &r.candidates).expect("prepares");

    // Every hole, attributed.
    let minimal_set: BTreeSet<u64> = t.minimal_holes.iter().copied().collect();
    let mut holes: Vec<Hole> = Vec::new();
    for &h in &t.holes {
        let status = r.status[h as usize];
        let node = &r.nodes[h as usize];
        let kind = if status & DEAD != 0 { "dead" } else { "owing" };
        let why = if kind == "dead" {
            format!("dead: {}", node.core.join(" "))
        } else {
            format!("owes {}: {}", node.owes, node.owed.join("; "))
        };
        let attributed = attribution::node(
            &r,
            &mut nodes,
            &ast_r,
            &sites,
            &root_plans,
            h,
            attribution::r_below(&r, h),
        );
        holes.push(Hole {
            mask: h,
            kind,
            also_owes: kind == "dead" && status & OWING != 0,
            why,
            attributed,
        });
    }
    drop(nodes);
    let minimal: Vec<&Hole> = holes
        .iter()
        .filter(|h| minimal_set.contains(&h.mask))
        .collect();

    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for h in &holes {
        let key = if h.attributed.causes.is_empty() {
            format!("{} · no site", h.kind)
        } else {
            let c: Vec<&str> = h.attributed.causes.iter().map(|c| c.key()).collect();
            format!("{} · {}", h.kind, c.join(" + "))
        };
        *by.entry(key).or_default() += 1;
    }

    // Fork and scratch, node for node.
    let cands: Vec<String> = r.candidates.iter().map(|c| c.text.clone()).collect();
    let fork_scratch = if scratch {
        let t1 = Instant::now();
        let s = scratch_statuses(src, &cands, jobs);
        let failed = s.iter().filter(|x| x.is_none()).count();
        let differ: Vec<String> = (0..s.len())
            .filter(|&m| s[m].is_some_and(|x| x != r.status[m]))
            .map(|m| r.name(m as u64))
            .collect();
        let scratch_status: Vec<u8> = s.iter().map(|x| x.unwrap_or(u8::MAX)).collect();
        let scratch_holes = if failed == 0 {
            Some(names(
                &r,
                &region::Topology::of(r.n(), &scratch_status).holes,
            ))
        } else {
            None
        };
        json!({
            "nodes": s.len(),
            "failed": failed,
            "status_differs": differ.len(),
            "differing": differ.iter().take(8).collect::<Vec<_>>(),
            "holes_same": scratch_holes.as_ref().map(|h| *h == names(&r, &t.holes)),
            "seconds": (t1.elapsed().as_secs_f64() * 10.0).round() / 10.0,
        })
    } else {
        Json::Null
    };

    let ps = pieces(&r);
    let pieces_json = ps.as_ref().map(|ps| {
        let mut seen: BTreeMap<u64, usize> = BTreeMap::new();
        for p in ps {
            for &m in &p.members {
                *seen.entry(m).or_default() += 1;
            }
        }
        json!({
            "count": ps.len(),
            "in_more_than_one": seen.values().filter(|&&c| c > 1).count(),
            "lines": ps.iter().map(|p| p.minimal.len() + p.maximal.len()).sum::<usize>(),
            "each": ps.iter().map(|p| json!({
                "size": p.members.len(),
                "minimal": names(&r, &p.minimal),
                "maximal": names(&r, &p.maximal),
            })).collect::<Vec<_>>(),
        })
    });

    let drafted = with_drafts.then(|| drafts(&r, src, &sites, &minimal, &holes, ps.as_deref()));

    json!({
        "path": src.name,
        "n": r.n(),
        "size_r": t.size_r,
        "cube_seconds": (cube_s * 100.0).round() / 100.0,
        "sites": sites.iter().map(|s| s.text.clone()).collect::<Vec<_>>(),
        "holes": {
            "total": holes.len(),
            "dead": holes.iter().filter(|h| h.kind == "dead").count(),
            "owing": holes.iter().filter(|h| h.kind == "owing").count(),
            "dead_and_owing": holes.iter().filter(|h| h.also_owes).count(),
            "by_cause": by,
            "unattributed": holes.iter().filter(|h| !h.attributed.named()).count(),
        },
        "minimal_holes": minimal.iter().map(|h| json!({
            "node": r.name(h.mask),
            "kind": h.kind,
            "why": h.why,
            "causes": h.attributed.causes.iter().map(|c| c.key()).collect::<Vec<_>>(),
            "sites": h.attributed.sites,
            "repairs": repairs_of(&r, h.mask).into_iter().map(|(f, y)| format!("+{f} → {y}")).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "repairs": t.repairs.len(),
        "local_max": names(&r, &t.local_max),
        "local_min": names(&r, &t.local_min),
        "components": t.components.iter().map(|c| c.size).collect::<Vec<_>>(),
        "subcubes": t.subcubes.as_ref().map(|s| s.iter().map(|&(a, b)| (b ^ a).count_ones()).collect::<Vec<_>>()),
        "lines": {
            "minimal": t.minimal.len(),
            "maximal": t.maximal.len(),
            "between": t.size_r as usize + holes.len(),
            "duality": [t.duality.minimal_holds, t.duality.maximal_holds],
        },
        "pieces": pieces_json,
        "fork_scratch": fork_scratch,
        "drafts": drafted,
    })
}

fn print_block(row: &Json) {
    let s = |v: &Json| v.as_str().unwrap_or("").to_string();
    let list = |v: &Json| -> Vec<String> { v.as_array().into_iter().flatten().map(s).collect() };
    let h = &row["holes"];
    println!(
        "{}\n  n {}  |R| {}  holes {} (dead {}, owing {}{})  minimal {}  repairs {}  — {:.2} s",
        s(&row["path"]),
        row["n"],
        row["size_r"],
        h["total"],
        h["dead"],
        h["owing"],
        if h["dead_and_owing"].as_u64().unwrap_or(0) > 0 {
            format!(", dead and owing {}", h["dead_and_owing"])
        } else {
            String::new()
        },
        row["minimal_holes"].as_array().map_or(0, Vec::len),
        row["repairs"],
        row["cube_seconds"].as_f64().unwrap_or(0.0),
    );
    for site in list(&row["sites"]) {
        println!("  site {site}");
    }
    if let Some(by) = h["by_cause"].as_object() {
        let v: Vec<String> = by.iter().map(|(k, c)| format!("{k} {c}")).collect();
        println!(
            "  by cause: {}",
            if v.is_empty() {
                "—".to_string()
            } else {
                v.join(", ")
            }
        );
    }
    for m in row["minimal_holes"].as_array().into_iter().flatten() {
        println!(
            "    minimal hole {} ({}) — {}\n      cause {} ← {}\n      repaired by {}",
            s(&m["node"]),
            s(&m["kind"]),
            s(&m["why"]),
            list(&m["causes"]).join(" + "),
            {
                let v = list(&m["sites"]);
                if v.is_empty() {
                    "no site".to_string()
                } else {
                    v.join("; ")
                }
            },
            list(&m["repairs"]).join(", "),
        );
    }
    let l = &row["lines"];
    println!(
        "  lines: minimal {} · maximal {} · {} nodes between them, {} of them in R; duality minimal {} maximal {}",
        l["minimal"], l["maximal"], l["between"], row["size_r"], l["duality"][0], l["duality"][1]
    );
    println!(
        "  one-fact test against the cube: local_max {:?}  local_min {:?}",
        list(&row["local_max"]),
        list(&row["local_min"])
    );
    println!(
        "  components {}  sub-cubes {}",
        row["components"],
        if row["subcubes"].is_null() {
            "not computed".to_string()
        } else {
            row["subcubes"].to_string()
        }
    );
    let p = &row["pieces"];
    if p.is_object() {
        println!(
            "  convex pieces {} ({} line members; {} states in more than one)",
            p["count"], p["lines"], p["in_more_than_one"]
        );
        for (i, e) in p["each"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .take(24)
        {
            println!(
                "    piece {}: {} states, minimal {} · maximal {}",
                i + 1,
                e["size"],
                list(&e["minimal"]).join(", "),
                list(&e["maximal"]).join(", ")
            );
        }
    } else {
        println!("  convex pieces: not computed");
    }
    let f = &row["fork_scratch"];
    if f.is_object() {
        println!(
            "  fork and scratch: {} nodes, status differs on {}, holes the same {}, {} failed to load, {:.1} s",
            f["nodes"],
            f["status_differs"],
            f["holes_same"],
            f["failed"],
            f["seconds"].as_f64().unwrap_or(0.0)
        );
    }
    let d = &row["drafts"];
    if d.is_object() {
        for (k, title) in [
            ("today", "today (printed by `ein solve`)"),
            ("a", "(a) R, its lines and its holes named — drafted"),
            ("b", "(b) R as convex pieces — drafted"),
            ("c", "(c) a qualifier — drafted"),
            ("d_stderr", "(d) a refusal, on stderr — drafted"),
        ] {
            let rows = list(&d[k]);
            println!("  {title}: {}", plural(rows.len(), "line", "lines"));
            for r in rows {
                println!("    | {r}");
            }
        }
    }
}

fn main() {
    let mut as_json = false;
    let mut with_drafts = false;
    let mut scratch = true;
    let mut use_written = false;
    let mut limit = region::DEFAULT_LIMIT;
    let mut jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(16));
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--json" => as_json = true,
            "--drafts" => with_drafts = true,
            "--no-scratch" => scratch = false,
            "--written" => use_written = true,
            "--limit" => limit = args.next().and_then(|v| v.parse().ok()).expect("--limit N"),
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            _ => {
                let p = PathBuf::from(&a);
                let p = if p.exists() {
                    p
                } else {
                    ein_corpus::repo_root().join(p)
                };
                files.push(std::fs::canonicalize(&p).unwrap_or(p));
            }
        }
    }
    let mut sources: Vec<Source> = Vec::new();
    if use_written {
        sources.extend(written::WRITTEN.iter().map(|(name, text)| Source {
            name: (*name).to_string(),
            text: (*text).to_string(),
            dir: None,
        }));
    }
    if files.is_empty() && !use_written {
        files = census::selection();
    }
    sources.extend(files.iter().filter_map(|p| Source::of(p)));

    let t_all = Instant::now();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows: Vec<Json> = Vec::new();
    for src in &sources {
        let row = row(src, limit, jobs, scratch, with_drafts);
        if let Some(why) = row["skipped"].as_str() {
            *skipped.entry(why.to_string()).or_default() += 1;
            if as_json {
                println!("{row}");
            }
            continue;
        }
        if as_json {
            println!("{row}");
        } else {
            print_block(&row);
        }
        rows.push(row);
    }
    if as_json {
        return;
    }

    // The census.
    let mut by: BTreeMap<String, (usize, BTreeSet<String>)> = BTreeMap::new();
    let (mut holes, mut dead, mut owing, mut minimal, mut unattributed, mut differ) =
        (0, 0, 0, 0, 0, 0);
    let mut with_holes = 0;
    for row in &rows {
        let h = &row["holes"];
        let total = h["total"].as_u64().unwrap_or(0) as usize;
        holes += total;
        if total > 0 {
            with_holes += 1;
        }
        dead += h["dead"].as_u64().unwrap_or(0) as usize;
        owing += h["owing"].as_u64().unwrap_or(0) as usize;
        unattributed += h["unattributed"].as_u64().unwrap_or(0) as usize;
        minimal += row["minimal_holes"].as_array().map_or(0, Vec::len);
        differ += row["fork_scratch"]["status_differs"].as_u64().unwrap_or(0) as usize;
        for (k, c) in h["by_cause"].as_object().into_iter().flatten() {
            let e = by.entry(k.clone()).or_default();
            e.0 += c.as_u64().unwrap_or(0) as usize;
            e.1.insert(row["path"].as_str().unwrap_or("").to_string());
        }
    }
    let skipped: Vec<String> = skipped.iter().map(|(k, c)| format!("{k} {c}")).collect();
    println!(
        "\n{} flagged cubes within --region-limit {limit} ({} with a hole); skipped: {}; {:.1} s",
        rows.len(),
        with_holes,
        skipped.join(", "),
        t_all.elapsed().as_secs_f64()
    );
    println!("holes {holes}: dead {dead}, owing {owing}; minimal {minimal}");
    for c in Cause::ALL {
        for (k, (n, progs)) in &by {
            if k.ends_with(c.key()) && !k.contains(" + ") {
                println!(
                    "  {:<6} {:<55} {:>4} holes on {}",
                    c.kind(),
                    c.words(),
                    n,
                    progs.iter().cloned().collect::<Vec<_>>().join(", ")
                );
            }
        }
    }
    for (k, (n, progs)) in &by {
        if k.contains(" + ") || k.ends_with("no site") {
            println!(
                "  {k:<62} {n:>4} holes on {}",
                progs.iter().cloned().collect::<Vec<_>>().join(", ")
            );
        }
    }
    println!(
        "a hole no site is named for: {unattributed}; a node whose status differs by fork and scratch: {differ}"
    );
}
