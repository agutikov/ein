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

//!
//! **Since M1e2 S1e2.1c.5 (2026-11-03) it reads the routes**
//! ([`ein_infer::explore`]): the walk it was written against was deleted on
//! the user's ruling of that day. The ladder is what builds H, the candidates
//! every route commits to, so the dispatch is a claim about the one `rung`
//! line a route narrates when it builds H at root; and the completeness claim
//! is that the hrule path and the obligation path build the same H, so their
//! R is one, and the maximal lines the obligation tree reaches agree.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::SharedMemo;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explore::{self, Answer, Ask, Mode as Question, Which};
use ein_infer::oblgen::{Choice, Mode};
use ein_ir::{Ast, load_file};

/// Ask one question about one file, optionally narrating.
fn ask_with(rel: &str, q: Question, events: &mut Events) -> Answer {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    explore::run_with(&mut kb, &mut terms, &ast, &explore::Options::new(q), events)
        .unwrap_or_else(|e| panic!("{rel}: {e}"))
}

const SINGLE: Question = Question {
    which: Which::All,
    ask: Ask::Single,
};

/// Root's tally, by the pass every route calls on root's fixpoint.
fn root_owes(rel: &str) -> usize {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb: Kb =
        load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut Events::off())
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    ein_infer::obligations::tally(
        &kb,
        &mut terms,
        &ast,
        &SharedMemo::default(),
        &mut Events::off(),
    )
    .unwrap_or_else(|e| panic!("{rel}: {e}"))
    .total()
}

/// The maximal line, as sets of the candidates each state holds, rendered.
///
/// Text rather than indices, because two runs intern independently and the
/// question is whether they found the same *states* — which is a statement
/// about propositions, not about numbering.
fn maximal(rel: &str) -> BTreeSet<BTreeSet<String>> {
    let a = ask_with(
        rel,
        Question {
            which: Which::Maximal,
            ask: Ask::Describe,
        },
        &mut Events::off(),
    );
    assert!(a.exhausted, "{rel}: the maximal line is not known");
    a.states
        .iter()
        .map(|s| {
            s.closure
                .iter()
                .map(|&b| a.candidates[b as usize].text.clone())
                .collect()
        })
        .collect()
}

/// H, rendered, in the route's order.
fn universe(rel: &str) -> Vec<String> {
    ask_with(rel, SINGLE, &mut Events::off())
        .candidates
        .into_iter()
        .map(|c| c.text)
        .collect()
}

/// The distinct `rung` lines a run narrated, in first-seen order.
fn rungs(rel: &str) -> Vec<(String, u64, u64, u64)> {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Normal);
    let _ = ask_with(rel, SINGLE, &mut events);
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
        // owes, and every candidate that would pay it is stored false. Until
        // M1f S1f.6.34 this was `23_total_owed.ein`, whose `:no-hypothesis`
        // named the relation it owes, which is refused at load since.
        ("tests/region/04_owing.ein", &["stuck"]),
        // an obligation whose guard scans a relation the rung proposes.
        (
            "tests/stdlib/bijection/06_blind_enumeration.ein",
            &["declined"],
        ),
        // no obligation rule at all — rung 3, and not a word about it.
        ("examples/branching/02_one_dead_one_alive.ein", &[]),
    ];
    for (rel, want) in cases {
        // H is built once, at root, and that is where the rung is decided.
        let got: Vec<String> = rungs(rel).into_iter().map(|(m, ..)| m).collect();
        let uniq: BTreeSet<&str> = got.iter().map(String::as_str).collect();
        let want: BTreeSet<&str> = want.iter().copied().collect();
        assert_eq!(uniq, want, "{rel} took the wrong rung(s)");
    }
    // The override, stated as the thing it overrides: zebra2 owes 36 slots at
    // root — the same 36 `zebra2-obligations.ein` branches on — and its hrule
    // is why nothing above reports a rung for it.
    assert_eq!(
        root_owes("examples/zebra2.ein"),
        36,
        "zebra2 owes what its twin owes"
    );
}

