//! The detector, drafted — M1e2
//! [S1e2.1b.5](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/s1e2.1b.5_the_detector.md)'s
//! T1e2.1b.5.1: the rows its proposal,
//! [`detector_proposal.md`](../../../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md),
//! shows on real programs, computed here rather than written by hand.
//!
//! Per program:
//!
//! - **the explicit check's work**: the load, root's fixpoint, the universe,
//!   root's tally and [`monotone::classify`], through `census::root_of`, which
//!   is `region::prepare` as a reader outside the crate runs it; timed from the
//!   load;
//! - **the static detector** of the stage's (b), on a flagged program only:
//!   the cycles through an `absent` over `D` with their parity
//!   (`node_census`'s graph, copied), and the **order** check, new: an
//!   `absent` rule that reads under an `absent` a relation in `D` which
//!   another `absent` rule concludes. At one priority the written order
//!   decides which the boundary admits first; at an earlier priority the
//!   reader is admitted first in every order;
//! - **a bare describe** (`explore::run`), on a second load: what it lists,
//!   its route and its cost;
//! - **the cube's half**, within the limit: every node through
//!   [`region::Nodes`], its two bits, and [`Topology::of`] over them, which is
//!   what route 2 computes; past the limit, *not known*;
//! - **stale facts** (`node_census` § 4's check, copied): on every node of the
//!   cube within the limit, and on every state the describe lists. The
//!   matcher's time is taken apart from the fixpoint's, because a route
//!   already holds each fixpoint when it reads its bits;
//! - **the parent-path spot check**, priced: for each listed state of two
//!   commitments or more, up to `--bound` parents `C \ {h}` (default 3): the
//!   parent's fixpoint, `h` written, saturated, compared with the direct
//!   fixpoint. A dead parent is skipped, as `sanity::check_commutativity`
//!   skips one.
//!
//! Then the **drafted rows**: the `lattice` block of a bare `ein solve`, and
//! the read-out of `ein solve --monotone`. Their words are the proposal's, and
//! nothing prints them.
//!
//! ```sh
//! cargo run --release -p ein-infer --example detector_draft -- FILE…            # the blocks
//! cargo run --release -p ein-infer --example detector_draft -- --json FILE…     # one row per program
//! cargo run --release -p ein-infer --example detector_draft -- --bound 0 FILE…  # no spot check
//! ```
//!
//! No route reads this file, no command reaches it, and it writes nothing.

#[allow(dead_code)]
#[path = "shared/attribution.rs"]
mod attribution;
#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use ein_core::{FactId, Kb, NafArg, NafRef, Prov, Symbol, Terms, Value};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::{Events, sexpr};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::match_::Matcher;
use ein_infer::monotone::{self, SiteKind};
use ein_infer::plan::{Disjunct, NafGuard, Plan, Slot, Span, Step};
use ein_infer::region::{self, DEAD, OWING, Topology};
use ein_infer::saturator::{Saturator, Session};

/// Every rule's priority since M1f S1f.5.15 removed the keyword: one, so the
/// draft's order check reads every pair as at one priority, as the shipped
/// detector does (`detector::order`).
const DEFAULT_PRIORITY: i64 = 1000;
use ein_infer::{contradiction, obligations};
use ein_ir::Ast;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use serde_json::{Value as Json, json};

const LIMIT: usize = region::DEFAULT_LIMIT as usize;
/// How many stale facts, sites or path differences a stdout row names before
/// it says how many more `--dump-answer` would write.
const ROWS: usize = 4;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut as_json = false;
    let mut bound = 3usize;
    let mut files: Vec<PathBuf> = Vec::new();
    while let Some(a) = args.next() {
        match a.as_str() {
            "--json" => as_json = true,
            "--bound" => bound = args.next().and_then(|v| v.parse().ok()).expect("--bound B"),
            f => files.push(PathBuf::from(f)),
        }
    }
    let t0 = Instant::now();
    for f in &files {
        let path = if f.is_absolute() {
            f.clone()
        } else {
            repo_root().join(f)
        };
        let row = draft(&path, bound);
        if as_json {
            println!("{row}");
        } else {
            print_block(&row);
        }
    }
    if !as_json {
        println!("wall {:.1} s", t0.elapsed().as_secs_f64());
    }
}

fn us(d: Duration) -> f64 {
    d.as_secs_f64() * 1e6
}

// ── One program ─────────────────────────────────────────────────────

