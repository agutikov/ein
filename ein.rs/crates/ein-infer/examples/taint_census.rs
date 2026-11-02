//! The taint census — M1e2
//! [S1e2.1b.4](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.4_refutation_resting_on_absent.md):
//! every dead node of every flagged cube the oracle reaches, with the `absent`
//! premises its death rests on, for F21's D6 and S1e2.2.5's F1–F3.
//!
//! A node's **taint** is `shared/taint.rs`'s: every contradiction of its
//! fixpoint, both sides of a pair, followed through every fact's primary
//! derivation, and with the activator of each plan a fact was derived by. A
//! premise is **static** (its relation outside `D`) or in `D`, **direct** (read
//! by the rule that concluded the contradiction) or **via** (recorded further
//! down), on a fact **root** holds or not, and reached **through an
//! activator** or not. Per program it prints:
//!
//! - each dead node: its premises, tainted or not under each reading (`D`;
//!   *eligible*, S1e.2.3's; the oracle's own `absent_premises`; the walk
//!   without activators, which is option (b) read off provenance alone), and
//!   whether it has an alive superset, is a hole, and is dead from scratch;
//! - an **untainted death with an alive superset** under each reading: a
//!   counterexample to (b) under that reading;
//! - what (b) buys: the nodes an untainted death lets a route skip, its proper
//!   supersets, against `2ⁿ`;
//! - the refutations a **node compiles**: plans of a rule with parameters whose
//!   activator a commitment derives, which conclude `(false)` or `(not …)` —
//!   and of those, the ones that read an `absent` over `D`, which is what a
//!   *forbid* check at each node's compile would refuse (S1e2.2.5's F3), and
//!   the ones whose activator rests on one.
//!
//! Past the limit it reads the static half: the activator sites, and whether a
//! plan of the rule root compiled concludes a refutation.
//!
//! ```sh
//! cargo run --release -p ein-infer --example taint_census                  # the flagged programs of the selection
//! cargo run --release -p ein-infer --example taint_census -- --written     # the programs in shared/written.rs
//! cargo run --release -p ein-infer --example taint_census -- [--json] FILE…
//! ```
//!
//! With no `FILE` it sweeps `site_census`'s selection, `corpus_files()` plus
//! the register's witnesses. It writes nothing, and no route reads it.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;
#[allow(dead_code)]
#[path = "shared/taint.rs"]
mod taint;
#[allow(dead_code)]
#[path = "shared/written.rs"]
mod written;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, Terms};
use ein_infer::compile::{SharedMemo, activators_for, compile_rule};
use ein_infer::events::{Events, sexpr};
use ein_infer::hypgen::eligible_relations;
use ein_infer::monotone::{self, SiteKind};
use ein_infer::region::{self, DEAD, OWING, Outcome};
use ein_infer::saturator::{Saturator, Session};
use ein_infer::{contradiction, obligations};
use ein_ir::{Ast, load, parse};
use rustc_hash::FxHashSet;
use serde_json::{Value as Json, json};
use taint::{Dynamic, Taint, Walk};

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

/// One node from scratch, to its two bits: `utils/region.py`'s node.
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

/// A premise's relation, bare: `q` for `q` and `(not q)`.
fn base(label: &str) -> String {
    label
        .strip_prefix("(not ")
        .map(|r| r.trim_end_matches(')').to_string())
        .unwrap_or_else(|| label.to_string())
}

/// Tainted under a reading: every contradiction has a premise the reading
/// calls extendable.
fn tainted_by(t: &Taint, extendable: &dyn Fn(&taint::Premise) -> bool) -> bool {
    !t.contradictions.is_empty()
        && t.contradictions
            .iter()
            .all(|c| c.premises.iter().any(extendable))
}

/// The plans `kb`, a node's fixpoint, compiles beyond root's: each rule with
/// parameters and an activator fact root does not hold, compiled. For each,
/// whether it concludes a refutation, whether it reads an `absent` over `D` at
/// odd depth, and whether its activator's derivation rests on one.
fn compiled_at(
    kb: &Kb,
    terms: &mut Terms,
    ast: &Ast,
    root_acts: &FxHashSet<FactId>,
    d: &Dynamic,
    root: &FxHashSet<FactId>,
) -> Vec<Json> {
    let mut out = Vec::new();
    let rules: Vec<_> = kb
        .program()
        .rules
        .values()
        .filter(|r| !r.params.is_empty())
        .cloned()
        .collect();
    for rule in rules {
        for act in activators_for(kb, terms, &rule).into_iter().flatten() {
            if root_acts.contains(&act) {
                continue;
            }
            let Ok(plan) = compile_rule(ast, terms, &rule, Some(act)) else {
                continue;
            };
            let Some(concludes) = census::refutes(&plan, terms) else {
                continue;
            };
            let odd: Vec<String> = monotone::reads_of(&plan, terms)
                .into_iter()
                .filter(|&(r, depth)| depth % 2 == 1 && d.contains(&r.label(terms)))
                .map(|(r, depth)| format!("{}@{depth}", r.label(terms)))
                .collect();
            // The activator's own derivation: the premises a walk from it finds.
            let act_taint = activator_taint(kb, terms, act, d, root);
            out.push(json!({
                "rule": terms.sym(rule.name),
                "activator": sexpr(terms, act),
                "concludes": concludes,
                "reads_absent_over_d": odd,
                "activator_rests_on": act_taint,
            }));
        }
    }
    out
}

