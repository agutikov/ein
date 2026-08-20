//! What the search's **answer** does not depend on — S1a.7.0 T1a.7.0.4/5.
//!
//! [P1a.7](../../../../plans/m1a_rust/p1a.7_parallelism/README.md) wants to
//! evaluate a layer's enterings on many cores. Two properties have to hold
//! before that is even a design, and neither is about threads:
//!
//! 1. **Order.** The answer must not depend on the order the layer's
//!    candidates are entered in — otherwise "which worker finished first"
//!    reaches the output.
//! 2. **Integration time.** The answer must not depend on *when* an entering's
//!    root writes — its learned clause and its singleton `(not h)` writeback —
//!    become visible to the enterings after it. A parallel layer necessarily
//!    tests a batch of candidates against **one** KB and integrates what they
//!    learned afterwards.
//!
//! Both are invariance claims about the **answer** — the verdict and the set
//! of models — and about nothing else. The counters *do* move under both, on
//! purpose: a later integration prunes less, so the engine enters more
//! commitments to reach the same models. That is the trade the parallel mode
//! makes, and
//! [design/08 §2a](../../../../plans/m1a_rust/design/08_parallelism.md)
//! is where the argument for it is written.
//!
//! These are cheap here and were not cheap in ein.py — an exhaustive `zebra`
//! is 47 ms against ~8 s — which is why the property gets a test rather than a
//! paragraph.

use ein_core::{Kb, Terms};
use ein_infer::Events;
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// The answer, rendered so two runs compare without sharing an arena.
///
/// Each run loads the file afresh — a solve *writes* to root, so it cannot be
/// re-used — which means each has its own `Terms` and its own `FactId`
/// numbering. So the models go out as sorted s-expressions, for the reason
/// `fork_audit` gives: an id comparison across two arenas reports a difference
/// where there is only a different interning order.
#[derive(PartialEq, Eq, Debug)]
struct Answered {
    verdict: &'static str,
    /// One entry per model, each the model's whole fact set, sorted. The outer
    /// vector is sorted too: `Ambiguity` is a *set* of models, and the order
    /// the search happened to find them in is exactly what this test is
    /// asserting is not observable.
    models: Vec<Vec<String>>,
    /// For a `Contradiction`, the unsat core — the printed answer there.
    core: Vec<String>,
}

fn facts_of(kb: &Kb, terms: &Terms) -> Vec<String> {
    let mut out: Vec<String> = kb
        .facts()
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect();
    out.sort();
    out
}

/// Solve `rel` under `tweak`, and return only what must not move.
fn answer(
    rel: &str,
    tweak: impl FnOnce(&mut SolveOptions, &mut ein_core::SolverConfig),
) -> Answered {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    let mut opts = SolveOptions {
        // Exhaustive: the *set* of models is the claim, and `stop_after`
        // would compare prefixes of two traversals instead.
        stop_after: None,
        ..SolveOptions::default()
    };
    tweak(&mut opts, &mut cfg);
    opts.config = Some(cfg);
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e:?}"));
    let verdict = solved.answer.as_str();
    let (mut models, core) = match &solved.answer {
        Answer::Verdict(Verdict::Solution(s)) => (vec![facts_of(&s.kb, &terms)], Vec::new()),
        Answer::Verdict(Verdict::Ambiguity(ss)) => (
            ss.iter().map(|s| facts_of(&s.kb, &terms)).collect(),
            Vec::new(),
        ),
        Answer::Verdict(Verdict::Contradiction { unsat_core }) => {
            let mut c: Vec<String> = unsat_core
                .iter()
                .map(|&f| ein_infer::events::sexpr(&terms, f))
                .collect();
            c.sort();
            (Vec::new(), c)
        }
        Answer::Aborted { reason } => panic!("{rel} aborted: {reason}"),
    };
    models.sort();
    Answered {
        verdict,
        models,
        core,
    }
}