fn draft(path: &Path, bound: usize) -> Json {
    let rel = census::relative(path);
    let mut row = json!({ "path": rel });

    // The explicit check: load, root, universe, tally, classify.
    let t = Instant::now();
    let Some((mut kb, mut terms, ast)) = census::loaded(path) else {
        row["loads"] = json!(false);
        return row;
    };
    let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
        row["saturates"] = json!(false);
        return row;
    };
    let check = t.elapsed();
    let mut cands = root.proposed.clone();
    cands.sort_by_key(|&f| sexpr(&terms, f));
    cands.dedup();
    let texts: Vec<String> = cands.iter().map(|&f| sexpr(&terms, f)).collect();
    let n = cands.len();
    let m = &root.monotonicity;
    row["n"] = json!(n);
    row["monotone"] = json!(m.is_monotone());
    row["check_us"] = json!(us(check));

    // The static detector, on a flagged program only.
    let mut sites: Vec<Json> = Vec::new();
    let (mut cycle_rows, mut order_rows) = (Vec::new(), Vec::new());
    let mut detector = Duration::ZERO;
    if !m.is_monotone() {
        for s in &m.sites {
            let mine: Vec<&Arc<Plan>> = root.plans.iter().filter(|p| p.rule == s.rule).collect();
            let concludes = if s.kind == SiteKind::ObligationTrigger {
                "(open …)"
            } else if mine.is_empty() {
                "root compiled no plan of it"
            } else {
                mine.iter()
                    .find_map(|p| census::refutes(p, &terms))
                    .unwrap_or("a positive fact")
            };
            let stake = match s.kind {
                SiteKind::Absent if concludes == "a positive fact" => "the state",
                SiteKind::Absent => "the state, and dead with it",
                SiteKind::ObligationTrigger => "the status: owes",
                SiteKind::Activator if kb.program().obligations.contains(s.rule) => {
                    "the status: owes, through a plan a node compiles"
                }
                SiteKind::Activator => "the state, through a plan a node compiles",
            };
            sites.push(json!({
                "site": s.render(&terms),
                "kind": s.kind.as_str(),
                "at": census::located_imported(path, terms.sym(s.rule)),
                "concludes": concludes,
                "at_stake": stake,
            }));
        }
        let t = Instant::now();
        let dynamic: BTreeSet<String> = m.dynamic.iter().map(|r| r.label(&terms)).collect();
        cycle_rows = cycles_over(&kb, &terms, &root.plans, &dynamic);
        order_rows = order(&kb, &terms, &root.plans, &dynamic);
        detector = t.elapsed();
    }
    row["sites"] = json!(sites);
    row["cycles"] = json!(cycle_rows);
    row["order"] = json!(order_rows);
    row["detector_us"] = json!(us(detector));

    // A bare describe, on its own load.
    let (mut kb2, mut terms2, ast2) = census::loaded(path).expect("loaded once");
    let t = Instant::now();
    let a = explore::run(
        &mut kb2,
        &mut terms2,
        &ast2,
        &explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        }),
    );
    let run_wall = t.elapsed();
    let Ok(a) = a else {
        row["run"] = json!("refused");
        return row;
    };
    row["run"] = json!({
        "route": match a.route { explore::Route::DownSet => "1", explore::Route::Cube => "2", explore::Route::States => "states", explore::Route::Tree => "tree", explore::Route::Root => "root" },
        "exhausted": a.exhausted,
        "listed": a.states.len(),
        "saturations": a.saturations,
        "wall_us": us(run_wall),
    });
    if m.is_monotone() {
        return row;
    }
    // The listed states, as this load's facts.
    let by_text: FxHashMap<&str, FactId> = texts
        .iter()
        .map(|t| t.as_str())
        .zip(cands.iter().copied())
        .collect();
    let listed: Vec<Vec<FactId>> = a
        .states
        .iter()
        .map(|s| {
            s.set
                .iter()
                .map(|&b| by_text[a.candidates[b as usize].text.as_str()])
                .collect()
        })
        .collect();

    let listed_names: Vec<String> = listed.iter().map(|s| set_name(&terms, s)).collect();
    let root_plans = attribution::RootPlans::of(&root.plans, &terms);
    let fact_text: FxHashMap<FactId, String> =
        cands.iter().copied().zip(texts.iter().cloned()).collect();
    let plans = root.plans.clone();
    let Ok(mut nodes) = region::Nodes::new(&mut kb, &mut terms, &ast, &[]) else {
        row["nodes"] = json!("refused");
        return row;
    };

    // The cube, within the limit: two bits and stale facts per node.
    if n <= LIMIT {
        let t = Instant::now();
        let mut matcher = Duration::ZERO;
        let mut status = Vec::with_capacity(1 << n);
        let mut stale_nodes: Vec<Json> = Vec::new();
        let (mut in_r, mut owing, mut dead, mut dead_on) = (0, 0, 0, 0);
        for mask in 0..1u64 << n {
            let facts: Vec<FactId> = bits(mask, n).map(|b| cands[b]).collect();
            let (st, stale, on, dt) = nodes
                .with_fixpoint(&facts, |kb, terms| {
                    let (st, _) = bits_of(kb, terms, &ast);
                    let t = Instant::now();
                    let (stale, ids) = stale_facts(kb, terms, &ast, &plans);
                    let on = st & DEAD != 0 && dead_on_stale(kb, terms, &ids);
                    (st, stale, on, t.elapsed())
                })
                .expect("a fork saturates");
            matcher += dt;
            status.push(st);
            if !stale.is_empty() {
                match class(st) {
                    "dead" => dead += 1,
                    "owing" => owing += 1,
                    _ => in_r += 1,
                }
                if on {
                    dead_on += 1;
                }
                stale_nodes.push(json!({
                    "node": name(&texts, mask),
                    "class": class(st),
                    "stale": stale,
                    "dead_on_stale": on,
                }));
            }
        }
        let topo = Topology::of(n, &status);
        let (h, d, o) = (
            topo.holes.len(),
            topo.dead_not_upward_closed.count,
            topo.discharged_not_upward_closed.count,
        );
        let bit_by = if h > 0 || d > 0 || o > 0 {
            bit_by(path, &root_plans)
        } else {
            Vec::new()
        };
        row["cube"] = json!({
            "bit_by": bit_by,
            "nodes": 1u64 << n,
            "holes": h,
            "dead_not_upward_closed": d,
            "discharged_not_upward_closed": o,
            "bit": if h > 0 || d > 0 || o > 0 { "exercised" } else { "conservative" },
            "stale_nodes": stale_nodes,
            "stale_in_r": in_r,
            "stale_owing": owing,
            "stale_dead": dead,
            "dead_on_stale": dead_on,
            "matcher_us": us(matcher),
            "wall_us": us(t.elapsed()),
        });
    } else {
        row["cube"] = json!({ "nodes": format!("2^{n}"), "bit": "not known" });
    }

    // The listed states: stale facts, and the spot check.
    let t = Instant::now();
    let mut matcher = Duration::ZERO;
    let mut stale_listed: Vec<Json> = Vec::new();
    let mut digests: Vec<u64> = Vec::with_capacity(listed.len());
    for (k, set) in listed.iter().enumerate() {
        let (dg, stale, dt) = nodes
            .with_fixpoint(set, |kb, terms| {
                let dg = digest(kb, terms);
                let t = Instant::now();
                let (stale, _) = stale_facts(kb, terms, &ast, &plans);
                (dg, stale, t.elapsed())
            })
            .expect("a fork saturates");
        matcher += dt;
        digests.push(dg);
        if !stale.is_empty() {
            stale_listed.push(json!({ "state": listed_names[k], "stale": stale }));
        }
    }
    row["listed"] = json!({
        "states": listed.len(),
        "stale": stale_listed,
        "matcher_us": us(matcher),
        "wall_us": us(t.elapsed()),
    });

    let t = Instant::now();
    let (mut states, mut parents, mut skipped, mut sats) = (0u64, 0u64, 0u64, 0u64);
    let mut differ: Vec<String> = Vec::new();
    if bound > 0 {
        for (k, (set, &direct)) in listed.iter().zip(&digests).enumerate() {
            if set.len() < 2 {
                continue;
            }
            states += 1;
            for i in 0..set.len().min(bound) {
                let parent: Vec<FactId> = set
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &f)| f)
                    .collect();
                let last = set[i];
                let got = nodes
                    .with_fixpoints(std::slice::from_ref(&parent), |kbs, terms| {
                        let kb = &mut kbs[0];
                        if !contradiction::detect(kb, terms).is_empty() {
                            return None;
                        }
                        let (rel, args) = terms.facts.get(last);
                        let args = args.to_vec();
                        let prov = terms.provs.push(Prov::from_hypothesis(0, None));
                        let _ = kb.add_and_index_fact(terms, rel, &args, Some(prov));
                        let mut events = Events::off();
                        let mut s = Session {
                            kb,
                            terms,
                            ast: &ast,
                            events: &mut events,
                            memo: SharedMemo::default(),
                        };
                        if let Ok(mut sat) = Saturator::new(&mut s) {
                            let _ = sat.saturate(&mut s, None, &mut |_| {});
                        }
                        Some(digest(s.kb, s.terms))
                    })
                    .expect("a fork saturates");
                sats += 1;
                match got {
                    None => skipped += 1,
                    Some(g) => {
                        sats += 1;
                        parents += 1;
                        if g != direct {
                            differ.push(format!(
                                "{} by way of its parent without {}",
                                listed_names[k], fact_text[&last]
                            ));
                        }
                    }
                }
            }
        }
    }
    row["paths"] = json!({
        "bound": bound,
        "states": states,
        "parents": parents,
        "dead_parents_skipped": skipped,
        "saturations": sats,
        "differ": differ,
        "wall_us": us(t.elapsed()),
    });
    row
}