/// The `absent` premises over `D` an activator fact's derivation rests on.
fn activator_taint(
    kb: &Kb,
    terms: &Terms,
    act: FactId,
    d: &Dynamic,
    root: &FxHashSet<FactId>,
) -> Vec<String> {
    let mut seen: FxHashSet<FactId> = FxHashSet::default();
    let mut stack = vec![act];
    let mut out: BTreeSet<String> = BTreeSet::new();
    while let Some(f) = stack.pop() {
        if !seen.insert(f) {
            continue;
        }
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        for r in prov.absent.iter() {
            let label = taint::naf_label(terms, r);
            if d.contains(&label) {
                out.insert(format!(
                    "{} ← {}{}",
                    label,
                    prov.rule.map_or("?", |r| terms.sym(r)),
                    if root.contains(&f) { " (at root)" } else { "" }
                ));
            }
        }
        stack.extend(prov.premises.iter().copied());
    }
    out.into_iter().collect()
}

/// The census of one program.
fn row(src: &Source, jobs: usize) -> Json {
    let skip = |why: &str| json!({ "path": src.name, "skipped": why });
    let Some((mut kb, mut terms, ast)) = src.load("") else {
        return skip("does not load");
    };
    let Some((root, eligible)) = census::root_of(&mut kb, &mut terms, &ast, |_, s| {
        let e = eligible_relations(s).ok()?;
        Some(
            e.iter()
                .map(|&r| s.terms.sym(r).to_string())
                .collect::<BTreeSet<String>>(),
        )
    }) else {
        return skip("does not saturate");
    };
    if root.monotonicity.is_monotone() {
        return skip("monotone");
    }
    // The static half: every activator site, and whether a plan of its rule
    // root compiled concludes a refutation; and root's refuting plans that read
    // an `absent` over D, what a check at load would see.
    let d0 = Dynamic::of(root.monotonicity.dynamic.iter().map(|r| r.label(&terms)));
    let activator_sites: Vec<Json> = root
        .monotonicity
        .sites
        .iter()
        .filter(|s| s.kind == SiteKind::Activator)
        .map(|s| {
            let plans: Vec<_> = root.plans.iter().filter(|p| p.rule == s.rule).collect();
            json!({
                "rule": terms.sym(s.rule),
                "plans_at_root": plans.len(),
                "a_root_plan_refutes": plans.iter().any(|p| census::refutes(p, &terms).is_some()),
            })
        })
        .collect();
    let at_load: Vec<String> = root
        .plans
        .iter()
        .filter(|p| kb.program().rules.contains(p.rule))
        .filter(|p| census::refutes(p, &terms).is_some())
        .filter(|p| {
            monotone::reads_of(p, &terms)
                .into_iter()
                .any(|(r, depth)| depth % 2 == 1 && d0.contains(&r.label(&terms)))
        })
        .map(|p| census::named(&terms, p.rule, &p.activator_args))
        .collect();

    let Some((mut kb_r, mut terms_r, ast_r)) = src.load("") else {
        return skip("does not load twice");
    };
    let opts = region::Options {
        jobs,
        ..region::Options::default()
    };
    let r = match region::compute(&mut kb_r, &mut terms_r, &ast_r, &opts, &mut |_| {}) {
        Ok(Outcome::Region(r)) => r,
        Ok(Outcome::Refused { n, .. }) => {
            return json!({ "path": src.name, "n": n, "past_the_limit": true,
                           "activator_sites": activator_sites, "refusable_at_load": at_load });
        }
        Err(e) => return skip(&format!("{e}")),
    };
    let n = r.n();
    let d = Dynamic::of(r.monotonicity.dynamic.iter().map(|x| x.label(&terms_r)));
    // Root's fixpoint and its activators, before `Nodes` borrows the load.
    let root_facts: FxHashSet<FactId> = kb_r.facts().collect();
    let root_acts: FxHashSet<FactId> = kb_r
        .program()
        .rules
        .values()
        .filter(|x| !x.params.is_empty())
        .flat_map(|x| activators_for(&kb_r, &terms_r, x).into_iter().flatten())
        .collect();
    let holes: BTreeSet<u64> = r.topology.holes.iter().copied().collect();
    let cands: Vec<String> = r.candidates.iter().map(|c| c.text.clone()).collect();
    let status = r.status.clone();
    let names: Vec<String> = (0..status.len() as u64).map(|m| r.name(m)).collect();
    let oracle_premises: Vec<Vec<String>> =
        r.nodes.iter().map(|x| x.absent_premises.clone()).collect();
    let refuted_by: Vec<Vec<String>> = r.nodes.iter().map(|x| x.refuted_by.clone()).collect();
    let facts_of = |mask: u64| -> Vec<FactId> {
        (0..n)
            .filter(|b| mask >> b & 1 == 1)
            .map(|b| r.candidates[b].fact)
            .collect()
    };
    let mut nodes =
        region::Nodes::new(&mut kb_r, &mut terms_r, &ast_r, &r.candidates).expect("prepares");

    let mut dead_rows: Vec<Json> = Vec::new();
    let mut compiled: BTreeMap<String, Json> = BTreeMap::new();
    let mut counter = [0usize; 6];
    let mut skippable: BTreeSet<u64> = BTreeSet::new();
    let mut untainted_dead: Vec<u64> = Vec::new();
    for m in 0..status.len() as u64 {
        let dead = status[m as usize] & DEAD != 0;
        let read = nodes
            .with_fixpoint(&facts_of(m), |kb, terms| {
                let plans = compiled_at(kb, terms, &ast_r, &root_acts, &d, &root_facts);
                let t = dead.then(|| {
                    (
                        taint::of(kb, terms, &root_facts, &d, Walk { activators: true }),
                        taint::of(kb, terms, &root_facts, &d, Walk { activators: false }),
                    )
                });
                (plans, t)
            })
            .expect("a node");
        for p in read.0 {
            let key = format!(
                "{} {}",
                p["rule"].as_str().unwrap_or(""),
                p["activator"].as_str().unwrap_or("")
            );
            compiled.entry(key).or_insert_with(|| {
                let mut p = p.clone();
                p["first_at"] = json!(names[m as usize]);
                p
            });
        }
        let Some((full, provenance_only)) = read.1 else {
            continue;
        };
        let alive_above = (0..status.len() as u64)
            .any(|x| x != m && x & m == m && status[x as usize] & DEAD == 0);
        let in_d = |p: &taint::Premise| p.dynamic;
        let eligible_ = |p: &taint::Premise| eligible.contains(&base(&p.label));
        let tainted_d = tainted_by(&full, &in_d);
        let tainted_eligible = tainted_by(&full, &eligible_);
        let tainted_provenance = tainted_by(&provenance_only, &in_d);
        let oracle_tainted = oracle_premises[m as usize].iter().any(|p| {
            let label = p
                .strip_prefix("(absent (")
                .map(|x| {
                    x.split_whitespace()
                        .next()
                        .unwrap_or("")
                        .trim_end_matches(')')
                        .to_string()
                })
                .unwrap_or_default();
            let label = if label == "not" {
                // `(absent (not (q A)))`
                let inner = p.trim_start_matches("(absent (not (");
                format!(
                    "(not {})",
                    inner
                        .split_whitespace()
                        .next()
                        .unwrap_or("?")
                        .trim_end_matches(')')
                )
            } else {
                label
            };
            d.contains(&label)
        });
        let scratch = {
            let text: Vec<&str> = (0..n)
                .filter(|b| m >> b & 1 == 1)
                .map(|b| cands[b].as_str())
                .collect();
            scratch_status(src, &text.join(" "))
        };
        if !tainted_d {
            counter[0] += 1;
            untainted_dead.push(m);
            if alive_above {
                counter[1] += 1;
            }
        } else {
            counter[2] += 1;
            if alive_above {
                counter[3] += 1;
            }
        }
        if !tainted_provenance && alive_above {
            counter[4] += 1;
        }
        if !oracle_tainted && alive_above {
            counter[5] += 1;
        }
        let premises: Vec<Json> = full
            .premises()
            .into_iter()
            .map(|p| {
                json!({
                    "text": p.text,
                    "class": if !p.dynamic { "static" } else if p.direct { "direct" } else { "via" },
                    "at_root": p.at_root,
                    "through_activator": p.through_activator,
                    "eligible": eligible.contains(&base(&p.label)),
                })
            })
            .collect();
        dead_rows.push(json!({
            "node": names[m as usize],
            "refuted_by": refuted_by[m as usize],
            "contradictions": full.contradictions.len(),
            "premises": premises,
            "tainted": { "D": tainted_d, "eligible": tainted_eligible,
                         "provenance_only": tainted_provenance, "oracle_field": oracle_tainted },
            "alive_superset": alive_above,
            "hole": holes.contains(&m),
            "scratch": scratch.map(|s| if s & DEAD != 0 { "dead" } else if s & OWING != 0 { "owing" } else { "R" }),
        }));
    }
    drop(nodes);
    for &u in &untainted_dead {
        for x in 0..status.len() as u64 {
            if x != u && x & u == u {
                skippable.insert(x);
            }
        }
    }
    json!({
        "path": src.name,
        "n": n,
        "nodes": status.len(),
        "dead": dead_rows.len(),
        "untainted": counter[0],
        "untainted_with_alive_superset": counter[1],
        "tainted": counter[2],
        "tainted_with_alive_superset": counter[3],
        "provenance_only_untainted_with_alive_superset": counter[4],
        "oracle_field_untainted_with_alive_superset": counter[5],
        "skippable": skippable.len(),
        "dead_rows": dead_rows,
        "compiled_refutations": compiled.into_values().collect::<Vec<_>>(),
        "activator_sites": activator_sites,
        "refusable_at_load": at_load,
    })
}

