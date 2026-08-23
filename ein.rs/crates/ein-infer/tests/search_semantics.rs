//! The search layer's semantics — T1a.10.2.2, the ported half of ten Python
//! files.
//!
//! Everything above the saturator and below the CLI: the lattice's arithmetic
//! ([`ein_infer::apriori`]), the commitment primitive
//! ([`ein_infer::commitment`]), contradiction detection
//! ([`ein_infer::contradiction`]), the one-step lookahead
//! ([`ein_infer::lookahead`]), the minimal-explanation search
//! ([`ein_infer::explain`]), guided hypothesis generation
//! ([`ein_infer::hypgen`]), state identity ([`ein_infer::canon`]) and what a
//! solution node *is*.
//!
//! Replaces, under S1a.10.2:
//!
//! | Python original | subject |
//! |---|---|
//! | `tests/inference/test_apriori.py` | canonicalise / prefix-join / filter / `layer_1` |
//! | `tests/inference/test_commitment.py` | the alive / dead-pre / dead-post trichotomy, and fail-fast |
//! | `tests/inference/test_contradiction.py` | the two contradiction shapes and the incremental check |
//! | `tests/inference/test_demos.py` | `examples/saturation/`'s 24 per-rule demos |
//! | `tests/inference/test_dies_immediately.py` | the lookahead's kill conditions |
//! | `tests/inference/test_frontier.py` | the OR-aware smallest frontier (S1.21.7) |
//! | `tests/inference/test_guided_hypgen.py` | `:hypothesis-relations` / `:no-hypothesis` |
//! | `tests/inference/test_solution.py` | `consistent ∧ complete` |
//! | `tests/inference/test_state_key.py` | canonical model identity (P1.21 R1) |
//! | `tests/inference/test_typed_blind_solve.py` | the blind enumerator's solve |
//!
//! **What did not come across.** Several Python tests reached into the Python
//! object graph — `kb._alt_justifications`, `sol.generate_hypotheses`
//! monkeypatched to count what a generator was asked for, a `_Collide` tuple
//! subclass forcing every state key into one dict bucket. Those pin CPython,
//! not the language. Where the internal protected an observable the observable
//! is what is asserted here, and where it protected nothing the behaviour is
//! reported `not-portable` rather than given a test that cannot fail.

use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

use ein_core::{
    FactId, Justifications, Kb, ProvKind, SolverConfig, Symbol, Terms, Value, unsat_core,
};
use ein_infer::apriori::{
    apriori_prefix_join, canonicalise, filter_candidate, generate_layer, layer_1,
};
use ein_infer::canon::state_key;
use ein_infer::commitment::{Kind, try_commitment_set};
use ein_infer::contradiction::{Contradiction, contradicts, detect};
use ein_infer::explain::{
    ExplanationBudget, minimal_contradiction_frontier, smallest_contradiction_frontier,
};
use ein_infer::hypgen::{HypGenStats, Skip};
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_infer::{Events, Firing, Lookahead, Matcher, Saturator, Session, SharedMemo};
use ein_ir::{Ast, load_file, parse};
use rustc_hash::FxHashSet;

