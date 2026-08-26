//! The monotonic solver's budget, its learned clauses, and what a dumper sees
//! — T1a.10.2.2.
//!
//! Replaces three Python files, all under `ein.py/tests/inference/monotonic/`:
//!
//! | Python | subject |
//! |---|---|
//! | `test_monotonic_budget.py` | `max_time` and the partial stats an abort carries |
//! | `test_monotonic_cdcl.py` | no-good learning: emit, subsume, prune, and the singleton `(not h)` writeback |
//! | `test_monotonic_dumper.py` | the lifecycle hooks — that they fire, that they are honest, and that they change nothing |
//!
//! The common subject is the **search's bookkeeping**: what it learns from a
//! dead commitment, what it refuses to learn twice, and what it tells an
//! observer it did. `ein-infer/src/{solve,nogoods}.rs`.
//!
//! Two notes on how the port differs from the originals.
//!
//! The Python tests read `kb._nogoods` and `kb._negated_facts` directly. Those
//! are the *store*, not the behaviour, so what is asserted here is what the
//! store buys: a superset of a dead set is never entered, a retired hypothesis
//! never comes back, and a live sibling of a dead one still reaches the
//! answer. Where the store itself is the claim — `emit_nogood`'s two
//! subsumption directions and its size floor, neither of which any `.ein`
//! program in the corpus reaches — it is called directly, as a unit.
//!
//! The dumper tests used `MonotonicDumper(tmp_path)` and read
//! `00_timeline.jsonl`. That writer lives in `ein-render`, which depends on
//! this crate, so a test here cannot construct one. [`Recorder`] stands in: it
//! implements the same [`Dumper`] trait and keeps the same fields the timeline
//! record carries, so "the record says X" becomes "the hook was handed X" —
//! one step closer to the engine and one step further from the file format,
//! which is the half that belongs here.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_core::{FactId, Kb, Program, SolverConfig, Terms};
use ein_infer::events::{Buffer, Events, Level, sexpr};
use ein_infer::solve::{
    Dumper, EnteringInfo, LatticeProof, MonotonicStats, NoDumper, OnBudget, SolveError,
    SolveOptions, Solved, solve,
};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file, parse};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

// ── The fixtures ───────────────────────────────────────────────────

/// One singleton dies, one survives — `test_monotonic_cdcl.py`'s
/// `SINGLETON_FIXTURE`.
///
/// `paint`'s slots carry `Thing` rather than the top `T`, which is what keeps
/// hypgen's name-free candidate set down to `{Red, Blue}`; `never` is declared
/// and never asserted so the goal stays unreachable and the search runs the
/// whole layer instead of stopping at the first model.
const SINGLETON: &str = r#"
(rule forbid-paint-Blue ()
  :match  (paint Blue ?y)
  :assert (false)
  :why    "Blue can't paint anything"
  :priority 100)
(relation paint Thing Thing)
(relation never T)
(is-a Thing T)
(is-a Red Thing) (is-a Blue Thing)

(query
       :goal  (never ?x)
       :hypothesis-relations paint)
"#;

/// A pair dies that neither of its elements dies alone — `MULTI_FIXTURE`.
///
/// `(R a b)` and `(R b c)` together fire the kill rule; either on its own
/// saturates fine. Solved to `max_set_size = 3` so there *is* a layer 3 for
/// the learned pair to prune.
const MULTI: &str = r#"
(rule kill-ab-bc ()
  :match  (and (R a b) (R b c))
  :assert (false)
  :why    "(R a b) + (R b c) is forbidden"
  :priority 100)
(relation R Thing Thing)
(relation never T)
(is-a Thing T)
(is-a a Thing) (is-a b Thing) (is-a c Thing)

(query
       :goal  (never ?x)
       :hypothesis-relations R)
"#;

/// Every layer-1 singleton dies — `ALL_DIE_FIXTURE`.
const ALL_DIE: &str = r#"
(rule forbid-h-a-b ()
  :match  (h a b)
  :assert (false)
  :why    "(h a b) forbidden"
  :priority 100)
(rule forbid-h-b-a ()
  :match  (h b a)
  :assert (false)
  :why    "(h b a) forbidden"
  :priority 100)
(relation h Thing Thing)
(is-a a Thing) (is-a b Thing)

(query
       :goal  (h ?x ?y)
       :hypothesis-relations h)