fn print_block(row: &Json) {
    let s = |v: &Json| v.as_str().unwrap_or("").to_string();
    if row["past_the_limit"].as_bool() == Some(true) {
        println!(
            "{}  n {} — past the limit; activator sites {}; refusable at load {}",
            s(&row["path"]),
            row["n"],
            row["activator_sites"],
            row["refusable_at_load"]
        );
        return;
    }
    println!(
        "{}\n  n {}  dead {} of {}: untainted {} ({} with an alive superset), tainted {} ({} with an alive superset); skippable by (b) {} of {}",
        s(&row["path"]),
        row["n"],
        row["dead"],
        row["nodes"],
        row["untainted"],
        row["untainted_with_alive_superset"],
        row["tainted"],
        row["tainted_with_alive_superset"],
        row["skippable"],
        row["nodes"],
    );
    println!(
        "  counterexamples to (b): D {}, provenance only {}, the oracle's field {}",
        row["untainted_with_alive_superset"],
        row["provenance_only_untainted_with_alive_superset"],
        row["oracle_field_untainted_with_alive_superset"]
    );
    for d in row["dead_rows"].as_array().into_iter().flatten() {
        let t = &d["tainted"];
        if !(d["alive_superset"].as_bool() == Some(true)
            || t["D"] != t["eligible"]
            || t["D"] != t["oracle_field"]
            || t["D"] != t["provenance_only"]
            || d["scratch"].as_str() != Some("dead"))
        {
            continue;
        }
        let ps: Vec<String> = d["premises"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|p| {
                format!(
                    "{} [{}{}{}{}]",
                    s(&p["text"]),
                    s(&p["class"]),
                    if p["at_root"].as_bool() == Some(true) {
                        ", at root"
                    } else {
                        ""
                    },
                    if p["through_activator"].as_bool() == Some(true) {
                        ", through an activator"
                    } else {
                        ""
                    },
                    if p["eligible"].as_bool() == Some(true) {
                        ", eligible"
                    } else {
                        ""
                    },
                )
            })
            .collect();
        println!(
            "    dead {} — refuted by {}; tainted D {} eligible {} provenance-only {} oracle {}; alive superset {}; hole {}; scratch {}\n      {}",
            s(&d["node"]),
            d["refuted_by"],
            t["D"],
            t["eligible"],
            t["provenance_only"],
            t["oracle_field"],
            d["alive_superset"],
            d["hole"],
            d["scratch"],
            if ps.is_empty() {
                "no `absent` premise".to_string()
            } else {
                ps.join("; ")
            }
        );
    }
    for c in row["compiled_refutations"].as_array().into_iter().flatten() {
        println!(
            "  compiled at {}: {} [{}] → {}; reads an absent over D: {}; its activator rests on: {}",
            s(&c["first_at"]),
            s(&c["rule"]),
            s(&c["activator"]),
            s(&c["concludes"]),
            c["reads_absent_over_d"],
            c["activator_rests_on"]
        );
    }
    if row["activator_sites"]
        .as_array()
        .is_some_and(|a| !a.is_empty())
    {
        println!("  activator sites {}", row["activator_sites"]);
    }
    println!("  refusable at load {}", row["refusable_at_load"]);
}