/// **A state that owes something it may not branch on is reported, not
/// silently complete.**
///
/// The trap the ladder's second rung opens: the generator proposes nothing, so
/// `complete()` says yes, so the node is recorded as a model — while the tally
/// beside it says the requirement is unmet. Ten corpus programs were in exactly
/// that state at S1d.2.5, every one of them because `:no-hypothesis` names the
/// relation they owe, and the `stuck` line is what tells them apart from a
/// state that is finished. **That trap is closed**: since S1d.2.6 such a node
/// is not recorded as a model by the read-out at all; and since M1f S1f.6.34 a
/// list that names a relation an obligation owes is refused at load (*"filtered
/// obligations are nonsense by definition"*), so a state is stuck because
/// every candidate that would pay it is already refuted, `04_owing.ein`'s Ann,
/// who likes neither food: owed 1, branched on 1, declined 0.
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
    let rel = "tests/region/04_owing.ein";
    let rows = rungs(rel);
    assert_eq!(
        rows,
        [("stuck".to_string(), 1, 1, 0)],
        "{rel} owes one, branches on it, and nothing it proposes survives"
    );
    let a = ask_with(rel, SINGLE, &mut Events::off());
    assert_eq!(
        a.verdict.map(|v| v.word()),
        Some("Open"),
        "a stuck state is not a model — S1d.2.6"
    );
    assert_eq!(root_owes(rel), 1, "and it still says what it owes");
}

/// **A discharged state proposes nothing, and that is what completes it.**
///
/// The other half of the same question: `24_total_owed_satisfied.ein` is
/// `23_total_owed.ein`'s twin with the witness present. Rung 2 runs, owes nothing, proposes
/// nothing — and *not* because anything was scoped out. The pair is what makes
/// "complete means discharged" a distinction rather than a coincidence, since
/// both files report `k = 1` and only one of them owes.
#[test]
fn a_discharged_state_owes_nothing_and_declines_nothing() {
    let rows = rungs("tests/stdlib/algebra/24_total_owed_satisfied.ein");
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
    let _ = ask_with("examples/zebra2-obligations.ein", SINGLE, &mut events);
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

/// **Every state the hrule path finds, the obligation path finds.**
///
/// T1d.2.5.3, and the phase's central claim: the ladder is exhaustive iff
/// obligations and saturation between them determine every remaining open
/// fact. On the zebra family the obligated arrows *are* the decision
/// variables, so the two paths must agree — and the pair below is one
/// determinate puzzle and one genuinely under-determined one, because a
/// comparison of two singleton sets proves much less.
///
/// Since M1e2 a route commits to H, the candidates the ladder builds at root,
/// and R is a function of the program's rules and H. The twins differ by the
/// `(hrule …)` and the `:hrules` clause alone, which are no rule, so **one H
/// is one R**: that is asserted on both pairs, and the maximal lines are
/// compared on `zebra2`, where the obligation tree reaches its one state in
/// 45 saturations. `zebra2-minus-15`'s 32 cost 2 346 a side (5 s in a release
/// build, 2026-11-03), so `EIN_CORPUS_SLOW` compares those, the way the
/// corpus sweep splits the same way. The CI nightly sets it since M1f
/// S1f.5.27 (Q-M1f.51), where no tier had.
#[test]
fn the_theory_finds_what_the_hrule_finds() {
    let deep = std::env::var_os("EIN_CORPUS_SLOW").is_some();
    for (hrule, obligations, k, compare) in [
        (
            "examples/zebra2.ein",
            "examples/zebra2-obligations.ein",
            1,
            true,
        ),
        (
            "examples/zebra2-minus-15.ein",
            "examples/zebra2-minus-15-obligations.ein",
            32,
            deep,
        ),
    ] {
        let (ha, hb) = (universe(hrule), universe(obligations));
        assert!(!ha.is_empty(), "{hrule}: an empty H");
        assert_eq!(ha, hb, "{obligations} built a different H than {hrule}");
        if !compare {
            continue;
        }
        let (ma, mb) = (maximal(hrule), maximal(obligations));
        assert_eq!(ma.len(), k, "{hrule} found {} states, not {k}", ma.len());
        assert_eq!(
            ma, mb,
            "{obligations} found a different maximal line than {hrule}"
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
