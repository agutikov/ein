//! The lattice search's own record — T1a.10.2.2, the ported half of ten
//! Python files under `ein.py/tests/inference/lattice/`.
//!
//! `solve` answers a question; under `store_lattice` it also hands back a
//! [`LatticeProof`](ein_infer::solve::LatticeProof) — the models it found, the
//! commitments it refuted and why, the frontier a depth cap cut, and the
//! clauses it learned. Everything here is about that record: what it contains,
//! what it must be consistent with, and which of its parts are *invariants of
//! the puzzle* rather than artefacts of the traversal.
//!
//! | Python original | what it owned |
//! |---|---|
//! | `lattice/test_contradictions_backbone.py` | the refutation half: deads, cores, learned clauses |
//! | `lattice/test_gaps_backbone.py` | the model half: how many, which, and where the root sits |
//! | `lattice/test_lattice_dumper.py` | what a dump tree does and does not contain |
//! | `lattice/test_lattice_fixtures.py` | the three `examples/lattice/` shapes, one claim each |
//! | `lattice/test_lattice_proof.py` | `alive_at_end` and `SolutionRecord` |
//! | `lattice/test_lattice_sanity.py` | `-y`, the saturation-commutativity check |
//! | `lattice/test_lattice_scoring.py` | `score-sum` and the popularity weights |
//! | `lattice/test_lattice_skeleton.py` | the degenerate KB |
//! | `lattice/test_p16_contract.py` | the P1.6 handoff contract, clause by clause |
//! | `lattice/test_shuffle_invariance.py` | 303 parametrised tests over two claims |
//!
//! **What did not come across, and why it is not a loss.** Most of what the
//! Python suite asserted about this record was the *Python* record:
//! `_record_setnode`'s MERGE path, `SetNode`'s multilabel invariant,
//! `LatticeStats.state_key_merges`, `proof.kb_index`, the `(verdict, stats)`
//! return tuple and half a dozen `isinstance` checks. ein.rs does not build the
//! per-`SetNode` DAG at all — `kb_index` is empty and `state_key_merges` is 0
//! by construction — so a port of those would assert a constant. What the DAG
//! *stood for* is the solution-node dedup, and that is asserted here as an
//! answer: [`a_state_hash_collision_is_one_model_not_two`] and
//! [`two_orientations_of_one_puzzle_collapse_to_a_single_model`].
//!
//! Two tests are **stronger than their originals**. The Python
//! commutativity-violation test monkeypatched `state_key` to return a fresh
//! integer per call, because "the spec's contrived violation fixture can't be
//! written within the IR's declarative semantics". It can now:
//! [`NON_MONOTONE`] is eleven lines of ein-lang and the violation is real —
//! see [`a_commutativity_violation_names_the_commitment_and_both_keys`]. And
//! the shuffle-invariance pair compares the *whole refutation half* of the
//! record rather than the model set alone.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, SolverConfig, Terms, Value};
use ein_infer::apriori::order_candidates;
use ein_infer::canon::state_key;
use ein_infer::commitment::Kind;
use ein_infer::events::sexpr;
use ein_infer::hypgen::{is_solution_node, score_hypothesis};
use ein_infer::sanity::check_commutativity;
use ein_infer::solve::{
    Dumper, EnteringInfo, LatticeProof, MonotonicStats, NoDumper, OnBudget, SolveError,
    SolveOptions, Solved, solve,
};
use ein_infer::verdict::{Answer, Verdict, goal_bindings};
use ein_infer::{Events, SharedMemo, Session};
use ein_ir::{Ast, load_file, parse};
use ein_render::dump::LatticeDumper;

// ── Scaffolding ────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// One solve, with the proof on and the caller's knobs applied to the file's
/// own `(config …)` block — which is how the CLI composes them too.
///
/// `on_budget: Verdict` rather than `Raise`: a corpus sweep must be able to
/// report "this one ran out" as an answer instead of ending the test.
struct Run {
    ast: Ast,
    terms: Terms,
    kb: Kb,
    solved: Solved,
}

impl Run {
    fn proof(&self) -> &LatticeProof {
        self.solved
            .proof
            .as_ref()
            .expect("store_lattice was on, so a proof is attached")
    }

    fn stats(&self) -> &MonotonicStats {
        &self.solved.stats
    }

    /// The verdict's own name — `Solution` / `Ambiguity` / `Contradiction` /
    /// `Aborted`.
    fn answer(&self) -> &'static str {
        self.solved.answer.as_str()
    }

    /// A commitment as sorted text, comparable across two `Terms` arenas where
    /// a `FactId` is not.
    fn set(&self, ids: &[FactId]) -> Vec<String> {
        let mut v: Vec<String> = ids.iter().map(|&f| sexpr(&self.terms, f)).collect();
        v.sort();
        v
    }

    fn sets(&self, sets: &[Vec<FactId>]) -> BTreeSet<Vec<String>> {
        sets.iter().map(|c| self.set(c)).collect()
    }

    /// The post-saturation states of the refuted commitments — the shape the
    /// shuffle-invariance claim is about.
    fn dead_states(&self) -> BTreeSet<Vec<String>> {
        self.proof()
            .dead_commitments
            .iter()
            .map(|d| self.set(&d.state_key))
            .collect()
    }

    fn root_state(&self) -> Vec<String> {
        self.set(&state_key(&self.kb))
    }

    /// The deads whose commitment is non-empty — every one except the Phase-1
    /// root dead, which is the record of root's own contradiction.
    fn branch_deads(&self) -> Vec<&ein_infer::solve::DeadCommitment> {
        self.proof()
            .dead_commitments
            .iter()
            .filter(|d| !d.commitment.is_empty())
            .collect()
    }

    fn root_deads(&self) -> usize {
        self.proof()
            .dead_commitments
            .iter()
            .filter(|d| d.commitment.is_empty())
            .count()
    }
}

fn options(max_set_size: u32, config: Option<SolverConfig>) -> SolveOptions {
    SolveOptions {
        stop_after: None,
        max_set_size,
        store_lattice: true,
        config,
        on_budget: OnBudget::Verdict,
        max_enterings: Some(4_000),
        ..SolveOptions::default()
    }
}

fn solve_kb(ast: Ast, mut terms: Terms, mut kb: Kb, opts: &SolveOptions) -> Run {
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, opts)
        .expect("the fixture solves");
    Run {
        ast,
        terms,
        kb,
        solved,
    }
}

/// Solve a corpus file at `rel`, with `f` applied to its configuration.
fn run_file(rel: &str, max_set_size: u32, f: impl FnOnce(&mut SolverConfig)) -> Run {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("the fixture loads");
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    f(&mut cfg);
    let opts = options(max_set_size, Some(cfg));
    solve_kb(ast, terms, kb, &opts)
}

/// Solve a corpus file with its own configuration untouched.
fn run(rel: &str, max_set_size: u32) -> Run {
    run_file(rel, max_set_size, |_| {})
}

/// Solve an inline program — no base directory, so no imports.
fn run_src(src: &str, max_set_size: u32) -> Run {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<fixture>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    let opts = options(max_set_size, kb.program().config.clone());
    solve_kb(ast, terms, kb, &opts)
}

/// Intern `(rel X)` — a unary proposition about one object, which is the shape
/// every hypothesis fixture here guesses over.
fn unary(terms: &mut Terms, rel: &str, arg: &str) -> FactId {
    let r = terms.intern_text(rel).expect("room");
    let a = terms.intern_text(arg).expect("room");
    terms.intern_fact(r, &[Value::sym(a)]).expect("room")
}