"#;

/// Lookahead off, as every one of the Python originals ran.
///
/// With it on, hypgen kills a doomed singleton *before* the candidate loop
/// sees it, and the `(not h)` lands through the kill cache instead of through
/// a death. Everything below is about what the monotonic loop does with a
/// death, so the deaths have to reach it.
fn no_lookahead() -> SolverConfig {
    SolverConfig {
        enable_pre_branch_lookahead: false,
        enable_lookahead_kill_cache: false,
        ..SolverConfig::default()
    }
}

// ── Running ────────────────────────────────────────────────────────

fn load(text: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<fixture>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// Solve an inline fixture under `opts`, with `dumper` watching.
fn run(text: &str, opts: &SolveOptions, dumper: &mut dyn Dumper) -> (Solved, Terms) {
    let (ast, mut terms, mut kb) = load(text);
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, dumper, opts)
        .unwrap_or_else(|e| panic!("the fixture solves: {e:?}"));
    (solved, terms)
}

/// The same for a corpus file.
fn run_file(rel: &str, opts: &SolveOptions, dumper: &mut dyn Dumper) -> (Solved, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, dumper, opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e:?}"));
    (solved, terms)
}

fn opts(max_set_size: u32) -> SolveOptions {
    SolveOptions {
        stop_after: None,
        max_set_size,
        config: Some(no_lookahead()),
        ..SolveOptions::default()
    }
}

/// Every fact of every model, sorted — the answer as text, so two runs with
/// two arenas compare.
fn models(solved: &Solved, terms: &Terms) -> Vec<Vec<String>> {
    let one = |k: &Kb| {
        let mut v: Vec<String> = k.facts().map(|f| sexpr(terms, f)).collect();
        v.sort();
        v
    };
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(s)) => vec![one(&s.kb)],
        Answer::Verdict(Verdict::Ambiguity(ss)) => ss.iter().map(|s| one(&s.kb)).collect(),
        _ => Vec::new(),
    }
}

/// The negated facts at root, as s-expressions — the `(not h)` writeback's
/// public face, and what `--print-final-state` shows.
fn retired(kb: &Kb, terms: &Terms) -> BTreeSet<String> {
    kb.negated().map(|f| sexpr(terms, f)).collect()
}

// ── The recording dumper ───────────────────────────────────────────

/// One lifecycle hook, with the fields `MonotonicDumper` would have written
/// into `00_timeline.jsonl` for it.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Hook {
    RootInitial,
    LayerStart {
        layer: u32,
    },
    Entering {
        layer: u32,
        outcome: String,
        commitment: Vec<String>,
        nogood_emitted: bool,
        nogood_subsumed: bool,
    },
    LayerEnd {
        layer: u32,
    },
    ProofSummary,
    Summary {
        verdict: String,
    },
    Close,
}

/// A [`Dumper`] that keeps the hook stream instead of writing it out.
#[derive(Default)]
struct Recorder {
    hooks: Vec<Hook>,
}

impl Recorder {
    /// The hook stream reduced to its names, which is what an ordering claim
    /// is actually about.
    fn kinds(&self) -> Vec<&'static str> {
        self.hooks
            .iter()
            .map(|h| match h {
                Hook::RootInitial => "root_initial",
                Hook::LayerStart { .. } => "layer_start",
                Hook::Entering { .. } => "entering",
                Hook::LayerEnd { .. } => "layer_end",
                Hook::ProofSummary => "proof_summary",
                Hook::Summary { .. } => "summary",
                Hook::Close => "close",
            })
            .collect()
    }

    fn enterings(&self) -> impl Iterator<Item = &Hook> {
        self.hooks
            .iter()
            .filter(|h| matches!(h, Hook::Entering { .. }))
    }
}

impl Dumper for Recorder {
    fn root_initial(&mut self, _kb: &Kb, _terms: &Terms) {
        self.hooks.push(Hook::RootInitial);
    }