/// Files with a real lattice search — every one has at least two enterings, and
/// between them they cover a `Solution`, an `Ambiguity` and a `Contradiction`.
///
/// Deliberately not the whole corpus: these run the *exhaustive* path twice or
/// four times each, and a corpus-wide sweep of the same properties is
/// `utils/spec_audit.py`'s job, not a unit test's.
const FILES: &[&str] = &[
    "examples/zebra2.ein",
    "examples/zebra.ein",
    "examples/zebra2-hints.ein",
    "examples/branching/02_one_dead_one_alive.ein",
    "examples/branching/03_five_hyps_one_alive.ein",
    "examples/branching/04_two_levels.ein",
    "examples/branching/05_mini_zebra.ein",
    "examples/branching/09_hrule.ein",
    "examples/branching/10_kill_cache_on.ein",
    "examples/branching/11_kill_cache_off.ein",
    "examples/lattice/01_subset_pruned.ein",
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/lattice/03_state_hash_collision.ein",
    "examples/domain_elim/b_branch.ein",
    "examples/saturation/hypothesis-contradiction/coloc-disproved.ein",
    "examples/features/08_disjunct_guard_sets.ein",
];

/// **T1a.7.0.4 — the answer does not depend on the entering order.**
///
/// Four traversals of each file: the canonical `lex` order, the `score-sum`
/// order (a *different* deterministic permutation, not a random one), and two
/// seeded shuffles. `--shuffle` carries its generator across layers, so seeds 1
/// and 7 are two genuinely different walks of the same lattice.
#[test]
fn the_answer_does_not_depend_on_the_entering_order() {
    for rel in FILES {
        let base = answer(rel, |_, _| {});
        let scored = answer(rel, |_, cfg| cfg.lattice_order = "score-sum".into());
        assert_eq!(base, scored, "{rel}: score-sum order changed the answer");
        for seed in [1i64, 7] {
            let shuffled = answer(rel, |_, cfg| cfg.lattice_order_seed = Some(seed));
            assert_eq!(
                base, shuffled,
                "{rel}: shuffle seed {seed} changed the answer"
            );
        }
    }
}

/// **T1a.7.0.5 — the answer does not depend on when a layer integrates what it
/// learned.**
///
/// Three integration policies per file: after every entering (the sequential
/// engine), after every 4th, and once at the end of the layer. In the last
/// one, every candidate of a layer forks *the same* KB — which is exactly the
/// shape a parallel layer has, and the batched one is the shape it has when a
/// layer is too big to hold in flight.
#[test]
fn the_answer_does_not_depend_on_when_the_layer_integrates() {
    for rel in FILES {
        let sequential = answer(rel, |_, _| {});
        for batch in [4usize, usize::MAX] {
            let deferred = answer(rel, |o, _| o.integrate_every = Some(batch));
            assert_eq!(
                sequential, deferred,
                "{rel}: integrating every {batch} enterings changed the answer"
            );
        }
    }
}

/// The two properties are not independent, and a parallel engine uses them
/// together: a batch is entered in some order *and* integrates late.
#[test]
fn order_and_late_integration_compose() {
    for rel in FILES {
        let base = answer(rel, |_, _| {});
        let both = answer(rel, |o, cfg| {
            o.integrate_every = Some(usize::MAX);
            cfg.lattice_order_seed = Some(7);
        });
        assert_eq!(
            base, both,
            "{rel}: shuffled + late integration changed the answer"
        );
    }
}

