//! Negation as failure at the closure/world boundary — S1.21.8, as law.
//!
//! **T1a.10.2.2.** These replace six Python files, all of them about the same
//! boundary from a different side:
//!
//! | Python original | its side of the boundary |
//! |---|---|
//! | `tests/inference/test_absent_semantics.py` | the probes P1–P8 of [`docs/kernel/inference/absent_semantics.md`](../../../../docs/kernel/inference/absent_semantics.md) — what `(absent P)` *means* |
//! | `tests/inference/test_world_boundary.py` | what the boundary *records*: the negative half of `Deps(Y)` |
//! | `tests/inference/test_saturator_naf.py` | the closure a guard is judged against, including one a rule built |
//! | `tests/inference/monotonic/test_root_stability_naf.py` | C1 — a NAF-derived fork fact is not a truth of root |
//! | `tests/inference/test_naf_deps.py` | the advisory stratification map and its one wired flag |
//! | `tests/inference/test_open.py` | `(open P)`, which is two guards in a trench coat |
//!
//! The one idea underneath all of them: **`absent` is a query about a world,
//! not an atom**. The closure runs purely positive to a fixpoint; every
//! `(absent …)` is lifted out of its disjunct at compile time and asked once,
//! at that fixpoint, of *that* world. Almost every claim below is a corollary
//! — priority stops deciding what is derivable (the closure finished before
//! anyone asked), the answer differs between root and a fork (different
//! worlds), and a fact admitted through a guard depends on an absence that a
//! sibling world does not share (so it must never be merged into root).
//!
//! What is deliberately **not** ported: the Python tests that reached into
//! `compile.split_naf`, `world.project` or `Saturator._parked` to watch the
//! mechanism. ein.rs has the same mechanism under different names, and a test
//! that renamed its way across would pin this build rather than the language.
//! Where such a test protected something observable — a parked candidate is
//! not a stalled saturator, a guard set's `watched` key is complete — the
//! observable is what is asserted here.

use ein_core::{FactId, Kb, NafArg, NafRef, Prov, SolverConfig, Terms, Value};
use ein_infer::commitment::{Kind, try_commitment_set};
use ein_infer::compile::SharedMemo;
use ein_infer::events::{self, Buffer, Events, Level};
use ein_infer::saturator::{Saturator, Session};
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::{Engine, compute_naf_map};
use ein_ir::{Ast, parse};
use std::collections::BTreeSet;

// ── Harness ────────────────────────────────────────────────────────

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

fn load_src(src: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, Some("<naf>")).expect("the fixture parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("the fixture loads");
    (ast, terms, kb)
}

/// Every fact of a KB as sorted s-expressions.
///
/// Sorted and rendered rather than compared by `FactId`, for the reason
/// `search_invariants.rs` gives: an id is an interning artefact, and two KBs
/// that hold the same facts are the claim.
fn facts(kb: &Kb, terms: &Terms) -> Vec<String> {
    let mut v: Vec<String> = kb.facts().map(|f| events::sexpr(terms, f)).collect();
    v.sort();
    v
}

/// A finished saturation: the world it reached, plus the boundary's counters.
struct Run {
    terms: Terms,
    kb: Kb,
    /// How many times the boundary was consulted. **Zero** when nothing was
    /// ever parked — `admit_from_boundary` returns before ticking its clock —
    /// which is what makes "the boundary is never consulted" assertable.
    naf_rounds: u32,
    naf_admitted: u32,
    naf_retired: u32,
    /// Structurally 0 since S1.21.8; asserted, not assumed.
    naf_dropped: u32,
    guard_evals: u64,
}

fn saturate(src: &str) -> Run {
    saturate_with(src, |_, _| {})
}

/// `saturate`, with a hook that may write to the KB before the saturator sees
/// it — the equality-class probe needs a union no surface syntax performs.
fn saturate_with(src: &str, prepare: impl FnOnce(&mut Kb, &mut Terms)) -> Run {
    let (ast, mut terms, mut kb) = load_src(src);
    prepare(&mut kb, &mut terms);
    let mut ev = Events::off();
    let counters = {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("the rules compile");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        (
            sat.naf_rounds,
            sat.naf_admitted,
            sat.naf_retired,
            sat.naf_dropped,
            sat.guard_evals,
        )
    };
    Run {
        terms,
        kb,
        naf_rounds: counters.0,
        naf_admitted: counters.1,
        naf_retired: counters.2,
        naf_dropped: counters.3,
        guard_evals: counters.4,
    }
}

impl Run {
    /// One relation's extent, sorted. Empty when the relation was never even
    /// interned, which is the answer a "nothing was derived" assertion wants.
    fn extent(&self, rel: &str) -> Vec<String> {
        extent_of(&self.kb, &self.terms, rel)
    }

    fn fact(&self, rel: &str, args: &[&str]) -> Option<FactId> {
        find_fact(&self.kb, &self.terms, rel, args)
    }

    /// The negative premises of a fact's **primary** derivation, rendered.
    fn absent_premises(&self, rel: &str, args: &[&str]) -> Vec<String> {
        let f = self
            .fact(rel, args)
            .unwrap_or_else(|| panic!("({rel} {args:?}) was never derived"));
        let p = self.kb.primary(f).expect("a derived fact has a primary record");
        self.terms
            .provs
            .get(p)
            .absent
            .iter()
            .map(|r| naf_sexpr(&self.terms, r))
            .collect()
    }
}

fn extent_of(kb: &Kb, terms: &Terms, rel: &str) -> Vec<String> {
    let Some(r) = terms.syms.get(rel) else {
        return Vec::new();
    };
    let mut v: Vec<String> = kb.facts_of(r).map(|f| events::sexpr(terms, f)).collect();
    v.sort();
    v
}

fn find_fact(kb: &Kb, terms: &Terms, rel: &str, args: &[&str]) -> Option<FactId> {
    let r = terms.syms.get(rel)?;
    let mut vals = Vec::with_capacity(args.len());
    for a in args {
        vals.push(Value::sym(terms.syms.get(a)?));
    }
    let id = terms.probe_fact(r, &vals)?;
    kb.contains(id).then_some(id)
}

/// Intern a fact that need not exist yet — a hypothesis to commit to.
fn intern_fact(terms: &mut Terms, rel: &str, args: &[&str]) -> FactId {
    let r = terms.intern_text(rel).expect("room");
    let vals: Vec<Value> = args
        .iter()
        .map(|a| Value::sym(terms.intern_text(a).expect("room")))
        .collect();
    terms.intern_fact(r, &vals).expect("room")
}