/// The sites the cube's violations are named for: each minimal hole and each
/// violation's witness, by `shared/attribution.rs`'s rule, on a third load
/// through `region::compute` — `site_census`'s `cube_row`, reduced to the
/// names.
fn bit_by(path: &Path, root: &attribution::RootPlans) -> Vec<String> {
    let Some((mut kb, mut terms, ast)) = census::loaded(path) else {
        return Vec::new();
    };
    let Ok(region::Outcome::Region(r)) = region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options::default(),
        &mut |_| {},
    ) else {
        return Vec::new();
    };
    let t = &r.topology;
    let sites = attribution::site_texts(&r.monotonicity.sites, &terms);
    let mut nodes = region::Nodes::new(&mut kb, &mut terms, &ast, &r.candidates).expect("prepares");
    let mut items: Vec<(u64, Option<u64>)> = t.minimal_holes.iter().map(|&h| (h, None)).collect();
    if let Some((a, b)) = t.dead_not_upward_closed.witness {
        items.push((a, Some(b)));
    }
    if let Some((a, b)) = t.discharged_not_upward_closed.witness {
        items.push((a, Some(b)));
    }
    let mut named: BTreeSet<String> = BTreeSet::new();
    for (mask, up) in items {
        let status = r.status[mask as usize];
        let a = match (up, status & DEAD == 0) {
            (Some(b), true) => attribution::owing(
                &mut nodes,
                &ast,
                &sites,
                root,
                &attribution::facts_of(&r, b),
                &attribution::facts_of(&r, mask),
            ),
            _ => attribution::node(
                &r,
                &mut nodes,
                &ast,
                &sites,
                root,
                mask,
                attribution::r_below(&r, mask),
            ),
        };
        // `rule [activator …]`, the site's text before its colon.
        named.extend(
            a.sites
                .iter()
                .map(|t| t.split(':').next().unwrap_or(t).to_string()),
        );
    }
    named.into_iter().collect()
}

// ── Reading a fixpoint (copied from node_census.rs) ─────────────────

fn bits_of(kb: &Kb, terms: &mut Terms, ast: &Ast) -> (u8, u32) {
    let dead = !contradiction::detect(kb, terms).is_empty();
    let owes = obligations::tally(kb, terms, ast, &SharedMemo::default(), &mut Events::off())
        .map(|o| o.total() as u32)
        .unwrap_or(u32::MAX);
    (
        (if dead { DEAD } else { 0 }) | (if owes > 0 { OWING } else { 0 }),
        owes,
    )
}

fn class(status: u8) -> &'static str {
    if status & DEAD != 0 {
        "dead"
    } else if status & OWING != 0 {
        "owing"
    } else {
        "R"
    }
}

fn digest(kb: &Kb, terms: &Terms) -> u64 {
    let mut facts: Vec<String> = kb.facts().map(|f| sexpr(terms, f)).collect();
    facts.sort();
    let mut h = FxHasher::default();
    facts.hash(&mut h);
    h.finish()
}

fn set_name(terms: &Terms, set: &[FactId]) -> String {
    if set.is_empty() {
        return "∅".to_string();
    }
    let mut v: Vec<String> = set.iter().map(|&f| sexpr(terms, f)).collect();
    v.sort();
    format!("{{{}}}", v.join(", "))
}