    fn layer_start(&mut self, layer: u32, _kb: &Kb, _terms: &Terms, _n_alive: usize) {
        self.hooks.push(Hook::LayerStart { layer });
    }

    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        let mut commitment: Vec<String> = commitment.iter().map(|&f| sexpr(terms, f)).collect();
        commitment.sort();
        self.hooks.push(Hook::Entering {
            layer,
            outcome: outcome.to_string(),
            commitment,
            nogood_emitted: info.nogood_emitted,
            nogood_subsumed: info.nogood_subsumed,
        });
    }

    fn layer_end(&mut self, layer: u32, _kb: &Kb, _terms: &Terms, _n: usize, _next: usize) {
        self.hooks.push(Hook::LayerEnd { layer });
    }

    fn proof_summary(&mut self, _proof: &LatticeProof, _terms: &Terms) {
        self.hooks.push(Hook::ProofSummary);
    }

    fn summary(&mut self, verdict: &Answer, _stats: &MonotonicStats) {
        self.hooks.push(Hook::Summary {
            verdict: verdict.as_str().to_string(),
        });
    }

    fn close(&mut self) {
        self.hooks.push(Hook::Close);
    }
}

// ── 1) The budget ──────────────────────────────────────────────────

/// **max-time-abort** — a spent time budget aborts at the next candidate, and
/// says so in the units it was given.
///
/// Two things here are easy to get wrong and neither is the abort itself.
///
/// The limit is rendered with `{:?}`, not `{}`: `0.0` has to come back as
/// `0.0s`, because CPython's `str(float)` never drops the fractional part and
/// Rust's `Display` always does. A `Display` here would produce
/// `max-time (0s) exceeded` and every other assertion in this test would still
/// pass.
///
/// And the partial stats have to say `exhausted = false`. `MonotonicStats`
/// starts life with `exhausted = true` — the abort raises before the verdict
/// is read, so nothing later would correct it, and an aborted run would report
/// itself as a completed proof of `k`.
#[test]
fn a_spent_time_budget_aborts_with_partial_stats_and_a_python_shaped_reason() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(
        &mut ast,
        &mut terms,
        &repo_root().join("examples/lattice/01_subset_pruned.ein"),
    )
    .expect("loads");
    let mut events = Events::off();
    let opts = SolveOptions {
        // Zero seconds: root saturation has already burned some wall clock by
        // the time the first candidate comes up, so the very first check
        // trips. Nothing here depends on how fast the machine is.
        max_time: Some(0.0),
        on_budget: OnBudget::Raise,
        ..SolveOptions::default()
    };
    let outcome = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts);
    let Err(SolveError::Budget { reason, stats }) = outcome else {
        panic!("expected a budget abort before the first entering");
    };
    assert_eq!(reason, "max-time (0.0s) exceeded");
    assert!(
        !stats.exhausted,
        "an aborted run is not a proof — `exhausted` must be false, \
         otherwise `k` reads as certified"
    );
    assert!(
        stats.base.layers_explored >= 1,
        "the abort happens inside the candidate loop, so layer 1 was entered"
    );
    assert_eq!(
        stats.base.enterings_total, 0,
        "the check runs before the counter is bumped, so a budget spent at \
         the first candidate reports zero enterings, not one"
    );
}

// ── 2) What a death teaches the search ─────────────────────────────

/// **writeback-retires-only-the-dead-candidate** — `(not h)` retires `h`, and
/// nothing else.
///
/// The subtlety is that `alive` is recomputed from root *after* the writeback,
/// so a writeback one predicate too wide would empty it and the run would end
/// `Contradiction` with `k = 0` — a plausible-looking answer for a fixture
/// whose whole point is that one branch dies. So the assertion is the
/// surviving sibling: `(paint Red Blue)` is still enterable after
/// `(paint Blue Red)` dies, it is the only complete branch, and it is the
/// model.
#[test]
fn the_writeback_retires_the_dead_hypothesis_and_not_its_live_sibling() {
    let mut rec = Recorder::default();
    let (ast, mut terms, mut kb) = load(SINGLETON);
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut rec, &opts(2))
        .expect("the fixture solves");

    assert_eq!(solved.answer.as_str(), "Solution");
    assert_eq!(solved.stats.solution_nodes, 1);
    assert_eq!(solved.stats.base.enterings_dead_post, 1);

    let model = models(&solved, &terms);
    assert!(
        model[0].iter().any(|f| f == "(paint Red Blue)"),
        "the surviving singleton is the model; got {:?}",
        model[0]
    );

    let out = retired(&kb, &terms);
    assert!(
        out.contains("(paint Blue Red)"),
        "the death writes `(not h)` back at root; got {out:?}"
    );
    assert!(
        !out.contains("(paint Red Blue)"),
        "only the candidate that died is retired; got {out:?}"
    );

    // And the survivor really was entered after the death — the writeback did
    // not quietly remove it from the candidate set.
    let entered_live = rec.enterings().any(|h| {
        matches!(h, Hook::Entering { commitment, outcome, .. }
                 if commitment.iter().any(|c| c == "(paint Red Blue)")
                    && outcome == "solution")
    });
    assert!(
        entered_live,
        "the live sibling must still be entered and reach a solution: {:?}",
        rec.hooks
    );
}

