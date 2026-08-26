//! M1d S1d.2.5 — **hypotheses from obligations, asserted.**
//!
//! [`obligation_reports.rs`](obligation_reports.rs)'s sibling one stage on.
//! That suite checks what a quiescent state *says* it owes; this one checks
//! that the debt is a **choice point** — that the ladder dispatches where it
//! should, that the branch is the obligation's own domain scan, and that the
//! search it drives finds the same models as the hand-written `(hrule …)` it
//! replaces.
//!
//! The central claim is `the_theory_finds_what_the_hrule_finds`, and it is a
//! *model-set* comparison rather than a counter one on purpose: counters may
//! move under a new traversal (Q-M1d.4 was spent on exactly that), answers may
//! not. That they did **not** move either is the stage's measurement, banked
//! in [the record], not asserted here — a counter equality pinned as a test
//! would be a test of the fixtures' arithmetic rather than of the engine.
//!
//! [the record]: `docs/history/m1d_satisfiability/hypotheses_from_obligations.md`

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::oblgen::{Choice, Mode};
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_ir::{Ast, load_file};

/// Solve one file to `depth`, optionally narrating.
fn run_with(rel: &str, events: &mut Events, depth: u32) -> (Ast, Terms, Kb, Solved) {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let opts = SolveOptions {
        config: Some(kb.program().config.clone().unwrap_or_default()),
        max_set_size: depth,
        stop_after: None,
        ..SolveOptions::default()
    };
    let solved = solve(&mut kb, &mut terms, &ast, events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel}: {e:?}"));
    (ast, terms, kb, solved)
}

/// Every model of a run, as a set of rendered fact sets.
///
/// Text rather than `FactId`, because two runs intern independently and the
/// question is whether they found the same *models* — which is a statement
/// about propositions, not about numbering.
fn models(terms: &Terms, solved: &Solved) -> BTreeSet<BTreeSet<String>> {
    let one = |kb: &Kb| -> BTreeSet<String> {
        kb.facts()
            .map(|f| ein_infer::events::sexpr(terms, f))
            .collect()
    };
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(s)) => BTreeSet::from([one(&s.kb)]),
        Answer::Verdict(Verdict::Ambiguity(v)) => v.iter().map(|s| one(&s.kb)).collect(),
        _ => BTreeSet::new(),
    }
}

/// The distinct `rung` lines a run narrated, in first-seen order.
fn rungs(rel: &str, depth: u32) -> Vec<(String, u64, u64, u64)> {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let _ = run_with(rel, &mut events, depth);
    let mut out: Vec<(String, u64, u64, u64)> = Vec::new();
    for line in buffer.to_string_lossy().lines() {
        let ev: serde_json::Value = serde_json::from_str(line).expect("event line");
        if ev["e"] != "rung" {
            continue;
        }
        let row = (
            ev["mode"].as_str().unwrap_or("?").to_string(),
            ev["owed"].as_u64().unwrap_or_default(),
            ev["branches"].as_u64().unwrap_or_default(),
            ev["declined"].as_u64().unwrap_or_default(),
        );
        if !out.contains(&row) {
            out.push(row);
        }
    }
    out
}

// ── The ladder ─────────────────────────────────────────────────────