/// Every fact whose primary derivation's `absent` guard the fixpoint now
/// falsifies, as `fact ← rule (absent …)`, and their ids.
fn stale_facts(
    kb: &Kb,
    terms: &Terms,
    ast: &Ast,
    plans: &[Arc<Plan>],
) -> (Vec<String>, Vec<FactId>) {
    let mut out = Vec::new();
    let mut ids_out = Vec::new();
    let mut m = Matcher::new();
    // Only the facts whose primary derivation records an `absent`: the rest
    // cannot be stale. Sorted by text for a stable report.
    let mut ids: Vec<FactId> = kb
        .facts()
        .filter(|&f| {
            kb.primary(f)
                .is_some_and(|p| !terms.provs.get(p).absent.is_empty())
        })
        .collect();
    ids.sort_by_key(|&f| sexpr(terms, f));
    for f in ids {
        let Some(p) = kb.primary(f) else { continue };
        let prov = terms.provs.get(p);
        if prov.absent.is_empty() {
            continue;
        }
        let Some(rule) = prov.rule else { continue };
        let mut verdict: Option<bool> = None;
        'plans: for plan in plans.iter().filter(|pl| pl.rule == rule) {
            let regs = regs_of(plan, &prov.bindings);
            for d in plan.disjuncts.iter() {
                if d.guards.is_empty() || refs_of(plan, d, &regs).as_slice() != &*prov.absent {
                    continue;
                }
                verdict = Some(
                    plan.guards(d.guards)
                        .iter()
                        .any(|g| m.holds(kb, terms, ast, plan, g, &regs)),
                );
                break 'plans;
            }
        }
        let falsified =
            verdict.unwrap_or_else(|| prov.absent.iter().any(|r| naf_holds(kb, terms, r)));
        if falsified {
            let guards: Vec<String> = prov
                .absent
                .iter()
                .map(|r| format!("(absent {})", naf_text(terms, r.rel, &r.args)))
                .collect();
            out.push(format!(
                "{} ← {} {}",
                sexpr(terms, f),
                terms.sym(rule),
                guards.join(" ")
            ));
            ids_out.push(f);
        }
    }
    (out, ids_out)
}

/// A contradiction's witness rests on one of `stale`.
fn dead_on_stale(kb: &Kb, terms: &Terms, stale: &[FactId]) -> bool {
    let mut memo: FxHashMap<FactId, bool> = stale.iter().map(|&f| (f, true)).collect();
    fn rests(kb: &Kb, terms: &Terms, f: FactId, memo: &mut FxHashMap<FactId, bool>) -> bool {
        if let Some(&v) = memo.get(&f) {
            return v;
        }
        memo.insert(f, false);
        let premises: Vec<FactId> = kb
            .primary(f)
            .map(|p| terms.provs.get(p).premises.to_vec())
            .unwrap_or_default();
        let v = premises.into_iter().any(|q| rests(kb, terms, q, memo));
        memo.insert(f, v);
        v
    }
    contradiction::detect(kb, terms).iter().any(|c| {
        [c.positive, Some(c.negative)]
            .into_iter()
            .flatten()
            .any(|f| rests(kb, terms, f, &mut memo))
    })
}

fn regs_of(plan: &Plan, bindings: &[(Symbol, Value)]) -> Vec<Value> {
    let mut regs = vec![Value::UNBOUND; plan.n_regs as usize];
    for (i, (_, v)) in plan.seed.iter().enumerate() {
        regs[i] = *v;
    }
    for (name, v) in bindings {
        if let Some(r) = plan.reg_names.iter().position(|n| n == name) {
            regs[r] = *v;
        }
    }
    regs
}

fn refs_of(plan: &Plan, d: &Disjunct, regs: &[Value]) -> Vec<NafRef> {
    fn ground(plan: &Plan, g: &NafGuard, slot: &Slot, regs: &[Value]) -> NafArg {
        match slot {
            Slot::Reg(r) => match g.scope_of[*r as usize] {
                Some(parent) => {
                    let v = regs[parent as usize];
                    if v.is_unbound() || v.as_fact().is_some() {
                        NafArg::Free
                    } else {
                        NafArg::Value(v)
                    }
                }
                None => NafArg::Free,
            },
            Slot::Const(v) => NafArg::Value(*v),
            Slot::Nested { rel, slots } => NafArg::Nested {
                rel: *rel,
                args: plan
                    .slots(*slots)
                    .iter()
                    .map(|s| ground(plan, g, s, regs))
                    .collect(),
            },
            Slot::Opaque(_) => NafArg::Free,
        }
    }
    fn collect(plan: &Plan, g: &NafGuard, span: Span, regs: &[Value], out: &mut Vec<NafRef>) {
        for step in plan.steps(span) {
            match step {
                Step::Rel(r) => out.push(NafRef {
                    rel: r.rel,
                    args: plan
                        .slots(r.slots)
                        .iter()
                        .map(|s| ground(plan, g, s, regs))
                        .collect(),
                }),
                Step::Absent { sub } => collect(plan, g, *sub, regs, out),
                Step::Guard { .. } => {}
            }
        }
    }
    let mut out = Vec::new();
    for g in plan.guards(d.guards) {
        collect(plan, g, g.sub, regs, &mut out);
    }
    let mut seen: Vec<NafRef> = Vec::new();
    for r in out {
        if !seen.contains(&r) {
            seen.push(r);
        }
    }
    seen
}

fn naf_holds(kb: &Kb, terms: &Terms, r: &NafRef) -> bool {
    fn args_match(terms: &Terms, args: &[Value], pat: &[NafArg]) -> bool {
        args.len() == pat.len()
            && args.iter().zip(pat).all(|(v, p)| match p {
                NafArg::Free => true,
                NafArg::Value(x) => v == x,
                NafArg::Nested { rel, args: ps } => v.as_fact().is_some_and(|f| {
                    let (r2, a2) = terms.facts.get(f);
                    r2 == *rel && args_match(terms, a2, ps)
                }),
            })
    }
    kb.facts_of(r.rel)
        .any(|f| args_match(terms, terms.facts.args(f), &r.args))
}

