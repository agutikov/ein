//! **Hypotheses from obligations, asserted**
//! ([04-hypothesis-generation.md § 4](../../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#4-the-obligations-what-would-pay-a-debt)).
//!
//! [`obligation_reports.rs`](obligation_reports.rs)'s sibling.
//! That suite checks what a quiescent state *says* it owes; this one checks
//! that the debt is a **choice point** — that the ladder dispatches where it
//! should, that the branch is the obligation's own domain scan, and that the
//! search it drives finds the same models as the hand-written a rule that concludes `(open F)` it
//! replaces.
//!
//! The central claim is `the_theory_finds_what_the_hypothesis_rule_finds`, and it is a
//! *model-set* comparison rather than a counter one on purpose: counters may
//! move under a new traversal, answers may not, and a counter equality
//! pinned as a test would be a test of the fixtures' arithmetic rather than
//! of the engine. The ladder these tests hold is [the page's].
//!
//! [the page's]: ../../../../docs/ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#2-the-union

//!
//! **It reads the routes** ([`ein_infer::explore`]). The ladder is what
//! builds H, the candidates
//! every route commits to, so the dispatch is a claim about the one `rung`
//! line a route narrates when it builds H at root; and the completeness claim
//! is that the hypothesis rule path and the obligation path build the same H, so their
//! R is one, and the maximal lines the obligation tree reaches agree.

use std::collections::BTreeSet;
use std::path::PathBuf;

use ein_core::{Kb, Terms};
use ein_corpus::{Scratch, repo_root};
use ein_infer::SharedMemo;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explore::{self, Answer, Ask, Mode as Question, Which};
use ein_infer::oblgen::{Mode, ORDER};
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

/// The stuck state, as a program of its own: `tests/region/04_owing.ein`
/// with Ann's two likings stored false so that every candidate that would
/// pay her debt is refuted, and no `(solve :no-search)`. The fixture states
/// `:no-search`, which runs no rung; the stuck rung is this program's.
/// Written into a scratch directory the caller
/// holds, as an absolute path, which `repo_root().join` keeps.
fn stuck_program() -> (Scratch, String) {
    let dir = Scratch::new("ein-obligation-rung-stuck");
    let path = dir.path().join("stuck.ein");
    std::fs::write(
        &path,
        "(import std.algebra :symbols (total-owed))\n\
         (relation is-a  T T :why \"{?3} is a {?4}\")\n\
         (relation likes Person Food :why \"{?3} likes {?4}\")\n\
         (is-a Ann Person) (is-a Bob Person)\n\
         (is-a Soup Food)  (is-a Stew Food)\n\
         (total-owed likes is-a)\n\
         (likes Bob Stew)\n\
         (not (likes Ann Soup)) (not (likes Ann Stew))\n\
         (solve :no-hypothesis (is-a))\n",
    )
    .expect("written");
    let path = path.to_string_lossy().into_owned();
    (dir, path)
}