/// **The dispatch, one file per rung.**
///
/// `(hrule …)` presence is still the switch it has always been — the override
/// rung — and the two rungs below it are the stage's addition. The fourth row
/// is the one that keeps the corpus still: a program that declares no
/// obligation rule narrates **nothing**, which is why every pre-M1d event
/// stream is byte-identical.
#[test]
fn the_ladder_dispatches_by_what_the_program_declares() {
    // (file, the modes its generation calls report)
    let cases: [(&str, &[&str]); 5] = [
        // Rung 1 — and it narrates nothing, exactly as rung 3 does, which is
        // what keeps every pre-M1d stream byte-identical. The claim that this
        // is an *override* rather than an absence of debt is the assertion
        // below: `zebra2.ein` owes 36 at root and takes the hrule path anyway.
        ("examples/zebra2.ein", &[]),
        // the same puzzle with the hrule deleted: rung 2.
        ("examples/zebra2-obligations.ein", &["obligations"]),
        // owes, and `:no-hypothesis` names the relation it owes.
        ("tests/stdlib/algebra/23_total_owed.ein", &["stuck"]),
        // an obligation whose guard scans a relation the rung proposes.
        (
            "tests/stdlib/bijection/06_blind_enumeration.ein",
            &["declined"],
        ),
        // no obligation rule at all — rung 3, and not a word about it.
        ("examples/branching/02_one_dead_one_alive.ein", &[]),
    ];
    for (rel, want) in cases {
        // Depth 1: the rung is decided at the root generation call, and the
        // layers below cost seconds to re-confirm it.
        let got: Vec<String> = rungs(rel, 1).into_iter().map(|(m, ..)| m).collect();
        let uniq: BTreeSet<&str> = got.iter().map(String::as_str).collect();
        let want: BTreeSet<&str> = want.iter().copied().collect();
        assert_eq!(uniq, want, "{rel} took the wrong rung(s)");
    }
    // The override, stated as the thing it overrides: zebra2 owes 36 slots at
    // root — the same 36 `zebra2-obligations.ein` branches on — and its hrule
    // is why nothing above reports a rung for it.
    let (_, _, _, solved) = run_with("examples/zebra2.ein", &mut Events::off(), 1);
    assert_eq!(
        solved.owes.root.total(),
        36,
        "zebra2 owes what its twin owes"
    );
}

/// **A state that owes something it may not branch on is reported, not
/// silently complete.**
///
/// The trap the ladder's second rung opens: the generator proposes nothing, so
/// `complete()` says yes, so the node is recorded as a model — while the tally
/// beside it says the requirement is unmet. Ten corpus programs are in exactly
/// that state today, every one of them because `:no-hypothesis` names the
/// relation they owe, and the `stuck` line is what tells them apart from a
/// state that is finished. **That trap is closed**: since S1d.2.6 such a node
/// is not recorded as a model by the read-out at all.
///
/// **The word moved at [S1d.2.6]**, which is what this stage's evidence bought:
/// a stuck state reports `Open`, not `Solution`. The two halves stay separate
/// on purpose — `stuck` is the *generator's* report (it proposed nothing and
/// says why) and `Open` is the *read-out's* (the state is not discharged) —
/// and this test asserts both, because a regression in either one alone would
/// leave the other still true.
///
/// [S1d.2.6]: `docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus`
#[test]
fn owing_and_unable_to_branch_is_stuck_and_says_so() {
    let rel = "tests/stdlib/algebra/23_total_owed.ein";
    let rows = rungs(rel, 1);
    assert_eq!(
        rows,
        [("stuck".to_string(), 1, 0, 1)],
        "{rel} owes one and branches on none"
    );
    let (_, _, _, solved) = run_with(rel, &mut Events::off(), 1);
    assert_eq!(
        solved.answer.as_str(),
        "Open",
        "a stuck state is not a model — S1d.2.6"
    );
    assert_eq!(
        solved.owes.root.total(),
        1,
        "and it still says what it owes"
    );
}

/// **A discharged state proposes nothing, and that is what completes it.**
///
/// The other half of the same question: `24_total_owed_satisfied.ein` is
/// `23`'s twin with the witness present. Rung 2 runs, owes nothing, proposes
/// nothing — and *not* because anything was scoped out. The pair is what makes
/// "complete means discharged" a distinction rather than a coincidence, since
/// both files report `k = 1` and only one of them owes.
#[test]
fn a_discharged_state_owes_nothing_and_declines_nothing() {
    let rows = rungs("tests/stdlib/algebra/24_total_owed_satisfied.ein", 1);
    assert_eq!(rows, [("obligations".to_string(), 0, 0, 0)]);
}

// ── The branch ─────────────────────────────────────────────────────