fn naf_text(terms: &Terms, rel: Symbol, args: &[NafArg]) -> String {
    let mut s = format!("({}", terms.sym(rel));
    for a in args {
        s.push(' ');
        match a {
            NafArg::Free => s.push('_'),
            NafArg::Value(v) => s.push_str(&ein_infer::events::sexpr_value(terms, *v)),
            NafArg::Nested { rel, args } => s.push_str(&naf_text(terms, *rel, args)),
        }
    }
    s.push(')');
    s
}

fn bits(mask: u64, n: usize) -> impl Iterator<Item = usize> {
    (0..n).filter(move |b| mask >> b & 1 == 1)
}

fn name(cands: &[String], mask: u64) -> String {
    if mask == 0 {
        return "∅".to_string();
    }
    let v: Vec<&str> = bits(mask, cands.len()).map(|b| cands[b].as_str()).collect();
    format!("{{{}}}", v.join(", "))
}

// ── The static detector ─────────────────────────────────────────────

/// The saturation rules' dependency graph over `D` (`node_census`'s, copied):
/// an edge from each relation a plan reads to each it writes, negative at odd
/// `absent` depth, and a positive one from an activator relation. Each
/// strongly connected component holding a negative edge, with its parity.
fn cycles_over(
    kb: &Kb,
    terms: &Terms,
    plans: &[Arc<Plan>],
    dynamic: &BTreeSet<String>,
) -> Vec<Json> {
    let program = kb.program();
    let sat: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let mut labels: BTreeSet<String> = BTreeSet::new();
    for p in &sat {
        for w in monotone::writes_of(p, terms) {
            labels.insert(w.label(terms));
        }
        for (r, _) in monotone::reads_of(p, terms) {
            labels.insert(r.label(terms));
        }
    }
    let negatives: Vec<String> = labels
        .iter()
        .filter(|l| l.starts_with("(not ") && *l != "(not ?)")
        .cloned()
        .collect();
    let mut edges: BTreeSet<(String, String, bool, String)> = BTreeSet::new();
    for p in &sat {
        let rule = terms.sym(p.rule).to_string();
        let writes: Vec<String> = monotone::writes_of(p, terms)
            .into_iter()
            .map(|w| w.label(terms))
            .collect();
        for (r, depth) in monotone::reads_of(p, terms) {
            let from = r.label(terms);
            let froms = if from == "(not ?)" {
                negatives.clone()
            } else {
                vec![from]
            };
            for f in froms {
                for w in &writes {
                    edges.insert((f.clone(), w.clone(), depth % 2 == 1, rule.clone()));
                }
            }
        }
    }
    for rule in program.rules.values() {
        if rule.params.is_empty() {
            continue;
        }
        let act = terms.sym(rule.name).to_string();
        for p in sat.iter().filter(|p| p.rule == rule.name) {
            for w in monotone::writes_of(p, terms) {
                edges.insert((act.clone(), w.label(terms), false, act.clone()));
            }
        }
    }
    let es: Vec<&(String, String, bool, String)> = edges
        .iter()
        .filter(|(a, b, _, _)| dynamic.contains(a) && dynamic.contains(b))
        .collect();
    cycles(&es)
}

fn cycles(edges: &[&(String, String, bool, String)]) -> Vec<Json> {
    let mut ids: BTreeMap<&str, usize> = BTreeMap::new();
    for (a, b, _, _) in edges {
        let k = ids.len();
        ids.entry(a.as_str()).or_insert(k);
        let k = ids.len();
        ids.entry(b.as_str()).or_insert(k);
    }
    let mut names = vec![""; ids.len()];
    for (s, &i) in &ids {
        names[i] = s;
    }
    let n = ids.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b, _, _) in edges {
        adj[ids[a.as_str()]].push(ids[b.as_str()]);
    }
    let (comp, ncomp) = scc(&adj);
    let mut out = Vec::new();
    for c in 0..ncomp {
        let inside: Vec<&&(String, String, bool, String)> = edges
            .iter()
            .filter(|(a, b, _, _)| comp[ids[a.as_str()]] == c && comp[ids[b.as_str()]] == c)
            .collect();
        if !inside.iter().any(|e| e.2) {
            continue;
        }
        let members: Vec<usize> = (0..n).filter(|&v| comp[v] == c).collect();
        let mut parity: BTreeMap<usize, bool> = BTreeMap::new();
        parity.insert(members[0], false);
        let mut odd = false;
        let mut changed = true;
        while changed {
            changed = false;
            for (a, b, neg, _) in inside.iter().map(|e| &***e) {
                let (a, b) = (ids[a.as_str()], ids[b.as_str()]);
                if let Some(&pa) = parity.get(&a) {
                    let want = pa ^ neg;
                    match parity.get(&b) {
                        None => {
                            parity.insert(b, want);
                            changed = true;
                        }
                        Some(&pb) if pb != want => odd = true,
                        _ => {}
                    }
                }
            }
        }
        let mut rels: Vec<&str> = members.iter().map(|&v| names[v]).collect();
        rels.sort();
        let negs: BTreeSet<String> = inside
            .iter()
            .filter(|e| e.2)
            .map(|(a, b, _, r)| format!("{r}: {a} →¬ {b}"))
            .collect();
        out.push(json!({ "relations": rels, "negative": negs, "odd": odd }));
    }
    out
}

fn scc(adj: &[Vec<usize>]) -> (Vec<usize>, usize) {
    struct T<'a> {
        adj: &'a [Vec<usize>],
        index: Vec<usize>,
        low: Vec<usize>,
        on: Vec<bool>,
        stack: Vec<usize>,
        comp: Vec<usize>,
        next: usize,
        ncomp: usize,
    }
    impl T<'_> {
        fn visit(&mut self, v: usize) {
            self.index[v] = self.next;
            self.low[v] = self.next;
            self.next += 1;
            self.stack.push(v);
            self.on[v] = true;
            for k in 0..self.adj[v].len() {
                let w = self.adj[v][k];
                if self.index[w] == usize::MAX {
                    self.visit(w);
                    self.low[v] = self.low[v].min(self.low[w]);
                } else if self.on[w] {
                    self.low[v] = self.low[v].min(self.index[w]);
                }
            }
            if self.low[v] == self.index[v] {
                loop {
                    let w = self.stack.pop().expect("on the stack");
                    self.on[w] = false;
                    self.comp[w] = self.ncomp;
                    if w == v {
                        break;
                    }
                }
                self.ncomp += 1;
            }
        }
    }
    let n = adj.len();
    let mut t = T {
        adj,
        index: vec![usize::MAX; n],
        low: vec![0; n],
        on: vec![false; n],
        stack: Vec::new(),
        comp: vec![usize::MAX; n],
        next: 0,
        ncomp: 0,
    };
    for v in 0..n {
        if t.index[v] == usize::MAX {
            t.visit(v);
        }
    }
    (t.comp, t.ncomp)
}

