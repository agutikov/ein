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

/// One run: what must not move, and the two numbers that say *how* it got
/// there. `depth` is root's layer stack at exit — never an output
/// ([`Kb::depth`] reaches four probes and no renderer), which is exactly why
/// it needs a test rather than a golden.
struct Ran {
    answered: Answered,
    enterings: u64,
    depth: usize,
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
    run(rel, tweak).answered
}

/// One solve, reporting both halves: the answer that must not move and the
/// traversal that is allowed to.
///
/// The two used to be separate helpers running separate solves. They are one
/// because T1a.7.2.0's claim needs both of the *same* run — a knob that keeps
/// the answer while moving the entering count is a different animal from one
/// that keeps both, and `coalesce_root_at` is the second kind.
fn run(rel: &str, tweak: impl FnOnce(&mut SolveOptions, &mut ein_core::SolverConfig)) -> Ran {
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
    Ran {
        answered: Answered {
            verdict,
            models,
            core,
        },
        enterings: solved.stats.base.enterings_total,
        depth: kb.depth(),
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
/// | `zebra2 -e` | 101 | **521** | 32 layer-1 writebacks, each pruning hard |
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
        let sequential = run(rel, |_, _| {});
        let whole_layer = run(rel, |o, _| o.integrate_every = Some(usize::MAX));
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

/// **T1a.7.2.0 — the layer barrier is what keeps root's layer stack shallow,
/// and unlike a deferral it costs no prune to do it.**
///
/// Every root write seals another layer — `Kb::fork` seals the top so the
/// parent's later appends land in a new one — and **every fork inherits the
/// whole stack**. On `branching/07 -e` the 162 mid-layer writebacks put root at
/// depth 164 and all 11 501 forks walk it; coalescing the stack at the layer
/// barrier takes the run from 899 ms to 278 ms on the dev machine at
/// `--jobs 1` (wall clock is not asserted here — a test must not depend on a
/// machine — and the numbers are
/// [scaling.md §6](../../../../plans/m1a_rust/p1a.7_parallelism/scaling.md)'s).
///
/// This test was `deferring_collapses_roots_layer_stack` until T1a.7.2.0, and
/// the re-pointing **is** the finding. The depth collapse was first measured
/// through [`SolveOptions::integrate_every`], which reached it by holding a
/// layer's root writes back — so the natural reading was that a parallel
/// engine had to defer to get it. The entering counts say otherwise: they are
/// equal in all three arms below, so none of the win is the deferral and all
/// of it is the read path. S1a.7.2 therefore takes the depth with
/// `Kb::flatten`, integration stays immediate, and the deferral's price —
/// 101 → 521 enterings on `zebra2 -e`,
/// `what_late_integration_costs_is_the_prune_it_defers` — is never paid.
#[test]
fn coalescing_at_the_barrier_collapses_roots_layer_stack() {
    let rel = "examples/branching/07_lookahead_off.ein";
    let uncoalesced = run(rel, |o, _| o.coalesce_root_at = None);
    let shipping = run(rel, |_, _| {});
    let deferred = run(rel, |o, _| {
        o.coalesce_root_at = None;
        o.integrate_every = Some(usize::MAX);
    });

    assert!(
        uncoalesced.depth > 100,
        "{rel}: with the barrier off, root should end deep — one sealed layer \
         per writeback — got depth {}. This file was chosen for its 162 \
         writebacks; if they have gone, so has the premise of the test",
        uncoalesced.depth
    );
    assert!(
        shipping.depth <= 8,
        "{rel}: the layer barrier should leave at most about one sealed layer \
         per search layer, got depth {}",
        shipping.depth
    );
    // The load-bearing one: same traversal, not merely the same answer. It is
    // what separates the flatten from the deferral, and what lets `--jobs N`
    // promise identical counters later in the phase.
    assert_eq!(
        shipping.enterings, uncoalesced.enterings,
        "{rel}: coalescing root moved the entering count — it rebuilds a \
         representation and must not reach the traversal"
    );
    assert_eq!(
        shipping.answered, uncoalesced.answered,
        "{rel}: coalescing root changed the answer"
    );
    // And the arm the finding came from: deferring reaches the same depth on
    // this file for the same enterings, which is why it looked like the way to
    // get there. Kept as the before-column, not as a shipping mode.
    assert!(
        deferred.depth <= 8 && deferred.enterings == uncoalesced.enterings,
        "{rel}: chosen because its writebacks prune nothing, so a whole-layer \
         deferral collapses the stack for free — got depth {} and {} enterings \
         against {}",
        deferred.depth,
        deferred.enterings,
        uncoalesced.enterings
    );
}

/// The same claim where the deferral is *not* free — the pair of files that
/// separates the two mechanisms in one table.
///
/// | | enterings | root depth at exit |
/// |---|---:|---:|
/// | `zebra2 -e`, barrier off | 101 | 35 |
/// | `zebra2 -e`, shipping | **101** | **2** |
/// | `zebra2 -e`, whole-layer deferral | **617** | 2 |
///
/// A deferral that buys depth by not pruning is a different trade from a
/// flatten that buys it by rebuilding a layer, and on the zebra family the
/// difference is 5.2× the enterings. Both zebras write back on almost every
/// layer-1 candidate, which is why they are the cell where it shows.
#[test]
fn coalescing_costs_no_prune_where_deferring_costs_many() {
    for rel in ["examples/zebra2.ein", "examples/zebra.ein"] {
        let uncoalesced = run(rel, |o, _| o.coalesce_root_at = None);
        let shipping = run(rel, |_, _| {});
        let deferred = run(rel, |o, _| o.integrate_every = Some(usize::MAX));
        assert!(
            uncoalesced.depth > shipping.depth,
            "{rel}: layer 1 writes back on nearly every candidate, so with the \
             barrier off root must end deeper than {} — got {}",
            shipping.depth,
            uncoalesced.depth
        );
        assert_eq!(
            shipping.enterings, uncoalesced.enterings,
            "{rel}: coalescing root moved the entering count"
        );
        assert_eq!(
            shipping.answered, uncoalesced.answered,
            "{rel}: coalescing root changed the answer"
        );
        assert!(
            deferred.enterings > shipping.enterings,
            "{rel}: this puzzle was chosen because deferring costs prunes here \
             — if the counts now agree, the comparison has lost its point"
        );
    }
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