/// `tests/stdlib/algebra/24_total_owed_satisfied.ein`'s program with no
/// `(solve :no-search)`: rung 2 runs and owes nothing.
fn discharged_program() -> (Scratch, String) {
    let dir = Scratch::new("ein-obligation-rung-discharged");
    let path = dir.path().join("discharged.ein");
    std::fs::write(
        &path,
        "(import std.algebra :symbols (total-owed))\n\
         (relation is-a  T T :why \"{?3} is a {?4}\")\n\
         (relation likes Person Food :why \"{?3} likes {?4}\")\n\
         (is-a Ann Person) (is-a Bob Person)\n\
         (is-a Soup Food)  (is-a Stew Food)\n\
         (total-owed likes is-a)\n\
         (likes Ann Soup) (likes Bob Stew)\n",
    )
    .expect("written");
    let path = path.to_string_lossy().into_owned();
    (dir, path)
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

/// **The dispatch, one file per mode.**
///
/// The obligations narrate a `rung` line wherever one is activated; written
/// `open` facts beside them make it `union` (Q-M1f.570). The last row is the
/// one that keeps the corpus still: a program that declares no obligation
/// rule narrates **nothing**, so its event stream carries no `rung` line.
#[test]
fn the_ladder_dispatches_by_what_the_program_declares() {
    // (file, the modes its generation calls report)
    let (_dir, stuck) = stuck_program();
    let cases: [(&str, &[&str]); 7] = [
        // A hypothesis rule beside the obligations: the union, which pays what root
        // owes with the hypothesis rule's candidates, and is no stuck state.
        ("examples/zebra2.ein", &["union"]),
        // The same where the obligation offers no candidate, its `absent`
        // ranging over nothing: H is the rule's two, which pay what root
        // owes, and root is not stuck.
        ("examples/lattice/18_two_candidates.ein", &["union"]),
        // the same puzzle with the hypothesis rule deleted: the obligations alone.
        ("examples/zebra2-obligations.ein", &["obligations"]),
        // owes, and every candidate that would pay it is stored false. A
        // `:no-hypothesis` naming the relation it owes would be refused at
        // load, and `tests/region/04_owing.ein` states `:no-search`.
        (&stuck, &["stuck"]),
        // `(solve :no-search)`: no rung runs, and none narrates,
        // though root owes.
        ("tests/region/04_owing.ein", &[]),
        // an obligation whose guard scans a relation it proposes: it does
        // not step aside (Q-M1f.579); no obligation is activated here, and
        // the blind enumerator runs.
        ("tests/stdlib/bijection/06_blind_enumeration.ein", &[]),
        // no obligation rule at all — the blind enumerator, and not a word
        // about it.
        ("examples/branching/02_one_dead_one_alive.ein", &[]),
    ];
    for (rel, want) in cases {
        // H is built once, at root, and that is where the rung is decided.
        let got: Vec<String> = rungs(rel).into_iter().map(|(m, ..)| m).collect();
        let uniq: BTreeSet<&str> = got.iter().map(String::as_str).collect();
        let want: BTreeSet<&str> = want.iter().copied().collect();
        assert_eq!(uniq, want, "{rel} took the wrong rung(s)");
    }
    // zebra2 owes 36 slots at root — the same 36 `zebra2-obligations.ein`
    // branches on — and its hypothesis rule's candidates pay them.
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
/// `complete()` says yes, so the node would be recorded as a model — while
/// the tally beside it says the requirement is unmet, and the `stuck` line is
/// what tells it apart from a state that is finished. **That trap is
/// closed**: such a node is not recorded as a model by the read-out at all;
/// and a list that names a relation an obligation owes is refused at load
/// (filtered obligations are nonsense by definition,
/// [07-diagnostics.md § 2.26](../../../../docs/ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)),
/// so a state is stuck because every candidate that would pay it is already
/// refuted, Ann's of [`stuck_program`], who likes neither food: owed 1,
/// branched on 1, declined 0.
///
/// **The word**: a stuck state reports `Open`, not `Solution`. The two
/// halves stay separate
/// on purpose — `stuck` is the *generator's* report (it proposed nothing and
/// says why) and `Open` is the *read-out's* (the state is not discharged) —
/// and this test asserts both, because a regression in either one alone would
/// leave the other still true.
#[test]
fn owing_and_unable_to_branch_is_stuck_and_says_so() {
    let (_dir, rel) = stuck_program();
    let rel = rel.as_str();
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
        "a stuck state is not a model"
    );
    assert_eq!(root_owes(rel), 1, "and it still says what it owes");
}

/// **A discharged state proposes nothing, and that is what completes it.**
///
/// The other half of the same question: [`discharged_program`] is
/// `24_total_owed_satisfied.ein`'s, `23_total_owed.ein`'s twin with the witness
/// present, without the `:no-search` both state. Rung 2 runs, owes nothing, proposes
/// nothing — and *not* because anything was scoped out. The pair is what makes
/// "complete means discharged" a distinction rather than a coincidence, since
/// both files report `k = 1` and only one of them owes.
#[test]
fn a_discharged_state_owes_nothing_and_declines_nothing() {
    let (_dir, rel) = discharged_program();
    let rows = rungs(&rel);
    assert_eq!(rows, [("obligations".to_string(), 0, 0, 0)]);
}

// ── The branch ─────────────────────────────────────────────────────

/// **The branch is the obligation's own domain scan, and nothing else.**
///
/// `zebra2-obligations` at root owes 36 instances — 36 of the 50 slots five
/// bijections need, the other 14 already witnessed — and proposes 180
/// candidates for them, which the filter pipeline dedups and refutes down to
/// the 56 the hypothesis rule path emits. The numbers matter in one direction each:
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

/// **Every state the hypothesis rule path finds, the obligation path finds.**
///
/// The ladder's central claim: it is exhaustive iff
/// obligations and saturation between them determine every remaining open
/// fact. On the zebra family the obligated arrows *are* the decision
/// variables, so the two paths must agree — and the pair below is one
/// determinate puzzle and one genuinely under-determined one, because a
/// comparison of two singleton sets proves much less.
///
/// A route commits to H, the candidates the writers build at root, and R is
/// a function of the program's rules and H. The twins differ by the
/// hypothesis rule `guess` and its activators alone, which conclude no fact
/// a state holds but `open` ones, so **one H is one R**: that is asserted on both pairs, and the maximal lines are
/// compared on `zebra2`, where the obligation tree reaches its one state in
/// a few dozen saturations. `zebra2-minus-15`'s 32 cost thousands a side, so
/// `EIN_CORPUS_SLOW` compares those, the way the corpus sweep splits the same
/// way.
#[test]
fn the_theory_finds_what_the_hypothesis_rule_finds() {
    let deep = std::env::var_os("EIN_CORPUS_SLOW").is_some();
    for (rules, obligations, k, compare) in [
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
        let (ha, hb) = (universe(rules), universe(obligations));
        assert!(!ha.is_empty(), "{rules}: an empty H");
        assert_eq!(ha, hb, "{obligations} built a different H than {rules}");
        if !compare {
            continue;
        }
        let (ma, mb) = (maximal(rules), maximal(obligations));
        assert_eq!(ma.len(), k, "{rules} found {} states, not {k}", ma.len());
        assert_eq!(
            ma, mb,
            "{obligations} found a different maximal line than {rules}"
        );
    }
}

/// The order the `rung` line names and the mode names the schema documents —
/// spelled once, here, so a rename has to come through a test.
#[test]
fn the_order_and_the_modes_keep_their_names() {
    assert_eq!(ORDER, "rule-order");
    assert_eq!(Mode::default(), Mode::Blind);
    for (m, s) in [
        (Mode::Blind, "blind"),
        (Mode::HypothesisRules, "hypothesis-rules"),
        (Mode::Obligations, "obligations"),
        (Mode::Stuck, "stuck"),
        (Mode::Declined, "declined"),
    ] {
        assert_eq!(m.as_str(), s);
    }
}