/// What one corpus sweep did — and, as importantly, what it did **not** do.
///
/// [S1a.10.1](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#2-the-finding--46--of-einrss-own-integration-tests-are-differential)
/// found 41 tests passing on a `SKIP` line nobody read. A sweep that dropped a
/// file on the floor would be that finding repeated inside its own repair, so
/// every file lands in exactly one of these four buckets and each caller
/// asserts all four.
#[derive(Debug, Default, PartialEq, Eq)]
struct Census {
    /// Solved with a proof — the files the caller's predicate actually saw.
    checked: usize,
    /// `examples/broken/`'s load-negative corpus: the loader refused.
    unloadable: usize,
    /// A rule the compiler refuses — `examples/broken/compile/` and the two
    /// `ein-bugs` fixtures. `solve` returns `Err`, so there is no proof.
    uncompilable: usize,
    /// The entering budget cut before the lattice was exhausted, so `solve`
    /// answered `Aborted` and attached no proof.
    over_budget: usize,
}

/// Solve every corpus file with the proof on, and hand each proof to `f`.
///
/// The budget is 1 000 enterings: enough that the whole corpus is 1.2 s and
/// only the seven genuinely large puzzles are cut, and low enough that a
/// pathological regression fails the floor rather than the CI wall clock.
fn each_solved_corpus_file(mut f: impl FnMut(&Run)) -> Census {
    let mut census = Census::default();
    for path in ein_oracle::corpus_files() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(mut kb) = load_file(&mut ast, &mut terms, &path) else {
            census.unloadable += 1;
            continue;
        };
        let opts = SolveOptions {
            stop_after: None,
            max_set_size: 3,
            store_lattice: true,
            config: kb.program().config.clone(),
            on_budget: OnBudget::Verdict,
            max_enterings: Some(1_000),
            ..SolveOptions::default()
        };
        let mut events = Events::off();
        let Ok(solved) = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) else {
            census.uncompilable += 1;
            continue;
        };
        if solved.proof.is_none() {
            census.over_budget += 1;
            continue;
        }
        let run = Run {
            ast,
            terms,
            kb,
            solved,
        };
        f(&run);
        census.checked += 1;
    }
    census
}

/// What [`each_solved_corpus_file`] must find, so that a shrinking sweep fails
/// rather than passing on fewer files.
fn assert_census(c: &Census) {
    assert!(c.checked >= 55, "the sweep reached only {} files", c.checked);
    assert!(
        c.uncompilable <= 6,
        "{} corpus files no longer compile",
        c.uncompilable
    );
    assert!(
        c.over_budget <= 20,
        "{} files ran out of enterings — the search got slower or the corpus grew",
        c.over_budget
    );
    assert_eq!(
        c.checked + c.unloadable + c.uncompilable + c.over_budget,
        ein_oracle::corpus_files().len(),
        "a corpus file fell out of every bucket"
    );
}

// ── The refutation half — `test_contradictions_backbone.py` ────────

/// **dead-records-match-the-dead-counters.** Every death is counted once, and
/// the one death that is not counted at all is the root's.
///
/// `enterings_dead_pre + enterings_dead_post` is what the `--stats` block
/// prints; `proof.dead_commitments` is what an explanation reads. They are two
/// views of the same events and a drift between them is invisible in either
/// alone. The exception is structural rather than an off-by-one: a Phase-1
/// root contradiction is appended to `dead_commitments` with an empty
/// commitment *without* bumping a counter, because no commitment was entered
/// — the puzzle was already refuted.
#[test]
fn the_dead_records_and_the_dead_counters_agree() {
    for (rel, m) in [
        ("examples/branching/04_two_levels.ein", 3),
        ("examples/lattice/01_subset_pruned.ein", 3),
        ("examples/ein-bugs/zebra2-bad.ein", 3),
    ] {
        let r = run(rel, m);
        let counted = r.stats().base.enterings_dead_pre + r.stats().base.enterings_dead_post;
        assert_eq!(
            counted as usize,
            r.proof().dead_commitments.len() - r.root_deads(),
            "{rel}: {counted} counted deaths vs {} records ({} of them root)",
            r.proof().dead_commitments.len(),
            r.root_deads(),
        );
    }
}

/// **unsat-core-is-the-union-of-the-dead-cores.** A `Contradiction`'s core is
/// assembled, not observed.
///
/// `finalise` builds the verdict's core by unioning every dead record's, so
/// the claim is that nothing is dropped on the way and nothing is invented:
/// the verdict cannot blame a fact no refutation blamed. It is the sharper
/// half of the pair — a core that grew would name an innocent premise in every
/// explanation downstream.
#[test]
fn the_verdict_core_is_the_union_of_the_dead_cores() {
    for (rel, m) in [
        ("examples/ein-bugs/zebra2-bad.ein", 3),
        ("examples/branching/04_two_levels.ein", 1),
    ] {
        let r = run(rel, m);
        let Answer::Verdict(Verdict::Contradiction { unsat_core }) = &r.solved.answer else {
            panic!("{rel} is supposed to be a Contradiction, not {}", r.answer());
        };
        let union: BTreeSet<String> = r
            .proof()
            .dead_commitments
            .iter()
            .flat_map(|d| d.unsat_core.iter())
            .map(|&f| sexpr(&r.terms, f))
            .collect();
        let core: BTreeSet<String> = unsat_core.iter().map(|&f| sexpr(&r.terms, f)).collect();
        assert_eq!(core, union, "{rel}: the verdict's core is not the union");
        if !r.proof().dead_commitments.is_empty() {
            assert!(!core.is_empty(), "{rel}: a refuted puzzle blames nothing");
        }
    }
}