/// **learned-clause-prunes-supersets-pre-fork** — a k-element clause stops
/// every later superset *before* it forks.
///
/// The pruning is a **subset** test against the store, not apriori's
/// prefix-join, and this fixture is chosen so the two differ: the dead pair is
/// `{(R a b), (R b c)}`, and the layer-3 triples that contain it are generated
/// by joining pairs that do not both contain it — a prefix-only filter would
/// let them through. What proves it happened pre-fork is that the triples were
/// never entered at all: had one forked, it would have re-fired the kill rule,
/// died, and left a subsumed size-3 clause behind. The store staying at one
/// clause and `nogoods_subsumed` staying at zero is that argument, and the
/// hook stream is the direct evidence.
#[test]
fn a_learned_clause_stops_every_superset_before_it_forks() {
    let mut rec = Recorder::default();
    let (ast, mut terms, mut kb) = load(MULTI);
    let mut events = Events::off();
    let o = SolveOptions {
        store_lattice: true,
        ..opts(3)
    };
    let solved =
        solve(&mut kb, &mut terms, &ast, &mut events, &mut rec, &o).expect("the fixture solves");

    assert_eq!(
        solved.stats.base.enterings_dead_post, 1,
        "the pair dies once and nothing else does"
    );
    assert_eq!(solved.stats.base.nogoods_emitted, 1);
    assert_eq!(
        solved.stats.base.nogoods_subsumed, 0,
        "a subsumed clause would mean a superset was entered after all"
    );

    let proof = solved.proof.as_ref().expect("store_lattice was asked for");
    let clauses: Vec<Vec<String>> = proof
        .learned_nogoods
        .iter()
        .map(|c| {
            let mut v: Vec<String> = c.iter().map(|&f| sexpr(&terms, f)).collect();
            v.sort();
            v
        })
        .collect();
    assert_eq!(
        clauses,
        vec![vec!["(R a b)".to_string(), "(R b c)".to_string()]],
        "the size-2 clause is the whole store — a superset of it would be \
         redundant, and subsumption exists to keep it out"
    );

    let (mut triples, mut triples_holding_one) = (0usize, 0usize);
    for h in rec.enterings() {
        let Hook::Entering {
            commitment, layer, ..
        } = h
        else {
            unreachable!()
        };
        let ab = commitment.iter().any(|c| c == "(R a b)");
        let bc = commitment.iter().any(|c| c == "(R b c)");
        assert!(
            *layer < 3 || !(ab && bc),
            "a superset of the dead pair was entered: {commitment:?} — the \
             pre-fork subset check did not fire"
        );
        if *layer == 3 {
            triples += 1;
            triples_holding_one += usize::from(ab || bc);
        }
    }
    assert!(
        triples > 0,
        "layer 3 has to happen for the loop above to have anything to say"
    );
    assert!(
        triples_holding_one > 0,
        "the survivors have to include triples built *from* the dead pair's \
         elements — otherwise the pair was pruned out of the generator \
         wholesale and the subset check was never the thing being tested"
    );
}

