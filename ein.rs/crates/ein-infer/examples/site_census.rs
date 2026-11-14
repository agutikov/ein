//! The site census — M1e2
//! [S1e2.1b.1](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.1_the_site_census.md):
//! where the static check's sites are, per site rather than per program, and
//! what the cube observes about each flagged program.
//!
//! For every file it reads root's fixpoint as `refutation_census` does
//! (`shared/census.rs`, one loader for both), calls [`monotone::classify`]
//! over root's plans, and calls [`region::compute`] on a second load of the
//! same program, asserting the two verdicts are one. Per **site**: the rule;
//! where it is written (`file:line`, a stdlib module or the program); its
//! [`SiteKind`], relation and `absent` depth; the activator; what the rule
//! concludes; whether it fires at root's fixpoint (an obligation: whether
//! root owes it); and its verdict at **fact grain**, the prototype in
//! `shared/fact_grain.rs`, which no route reads. Per **flagged program within
//! the limit**: the cube's three violations, each with its smallest witness,
//! every minimal hole and every witness attributed to a cause and the sites
//! that made it (`shared/attribution.rs`, which `hole_census` reads too: a dead
//! node by its refutation, an owing one by its debt, since S1e2.1b.3 including
//! a witness an `absent` took away and an obligation a node compiled), and
//! *exercised* or *conservative*.
//! Past the limit: what route 2 certifies today (`explore::run`, as a bare
//! `ein solve` asks it), and for `n` = 17–19 the oracle's projected cost.
//!
//! ```sh
//! cargo run --release -p ein-infer --example site_census -- [--json] [--jobs J] [FILE…]
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()` plus the register's witnesses,
//! `refutation_census`'s selection. A `FILE` is read from the working
//! directory, else from the repository root. `--json` prints one line per file, every
//! file of the selection, so each of the older counts can be re-derived from
//! the rows under its own selection; without it, one block per flagged
//! program and the census. It writes nothing and changes no answer.

// `refutation_census` reads the rest of it.
#[allow(dead_code)]
#[path = "shared/attribution.rs"]
mod attribution;
#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;
#[path = "shared/fact_grain.rs"]
mod fact_grain;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Instant;

use attribution::{Attributed, RootPlans};
use census::{Root, located_imported, module_of};
use ein_core::{FactId, Kb, Symbol, Terms};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::monotone::{self, Site, SiteKind};
use ein_infer::region::{self, Outcome, Region};
use ein_ir::Ast;
use serde_json::{Value as Json, json};

/// What a site's rule concludes.
fn concludes(root: &Root, kb: &Kb, terms: &Terms, site: &Site) -> String {
    if kb.program().obligations.contains(site.rule) {
        return "(open …)".to_string();
    }
    let plans: Vec<_> = root
        .plans
        .iter()
        .filter(|p| p.rule == site.rule)
        .filter(|p| site.kind == SiteKind::Activator || p.activator_args == site.activator)
        .collect();
    let Some(plan) = plans.first() else {
        return "— (no plan at root)".to_string();
    };
    if let Some(r) = census::refutes(plan, terms) {
        return r.to_string();
    }
    let mut rels: Vec<String> = monotone::writes_of(plan, terms)
        .into_iter()
        .map(|r| r.label(terms))
        .collect();
    rels.sort();
    rels.dedup();
    format!("positive: {}", rels.join(", "))
}

/// How often the site's rule fired in root's saturation — an obligation: how
/// many instances root owes.
fn at_root(root: &Root, kb: &Kb, site: &Site) -> usize {
    if kb.program().obligations.contains(site.rule) {
        return root
            .owes
            .instances()
            .iter()
            .filter(|o| o.rule == site.rule)
            .filter(|o| site.kind == SiteKind::Activator || o.activator == site.activator)
            .count();
    }
    root.firings
        .iter()
        .filter(|f| f.rule == site.rule && !f.redundant)
        .filter(|f| site.kind == SiteKind::Activator || f.activator == site.activator)
        .count()
}

/// The cube's half of a row: violations, holes, and each minimal hole and
/// each violation's smallest witness attributed to a cause and its sites, by
/// `shared/attribution.rs`'s rule (S1e2.1b.3), which `hole_census` reads too.
fn cube_row(r: &Region, kb_r: &mut Kb, terms_r: &mut Terms, ast_r: &Ast, root: &RootPlans) -> Json {
    let t = &r.topology;
    let sites = attribution::site_texts(&r.monotonicity.sites, terms_r);
    let mut nodes = region::Nodes::new(kb_r, terms_r, ast_r, &r.candidates).expect("prepares");
    let mut items: Vec<(&str, u64, Option<u64>)> = t
        .minimal_holes
        .iter()
        .map(|&h| ("minimal hole", h, None))
        .collect();
    if let Some((a, b)) = t.dead_not_upward_closed.witness {
        items.push(("dead, with an alive superset", a, Some(b)));
    }
    if let Some((a, b)) = t.discharged_not_upward_closed.witness {
        items.push(("discharged, with an owing superset", a, Some(b)));
    }
    let mut attributed: Vec<Json> = Vec::new();
    let mut unattributed = 0usize;
    for (what, mask, up) in items {
        let status = r.status[mask as usize];
        let mut why = serde_json::Map::new();
        let mut a = Attributed::default();
        if let (Some(b), true) = (up, status & region::DEAD == 0) {
            // A discharged node: the debt is the owing superset's, and new
            // against this node.
            a.merge(attribution::owing(
                &mut nodes,
                ast_r,
                &sites,
                root,
                &attribution::facts_of(r, b),
                &attribution::facts_of(r, mask),
            ));
            why.insert("owed".into(), json!(r.nodes[b as usize].owed));
        } else {
            if status & region::DEAD != 0 {
                let node = &r.nodes[mask as usize];
                a.merge(attribution::dead(&sites, root, node));
                why.insert("absent".into(), json!(node.absent_premises));
                why.insert("refuted_by".into(), json!(node.refuted_by));
            }
            if status & region::OWING != 0 {
                let below = attribution::r_below(r, mask)
                    .map(|b| attribution::facts_of(r, b))
                    .unwrap_or_default();
                a.merge(attribution::owing(
                    &mut nodes,
                    ast_r,
                    &sites,
                    root,
                    &attribution::facts_of(r, mask),
                    &below,
                ));
                why.insert("owed".into(), json!(r.nodes[mask as usize].owed));
            }
        }
        if !a.named() {
            unattributed += 1;
        }
        attributed.push(json!({
            "what": what,
            "node": r.name(mask),
            "superset": up.map(|b| r.name(b)),
            "kind": if status & region::DEAD != 0 { "dead" } else if status & region::OWING != 0 { "owing" } else { "R" },
            "why": why,
            "causes": a.causes.iter().map(|c| c.key()).collect::<Vec<_>>(),
            "sites": a.sites,
        }));
    }
    let violated = !t.holes.is_empty()
        || t.dead_not_upward_closed.count > 0
        || t.discharged_not_upward_closed.count > 0;
    json!({
        "size_r": t.size_r,
        "holes": t.holes.len(),
        "minimal_holes": t.minimal_holes.len(),
        "dead_not_upward_closed": t.dead_not_upward_closed.count,
        "discharged_not_upward_closed": t.discharged_not_upward_closed.count,
        "violated": violated,
        "attributed": attributed,
        "unattributed": unattributed,
    })
}

/// What route 2 certifies today on a flagged program past the limit — a bare
/// `ein solve`'s question, `explore::run` over a fresh load.
fn past_row(path: &Path, n: usize, jobs: usize) -> Json {
    let Some((mut kb, mut terms, ast)) = census::loaded(path) else {
        return Json::Null;
    };
    let mut opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    opts.jobs = jobs;
    let t0 = Instant::now();
    let Ok(a) = explore::run(&mut kb, &mut terms, &ast, &opts) else {
        return json!({"error": "explore failed"});
    };
    let wall = t0.elapsed().as_secs_f64();
    // Layers 0 … L whole, in layer order: Σ C(n, k) for k ≤ L within what
    // it saturated.
    let mut whole: Option<usize> = None;
    let mut sum: u128 = 0;
    let mut c: u128 = 1;
    for k in 0..=n {
        if k > 0 {
            c = c * (n - k + 1) as u128 / k as u128;
        }
        sum += c;
        if sum <= a.saturations as u128 {
            whole = Some(k);
        } else {
            break;
        }
    }
    let d = a.described.as_deref();
    let projection = if (region::WARN_FROM..=19).contains(&n) {
        projection(path, n)
    } else {
        None
    };
    json!({
        "route": match a.route { explore::Route::DownSet => 1, explore::Route::Cube => 2, explore::Route::States => 3, explore::Route::Tree => 4, explore::Route::Root => 5 },
        "licences": a.licences.iter().map(|l| l.label()).collect::<Vec<_>>(),
        "exhausted": a.exhausted,
        "stopped": a.stopped.as_ref().map(|s| format!("{s:?}")),
        "saturations": a.saturations,
        "layers_whole": whole,
        "k_c": a.counts.map(|c| c.k_c),
        "k_s": a.counts.map(|c| c.k_s),
        "minimal_certified": d.map(|d| d.minimal_commitments.len()),
        "empty_in_r": d.map(|d| d.root.alive && d.root.owes == 0),
        "root_dead": a.root_dead,
        "nogoods_met": d.map(|d| d.nogoods),
        "wall_s": wall,
        "projection": projection.map(|p| json!({
            "n": p.n, "nodes": p.nodes,
            "per_node_us": p.per_node.as_secs_f64() * 1e6,
            "projected_s": p.projected.as_secs_f64(),
        })),
    })
}

/// The oracle's own projection at `--region-limit n`, and nothing run past
/// its probe: the deadline is now, so the cube stops before its first batch.
fn projection(path: &Path, n: usize) -> Option<region::Projection> {
    let (mut kb, mut terms, ast) = census::loaded(path)?;
    let opts = region::Options {
        limit: n as u32,
        deadline: Some(Instant::now()),
        ..region::Options::default()
    };
    let mut got = None;
    let _ = region::compute(&mut kb, &mut terms, &ast, &opts, &mut |p| got = Some(*p));
    got
}

/// What C1 could do to the flag, at relation grain and with the check as it
/// ships: the verdict over `H` without the candidates that name a type
/// (Q-M1e.12's reading (c), with S1e2.2.10's definition: a type is an object
/// on the right of an `is-a`), and each relation of `H` whose removal alone
/// leaves no site — what the blind rung no longer proposing it would do —
/// with how many of its facts root holds and whether a rule derives it, so
/// a given-only relation can be told from a derived one.
fn c1_row(root: &Root, kb: &Kb, terms: &Terms) -> Json {
    let is_a = terms.syms.get("is-a");
    let types: rustc_hash::FxHashSet<Symbol> = match is_a {
        Some(isa) => kb
            .facts()
            .filter(|&f| terms.facts.get(f).0 == isa)
            .filter_map(|f| terms.facts.get(f).1.get(1).and_then(|v| v.as_sym()))
            .collect(),
        None => rustc_hash::FxHashSet::default(),
    };
    let classify = |h: &rustc_hash::FxHashSet<Symbol>| {
        monotone::classify(kb, terms, &root.plans, h).is_monotone()
    };
    let typed: rustc_hash::FxHashSet<Symbol> = root
        .proposed
        .iter()
        .filter(|&&f| {
            !terms
                .facts
                .get(f)
                .1
                .iter()
                .any(|v| v.as_sym().is_some_and(|s| types.contains(&s)))
        })
        .map(|&f| terms.facts.get(f).0)
        .collect();
    let dropped_by_types = root
        .proposed
        .iter()
        .filter(|&&f| {
            terms
                .facts
                .get(f)
                .1
                .iter()
                .any(|v| v.as_sym().is_some_and(|s| types.contains(&s)))
        })
        .count();
    let h = &root.monotonicity.hypothesised;
    let mut alone: Vec<Json> = Vec::new();
    for &rel in h {
        let rest: rustc_hash::FxHashSet<Symbol> = h.iter().copied().filter(|&x| x != rel).collect();
        if classify(&rest) {
            let given = kb.facts().filter(|&f| terms.facts.get(f).0 == rel).count();
            let derived = root
                .plans
                .iter()
                .filter(|p| kb.program().rules.contains(p.rule))
                .any(|p| {
                    monotone::writes_of(p, terms)
                        .iter()
                        .any(|w| !w.negated && w.name == rel)
                });
            alone.push(json!({
                "relation": terms.sym(rel),
                "candidates": root.proposed.iter().filter(|&&f| terms.facts.get(f).0 == rel).count(),
                "facts_at_root": given,
                "a_rule_derives_it": derived,
            }));
        }
    }
    // determinism-ok: rendered and sorted on the next line.
    let mut type_names: Vec<&str> = types.iter().map(|&s| terms.sym(s)).collect();
    type_names.sort();
    json!({
        "types": type_names,
        "types_excluded": {"dropped": dropped_by_types, "monotone": classify(&typed)},
        "unflagged_by_dropping": alone,
    })
}

#[derive(Default)]
struct Totals {
    given: usize,
    loads: usize,
    saturates: usize,
    flagged: usize,
    within: Vec<(String, usize, bool)>,
    past: Vec<(String, usize)>,
    sites_by_kind: BTreeMap<String, usize>,
    sites_by_module: BTreeMap<String, usize>,
    rules: BTreeMap<(String, String, String), BTreeSet<String>>,
    fact_leaves: Vec<String>,
    fact_incomplete: Vec<String>,
    fact_unsound: Vec<String>,
    c1_types: Vec<String>,
    c1_drop: Vec<String>,
    unattributed: Vec<String>,
    by_cause: BTreeMap<String, usize>,
    monotone_violated: Vec<String>,
    // The four older selections.
    c1015: [usize; 4],
    c1016: [usize; 3],
    route1: [usize; 3],
}

fn main() {
    let mut as_json = false;
    let mut jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(8));
    let mut files: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--json" => as_json = true,
            "--jobs" => jobs = args.next().and_then(|v| v.parse().ok()).expect("--jobs J"),
            // A path from the working directory, else from the repository —
            // so `plans/…` works from `ein.rs/` as from the root.
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
    let swept_selection = files.is_empty();
    if swept_selection {
        files = census::selection();
    }
    let corpus: BTreeSet<PathBuf> = ein_corpus::corpus_files().into_iter().collect();
    let register: BTreeSet<PathBuf> = census::register_witnesses().into_iter().collect();

    let t_all = Instant::now();
    let mut tot = Totals {
        given: files.len(),
        ..Totals::default()
    };
    for path in files {
        let rel = census::relative(&path);
        let mut row = json!({
            "path": rel,
            "corpus": corpus.contains(&path),
            "register": register.contains(&path),
            "loads": false,
            "saturates": false,
        });
        let Some((mut kb, mut terms, ast)) = census::loaded(&path) else {
            if as_json {
                println!("{row}");
            }
            continue;
        };
        tot.loads += 1;
        row["loads"] = json!(true);
        let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
            if as_json {
                println!("{row}");
            }
            continue;
        };
        tot.saturates += 1;
        row["saturates"] = json!(true);
        let rung = root.rung.as_str();
        let mut cands: Vec<FactId> = root.proposed.clone();
        cands.sort_by_key(|&f| ein_infer::events::sexpr(&terms, f));
        cands.dedup();
        let n = cands.len();
        let m = &root.monotonicity;
        let fg = fact_grain::classify(&kb, &terms, &root.plans, &cands, m);
        let h: Vec<&str> = m.hypothesised.iter().map(|&s| terms.sym(s)).collect();
        row["n"] = json!(n);
        row["rung"] = json!(rung);
        row["H"] = json!(h);
        row["monotone"] = json!(m.is_monotone());
        row["fact_grain"] = json!({
            "monotone": fg.is_monotone(),
            "complete": fg.complete,
            "shapes": fg.shapes.as_ref().map(Vec::len),
            "gave_up": fg.gave_up,
        });

        // Root's plans, by text: what a node compiles is what is not here.
        let root_plans = RootPlans::of(&root.plans, &terms);
        // The cube, on a second load: the oracle's own preparation.
        let mut cube = Json::Null;
        let mut computed = false;
        if let Some((mut kb_r, mut terms_r, ast_r)) = census::loaded(&path) {
            let opts = region::Options {
                jobs,
                ..region::Options::default()
            };
            match region::compute(&mut kb_r, &mut terms_r, &ast_r, &opts, &mut |_| {}) {
                Ok(Outcome::Region(r)) => {
                    computed = true;
                    let mine: Vec<String> = m.sites.iter().map(|s| s.render(&terms)).collect();
                    let its: Vec<String> = r
                        .monotonicity
                        .sites
                        .iter()
                        .map(|s| s.render(&terms_r))
                        .collect();
                    assert_eq!(mine, its, "{rel}: two loads, two verdicts");
                    assert_eq!(n, r.n(), "{rel}: two loads, two universes");
                    cube = cube_row(&r, &mut kb_r, &mut terms_r, &ast_r, &root_plans);
                }
                Ok(Outcome::Refused { .. }) => computed = true,
                Err(_) => {}
            }
        }
        row["region_computed"] = json!(computed);
        let violated = cube["violated"].as_bool().unwrap_or(false);
        row["cube"] = cube.clone();

        // The four older selections' counts.
        if computed && corpus.contains(&path) {
            tot.c1015[0] += 1;
            if n <= region::DEFAULT_LIMIT as usize {
                tot.c1015[1] += 1;
            }
            if !m.is_monotone() {
                tot.c1015[2] += 1;
            }
            if cube["holes"].as_u64().unwrap_or(0) > 0 {
                tot.c1015[3] += 1;
            }
        }
        if computed && n <= region::DEFAULT_LIMIT as usize {
            tot.c1016[0] += 1;
            if !m.is_monotone() {
                tot.c1016[1] += 1;
                if violated {
                    tot.c1016[2] += 1;
                }
            }
        }
        if computed {
            tot.route1[0] += 1;
            if !m.is_monotone() {
                if n <= region::DEFAULT_LIMIT as usize {
                    tot.route1[1] += 1;
                } else {
                    tot.route1[2] += 1;
                }
            }
        }

        if m.is_monotone() {
            if violated {
                tot.monotone_violated.push(rel.clone());
            }
            if as_json {
                println!("{row}");
            }
            continue;
        }
        tot.flagged += 1;

        // Per site.
        let mut sites_json = Vec::new();
        for (site, fact) in m.sites.iter().zip(&fg.sites) {
            let at = located_imported(&path, terms.sym(site.rule));
            let module = module_of(&at);
            *tot.sites_by_kind
                .entry(site.kind.as_str().to_string())
                .or_default() += 1;
            *tot.sites_by_module.entry(module.clone()).or_default() += 1;
            tot.rules
                .entry((
                    site.kind.as_str().to_string(),
                    terms.sym(site.rule).to_string(),
                    at.clone(),
                ))
                .or_default()
                .insert(rel.clone());
            let act: Vec<String> = site
                .activator
                .iter()
                .map(|&a| match a.as_sym() {
                    Some(s) => terms.sym(s).to_string(),
                    None => terms.display(a),
                })
                .collect();
            sites_json.push(json!({
                "rule": terms.sym(site.rule),
                "activator": act,
                "kind": site.kind.as_str(),
                "relation": site.relation.label(&terms),
                "depth": site.depth,
                "at": at,
                "module": module,
                "concludes": concludes(&root, &kb, &terms, site),
                "at_root": at_root(&root, &kb, site),
                "fact_grain": fact,
                "render": site.render(&terms),
            }));
        }
        row["sites"] = json!(sites_json);
        row["c1"] = c1_row(&root, &kb, &terms);
        if row["c1"]["types_excluded"]["monotone"].as_bool() == Some(true) {
            tot.c1_types.push(rel.clone());
        }
        for j in row["c1"]["unflagged_by_dropping"]
            .as_array()
            .into_iter()
            .flatten()
        {
            tot.c1_drop.push(format!(
                "{rel}: {} ({} facts at root{})",
                j["relation"].as_str().unwrap_or(""),
                j["facts_at_root"],
                if j["a_rule_derives_it"].as_bool() == Some(true) {
                    ", derived"
                } else {
                    ""
                }
            ));
        }
        if fg.is_monotone() {
            tot.fact_leaves.push(rel.clone());
            if violated {
                tot.fact_unsound.push(rel.clone());
            }
        }
        if !fg.complete {
            tot.fact_incomplete.push(rel.clone());
        }
        if n <= region::DEFAULT_LIMIT as usize && cube.is_object() {
            tot.within.push((rel.clone(), n, violated));
            row["verdict"] = json!(if violated {
                "exercised"
            } else {
                "conservative"
            });
            if cube["unattributed"].as_u64().unwrap_or(0) > 0 {
                tot.unattributed.push(rel.clone());
            }
            for a in cube["attributed"].as_array().into_iter().flatten() {
                for c in a["causes"].as_array().into_iter().flatten() {
                    *tot.by_cause
                        .entry(c.as_str().unwrap_or("").to_string())
                        .or_default() += 1;
                }
            }
        } else {
            tot.past.push((rel.clone(), n));
            row["past"] = past_row(&path, n, jobs);
        }

        if as_json {
            println!("{row}");
        } else {
            print_block(&row);
        }
    }
    if as_json {
        return;
    }
    print_totals(&tot, swept_selection, t_all.elapsed().as_secs_f64());
}

fn print_block(row: &Json) {
    let s = |v: &Json| v.as_str().unwrap_or("").to_string();
    println!(
        "{}\n  n {}  rung {}  H {{{}}}  fact grain: {}{}",
        s(&row["path"]),
        row["n"],
        s(&row["rung"]),
        row["H"]
            .as_array()
            .map(|a| a.iter().map(s).collect::<Vec<_>>().join(" "))
            .unwrap_or_default(),
        if row["fact_grain"]["monotone"].as_bool() == Some(true) {
            "monotone"
        } else {
            "flagged"
        },
        if row["fact_grain"]["gave_up"].as_bool() == Some(true) {
            " (the closure gave up: every site is dynamic)"
        } else if row["fact_grain"]["complete"].as_bool() == Some(false) {
            " (incomplete: an activator is dynamic)"
        } else {
            ""
        },
    );
    for site in row["sites"].as_array().into_iter().flatten() {
        let act = site["activator"]
            .as_array()
            .map(|a| a.iter().map(s).collect::<Vec<_>>().join(" "))
            .unwrap_or_default();
        println!(
            "  site {:<18} {}[{}] reads {} @{} → {}  at root {}  ({}, {})  fact grain: {}",
            s(&site["kind"]),
            s(&site["rule"]),
            act,
            s(&site["relation"]),
            site["depth"],
            s(&site["concludes"]),
            site["at_root"],
            s(&site["at"]),
            s(&site["module"]),
            match site["fact_grain"].as_str() {
                Some(w) => format!("dynamic, e.g. {w}"),
                None => "static".to_string(),
            }
        );
    }
    let c1 = &row["c1"];
    let alone: Vec<String> = c1["unflagged_by_dropping"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|j| {
            format!(
                "{} ({} candidates, {} facts at root{})",
                s(&j["relation"]),
                j["candidates"],
                j["facts_at_root"],
                if j["a_rule_derives_it"].as_bool() == Some(true) {
                    ", derived"
                } else {
                    ""
                }
            )
        })
        .collect();
    println!(
        "  C1: types excluded drops {} candidates → {}; dropping one relation from H unflags it: {}",
        c1["types_excluded"]["dropped"],
        if c1["types_excluded"]["monotone"].as_bool() == Some(true) {
            "monotone"
        } else {
            "still flagged"
        },
        if alone.is_empty() {
            "none".to_string()
        } else {
            alone.join("; ")
        }
    );
    let cube = &row["cube"];
    if cube.is_object() {
        println!(
            "  cube  |R| {}  holes {} (minimal {})  dead↑ {}  discharged↑ {}  — {}",
            cube["size_r"],
            cube["holes"],
            cube["minimal_holes"],
            cube["dead_not_upward_closed"],
            cube["discharged_not_upward_closed"],
            s(&row["verdict"])
        );
        for a in cube["attributed"].as_array().into_iter().flatten() {
            let sup = match a["superset"].as_str() {
                Some(b) => format!(" ⊂ {b}"),
                None => String::new(),
            };
            let sites: Vec<String> = a["sites"].as_array().into_iter().flatten().map(s).collect();
            let causes: Vec<String> = a["causes"]
                .as_array()
                .into_iter()
                .flatten()
                .map(s)
                .collect();
            println!(
                "    {} {}{} ({}) — {}  ← {}",
                s(&a["what"]),
                s(&a["node"]),
                sup,
                s(&a["kind"]),
                a["why"],
                if sites.is_empty() {
                    "no site".to_string()
                } else {
                    format!("{}: {}", causes.join(" + "), sites.join("; "))
                }
            );
        }
    }
    let past = &row["past"];
    if past.is_object() {
        let exhausted = past["exhausted"].as_bool() == Some(true);
        let (whole, bound) = if exhausted {
            ("every node decided".to_string(), "=")
        } else {
            (format!("layers 0…{} whole", past["layers_whole"]), "≥")
        };
        println!(
            "  past the limit: route {} ({}), exhausted {}, {} saturations, {whole}, k_c {bound} {}, k_s {bound} {}, minimal certified {}, ∅ in R {}, {:.1} s{}",
            past["route"],
            past["licences"]
                .as_array()
                .map(|a| a.iter().map(s).collect::<Vec<_>>().join(", "))
                .unwrap_or_default(),
            past["exhausted"],
            past["saturations"],
            past["k_c"],
            past["k_s"],
            past["minimal_certified"],
            past["empty_in_r"],
            past["wall_s"].as_f64().unwrap_or(0.0),
            match past["projection"].as_object() {
                Some(p) => format!(
                    "; the oracle at --region-limit {}: {} nodes × {:.0} µs ≈ {:.0} s at one job",
                    p["n"],
                    p["nodes"],
                    p["per_node_us"].as_f64().unwrap_or(0.0),
                    p["projected_s"].as_f64().unwrap_or(0.0)
                ),
                None => String::new(),
            }
        );
    }
}

fn print_totals(tot: &Totals, swept_selection: bool, wall: f64) {
    println!(
        "\nswept {} files: {} load, {} load and saturate; flagged non-monotone {} ({} within --region-limit 16, {} past it); {:.1} s",
        tot.given,
        tot.loads,
        tot.saturates,
        tot.flagged,
        tot.within.len(),
        tot.past.len(),
        wall
    );
    let total: usize = tot.sites_by_kind.values().sum();
    let kinds: Vec<String> = tot
        .sites_by_kind
        .iter()
        .map(|(k, c)| format!("{k} {c}"))
        .collect();
    let modules: Vec<String> = tot
        .sites_by_module
        .iter()
        .map(|(k, c)| format!("{k} {c}"))
        .collect();
    println!("sites {total}: {}", kinds.join(", "));
    println!("  written in: {}", modules.join(", "));
    println!("  distinct (kind, rule, where): {}", tot.rules.len());
    for ((k, r, at), e) in &tot.rules {
        let e: Vec<&str> = e.iter().map(String::as_str).collect();
        println!("    {k:<18} {r} ({at}) — {}", e.join(", "));
    }
    let (ex, cons): (Vec<_>, Vec<_>) = tot.within.iter().partition(|(_, _, v)| *v);
    println!(
        "flagged within the limit {}: exercised {}, conservative {}",
        tot.within.len(),
        ex.len(),
        cons.len()
    );
    for (p, n, _) in &ex {
        println!("    exercised     n={n:<3} {p}");
    }
    for (p, n, _) in &cons {
        println!("    conservative  n={n:<3} {p}");
    }
    println!("flagged past the limit {}:", tot.past.len());
    for (p, n) in &tot.past {
        println!("    n={n:<3} {p}");
    }
    println!(
        "fact grain: {} of {} flagged leave the set; incomplete on {}; a fact-grain monotone verdict beside a violation on {}",
        tot.fact_leaves.len(),
        tot.flagged,
        tot.fact_incomplete.len(),
        tot.fact_unsound.len()
    );
    for p in &tot.fact_leaves {
        println!("    leaves        {p}");
    }
    for p in &tot.fact_incomplete {
        println!("    incomplete    {p}");
    }
    for p in &tot.fact_unsound {
        println!("    UNSOUND       {p}");
    }
    println!(
        "C1 at relation grain: the universe without a candidate naming a type unflags {}; one relation of H removed unflags {}:",
        tot.c1_types.len(),
        tot.c1_drop.len()
    );
    for p in &tot.c1_types {
        println!("    types         {p}");
    }
    for p in &tot.c1_drop {
        println!("    without       {p}");
    }
    let by_cause: Vec<String> = tot
        .by_cause
        .iter()
        .map(|(k, c)| format!("{k} {c}"))
        .collect();
    println!(
        "minimal holes and violation witnesses attributed, by cause: {}",
        by_cause.join(", ")
    );
    println!(
        "a violation no site is named for: {}; a monotone cube with a violation: {}",
        tot.unattributed.len(),
        tot.monotone_violated.len()
    );
    for p in tot.unattributed.iter().chain(&tot.monotone_violated) {
        println!("    {p}");
    }
    if swept_selection {
        println!("the older counts, each under its own selection:");
        println!(
            "  S1e2.1a.1 census_for_inspection (corpus_files): {} compute a region or refuse; {} at n ≤ 16; {} flagged; {} with a hole",
            tot.c1015[0], tot.c1015[1], tot.c1015[2], tot.c1015[3]
        );
        println!(
            "  region_census --all (corpus + register): {} regions at n ≤ 16; {} flagged; {} with a violation",
            tot.c1016[0], tot.c1016[1], tot.c1016[2]
        );
        println!(
            "  route1_reach (corpus + register, load and compile): {} programs; {} flagged at n ≤ 16; {} flagged past it",
            tot.route1[0], tot.route1[1], tot.route1[2]
        );
        println!(
            "  refutation_census (corpus + register): {} files; {} load and saturate; {} flagged",
            tot.given, tot.saturates, tot.flagged
        );
    }
}