/// **a-root-contradiction-records-one-root-dead.** A puzzle that refutes
/// itself before any hypothesis still leaves a record, and the record says so.
///
/// `k = 0` and `enterings_total = 0` are the observable difference between
/// "no model within the search" and "no model at all": nothing was tried. The
/// single dead carries the empty commitment, layer 0, an empty learned clause
/// — there is nothing to learn from, no hypothesis was involved — and a core
/// that is the *source frontier*, so it names the stated fact rather than the
/// derived `(false)`.
#[test]
fn a_root_contradiction_records_exactly_one_root_dead() {
    let r = run_src(
        "(relation p T T)
         (rule boom () :match (p ?a ?b) :assert (false) :why \"boom\")
         (p a b :source \"(1)\")",
        3,
    );
    assert_eq!(r.answer(), "Contradiction");
    assert_eq!(r.stats().solution_nodes, 0, "k");
    assert_eq!(r.stats().base.enterings_total, 0, "nothing was entered");
    assert_eq!(r.proof().dead_commitments.len(), 1);
    let d = &r.proof().dead_commitments[0];
    assert!(d.commitment.is_empty(), "the root dead commits to nothing");
    assert_eq!(d.layer, 0);
    assert!(d.learned_clause.is_empty(), "and learns nothing");
    assert_eq!(
        r.set(&d.unsat_core),
        ["(p a b)"],
        "the core is the source frontier, not the derived (false)"
    );
    assert_eq!(r.root_deads(), 1);
}

/// **a-satisfiable-puzzle-still-carries-its-refutations.** The proof is not
/// only about the answer.
///
/// A puzzle with two models still refuted sixteen commitments getting there,
/// and each of those refutations is a well-formed claim in its own right: a
/// kind, a non-empty core, a learned clause, and a layer at or below the cap.
/// This is what makes the record usable for "why not X" as well as "why Y" —
/// [idea 08](../../../../plans/ideas/08-human-style-deductive-trace.md)'s
/// reductio steps are read straight off it.
#[test]
fn a_satisfiable_puzzle_still_carries_its_refutations() {
    let r = run("examples/branching/04_two_levels.ein", 3);
    assert_eq!(r.answer(), "Ambiguity");
    assert!(
        r.proof().dead_commitments.len() >= 8,
        "only {} refutations on a puzzle that had to prune",
        r.proof().dead_commitments.len()
    );
    for d in r.branch_deads() {
        assert!(
            matches!(d.kind, Kind::DeadPre | Kind::DeadPost),
            "a live commitment is in the dead list"
        );
        assert!(!d.unsat_core.is_empty(), "a refutation that blames nothing");
        assert!(!d.learned_clause.is_empty(), "and teaches nothing");
        assert!(d.layer >= 1, "a branch death cannot be at root's layer");
        assert!(d.layer <= 3, "beyond the depth cap");
    }
}

/// **learned-nogoods-mirror-the-store-at-return.** The proof's clause list is
/// root's no-good store, read at termination.
///
/// The two could drift in either direction — a clause learned and not
/// recorded, or a record of a clause the store never accepted — and both would
/// make the proof unusable as a resumption point. `nogoods_emitted >= 1`
/// whenever anything died is the non-vacuity half: a store that stopped
/// learning would satisfy the mirror trivially.
#[test]
fn the_learned_nogoods_mirror_roots_store_at_return() {
    let r = run("examples/branching/04_two_levels.ein", 3);
    let recorded: BTreeSet<Vec<String>> = r
        .proof()
        .learned_nogoods
        .iter()
        .map(|c| r.set(c))
        .collect();
    let store: BTreeSet<Vec<String>> = r
        .kb
        .nogoods()
        .read()
        .expect("the no-good store")
        .iter()
        .map(|c| r.set(c))
        .collect();
    assert_eq!(recorded, store, "the proof and the store disagree");
    assert!(!r.proof().dead_commitments.is_empty());
    assert!(
        r.stats().base.nogoods_emitted >= 1,
        "commitments died and nothing was learned"
    );
}

// ── The model half — `test_gaps_backbone.py` ───────────────────────

/// **branching-04-has-two-models-and-they-bind-blue-and-green.** The
/// canonical ambiguity, named.
///
/// The fixture's own header calls this out: Blue and Green are free to swap
/// between H2 and H3 and nothing distinguishes them, so `Ambiguity` is the
/// *correct* verdict and a `Solution` here would be a soundness bug rather
/// than a lucky tie-break. Asserting the bindings rather than the count is
/// what makes that difference visible — two models that both said "Blue"
/// would pass a count test.
#[test]
fn branching_04_has_two_models_and_they_bind_blue_and_green() {
    let mut r = run("examples/branching/04_two_levels.ein", 3);
    assert_eq!(r.answer(), "Ambiguity");
    assert_eq!(r.stats().solution_nodes, 2, "k");
    assert_eq!(r.proof().solutions.len(), 2);

    let Answer::Verdict(Verdict::Ambiguity(branches)) = &r.solved.answer else {
        unreachable!("checked above")
    };
    assert_eq!(branches.len(), 2, "the verdict carries both models");

    let ast = std::mem::replace(&mut r.ast, Ast::new());
    let mut bound: BTreeSet<String> = BTreeSet::new();
    let Answer::Verdict(Verdict::Ambiguity(branches)) = &r.solved.answer else {
        unreachable!()
    };
    for b in branches {
        let rows = goal_bindings(&ast, &mut r.terms, &b.kb, None);
        assert!(!rows.is_empty(), "a branch that does not answer the goal");
        for row in rows {
            for (_, v) in row {
                bound.insert(r.terms.display(v));
            }
        }
    }
    assert_eq!(
        bound,
        ["Blue", "Green"].iter().map(|s| s.to_string()).collect(),
        "the two models bind the colour to Blue and to Green"
    );
    r.ast = ast;
}

/// **two-orientations-of-a-symmetric-commitment-collapse-to-one-model.** Six
/// alive commitments, one answer.
///
/// `05_mini_zebra` enters six commitments and refutes none of them, yet `k`
/// is 1: the survivors saturate to the *same* KB, and a solution node is
/// keyed by that state rather than by the path that reached it. Without the
/// dedup this puzzle reports an ambiguity between a model and itself, which is
/// the failure mode P1.21 R1 exists to prevent — and it is the reason `k` is
/// read off distinct states and not off `enterings_alive`.
#[test]
fn two_orientations_of_one_puzzle_collapse_to_a_single_model() {
    let r = run("examples/branching/05_mini_zebra.ein", 3);
    assert_eq!(r.answer(), "Solution");
    assert_eq!(r.stats().solution_nodes, 1, "k");
    assert!(
        r.stats().base.enterings_alive >= 2,
        "only {} alive commitments — the dedup has nothing to do",
        r.stats().base.enterings_alive
    );
    assert!(r.proof().dead_commitments.is_empty(), "nothing was refuted");
    let states: BTreeSet<Vec<String>> = r
        .proof()
        .solutions
        .iter()
        .map(|s| r.set(&state_key(&s.kb)))
        .collect();
    assert_eq!(states.len(), 1, "one distinct model state");
}

/// **root-is-a-solution-node-when-nothing-is-open.** A puzzle can be solved
/// before the search starts.
///
/// A fork is recorded on `complete ∧ consistent` — no *open* hypothesis —
/// rather than on matching the goal, and root satisfies that as readily as any
/// fork does. `enterings_total = 0` with a layer-0 record whose commitment is
/// empty is the whole claim: saturation answered it, and the lattice was never
/// entered.
#[test]
fn the_root_is_a_solution_node_when_nothing_is_open() {
    let r = run("examples/branching/01_saturate_only.ein", 3);
    assert_eq!(r.answer(), "Solution");
    assert_eq!(r.stats().solution_nodes, 1, "k");
    assert_eq!(r.stats().base.enterings_total, 0, "nothing was entered");
    assert!(r.stats().exhausted, "and the lattice was exhausted, trivially");
    assert_eq!(r.proof().solutions.len(), 1);
    let rec = &r.proof().solutions[0];
    assert!(rec.commitment.is_empty(), "root commits to nothing");
    assert_eq!(rec.layer, 0);
}

/// **lattice-stats-solutions-found-tracks-the-proof**, on the whole corpus.
///
/// Two counters and one list, checked against each other on every file that
/// solves: `solutions_found` is the proof's own copy of `k`, and a layer must
/// have been explored if any commitment was entered. Neither can be checked on
/// a fixture — a fixture pins the number, and what is worth pinning is the
/// relation.
#[test]
fn the_proofs_counters_track_its_contents_on_every_corpus_file() {
    let checked = each_solved_corpus_file(|r| {
        let p = r.proof();
        assert_eq!(
            p.stats.solutions_found as usize,
            p.solutions.len(),
            "solutions_found disagrees with the record"
        );
        assert_eq!(
            p.stats.solutions_found, r.stats().solution_nodes,
            "the proof's k disagrees with the run's"
        );
        if r.stats().base.enterings_total >= 1 {
            assert!(
                r.stats().base.layers_explored >= 1,
                "commitments were entered outside any layer"
            );
        }
    });
    assert_census(&checked);
}

// ── The dump tree — `test_lattice_dumper.py` ───────────────────────

/// Every path under `dir`, relative and sorted.
fn tree(dir: &Path) -> Vec<String> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else {
            return;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                walk(&p, out);
            } else {
                out.push(p);
            }
        }
    }
    let mut paths = Vec::new();
    walk(dir, &mut paths);
    let mut out: Vec<String> = paths
        .iter()
        .map(|p| {
            p.strip_prefix(dir)
                .unwrap_or(p)
                .display()
                .to_string()
                .replace('\\', "/")
        })
        .collect();
    out.sort();
    out
}

/// A directory the test owns and removes.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let dir = std::env::temp_dir().join(format!(
            "ein-lattice-semantics-{}-{tag}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        Scratch(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The outcome histogram of a dump tree — `enterings/**/outcome.txt`.
fn outcomes(dir: &Path) -> BTreeMap<String, usize> {
    let mut by = BTreeMap::new();
    for rel in tree(&dir.join("enterings")) {
        if !rel.ends_with("outcome.txt") {
            continue;
        }
        let text = std::fs::read_to_string(dir.join("enterings").join(&rel)).expect("outcome.txt");
        *by.entry(text.trim().to_string()).or_insert(0) += 1;
    }
    by
}