/// **empty-alive-is-a-contradiction** — when every layer-1 singleton dies the
/// search stops there and reports `Contradiction`, not "no model found yet".
///
/// The distinction is `k`. Both singletons die, so no solution node is ever
/// recorded and `k = 0`; the verdict is *read* from `k` rather than chosen, so
/// the same zero that would mean "truncated" under a budget means "refuted"
/// here — which is why `exhausted` is asserted alongside it. The clauses stay
/// at two because the two deaths are disjoint and neither subsumes the other.
#[test]
fn every_layer_1_singleton_dying_is_a_contradiction() {
    let mut rec = Recorder::default();
    let (ast, mut terms, mut kb) = load(ALL_DIE);
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut rec, &opts(2))
        .expect("the fixture solves");

    assert_eq!(solved.answer.as_str(), "Contradiction");
    assert_eq!(solved.stats.solution_nodes, 0, "k = 0");
    assert!(
        solved.stats.exhausted,
        "nothing truncated this run — the zero is a refutation, not a cap"
    );
    assert_eq!(solved.stats.base.enterings_dead_post, 2);
    assert_eq!(
        solved.stats.base.enterings_alive, 0,
        "no candidate survived, so no layer 2 was ever generated"
    );
    assert_eq!(solved.stats.base.nogoods_emitted, 2);

    let out = retired(&kb, &terms);
    for h in ["(h a b)", "(h b a)"] {
        assert!(out.contains(h), "{h} must be retired at root; got {out:?}");
    }
    for h in rec.enterings() {
        let Hook::Entering { layer, outcome, .. } = h else {
            unreachable!()
        };
        assert_eq!(*layer, 1, "the search never left layer 1");
        assert!(outcome.starts_with("dead"), "outcome was {outcome}");
    }
}

/// **a-cap-of-zero-is-a-cap** —
/// [T1d.10.5.0](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.5_contract.md).
/// The twin of the test above, and the reason that one asserts `exhausted`
/// rather than just reading `k`.
///
/// `for layer in 1..=max_set_size` is empty at zero, so before this the cut
/// had no door to be recorded through: `truncated` stayed false, `exhausted`
/// stayed at its `true` default, and the run reported *the constraints are
/// contradictory* with an empty unsat core over a frontier it had not looked
/// at — on every program with anything to guess. The same fixture, the same
/// `k = 0`, and the opposite meaning: at `-m 2` the zero is a refutation and
/// at `-m 0` it is a cap.
///
/// `alive_at_end` stays empty here, which is `stop_after`'s shape rather than
/// the depth cap's — the field is what was entered and *survived*, and a cap
/// of zero enters nothing.
#[test]
fn a_cap_of_zero_is_a_truncation_not_a_refutation() {
    let (cut, _) = run(ALL_DIE, &opts(0), &mut NoDumper);
    assert_eq!(cut.answer.as_str(), "Contradiction");
    assert_eq!(cut.stats.solution_nodes, 0, "k = 0");
    assert!(
        !cut.stats.exhausted,
        "a cap of zero left the whole frontier unexplored — the zero is \
         \"no model within the cap\", not a refutation"
    );
    assert_eq!(cut.stats.base.enterings_total, 0, "nothing was entered");
    assert_eq!(cut.stats.base.layers_explored, 0, "no layer ran");
    assert_eq!(
        cut.stats.base.nogoods_emitted, 0,
        "nothing died, so nothing was learned — the empty unsat core is the \
         same emptiness"
    );
    assert!(
        cut.proof.as_ref().is_none_or(|p| p.alive_at_end.is_empty()),
        "a cap that entered nothing has no survivors to hand a deeper run"
    );

    // The contrast, and it is the whole point: same program, same `k`.
    let (done, _) = run(ALL_DIE, &opts(2), &mut NoDumper);
    assert_eq!(done.stats.solution_nodes, 0, "k = 0 either way");
    assert!(
        done.stats.exhausted,
        "at -m 2 the same zero is a refutation"
    );
}

/// **a-cap-of-zero-does-not-refuse-a-question-the-root-answers.** The other
/// half of [T1d.10.5.0](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/s1d.10.5_contract.md),
/// and the reason `-m 0` is a *truncation* and not the `Aborted` shape `-E 0`
/// uses.
///
/// A program whose root is already complete has no lattice to exhaust, so
/// `-m 0` answers it exactly and `exhausted` is honestly `true`. Refusing
/// these would be refusing a question the engine can answer — and P1d.10's own
/// reconnaissance asks it 171 times, once per node, as
/// `ein solve -m 0 --json-summary`.
///
/// Both verdicts are covered because they reach `finalise` by different
/// routes: `Solution` records a node in Phase 1's empty-`alive` arm, `Open`
/// records the same node and then fails to discharge what it owes.
#[test]
fn a_cap_of_zero_still_answers_a_root_that_needs_no_search() {
    // The file's own `(config …)`, not `opts`' lookahead-off override: what is
    // being checked is the shipping answer.
    let bare = SolveOptions {
        max_set_size: 0,
        ..SolveOptions::default()
    };
    for (rel, want) in [
        ("examples/branching/01_saturate_only.ein", "Solution"),
        ("tests/stdlib/algebra/23_total_owed.ein", "Open"),
    ] {
        let (r, _) = run_file(rel, &bare, &mut NoDumper);
        assert_eq!(r.answer.as_str(), want, "{rel}");
        assert!(
            r.stats.exhausted,
            "{rel}: root is complete, so there is no lattice to leave unexplored"
        );
        assert_eq!(r.stats.solution_nodes, 1, "{rel}: the root state itself");
        assert_eq!(r.stats.base.layers_explored, 0, "{rel}");
    }
}

