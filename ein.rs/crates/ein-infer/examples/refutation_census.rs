//! The exposed set — M1e2
//! [S1e2.4.1](../../../../plans/m1e2_correctness/p1e2.4_what_the_switch_left/s1e2.4.1_refutation_under_absent.md),
//! which is M1f S1f.10.8's T1 taken in M1e2: every rule whose `(false)` or
//! `(not …)` conclusion rests on an `(absent P)` over a relation the search can
//! still extend, per corpus entry and register witness.
//!
//! *Still extendable* has two readings in the engine, and this prints both,
//! because the ruling has to say which one it means:
//!
//! | column | extendable means | where it is computed |
//! |---|---|---|
//! | `s1e23` | the relation is **hypothesis-eligible** ([`hypgen::eligible_relations`]): the generator's scope, read directly | [`naf_deps::compute_naf_map`], M1e S1e.2.3's census, pinned by `ein-infer/tests/refutation_under_absent.rs` |
//! | `direct` | the relation is in **`D`**: the root candidate list's relations, config-free, closed under *a saturation plan that reads `D` writes `D`* | [`monotone::classify`], the static check the region modes route on |
//! | `via` | the plan has no such `absent` itself, but reads **positively** a relation a non-monotone plan derives, so a commitment can take the premise away | this file: `N`, closed the same way `D` is |
//!
//! A `direct` row is a [`monotone::SiteKind::Absent`] site whose plan
//! refutes. The other sites — a non-refuting `absent` (W8's shape), an
//! obligation trigger, an activator — are counted, because they are what a
//! program still has after *forbid*.
//!
//! ```sh
//! cargo run --release -p ein-infer --example refutation_census -- [--json] [FILE…]
//! ```
//!
//! With no `FILE` it sweeps `corpus_files()` plus the register's witnesses.
//! It writes nothing and changes no answer: it loads, saturates root, takes
//! the universe and classifies, exactly as `region::prepare` does.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{Kb, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::compile::SharedMemo;
use ein_infer::events::Events;
use ein_infer::hypgen::{HypGenStats, eligible_relations, generate_config_free};
use ein_infer::monotone::{self, Rel, SiteKind};
use ein_infer::naf_deps::compute_naf_map;
use ein_infer::plan::Plan;
use ein_infer::saturator::{Saturator, Session};
use ein_ir::{Ast, load, parse};
use serde_json::json;

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

/// `R ∈ set`, the wildcard negative `(not ?)` read both ways — `monotone`'s
/// own membership test.
fn member(set: &BTreeSet<(String, bool)>, r: Rel, terms: &Terms) -> bool {
    if r.negated && r.name == terms.kernel.not {
        return set.iter().any(|(_, neg)| *neg);
    }
    let name = terms.sym(r.name).to_string();
    set.contains(&(name, r.negated))
        || (r.negated && set.contains(&(terms.sym(terms.kernel.not).to_string(), true)))
}

fn key(r: Rel, terms: &Terms) -> (String, bool) {
    (terms.sym(r.name).to_string(), r.negated)
}

/// `(false)` or `(not …)`, from every conclusion, `(false)` outranking.
fn refutes(plan: &Plan, terms: &Terms) -> Option<&'static str> {
    let w = monotone::writes_of(plan, terms);
    if w.iter()
        .any(|r| !r.negated && r.name == terms.kernel.r#false)
    {
        Some("(false)")
    } else if w.iter().any(|r| r.negated) {
        Some("(not …)")
    } else {
        None
    }
}

/// `rule[a b]`.
fn named(terms: &Terms, rule: Symbol, act: &[Symbol]) -> String {
    let a: Vec<&str> = act.iter().map(|&s| terms.sym(s)).collect();
    format!("{}[{}]", terms.sym(rule), a.join(" "))
}