/// Solve `branching/04` into `dir`, with the proof on or off.
fn dump_into(dir: &Path, store_lattice: bool) -> bool {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(
        &mut ast,
        &mut terms,
        &repo_root().join("examples/branching/04_two_levels.ein"),
    )
    .expect("the fixture loads");
    let mut dumper = LatticeDumper::new(Some(dir)).expect("the dump tree opens");
    let opts = SolveOptions {
        store_lattice,
        ..options(3, None)
    };
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut dumper, &opts)
        .expect("the fixture solves");
    dumper.close();
    solved.proof.is_some()
}

/// **a-dumper-without-a-proof-writes-no-proof-summary.** The two halves of a
/// dump tree have different sources, and only one of them needs the proof.
///
/// `--dump-states` writes from the live `Dumper` callbacks, which fire whether
/// or not a proof is being kept; `proof_summary.json` is written from the
/// proof itself, and the CLI only turns `store_lattice` on for `--trace`. So a
/// `--dump-states` run without `--trace` produces every `enterings/` folder
/// and no summary — and `kb_index/` appears in neither, because `solve` does
/// not build the per-`SetNode` DAG the folder would index.
#[test]
fn a_dump_without_a_proof_still_writes_every_entering() {
    let with = Scratch::new("with-proof");
    let without = Scratch::new("no-proof");
    assert!(dump_into(&with.0, true), "store_lattice attaches a proof");
    assert!(!dump_into(&without.0, false), "and off it attaches none");

    assert_eq!(
        outcomes(&with.0),
        outcomes(&without.0),
        "the live callbacks do not depend on the proof"
    );
    assert!(
        outcomes(&with.0).values().sum::<usize>() >= 30,
        "the sweep wrote almost nothing: {:?}",
        outcomes(&with.0)
    );

    let top = |s: &Scratch| -> Vec<String> {
        tree(&s.0)
            .into_iter()
            .filter(|p| !p.contains('/'))
            .collect()
    };
    assert!(
        top(&with).contains(&"proof_summary.json".to_string()),
        "no proof summary under store_lattice: {:?}",
        top(&with)
    );
    assert!(
        !top(&without).contains(&"proof_summary.json".to_string()),
        "a proof summary without a proof: {:?}",
        top(&without)
    );
    for s in [&with, &without] {
        assert!(
            !s.0.join("kb_index").exists(),
            "a kb_index/ folder for a DAG solve does not build"
        );
    }
}

/// **a-dumper-with-no-out-dir-writes-nothing.** `out_dir = None` is a mute
/// dumper, not an absent one.
///
/// Every lifecycle hook still fires — which is what makes it usable as a base
/// for an in-memory consumer — and none of them touches the filesystem. The
/// counting wrapper is the falsifier for the first half: a dumper that
/// silently stopped receiving `entering` would satisfy "wrote nothing" for the
/// wrong reason. `root_saturating` is the progress hook and needs a root
/// saturation longer than 50 firings, so the fixture is a twelve-node
/// transitive closure rather than one of the branching demos.
#[test]
fn a_dumper_with_no_output_directory_writes_nothing_and_still_hears_everything() {
    let mut src = String::from(
        "(relation edge T T)
         (relation h T)
         (relation is-a T T)
         (rule transitive () :match (and (edge ?a ?b) (edge ?b ?c) (neq ?a ?c))
           :assert (edge ?a ?c) :why \"tr\")
         (hrule guess (?R ?T) :match (is-a ?x ?T) :assert (?R ?x) :why \"guess {?x}\")
         (is-a Item T) (is-a X Item) (is-a Y Item)
         (query :goal (h ?x) :hrules (guess (h Item)))\n",
    );
    for i in 1..12 {
        src.push_str(&format!("(edge n{i} n{} :source \"{i}\")\n", i + 1));
    }
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &src, Some("<chain>")).expect("the fixture parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");

    let dumper = LatticeDumper::new(None).expect("a dumper with nowhere to write");
    assert!(dumper.timeline.out_dir.is_none());
    let mut counting = Counting {
        inner: dumper,
        hooks: BTreeMap::new(),
    };
    let opts = options(3, None);
    let mut events = Events::off();
    let solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut events,
        &mut counting,
        &opts,
    )
    .expect("the fixture solves");
    counting.close();

    assert!(solved.proof.is_some(), "the verdict still carries its proof");
    let fired: BTreeSet<&str> = counting.hooks.keys().copied().collect();
    assert_eq!(
        fired,
        [
            "close",
            "entering",
            "layer_end",
            "layer_start",
            "proof_summary",
            "root_initial",
            "root_saturating",
            "summary",
        ]
        .into_iter()
        .collect::<BTreeSet<&str>>(),
        "a lifecycle hook did not fire: {:?}",
        counting.hooks
    );
}

/// A `Dumper` that records which hooks it was handed, then forwards.
struct Counting {
    inner: LatticeDumper,
    hooks: BTreeMap<&'static str, usize>,
}

impl Counting {
    fn bump(&mut self, hook: &'static str) {
        *self.hooks.entry(hook).or_insert(0) += 1;
    }
}

impl Dumper for Counting {
    fn root_saturating(&mut self, n: usize) {
        self.bump("root_saturating");
        self.inner.root_saturating(n);
    }
    fn root_initial(&mut self, kb: &Kb, terms: &Terms) {
        self.bump("root_initial");
        self.inner.root_initial(kb, terms);
    }
    fn layer_start(&mut self, layer: u32, kb: &Kb, terms: &Terms, alive: usize) {
        self.bump("layer_start");
        self.inner.layer_start(layer, kb, terms, alive);
    }
    fn entering(
        &mut self,
        layer: u32,
        commitment: &[FactId],
        terms: &Terms,
        outcome: &str,
        info: &EnteringInfo<'_>,
    ) {
        self.bump("entering");
        self.inner.entering(layer, commitment, terms, outcome, info);
    }
    fn layer_end(&mut self, layer: u32, kb: &Kb, terms: &Terms, alive: usize, next: usize) {
        self.bump("layer_end");
        self.inner.layer_end(layer, kb, terms, alive, next);
    }
    fn proof_summary(&mut self, proof: &LatticeProof, terms: &Terms) {
        self.bump("proof_summary");
        self.inner.proof_summary(proof, terms);
    }
    fn summary(&mut self, verdict: &Answer, stats: &MonotonicStats) {
        self.bump("summary");
        self.inner.summary(verdict, stats);
    }
    fn close(&mut self) {
        self.bump("close");
        self.inner.close();
    }
}

// ── The three `examples/lattice/` shapes ───────────────────────────

/// **fail-fast-fork-is-verdict-and-proof-neutral.** An optimisation that stops
/// a doomed saturation early changes what was computed, not what was
/// concluded.
///
/// S1.9.E23 cuts a fork's saturation at the firing that makes it
/// inconsistent. Everything a caller reads must be identical either way — the
/// verdict, every counter, the set of refuted commitments and every learned
/// clause — and exactly two things are allowed to differ: the dying fork's
/// firing *prefix*, and the `DeadCommitment.state_key` computed from the KB
/// that prefix produced. The second is the interesting one, and it is safe
/// only because the search never reads a dead's state key back; this test
/// asserts the difference is *real* on at least one fixture, so "neutral"
/// cannot be passing because the flag does nothing.
#[test]
fn the_fail_fast_fork_is_verdict_and_proof_neutral() {
    let mut keys_differed = false;
    for (rel, m) in [
        ("examples/lattice/01_subset_pruned.ein", 3),
        ("examples/lattice/02_genuine_3set_death.ein", 3),
        ("examples/lattice/03_state_hash_collision.ein", 2),
        ("examples/branching/04_two_levels.ein", 3),
    ] {
        let on = run_file(rel, m, |c| c.enable_fail_fast_fork = true);
        let off = run_file(rel, m, |c| c.enable_fail_fast_fork = false);

        assert_eq!(on.answer(), off.answer(), "{rel}: the verdict moved");
        assert_eq!(on.stats(), off.stats(), "{rel}: a counter moved");
        assert_eq!(
            on.sets(
                &on.proof()
                    .dead_commitments
                    .iter()
                    .map(|d| d.commitment.clone())
                    .collect::<Vec<_>>()
            ),
            off.sets(
                &off.proof()
                    .dead_commitments
                    .iter()
                    .map(|d| d.commitment.clone())
                    .collect::<Vec<_>>()
            ),
            "{rel}: a different set of commitments was refuted"
        );
        assert_eq!(
            on.sets(
                &on.proof()
                    .learned_nogoods
                    .iter()
                    .map(|c| c.to_vec())
                    .collect::<Vec<_>>()
            ),
            off.sets(
                &off.proof()
                    .learned_nogoods
                    .iter()
                    .map(|c| c.to_vec())
                    .collect::<Vec<_>>()
            ),
            "{rel}: a different clause was learned"
        );
        let keys = |r: &Run| -> Vec<Vec<String>> {
            r.proof()
                .dead_commitments
                .iter()
                .map(|d| r.set(&d.state_key))
                .collect()
        };
        keys_differed |= keys(&on) != keys(&off);
    }
    assert!(
        keys_differed,
        "no dead's state key moved — the flag is inert and neutrality is vacuous"
    );
}

