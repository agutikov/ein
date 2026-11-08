//! **The derived band against the declared priority** — M1f S1f.5.15's T3
//! (T1f.5.15.3), the task's deliverable.
//!
//! ```text
//! cargo run --release -p ein-infer --example rule_bands [-- --rows] [FILE…]
//! ```
//!
//! For each program (with no `FILE`, the corpus and the register's witnesses)
//! it saturates root, takes the plans the run compiled (the obligation pass's
//! included), builds the dependency graph and its strata
//! ([`ein_infer::strata::graph`]), and reads each rule's band off its shape
//! ([`ein_infer::strata::band`]) beside the band its `:priority` falls in
//! ([`ein_infer::strata::declared_band`]). A rule is counted once per
//! **definition**, its file and line, so a stdlib rule a hundred programs
//! import is one rule: the shape it is given is the first program's that
//! compiles it, and a definition whose shape differs between programs is
//! named. `--rows` prints one line per definition. It writes nothing and
//! changes no answer.

#[allow(dead_code)]
#[path = "shared/census.rs"]
mod census;

use std::collections::BTreeMap;
use std::path::PathBuf;

use ein_infer::strata::{self, Band, RuleBand, Shape};

/// One rule definition, as first met.
struct Def {
    rule: String,
    declared: Option<i64>,
    shape: Option<Shape>,
    derived: Option<Band>,
    stratum: Option<u32>,
    /// Distinct shapes over every program that compiles it.
    shapes: Vec<Shape>,
    programs: usize,
}

fn main() {
    let mut rows = false;
    let mut files: Vec<PathBuf> = Vec::new();
    for a in std::env::args().skip(1) {
        if a == "--rows" {
            rows = true;
        } else {
            let p = PathBuf::from(&a);
            files.push(if p.exists() {
                p
            } else {
                ein_corpus::repo_root().join(p)
            });
        }
    }
    if files.is_empty() {
        files = census::selection();
    }
    let mut defs: BTreeMap<String, Def> = BTreeMap::new();
    let (mut programs, mut stratified, mut deepest) = (0usize, 0usize, (0u32, String::new()));
    let mut unstratified: Vec<String> = Vec::new();
    for path in &files {
        let Some((mut kb, mut terms, ast)) = census::loaded(path) else {
            continue;
        };
        let Some((root, ())) = census::root_of(&mut kb, &mut terms, &ast, |_, _| Some(())) else {
            continue;
        };
        programs += 1;
        let g = strata::graph(&kb, &terms, &root.plans);
        let rel = census::relative(path);
        if g.stratified() {
            stratified += 1;
        } else {
            unstratified.push(rel.clone());
        }
        if g.depth() > deepest.0 {
            deepest = (g.depth(), rel.clone());
        }
        for RuleBand {
            rule,
            declared,
            shape,
            derived,
            stratum,
            ..
        } in strata::rule_bands(&kb, &terms, &root.plans, &g)
        {
            let name = terms.sym(rule).to_string();
            // A loaded rule carries no `Loc`, so its definition is read off
            // the text: the program's own, else the stdlib module it imports.
            let at = census::located_imported(path, &name);
            let at = if at == "?" {
                format!("{rel}:?{name}")
            } else {
                at
            };
            let d = defs.entry(at).or_insert_with(|| Def {
                rule: name.clone(),
                declared,
                shape: None,
                derived: None,
                stratum: None,
                shapes: Vec::new(),
                programs: 0,
            });
            d.programs += 1;
            if let Some(s) = shape {
                if d.shape.is_none() {
                    d.shape = Some(s);
                    d.derived = derived;
                    d.stratum = stratum;
                }
                if !d.shapes.contains(&s) {
                    d.shapes.push(s);
                }
            }
        }
    }
    // The diff.
    let mut table: BTreeMap<(String, String), usize> = BTreeMap::new();
    let (mut agree, mut differ, mut undeclared, mut uncompiled) = (0, 0, 0, 0);
    let mut varying = Vec::new();
    for (at, d) in &defs {
        if d.shapes.len() > 1 {
            varying.push(format!("{at} {}", d.rule));
        }
        let dec = d.declared.map(strata::declared_band);
        let key = (
            dec.map_or("(none)", Band::label).to_string(),
            d.derived.map_or("(not compiled)", Band::label).to_string(),
        );
        *table.entry(key).or_default() += 1;
        match (dec, d.derived) {
            (_, None) => uncompiled += 1,
            (None, Some(_)) => undeclared += 1,
            (Some(a), Some(b)) if a == b => agree += 1,
            _ => differ += 1,
        }
        if rows {
            let s = d.shape.map_or("-".to_string(), |s| {
                format!(
                    "{} naf={} premises={} activates={} exhausts={}",
                    s.concludes.label(),
                    s.naf,
                    s.premises,
                    s.activates,
                    s.exhausts
                )
            });
            println!(
                "{at}\t{}\t{}\t{}\t{}\t{}\t{s}\t{}",
                d.rule,
                d.declared.map_or("-".to_string(), |p| p.to_string()),
                dec.map_or("-", Band::label),
                d.derived.map_or("-", Band::label),
                d.stratum.map_or("-".to_string(), |k| k.to_string()),
                d.programs
            );
        }
    }
    println!(
        "{programs} programs saturated; {stratified} stratified; the deepest {} strata ({}); \
         a negative cycle on {}: {unstratified:?}",
        deepest.0,
        deepest.1,
        unstratified.len()
    );
    println!(
        "{} rule definitions: {agree} derive the band their priority falls in, {differ} another; \
         {undeclared} declare no priority; {uncompiled} compiled by no program",
        defs.len()
    );
    println!("declared band → derived band: definitions");
    for ((a, b), n) in &table {
        println!("  {a:>10} → {b:<14} {n}");
    }
    if !varying.is_empty() {
        println!("definitions whose shape differs between programs: {varying:?}");
    }
}