/// The order check, new. A saturation rule is **deferred** when a disjunct
/// carries an `absent` guard: the boundary admits its candidates one a round,
/// in (priority, FIFO) order. A deferred rule A that reads under an `absent`
/// a relation in `D` which a deferred rule B concludes — directly, or through
/// rules that are not deferred, which fire before the boundary speaks — can be
/// admitted before B and keep a fact B's conclusion then falsifies. At one
/// priority the written order decides which goes first; with A's priority
/// earlier, A goes first in every order. B earlier is the stratified order.
fn order(kb: &Kb, terms: &Terms, plans: &[Arc<Plan>], dynamic: &BTreeSet<String>) -> Vec<Json> {
    let program = kb.program();
    let sat: Vec<&Plan> = plans
        .iter()
        .filter(|p| program.rules.contains(p.rule))
        .map(|p| &**p)
        .collect();
    let deferred = |p: &Plan| p.disjuncts.iter().any(|d| !d.guards.is_empty());
    let priority = |_: &Plan| -> i64 { DEFAULT_PRIORITY };
    // What each relation reaches through rules that are not deferred.
    let mut forward: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in sat.iter().filter(|p| !deferred(p)) {
        let w: Vec<String> = monotone::writes_of(p, terms)
            .into_iter()
            .map(|r| r.label(terms))
            .collect();
        for (r, d) in monotone::reads_of(p, terms) {
            if d % 2 == 0 {
                forward
                    .entry(r.label(terms))
                    .or_default()
                    .extend(w.iter().cloned());
            }
        }
    }
    let reach = |start: Vec<String>| -> BTreeSet<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut stack = start;
        while let Some(x) = stack.pop() {
            if seen.insert(x.clone())
                && let Some(next) = forward.get(&x)
            {
                stack.extend(next.iter().cloned());
            }
        }
        seen
    };
    let mut out: Vec<Json> = Vec::new();
    let mut said: FxHashSet<(String, String)> = FxHashSet::default();
    for a in sat.iter().filter(|p| deferred(p)) {
        let reads: BTreeSet<String> = monotone::reads_of(a, terms)
            .into_iter()
            .filter(|(_, d)| d % 2 == 1)
            .map(|(r, _)| r.label(terms))
            .filter(|l| dynamic.contains(l))
            .collect();
        for b in sat.iter().filter(|p| deferred(p) && p.rule != a.rule) {
            let concludes = reach(
                monotone::writes_of(b, terms)
                    .into_iter()
                    .map(|r| r.label(terms))
                    .collect(),
            );
            let raced: Vec<&String> = reads.intersection(&concludes).collect();
            if raced.is_empty() {
                continue;
            }
            let (ra, rb) = (terms.sym(a.rule).to_string(), terms.sym(b.rule).to_string());
            if !said.insert((ra.clone(), rb.clone())) {
                continue;
            }
            let (pa, pb) = (priority(a), priority(b));
            let how = if pa == pb {
                "one priority: the written order decides"
            } else if pa < pb {
                "the reader's priority is earlier: it goes first in every order"
            } else {
                continue;
            };
            out.push(json!({
                "reader": ra,
                "writer": rb,
                "relations": raced,
                "priorities": [pa, pb],
                "how": how,
            }));
        }
    }
    out
}

// ── The drafted rows ────────────────────────────────────────────────

fn row(label: &str, depth: usize, text: &str) -> String {
    let pad = if depth == 0 { 18 } else { 16 };
    format!("{}{label:<pad$}{text}", "  ".repeat(depth + 1))
}