/// **a-layer-that-yields-complete-models-stops-descending.** The 3-set that
/// dies is never entered, because the search had already finished.
///
/// `02_genuine_3set_death` is built so that its size-3 commitment is
/// contradictory while all three 2-subsets are alive — the apriori pruning
/// cannot see it coming. What stops the search is not pruning but completion:
/// every alive 2-subset is already `complete ∧ consistent`, so it is recorded
/// as a model and leaves the frontier, and layer 3 is generated from nothing.
/// `dead_commitments` being **empty** on a fixture named for a death is the
/// whole point.
#[test]
fn a_layer_that_yields_complete_models_stops_descending() {
    let r = run("examples/lattice/02_genuine_3set_death.ein", 3);
    assert_eq!(r.answer(), "Ambiguity");
    assert_eq!(r.stats().solution_nodes, 3, "k");
    assert!(
        r.proof().dead_commitments.is_empty(),
        "the 3-set was entered after all: {:?}",
        r.proof()
            .dead_commitments
            .iter()
            .map(|d| r.set(&d.commitment))
            .collect::<Vec<_>>()
    );
    for rec in &r.proof().solutions {
        assert_eq!(rec.commitment.len(), 2, "a model of another size");
        assert_eq!(rec.layer, 2);
    }
    assert!(r.stats().exhausted, "and the lattice was exhausted");
}

/// **a-state-hash-collision-is-one-model-not-two.** Two commitments, one
/// state, one answer.
///
/// `03_state_hash_collision`'s bridge rules make `{h2}` derive `h1` and `h3`,
/// so committing to `h2` alone is already a complete consistent model and the
/// search records it at layer 1. The fixture's name is about what would happen
/// *without* the dedup — `{h1,h2}` and `{h2,h3}` saturate to that same state —
/// and the observable is that `k` is 1 with nothing refuted. The depth cap
/// leaves `{h1,h3}` on the frontier, which is why the run is not exhausted.
#[test]
fn a_state_hash_collision_is_one_model_not_two() {
    let r = run("examples/lattice/03_state_hash_collision.ein", 2);
    assert_eq!(r.answer(), "Solution");
    assert_eq!(r.stats().solution_nodes, 1, "k");
    assert!(r.proof().dead_commitments.is_empty(), "nothing was refuted");
    assert_eq!(r.proof().solutions.len(), 1);
    let rec = &r.proof().solutions[0];
    assert_eq!(r.set(&rec.commitment), ["(h2 X)"], "the bridge hypothesis");
    assert_eq!(rec.layer, 1, "found before the pairs were ever generated");
    assert!(
        !rec.firings.is_empty(),
        "committing to h2 derived nothing, so it is not the bridge"
    );
}

// ── The proof's own fields — `test_lattice_proof.py` ───────────────

/// **alive-at-end-is-the-frontier-the-depth-cap-cut.** What is left over says
/// which terminator ended the search.
///
/// Two terminators, two shapes. When `max_set_size` cuts, the survivors are
/// exactly the commitments that were alive at that size and every entry has
/// that size — the frontier a deeper run would have continued from, and the
/// reason `exhausted` is false and `k` a lower bound. When the loop instead
/// runs out of alive commitments, `alive_at_end` is empty and `exhausted` is
/// true. Reading a `k` without reading these two is how "no model within the
/// cap" gets reported as "no model".
#[test]
fn alive_at_end_is_the_frontier_the_depth_cap_cut() {
    for cap in [1u32, 2] {
        let r = run("examples/branching/04_two_levels.ein", cap);
        assert!(!r.stats().exhausted, "cap {cap}: the cap did not cut");
        assert!(
            !r.proof().alive_at_end.is_empty(),
            "cap {cap}: the cap cut and left no frontier"
        );
        for c in &r.proof().alive_at_end {
            assert_eq!(
                c.len() as u32,
                cap,
                "cap {cap}: a survivor of another size — {:?}",
                r.set(c)
            );
        }
    }
    let r = run("examples/branching/04_two_levels.ein", 3);
    assert!(r.stats().exhausted, "the lattice was not exhausted");
    assert!(
        r.proof().alive_at_end.is_empty(),
        "an exhausted lattice left a frontier: {:?}",
        r.proof()
            .alive_at_end
            .iter()
            .map(|c| r.set(c))
            .collect::<Vec<_>>()
    );
}

/// **a-solution-record-carries-its-commitment-layer-and-firings.** A model is
/// reportable on its own.
///
/// The record is what an explanation is read off, and it has to stand without
/// the run that produced it: the commitment that reached the model, the layer
/// it was found at, the firings that derived it, and a KB **snapshot** rather
/// than a view of root. The snapshot is the part that would fail silently —
/// root keeps growing after the record is taken (singleton writebacks, forced
/// positives), and a shared KB would make every recorded model report the
/// state of the last one.
#[test]
fn a_solution_record_carries_its_commitment_layer_and_firings() {
    let mut r = run("examples/branching/04_two_levels.ein", 3);
    let root_facts = r.kb.n_facts();
    assert_eq!(r.proof().solutions.len(), 2);
    for i in 0..2 {
        let rec = &r.proof().solutions[i];
        assert!(!rec.commitment.is_empty(), "a model that committed to nothing");
        assert!(rec.layer >= 1, "a branch model at root's layer");
        assert!(!rec.firings.is_empty(), "a model that derived nothing");
        assert!(
            rec.kb.n_facts() > root_facts,
            "the model's KB is no larger than root's, so it is root's"
        );
    }
    // Independence: growing root does not move a record.
    let before: Vec<usize> = r.proof().solutions.iter().map(|s| s.kb.n_facts()).collect();
    let rel = r.terms.intern_text("late-arrival").expect("room");
    let arg = r.terms.value_text("x").expect("room");
    r.kb.add_and_index_fact(&mut r.terms, rel, &[arg], None)
        .expect("room");
    let after: Vec<usize> = r.proof().solutions.iter().map(|s| s.kb.n_facts()).collect();
    assert_eq!(before, after, "a snapshot grew with its source");
}

// ── `-y`, the commutativity check — `test_lattice_sanity.py` ───────