// ── Scaffolding ────────────────────────────────────────────────────

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// Parse + load an inline fixture.
fn kb_of(src: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<fixture>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// Run to the fixpoint, returning the firings in order.
fn saturate(ast: &Ast, terms: &mut Terms, kb: &mut Kb) -> Vec<Firing> {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("the fixture compiles");
    let mut out = Vec::new();
    sat.saturate(&mut s, None, &mut |f| out.push(f.clone()))
        .expect("the fixture saturates");
    out
}

/// Borrow a `Session` for the length of one call.
fn in_session<R>(
    ast: &Ast,
    terms: &mut Terms,
    kb: &mut Kb,
    f: impl FnOnce(&mut Session<'_>) -> R,
) -> R {
    let mut events = Events::off();
    let mut s = Session {
        kb,
        terms,
        ast,
        events: &mut events,
        memo: SharedMemo::default(),
    };
    f(&mut s)
}

/// Intern `(rel arg…)` — the proposition, whether or not any KB believes it.
fn fact(terms: &mut Terms, rel: &str, args: &[&str]) -> FactId {
    let r = terms.intern_text(rel).expect("room for a relation name");
    let mut vals = Vec::with_capacity(args.len());
    for a in args {
        vals.push(Value::sym(
            terms.intern_text(a).expect("room for an argument"),
        ));
    }
    terms.intern_fact(r, &vals).expect("room for a fact")
}

/// A KB's facts as sorted s-expressions — readable, and comparable across two
/// `Terms` arenas where a `FactId` is not.
fn sexprs(terms: &Terms, facts: impl IntoIterator<Item = FactId>) -> BTreeSet<String> {
    facts
        .into_iter()
        .map(|f| ein_infer::events::sexpr(terms, f))
        .collect()
}

/// The distinct relation names of a fact set, sorted.
fn rel_names(terms: &Terms, facts: &[FactId]) -> Vec<String> {
    let mut out: Vec<String> = facts
        .iter()
        .map(|&f| terms.sym(terms.facts.rel(f)).to_string())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// The rule that recorded a fact's primary derivation, if any.
fn primary_rule(kb: &Kb, terms: &Terms, f: FactId) -> Option<String> {
    let p = kb.primary(f)?;
    terms.provs.get(p).rule.map(|r| terms.sym(r).to_string())
}

fn primary_kind(kb: &Kb, terms: &Terms, f: FactId) -> Option<ProvKind> {
    kb.primary(f).map(|p| terms.provs.get(p).kind)
}

/// The union of every contradiction witness's premise closure, under one
/// reading of the OR-node — `_union_core` in the Python original, which took
/// the `Primary` reading by default.
fn union_core(kb: &Kb, terms: &Terms, how: Justifications) -> FxHashSet<FactId> {
    let w: Vec<FactId> = detect(kb, terms)
        .iter()
        .map(Contradiction::witness)
        .collect();
    unsat_core(kb, terms, &w, how).into_iter().collect()
}

// ── The lattice's arithmetic — `test_apriori.py` ───────────────────

/// Four propositions whose **content** order is the reverse of their
/// **interning** order, so every ordering assertion below separates the two.
///
/// ein.py sorts `(relation_name, args)` tuples; a `FactId` is interning order,
/// which is an artefact of what the loader happened to see first. A port that
/// sorted by id would pass a test built on ids and produce a different
/// traversal on every puzzle.
fn four_out_of_order() -> (Terms, [FactId; 4]) {
    let mut terms = Terms::new();
    let d = fact(&mut terms, "p", &["d"]);
    let c = fact(&mut terms, "p", &["c"]);
    let b = fact(&mut terms, "p", &["b"]);
    let a = fact(&mut terms, "p", &["a"]);
    assert!(d.0 < c.0 && c.0 < b.0 && b.0 < a.0, "interned in reverse");
    (terms, [a, b, c, d])
}

fn clause(mut ids: Vec<FactId>) -> Box<[FactId]> {
    // `Nogoods` stores a clause sorted by `FactId`, because `is_subset` is a
    // merge walk over that order and not a content one.
    ids.sort_unstable();
    ids.into_boxed_slice()
}

fn nogoods(clauses: &[Vec<FactId>]) -> ein_core::Nogoods {
    let mut ng = ein_core::Nogoods::default();
    for c in clauses {
        ng.insert(clause(c.clone()));
    }
    ng
}

/// `canonicalise` is a *function of the set*, not of the iterable: order in,
/// duplicates and all, cannot reach the answer, and a second application is a
/// no-op.
///
/// The node identity of the whole lattice is this tuple, so an idempotence
/// failure would not show up as a wrong answer — it would show up as the same
/// commitment being entered twice under two spellings, which the dedup would
/// never catch.
#[test]
fn canonicalise_sorts_dedups_and_is_idempotent() {
    let (terms, [a, b, c, d]) = four_out_of_order();
    let once = canonicalise(&terms, [c, a, d, b, a, c, d]);
    assert_eq!(once, vec![a, b, c, d], "sorted by content, deduped");
    assert_eq!(canonicalise(&terms, once.clone()), once, "idempotent");
    assert_eq!(
        canonicalise(&terms, [d, c, b, a]),
        once,
        "the input order does not reach the answer"
    );
    assert_ne!(
        once,
        vec![d, c, b, a],
        "an id-ordered canonicalise would have produced exactly this"
    );
}

/// The k = 1 → 2 step every real search takes first: a layer of singletons
/// joins into every unordered pair, each exactly once, in canonical order.
///
/// Worth its own test rather than folding into the triangle case, because the
/// singleton layer is the one where the shared prefix is **empty** — every
/// pair of singletons agrees on it, so this is the only layer where the
/// prefix-join's `break` never fires and its whole quadratic scan runs.
#[test]
fn the_join_from_singletons_is_every_unordered_pair_once() {
    let (terms, [a, b, c, _d]) = four_out_of_order();
    // Fed in reverse, because the join sorts its input before walking it.
    let a_prev = vec![vec![c], vec![b], vec![a]];
    assert_eq!(
        apriori_prefix_join(&terms, &a_prev),
        vec![vec![a, b], vec![a, c], vec![b, c]],
        "C(3,2) pairs, canonical order, no repeats and no (x, x)"
    );
}

/// A candidate holding an element that has left `alive` is dropped — and it is
/// still dropped when a learned clause would also have dropped it.
///
/// The second half is the one that matters: the two checks are sequential, so
/// a refactor that folded them into one condition could pass every
/// single-cause test and silently drop the alive check on the overlap.
#[test]
fn a_candidate_with_a_dead_element_is_dropped() {
    let (_terms, [a, b, c, _d]) = four_out_of_order();
    let empty = ein_core::Nogoods::default();
    let all: FxHashSet<FactId> = [a, b, c].into_iter().collect();
    let without_b: FxHashSet<FactId> = [a, c].into_iter().collect();
    let only_a: FxHashSet<FactId> = [a].into_iter().collect();

    assert!(
        filter_candidate(&[a, b, c], &all, &empty),
        "everything alive and nothing learned — the control"
    );
    assert!(
        !filter_candidate(&[a, b, c], &without_b, &empty),
        "b left alive, and the no-good store is empty"
    );
    assert!(
        !filter_candidate(&[a, b, c], &only_a, &nogoods(&[vec![a, b]])),
        "both checks fail; neither short-circuits the other away"
    );
}

/// A candidate is dropped iff some stored clause is a **subset** of it.
///
/// The direction is the whole point and it is easy to invert. A clause is
/// "every branch whose path condition is a *superset* of this is dead", so
/// containment runs clause ⊆ candidate; testing candidate ⊆ clause would keep
/// exactly the branches the search has already refuted.
#[test]
fn a_candidate_containing_a_learned_clause_is_dropped() {
    let (_terms, [a, b, c, d]) = four_out_of_order();
    let alive: FxHashSet<FactId> = [a, b, c].into_iter().collect();

    assert!(
        !filter_candidate(&[a, b, c], &alive, &nogoods(&[vec![a, b]])),
        "{{a,b}} ⊆ {{a,b,c}} — refuted"
    );
    assert!(
        filter_candidate(&[a, b, c], &alive, &nogoods(&[vec![a, d]])),
        "{{a,d}} holds a d the candidate lacks, so it says nothing about it"
    );
    assert!(
        filter_candidate(&[a, b, c], &alive, &ein_core::Nogoods::default()),
        "an empty store drops nothing"
    );
    assert!(
        !filter_candidate(&[a, b, c], &alive, &nogoods(&[vec![d], vec![b, c]])),
        "one matching clause among several is enough"
    );
}

/// `generate_layer` is the join followed by the filter, and it returns the
/// survivors **in the join's own emission order** — it does not re-sort.
///
/// That the order survives the filter is what makes the layer's traversal
/// order a property of the lattice rather than of which candidates happened to
/// die: re-sorting survivors would agree with this on the unfiltered case and
/// diverge on every real one.
#[test]
fn generate_layer_is_join_then_filter_in_emission_order() {
    let (terms, [a, b, c, d]) = four_out_of_order();
    let a_prev = vec![
        vec![a, b],
        vec![a, c],
        vec![a, d],
        vec![b, c],
        vec![b, d],
        vec![c, d],
    ];
    let all: FxHashSet<FactId> = [a, b, c, d].into_iter().collect();
    assert_eq!(
        generate_layer(&terms, &a_prev, &all, &ein_core::Nogoods::default()),
        vec![vec![a, b, c], vec![a, b, d], vec![a, c, d], vec![b, c, d]],
        "unfiltered: every 3-subset once, canonical order"
    );

    // `b` was back-propagated dead after `a_prev` closed, and `{a,b}` was
    // learned. The three b-bearing triples fail the alive check; `{a,c,d}`
    // escapes the clause because it holds no `b`.
    let without_b: FxHashSet<FactId> = [a, c, d].into_iter().collect();
    assert_eq!(
        generate_layer(&terms, &a_prev, &without_b, &nogoods(&[vec![a, b]])),
        vec![vec![a, c, d]]
    );
}

/// `layer_1` over an empty alive set yields **no** candidates — not one empty
/// candidate.
///
/// The distinction decides termination rather than cost. `layer_1` is
/// documented as "what the join would produce from `A_0 = {()}`", and an
/// implementation that took that literally would hand the search a single
/// empty commitment to enter, which is alive by construction and expands
/// forever.
#[test]
fn layer_1_of_an_empty_alive_set_is_empty() {
    let (terms, [a, b, c, _d]) = four_out_of_order();
    let none: FxHashSet<FactId> = FxHashSet::default();
    let out = layer_1(&terms, &none);
    assert!(out.is_empty(), "got {out:?}");
    assert!(
        !out.contains(&Vec::new()),
        "the empty commitment is not a candidate"
    );

    let some: FxHashSet<FactId> = [c, a, b].into_iter().collect();
    assert_eq!(
        layer_1(&terms, &some),
        vec![vec![a], vec![b], vec![c]],
        "the non-empty control, sorted by content"
    );
}

// ── The commitment primitive — `test_commitment.py` ────────────────

/// A fork whose hypothesis triggers a rule keeps the consequence to itself,
/// and `hypothesis_facts` is exactly what was committed.
///
/// Both halves are P1.21 R2 and neither is visible from a verdict: an engine
/// that wrote the derived fact back to root would still answer this puzzle
/// correctly, and only start returning wrong models once a *later* commitment
/// inherited a consequence of a hypothesis it never made.
#[test]
fn an_alive_entering_keeps_its_consequences_fork_local() {
    let (ast, mut terms, mut kb) = kb_of(
        "(rule swap ()\n  :match (target ?x ?y) :assert (other ?y ?x)\n\
         \x20 :why \"swap target → other\" :priority 100)\n\
         (relation target T T)\n(relation other T T)\n(is-a c T) (is-a d T)\n",
    );
    let h = fact(&mut terms, "target", &["c", "d"]);
    let derived = fact(&mut terms, "other", &["d", "c"]);
    let mut ev = Events::off();
    let r = try_commitment_set(
        kb.sealed(),
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h],
        None,
        None,
    )
    .expect("enters");

    assert_eq!(r.kind, Kind::Alive);
    assert_eq!(r.hypothesis_facts, vec![h], "only the committed writes");
    assert!(r.kb.contains(derived), "the fork derived (other d c)");
    assert!(!kb.contains(derived), "…and root did not learn it");
    assert!(!kb.contains(h), "…nor the hypothesis itself");
}

/// Both death kinds report a non-empty unsat core naming a **committed
/// hypothesis**, and the dead-post one gets there only after saturation ran.
///
/// A dead fork whose core did not name a hypothesis would be a dead *root*:
/// the search would learn a clause it can never satisfy and prune branches
/// that were never at fault. The two kinds reach the core by different routes
/// — dead-pre's witness is the committed positive itself, dead-post's is a
/// fact several firings downstream — so a core walk can be right for one and
/// wrong for the other.
///
/// Merges the work-list's `dead-post-is-detected-after-saturation-ran`: the
/// `firings` assertion below *is* that claim, and splitting it across two
/// tests would mean entering the same commitment twice to check two halves of
/// one result.
#[test]
fn a_dead_entering_names_a_committed_hypothesis_in_its_core() {
    // dead-pre — root already carries the negation of what is committed.
    let (ast, mut terms, mut kb) = kb_of(
        "(relation target T T)\n(is-a c T) (is-a d T)\n\
         (not (target c d) :source \"a prior back-prop write\")\n",
    );
    let h = fact(&mut terms, "target", &["c", "d"]);
    let mut ev = Events::off();
    let pre = try_commitment_set(
        kb.sealed(),
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h],
        None,
        None,
    )
    .expect("enters");
    assert_eq!(pre.kind, Kind::DeadPre);
    assert!(pre.firings.is_empty(), "no saturation ran");
    assert!(pre.unsat_core.contains(&h), "{:?}", pre.unsat_core);
    assert_eq!(pre.hypothesis_facts, vec![h]);

    // dead-post — two hypotheses whose rules derive `(x a)` and `(not (x a))`.
    let (ast, mut terms, mut kb) = kb_of(CLASH_LADDER);
    let h1 = fact(&mut terms, "h1", &["a"]);
    let h2 = fact(&mut terms, "h2", &["a"]);
    let mut ev = Events::off();
    let post = try_commitment_set(
        kb.sealed(),
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h1, h2],
        None,
        None,
    )
    .expect("enters");
    assert_eq!(post.kind, Kind::DeadPost);
    assert!(
        !post.firings.is_empty(),
        "dead-post means the post-saturation detector found it, not the pre-check"
    );
    assert_eq!(post.hypothesis_facts, vec![h1, h2]);
    assert!(
        post.unsat_core.contains(&h1) || post.unsat_core.contains(&h2),
        "core {:?} names neither committed hypothesis",
        post.unsat_core
    );
}