/// A recorded negative premise as `(rel arg …)`, with `_` where the query
/// ranged free. ein.py stores `None` in that position and the distinction is
/// the whole subject of one test below, so it gets a visible spelling.
fn naf_sexpr(terms: &Terms, r: &NafRef) -> String {
    let args: Vec<String> = r.args.iter().map(|a| naf_arg(terms, a)).collect();
    if args.is_empty() {
        format!("({})", terms.sym(r.rel))
    } else {
        format!("({} {})", terms.sym(r.rel), args.join(" "))
    }
}

fn naf_arg(terms: &Terms, a: &NafArg) -> String {
    match a {
        NafArg::Free => "_".to_string(),
        NafArg::Value(v) => terms.display(*v),
        NafArg::Nested { rel, args } => naf_sexpr(
            terms,
            &NafRef {
                rel: *rel,
                args: args.clone(),
            },
        ),
    }
}

// ── C1 — a fork's NAF-derived fact is not a truth of root ──────────

/// The P1.21 R2 probe, verbatim. `(y s)` follows from the **root** fact
/// `(seed s)` plus the absence of `(x a)`, which is exactly why the retired
/// "unconditional fact" walk read it as provably true at root — an `absent`
/// guard contributes no premise, so the fact's positive provenance chain
/// grounds out in root alone.
const PROBE: &str = "\
(rule y-when-no-x ()
  :match  (and (seed ?s) (absent (x a)))
  :assert (y ?s)
  :why    \"no x(a) -> y {?s}\"
  :priority 100)
(rule x-y-clash ()
  :match  (and (x ?o) (y ?s))
  :assert (false)
  :why    \"x and y together are inconsistent\"
  :priority 250)
(relation x T) (relation y T) (relation seed T) (relation g T)
(is-a a T) (is-a s T) (is-a b T)
(seed s)
";

/// **A fact derived through a guard stays in the fork that derived it.**
///
/// Not obvious, because the commitment `{g(b)}` has nothing to do with `x` or
/// `y`: the fork derives `(y s)` for a reason that looks entirely root-local,
/// and a merge policy keyed on "did this derivation use a hypothesis?" would
/// wave it through. The absence it leaned on is the part with no premise to
/// inspect, so the only safe rule is the one the engine implements — root is
/// never written from a fork at all.
#[test]
fn a_naf_derived_fork_fact_never_reaches_root() {
    let (ast, mut terms, mut kb) = load_src(PROBE);
    let before = facts(&kb, &terms);
    let h = intern_fact(&mut terms, "g", &["b"]);
    let mut ev = Events::off();
    let r = try_commitment_set(
        &mut kb,
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
    assert!(
        extent_of(&r.kb, &terms, "y").contains(&"(y s)".to_string()),
        "the fork did not derive the NAF fact, so the test proves nothing"
    );
    assert_eq!(facts(&kb, &terms), before, "root gained a fork's fact");
    assert!(
        find_fact(&kb, &terms, "y", &["s"]).is_none(),
        "(y s) leaked into root"
    );
}

/// **The sibling world that supplies the missing positive is alive without
/// it.**
///
/// This is the refutation, not a second illustration of the first test: a
/// fact true *at root* must hold in every consistent extension of root, and
/// `{x(a)}` is one — it saturates, it is alive, and `(y s)` is not in it.
/// So `(y s)` is not a truth of root, however root-local its premises look.
#[test]
fn an_absent_derived_fact_is_not_a_truth_of_root() {
    let (ast, mut terms, mut kb) = load_src(PROBE);
    let h = intern_fact(&mut terms, "x", &["a"]);
    let mut ev = Events::off();
    let r = try_commitment_set(
        &mut kb,
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h],
        None,
        None,
    )
    .expect("enters");

    assert_eq!(r.kind, Kind::Alive, "the x-world is genuinely consistent");
    assert!(
        find_fact(&r.kb, &terms, "y", &["s"]).is_none(),
        "the x-world holds (y s), so it is not the counterexample it has to be"
    );
}

/// **Merging it would kill a consistent world** — the soundness bug, run.
///
/// The previous test says `(y s)` is not root-true; this one says what it
/// *costs* to write it there anyway, because "unsound in principle" is the
/// kind of claim that survives in a comment while the code quietly stops
/// doing it. Perform the retired merge by hand and the alive `{x(a)}` world
/// flips to `dead-post`: one fewer model, an undercounted `k`, and a verdict
/// that says `Solution` where the truth was `Ambiguity`.
#[test]
fn merging_a_fork_fact_into_root_kills_a_consistent_world() {
    let (ast, mut terms, mut kb) = load_src(PROBE);
    // Root' = root + (y s), under a synthetic rule provenance — exactly the
    // shape the extraction wrote.
    let rule = terms.intern_text("<retired-merge-simulation>").expect("room");
    let y = terms.intern_text("y").expect("room");
    let s_arg = Value::sym(terms.intern_text("s").expect("room"));
    let prov = terms.provs.push(Prov::from_rule(rule, Box::new([]), None));
    kb.add_and_index_fact(&mut terms, y, &[s_arg], Some(prov))
        .expect("room");

    let h = intern_fact(&mut terms, "x", &["a"]);
    let mut ev = Events::off();
    let r = try_commitment_set(
        &mut kb,
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h],
        None,
        None,
    )
    .expect("enters");

    assert_eq!(
        r.kind,
        Kind::DeadPost,
        "the merge did not refute the x-world, so the extraction was harmless \
         after all — which contradicts why it was removed"
    );
}

/// The solve-level fixture. The NAF rule's positive premise is the
/// *hypothesis*, so `(y …)` can only ever arise inside a fork; a root-grounded
/// firing would have happened during Phase-1 saturation and proved nothing.
/// No singleton ever dies and the alive set never becomes one, so the run
/// performs zero *sound* root writes either — which is what lets the assertion
/// be "byte-identical" rather than "identical apart from the writeback".
const SOLVE_FIXTURE: &str = "\
(rule y-when-h-no-x ()
  :match  (and (h ?s ?t) (absent (x a s)))
  :assert (y ?s ?t)
  :why    \"h {?s} {?t} and no x(a,s) -> y {?s} {?t}\"
  :priority 100)