/// A program whose saturation does **not** commute with hypothesis order.
///
/// M1's rule set is monotone, which is why the Python original had to
/// monkeypatch `state_key` to reach this path at all. Negation as failure is
/// the exception: `(absent (p ?x))` is judged against the world as it stands,
/// so committing `{q}` first derives `(r X)` — nothing had said `p` yet — and
/// adding `p` afterwards cannot retract it. Committing to both at once judges
/// the guard against a world that already holds `p`, and `(r X)` is never
/// derived. Two paths to `{p, q}`, two different KBs, and the check is exactly
/// the instrument that notices.
const NON_MONOTONE: &str = "\
(relation p T)
(relation q T)
(relation r T)
(relation is-a T T)
(rule r-when-no-p ()
  :match  (and (q ?x) (absent (p ?x)))
  :assert (r ?x)
  :why    \"no p for {?x}, so r\")
(hrule guess (?R ?T)
  :match  (is-a ?x ?T)
  :assert (?R ?x)
  :why    \"guess: is {?x} {?R}?\")
(is-a Item T)
(is-a X Item)
(query
  :goal (r ?x)
  :hrules (
    guess (p Item)
    guess (q Item)))
";

/// Load an inline program and hand back the pieces `check_commutativity`
/// needs, which are the same ones a `Session` needs.
fn checkable(src: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<sanity>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// `check_commutativity` on one commitment, with everything it borrows.
fn check(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    commitment: &[FactId],
) -> Option<ein_infer::sanity::SanityError> {
    let mut events = Events::off();
    check_commutativity(kb, terms, ast, &mut events, &SharedMemo::default(), commitment)
        .expect("the check saturates")
}

/// **the-sanity-check-passes-on-every-monotone-fixture.** The premise the
/// engine is built on, verified rather than assumed.
///
/// Every rule M1 ships is monotone, so every lattice path to a commitment must
/// saturate to the same KB — that is what makes the apriori traversal sound in
/// the first place. `-y` is the instrument that would notice otherwise, and
/// running it over the branching and lattice fixtures is the check that the
/// premise still holds. The verdict must also be unchanged: an instrument that
/// altered the answer would not be one.
#[test]
fn the_sanity_check_passes_on_every_monotone_fixture() {
    for (rel, m) in [
        ("examples/branching/04_two_levels.ein", 3),
        ("examples/lattice/02_genuine_3set_death.ein", 3),
        ("examples/lattice/03_state_hash_collision.ein", 2),
    ] {
        let off = run(rel, m);
        let on = run_file(rel, m, |c| c.lattice_sanity_check = true);
        assert_eq!(on.answer(), off.answer(), "{rel}: -y moved the verdict");
        assert_eq!(
            on.stats().solution_nodes,
            off.stats().solution_nodes,
            "{rel}: -y moved k"
        );
    }
}

/// **the-sanity-check-is-off-by-default**, and a `(config …)` block is the
/// other way to turn it on.
///
/// A `k+1`-saturations-per-commitment instrument is not something a shipping
/// solve should pay for by accident, and the default is the whole guarantee:
/// the check has **no counter**. It runs its saturations through
/// `try_commitment_set` and a fresh `Saturator` rather than through the run's
/// own accounting, so `saturate_count` is 1 whether or not it fired and a test
/// that asserted a cost would assert nothing. What is observable is the
/// *effect*, so that is what is asserted, on the one program in the tree that
/// the check has something to say about: by default it answers, with the flag
/// it fails, and a `(config :lattice-sanity-check true)` line in the file does
/// what the flag does — which is the claim, since `-y` and the block write to
/// the same field.
#[test]
fn the_sanity_check_is_off_until_a_flag_or_a_config_block_turns_it_on() {
    assert!(
        !SolverConfig::default().lattice_sanity_check,
        "the default turned the check on"
    );
    let (_, _, kb) = checkable(NON_MONOTONE);
    assert!(
        kb.program()
            .config
            .as_ref()
            .is_none_or(|c| !c.lattice_sanity_check),
        "the fixture turns the check on by itself, so the default proves nothing"
    );
    assert_eq!(run_src(NON_MONOTONE, 3).answer(), "Solution", "by default");

    // The `(config …)` block, which is what `-y` writes to.
    let configured = format!("(config :lattice-sanity-check true)\n{NON_MONOTONE}");
    let (ast, mut terms, mut kb) = checkable(&configured);
    assert!(
        kb.program()
            .config
            .as_ref()
            .expect("the block loaded")
            .lattice_sanity_check,
        "the block did not reach the config"
    );
    let mut events = Events::off();
    let opts = options(3, kb.program().config.clone());
    match solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) {
        Err(SolveError::Sanity(_)) => {}
        Err(other) => panic!("the block failed the solve for another reason: {other}"),
        Ok(s) => panic!("the block did not turn the check on: {}", s.answer.as_str()),
    }
}

/// **the-check-is-a-no-op-below-size-two.** A singleton has no parents, so
/// there is nothing for two paths to disagree about.
///
/// The guard is at the top of the function and it matters for cost rather than
/// for correctness: layer 1 is the widest layer of the search, and a check that
/// saturated there would double the price of every run for a claim that is
/// vacuously true.
#[test]
fn the_commutativity_check_is_a_no_op_below_size_two() {
    let (ast, mut terms, mut kb) = checkable(NON_MONOTONE);
    let p = unary(&mut terms, "p", "X");
    let q = unary(&mut terms, "q", "X");
    assert!(
        check(&ast, &mut terms, &mut kb, &[p]).is_none(),
        "a singleton was reported as a violation"
    );
    assert!(
        check(&ast, &mut terms, &mut kb, &[]).is_none(),
        "the empty commitment was reported as a violation"
    );
    // Non-vacuity: the same fixture *does* report the pair.
    let mut pair = vec![p, q];
    pair.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));
    assert!(
        check(&ast, &mut terms, &mut kb, &pair).is_some(),
        "the pair is supposed to violate, so the singleton skip proves nothing"
    );
}

/// **a-dead-or-unrecognised-direct-path-is-skipped-not-failed.** Absence of a
/// comparison is not evidence of a violation.
///
/// Two skips, for two different reasons. A `dead-pre` direct path has no
/// saturated fork to compare against — the contradiction is set-union fact
/// equality and is deterministic on its own — and a parent that is not alive
/// means the lattice path *through* it does not exist, so there is nothing to
/// commute with. And a commitment of propositions the KB has never seen is
/// neither: it saturates to nothing on every path, which is agreement.
#[test]
fn a_dead_or_unrecognised_path_is_skipped_rather_than_reported() {
    let (ast, mut terms, mut kb) = checkable(NON_MONOTONE);
    let a = unary(&mut terms, "nonexistent-rel", "nonsense");
    let b = unary(&mut terms, "other-fake", "nonsense");
    let mut fake = vec![a, b];
    fake.sort_by(|&x, &y| terms.cmp_fact_semantic(x, y));
    assert!(
        check(&ast, &mut terms, &mut kb, &fake).is_none(),
        "an unrecognised commitment was reported as a violation"
    );

    // A `dead-pre` direct path: a proposition and its own negation.
    let (ast, mut terms, mut kb) = checkable(
        "(relation s T)
         (relation t T)
         (not (s X) :source \"(1)\")",
    );
    let s = unary(&mut terms, "s", "X");
    let t = unary(&mut terms, "t", "X");
    let mut dead = vec![s, t];
    dead.sort_by(|&x, &y| terms.cmp_fact_semantic(x, y));
    assert!(
        check(&ast, &mut terms, &mut kb, &dead).is_none(),
        "a dead-pre direct path was reported as a violation"
    );
}

/// **a-commutativity-violation-names-both-state-keys**, and stops the solve.
///
/// The report has to be actionable, because a violation means the traversal
/// itself is unsound for that program: it names the commitment, the state key
/// the direct path reached, and *only* the parents that disagreed, each with
/// the key its path reached. A report that listed every parent would bury the
/// one that matters — here exactly one of the two does, because `{p}` then `q`
/// judges the guard against a world that already holds `p` and agrees with the
/// direct path, while `{q}` then `p` does not.
///
/// End to end, `-y` turns the violation into a `SolveError::Sanity`: the run
/// fails rather than returning an answer it cannot stand behind. Without the
/// flag the same puzzle answers `Solution` — which is the point of the check
/// being an instrument rather than a guard.
#[test]
fn a_commutativity_violation_names_the_commitment_and_both_keys() {
    let (ast, mut terms, mut kb) = checkable(NON_MONOTONE);
    let p = unary(&mut terms, "p", "X");
    let q = unary(&mut terms, "q", "X");
    let mut pair = vec![p, q];
    pair.sort_by(|&a, &b| terms.cmp_fact_semantic(a, b));

    let err = check(&ast, &mut terms, &mut kb, &pair).expect("the NAF fixture does not commute");
    assert_eq!(err.commitment, pair, "the report names another commitment");
    assert_eq!(
        err.parent_state_keys.len(),
        1,
        "both parents were blamed; only {{q}} disagrees"
    );
    let (parent, key) = &err.parent_state_keys[0];
    assert_eq!(
        parent.iter().map(|&f| sexpr(&terms, f)).collect::<Vec<_>>(),
        ["(q X)"],
        "the wrong parent was blamed"
    );
    assert_ne!(
        key.as_ref(),
        err.direct_state_key.as_ref(),
        "a parent was listed whose key matched"
    );
    let rendered = err.to_string();
    for want in [
        "Saturation commutativity violated",
        "direct state_key digest",
        "parent paths",
        "('q', ('X',))",
    ] {
        assert!(rendered.contains(want), "the message omits {want:?}:\n{rendered}");
    }

    // End to end.
    let (ast, mut terms, mut kb) = checkable(NON_MONOTONE);
    let mut cfg = kb.program().config.clone().unwrap_or_default();
    cfg.lattice_sanity_check = true;
    let mut events = Events::off();
    let outcome = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut events,
        &mut NoDumper,
        &options(3, Some(cfg)),
    );
    match outcome {
        Err(SolveError::Sanity(e)) => assert_eq!(e.parent_state_keys.len(), 1),
        Err(other) => panic!("-y failed for the wrong reason: {other}"),
        Ok(s) => panic!("-y returned {} instead of failing", s.answer.as_str()),
    }
    let without = run_src(NON_MONOTONE, 3);
    assert_eq!(
        without.answer(),
        "Solution",
        "without -y the same puzzle answers, which is what makes the check an instrument"
    );
}