// ── 3) `emit_nogood` as a unit ─────────────────────────────────────

/// A clause of hand-made facts. They never enter a `Kb` — `emit_nogood` reads
/// only the no-good store and `Terms`, which is the point of testing it here
/// rather than through a program.
fn clause(terms: &mut Terms, rel: &str, pairs: &[(&str, &str)]) -> Vec<FactId> {
    let r = terms.intern_text(rel).expect("room");
    pairs
        .iter()
        .map(|(a, b)| {
            let a = terms.value_text(a).expect("room");
            let b = terms.value_text(b).expect("room");
            terms.intern_fact(r, &[a, b]).expect("room")
        })
        .collect()
}

fn empty_kb() -> Kb {
    Kb::new(Program::new())
}

fn stored(kb: &Kb, terms: &Terms) -> BTreeSet<Vec<String>> {
    kb.nogoods()
        .read()
        .expect("the store")
        .iter()
        .map(|c| {
            let mut v: Vec<String> = c.iter().map(|&f| sexpr(terms, f)).collect();
            v.sort();
            v
        })
        .collect()
}

/// The `nogood` events of one run, as `(emitted, subsumed, removed)` — with
/// `-1` standing for "no `removed` field", which is what a refusal writes.
fn nogood_events(buf: &Buffer) -> Vec<(bool, bool, i64)> {
    buf.to_string_lossy()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|v| v["e"] == "nogood")
        .map(|v| {
            (
                v["emitted"].as_bool().expect("emitted"),
                v["subsumed"].as_bool().expect("subsumed"),
                v["removed"].as_i64().unwrap_or(-1),
            )
        })
        .collect()
}

/// The clause `{(R a b), (R c d)}`, as the store renders it.
fn the_pair() -> BTreeSet<Vec<String>> {
    BTreeSet::from([vec!["(R a b)".to_string(), "(R c d)".to_string()]])
}

/// **emit-nogood-subsumption-both-directions** — the store stays minimal
/// whichever order the clauses arrive in.
///
/// Both directions matter and only one of them is obvious. A superset arriving
/// second is *dropped*: the stored clause already forbids everything it
/// forbids, so keeping it would mean checking two clauses to learn one thing.
/// A subset arriving second is stronger, so it goes in and every stored strict
/// superset comes out; the count of what it evicted rides on the `nogood`
/// event, which is the only place that eviction is visible at all.
///
/// No `.ein` program in the corpus reaches either branch — the pre-fork subset
/// check means a superset is never entered, so it never gets as far as being
/// offered here, and `a_learned_clause_stops_every_superset_before_it_forks`
/// is that half. This is the direct test of the guard that makes it true.
#[test]
fn emit_nogood_keeps_the_store_minimal_in_both_directions() {
    let mut terms = Terms::new();
    let c = clause(&mut terms, "R", &[("a", "b"), ("c", "d"), ("e", "f")]);
    let (pair, triple) = (&c[..2], &c[..3]);

    // Superset second: refused, store untouched, reported as subsumed.
    let kb = empty_kb();
    let buf = Buffer::new();
    let mut ev = Events::to(Box::new(buf.clone()), Level::Verbose);
    assert!(ein_infer::nogoods::emit_nogood(
        &kb, &terms, &mut ev, pair, 1
    ));
    assert!(
        !ein_infer::nogoods::emit_nogood(&kb, &terms, &mut ev, triple, 1),
        "a superset of a stored clause adds nothing"
    );
    assert_eq!(
        stored(&kb, &terms),
        the_pair(),
        "the refused clause must not have landed alongside the one that \
         refused it"
    );
    assert_eq!(
        nogood_events(&buf),
        vec![(true, false, 0), (false, true, -1)],
        "the second attempt is narrated as subsumed, not as a silent no-op"
    );

    // Subset second: accepted, and it evicts the superset it now covers.
    let kb = empty_kb();
    let buf = Buffer::new();
    let mut ev = Events::to(Box::new(buf.clone()), Level::Verbose);
    assert!(ein_infer::nogoods::emit_nogood(
        &kb, &terms, &mut ev, triple, 1
    ));
    assert!(ein_infer::nogoods::emit_nogood(
        &kb, &terms, &mut ev, pair, 1
    ));
    assert_eq!(
        stored(&kb, &terms),
        the_pair(),
        "the stronger clause replaces the weaker one rather than joining it"
    );
    assert_eq!(
        nogood_events(&buf),
        vec![(true, false, 0), (true, false, 1)],
        "the eviction count is on the event — one superset removed"
    );
}