(rule x-y-clash ()
  :match  (and (x ?o ?p) (y ?s ?t))
  :assert (false)
  :why    \"x and y together are inconsistent\"
  :priority 250)
(relation x Thing Thing)
(relation h Thing Thing)
(relation y Thing Thing)
(relation never T)
(is-a Thing T)
(is-a a Thing) (is-a s Thing)

(query
  :goal  (never ?q)
  :hypothesis-relations (x h))
";

/// **A whole search leaves root's fact set untouched.**
///
/// The per-commitment tests above are about one entering; this is the
/// invariant across an exhaustive lattice walk, and it is worth its own test
/// because the search layer *does* write to root — the singleton `(not h)`
/// writeback and the forced-positive cascade are both root writes, and both
/// are sound. The claim is that those are the *only* two: a fork fact is
/// neither, so on a fixture where neither fires, root comes out unchanged.
/// The two counters are what distinguishes "no fork fact leaked" from "no
/// root write happened to run".
#[test]
fn a_naf_search_never_writes_a_fork_fact_to_root() {
    let (ast, mut terms, mut kb) = load_src(SOLVE_FIXTURE);
    let before = facts(&kb, &terms);
    let mut ev = Events::off();
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 4,
        // Every death stays on the `try_commitment_set` path: a lookahead kill
        // cache writes `(not h)` to root, which is a sound write and would
        // muddy a stability check that is about the unsound one.
        config: Some(SolverConfig {
            enable_pre_branch_lookahead: false,
            enable_lookahead_kill_cache: false,
            ..SolverConfig::default()
        }),
        ..SolveOptions::default()
    };
    let solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut ev,
        &mut NoDumper,
        &opts,
    )
    .expect("solves");

    assert!(
        solved.stats.base.enterings_dead_post >= 2,
        "no fork reached the x/y clash, so no fork derived (y …) through the \
         guard and the fixture is not exercising NAF at all"
    );
    assert_eq!(facts(&kb, &terms), before, "root's fact set moved");
    assert!(
        extent_of(&kb, &terms, "y").is_empty(),
        "a NAF-derived fork fact leaked into root"
    );
    assert_eq!(solved.stats.base.facts_merged, 0);
    assert_eq!(solved.stats.base.forced_positives, 0);

    // And the two worlds still say what they said before the search: the
    // x-world is satisfiable against the post-solve root and holds no `y`,
    // and the h-world derives its NAF fact fork-locally.
    let x = intern_fact(&mut terms, "x", &["a", "s"]);
    let memo = SharedMemo::default();
    let rx = try_commitment_set(&mut kb, &mut terms, &ast, &mut ev, &memo, &[x], None, None)
        .expect("enters");
    assert_eq!(rx.kind, Kind::Alive);
    assert!(extent_of(&rx.kb, &terms, "y").is_empty());

    let h = intern_fact(&mut terms, "h", &["a", "s"]);
    let rh = try_commitment_set(&mut kb, &mut terms, &ast, &mut ev, &memo, &[h], None, None)
        .expect("enters");
    assert_eq!(rh.kind, Kind::Alive);
    assert_eq!(extent_of(&rh.kb, &terms, "y"), ["(y a s)"]);
    assert!(
        extent_of(&kb, &terms, "y").is_empty(),
        "the probe itself wrote the fork fact back to root"
    );
}

// ── The guard is judged against the positive fixpoint ──────────────

/// `p ← seed ∧ absent q` and `q ← t`, with the two bands as parameters.
fn gate_program(gate: i64, derive: i64) -> String {
    format!(
        "(rule gate ()\n  :match (and (seed ?x) (absent (q ?x)))\n  \
         :assert (p ?x)\n  :why \"p unless q\" :priority {gate})\n\
         (rule derive-q ()\n  :match (t ?x)\n  :assert (q ?x)\n  \
         :why \"derive q\" :priority {derive})\n\
         (relation seed T)\n(relation t T)\n(relation p T)\n(relation q T)\n\
         (seed A :source \"(1)\")\n(t A :source \"(2)\")\n"
    )
}

/// **Priority no longer decides what is derivable** — P1 and P2 of
/// `absent_semantics.md`, and the same claim `test_saturator_naf.py` made
/// under the name "a guard is judged against the closure fixpoint".
///
/// *(work-list: `priority-does-not-decide-what-is-derivable` merged with
/// `a-guard-is-judged-against-the-closure-fixpoint` — two fixtures, one
/// claim.)*
///
/// This is the flip that S1.21.8 bought, and it is worth stating what the old
/// engine did: it evaluated a guard when the candidate was **dequeued**, so
/// whichever band popped first won. Gate-first gave `{p, q}` — `q` sitting in
/// the closure while `(absent q)` had licensed `p`, which is not a model of
/// anything — and derive-first gave `{q}`. Two answers, one program, chosen by
/// a number the author wrote for scheduling reasons.
///
/// Now both orderings give `{q}`, because the closure is purely positive and
/// runs to a fixpoint before any guard is asked. `naf_dropped == 0` is the
/// mechanical half: the gate is not admitted-then-dropped, it is **never
/// admitted**, and `naf_rounds >= 1` proves the boundary actually ran rather
/// than the candidate quietly failing to match.
#[test]
fn priority_does_not_decide_what_is_derivable() {
    for (gate, derive) in [(100, 200), (200, 100)] {
        let run = saturate(&gate_program(gate, derive));
        assert_eq!(
            run.extent("p"),
            Vec::<String>::new(),
            "gate@{gate}/derive@{derive}: the gate fired although (q A) holds \
             at the positive fixpoint its guard is judged against"
        );
        assert_eq!(run.extent("q"), ["(q A)"]);
        assert_eq!(run.naf_dropped, 0, "a guard was admitted and then dropped");
        assert!(run.naf_rounds >= 1, "the boundary was never consulted");
    }
}