/// `64 761`: the read-out's digit grouping.
fn group(n: u64) -> String {
    let d = n.to_string();
    let mut out = String::new();
    for (i, c) in d.chars().enumerate() {
        if i > 0 && (d.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

fn plural(n: u64, one: &str, many: &str) -> String {
    format!("{} {}", group(n), if n == 1 { one } else { many })
}

fn strs(v: &Json) -> Vec<&str> {
    v.as_array()
        .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
        .unwrap_or_default()
}

/// What is assumed, one row: the node, and who decides what it should be.
const ASSUMES: &str = "every node is a fork of root (root's fixpoint, its commitment added, \
                       saturated); what a node is on this program is open: Q-M1e2.2, F21";

/// The static half of the warning: one row per site with the property at
/// stake, then the cycles and the order findings.
fn static_rows(r: &Json) -> Vec<String> {
    let mut out = Vec::new();
    let sites = r["sites"].as_array().cloned().unwrap_or_default();
    for (i, s) in sites.iter().enumerate() {
        if i == ROWS {
            out.push(row(
                "…",
                1,
                &format!(
                    "{} more sites: --dump-answer DIR writes every one",
                    sites.len() - ROWS
                ),
            ));
            break;
        }
        let what = if s["kind"] == "activator" {
            format!("; {}", s["concludes"].as_str().unwrap_or(""))
        } else {
            format!(", concludes {}", s["concludes"].as_str().unwrap_or(""))
        };
        out.push(row(
            "site",
            1,
            &format!(
                "{}{what} — at stake: {}",
                s["site"].as_str().unwrap_or(""),
                s["at_stake"].as_str().unwrap_or("")
            ),
        ));
    }
    for c in r["cycles"].as_array().cloned().unwrap_or_default() {
        let odd = c["odd"].as_bool().unwrap_or(false);
        out.push(row(
            "cycle",
            1,
            &format!(
                "{} through an absent — {} — {}",
                if odd { "odd" } else { "even" },
                strs(&c["negative"]).join(" · "),
                if odd {
                    "a state with no stale fact need not exist"
                } else {
                    "several states with no stale fact can exist"
                }
            ),
        ));
    }
    let orders = r["order"].as_array().cloned().unwrap_or_default();
    let key = |o: &Json| {
        (
            o["reader"].as_str().unwrap_or("").to_string(),
            o["writer"].as_str().unwrap_or("").to_string(),
        )
    };
    let pairs: BTreeSet<(String, String)> = orders.iter().map(key).collect();
    let mut said: BTreeSet<(String, String)> = BTreeSet::new();
    for o in &orders {
        let (reader, writer) = key(o);
        if said.contains(&(writer.clone(), reader.clone())) {
            continue;
        }
        said.insert((reader.clone(), writer.clone()));
        let p = o["priorities"].as_array().cloned().unwrap_or_default();
        let at = p.first().cloned().unwrap_or_default();
        let mutual = pairs.contains(&(writer.clone(), reader.clone()));
        let text = if mutual {
            format!(
                "{reader} and {writer} each read under an absent what the other concludes, both at \
                 priority {at}: which the boundary admits first is the order they are written in"
            )
        } else if p.first() == p.get(1) {
            format!(
                "{reader} reads {} under an absent and {writer} concludes it, both at priority {at}: \
                 which the boundary admits first is the order they are written in",
                strs(&o["relations"]).join(", ")
            )
        } else {
            format!(
                "{reader} (priority {at}) reads {} under an absent and {writer} ({}) concludes it: \
                 {reader} is admitted first, whatever the order",
                strs(&o["relations"]).join(", "),
                p.get(1).cloned().unwrap_or_default()
            )
        };
        out.push(row("order", 1, &text));
    }
    out
}

/// The dynamic half: whether it bit, in the status (the cube) and in the
/// state (stale facts; with the spot check, parent paths).
fn bit_rows(r: &Json, with_paths: bool) -> Vec<String> {
    let mut out = Vec::new();
    let cube = &r["cube"];
    let n = r["n"].as_u64().unwrap_or(0);
    match cube["bit"].as_str() {
        Some("not known") => out.push(row(
            "status",
            1,
            &format!("not known — the cube is 2^{n} nodes, past --region-limit {LIMIT}"),
        )),
        Some("exercised") => {
            let (h, d, o) = (
                cube["holes"].as_u64().unwrap_or(0),
                cube["dead_not_upward_closed"].as_u64().unwrap_or(0),
                cube["discharged_not_upward_closed"].as_u64().unwrap_or(0),
            );
            out.push(row(
                "status",
                1,
                &format!(
                    "non-monotone here (exercised, by {}) — the cube: {} · {} · {}",
                    strs(&cube["bit_by"]).join(", "),
                    plural(h, "hole", "holes"),
                    plural(
                        d,
                        "dead node with an alive superset",
                        "dead nodes with an alive superset"
                    ),
                    plural(
                        o,
                        "discharged node with an owing superset",
                        "discharged nodes with an owing superset"
                    ),
                ),
            ));
        }
        Some(_) => out.push(row(
            "status",
            1,
            "monotone here (conservative) — the cube: no hole, no dead node with an alive \
             superset, no discharged node with an owing superset",
        )),
        None => {}
    }
    let (where_, stale): (String, Vec<Json>) = if cube["stale_nodes"].is_array() {
        (
            format!("the cube's {} nodes", cube["nodes"].as_u64().unwrap_or(0)),
            cube["stale_nodes"].as_array().cloned().unwrap_or_default(),
        )
    } else {
        let l = &r["listed"];
        (
            format!(
                "the {} met",
                plural(l["states"].as_u64().unwrap_or(0), "member", "members")
            ),
            l["stale"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|s| json!({ "node": s["state"], "class": "R", "stale": s["stale"] }))
                .collect(),
        )
    };
    if stale.is_empty() {
        out.push(row("state", 1, &format!("no stale fact in {where_}")));
    } else {
        out.push(row(
            "state",
            1,
            &format!(
                "stale facts — {} of {where_} {} one: a fact whose absent guard the node's own fixpoint falsifies",
                group(stale.len() as u64),
                if stale.len() == 1 { "holds" } else { "hold" }
            ),
        ));
        for (i, s) in stale.iter().enumerate() {
            if i == ROWS {
                out.push(row(
                    "…",
                    1,
                    &format!(
                        "{} more: --dump-answer DIR writes them",
                        group((stale.len() - ROWS) as u64)
                    ),
                ));
                break;
            }
            let class = match s["class"].as_str().unwrap_or("") {
                "R" => "in R",
                c => c,
            };
            out.push(row(
                "stale",
                1,
                &format!(
                    "{}, {class}: {}",
                    s["node"].as_str().unwrap_or(""),
                    strs(&s["stale"]).join(" · ")
                ),
            ));
        }
    }
    if with_paths {
        let p = &r["paths"];
        let (states, parents) = (
            p["states"].as_u64().unwrap_or(0),
            p["parents"].as_u64().unwrap_or(0),
        );
        let differ = strs(&p["differ"]);
        let text = if !cube["stale_nodes"].is_array() {
            format!(
                "not checked — the read-out prints no state of two commitments or more; its {} \
                 met would cost {} saturations",
                plural(
                    r["listed"]["states"].as_u64().unwrap_or(0),
                    "member",
                    "members"
                ),
                group(p["saturations"].as_u64().unwrap_or(0))
            )
        } else if parents == 0 {
            "no state the read-out prints has an alive parent to check".to_string()
        } else if differ.is_empty() {
            format!(
                "every parent path checked reaches the same state — {} of {}",
                plural(parents, "parent", "parents"),
                plural(states, "printed state", "printed states")
            )
        } else {
            format!(
                "path-dependent — {} of {} checked reach{} another state: {}",
                differ.len(),
                plural(parents, "parent path", "parent paths"),
                if differ.len() == 1 { "es" } else { "" },
                differ[0]
            )
        };
        out.push(row("paths", 1, &text));
    }
    out
}

fn lattice_block(r: &Json, with_paths: bool) -> Vec<String> {
    let mut out = Vec::new();
    if r["monotone"].as_bool().unwrap_or(true) {
        out.push(row(
            "lattice",
            0,
            "monotone (the static check) — R is convex: no hole",
        ));
        return out;
    }
    let sites = r["sites"].as_array().map_or(0, |a| a.len()) as u64;
    let holes = match r["cube"]["holes"].as_u64() {
        Some(0) => "no hole in this cube".to_string(),
        Some(h) => plural(h, "hole", "holes"),
        None => "holes not known: the cube was not enumerated".to_string(),
    };
    out.push(row(
        "lattice",
        0,
        &format!(
            "not monotone: {} (the static check) — {holes}",
            plural(sites, "site", "sites")
        ),
    ));
    out.push(row("assumes", 1, ASSUMES));
    out.extend(static_rows(r));
    out.extend(bit_rows(r, with_paths));
    out
}

fn monotone_block(r: &Json) -> Vec<String> {
    let n = r["n"].as_u64().unwrap_or(0);
    let mut out = vec![row(
        "asked",
        0,
        "is the program monotone? — the static check, on root's fixpoint",
    )];
    if r["monotone"].as_bool().unwrap_or(true) {
        out.push(row(
            "monotone",
            0,
            "yes — no site: dead and discharged are upward-closed, so R has no hole and \
             its two lines describe it",
        ));
    } else {
        let sites = r["sites"].as_array().map_or(0, |a| a.len()) as u64;
        out.push(row(
            "monotone",
            0,
            &format!("no — {}", plural(sites, "site", "sites")),
        ));
        out.push(row("assumes", 1, ASSUMES));
        out.extend(static_rows(r));
        let bit = if n as usize <= LIMIT {
            format!(
                "not computed — the cube is 2^{n} = {} saturations: `ein oracle FILE` computes \
                 it, and so does a bare `ein solve FILE`",
                1u64 << n
            )
        } else {
            format!("not computed — the cube is 2^{n} nodes, past --region-limit {LIMIT}")
        };
        out.push(row("bit", 1, &bit));
    }
    out.push(row("universe", 0, &plural(n, "candidate", "candidates")));
    out.push(row(
        "route",
        0,
        "none — a question about the program: root's fixpoint, then the static check",
    ));
    out.push(row("work", 0, "1 saturation, root's · 0 facts beyond root"));
    out
}

fn print_block(r: &Json) {
    let path = r["path"].as_str().unwrap_or("");
    if r["loads"] == json!(false) || r["saturates"] == json!(false) {
        println!("{path}: does not load or saturate\n");
        return;
    }
    let n = r["n"].as_u64().unwrap_or(0);
    let mono = r["monotone"].as_bool().unwrap_or(true);
    println!(
        "{path}  n {n} · {} · the check {:.2} ms from the load · the static detector {:.0} µs",
        if mono { "monotone" } else { "flagged" },
        r["check_us"].as_f64().unwrap_or(0.0) / 1e3,
        r["detector_us"].as_f64().unwrap_or(0.0)
    );
    let run = &r["run"];
    println!(
        "  run      route {} · exhausted {} · listed {} · {} saturations · {:.1} ms",
        run["route"].as_str().unwrap_or("?"),
        run["exhausted"],
        run["listed"],
        run["saturations"],
        run["wall_us"].as_f64().unwrap_or(0.0) / 1e3
    );
    if !mono {
        let c = &r["cube"];
        if c["stale_nodes"].is_array() {
            println!(
                "  cube     {} nodes · {} · stale nodes: in R {}, owing {}, dead {} (dead on a stale fact {}) · matcher {:.0} µs · wall {:.1} ms",
                c["nodes"],
                c["bit"].as_str().unwrap_or(""),
                c["stale_in_r"],
                c["stale_owing"],
                c["stale_dead"],
                c["dead_on_stale"],
                c["matcher_us"].as_f64().unwrap_or(0.0),
                c["wall_us"].as_f64().unwrap_or(0.0) / 1e3
            );
        } else {
            println!("  cube     not enumerated: 2^{n}");
        }
        let l = &r["listed"];
        println!(
            "  listed   {} states · {} with a stale fact · matcher {:.0} µs · wall {:.1} ms",
            l["states"],
            l["stale"].as_array().map_or(0, |a| a.len()),
            l["matcher_us"].as_f64().unwrap_or(0.0),
            l["wall_us"].as_f64().unwrap_or(0.0) / 1e3
        );
        let p = &r["paths"];
        println!(
            "  paths    bound {} · {} listed states of k ≥ 2 · {} parents checked · {} dead parents skipped · {} saturations · {} differ · {:.1} ms",
            p["bound"],
            p["states"],
            p["parents"],
            p["dead_parents_skipped"],
            p["saturations"],
            p["differ"].as_array().map_or(0, |a| a.len()),
            p["wall_us"].as_f64().unwrap_or(0.0) / 1e3
        );
        for d in strs(&p["differ"]) {
            println!("           {d}");
        }
    }
    println!("  ── drafted: the lattice block of a bare `ein solve`, the spot check's row last");
    for l in lattice_block(r, !mono) {
        println!("{l}");
    }
    if !mono && r["cube"]["holes"].as_u64().unwrap_or(0) > 0 {
        println!("    [today's minimal hole rows, unchanged: F21's D4]");
    }
    println!("  ── drafted: `ein solve FILE --monotone`, after the header");
    for l in monotone_block(r) {
        println!("{l}");
    }
    println!();
}