/// **min-size-floor** — `emit_nogood` refuses anything shorter than the
/// `min_size` it was handed, and refuses it without side effects.
///
/// The floor is not decoration: `min_size` is what splits the two learners.
/// The set-indexed engine passes 1, because a layer-1 singleton death has to
/// prune inside its own layer, before `alive` is recomputed; the default 2
/// leaves size-1 clauses to the `(not h)` writeback instead. A floor that
/// merely *warned* and stored anyway would make the two indistinguishable.
#[test]
fn emit_nogood_refuses_a_clause_below_the_size_floor() {
    let mut terms = Terms::new();
    let c = clause(&mut terms, "R", &[("a", "b")]);
    let kb = empty_kb();
    let mut ev = Events::off();

    assert!(
        !ein_infer::nogoods::emit_nogood(&kb, &terms, &mut ev, &c, 2),
        "a singleton is below the default floor of 2"
    );
    assert!(
        stored(&kb, &terms).is_empty(),
        "a refused clause leaves the store untouched"
    );
    assert!(
        ein_infer::nogoods::emit_nogood(&kb, &terms, &mut ev, &c, 1),
        "the same clause lands at min_size 1 — the floor is the only thing \
         that rejected it"
    );
    assert_eq!(stored(&kb, &terms).len(), 1);
}

// ── 4) What the dumper sees ────────────────────────────────────────

/// **dumper-has-no-semantic-effect** — watching the search does not change it.
///
/// This is the licence for every other test in this file: they assert the
/// engine's behaviour *through* a dumper, and that is only sound if a dumper
/// is an observer. The counters are the sharp end — a hook that re-saturated a
/// fork to render it, or consumed an iterator the loop consumes again, would
/// leave the verdict intact and `saturate_count` different, so `MonotonicStats`
/// is compared whole rather than field by field.
///
/// It also carries the engine half of ein.py's
/// `test_dumper_out_dir_none_writes_no_files`: [`Recorder`] holds no output
/// directory, and the run still delivers the whole lifecycle vocabulary to it.
/// The other half — that such a dumper touches the filesystem nowhere — is a
/// property of `ein_render::dump`'s writer and has to be asserted there.
#[test]
fn attaching_a_dumper_changes_neither_the_verdict_nor_a_counter() {
    let rel = "examples/lattice/01_subset_pruned.ein";
    let o = SolveOptions::default();

    let (bare, terms_a) = run_file(rel, &o, &mut NoDumper);
    let mut rec = Recorder::default();
    let (watched, terms_b) = run_file(rel, &o, &mut rec);

    assert_eq!(bare.answer.as_str(), watched.answer.as_str());
    assert_eq!(models(&bare, &terms_a), models(&watched, &terms_b));
    assert_eq!(
        bare.stats, watched.stats,
        "a dumper must not cost the search a single entering or saturation"
    );
    // `proof_summary` needs `store_lattice` and `close` is the abort path's,
    // so five of the seven hooks are what an ordinary exhausted run delivers.
    let kinds = rec.kinds();
    for hook in [
        "root_initial",
        "layer_start",
        "entering",
        "layer_end",
        "summary",
    ] {
        assert!(
            kinds.contains(&hook),
            "a dumper with nowhere to write still gets every lifecycle hook, \
             and {hook} is missing from {kinds:?} — without this the equality \
             above would hold for a dumper that was never called"
        );
    }
}