/// **An unstratifiable feedback loop converges, and to a *supported* model.**
///
/// `p ← seed ∧ absent q; q ← p` has **no** stable model: the reduct of
/// `{p, q}` by its own negative literals derives `q` from `p` but never
/// derives `p`, so the candidate set is not reproduced. The engine accepts the
/// program anyway and answers `{p, q}`. That is the honest statement of what
/// Ein computes — a fixpoint supported at the boundary — and it is worth
/// pinning precisely *because* it is the answer a stable-model semantics would
/// refuse to give.
#[test]
fn an_unstratifiable_loop_converges_to_a_supported_model() {
    let run = saturate(
        "(rule derive-p ()\n  :match (and (seed ?x) (absent (q ?x)))\n  \
         :assert (p ?x)\n  :why \"p unless q\" :priority 100)\n\
         (rule derive-q ()\n  :match (p ?x)\n  :assert (q ?x)\n  \
         :why \"q from p\" :priority 100)\n\
         (relation seed T)\n(relation p T)\n(relation q T)\n\
         (seed A :source \"(1)\")\n",
    );
    assert_eq!(run.extent("p"), ["(p A)"]);
    assert_eq!(
        run.extent("q"),
        ["(q A)"],
        "the q that p enabled is missing — something retracted p, and E3 says \
         nothing retracts"
    );
}

/// **The same ground guard answers differently in two worlds** — C6.
///
/// `(absent (r A B))` has no free variables: it is as close to a ground atom
/// as a guard gets, and a naive implementation would be tempted to evaluate it
/// once and cache the answer. It cannot be cached, because the answer is a
/// property of the world asked, not of the query: root derives `gated`, and
/// the fork that committed `(r A B)` — alive, consistent, saturated — does
/// not. This is the reason C1 exists rather than a second illustration of it.
#[test]
fn absent_answers_differently_in_a_fork_than_at_root() {
    let src = "(rule gate ()\n  :match (and (seed ?x) (absent (r A B)))\n  \
               :assert (gated ?x)\n  :why \"gated unless (r A B)\" :priority 100)\n\
               (relation seed T)\n(relation r T T)\n(relation gated T)\n\
               (seed A :source \"(1)\")\n";
    let (ast, mut terms, mut kb) = load_src(src);
    let h = intern_fact(&mut terms, "r", &["A", "B"]);
    let mut ev = Events::off();
    let fork = try_commitment_set(
        &mut kb,
        &mut terms,
        &ast,
        &mut ev,
        &SharedMemo::default(),
        &[h],
        None,
        None,
    )
    .expect("enters");
    assert_eq!(fork.kind, Kind::Alive);
    assert!(
        extent_of(&fork.kb, &terms, "gated").is_empty(),
        "the fork committed (r A B) and still derived the gate"
    );

    // The same program, same query, no commitment.
    let run = saturate(src);
    assert_eq!(run.extent("gated"), ["(gated A)"]);
}

/// **A guard over a conjunction is ¬∃ over the conjunction's free variables.**
///
/// The reading that is *not* implemented: "for each `?y`, no `(g ?x ?y)` and
/// no `(h ?y)`", which would make the guard fail for both seeds. What the
/// definition says is that the guard fails iff **some** extension of the outer
/// bindings satisfies the whole conjunction — so a `g`/`h` witness chained
/// through the shared `?y` blocks `A` and leaves `B` alone. This is also where
/// the `forall` macro's universal comes from: it is this existential, negated
/// twice.
#[test]
fn a_guard_over_a_conjunction_is_a_negated_existential() {
    let run = saturate(
        "(rule gate ()\n  :match (and (seed ?x) (absent (and (g ?x ?y) (h ?y))))\n  \
         :assert (ok ?x)\n  :why \"ok unless a g-h witness exists\" :priority 100)\n\
         (relation seed T)\n(relation g T T)\n(relation h T)\n(relation ok T)\n\
         (seed A :source \"(1)\")\n(seed B :source \"(2)\")\n\
         (g A W :source \"(3)\")\n(h W :source \"(4)\")\n",
    );
    assert_eq!(
        run.extent("ok"),
        ["(ok B)"],
        "A has a witness pair (g A W)+(h W) and B has none, so the guard must \
         fail for A only"
    );
}

/// **A guard inside an `(or …)` disjunct is judged on the boundary too** —
/// divergence D5, which was a confirmed unsound firing.
///
/// The gate matches through the *second* disjunct, whose guard watches `r2`,
/// and another rule derives `(r2 A)` into the quiesced world. The firing must
/// not happen. It used to, and the reason is worth keeping: the disjunct's
/// guard lived in a second collection that the fire-time re-check did not
/// walk. The fix is structural — every match is produced together with **its
/// own disjunct's** guards, so there is no longer a tuple a caller can forget
/// — and `naf_dropped == 0` is what says so: the candidate is never admitted,
/// rather than admitted and then caught.
#[test]
fn an_or_disjuncts_guard_is_judged_on_the_boundary() {
    let run = saturate(
        "(rule gate ()\n  :match (or (and (t1 ?x) (absent (r1 ?x)))\n             \
         (and (t2 ?x) (absent (r2 ?x))))\n  :assert (gated ?x)\n  \
         :why \"gated via either NAF disjunct\" :priority 200)\n\
         (rule derive-r2 ()\n  :match (raw ?x)\n  :assert (r2 ?x)\n  \
         :why \"derive r2\" :priority 100)\n\
         (relation t1 T)\n(relation t2 T)\n(relation r1 T)\n(relation r2 T)\n\
         (relation raw T)\n(relation gated T)\n\
         (t2 A :source \"(1)\")\n(raw A :source \"(2)\")\n",
    );
    assert_eq!(run.extent("r2"), ["(r2 A)"], "precondition: r2 was derived");
    assert!(
        run.extent("gated").is_empty(),
        "(r2 A) is in the quiesced world the guard is judged against, so the \
         second disjunct must not be admitted"
    );
    assert_eq!(run.naf_dropped, 0, "admitted and dropped, not never-admitted");
}

// ── The closure a guard is judged against ──────────────────────────