/// **The branch is the obligation's own domain scan, and nothing else.**
///
/// `zebra2-obligations` at root owes 36 instances — 36 of the 50 slots five
/// bijections need, the other 14 already witnessed — and proposes 180
/// candidates for them, which the filter pipeline dedups and refutes down to
/// the 56 the hrule path emits. The numbers matter in one direction each:
/// `branches == owed` says nothing was declined, and `candidates` well under
/// `owed × |House|` would say a scan came back short.
#[test]
fn the_candidates_are_what_the_guard_scans() {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let _ = run_with("examples/zebra2-obligations.ein", &mut events, 1);
    let first = buffer
        .to_string_lossy()
        .lines()
        .map(|l| serde_json::from_str::<serde_json::Value>(l).expect("event line"))
        .find(|e| e["e"] == "rung")
        .expect("a rung line");
    assert_eq!(first["mode"], "obligations");
    assert_eq!(first["owed"], 36);
    assert_eq!(first["branches"], 36, "nothing was declined at root");
    // Five houses per owed slot: `total-owed` scans House for an unlocated
    // value, `surjective-owed` scans the value type for an unfilled house, and
    // both types have five members.
    assert_eq!(first["candidates"], 180);
    // is-a, is-a*, right-of, next-to — the four relations no obligation names,
    // which is the ladder's completeness condition as a number.
    assert_eq!(first["uncovered"], 4);
}

// ── The completeness condition ─────────────────────────────────────

/// **Every model the hrule path finds, the obligation path finds.**
///
/// T1d.2.5.3, and the phase's central claim: the ladder is exhaustive iff
/// obligations and saturation between them determine every remaining open
/// fact. On the zebra family the obligated arrows *are* the decision
/// variables, so the two paths must agree — and the pair below is one
/// determinate puzzle and one genuinely under-determined one, because a
/// comparison of two singleton sets proves much less.
///
/// `zebra2-minus-15` runs to depth 3, which is where all 32 of its models are
/// ([the layer census](../../../../docs/history/m1d_satisfiability/layer_census.md)
/// §4); depths 4 and 5 exist only to prove there are no more, cost 416 s, and
/// prove nothing this test is asking about.
#[test]
fn the_theory_finds_what_the_hrule_finds() {
    // Depth 3 is where the last four of `zebra2-minus-15`'s 32 models are, and
    // it costs 26 s a side. Depth 2 finds 28 of them for 1.6 s, which is the
    // same claim about 28 models — so the gate runs that and `EIN_CORPUS_SLOW`
    // runs the whole set, the way the corpus sweep splits the same way.
    let deep = std::env::var_os("EIN_CORPUS_SLOW").is_some();
    let (depth, k) = if deep { (3, 32) } else { (2, 28) };
    for (hrule, obligations, depth, k) in [
        (
            "examples/zebra2.ein",
            "examples/zebra2-obligations.ein",
            2,
            1,
        ),
        (
            "examples/zebra2-minus-15.ein",
            "examples/zebra2-minus-15-obligations.ein",
            depth,
            k,
        ),
    ] {
        let (_, ta, _, a) = run_with(hrule, &mut Events::off(), depth);
        let (_, tb, _, b) = run_with(obligations, &mut Events::off(), depth);
        let (ma, mb) = (models(&ta, &a), models(&tb, &b));
        assert_eq!(ma.len(), k, "{hrule} found {} models, not {k}", ma.len());
        assert_eq!(
            ma, mb,
            "{obligations} found a different model set than {hrule}"
        );
    }
}

/// The two names the lever answers to, and the mode names the schema
/// documents — spelled once, here, so a rename has to come through a test.
#[test]
fn the_lever_and_the_modes_keep_their_names() {
    assert_eq!(Choice::RuleOrder.as_str(), "rule-order");
    assert_eq!(Choice::FailFirst.as_str(), "fail-first");
    assert_eq!(Choice::Off.as_str(), "off");
    assert_eq!(Mode::default(), Mode::Blind);
    for (m, s) in [
        (Mode::Blind, "blind"),
        (Mode::Hrules, "hrules"),
        (Mode::Obligations, "obligations"),
        (Mode::Stuck, "stuck"),
        (Mode::Declined, "declined"),
    ] {
        assert_eq!(m.as_str(), s);
    }
}
