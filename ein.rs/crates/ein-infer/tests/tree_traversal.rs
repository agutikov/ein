//! M1d [T1d.10.6.3](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.6_the_traversal.md)
//! — **the per-obligation tree**, in a process of its own.
//!
//! `obligation_rung_control.rs`'s idiom, for its reason: `EIN_TRAVERSAL` is
//! read from the process environment, so a file that sets it cannot share a
//! binary with tests that assert the default. Cargo gives each `tests/*.rs` its
//! own process, which is the cheapest serialisation there is.
//!
//! What the two tests here are for is the stage's second acceptance bullet —
//! *"the same models, on every entry that can run both. Not 'the same count' —
//! the same fact sets"* — and its guard, which is the finding that building it
//! produced: a tree on a rung that is **not** the obligations one is the
//! depth-first solver P1.5b deleted, and it costs what that cost.

use std::collections::BTreeSet;

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level, sexpr};
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file};

/// Solve a corpus entry, and return its model set as text plus the run's
/// counters. The fact sets are strings because two runs intern into two
/// arenas and a `FactId` does not survive the crossing.
fn run(rel: &str, cap: u32) -> (BTreeSet<Vec<String>>, u64, u64) {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let path = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let opts = SolveOptions {
        config: Some(kb.program().config.clone().unwrap_or_default()),
        stop_after: None,
        max_set_size: cap,
        ..SolveOptions::default()
    };
    let solved: Solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e:?}"));
    let mut models = BTreeSet::new();
    if let Answer::Verdict(Verdict::Ambiguity(branches)) = &solved.answer {
        for b in branches.iter() {
            let mut v: Vec<String> = b.kb.facts().map(|f| sexpr(&terms, f)).collect();
            v.sort();
            models.insert(v);
        }
    }
    if let Answer::Verdict(Verdict::Solution(s)) = &solved.answer {
        let mut v: Vec<String> = s.kb.facts().map(|f| sexpr(&terms, f)).collect();
        v.sort();
        models.insert(v);
    }
    let declined = buffer
        .to_string_lossy()
        .lines()
        .filter(|l| l.contains("\"traversal\"") && l.contains("declined"))
        .count() as u64;
    (models, solved.stats.base.enterings_total, declined)
}

/// **The tree finds the lattice's models, fact for fact, and enters 567× less
/// to do it.**
///
/// `examples/zebra2-minus-15-obligations.ein` is the entry the phase is named
/// after and the only corpus program with a model set large enough for the
/// comparison to mean anything: 32 models, which the lattice reaches at depth 3
/// after **48 745** enterings and cannot certify without 17 204 592.
///
/// **The lattice arm runs at `-m 2`, and the asymmetry is affordability rather
/// than a weaker claim.** All 32 need depth 3 and 48 745 enterings, which a
/// debug build does not survive; depth 2 has **28** of them for 4 656, and
/// *"every model the lattice found is one the tree found"* is what a lost model
/// would break. The tree takes no cap at all — its depth is bounded by the 46
/// instances root owes, not by `--max-set-size` — and `= 32` pins the other
/// direction, because a tree that lost four would still contain all 28.
///
/// The comparison is of **fact sets**, never of `k`. Two searches that agree on
/// a count and disagree on a model are exactly the failure
/// [`completeness.md`](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/completeness.md)
/// exists to rule out, and a count would not see it.
#[test]
fn the_tree_finds_the_lattices_models_fact_for_fact() {
    // SAFETY: single-threaded, before any solve in this process.
    unsafe { std::env::set_var("EIN_TRAVERSAL", "lattice") };
    let (lattice, lat_enterings, _) = run("examples/zebra2-minus-15-obligations.ein", 2);

    unsafe { std::env::set_var("EIN_TRAVERSAL", "tree") };
    let (tree, tree_enterings, declined) = run("examples/zebra2-minus-15-obligations.ein", 5);

    assert_eq!(declined, 0, "the tree declined on an obligations program");
    assert_eq!(
        lattice.len(),
        28,
        "the lattice arm is not the known baseline"
    );
    assert_eq!(tree.len(), 32, "the tree is not the known baseline");
    let lost: Vec<&Vec<String>> = lattice.difference(&tree).collect();
    assert!(
        lost.is_empty(),
        "{} model(s) the lattice found are not in the tree's set",
        lost.len()
    );
    assert!(
        tree_enterings * 10 < lat_enterings,
        "the tree entered {tree_enterings} against the lattice's {lat_enterings} \
         at a cap two layers shallower than the lattice needs"
    );
}

/// **A tree on any other rung is the solver that was deleted, so it declines.**
///
/// An hrule's candidates are not one owed instance's alternatives: they are not
/// jointly exhaustive, so branching on them walks hypothesis *paths* and
/// reaches a size-`d` commitment by `d!` routes — which is what P1.5b removed
/// the tree solver for in `8d77b02` (2026-05-29), and it came back on contact.
/// Measured before the guard existed: `examples/zebra2.ein` cost **7 877**
/// enterings against the lattice's 101, for the same single model.
///
/// So the tree asks the rung first and hands the run back. What this pins is
/// that it hands it back *identically* — the entering count on an hrule program
/// is the lattice's to the digit, because the lattice is what ran.
#[test]
fn the_tree_declines_on_a_rung_that_is_not_the_obligations_one() {
    unsafe { std::env::set_var("EIN_TRAVERSAL", "lattice") };
    let (lat_models, lat_enterings, _) = run("examples/zebra2.ein", 5);

    unsafe { std::env::set_var("EIN_TRAVERSAL", "tree") };
    let (tree_models, tree_enterings, declined) = run("examples/zebra2.ein", 5);

    assert_eq!(declined, 1, "the decline was not narrated");
    assert_eq!(
        tree_enterings, lat_enterings,
        "a declined tree did not hand the run back unchanged"
    );
    assert_eq!(tree_models, lat_models);
    assert_eq!(lat_enterings, 101, "the hrule baseline moved");
}
