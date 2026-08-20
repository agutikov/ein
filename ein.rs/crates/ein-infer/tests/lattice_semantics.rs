//! probe 6 — temporary

use ein_core::Terms;
use ein_infer::events::Events;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file};

#[test]
fn probe_contradictions_with_deads() {
    let t0 = std::time::Instant::now();
    for path in ein_oracle::corpus_files() {
        let rel = path
            .strip_prefix(ein_oracle::repo_root())
            .unwrap_or(&path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(mut kb) = load_file(&mut ast, &mut terms, &path) else {
            continue;
        };
        let opts = SolveOptions {
            stop_after: None,
            max_set_size: 5,
            store_lattice: true,
            max_enterings: Some(400),
            on_budget: OnBudget::Verdict,
            ..SolveOptions::default()
        };
        let mut events = Events::off();
        let Ok(solved) = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) else {
            continue;
        };
        let Answer::Verdict(Verdict::Contradiction { unsat_core }) = &solved.answer else {
            continue;
        };
        let Some(proof) = solved.proof.as_ref() else {
            continue;
        };
        let mut union: Vec<String> = proof
            .dead_commitments
            .iter()
            .flat_map(|d| d.unsat_core.iter())
            .map(|&f| ein_infer::events::sexpr(&terms, f))
            .collect();
        union.sort();
        union.dedup();
        let mut core: Vec<String> = unsat_core
            .iter()
            .map(|&f| ein_infer::events::sexpr(&terms, f))
            .collect();
        core.sort();
        eprintln!(
            "{rel}: deads={} core={} union_eq={} exhausted={}",
            proof.dead_commitments.len(),
            core.len(),
            core == union,
            solved.stats.exhausted,
        );
    }
    eprintln!("elapsed {:?}", t0.elapsed());
}