/// **A guard sees the *closed* extent of the relation it watches.**
///
/// The shape that motivated the whole design. `next-to` is not given: it is
/// derived from `right-of` by an `includes` rule and then closed by a
/// `symmetric` one, so the pairs a guard has to see arrive over several
/// firings. A guard asked before that chain finished would find `(next-to H1
/// H2)` missing and derive `non-neighbour(H1, H2)` — a fact that is simply
/// false about the fixture. Judging at the fixpoint makes the question
/// well-posed: by the time anything is asked, `next-to` is complete, and every
/// candidate is refused.
#[test]
fn a_guard_sees_the_closed_extent_of_its_watched_relation() {
    let run = saturate(
        "(rule symmetric (?rel)\n  :match (?rel ?a ?b)\n  :assert (?rel ?b ?a)\n  \
         :why \"symmetric\" :priority 100)\n\
         (rule includes (?p ?q)\n  :match (?p ?a ?b)\n  :assert (?q ?a ?b)\n  \
         :why \"includes\" :priority 100)\n\
         (rule gate-non-neighbour ()\n  \
         :match (and (anchor ?h1) (other ?h_o) (neq ?h_o ?h1)\n              \
         (absent (next-to ?h_o ?h1)))\n  :assert (non-neighbour ?h_o ?h1)\n  \
         :why \"h_o is not next-to h1\" :priority 250)\n\
         (relation right-of T T)\n(relation next-to T T)\n(relation anchor T)\n\
         (relation other T)\n(relation non-neighbour T T)\n\
         (symmetric next-to)\n(includes right-of next-to)\n\
         (right-of H2 H1 :source \"(1)\")\n(right-of H3 H2 :source \"(2)\")\n\
         (anchor H2 :source \"(3)\")\n\
         (other H1 :source \"(4)\")\n(other H2 :source \"(5)\")\n(other H3 :source \"(6)\")\n",
    );
    assert_eq!(
        run.extent("next-to"),
        [
            "(next-to H1 H2)",
            "(next-to H2 H1)",
            "(next-to H2 H3)",
            "(next-to H3 H2)"
        ],
        "the includes+symmetric chain did not close, so the guard was never \
         asked the hard question"
    );
    assert!(
        run.extent("non-neighbour").is_empty(),
        "H1 and H3 are both next-to H2 by derivation, and (H2,H2) is excluded \
         by neq — so every candidate must be refused"
    );
}

// ── What a boundary-admitted firing records ────────────────────────

/// **A firing admitted through the boundary records what had to be absent** —
/// C2, the negative half of `Deps(Y)`.
///
/// *(work-list: `boundary-admitted-firing-records-its-negative-premises`, which
/// covers both the guarded and the purely positive case.)*
///
/// Before S1.21.8 this was invisible: a guard contributed no premise, so the
/// provenance of `(ok A)` named `(seed A)` and stopped — and every walk over
/// it, including the unsat-core minimiser, believed `(ok A)` depended on that
/// one fact. It depends on an *absence* too, and the pair of assertions here
/// is what makes the record meaningful: a guarded firing records the query,
/// and a purely positive one records nothing, so the field distinguishes the
/// two rather than being decoration on both.
#[test]
fn a_boundary_admitted_firing_records_what_was_absent() {
    let guarded = saturate(
        "(rule gate ()\n  :match (and (seed ?x) (absent (r ?x)))\n  \
         :assert (ok ?x)\n  :why \"g\" :priority 100)\n\
         (relation seed T)\n(relation r T)\n(relation ok T)\n\
         (seed A :source \"(1)\")\n",
    );
    assert_eq!(guarded.extent("ok"), ["(ok A)"]);
    assert_eq!(
        guarded.absent_premises("ok", &["A"]),
        ["(r A)"],
        "the guard's projected bindings must be substituted in: the question \
         asked was `no (r A)`, not `no r at all`"
    );

    let plain = saturate(
        "(rule chain ()\n  :match (seed ?x)\n  :assert (ok ?x)\n  \
         :why \"c\" :priority 100)\n\
         (relation seed T)\n(relation ok T)\n(seed A :source \"(1)\")\n",
    );
    assert!(
        plain.absent_premises("ok", &["A"]).is_empty(),
        "a purely positive firing recorded a negative premise, so the field \
         says nothing about whether negation was involved"
    );
}

/// **A position the guard left free is recorded as free, not as a binding.**
///
/// The distinction is the difference between two different questions, and a
/// consumer of the record has to be able to tell them apart. `(absent (r ?x
/// ?y))` under `?x = A` asks "is there no `(r A _)` **at all**?" — one query
/// over the whole `?y` column. Recording it as `(r A y)`, with whatever `?y`
/// happened to hold, would claim the firing depends on one missing tuple when
/// it depends on the column being empty; a sound deletion-based minimiser
/// built on that record would preserve the wrong thing.
#[test]
fn a_free_position_in_a_negative_premise_is_recorded_as_free() {
    let run = saturate(
        "(rule gate ()\n  :match (and (seed ?x) (absent (r ?x ?y)))\n  \
         :assert (ok ?x)\n  :why \"g\" :priority 100)\n\
         (relation seed T)\n(relation r T T)\n(relation ok T)\n\
         (seed A :source \"(1)\")\n",
    );
    assert_eq!(run.extent("ok"), ["(ok A)"]);
    assert_eq!(run.absent_premises("ok", &["A"]), ["(r A _)"]);
}

/// **An alternative justification carries *its own* negative premises.**
///
/// Provenance is per derivation (S1.21.7), so a fact believed for two reasons
/// is an OR-node over two AND-nodes — and the two need not have the same
/// dependencies. Here `(target A)` is derived once by a purely positive rule
/// and once through a guard, and only the second depends on `(block A)` being
/// missing. The re-derivation path is the highest-volume path in the engine
/// and the easy mistake is to record the alternative from the plan alone,
/// which silently drops the negative half of exactly the derivation that had
/// one.
#[test]
fn an_alternative_justification_carries_its_own_negative_premises() {
    let run = saturate(
        "(rule r-pos ()\n  :match (other ?x)\n  :assert (target ?x)\n  \
         :why \"p\" :priority 100)\n\
         (rule r-naf ()\n  :match (and (seed ?x) (absent (block ?x)))\n  \
         :assert (target ?x)\n  :why \"n\" :priority 500)\n\
         (relation seed T)\n(relation other T)\n(relation block T)\n\
         (relation target T)\n\
         (seed A :source \"(1)\")\n(other A :source \"(2)\")\n",
    );
    let target = run.fact("target", &["A"]).expect("derived");
    let mut by_rule: Vec<(String, Vec<String>)> = run
        .kb
        .justifications(target)
        .into_iter()
        .map(|p| {
            let rec = run.terms.provs.get(p);
            let rule = rec
                .rule
                .map(|r| run.terms.sym(r).to_string())
                .unwrap_or_default();
            let absent = rec
                .absent
                .iter()
                .map(|n| naf_sexpr(&run.terms, n))
                .collect();
            (rule, absent)
        })
        .collect();
    by_rule.sort();
    assert_eq!(
        by_rule,
        vec![
            ("r-naf".to_string(), vec!["(block A)".to_string()]),
            ("r-pos".to_string(), Vec::<String>::new()),
        ],
        "the two justifications must differ in exactly their negative half"
    );
}