/// `try_commitment_set(root.sealed(), [])` is the layer-zero sentinel: alive, nothing
/// written, and over an already-saturated root the fork is root.
///
/// The empty set is not a degenerate input to be rejected — the search asks
/// for it, and an implementation that treated "no hypotheses" as "nothing to
/// check" and skipped the detector would call a contradictory root alive.
#[test]
fn the_empty_commitment_is_the_layer_zero_sentinel() {
    let (ast, mut terms, mut kb) = kb_of(
        "(rule sym-r ()\n  :match (r ?x ?y) :assert (r ?y ?x)\n\
         \x20 :why \"symmetric r\" :priority 100)\n\
         (relation r T T)\n(is-a a T) (is-a b T)\n(r a b :source \"(1)\")\n",
    );
    saturate(&ast, &mut terms, &mut kb);
    let before: FxHashSet<FactId> = kb.facts().collect();

    let mut ev = Events::off();
    let r = try_commitment_set(
        kb.sealed(),
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[],
        None,
        None,
    )
    .expect("enters");

    assert_eq!(r.kind, Kind::Alive);
    assert!(r.hypothesis_facts.is_empty());
    assert_eq!(
        r.kb.facts().collect::<FxHashSet<_>>(),
        before,
        "a pre-saturated root has nothing left to derive"
    );
}