fn main() {
    let mut as_json = false;
    let mut use_written = false;
    let mut jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(16));
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--json" => as_json = true,
            "--written" => use_written = true,
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

    let mut tot: BTreeMap<&str, usize> = BTreeMap::new();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    let mut programs = 0usize;
    let mut cubes = 0usize;
    let mut differ_eligible: Vec<String> = Vec::new();
    let mut differ_oracle: Vec<String> = Vec::new();
    let mut fork_only: Vec<String> = Vec::new();
    let mut compiled: Vec<String> = Vec::new();
    let mut act_sites: Vec<String> = Vec::new();
    let mut load_refusable = 0usize;
    for src in &sources {
        let row = row(src, jobs);
        if as_json {
            println!("{row}");
        }
        if let Some(why) = row["skipped"].as_str() {
            *skipped.entry(why.to_string()).or_default() += 1;
            continue;
        }
        programs += 1;
        if !as_json {
            print_block(&row);
        }
        let name = row["path"].as_str().unwrap_or("").to_string();
        for a in row["activator_sites"].as_array().into_iter().flatten() {
            act_sites.push(format!("{name}: {a}"));
        }
        if !row["refusable_at_load"]
            .as_array()
            .is_none_or(Vec::is_empty)
        {
            load_refusable += 1;
        }
        if row["past_the_limit"].as_bool() == Some(true) {
            continue;
        }
        cubes += 1;
        for k in [
            "dead",
            "untainted",
            "untainted_with_alive_superset",
            "tainted",
            "tainted_with_alive_superset",
            "provenance_only_untainted_with_alive_superset",
            "oracle_field_untainted_with_alive_superset",
            "skippable",
            "nodes",
        ] {
            *tot.entry(k).or_default() += row[k].as_u64().unwrap_or(0) as usize;
        }
        for d in row["dead_rows"].as_array().into_iter().flatten() {
            let t = &d["tainted"];
            let node = format!("{name} {}", d["node"].as_str().unwrap_or(""));
            if t["D"] != t["eligible"] {
                differ_eligible.push(format!("{node} (D {}, eligible {})", t["D"], t["eligible"]));
            }
            if t["D"] != t["oracle_field"] {
                differ_oracle.push(format!(
                    "{node} (D {}, oracle {})",
                    t["D"], t["oracle_field"]
                ));
            }
            if d["scratch"].as_str() != Some("dead") {
                fork_only.push(format!("{node} (scratch: {})", d["scratch"]));
            }
            if d["hole"].as_bool() == Some(true) && t["D"].as_bool() != Some(true) {
                *tot.entry("untainted_holes").or_default() += 1;
            }
            if d["hole"].as_bool() == Some(true) {
                *tot.entry("dead_holes").or_default() += 1;
            }
            for p in d["premises"].as_array().into_iter().flatten() {
                *tot.entry(match p["class"].as_str() {
                    Some("static") => "premises static",
                    Some("direct") => "premises in D, direct",
                    _ => "premises in D, via",
                })
                .or_default() += 1;
                if p["at_root"].as_bool() == Some(true) {
                    *tot.entry("premises on a root fact").or_default() += 1;
                }
                if p["through_activator"].as_bool() == Some(true) {
                    *tot.entry("premises through an activator").or_default() += 1;
                }
            }
        }
        for c in row["compiled_refutations"].as_array().into_iter().flatten() {
            compiled.push(format!("{name}: {c}"));
        }
    }
    if as_json {
        return;
    }
    let skipped: Vec<String> = skipped.iter().map(|(k, c)| format!("{k} {c}")).collect();
    println!(
        "\n{programs} flagged programs, {cubes} cubes within the limit; skipped: {}",
        skipped.join(", ")
    );
    for (k, v) in &tot {
        println!("  {k:<50} {v}");
    }
    println!(
        "tainted under D and under eligible differ on {}:",
        differ_eligible.len()
    );
    for x in &differ_eligible {
        println!("    {x}");
    }
    println!(
        "tainted under D and by the oracle's own absent_premises differ on {}:",
        differ_oracle.len()
    );
    for x in &differ_oracle {
        println!("    {x}");
    }
    println!("dead by fork and not from scratch: {}", fork_only.len());
    for x in &fork_only {
        println!("    {x}");
    }
    println!("refutations a node compiles: {}", compiled.len());
    for x in &compiled {
        println!("    {x}");
    }
    println!("activator sites: {}", act_sites.len());
    for x in &act_sites {
        println!("    {x}");
    }
    println!(
        "programs with a refuting plan over an `absent` in D at root (a check at load): {load_refusable}"
    );
}