/// **A nested guard records both levels of the query it ran.**
///
/// `(forall ?q G B)` desugars to `(absent (and G (absent B)))`, and what had
/// to fail is the **whole** query — there is no `(player …)` that both
/// qualifies and is unbeaten. Recording only the outer scan would say
/// `undefeated(Alice)` depends on `player` and not on `beats`, which inverts
/// the reading: the fact is fragile precisely because a new `beats` row can
/// destroy it. Both relations have to appear or the record is worse than
/// absent, because it looks complete.
#[test]
fn a_nested_guard_records_both_levels() {
    let run = saturate(
        "(import std.macro :symbols (forall))\n\
         (rule undefeated ()\n  :match (and (player ?p)\n              \
         (forall ?q (and (player ?q) (neq ?p ?q)) (beats ?p ?q)))\n  \
         :assert (undefeated ?p)\n  :why \"u\" :priority 100)\n\
         (relation player T)\n(relation beats T T)\n(relation undefeated T)\n\
         (player Alice :source \"(1)\")\n(player Bob :source \"(2)\")\n\
         (beats Alice Bob :source \"(3)\")\n",
    );
    assert_eq!(run.extent("undefeated"), ["(undefeated Alice)"]);
    let mut rels: Vec<String> = run
        .absent_premises("undefeated", &["Alice"])
        .iter()
        .map(|s| {
            s.trim_start_matches('(')
                .split([' ', ')'])
                .next()
                .unwrap_or("")
                .to_string()
        })
        .collect();
    rels.sort();
    rels.dedup();
    assert_eq!(
        rels,
        ["beats", "player"],
        "an `undefeated` fact depends on both the scan that found the rivals \
         and the nested absence of a win over them"
    );
}

/// **`(or …)` is commutative even when its disjuncts produce the same
/// bindings** — the S1.22.0 regression, in the only shape that exposes it.
///
/// Both disjuncts scan `(a ?x)` and so bind `?x = X` identically; only their
/// guards differ, and only one passes. The saturator dedups candidates by
/// their bindings, so if the guards are not part of that key the two collide,
/// the first is kept — and because a failed monotone guard *retires* its
/// candidate rather than re-asking it, the surviving disjunct is masked
/// permanently. The symptom is that the rule fires or not depending on the
/// order the author wrote the disjuncts in, which no reading of `or` allows.
#[test]
fn or_is_commutative_when_its_disjuncts_share_bindings() {
    let program = |d1: &str, d2: &str| {
        format!(
            "(rule gate ()\n  :match (or {d1} {d2})\n  :assert (gated ?x)\n  \
             :why \"either\" :priority 100)\n\
             (relation a T)\n(relation block T)\n(relation other T)\n\
             (relation gated T)\n\
             (a X :source \"(1)\")\n(block X :source \"(2)\")\n"
        )
    };
    let blocked = "(and (a ?x) (absent (block ?x)))";
    let free = "(and (a ?x) (absent (other ?x)))";
    for (d1, d2) in [(blocked, free), (free, blocked)] {
        let run = saturate(&program(d1, d2));
        assert_eq!(
            run.extent("gated"),
            ["(gated X)"],
            "(other X) is absent, so the other-guarded disjunct must fire \
             whichever order the two are written in"
        );
    }
}

/// **A saturator holding a passing parked candidate is not stalled.**
///
/// "Stalled" is not "the positive queue is empty" — at the boundary design's
/// core, the positive queue is *always* empty when a guard is about to decide
/// something. A saturator that answered from the queue alone would report
/// quiescence one round early and lose the firing entirely, and every caller
/// that drives `step` to a fixpoint (the fork loop, the lookahead, the
/// contradiction detector) would see a world one admission short of the real
/// one. So the question has to reach the boundary, and after the boundary has
/// been exhausted the same question has to answer the other way.
#[test]
fn the_boundary_decides_whether_a_saturator_is_stalled() {
    let (ast, mut terms, mut kb) = load_src(
        "(rule gate ()\n  :match (and (seed ?x) (absent (r ?x)))\n  \
         :assert (ok ?x)\n  :why \"g\" :priority 100)\n\
         (relation seed T)\n(relation r T)\n(relation ok T)\n\
         (seed A :source \"(1)\")\n",
    );
    let mut ev = Events::off();
    let mut s = Session {
        kb: &mut kb,
        terms: &mut terms,
        ast: &ast,
        events: &mut ev,
        memo: SharedMemo::default(),
    };
    let mut sat = Saturator::new(&mut s).expect("compiles");
    assert!(
        !sat.is_stalled(&mut s).expect("asks"),
        "nothing is in the positive queue, but a parked candidate whose guard \
         passes is an available firing"
    );
    sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
    assert!(sat.is_stalled(&mut s).expect("asks"));
    assert_eq!(
        extent_of(s.kb, s.terms, "ok"),
        ["(ok A)"],
        "the firing the early `stalled` would have lost"
    );
}

/// **Matching is raw structural equality; it does not resolve equality
/// classes.**
///
/// This looks like a limitation and is load-bearing. The boundary skips
/// re-judging a parked candidate when no relation its guard reads has *grown*,
/// and that shortcut is only valid while a query's match set is a function of
/// the stored tuples: if `A` and `C` were interchangeable to the matcher, a
/// `union(A, C)` would change a guard's verdict without changing any extent
/// size, and the candidate would never be re-asked. So the test asserts both
/// halves — the class really is recorded, and the matcher really ignores it —
/// because the day the second half changes, the boundary's stamp needs
/// redesigning.
#[test]
fn matching_does_not_resolve_equality_classes() {
    let mut equivalent = false;
    let run = saturate_with(
        "(rule scan ()\n  :match (r C ?y)\n  :assert (found ?y)\n  \
         :why \"s\" :priority 100)\n\
         (rule gate ()\n  :match (and (seed ?x) (absent (r C ?y)))\n  \
         :assert (ok ?x)\n  :why \"g\" :priority 200)\n\
         (relation r T T)\n(relation seed T)\n(relation found T)\n(relation ok T)\n\
         (r A B :source \"(1)\")\n(seed S :source \"(2)\")\n",
        |kb, terms| {
            let a = terms.intern_text("A").expect("room");
            let c = terms.intern_text("C").expect("room");
            kb.classes().union(a, c);
            equivalent = kb.classes().equivalent(a, c);
        },
    );
    assert!(equivalent, "the union did not take, so nothing is being tested");
    assert!(
        run.extent("found").is_empty(),
        "a scan for (r C ?y) matched the stored (r A B) — the unifier now \
         resolves eq-classes, and the boundary's extent-size stamp no longer \
         implies match-set equality"
    );
    assert_eq!(
        run.extent("ok"),
        ["(ok S)"],
        "and the same blindness applies to a negative query: (absent (r C ?y)) \
         must pass"
    );
}