/// A dead-post commitment plus a five-step ladder that is independent of the
/// clash — the fixture the three fail-fast tests share.
///
/// `h1 → x` and `h2 → ¬x` kill the fork at priority 100; `chain0 → … → chain5`
/// is queued behind them at 200. Fail-fast is visible as the ladder not being
/// walked, which is a *fact-set* difference rather than only a counter one.
const CLASH_LADDER: &str = "\
(rule h1-implies-x () :match (h1 ?x) :assert (x ?x) :why \"h1 → x\" :priority 100)
(rule h2-forbids-x () :match (h2 ?x) :assert (not (x ?x)) :why \"h2 → ¬x\" :priority 100)
(rule step1 () :match (chain0 ?x) :assert (chain1 ?x) :why \"ladder\" :priority 200)
(rule step2 () :match (chain1 ?x) :assert (chain2 ?x) :why \"ladder\" :priority 200)
(rule step3 () :match (chain2 ?x) :assert (chain3 ?x) :why \"ladder\" :priority 200)
(rule step4 () :match (chain3 ?x) :assert (chain4 ?x) :why \"ladder\" :priority 200)
(rule step5 () :match (chain4 ?x) :assert (chain5 ?x) :why \"ladder\" :priority 200)
(relation h1 T) (relation h2 T) (relation x T)
(relation chain0 T) (relation chain1 T) (relation chain2 T)
(relation chain3 T) (relation chain4 T) (relation chain5 T)
(is-a a T)
(chain0 a :source \"(1)\")
";

/// Enter `commitment` on a fresh `CLASH_LADDER` root with fail-fast set.
fn ladder_entering(fail_fast: bool, names: &[&str]) -> (Terms, ein_infer::CommitmentSetResult) {
    let (ast, mut terms, mut kb) = kb_of(CLASH_LADDER);
    kb.program_mut().config = Some(SolverConfig {
        enable_fail_fast_fork: fail_fast,
        ..SolverConfig::default()
    });
    let commitment: Vec<FactId> = names.iter().map(|n| fact(&mut terms, n, &["a"])).collect();
    let mut ev = Events::off();
    let r = try_commitment_set(
        kb.sealed(),
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &commitment,
        None,
        None,
    )
    .expect("enters");
    (terms, r)
}

/// Fail-fast stops a dying fork at the firing that killed it, and the verdict
/// does not move.
///
/// Sound because the KB is append-only — a contradiction can be created but
/// never retracted — so the *only* thing that may differ between the two runs
/// is how much dead-branch work was done. That is why this asserts a strict
/// subset in both directions at once: fewer firings **and** fewer facts, with
/// the clashing fact still present (it is what stopped it) and the ladder
/// finished only in the full run. An "optimisation" that stopped one firing
/// too early would lose the clash; one that stopped too late would keep the
/// ladder.
#[test]
fn fail_fast_truncates_a_dying_fork_and_keeps_its_verdict() {
    let (full_terms, full) = ladder_entering(false, &["h1", "h2"]);
    let (fast_terms, fast) = ladder_entering(true, &["h1", "h2"]);

    assert_eq!(full.kind, Kind::DeadPost);
    assert_eq!(fast.kind, full.kind, "the verdict is the whole contract");
    assert!(
        fast.firings.len() < full.firings.len(),
        "fast {} vs full {}",
        fast.firings.len(),
        full.firings.len()
    );

    // Two roots, two arenas, so the fact sets compare as s-expressions.
    let full_facts = sexprs(&full_terms, full.kb.facts());
    let fast_facts = sexprs(&fast_terms, fast.kb.facts());
    assert!(
        fast_facts.is_subset(&full_facts) && fast_facts.len() < full_facts.len(),
        "a truncated fork is a strict prefix of the fixpoint one"
    );
    assert!(fast_facts.contains("(x a)"), "the clash is what stopped it");
    assert!(full_facts.contains("(chain5 a)"), "the ladder ran in full");
    assert!(!fast_facts.contains("(chain5 a)"), "…and not in fast");

    let core = sexprs(&fast_terms, fast.unsat_core.iter().copied());
    assert!(
        core.contains("(h1 a)") || core.contains("(h2 a)"),
        "a truncated fork still explains itself: {core:?}"
    );
}

/// With no contradiction there is nothing to stop at, so the flag is invisible
/// on a surviving fork.
///
/// This is the half that makes the previous test a *speed* claim rather than a
/// semantic one. Fail-fast reads every derived fact through `contradicts`, and
/// a check that answered `true` one fact early would truncate live branches
/// too — which no verdict on a satisfiable puzzle would reveal, because the
/// model it found would still be a model.
#[test]
fn fail_fast_does_not_touch_a_surviving_fork() {
    let (full_terms, full) = ladder_entering(false, &["h1"]);
    let (fast_terms, fast) = ladder_entering(true, &["h1"]);

    assert_eq!(full.kind, Kind::Alive);
    assert_eq!(fast.kind, Kind::Alive);
    assert_eq!(fast.firings.len(), full.firings.len());
    let fast_facts = sexprs(&fast_terms, fast.kb.facts());
    assert_eq!(fast_facts, sexprs(&full_terms, full.kb.facts()));
    assert!(fast_facts.contains("(chain5 a)"), "reached the fixpoint");
}

/// A commitment already refuted at root dies before any saturation, whatever
/// the flag says.
///
/// Fail-fast lives inside the saturation loop, so the pre-check has to run in
/// front of it. Reordering the two would still produce `dead-pre` — the
/// detector would find the same pair — but with a firing list, and the search
/// would have paid for a fork it never needed to build.
#[test]
fn fail_fast_cannot_reach_a_dead_pre() {
    for flag in [false, true] {
        let (ast, mut terms, mut kb) =
            kb_of(&format!("{CLASH_LADDER}(not (h1 a) :source \"stated\")\n"));
        kb.program_mut().config = Some(SolverConfig {
            enable_fail_fast_fork: flag,
            ..SolverConfig::default()
        });
        let h1 = fact(&mut terms, "h1", &["a"]);
        let mut ev = Events::off();
        let r = try_commitment_set(
            kb.sealed(),
            &mut terms,
            &ast,
            &mut ev,
            &SharedMemo::default(),
            &[h1],
            None,
            None,
        )
        .expect("enters");
        assert_eq!(r.kind, Kind::DeadPre, "fail_fast={flag}");
        assert!(r.firings.is_empty(), "fail_fast={flag}: saturation ran");
    }
}

// ── Contradiction detection — `test_contradiction.py` ──────────────

/// A pair is a contradiction however each side entered the KB — the S1.22.1b
/// soundness fix.
///
/// `(sits A S1)` and `(sits B S1)` are both authored clues; `one-per-seat`
/// derives the negation of each from the other. Until S1.22.1b the detector
/// skipped a pair whose facts sat in different knowledge layers, so this KB —
/// flatly inconsistent with its own input — reported **zero** contradictions.
/// The corpus-scale instance of the same bug is
/// `examples/ein-bugs/zebra2-bad.ein`, whose `Contradiction` verdict
/// `acceptance.rs` owns; this is the unit-level pin, and it checks the
/// provenance of both sides so that a detector which had merely stopped
/// looking at layers could not pass it by accident.
#[test]
fn mutual_negation_is_detected_whatever_either_sides_origin() {
    let (ast, mut terms, mut kb) = kb_of(
        "(relation sits Person Seat)\n(relation is-a Thing Thing)\n\
         (is-a A Person) (is-a B Person)\n\
         (rule one-per-seat ()\n\
         \x20 :match  (and (sits ?p1 ?s) (is-a ?p2 Person) (neq ?p1 ?p2))\n\
         \x20 :assert (not (sits ?p2 ?s))\n\
         \x20 :why \"{?s} is taken by {?p1}\" :priority 100)\n\
         (sits A S1 :source \"clue (1)\")\n(sits B S1 :source \"clue (2)\")\n",
    );
    saturate(&ast, &mut terms, &mut kb);

    let found = detect(&kb, &terms);
    assert_eq!(
        found.len(),
        2,
        "{:?}",
        sexprs(&terms, found.iter().map(|c| c.negative))
    );
    assert_eq!(
        sexprs(&terms, found.iter().filter_map(|c| c.positive)),
        ["(sits A S1)", "(sits B S1)"]
            .map(str::to_string)
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    for c in &found {
        let positive = c.positive.expect("a pair");
        assert_eq!(primary_kind(&kb, &terms, positive), Some(ProvKind::Source));
        assert_eq!(
            primary_rule(&kb, &terms, c.negative).as_deref(),
            Some("one-per-seat"),
            "the negative is rule-derived"
        );
    }
}

/// `detect` reports one `Contradiction` per distinct `(X, ¬X)` pair, not the
/// first one it meets.
///
/// The unsat core of a `k = 0` verdict is the union over every recorded dead
/// commitment's core, so a detector that stopped at the first pair would
/// narrow the explanation to whichever conflict the extent order happened to
/// put first — an answer that changes with the input's line order.
#[test]
fn every_pair_is_reported_not_only_the_first() {
    let (terms, kb) = {
        let (_ast, terms, kb) = kb_of(
            "(relation r T T)\n\
             (r A B) (not (r A B))\n(r C D) (not (r C D))\n(r E F) (not (r E F))\n",
        );
        (terms, kb)
    };
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 3);
    assert_eq!(
        sexprs(&terms, found.iter().filter_map(|c| c.positive)),
        ["(r A B)", "(r C D)", "(r E F)"]
            .map(str::to_string)
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
}

/// The inner positive of a `(not …)` is matched by the structural identity of
/// its own nested-fact argument, one level further in than a flat `(p one)`.
///
/// `(not (hypothesis (co-located N H2)))` is the shape Q40 introduced and the
/// one the hypothesis-contradiction idiom actually writes. The detector's
/// lookup is `kb.contains(inner)`, so it is right at any depth *provided*
/// interning is structural all the way down — which is precisely what a
/// pointer-equality or top-level-only comparison would get wrong.
#[test]
fn a_nested_positive_pairs_through_structural_identity() {
    let (_ast, terms, kb) = kb_of(
        "(relation hypothesis T)\n(relation co-located T T)\n\
         (hypothesis (co-located Norwegian House-2))\n\
         (not (hypothesis (co-located Norwegian House-2)))\n",
    );
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        ein_infer::events::sexpr(&terms, found[0].positive.expect("a pair")),
        "(hypothesis (co-located Norwegian House-2))"
    );
    assert_eq!(found[0].kind, ein_infer::contradiction::Kind::Pair);
}

/// A `(not 5)` — a negation whose argument is not a fact — yields no
/// contradiction and no panic, from either the scan or the incremental check.
///
/// Neither the loader nor the matcher produces this shape, which is exactly
/// why it needs a test: the `else` branch of
/// `args.first().and_then(|v| v.as_fact())` is unreachable from any corpus
/// input, so nothing but a hand-built fact exercises it, and the tolerant
/// reading is a decision (Q40 / R9) rather than an accident.
#[test]
fn a_malformed_negation_is_tolerated_rather_than_crashing() {
    let (_ast, mut terms, mut kb) = kb_of("(relation r T T)\n(r A B :source \"(1)\")\n");
    let not = terms.kernel.not;
    let five = Value::int(terms.intern_int("5").expect("room"));
    let malformed = terms.intern_fact(not, &[five]).expect("room");
    kb.add_and_index_fact(&mut terms, not, &[five], None)
        .expect("room");

    assert!(kb.contains(malformed), "the fixture is in the KB");
    assert!(detect(&kb, &terms).is_empty());
    assert!(!contradicts(&kb, &terms, malformed));
    assert!(!ein_infer::has_contradiction(&kb, &terms));
}

/// `witness()` is the positive for a pair and the `(false)` fact for a direct
/// ⊥ — in both shapes a fact the KB holds, so the unsat-core walk has a
/// derivation to seed from.
///
/// No ein.rs test called `witness()` before this one: every caller reaches it
/// through a digest, where returning the *negative* of a pair would still
/// produce a plausible-looking core (the negation's own premises) and would
/// only be wrong in that it explains the rebuttal instead of the claim.
#[test]
fn witness_is_the_positive_for_a_pair_and_the_false_fact_for_a_direct() {
    let (_ast, terms, kb) = kb_of(
        "(relation r T T)\n(r A B :source \"(1)\")\n(not (r A B) :source \"(2)\")\n\
         (false :source \"(3)\")\n",
    );
    let found = detect(&kb, &terms);
    assert_eq!(found.len(), 2, "{found:?}");

    let direct = found
        .iter()
        .find(|c| c.kind == ein_infer::contradiction::Kind::Direct)
        .expect("a direct ⊥");
    assert_eq!(direct.positive, None);
    assert_eq!(direct.witness(), direct.negative);

    let pair = found
        .iter()
        .find(|c| c.kind == ein_infer::contradiction::Kind::Pair)
        .expect("a pair");
    assert_eq!(pair.witness(), pair.positive.expect("a pair has one"));
    assert_ne!(pair.witness(), pair.negative);

    for c in &found {
        assert!(kb.contains(c.witness()), "the witness is believed");
        assert!(
            !smallest_contradiction_frontier(&kb, &terms, Some(&[c.witness()])).is_empty(),
            "and it seeds a frontier"
        );
    }
}

/// Replaying a saturation firing by firing, the incremental check flips true
/// at exactly the firing at which a full scan first becomes non-empty.
///
/// This is the equivalence S1.9.E23's fail-fast fork rests on, and it is a
/// claim about a *running* saturation rather than a static KB: a
/// `contradicts` that only consulted the KB's own indexes would agree with
/// `detect` on any settled state and still be a firing late during the run —
/// which is the one moment at which the answer is used.
#[test]
fn fail_fast_agrees_with_the_post_hoc_scan_over_a_real_saturation() {
    let (ast, mut terms, mut kb) = kb_of(
        "(rule h1-implies-x () :match (h1 ?x) :assert (x ?x) :why \"h1 → x\" :priority 100)\n\
         (rule h2-forbids-x () :match (h2 ?x) :assert (not (x ?x)) :why \"h2 → ¬x\" :priority 100)\n\
         (relation h1 T) (relation h2 T) (relation x T)\n(is-a a T)\n(h1 a) (h2 a)\n",
    );
    let (first_incremental, first_scan) = in_session(&ast, &mut terms, &mut kb, |s| {
        let mut sat = Saturator::new(s).expect("compiles");
        let (mut incremental, mut scan, mut i) = (None, None, 0usize);
        while let Some(firing) = sat.step(s).expect("saturates") {
            i += 1;
            if scan.is_none() && !detect(s.kb, s.terms).is_empty() {
                scan = Some(i);
            }
            if incremental.is_none()
                && !firing.redundant
                && firing
                    .derived
                    .iter()
                    .any(|&d| contradicts(s.kb, s.terms, d))
            {
                incremental = Some(i);
            }
        }
        (incremental, scan)
    });
    assert!(first_incremental.is_some(), "the fixture must die");
    assert_eq!(first_incremental, first_scan);
}

// ── The per-rule demos — `test_demos.py` ───────────────────────────

/// The eight rule directories, each with three scenarios.
const DEMO_RULES: [&str; 8] = [
    "hypothesis-contradiction",
    "implies",
    "square-bwd",
    "square-fwd",
    "square-unique",
    "symmetric",
    "transitive",
    "type-exclusivity",
];

/// `(<rule name>, <path>)` for every demo, sorted.
fn demos() -> Vec<(String, PathBuf)> {
    let root = repo_root().join("examples/saturation");
    let mut out = Vec::new();
    for rule in DEMO_RULES {
        let dir = root.join(rule);
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
            .map(|e| e.expect("dir entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "ein"))
            .collect();
        files.sort();
        out.extend(files.into_iter().map(|p| (rule.to_string(), p)));
    }
    out
}

fn demo_id(path: &Path) -> String {
    let stem = path.file_stem().expect("a file").to_string_lossy();
    let dir = path
        .parent()
        .and_then(Path::file_name)
        .expect("a directory")
        .to_string_lossy();
    format!("{dir}/{stem}")
}

/// `examples/saturation/` is exactly eight rule directories of three scenarios
/// — 24 files, no more and no fewer.
///
/// The layout is load-bearing rather than tidy: every other demo test derives
/// the rule name it expects from the *directory* name, so a demo filed under
/// the wrong rule, or a ninth rule added without scenarios, would make those
/// tests weaker without making them fail.
#[test]
fn the_demo_directory_layout_is_eight_rules_times_three_scenarios() {
    let root = repo_root().join("examples/saturation");
    let mut dirs: Vec<String> = std::fs::read_dir(&root)
        .expect("examples/saturation")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.is_dir())
        .map(|p| {
            p.file_name()
                .expect("a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    dirs.sort();
    assert_eq!(dirs, DEMO_RULES);

    let found = demos();
    assert_eq!(found.len(), 24, "8 rules × 3 scenarios");
    for rule in DEMO_RULES {
        assert_eq!(
            found.iter().filter(|(r, _)| r == rule).count(),
            3,
            "{rule}/ should hold three scenarios"
        );
    }
}

/// Each demo's directory name is a rule name, and saturating the demo fires
/// that rule.
///
/// The demos are documentation that runs: their whole claim is "this is what
/// rule X looks like doing its job". A demo that saturates fine while never
/// firing the rule it is filed under still renders, still passes the corpus
/// digests and teaches the reader the wrong thing.
#[test]
fn each_demo_fires_the_rule_its_directory_names() {
    for (rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let firings = saturate(&ast, &mut terms, &mut kb);
        let seen: Vec<&str> = firings.iter().map(|f| terms.sym(f.rule)).collect();
        assert!(
            seen.contains(&rule.as_str()),
            "{}: {rule:?} did not fire; observed {seen:?}",
            demo_id(&path)
        );
    }
}

/// At least one of the named rule's firings concludes a **genuinely derived**
/// fact rather than re-deriving an authored one.
///
/// The two are indistinguishable from a firing count. `symmetric` matches
/// `(rel A B)`, derives `(rel B A)`, then matches `(rel B A)` and re-derives
/// `(rel A B)` — which already exists, so the insert dedupes back onto the
/// authored fact and keeps its `source` provenance. A demo whose *every*
/// firing bounced that way would fire the rule and show nothing, so the
/// provenance of the conclusion is what has to be checked.
#[test]
fn each_demo_produces_a_genuinely_derived_fact() {
    for (rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let firings = saturate(&ast, &mut terms, &mut kb);
        let kinds: Vec<Option<ProvKind>> = firings
            .iter()
            .filter(|f| terms.sym(f.rule) == rule)
            .filter_map(|f| f.derived.first().copied())
            .map(|d| primary_kind(&kb, &terms, d))
            .collect();
        assert!(
            kinds.contains(&Some(ProvKind::Rule)),
            "{}: {rule:?} fired but derived nothing new; provenance kinds {kinds:?}",
            demo_id(&path)
        );
    }
}

/// Every demo carries a `(query … :goal …)` naming the fact its rule is
/// expected to produce.
///
/// The goal is the demo's assertion in the file itself — the machine-readable
/// half of the prose header — and it is what lets a reader (or the trace
/// renderer) check the demo against its own claim rather than against this
/// test's idea of it.
#[test]
fn each_demo_carries_a_query_goal() {
    for (_rule, path) in demos() {
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let kb = load_file(&mut ast, &mut terms, &path).expect("the demo loads");
        let query = kb
            .program()
            .query()
            .unwrap_or_else(|| panic!("{}: no (query …) block", demo_id(&path)));
        assert!(
            ein_infer::query_value(&ast, query, "goal").is_some(),
            "{}: (query …) has no :goal",
            demo_id(&path)
        );
    }
}

// ── The one-step lookahead — `test_dies_immediately.py` ────────────

/// Saturate `src`, then ask the lookahead about `(rel arg…)`.
fn dies(src: &str, rel: &str, args: &[&str]) -> bool {
    let (ast, mut terms, mut kb) = kb_of(src);
    saturate(&ast, &mut terms, &mut kb);
    let h = fact(&mut terms, rel, args);
    in_session(&ast, &mut terms, &mut kb, |s| {
        let l = Lookahead::new(s).expect("compiles");
        l.dies_immediately(s, &mut Matcher::new(), h)
    })
}

/// A rule that derives `(not h)` straight from `h` kills the candidate `h`,
/// even though neither `h` nor its negation is in the KB.
///
/// This is the branch where **both** sides of the pair are hypothetical, and
/// it is the one the ordinary detector cannot express: `contradicts` asks the
/// KB, and the KB holds neither fact. The probe has to notice that the
/// conclusion it just built *is* the negation of the thing it posited — the
/// `g == h` test — which is asserted here by first checking the KB knows
/// nothing about `(r A B)` at all.
#[test]
fn a_rule_that_negates_the_candidate_itself_kills_it() {
    let src = "(rule deny (?R)\n  :match (?R ?a ?b) :assert (not (?R ?a ?b))\n\
               \x20 :why \"deny\" :priority 100)\n(relation r T T)\n(deny r)\n";
    let (ast, mut terms, mut kb) = kb_of(src);
    saturate(&ast, &mut terms, &mut kb);
    let h = fact(&mut terms, "r", &["A", "B"]);
    assert!(!kb.contains(h), "the KB does not hold the candidate");
    assert!(!kb.is_negated(h), "…nor its negation");
    assert!(dies(src, "r", &["A", "B"]));
}

/// A derived `(not g)` kills the candidate whenever `g` is believed — stated
/// or itself derived — and kills nothing when `g` is absent.
///
/// The mirror of the detector's S1.22.1b fix. Before it, the source-annotated
/// `(r A)` was exempt because it sat in a different knowledge layer from the
/// derived negative, so a candidate survived a step it could not survive; the
/// filter and the detector disagreeing that way is how a branch reaches the
/// search only to die on the next line. The absent-`g` case is the guard in
/// the other direction: this filter may only ever **under**-approximate death,
/// because a hypothesis wrongly reported dead is silently lost.
#[test]
fn a_derived_negative_kills_against_a_positive_of_any_origin() {
    let src = "(rule deny ()\n  :match (cand ?x) :assert (not (r ?x))\n\
               \x20 :why \"deny\" :priority 100)\n\
               (rule make-r ()\n  :match (seed ?x) :assert (r ?x)\n\
               \x20 :why \"derive\" :priority 50)\n\
               (relation cand T)\n(relation r T)\n(relation seed T)\n\
               (r A :source \"given\")\n(seed C :source \"(1)\")\n";

    // The two positives really do have different origins.
    let (ast, mut terms, mut kb) = kb_of(src);
    saturate(&ast, &mut terms, &mut kb);
    let stated = fact(&mut terms, "r", &["A"]);
    let derived = fact(&mut terms, "r", &["C"]);
    assert_eq!(primary_kind(&kb, &terms, stated), Some(ProvKind::Source));
    assert_eq!(primary_kind(&kb, &terms, derived), Some(ProvKind::Rule));

    assert!(dies(src, "cand", &["A"]), "stated positive");
    assert!(dies(src, "cand", &["C"]), "derived positive");
    assert!(
        !dies(src, "cand", &["E"]),
        "no (r E) to clash with — a live hypothesis must survive"
    );
}

/// With no rules there are no plans, so nothing can die in one step and every
/// candidate is reported alive.
///
/// The default has to be *alive*: the filter runs before the fork, so a
/// vacuous "no plan disproved it, therefore dead" would delete the entire
/// search space of any rule-free puzzle without a single contradiction being
/// found.
#[test]
fn a_kb_with_no_rules_never_kills() {
    let src = "(relation co-located T T)\n";
    let (_ast, _terms, kb) = kb_of(src);
    assert!(
        kb.program().rules.is_empty(),
        "the fixture declares no rules"
    );
    assert!(!dies(src, "co-located", &["A", "B"]));
}

// ── The smallest frontier — `test_frontier.py` ─────────────────────

/// One fact with two derivations, plus one clash — the R3 report's E3 fixture.
///
/// `(X a)` follows from `(A a) ∧ (B a)` via `join` and from `(C a)` alone via
/// `chain`; `(X a) ∧ (Y a)` is absurd. Only the two rules' `:priority` moves
/// between instantiations, and lower fires first — so which derivation wins
/// the dedup race and becomes `(X a)`'s *primary* provenance flips with it.
fn two_derivations(join: i32, chain: i32) -> String {
    format!(
        "(relation A T)\n(relation B T)\n(relation C T)\n(relation X T)\n(relation Y T)\n\
         (rule join ()\n  :match (and (A ?o) (B ?o)) :assert (X ?o)\n\
         \x20 :why \"join\" :priority {join})\n\
         (rule chain ()\n  :match (C ?o) :assert (X ?o)\n\
         \x20 :why \"chain\" :priority {chain})\n\
         (rule clash ()\n  :match (and (X ?o) (Y ?o)) :assert (false)\n\
         \x20 :why \"clash\" :priority 300)\n\
         (A a :source \"(A)\")\n(B a :source \"(B)\")\n(C a :source \"(C)\")\n\
         (Y a :source \"(Y)\")\n"
    )
}

/// A single functional clash — one `(false)` witness whose frontier is the
/// clashing pair.
const FUNCTIONAL: &str = "\
(rule functional (?R)
  :match  (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
  :assert (false)
  :why    \"fn\" :priority 100)
(relation R T T)
(functional R)
(R x One :source \"(1)\")
(R x Two :source \"(2)\")
";

fn saturated(src: &str) -> (Ast, Terms, Kb) {
    let (ast, mut terms, mut kb) = kb_of(src);
    saturate(&ast, &mut terms, &mut kb);
    (ast, terms, kb)
}

/// Flipping the two rules' priorities flips which derivation is recorded
/// primary and leaves the reported frontier identical.
///
/// The deliberate inversion of a test that used to pin the opposite. With one
/// justification per fact, the frontier was `{C, Y}` when `chain` fired first
/// and `{A, B, Y}` when `join` did — even though the two-fact explanation
/// still existed in the second run and nothing about the puzzle had changed.
/// S1.21.7 records the loser as an alternative and searches the AND/OR graph,
/// so what a reader is told no longer depends on a dedup race. The primary
/// provenance is asserted here *because* it still flips: that is what shows
/// the fix is in the search rather than in which derivation wins.
#[test]
fn the_reported_frontier_does_not_depend_on_which_derivation_fired_first() {
    for (join, chain, expected_primary) in [(100, 50, "chain"), (50, 100, "join")] {
        let (_ast, terms, kb) = saturated(&two_derivations(join, chain));
        let frontier = smallest_contradiction_frontier(&kb, &terms, None);
        assert_eq!(
            rel_names(&terms, &frontier),
            ["C", "Y"],
            "join={join} chain={chain}"
        );

        let mut terms = terms;
        let x_a = fact(&mut terms, "X", &["a"]);
        assert_eq!(
            primary_rule(&kb, &terms, x_a).as_deref(),
            Some(expected_primary),
            "the dedup race still has a winner"
        );
        let alts: Vec<String> = kb
            .alternatives(x_a)
            .iter()
            .filter_map(|&p| terms.provs.get(p).rule)
            .map(|r| terms.sym(r).to_string())
            .collect();
        assert_eq!(alts.len(), 1, "the loser is retained: {alts:?}");
        assert_ne!(alts[0], expected_primary);
    }
}

/// An explanation names only facts the **OR-aware** premise closure reaches,
/// and on the flipped run it is deliberately *not* a subset of the
/// primary-only union core.
///
/// Half a soundness envelope is not one. Every explanation is inside the
/// `All` closure — that is what makes it a real set of derivation leaves — but
/// an implementation that quietly returned `unsat_core(witnesses, Primary)`
/// would satisfy that half on every input. The second assertion is what
/// separates them: in the `join`-first run the primary-only core is
/// `{A, B, Y}`, and reporting `{C, Y}` means stepping *outside* it. That
/// over-report is exactly what S1.21.7 removed.
#[test]
fn an_explanation_is_a_subset_of_the_or_aware_closure_and_not_of_the_primary_only_union() {
    let (_ast, terms, kb) = saturated(FUNCTIONAL);
    let frontier = smallest_contradiction_frontier(&kb, &terms, None);
    assert!(!frontier.is_empty(), "a single clash still explains itself");
    let primary = union_core(&kb, &terms, Justifications::Primary);
    assert!(
        frontier.iter().all(|f| primary.contains(f)),
        "with one derivation per fact the two readings coincide"
    );

    let (_ast, terms, kb) = saturated(&two_derivations(50, 100));
    let frontier = smallest_contradiction_frontier(&kb, &terms, None);
    let all = union_core(&kb, &terms, Justifications::All);
    assert!(
        frontier.iter().all(|f| all.contains(f)),
        "sound: {:?} ⊄ {:?}",
        sexprs(&terms, frontier.iter().copied()),
        // The assertion above is `⊆`, which has no order; the set below is
        // only rendered once it has already failed, and nothing reads it.
        // determinism-ok: an `⊆` assertion's failure message.
        sexprs(&terms, all.iter().copied())
    );
    let primary = union_core(&kb, &terms, Justifications::Primary);
    assert!(
        !frontier.iter().all(|f| primary.contains(f)),
        "the union core is {:?} — an explanation confined to it would be the \
         pre-S1.21.7 over-report",
        sexprs(&terms, primary.iter().copied())
    );
}

/// An unbudgeted search reports exhaustion; the same search under
/// `max_rounds = 1` says it did not, and stays sound.
///
/// Minimum-cardinality source frontier over an AND/OR graph is worst-case
/// exponential, so every truncation is legitimate — but a truncated search
/// that still claimed `exhausted` would present "possibly not smallest" as
/// "proven smallest", and there is nothing downstream that could tell the
/// difference. `rounds > 0` and a named `target` are the other half: an
/// implementation that returned the recorded fallback would also be sound and
/// would also be `exhausted = false`, and it is the fallback this budget is
/// *not* supposed to reach.
#[test]
fn a_full_search_reports_exhaustion_and_a_budgeted_one_admits_it_did_not() {
    let (_ast, terms, kb) = saturated(&two_derivations(50, 100));

    let full = minimal_contradiction_frontier(&kb, &terms, None, &ExplanationBudget::default());
    assert!(full.exhausted);
    assert!(full.target.is_some());
    assert!(full.rounds > 0);
    assert_eq!(rel_names(&terms, &full.frontier), ["C", "Y"]);

    let tight = minimal_contradiction_frontier(
        &kb,
        &terms,
        None,
        &ExplanationBudget {
            max_rounds: 1,
            ..ExplanationBudget::default()
        },
    );
    assert!(!tight.exhausted, "a capped search must admit it was capped");
    let envelope = union_core(&kb, &terms, Justifications::All);
    assert!(
        tight.frontier.iter().all(|f| envelope.contains(f)),
        "a truncated frontier is still a real set of derivation leaves"
    );
}

/// Turning alternative recording off restores the pre-S1.21.7 answer — the
/// gate moves the *frontier*, not merely a counter.
///
/// A config flag that only stopped a table from filling would be untestable
/// from the outside and indistinguishable from dead configuration. Here the
/// same saturated KB reports `{A, B, Y}` with the flag off and `{C, Y}` with
/// it on, which is what makes it a real off switch: the alternatives are the
/// only reason the smaller explanation is findable at all.
#[test]
fn turning_alternative_recording_off_restores_the_primary_only_frontier() {
    let src = two_derivations(50, 100);

    let (ast, mut terms, mut kb) = kb_of(&src);
    kb.program_mut().config = Some(SolverConfig {
        record_alternative_justifications: false,
        ..SolverConfig::default()
    });
    saturate(&ast, &mut terms, &mut kb);
    assert!(
        !kb.has_alternative_justifications(),
        "the alternatives table stays empty"
    );
    assert_eq!(
        rel_names(&terms, &smallest_contradiction_frontier(&kb, &terms, None)),
        ["A", "B", "Y"],
        "the recorded-primary answer"
    );

    let (_ast, terms, kb) = saturated(&src);
    assert!(
        kb.has_alternative_justifications(),
        "the control records them"
    );
    assert_eq!(
        rel_names(&terms, &smallest_contradiction_frontier(&kb, &terms, None)),
        ["C", "Y"]
    );
}

// ── Guided hypothesis generation — `test_guided_hypgen.py` ─────────

/// Two declared relations over two objects — the blind enumerator proposes
/// both unless a query keyword narrows it.
const TWO_REL: &str = "(relation co-located T T)\n(relation likes T T)\n\
                       (is-a Thing T)\n(is-a A Thing) (is-a B Thing)\n";

/// The candidates one `generate` call emits, plus its stats.
fn candidates(src: &str) -> (BTreeSet<String>, Vec<String>, HypGenStats) {
    let (ast, mut terms, mut kb) = kb_of(src);
    let mut stats = HypGenStats::new();
    let mut out = Vec::new();
    in_session(&ast, &mut terms, &mut kb, |s| {
        ein_infer::generate(s, &mut stats, &mut |f| {
            out.push(f);
            ControlFlow::Continue(())
        })
        .expect("generates")
    });
    let rels = rel_names(&terms, &out);
    (sexprs(&terms, out), rels, stats)
}

/// A `:hypothesis-relations` / `:no-hypothesis` value may be a bare SYMBOL,
/// and it means exactly what the one-element list means.
///
/// No corpus file uses the bare form, so nothing else in the suite would
/// notice if `coerce_relation_names` stopped reading an `Atom` value and fell
/// through to the "keyword absent" branch — which for a whitelist is
/// *unrestricted*, i.e. the filter silently turning itself off rather than
/// failing.
#[test]
fn query_relation_keyword_accepts_a_bare_symbol() {
    let goal = "(query :goal (co-located A B)";
    for (bare, list) in [
        (
            ":hypothesis-relations co-located)",
            ":hypothesis-relations (co-located))",
        ),
        (":no-hypothesis likes)", ":no-hypothesis (likes))"),
    ] {
        let (bare_facts, bare_rels, _) = candidates(&format!("{TWO_REL}{goal} {bare}"));
        let (list_facts, list_rels, _) = candidates(&format!("{TWO_REL}{goal} {list}"));
        assert!(!bare_facts.is_empty(), "{bare}: nothing generated at all");
        assert_eq!(bare_rels, ["co-located"], "{bare}");
        assert_eq!(bare_facts, list_facts, "{bare} differs from {list}");
        assert_eq!(bare_rels, list_rels);
    }
}

/// A relation named by both `:hypothesis-relations` and `:no-hypothesis`
/// yields nothing, and the skip is attributed to the **blacklist**.
///
/// Which counter it lands in is the observable that says the two filters are
/// ordered rather than merely both applied. If the blacklist ran first, the
/// relation would never reach the whitelist test and the skip would be
/// reported as `no_hypothesis_relation` too — so the discriminating assertion
/// is the *other* counter staying at zero, which pins that the whitelist
/// admitted the relation and the blacklist then took it away.
#[test]
fn no_hypothesis_overrides_the_whitelist() {
    let (facts, rels, stats) = candidates(&format!(
        "{TWO_REL}(query :goal (co-located A B) \
         :hypothesis-relations (co-located likes) :no-hypothesis (likes))"
    ));
    assert!(!facts.is_empty());
    assert_eq!(rels, ["co-located"], "likes is excluded");
    assert!(
        stats.pre_candidate[Skip::NoHypothesisRelation as usize] > 0,
        "the blacklist is what skipped it"
    );
    assert_eq!(
        stats.pre_candidate[Skip::RelationNotWhitelisted as usize],
        0,
        "the whitelist admitted it first"
    );
}

// ── What a solution node is — `test_solution.py` ───────────────────

/// A KB is a solution node iff it is **both** consistent and complete.
///
/// The two halves fail independently and each on its own looks like an answer:
/// a consistent KB with candidates left is a partial dead-end, which is what
/// S1.7.3 found being certified as a solution, and a complete KB holding
/// `(false)` is a refuted branch that nothing can extend. `is_solution_node`
/// is the *definition* of an answer in P1.7a — the goal pattern does not
/// decide the verdict — so an implementation that dropped either conjunct
/// would report a `k` the search never justified.
#[test]
fn solution_node_needs_both_halves() {
    // Two objects and a binary relation — the enumerator has candidates.
    let (ast, mut terms, mut kb) = kb_of("(relation r T T)\n(is-a a T) (is-a b T)\n");
    let (consistent, complete, node) = in_session(&ast, &mut terms, &mut kb, |s| {
        (
            ein_infer::consistent(s.kb, s.terms),
            ein_infer::complete(s).expect("generates"),
            ein_infer::is_solution_node(s).expect("generates"),
        )
    });
    assert!(consistent && !complete && !node, "consistent, not complete");

    // No declared relation to hypothesise over — the enumerator yields nothing.
    let (ast, mut terms, mut kb) = kb_of("(is-a a T) (is-a b T)\n");
    let (consistent, complete, node) = in_session(&ast, &mut terms, &mut kb, |s| {
        (
            ein_infer::consistent(s.kb, s.terms),
            ein_infer::complete(s).expect("generates"),
            ein_infer::is_solution_node(s).expect("generates"),
        )
    });
    assert!(consistent && complete && node, "both halves");

    // …and the same KB with a `(false)` in it.
    let (ast, mut terms, mut kb) = kb_of("(is-a a T) (is-a b T)\n(false :source \"(1)\")\n");
    let (consistent, complete, node) = in_session(&ast, &mut terms, &mut kb, |s| {
        (
            ein_infer::consistent(s.kb, s.terms),
            ein_infer::complete(s).expect("generates"),
            ein_infer::is_solution_node(s).expect("generates"),
        )
    });
    assert!(complete && !consistent && !node, "complete, not consistent");
}

// ── Canonical state identity — `test_state_key.py` ─────────────────

fn solve_file(rel: &str, stop_after: Option<u64>) -> (Solved, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut events = Events::off();
    let opts = SolveOptions {
        stop_after,
        ..SolveOptions::default()
    };
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel} solves: {e}"));
    (solved, terms)
}

/// Solution-node identity is the canonical fact list itself, never a digest of
/// it — so two distinct closed models can never merge.
///
/// The Python original forced the failure by monkeypatching the state key into
/// a tuple subclass with a constant hash, so every state landed in one dict
/// bucket. That has no referent here: `state_key` *returns* the sorted fact
/// list and every identity site compares lists, so there is no hash in the
/// path to collide — which is the claim, and the first assertion below is how
/// it is checked rather than argued. The second is the consequence the bug
/// report (REVIEW_M1-01 §1) was written about: under hash-as-identity
/// `branching/04` collapsed to a falsely-certified `Solution`, so `k == 2`
/// with two genuinely different fact sets is the observable that would have
/// caught it.
#[test]
fn state_key_is_identity_not_a_digest() {
    let (solved, terms) = solve_file("examples/branching/04_two_levels.ein", None);
    assert!(solved.stats.exhausted, "the lattice was fully explored");
    assert_eq!(solved.stats.solution_nodes, 2);

    let Answer::Verdict(Verdict::Ambiguity(branches)) = &solved.answer else {
        panic!("expected Ambiguity, got {}", solved.answer.as_str());
    };
    assert_eq!(branches.len(), 2);

    for s in branches {
        let key = state_key(&s.kb);
        let mut facts: Vec<FactId> = s.kb.facts().collect();
        facts.sort_unstable();
        assert_eq!(&*key, &facts[..], "the key is the representation");
    }
    assert_ne!(
        sexprs(&terms, branches[0].kb.facts()),
        sexprs(&terms, branches[1].kb.facts()),
        "two lattice nodes that were never merged hold different facts"
    );
}

/// Two different puzzles' root closures are different states, and their keys
/// differ.
///
/// Both are loaded into **one** `Terms`, so the two key vectors are written in
/// the same alphabet and a difference between them is a difference between the
/// KBs rather than between two interning orders. Loading `zebra2` twice is the
/// control: without it, "the keys differ" would also be satisfied by a key
/// that carried arena-local noise.
#[test]
fn distinct_closures_have_distinct_keys() {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let root = repo_root();
    let mut key_of = |rel: &str| {
        let mut kb = load_file(&mut ast, &mut terms, &root.join(rel)).expect("loads");
        saturate(&ast, &mut terms, &mut kb);
        state_key(&kb)
    };
    let zebra2 = key_of("examples/zebra2.ein");
    let again = key_of("examples/zebra2.ein");
    let minus_15 = key_of("examples/zebra2-minus-15.ein");

    assert!(!zebra2.is_empty());
    assert_eq!(zebra2, again, "one fixture, two runs, one closure");
    assert_ne!(zebra2, minus_15);
}

// ── The blind enumerator's solve — `test_typed_blind_solve.py` ─────

const BLIND: &str = "examples/branching/12_typed_blind_solve.ein";

/// A puzzle with no `(hrule …)` solves on the blind combinatorial path, to
/// exactly one model.
///
/// The M1 gate (`zebra2`) is hrule-driven, so it never enters the blind
/// branch of the generator at all. Without a fixture that both takes that path
/// *and* converges, retiring the kernel `is-a` type-filter would have been a
/// silent change: the demos that run blind abort at `k = 0`, so they cannot
/// witness "still solves". The `hrules.is_empty()` assertion is what keeps
/// this test on the path it names — an hrule added to the fixture would make
/// the solve pass while testing nothing.
#[test]
fn the_blind_path_solves_to_a_single_model() {
    let (solved, _terms) = solve_file(BLIND, Some(1));
    assert_eq!(solved.stats.solution_nodes, 1);
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(_)) => {}
        other => panic!("blind-path fixture must solve, got {}", other.as_str()),
    }

    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(BLIND)).expect("loads");
    assert!(
        kb.program().hrules.is_empty(),
        "the fixture must stay blind-path"
    );
}

/// The recovered model is a total House → Colour bijection, not a partial
/// dead-end certified as complete.
///
/// `complete` is the emptiness test on the open-hypothesis set, so a KB can be
/// "complete" for reasons that have nothing to do with the puzzle being
/// answered — every remaining candidate filtered away, say. Counting the
/// positives and checking both projections is total is what distinguishes a
/// model from a stuck search: three cells, three houses, three colours, and
/// the authored anchor still standing.
#[test]
fn the_blind_model_is_a_total_bijection() {
    let (solved, terms) = solve_file(BLIND, Some(1));
    let Answer::Verdict(Verdict::Solution(s)) = &solved.answer else {
        panic!("expected a Solution, got {}", solved.answer.as_str());
    };
    let color_of: Symbol = terms.syms.get("color-of").expect("interned");
    let cells: Vec<(String, String)> =
        s.kb.facts_of(color_of)
            .map(|f| {
                let args = terms.facts.args(f);
                assert_eq!(args.len(), 2, "color-of is binary");
                let name = |v: Value| terms.sym(v.as_sym().expect("a symbol")).to_string();
                (name(args[0]), name(args[1]))
            })
            .collect();

    assert_eq!(
        cells.len(),
        3,
        "a bijection has exactly 3 positives: {cells:?}"
    );
    let houses: BTreeSet<&str> = cells.iter().map(|(h, _)| h.as_str()).collect();
    let colours: BTreeSet<&str> = cells.iter().map(|(_, c)| c.as_str()).collect();
    assert_eq!(houses, ["H1", "H2", "H3"].into_iter().collect());
    assert_eq!(colours, ["Blue", "Green", "Red"].into_iter().collect());
    assert!(
        cells.contains(&("H1".to_string(), "Red".to_string())),
        "the authored anchor is respected: {cells:?}"
    );
}