/// **early-exit-still-summarises** — a run that returns mid-layer still tells
/// the observer how it ended.
///
/// `stop_after` returns from inside the candidate loop, which is precisely the
/// path where a summary hook is easiest to lose: the layer it stopped in never
/// reaches its `layer_end`, so a summary emitted at the end of the layer loop
/// would be emitted never. It comes from the single exit hook instead, which
/// is what makes `summary.json` land on this path — and the last `layer_start`
/// having no `layer_end` after it is how you can tell the run really did cut
/// out mid-layer rather than finishing early by luck.
#[test]
fn a_run_that_stops_mid_layer_still_reaches_its_summary() {
    let mut rec = Recorder::default();
    let o = SolveOptions {
        stop_after: Some(1),
        max_set_size: 3,
        ..SolveOptions::default()
    };
    let (solved, _) = run_file("examples/branching/05_mini_zebra.ein", &o, &mut rec);

    assert_eq!(solved.answer.as_str(), "Solution");
    assert_eq!(solved.stats.solution_nodes, 1);
    assert!(
        !solved.stats.exhausted,
        "`stop_after` returns a model without certifying it is the only one"
    );

    let kinds = rec.kinds();
    assert_eq!(kinds.first(), Some(&"root_initial"));
    assert_eq!(
        kinds.last(),
        Some(&"summary"),
        "the summary is the last thing that happens, whatever the exit path"
    );
    assert!(
        rec.enterings().any(|h| matches!(
            h,
            Hook::Entering { outcome, .. } if outcome == "solution"
        )),
        "the terminating node's entering is recorded before the return"
    );
    let last_start = kinds
        .iter()
        .rposition(|k| *k == "layer_start")
        .expect("the search entered a layer");
    assert!(
        !kinds[last_start..].contains(&"layer_end"),
        "the layer the run stopped in must have no layer_end: {kinds:?}"
    );
}

/// **dead-record-is-honest-with-nogoods-off** — with no-good learning
/// disabled, a dead entering reports neither an emitted clause nor a subsumed
/// one.
///
/// `nogood_subsumed` is not `!nogood_emitted`. With learning off nothing is
/// *attempted*, so "subsumed" — which means "we already knew this" — would be
/// a claim about a clause that was never offered. The Python original
/// (divergence D-R5-1) is a regression test for the sharper version of the
/// same bug: the flag was assigned inside the `if enable_path_nogoods` branch
/// and read outside it, so the two features were fine apart and crashed
/// together. The second half of this test is the discriminator: with learning
/// on, the very same dead entering does report an emitted clause, so a
/// hard-coded `false` would not pass.
#[test]
fn a_dead_entering_with_nogoods_off_claims_neither_emitted_nor_subsumed() {
    let off_cfg = SolverConfig {
        enable_path_nogoods: false,
        ..no_lookahead()
    };
    let mut rec = Recorder::default();
    let (solved, _) = run(
        SINGLETON,
        &SolveOptions {
            config: Some(off_cfg),
            ..opts(2)
        },
        &mut rec,
    );
    assert_eq!(solved.stats.base.nogoods_emitted, 0);
    assert_eq!(solved.stats.base.nogoods_subsumed, 0);

    let dead: Vec<&Hook> = rec
        .enterings()
        .filter(|h| matches!(h, Hook::Entering { outcome, .. } if outcome.starts_with("dead")))
        .collect();
    assert!(
        !dead.is_empty(),
        "the fixture must produce a dead entering for this to mean anything"
    );
    for h in dead {
        let Hook::Entering {
            nogood_emitted,
            nogood_subsumed,
            ..
        } = h
        else {
            unreachable!()
        };
        assert!(!nogood_emitted, "nothing was attempted, so nothing landed");
        assert!(
            !nogood_subsumed,
            "nothing was attempted, so nothing was already known either"
        );
    }

    // The same fixture with learning on: the dead entering does claim a clause.
    let mut rec_on = Recorder::default();
    run(SINGLETON, &opts(2), &mut rec_on);
    assert!(
        rec_on.enterings().any(|h| matches!(
            h,
            Hook::Entering { outcome, nogood_emitted, .. }
                if outcome.starts_with("dead") && *nogood_emitted
        )),
        "with learning on the same death emits — otherwise the assertions \
         above hold for a fixture that never learns anything"
    );
}