/// **A guard over a nested `(not …)` pattern watches `not` and nothing else.**
///
/// The invalidation key of `(absent (not (r ?x ?y)))` is the single relation
/// `not`, which reads like an under-approximation — surely a query mentioning
/// `r` depends on `r`? It does not, and the reason is a storage fact: a stored
/// `(not …)` holds its inner pattern as one frozen argument, so adding rows to
/// `r` cannot create, change or remove any `not` fact. The guard's match set
/// moves only when `not` itself grows. Growing `r` under the guard's feet and
/// getting the same answer is what makes that argument checkable rather than
/// merely plausible.
#[test]
fn a_guard_over_a_nested_pattern_watches_the_outer_relation_only() {
    let program = |extra: &str| {
        format!(
            "(rule gate ()\n  :match (and (seed ?x) (absent (not (r ?x ?y))))\n  \
             :assert (ok ?x)\n  :why \"g\" :priority 100)\n\
             (relation seed T)\n(relation r T T)\n(relation ok T)\n\
             (seed X :source \"(1)\")\n(seed Y :source \"(2)\")\n\
             (not (r X B) :source \"(3)\")\n{extra}"
        )
    };
    let bare = saturate(&program(""));
    assert_eq!(
        bare.extent("ok"),
        ["(ok Y)"],
        "X has a stored (not (r X B)) and Y has none"
    );

    let grown = saturate(&program(
        "(r X Q :source \"(4)\")\n(r Z W :source \"(5)\")\n(r Y P :source \"(6)\")\n",
    ));
    assert_eq!(grown.extent("r").len(), 3, "precondition: `r` really grew");
    assert_eq!(
        grown.extent("ok"),
        bare.extent("ok"),
        "rows added to `r` changed the guard's answer, so keying it on `not` \
         alone is incomplete"
    );
}

/// **A rule parameter is in scope for its guards when they are judged.**
///
/// A parameterised rule's `?P` is bound from the activator fact, and for a
/// relation slot the compiler can substitute it away. A *predicate* inside a
/// guard cannot be substituted away: `(eq ?y ?P)` is compiled from the raw
/// source, so if `?P` is not in the guard's scope at judgement time it
/// resolves to nothing, the negative query finds no match, and the guard
/// **passes when it had to fail**. That failure mode is silent — a rule that
/// fires too often, not an error — so the test is written as a pair: the same
/// rule must refuse `BAD` and admit `GOOD`, which is only possible if `?P` was
/// actually consulted.
#[test]
fn a_rule_parameter_is_in_scope_for_its_guards() {
    let program = |stored: &str| {
        format!(
            "(rule gate (?P)\n  \
             :match (and (trigger ?a) (absent (and (r ?a ?y) (eq ?y ?P))))\n  \
             :assert (gated ?a)\n  :why \"g\" :priority 200)\n\
             (relation trigger T)\n(relation r T T)\n(relation gated T)\n\
             (relation gate T)\n\
             (gate BAD :source \"activator\")\n(trigger X :source \"(1)\")\n\
             (r X {stored} :source \"(2)\")\n"
        )
    };
    let blocked = saturate(&program("BAD"));
    assert!(
        blocked.extent("gated").is_empty(),
        "(r X BAD) with ?P = BAD matches the guard's query, so the firing must \
         be refused — a passing guard here means ?P resolved to nothing"
    );
    let admitted = saturate(&program("GOOD"));
    assert_eq!(
        admitted.extent("gated"),
        ["(gated X)"],
        "with (r X GOOD) the `(eq ?y ?P)` filter rejects the only row, so the \
         guard passes — without this half the test above would also pass if \
         the guard simply never fired"
    );
}

/// **A rule with no `(absent …)` never consults the boundary.**
///
/// The two-phase loop has to be free for the programs that do not use it, and
/// "free" is checkable rather than a claim about the code: a purely positive
/// program parks nothing, and the boundary's clock never ticks — no round, no
/// guard evaluation. Adding one guard to the same program moves both counters
/// off zero and admits exactly one candidate. Without the second half this
/// would pass on an engine whose boundary had been deleted.
#[test]
fn a_guard_free_rule_parks_nothing() {
    let plain = saturate(
        "(rule plain ()\n  :match (seed ?x)\n  :assert (ok ?x)\n  \
         :why \"p\" :priority 100)\n\
         (relation seed T)\n(relation ok T)\n(seed A :source \"(1)\")\n",
    );
    assert_eq!(plain.extent("ok"), ["(ok A)"], "the rule still fires");
    assert_eq!(plain.naf_rounds, 0, "the boundary was consulted anyway");
    assert_eq!(plain.guard_evals, 0);
    assert_eq!(plain.naf_admitted, 0);
    assert_eq!(plain.naf_retired, 0);

    let guarded = saturate(
        "(rule gate ()\n  :match (and (seed ?x) (absent (r ?x)))\n  \
         :assert (ok ?x)\n  :why \"g\" :priority 100)\n\
         (relation seed T)\n(relation r T)\n(relation ok T)\n(seed A :source \"(1)\")\n",
    );
    assert_eq!(guarded.extent("ok"), ["(ok A)"]);
    assert!(guarded.naf_rounds >= 1, "one guard and still no boundary round");
    assert!(guarded.guard_evals >= 1);
    assert_eq!(guarded.naf_admitted, 1);
}

// ── `(open P)` — the third state ───────────────────────────────────