/// Where `(rule NAME` is written, `file:line`: the entry itself, else every
/// stdlib module that defines the name (a loaded rule carries no `Loc` the
/// loader fills, so this reads the text).
fn located(entry: &Path, name: &str) -> String {
    let head = format!("(rule {name} ");
    let find = |p: &Path| -> Option<usize> {
        let text = std::fs::read_to_string(p).ok()?;
        text.lines()
            .position(|l| l.trim_start().starts_with(&head))
            .map(|i| i + 1)
    };
    let rel = |p: &Path| {
        p.strip_prefix(repo_root())
            .unwrap_or(p)
            .to_string_lossy()
            .to_string()
    };
    if let Some(line) = find(entry) {
        return format!("{}:{line}", rel(entry));
    }
    let mut hits: Vec<String> = std::fs::read_dir(repo_root().join("stdlib"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "ein"))
        .filter_map(|p| find(&p).map(|line| format!("{}:{line}", rel(&p))))
        .collect();
    hits.sort();
    if hits.is_empty() {
        "?".to_string()
    } else {
        hits.join(" | ")
    }
}

fn main() {
    let mut as_json = false;
    let mut files: Vec<PathBuf> = Vec::new();
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--json" => as_json = true,
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

    let (mut swept, mut flagged) = (0usize, 0usize);
    let given = files.len();
    // rule-location → entries, per column.
    let mut by_rule: BTreeMap<(&'static str, String, String), BTreeSet<String>> = BTreeMap::new();
    for path in files {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let memo = SharedMemo::default();
        let mut events = Events::off();

        // Root's fixpoint, and S1e.2.3's column off it.
        let s1e23 = {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
                memo: memo.clone(),
            };
            let Ok(mut sat) = Saturator::new(&mut s) else {
                continue;
            };
            if sat.saturate(&mut s, None, &mut |_| {}).is_err() {
                continue;
            }
            let Ok(eligible) = eligible_relations(&mut s) else {
                continue;
            };
            compute_naf_map(&sat.engine, s.terms, &eligible)
                .into_iter()
                .filter_map(|d| {
                    let r = d.refutation?;
                    Some((
                        d.rule,
                        d.activator.clone(),
                        r.concludes,
                        r.watching.join(", "),
                    ))
                })
                .collect::<Vec<_>>()
        };

        // The universe, config-free, and the static check — `region::prepare`.
        let mut proposed = Vec::new();
        {
            let mut s = Session {
                kb: &mut kb,
                terms: &mut terms,
                ast: &ast,
                events: &mut events,
                memo: memo.clone(),
            };
            let mut stats = HypGenStats::new();
            if generate_config_free(&mut s, &mut stats, &mut |f| {
                proposed.push(f);
                std::ops::ControlFlow::Continue(())
            })
            .is_err()
            {
                continue;
            }
        }
        if ein_infer::obligations::tally(&kb, &mut terms, &ast, &memo, &mut events).is_err() {
            continue;
        }
        let hypothesised: rustc_hash::FxHashSet<Symbol> =
            proposed.iter().map(|&f| terms.facts.get(f).0).collect();
        let plans = monotone::plans_of(&memo);
        let m = monotone::classify(&kb, &terms, &plans, &hypothesised);
        swept += 1;

        let dynamic: BTreeSet<(String, bool)> = m.dynamic.iter().map(|&r| key(r, &terms)).collect();
        let program = kb.program();
        let saturation: Vec<&Plan> = plans
            .iter()
            .filter(|p| program.rules.contains(p.rule))
            .map(|p| &**p)
            .collect();
        let odd_dynamic = |p: &Plan| -> Vec<String> {
            let mut v: Vec<String> = monotone::reads_of(p, &terms)
                .into_iter()
                .filter(|&(r, d)| d % 2 == 1 && member(&dynamic, r, &terms))
                .map(|(r, d)| format!("{}@{d}", r.label(&terms)))
                .collect();
            v.sort();
            v.dedup();
            v
        };

        // N: what a non-monotone saturation plan derives, closed under
        // "a saturation plan that reads N positively writes N".
        let mut shrinking: BTreeSet<(String, bool)> = BTreeSet::new();
        for p in &saturation {
            if !odd_dynamic(p).is_empty() {
                for w in monotone::writes_of(p, &terms) {
                    if w.name != terms.kernel.r#false {
                        shrinking.insert(key(w, &terms));
                    }
                }
            }
        }
        loop {
            let mut grew = false;
            for p in &saturation {
                let reads = monotone::reads_of(p, &terms);
                if reads
                    .iter()
                    .any(|&(r, d)| d % 2 == 0 && member(&shrinking, r, &terms))
                {
                    for w in monotone::writes_of(p, &terms) {
                        if w.name != terms.kernel.r#false {
                            grew |= shrinking.insert(key(w, &terms));
                        }
                    }
                }
            }
            if !grew {
                break;
            }
        }

        let mut direct = Vec::new();
        let mut via = Vec::new();
        for p in &saturation {
            let Some(concludes) = refutes(p, &terms) else {
                continue;
            };
            let odd = odd_dynamic(p);
            if !odd.is_empty() {
                direct.push((p.rule, p.activator_args.clone(), concludes, odd.join(", ")));
                continue;
            }
            let mut pos: Vec<String> = monotone::reads_of(p, &terms)
                .into_iter()
                .filter(|&(r, d)| d % 2 == 0 && member(&shrinking, r, &terms))
                .map(|(r, d)| format!("{}@{d}", r.label(&terms)))
                .collect();
            pos.sort();
            pos.dedup();
            if !pos.is_empty() {
                via.push((p.rule, p.activator_args.clone(), concludes, pos.join(", ")));
            }
        }
        let count = |k: SiteKind| m.sites.iter().filter(|s| s.kind == k).count();
        let absent_sites = count(SiteKind::Absent);
        let other_absent = m
            .sites
            .iter()
            .filter(|s| s.kind == SiteKind::Absent)
            .filter(|s| {
                !direct
                    .iter()
                    .any(|(r, a, _, _)| *r == s.rule && *a == s.activator)
            })
            .count();
        let triggers = count(SiteKind::ObligationTrigger);
        let activators = count(SiteKind::Activator);
        if !m.is_monotone() {
            flagged += 1;
        }
        if direct.is_empty() && via.is_empty() && s1e23.is_empty() && m.is_monotone() {
            continue;
        }

        let rows = |v: &[(Symbol, Box<[Symbol]>, &'static str, String)]| -> Vec<String> {
            let mut out: Vec<String> = v
                .iter()
                .map(|(r, a, c, w)| {
                    format!(
                        "{} {c} ← {w}  ({})",
                        named(&terms, *r, a),
                        located(&path, terms.sym(*r))
                    )
                })
                .collect();
            out.sort();
            out
        };
        for (col, v) in [("s1e23", &s1e23), ("direct", &direct), ("via", &via)] {
            for (r, _, _, _) in v.iter() {
                by_rule
                    .entry((
                        col,
                        terms.sym(*r).to_string(),
                        located(&path, terms.sym(*r)),
                    ))
                    .or_default()
                    .insert(rel.clone());
            }
        }
        let h: Vec<&str> = m.hypothesised.iter().map(|&s| terms.sym(s)).collect();
        if as_json {
            println!(
                "{}",
                json!({
                    "path": rel,
                    "n": proposed.len(),
                    "H": h,
                    "monotone": m.is_monotone(),
                    "s1e23": rows(&s1e23),
                    "direct": rows(&direct),
                    "via": rows(&via),
                    "sites": {"absent": absent_sites, "absent_not_refuting": other_absent,
                              "obligation_trigger": triggers, "activator": activators},
                })
            );
        } else {
            println!(
                "{rel}\n  n {}  H {{{}}}  monotone {}  sites: absent {absent_sites} \
                 (not refuting {other_absent}), obligation-trigger {triggers}, activator {activators}",
                proposed.len(),
                h.join(" "),
                if m.is_monotone() { "yes" } else { "no" },
            );
            for (col, v) in [("s1e23", &s1e23), ("direct", &direct), ("via", &via)] {
                for line in rows(v) {
                    println!("  {col:<6} {line}");
                }
            }
        }
    }
    if !as_json {
        println!(
            "\nswept {given} files: {swept} load and saturate; flagged non-monotone {flagged}"
        );
        for col in ["s1e23", "direct", "via"] {
            let rules: Vec<_> = by_rule.iter().filter(|((c, _, _), _)| *c == col).collect();
            let entries: BTreeSet<&String> = rules.iter().flat_map(|(_, e)| e.iter()).collect();
            println!(
                "{col:<6} {} rules over {} entries",
                rules.len(),
                entries.len()
            );
            for ((_, r, at), e) in &rules {
                let e: Vec<&str> = e.iter().map(String::as_str).collect();
                println!("         {r} ({at}) — {}", e.join(", "));
            }
        }
    }
}