// ── Ordering — `test_lattice_scoring.py` ───────────────────────────

/// **score-sum-picks-up-the-popularity-weights.** The informed ordering is
/// informed by something.
///
/// `score-sum` sums `score_hypothesis` over the set and sorts descending, so
/// under `hypgen_scoring = "popularity"` a candidate over a relation with more
/// facts sorts first — and that is a *different* order from `lex`, which is
/// the canonical tuple sort. Both halves are needed: determinism alone is
/// satisfied by a mode that silently fell back to `lex`, and a permutation
/// alone could be noise. The third arm is the reason the scorer reads the
/// KB's own `(config …)` block rather than the default: a KB with no block at
/// all scores 0.0 and collapses to `lex`, which ein.py does too and is
/// reproduced rather than tidied.
#[test]
fn score_sum_orders_by_popularity_and_lex_does_not() {
    let (_, mut terms, kb) = checkable(
        "(config :hypgen-scoring \"popularity\")
         (relation p T)
         (relation q T)
         (q A :source \"1\") (q B :source \"2\") (q C :source \"3\")
         (p D :source \"4\")",
    );
    let candidates: Vec<Vec<FactId>> = ["p", "q"]
        .iter()
        .map(|r| vec![unary(&mut terms, r, "X")])
        .collect();
    let show = |v: &[Vec<FactId>]| -> Vec<String> {
        v.iter().map(|c| sexpr(&terms, c[0])).collect()
    };
    let lex = order_candidates(&kb, &terms, &candidates, "lex").expect("lex orders");
    let scored = order_candidates(&kb, &terms, &candidates, "score-sum").expect("score-sum orders");
    assert_eq!(show(&lex), ["(p X)", "(q X)"], "lex is the tuple sort");
    assert_eq!(
        show(&scored),
        ["(q X)", "(p X)"],
        "the busier relation did not sort first"
    );
    assert_eq!(
        show(&order_candidates(&kb, &terms, &candidates, "score-sum").expect("again")),
        show(&scored),
        "the same KB gave two orders"
    );
    assert_eq!(
        score_hypothesis(&kb, &terms, candidates[0][0]).expect("scored"),
        1.0,
        "(p X): one p fact, and X is in no fact's arguments"
    );
    assert_eq!(
        score_hypothesis(&kb, &terms, candidates[1][0]).expect("scored"),
        3.0,
        "(q X): three q facts"
    );

    // No `(config …)` block at all — the neutral score, not the default's.
    let (_, mut terms, kb) = checkable("(relation q T)\n(q A :source \"1\") (q B :source \"2\")");
    let q = unary(&mut terms, "q", "X");
    assert_eq!(
        score_hypothesis(&kb, &terms, q).expect("scored"),
        0.0,
        "a KB with no config block must score neutrally"
    );
}

// ── The degenerate KB — `test_lattice_skeleton.py` ─────────────────

/// **an-empty-kb-is-a-trivial-solution.** Nothing to prove is not the same as
/// nothing proved.
///
/// A solution node is `complete ∧ consistent`, and the empty KB is both: no
/// open hypothesis, no contradiction. So `solve` answers `Solution` with `k =
/// 1` having entered nothing, and still attaches a proof under
/// `store_lattice`. The tempting alternative — a `Contradiction` because no
/// model was *found* — would be wrong in the way that matters: it would report
/// unsatisfiability for a program that is satisfied by everything.
#[test]
fn an_empty_kb_is_a_trivial_solution() {
    let r = run_src("", 3);
    assert_eq!(r.answer(), "Solution");
    assert_eq!(r.stats().solution_nodes, 1, "k");
    assert_eq!(r.stats().base.enterings_total, 0);
    assert!(r.stats().exhausted);
    assert_eq!(r.proof().solutions.len(), 1);
    assert!(r.proof().dead_commitments.is_empty());
    assert_eq!(r.kb.n_facts(), 0, "the fixture is supposed to be empty");
}

// ── The P1.6 handoff contract — `test_p16_contract.py` ─────────────

/// **every-solution-record-satisfies-the-goal**, under the reading `solve`
/// actually uses.
///
/// The contract's first clause is that a `SolutionRecord`'s KB is a solution
/// node *on its own* — re-checkable without the run, which is what makes the
/// record a handoff rather than a log line. The reading matters: `solve`
/// records `complete ∧ consistent`, not goal-matching, and the two genuinely
/// differ. On `lattice/02` all three models are solution nodes and only two of
/// them bind the query goal, so a contract written as "the goal matches" would
/// fail on a correct engine.
#[test]
fn every_solution_record_is_independently_a_solution_node() {
    for (rel, m) in [
        ("examples/branching/04_two_levels.ein", 3),
        ("examples/branching/05_mini_zebra.ein", 3),
        ("examples/branching/01_saturate_only.ein", 3),
        ("examples/lattice/02_genuine_3set_death.ein", 3),
    ] {
        let mut r = run(rel, m);
        let ast = std::mem::replace(&mut r.ast, Ast::new());
        let mut proof = r.solved.proof.take().expect("a proof");
        let n = proof.solutions.len();
        assert!(n >= 1, "{rel}: no models to check");
        let mut goal_bound = 0usize;
        for rec in proof.solutions.iter_mut() {
            let mut kb = rec.kb.snapshot();
            let mut events = Events::off();
            let ok = {
                let mut s = Session {
                    kb: &mut kb,
                    terms: &mut r.terms,
                    ast: &ast,
                    events: &mut events,
                    memo: SharedMemo::default(),
                };
                is_solution_node(&mut s).expect("the model re-checks")
            };
            assert!(ok, "{rel}: a recorded model is not a solution node");
            if !goal_bindings(&ast, &mut r.terms, &kb, None).is_empty() {
                goal_bound += 1;
            }
        }
        if rel.ends_with("02_genuine_3set_death.ein") {
            assert_eq!(
                (goal_bound, n),
                (2, 3),
                "the two readings stopped differing, so the clause is untested"
            );
        }
        r.solved.proof = Some(proof);
        r.ast = ast;
    }
}

/// **every-dead-record-is-well-formed**, with one named exception.
///
/// A refutation that blames nothing cannot be explained, and one that teaches
/// nothing cannot prune. So every dead with a commitment carries a non-empty
/// core and a learned clause that is exactly its commitment — the clause *is*
/// "not all of these together". The Phase-1 root dead is the exception and it
/// is structural rather than tolerated: it has no commitment to negate, so its
/// clause is empty, and its core is the source frontier of root's own
/// contradiction.
#[test]
fn every_dead_record_is_well_formed() {
    for (rel, m) in [
        ("examples/branching/04_two_levels.ein", 3),
        ("examples/lattice/01_subset_pruned.ein", 3),
        ("examples/ein-bugs/zebra2-bad.ein", 3),
    ] {
        let r = run(rel, m);
        for d in r.branch_deads() {
            assert!(!d.unsat_core.is_empty(), "{rel}: a core-less refutation");
            assert_eq!(
                r.set(&d.learned_clause),
                r.set(&d.commitment),
                "{rel}: the learned clause is not the commitment"
            );
        }
        for d in r
            .proof()
            .dead_commitments
            .iter()
            .filter(|d| d.commitment.is_empty())
        {
            assert!(d.learned_clause.is_empty(), "{rel}: the root dead learned");
            assert!(!d.unsat_core.is_empty(), "{rel}: the root dead blames nothing");
            assert_eq!(d.layer, 0);
        }
    }
}

/// **every-dead-clause-is-subsumed-by-a-learned-nogood.** The store covers
/// every refutation it was told about.
///
/// Subset rather than equality, and that is the interesting part: a clause may
/// be *subsumed* by a shorter one learned earlier, so what has to hold is that
/// some stored clause implies each recorded death — which is what makes the
/// store a sound summary of the whole refutation list. The root dead is exempt
/// because it emits no clause at all; encoding the exemption is the port's
/// job, and restoring an empty-clause entry to make the loop uniform would be
/// the wrong repair.
#[test]
fn every_dead_clause_is_subsumed_by_a_learned_nogood() {
    for (rel, m) in [
        ("examples/branching/04_two_levels.ein", 3),
        ("examples/lattice/01_subset_pruned.ein", 3),
    ] {
        let r = run(rel, m);
        assert!(!r.branch_deads().is_empty(), "{rel}: nothing died");
        for d in r.branch_deads() {
            let clause: BTreeSet<FactId> = d.learned_clause.iter().copied().collect();
            assert!(
                r.proof()
                    .learned_nogoods
                    .iter()
                    .any(|ng| ng.iter().all(|f| clause.contains(f))),
                "{rel}: no stored clause covers {:?}",
                r.set(&d.learned_clause)
            );
        }
    }
}

/// **proof-coherence-holds-on-every-corpus-file.** The clauses above, swept.
///
/// The fixtures pin the shapes; this pins that no corpus file violates them.
/// It is the counterpart of `ein-cli/tests/summary_properties.rs` with the
/// proof turned on: the same idea — bank the arithmetic, not the numbers —
/// applied to the record rather than to the counters, and the reason neither
/// of them is a golden.
#[test]
fn the_proof_is_coherent_on_every_corpus_file() {
    let checked = each_solved_corpus_file(|r| {
        let p = r.proof();
        let roots = r.root_deads();
        assert!(roots <= 1, "more than one root dead");
        assert_eq!(
            (r.stats().base.enterings_dead_pre + r.stats().base.enterings_dead_post) as usize,
            p.dead_commitments.len() - roots,
            "the dead counters and the dead records disagree"
        );
        assert_eq!(p.stats.solutions_found as usize, p.solutions.len());
        for d in p.dead_commitments.iter().filter(|d| !d.commitment.is_empty()) {
            assert!(!d.unsat_core.is_empty(), "a core-less refutation");
            assert_eq!(
                r.set(&d.learned_clause),
                r.set(&d.commitment),
                "the learned clause is not the commitment"
            );
            let clause: BTreeSet<FactId> = d.learned_clause.iter().copied().collect();
            assert!(
                p.learned_nogoods
                    .iter()
                    .any(|ng| ng.iter().all(|f| clause.contains(f))),
                "no stored clause covers {:?}",
                r.set(&d.learned_clause)
            );
        }
        for c in &p.alive_at_end {
            assert!(!c.is_empty(), "the empty commitment survived to the frontier");
        }
    });
    assert_census(&checked);
}

// ── Shuffle invariance — `test_shuffle_invariance.py` ──────────────

/// **the-refutation-half-of-the-snapshot-is-shuffle-invariant.** The traversal
/// order is free; the record is not.
///
/// `lattice_order_seed` permutes the within-layer candidate order, so a
/// different set of commitments is *tried first* on every seed. What must not
/// move is anything a caller reads: the verdict, `k`, the refuted states, the
/// surviving frontier, and root's own terminal state key — which carries the
/// singleton `(not h)` writebacks and the forced-positive promotions, and is
/// therefore the one place a first-arrival attribution could leak into the
/// answer. The Python original compared the model set alone; the refutation
/// half is where a nogood-subsumption order would show up.
#[test]
fn the_refutation_half_of_the_record_is_shuffle_invariant() {
    for rel in [
        "examples/branching/04_two_levels.ein",
        "examples/lattice/01_subset_pruned.ein",
    ] {
        let base = run(rel, 3);
        for seed in [1i64, 7, 42] {
            let shuffled = run_file(rel, 3, |c| c.lattice_order_seed = Some(seed));
            assert_eq!(shuffled.answer(), base.answer(), "{rel}/{seed}: the verdict");
            assert_eq!(
                shuffled.stats().solution_nodes,
                base.stats().solution_nodes,
                "{rel}/{seed}: k"
            );
            assert_eq!(
                shuffled.dead_states(),
                base.dead_states(),
                "{rel}/{seed}: a different set of states was refuted"
            );
            assert_eq!(
                shuffled.sets(&shuffled.proof().alive_at_end),
                base.sets(&base.proof().alive_at_end),
                "{rel}/{seed}: a different frontier survived"
            );
            assert_eq!(
                shuffled.root_state(),
                base.root_state(),
                "{rel}/{seed}: root ended in a different state"
            );
        }
    }
}

/// **shuffle-invariance-holds-under-a-depth-cap.** Including the one regime
/// that has a frontier to compare.
///
/// At `max_set_size = 1` the loop ends on the depth cap rather than on
/// frontier exhaustion, which is the only regime that populates
/// `alive_at_end` — so it is the only one where the previous test's frontier
/// comparison is not a comparison of two empty lists. Caps 2 and 3 are here
/// because the cap also decides *which* commitments exist to permute.
#[test]
fn shuffle_invariance_holds_under_a_depth_cap() {
    let rel = "examples/branching/04_two_levels.ein";
    let mut saw_a_frontier = false;
    for cap in [1u32, 2, 3] {
        let base = run(rel, cap);
        saw_a_frontier |= !base.proof().alive_at_end.is_empty();
        for seed in [1i64, 7, 42] {
            let shuffled = run_file(rel, cap, |c| c.lattice_order_seed = Some(seed));
            assert_eq!(shuffled.answer(), base.answer(), "cap {cap}, seed {seed}");
            assert_eq!(
                shuffled.stats().solution_nodes,
                base.stats().solution_nodes,
                "cap {cap}, seed {seed}: k"
            );
            assert_eq!(
                shuffled.dead_states(),
                base.dead_states(),
                "cap {cap}, seed {seed}: the refuted states"
            );
            assert_eq!(
                shuffled.sets(&shuffled.proof().alive_at_end),
                base.sets(&base.proof().alive_at_end),
                "cap {cap}, seed {seed}: the frontier"
            );
            assert_eq!(
                shuffled.root_state(),
                base.root_state(),
                "cap {cap}, seed {seed}: root's terminal state"
            );
        }
    }
    assert!(
        saw_a_frontier,
        "no cap left a frontier, so the frontier comparison is vacuous"
    );
}