/// The counters **do** move — and a test that did not say so would be read as
/// claiming they do not.
///
/// Late integration prunes later: a clause learned by candidate *i* does not
/// filter candidate *i + 1* of the same batch, and the `(not h)` writeback is
/// not in its fork. So the engine enters **at least** as many commitments to
/// reach the same models. Whether it enters *more* is a property of the
/// puzzle, and the three cells here are the three answers:
///
/// | puzzle | sequential | whole layer | why |
/// |---|---:|---:|---|
/// | `zebra2 -e` | 101 | **617** | 32 layer-1 writebacks, each pruning hard |
/// | `branching/06 -e` | 5 173 | 5 173 | **0** writebacks — nothing to defer |
/// | `branching/07 -e` | 11 501 | 11 501 | 162 writebacks that prune nothing |
///
/// So the cost of the mode is not a constant: it is whatever the singleton
/// writeback was buying on that puzzle, and on the deep searches that want
/// cores it is buying nothing.
#[test]
fn what_late_integration_costs_is_the_prune_it_defers() {
    for (rel, expect_more) in [
        ("examples/zebra2.ein", true),
        ("examples/branching/06_lookahead_on.ein", false),
    ] {
        let sequential = run_stats(rel, None);
        let whole_layer = run_stats(rel, Some(usize::MAX));
        assert!(
            whole_layer.enterings >= sequential.enterings,
            "{rel}: late integration entered fewer commitments \
             ({} < {}) — a prune cannot be *gained* by deferring it",
            whole_layer.enterings,
            sequential.enterings
        );
        if expect_more {
            assert!(
                whole_layer.enterings > sequential.enterings,
                "{rel}: late integration entered the same {} commitments — \
                 this puzzle was chosen because its layer 1 has 32 writebacks, \
                 so if the counts now agree the deferral is not deferring",
                sequential.enterings
            );
        } else {
            assert_eq!(
                whole_layer.enterings, sequential.enterings,
                "{rel}: this puzzle has no singleton writeback, so there is \
                 nothing for a deferral to cost"
            );
        }
    }
}

/// **Deferring is also what keeps root's layer stack shallow** — the finding
/// this test exists to pin, because it is the reason the mode is not merely
/// tolerable.
///
/// Every root write seals another layer (`Kb::fork` seals the top so the
/// parent's later appends land in a new one), and **every fork inherits the
/// whole stack**. On `branching/07 -e` the 162 mid-layer writebacks put root
/// at depth 164, and all 11 501 forks walk it. Deferring collapses that to
/// depth 3 — one sealed layer per layer barrier — for the *same* 11 501
/// enterings and the same answer, and the run is ~2.8× faster single-threaded
/// (1 117 ms → 401 ms on the dev machine; wall clock is not asserted here
/// because a test must not depend on a machine).
#[test]
fn deferring_collapses_roots_layer_stack() {
    let rel = "examples/branching/07_lookahead_off.ein";
    let sequential = run_stats(rel, None);
    let batched = run_stats(rel, Some(20));
    let whole_layer = run_stats(rel, Some(usize::MAX));

    assert_eq!(
        sequential.enterings, whole_layer.enterings,
        "{rel}: chosen because its writebacks prune nothing — if the entering \
         counts now differ, the premise of the depth comparison is gone"
    );
    assert!(
        sequential.depth > 100,
        "{rel}: expected root to end deep (one sealed layer per writeback), \
         got depth {}",
        sequential.depth
    );
    assert!(
        whole_layer.depth <= 8,
        "{rel}: a whole-layer barrier should leave about one sealed layer per \
         search layer, got depth {}",
        whole_layer.depth
    );
    assert!(
        batched.depth < sequential.depth && batched.depth > whole_layer.depth,
        "{rel}: batching at 20 should land between the two — got {} against \
         {} and {}",
        batched.depth,
        sequential.depth,
        whole_layer.depth
    );
}

/// A deep, multi-layer search is answer-identical under deferral too.
///
/// The fast files above are two-layer puzzles. These are five-layer searches
/// of 5 173 and 11 501 enterings, where a deferral compounds across layers —
/// and where a parallel engine would actually be used.
#[test]
fn a_deep_search_is_answer_identical_under_deferral() {
    for rel in [
        "examples/branching/06_lookahead_on.ein",
        "examples/branching/07_lookahead_off.ein",
    ] {
        let sequential = answer(rel, |_, _| {});
        let deferred = answer(rel, |o, _| o.integrate_every = Some(usize::MAX));
        assert_eq!(
            sequential, deferred,
            "{rel}: late integration changed the answer"
        );
    }
}

struct Ran {
    enterings: u64,
    depth: usize,
}

fn run_stats(rel: &str, batch: Option<usize>) -> Ran {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let opts = SolveOptions {
        stop_after: None,
        integrate_every: batch,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e:?}"));
    Ran {
        enterings: solved.stats.base.enterings_total,
        depth: kb.depth(),
    }
}
