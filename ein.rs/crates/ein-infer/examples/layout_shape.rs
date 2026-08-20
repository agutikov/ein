//! The distributions [S1a.6.2](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.2_memory_layout.md)
//! sizes its inline capacities from — T1a.6.2.3, and the evidence T1a.6.2.6
//! and T1a.6.2.2 are decided by.
//!
//! ```sh
//! cargo run --release -p ein-infer --example layout_shape
//! ```
//!
//! A layout is a bet about a distribution: an inline capacity of two is right
//! if two covers the mass and wrong if the tail is fat, and neither
//! `design/03` nor `design/05` had the histogram when they guessed. This
//! prints it — for the store the matcher reads on every candidate, for the
//! extents it scans, and for the plans it runs — after a **real solve**, so a
//! fork's derived facts are in it and not just the loaded ones.

use ein_core::{FactId, Kb, Terms};
use ein_infer::solve::{Dumper, EnteringInfo, SolveOptions, solve};
use ein_infer::{Events, compile_rule};
use ein_ir::{Ast, load_file};
use rustc_hash::FxHashMap;

/// The believed extents of every entering's *saturated* fork — which is what
/// `facts_of` actually walks, and is nothing like the loaded root's.
#[derive(Default)]
struct Extents {
    live: Vec<usize>,
    depth: Vec<usize>,
    facts: Vec<usize>,
}

impl Dumper for Extents {
    fn entering(
        &mut self,
        _layer: u32,
        _commitment: &[ein_core::FactId],
        terms: &Terms,
        _outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        if let Some(kb) = info.kb {
            self.depth.push(kb.depth());
            self.facts.push(kb.n_facts());
            // By what the facts *are*, not by what the program declares: a
            // unified is-a encoding introduces `co-located` as a fact, and a
            // histogram over `program().relations` misses exactly the extents
            // the matcher spends its time in.
            let mut per: FxHashMap<ein_core::Symbol, usize> = FxHashMap::default();
            for id in kb.facts() {
                *per.entry(terms.facts.rel(id)).or_default() += 1;
            }
            // determinism-ok: the extents go to `hist`, which histograms them and sorts its own keys — a multiset.
            self.live.extend(per.values().copied());
        }
    }
}

fn hist(label: &str, values: &[usize]) {
    if values.is_empty() {
        println!("  {label:<22} —");
        return;
    }
    let mut counts: FxHashMap<usize, usize> = FxHashMap::default();
    for v in values {
        *counts.entry(*v).or_default() += 1;
    }
    // determinism-ok: sorted on the next line, before any use.
    let mut keys: Vec<usize> = counts.keys().copied().collect();
    keys.sort_unstable();
    let n = values.len();
    let total: usize = values.iter().sum();
    let mut cum = 0usize;
    let mut cover = String::new();
    for k in &keys {
        cum += counts[k];
        cover.push_str(&format!("{k}:{:.1}% ", cum as f64 * 100.0 / n as f64));
    }
    println!(
        "  {label:<22} n={n}  mean={:.2}  max={}  cumulative {}",
        total as f64 / n as f64,
        keys.last().copied().unwrap_or(0),
        cover.trim_end()
    );
}

fn main() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root");

    for (rel, stop_after) in [
        ("examples/zebra2.ein", Some(1)),
        ("examples/zebra2.ein", None),
        ("examples/zebra.ein", Some(1)),
        ("examples/zebra.ein", None),
    ] {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb: Kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        let mut events = Events::off();
        let opts = SolveOptions {
            stop_after,
            ..SolveOptions::default()
        };
        let mut seen = Extents::default();
        let solved =
            solve(&mut kb, &mut terms, &ast, &mut events, &mut seen, &opts).expect("solves");
        let _ = solved;

        println!(
            "\n{rel} — after a {} solve",
            if stop_after.is_some() {
                "fast"
            } else {
                "exhaustive"
            }
        );

        // ── the fact store: what every candidate loads ──────────────
        let n = terms.facts.len();
        let arities: Vec<usize> = (0..n)
            .map(|i| terms.facts.arity(FactId(i as u32)))
            .collect();
        let args: usize = arities.iter().sum();
        let (inline, of) = terms.facts.inline_share();
        println!(
            "  facts={n}  args={args}  store={} KB  inline args {inline}/{of} \
             ({:.1}%)",
            terms.facts.footprint() / 1024,
            inline as f64 * 100.0 / of as f64
        );
        hist("arity per fact", &arities);

        // ── the extents the matcher scans ───────────────────────────
        let mut by_rel: FxHashMap<_, usize> = FxHashMap::default();
        for i in 0..n {
            *by_rel.entry(terms.facts.rel(FactId(i as u32))).or_default() += 1;
        }
        let mut extents: Vec<(String, usize)> = by_rel
            .iter()
            .map(|(r, c)| (terms.sym(*r).to_string(), *c))
            .collect();
        extents.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let sizes: Vec<usize> = extents.iter().map(|(_, c)| *c).collect();
        hist("interned per relation", &sizes);
        print!("  largest:");
        for (name, count) in extents.iter().take(6) {
            print!(" {name}={count}");
        }
        println!();

        // ── believed extents, which is what `facts_of` walks ────────
        hist("believed per relation", &seen.live);
        hist("facts per fork KB", &seen.facts);
        hist("layers per fork KB", &seen.depth);

        // ── the plans ───────────────────────────────────────────────
        let mut regs: Vec<usize> = Vec::new();
        let mut prems: Vec<usize> = Vec::new();
        let rules: Vec<_> = kb.program().rules.values().cloned().collect();
        for rule in rules {
            if let Ok(plan) = compile_rule(&ast, &mut terms, &rule, None) {
                regs.push(plan.n_regs as usize);
                for d in &plan.disjuncts {
                    prems.push(d.n_premises as usize);
                }
            }
        }
        hist("registers per plan", &regs);
        hist("premises per disjunct", &prems);
    }
}
