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
//! cargo run --release -p ein-infer --example refutation_census -- --option-e [--json] [FILE…]
//! ```
//!
//! **`--option-e`** (M1e2 S1e2.1b.7) reads F21's D1, option (E): *forbid*
//! extended to every `absent` at odd depth in a saturation rule, whatever the
//! rule concludes, under two of S1e2.2.5's F1 readings of *extendable*: the
//! relation in `D` (b), and the relation hypothesis-eligible (a). Per program
//! it lists the rules (E) would refuse under each, what each concludes, and
//! which of them *forbid* already refuses (`direct`) or would through F2's
//! `via`. F1's fact grain, (c), is `site_census`'s per-site verdict.
//!
//! With no `FILE` it sweeps `corpus_files()` plus the register's witnesses.
//! It writes nothing and changes no answer: it loads, saturates root, takes
//! the universe and classifies, exactly as `region::prepare` does.

// `site_census` reads the rest of it.
#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use census::{located, named, refutes};
use ein_core::{Symbol, Terms, Value};
use ein_infer::hypgen::eligible_relations;
use ein_infer::monotone::{self, Rel, SiteKind};
use ein_infer::naf_deps::compute_naf_map;
use ein_infer::plan::Plan;
use serde_json::json;

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

fn main() {
    let mut as_json = false;
    let mut option_e = false;
    let mut files: Vec<PathBuf> = Vec::new();
    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--json" => as_json = true,
            "--option-e" => option_e = true,
            _ => files.push(PathBuf::from(a)),
        }
    }
    if files.is_empty() {
        files = census::selection();
    }
    if option_e {
        return e_census(&files, as_json);
    }

    let (mut swept, mut flagged) = (0usize, 0usize);
    let given = files.len();
    // rule-location → entries, per column.
    let mut by_rule: BTreeMap<(&'static str, String, String), BTreeSet<String>> = BTreeMap::new();
    for path in files {
        let rel = census::relative(&path);
        let Some((mut kb, mut terms, ast)) = census::loaded(&path) else {
            continue;
        };

        // Root's fixpoint, S1e.2.3's column off it, and the static check —
        // `region::prepare`.
        let Some((root, s1e23)) = census::root_of(&mut kb, &mut terms, &ast, |sat, s| {
            let eligible = eligible_relations(s).ok()?;
            Some(
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
                    .collect::<Vec<_>>(),
            )
        }) else {
            continue;
        };
        let (plans, m, proposed) = (root.plans, root.monotonicity, root.proposed);
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

        let rows = |v: &[(Symbol, Box<[Value]>, &'static str, String)]| -> Vec<String> {
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

/// F21's D1, option (E), per program: every saturation rule that reads a
/// relation at odd `absent` depth, under F1 (b) (`D`) and F1 (a) (eligible),
/// with what it concludes and whether *forbid* already refuses it.
fn e_census(files: &[PathBuf], as_json: bool) {
    let mut swept = 0usize;
    // reading → (programs refused, of which forbid already refuses them all)
    let mut sum: BTreeMap<&'static str, (BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
    for path in files {
        let rel = census::relative(path);
        let Some((mut kb, mut terms, ast)) = census::loaded(path) else {
            continue;
        };
        let Some((root, eligible)) =
            census::root_of(&mut kb, &mut terms, &ast, |_, s| eligible_relations(s).ok())
        else {
            continue;
        };
        swept += 1;
        let m = root.monotonicity;
        let dynamic: BTreeSet<(String, bool)> = m.dynamic.iter().map(|&r| key(r, &terms)).collect();
        let eligible: BTreeSet<(String, bool)> = eligible
            .iter()
            .map(|&s| (terms.sym(s).to_string(), false))
            .collect();
        let program = kb.program();
        let saturation: Vec<&Plan> = root
            .plans
            .iter()
            .filter(|p| program.rules.contains(p.rule))
            .map(|p| &**p)
            .collect();
        let mut out = serde_json::Map::new();
        for (reading, set) in [("D", &dynamic), ("eligible", &eligible)] {
            let mut rows: Vec<serde_json::Value> = Vec::new();
            for p in &saturation {
                let mut reads: Vec<String> = monotone::reads_of(p, &terms)
                    .into_iter()
                    .filter(|&(r, d)| d % 2 == 1 && member(set, r, &terms))
                    .map(|(r, d)| format!("{}@{d}", r.label(&terms)))
                    .collect();
                reads.sort();
                reads.dedup();
                if reads.is_empty() {
                    continue;
                }
                let concludes = match refutes(p, &terms) {
                    Some(c) => c.to_string(),
                    None => {
                        let mut w: Vec<String> = monotone::writes_of(p, &terms)
                            .into_iter()
                            .map(|w| w.label(&terms))
                            .collect();
                        w.sort();
                        w.dedup();
                        w.join(" ")
                    }
                };
                rows.push(json!({
                    "rule": named(&terms, p.rule, &p.activator_args),
                    "at": located(path, terms.sym(p.rule)),
                    "reads": reads,
                    "concludes": concludes,
                    "refutes": refutes(p, &terms).is_some(),
                }));
            }
            rows.sort_by_key(|r| r["rule"].as_str().unwrap_or("").to_string());
            rows.dedup();
            if !rows.is_empty() {
                let e = sum
                    .entry(if reading == "D" {
                        "(b) D"
                    } else {
                        "(a) eligible"
                    })
                    .or_default();
                e.0.insert(rel.clone());
                if rows.iter().all(|r| r["refutes"].as_bool() == Some(true)) {
                    e.1.insert(rel.clone());
                }
            }
            out.insert(reading.to_string(), json!(rows));
        }
        if out
            .values()
            .all(|v| v.as_array().is_some_and(|a| a.is_empty()))
        {
            continue;
        }
        let row = json!({ "path": rel, "n": root.proposed.len(), "monotone": m.is_monotone(), "rules": out });
        if as_json {
            println!("{row}");
        } else {
            println!(
                "{rel}  n {}  {}",
                root.proposed.len(),
                if m.is_monotone() {
                    "monotone"
                } else {
                    "flagged"
                }
            );
            for reading in ["D", "eligible"] {
                for r in row["rules"][reading].as_array().into_iter().flatten() {
                    println!(
                        "  {reading:<8} {} → {} ← {}  ({})",
                        r["rule"].as_str().unwrap_or("?"),
                        r["concludes"].as_str().unwrap_or("?"),
                        r["reads"],
                        r["at"].as_str().unwrap_or("?")
                    );
                }
            }
        }
    }
    if !as_json {
        println!("\nswept {} files: {swept} load and saturate", files.len());
        for (reading, (all, only_refuting)) in &sum {
            println!(
                "{reading}: (E) refuses {} programs; on {} of them every such rule refutes, so forbid's direct set already reaches them",
                all.len(),
                only_refuting.len()
            );
            let rest: Vec<&String> = all.difference(only_refuting).collect();
            for r in rest {
                println!("    {r}");
            }
        }
    }
}