/// **`(open P)` admits exactly the pairs that are neither asserted nor
/// negated.**
///
/// `open` is not a primitive: the macro expands it to `(and (absent P) (absent
/// (not P)))`, two guards over the same pattern, and the interesting part is
/// that it therefore inherits the boundary's semantics wholesale. What it buys
/// is the three-valued reading a KB with explicit negation needs — asserted,
/// denied, undecided — and the test covers all three states plus the degenerate
/// case where the relation's extent is empty and *every* pair is undecided,
/// which is the state a puzzle starts in and the one a hypothesis generator
/// enumerates over.
#[test]
fn open_admits_exactly_the_undecided_pairs() {
    let rule = "(import std.macro :symbols (open))\n\
                (rule find-open ()\n  \
                :match (and (is-a ?a Person) (is-a ?b Person) (neq ?a ?b)\n              \
                (open (likes ?a ?b)))\n  :assert (open-likes ?a ?b)\n  \
                :why \"{?a} to {?b} undecided\" :priority 100)\n\
                (relation likes T T)\n(relation open-likes T T)\n\
                (is-a Person T)\n";

    let three = saturate(&format!(
        "{rule}(is-a Alice Person)\n(is-a Bob Person)\n(is-a Carol Person)\n\
         (likes Alice Bob :source \"(1)\")\n(not (likes Alice Carol) :source \"(2)\")\n"
    ));
    assert_eq!(
        three.extent("open-likes"),
        [
            "(open-likes Bob Alice)",
            "(open-likes Bob Carol)",
            "(open-likes Carol Alice)",
            "(open-likes Carol Bob)",
        ],
        "(Alice Bob) is asserted and (Alice Carol) is denied; every other \
         ordered pair is undecided, and `neq` removes the diagonal"
    );

    let empty = saturate(&format!(
        "{rule}(is-a Alice Person)\n(is-a Bob Person)\n"
    ));
    assert_eq!(
        empty.extent("open-likes"),
        ["(open-likes Alice Bob)", "(open-likes Bob Alice)"],
        "with no `likes` fact of either polarity, both guards pass for every \
         pair — the state a puzzle starts in"
    );
}

// ── The advisory stratification map ────────────────────────────────

const DERIVED_NAF: &str = "\
(rule probe ()
  :match  (and (seed ?a ?b) (absent (target ?a ?b)))
  :assert (out ?a ?b) :why \"p\" :priority 100)
(rule mk-target ()
  :match  (src ?a ?b)
  :assert (target ?a ?b) :why \"m\" :priority 100)
(relation seed   T T)
(relation target T T)
(relation out    T T)
(relation src    T T)
(seed A B)
(query :goal (out ?x ?y))
";

/// **`warn-derived-naf` is what turns the stratification advisory on, and it
/// is off by default.**
///
/// The distinction the warning draws — a guard watching a *rule-derived*
/// relation, as against one whose extension the puzzle fixes — stopped being a
/// soundness signal at S1.21.8 and became a stratification one: only derived
/// NAF can make a rule set non-stratifiable, and on such a program the engine
/// reports one model without saying that others exist. It stays off by
/// default because it is advice about a program, not a diagnosis of a bug, and
/// a default-on advisory on every zebra rule is noise. Both halves are
/// asserted, because a flag that warns unconditionally is as wrong as one that
/// never does.
#[test]
fn warn_derived_naf_gates_the_solve_time_warning() {
    let stream = |warn: bool| {
        let (ast, mut terms, mut kb) = load_src(DERIVED_NAF);
        let buf = Buffer::new();
        let mut ev = Events::to(Box::new(buf.clone()), Level::Normal);
        let opts = SolveOptions {
            stop_after: Some(1),
            config: Some(SolverConfig {
                warn_derived_naf: warn,
                ..SolverConfig::default()
            }),
            ..SolveOptions::default()
        };
        solve(&mut kb, &mut terms, &ast, &mut ev, &mut NoDumper, &opts).expect("solves");
        buf.to_string_lossy()
    };

    let on = stream(true);
    let warnings: Vec<&str> = on
        .lines()
        .filter(|l| l.contains("DerivedNafWarning"))
        .collect();
    assert_eq!(warnings.len(), 1, "expected one warning, got:\n{on}");
    assert!(
        warnings[0].contains("probe") && warnings[0].contains("target"),
        "the warning names neither the rule nor the watched relation, so it \
         cannot be acted on: {}",
        warnings[0]
    );

    let off = stream(false);
    assert!(
        !off.contains("DerivedNafWarning"),
        "the advisory fired with the flag at its default"
    );
}

/// **The NAF map is incomplete before saturation, which is why the warning
/// site is post-saturation.**
///
/// A rule is compiled once per `(rule, activator)` pair, and most NAF-bearing
/// rules in the zebra family are activated by facts that do not exist at load
/// — `adjacent-via-*` by a derived `next-to`, the elimination and totality
/// rules by their own derived companions. Their plans therefore do not exist
/// yet, so a map read off a freshly compiled cache silently omits exactly the
/// rules the advisory is about. Asserting a strict subset rather than a count
/// is deliberate: the claim is that the load-time map *misses* rules, and
/// nothing about it should be able to gain one.
#[test]
fn the_naf_map_is_incomplete_before_saturation() {
    let path = repo_root().join("examples/zebra2.ein");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = ein_ir::load_file(&mut ast, &mut terms, &path).expect("zebra2 loads");
    let mut ev = Events::off();

    let mut engine = Engine::new();
    engine
        .compile_all(&ast, &mut terms, &kb, &mut ev)
        .expect("compiles");
    let at_load: BTreeSet<String> = flagged(&engine, &terms);

    let saturated: BTreeSet<String> = {
        let mut s = Session {
            kb: &mut kb,
            terms: &mut terms,
            ast: &ast,
            events: &mut ev,
            memo: SharedMemo::default(),
        };
        let mut sat = Saturator::new(&mut s).expect("compiles");
        sat.saturate(&mut s, None, &mut |_| {}).expect("saturates");
        flagged(&sat.engine, s.terms)
    };

    eprintln!("AT_LOAD {at_load:?}\nSATURATED {saturated:?}");
    assert!(
        at_load.is_subset(&saturated) && at_load != saturated,
        "the load-time map is not a strict subset of the saturated one:\n  \
         load-time: {at_load:?}\n  saturated: {saturated:?}"
    );
    for rule in ["adjacent-via-fwd", "total", "domain-elimination"] {
        assert!(
            !at_load.contains(rule),
            "{rule} has a rule-derived activator and cannot have a plan at load"
        );
        assert!(saturated.contains(rule), "{rule} missing after saturation");
    }
}

/// The rules whose `(absent …)` watches a rule-derived relation, by name.
fn flagged(engine: &Engine, terms: &Terms) -> BTreeSet<String> {
    compute_naf_map(engine, terms)
        .into_iter()
        .filter(|d| !d.derived.is_empty())
        .map(|d| terms.sym(d.rule).to_string())
        .collect()
}
