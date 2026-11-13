//! M1d S1d.2.4 — **what a quiescent state owes, asserted.**
//!
//! `stdlib_coverage.rs`'s sibling, and it exists for a reason that is stated
//! in every fixture it reads: `:expect` has three forms and all three are
//! assertions about **facts** (`(model …)`, `(or (model …) …)`, `(false)` —
//! `01_grammar.md` § Query). An `open` conclusion is by construction never a
//! fact — it is a tally on the search-lattice node, because a stored one would
//! survive its own discharge in a fork that paid it — so **no expectation can
//! observe an owe count**. Meanwhile the coverage gate demands that every
//! program under `tests/` state one. So each fixture carries an ordinary
//! `:expect (model …)` about the facts it derives, which is what pins the
//! state, and the owe claim is asserted here: in-process, no binary, the
//! S1c.1.5 shape.
//!
//! Growing `:expect` a word for an open verdict is deliberately **not** taken:
//! it is a verdict-vocabulary change, this stage moves no verdict word, and
//! [S1d.2.6] routes it to [P1d.4]. It becomes the right answer the moment a
//! puzzle author outside this suite needs to state the claim; until then a
//! Rust test is the honest channel and costs no grammar.
//!
//!
//! **Since M1e2 S1e2.1c.5 (2026-11-03) every claim here is read off a route**
//! ([`ein_infer::explore`]), the walk these tests first ran on having been
//! deleted on the user's ruling of that day. What a route reports of a
//! quiescent state is root's tally ([`Described::root`]), each owing frontier
//! node's (`Described::frontier`) and `--single`'s word; the two numbers
//! about root's ledger read the same pass the routes call,
//! [`ein_infer::obligations::tally`], over root's fixpoint, because describing
//! `zebra2-minus-15`'s R to read root's tally would cost the slow tier's run.
//!
//! [S1d.2.6]: `docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus`
//! [P1d.4]: `docs/history/m1d_satisfiability/README.md#p1d4--closing-the-model-set`

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::SharedMemo;
use ein_infer::described::Described;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::explore::{self, Answer, Ask, Mode, Verdict, Which};
use ein_infer::obligations::Owes;
use ein_ir::{Ast, load_file};

fn load(rel: &str) -> (Ast, Terms, Kb) {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    (ast, terms, kb)
}

/// Ask `mode` about one file, as `ein solve` does, optionally narrating.
fn ask_with(rel: &str, mode: Mode, events: &mut Events) -> (Terms, Kb, Answer) {
    let (ast, mut terms, mut kb) = load(rel);
    let a = explore::run_with(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(mode),
        events,
    )
    .unwrap_or_else(|e| panic!("{rel}: {e}"));
    (terms, kb, a)
}

/// `ein solve FILE`: R described.
fn describe(rel: &str) -> Described {
    let mode = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let (_, _, a) = ask_with(rel, mode, &mut Events::off());
    assert!(a.exhausted, "{rel}: R is not known");
    *a.described
        .unwrap_or_else(|| panic!("{rel}: describe describes"))
}

/// `ein solve FILE --single`: the word.
fn single(rel: &str) -> &'static str {
    let mode = Mode {
        which: Which::All,
        ask: Ask::Single,
    };
    let (_, _, a) = ask_with(rel, mode, &mut Events::off());
    a.verdict
        .map(Verdict::word)
        .unwrap_or_else(|| panic!("{rel}: --single reached no verdict"))
}

/// Root's tally, by the pass every route calls on root's fixpoint
/// (`region::prepare`): root saturated, then [`ein_infer::obligations::tally`].
fn root_tally(rel: &str) -> (Terms, Kb, Owes) {
    let (ast, mut terms, mut kb) = load(rel);
    ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut Events::off())
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
    let owes = ein_infer::obligations::tally(
        &kb,
        &mut terms,
        &ast,
        &SharedMemo::default(),
        &mut Events::off(),
    )
    .unwrap_or_else(|e| panic!("{rel}: {e}"));
    (terms, kb, owes)
}

// ── the conformance pairs ──────────────────────────────────────────

/// **Each new stdlib rule owes what its fixture says, and its twin owes
/// nothing.**
///
/// The P1c.1 pair, in the form the obligation duals need it. The firing half
/// is what the coverage gate already demands; the **satisfied** half is the
/// one a guard bug lives in, because discharge *is* the guard — there is no
/// second `∃b: G ∧ B` query that could disagree with it, so "this file reports
/// nothing" is the whole claim that the guard is right.
///
/// The counts are not round numbers by accident. `09_owed_room` owes 3 rather
/// than 5 because the rule's `(neq ?Ta ?index)` guard keeps a Seat from owing
/// itself a seat; `11_owed_fill` owes 3 over `seats × non-seat types` minus
/// the one pair that is filled. A mutant that drops either quantifier moves
/// one of these numbers.
#[test]
fn each_obligation_rule_owes_exactly_what_its_fixture_states() {
    // (file, expected count, one `:why` the report must contain)
    let cases: [(&str, usize, &str); 8] = [
        (
            "tests/stdlib/algebra/23_total_owed.ein",
            1,
            "likes owes Ann a Food.",
        ),
        ("tests/stdlib/algebra/24_total_owed_satisfied.ein", 0, ""),
        (
            "tests/stdlib/algebra/25_surjective_owed.ein",
            1,
            "likes owes Soup a preimage in Person.",
        ),
        (
            "tests/stdlib/algebra/26_surjective_owed_satisfied.ein",
            0,
            "",
        ),
        (
            "tests/stdlib/slots/09_owed_room.ein",
            3,
            "Bob is in no slot yet — some Seat is still owed.",
        ),
        ("tests/stdlib/slots/10_owed_room_satisfied.ein", 0, ""),
        (
            "tests/stdlib/slots/11_owed_fill.ein",
            3,
            "slot S2 has no Who yet — one is still owed.",
        ),
        ("tests/stdlib/slots/12_owed_fill_satisfied.ein", 0, ""),
    ];
    for (rel, want, why) in cases {
        let d = describe(rel);
        let got = d.root.owed.clone();
        assert_eq!(
            (got.len(), d.root.owes as usize),
            (want, want),
            "{rel}: root owes {} where the fixture states {want}:\n  {}",
            d.root.owes,
            got.join("\n  "),
        );
        // An owing root is no state of R, and the obligation proposes what
        // would pay it, so R is the states that do (since M1f S1f.6.34, which
        // refuses the `:no-hypothesis` that kept it from proposing, and R was
        // empty with root its owing frontier); a discharged root is R's one
        // state.
        if want > 0 {
            assert!(!d.root.in_r(), "{rel}: a root that owes is no state of R");
            assert!(d.k_s() > 0, "{rel}: the obligation proposes what pays it");
            assert!(
                d.frontier.is_empty(),
                "{rel}: no alive node owes with no alive node above it"
            );
        } else {
            assert!(d.root.in_r(), "{rel}: a discharged root is in R");
        }
        if !why.is_empty() {
            assert!(
                got.iter().any(|w| w == why),
                "{rel}: no outstanding obligation renders as {why:?}; got:\n  {}",
                got.join("\n  "),
            );
        }
    }
}

/// **A state that owes is not a `Solution`** — and this is the test S1d.2.4
/// wrote to be edited here.
///
/// `tests/region/04_owing.ein` is `consistent ∧ complete` by the only
/// completeness test the engine had — *does the generator propose anything?*
/// — and it owes Ann a Food. (It was `23_total_owed.ein` until M1f S1f.6.34,
/// whose `:no-hypothesis` kept the obligation from proposing; `04` is its
/// program with Ann's two likings stored false, so nothing that would pay her
/// debt survives.) S1d.2.4 asserted `Solution` and said in its own doc comment
/// that [S1d.2.6] would have to change it. It did: `complete` in the verdict
/// read-out now means **discharged**, so the state is reported `Open`, the
/// word is `owes 1`, and the debt is the reason rather than a footnote beside
/// a model.
///
/// **The numbers that must disagree**, because their disagreement is the
/// whole of what shipped. The walk recorded the node and its read-out did not
/// call it a model; since M1e2 a node is in R only when it is alive **and**
/// discharged, so R is empty and its owing frontier is root, owing one:
///
/// | | | |
/// |---|---:|---|
/// | `k_s` | 0 | what R holds |
/// | the frontier | ∅ | the alive node that owes, with no alive node above it |
/// | `root.owes` | 1 | why |
///
/// [S1d.2.6]: `docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus`
#[test]
fn a_state_that_owes_is_open_and_not_a_solution() {
    let rel = "tests/region/04_owing.ein";
    assert_eq!(
        single(rel),
        "Open",
        "a state that owes is not a model — S1d.2.6"
    );
    let (_, _, kb) = load(rel);
    assert_eq!(
        kb.program().obligations.len(),
        1,
        "in scope: it states one obligation"
    );
    let d = describe(rel);
    assert_eq!(d.k_s(), 0, "no model");
    assert_eq!(d.root.owes, 1, "and it owes one");
    assert!(d.root.alive, "alive: the debt is not a refutation");
    assert_eq!(
        d.frontier.iter().map(|o| o.owes).collect::<Vec<_>>(),
        vec![1],
        "one debt, on the one open state"
    );
}

/// **The scope rule, as a test**: the same shape with no obligation stated
/// keeps `Solution`.
///
/// `03_closed_and_owing.ein` was in exactly this position until S1d.2.6 — a
/// state owing something it can never pay, reported as a model — and what
/// moved it was one declaration, not the verdict change. So the rule needs a
/// witness on the other side: a program judged by *exhaustion* because it
/// never said what it owed. `01_infer_closure.ein` is the neighbour that
/// states nothing, and since M1f S1f.6.34 so is `02_closed_and_satisfied.ein`
/// (the next test).
#[test]
fn a_program_that_states_no_obligation_keeps_its_word() {
    let rel = "tests/stdlib/closure/01_infer_closure.ein";
    let (_, _, kb) = load(rel);
    assert!(kb.program().obligations.is_empty(), "states no obligation");
    assert!(
        !describe(rel).owing_met,
        "so no node it reaches owes anything"
    );
    assert_eq!(
        single(rel),
        "Solution",
        "and is judged by exhaustion, exactly as before P1d.2"
    );
}

/// The pair S1d.2.2 banked, and what became of it: **the corner is a
/// conflict**.
///
/// `02_closed_and_satisfied.ein` and `03_closed_and_owing.ein` both declared
/// `(total-owed r is-a)` beside the closure `infer-closure` derives, one fact
/// apart, and from S1d.2.6 one word apart: `Solution`, and `Open — owes 1` with
/// the rung `stuck`. Since M1f S1f.6.34 an obligation on a closed relation is
/// refused at load (*"obligated relation can't be __closed__ also"*, the user,
/// 2026-11-13): the twin is `examples/ein-bugs/closed-and-owing.ein`, the
/// refusal's fixture, which the mechanisms check refuses with one line naming
/// the obligation, its activator, the closure and the rule that derives it;
/// and `02` keeps the closure, drops the obligation, and is `Solution` by
/// exhaustion.
#[test]
fn the_closed_and_owing_corner_is_a_conflict() {
    let sat = "tests/stdlib/closure/02_closed_and_satisfied.ein";
    assert_eq!(single(sat), "Solution", "closed, and owing nothing");
    let (_, _, kb) = load(sat);
    assert!(kb.program().obligations.is_empty(), "it states none");

    let owing = "examples/ein-bugs/closed-and-owing.ein";
    let (ast, mut terms, mut kb) = load(owing);
    let report = ein_infer::mechanisms::check(&ast, &mut terms, &mut kb).expect("root saturates");
    assert_eq!(report.conflicts.len(), 1, "{:?}", report.conflicts);
    let line = &report.conflicts[0];
    for want in [
        "obligation 'total-owed', activated by (total-owed r is-a)",
        "owes r, and (__closed__ r), which rule 'infer-closure' derives, says r is closed",
    ] {
        assert!(line.contains(want), "{line:?} does not say {want:?}");
    }
}

// ── the two numbers ────────────────────────────────────────────────

/// **`zebra2-minus-15` owes 46 at root** — the hand census, reproduced.
///
/// `obligation_forms.md` §5 counted this by hand on 2026-08-24, before the
/// form had a name: five `(bijective R)` declarations imply 5 × 5 × 2 = **50**
/// obligations, the two arrows already true at root discharge 2 × 2 = 4 of
/// them, and 23 forward + 23 backward = **46** remain. The engine reproducing
/// that number is S1d.2.4's acceptance in one line.
///
/// The per-relation split is the sharper half and the hand census did not have
/// it: `nation-loc` and `drink-loc` are the two relations carrying a stated
/// arrow (`Norwegian@House-1`, `Milk@House-3`), so they owe 8 where the other
/// three owe 10. A tally that got 46 by any other route fails here.
#[test]
fn zebra2_minus_15_owes_the_number_the_hand_census_found() {
    let (terms, _, owes) = root_tally("examples/zebra2-minus-15.ein");
    assert_eq!(owes.total(), 46, "root's tally");

    let mut by: Vec<(String, usize)> = owes
        .by_relation()
        .into_iter()
        .map(|(r, n)| (terms.sym(r).to_string(), n))
        .collect();
    by.sort();
    assert_eq!(
        by,
        vec![
            ("color-loc".to_string(), 10),
            ("drink-loc".to_string(), 8),
            ("nation-loc".to_string(), 8),
            ("pet-loc".to_string(), 10),
            ("smoke-loc".to_string(), 10),
        ],
        "the split, not just the sum"
    );
}

/// The extent of `(isa _ T)` — the set of objects the obligation quantifies
/// over.
///
/// A `Vec`, because `Symbol` is an interner index with no order worth sorting
/// by: these extents are five elements wide and membership is a scan.
fn extent(kb: &Kb, terms: &Terms, isa: Symbol, ty: Symbol) -> Vec<Symbol> {
    let mut out: Vec<Symbol> = Vec::new();
    for f in kb.facts_of(isa) {
        let args = terms.facts.args(f);
        if args.len() != 2 {
            continue;
        }
        if let (Some(a), Some(t)) = (args[0].as_sym(), args[1].as_sym())
            && t == ty
            && !out.contains(&a)
        {
            out.push(a);
        }
    }
    out
}

/// **The conservation audit** — the ledger's size, predicted from the
/// declarations and diffed against what the rules emitted.
///
/// `ideas.md`'s invariant is *"open assertions = obligated facts × arity"*: a
/// relation declared `bijective` over an n × m signature owes each domain
/// element an image and each range element a preimage, and every stored arrow
/// discharges the two ends it touches. So for each `(total-owed R isa)` the
/// engine derived, the prediction is
///
/// ```text
///   |{a ∈ isa⁻¹(A) : ∄b ∈ isa⁻¹(B). (R a b)}|      the forward debts
/// + |{b ∈ isa⁻¹(B) : ∄a ∈ isa⁻¹(A). (R a b)}|      the backward debts
/// ```
///
/// computed here from the fact store alone, with no reference to the pass. It
/// is the [`layer_census`](../../../../docs/history/m1d_satisfiability/layer_census.md)
/// style of claim: the engine can predict the ledger from the declarations, and
/// a mismatch is an encoding bug with a number attached.
///
/// Run over **every corpus program that declares a bijection**, so it is a
/// claim about the mechanism rather than about one puzzle.
#[test]
fn the_ledger_is_the_size_the_declarations_predict() {
    let files = [
        "examples/zebra2.ein",
        "examples/zebra2-minus-15.ein",
        "tests/stdlib/bijection/01_setup_and_negatives.ein",
        "tests/stdlib/bijection/02_domain_elimination.ein",
        "tests/stdlib/bijection/03_range_elimination.ein",
    ];
    let mut audited = 0usize;
    for rel in files {
        // The audit reads root's tally and root's fact store, and nothing
        // past root's fixpoint.
        let (terms, kb, root_owes) = root_tally(rel);
        let Some(total_owed) = terms.syms.get("total-owed") else {
            panic!("{rel}: nothing derived a total-owed activator");
        };
        let relation = terms.kernel.relation;

        // `(relation R A B)` — the signature the rules read their types from.
        let mut sig: Vec<(Symbol, (Symbol, Symbol))> = Vec::new();
        for f in kb.facts_of(relation) {
            let args = terms.facts.args(f);
            if args.len() != 3 {
                continue;
            }
            if let (Some(r), Some(a), Some(b)) =
                (args[0].as_sym(), args[1].as_sym(), args[2].as_sym())
            {
                sig.push((r, (a, b)));
            }
        }

        for f in kb.facts_of(total_owed).collect::<Vec<FactId>>() {
            let args = terms.facts.args(f).to_vec();
            let (Some(r), Some(isa)) = (args[0].as_sym(), args.get(1).and_then(|v| v.as_sym()))
            else {
                continue;
            };
            let Some(&(_, (ta, tb))) = sig.iter().find(|(name, _)| *name == r) else {
                continue;
            };
            let (dom, ran) = (extent(&kb, &terms, isa, ta), extent(&kb, &terms, isa, tb));
            let arrows: Vec<(Symbol, Symbol)> = kb
                .facts_of(r)
                .filter_map(|f| {
                    let a = terms.facts.args(f);
                    if a.len() != 2 {
                        return None;
                    }
                    match (a[0].as_sym(), a[1].as_sym()) {
                        (Some(x), Some(y)) => Some((x, y)),
                        _ => None,
                    }
                })
                .collect();
            let forward = dom
                .iter()
                .filter(|a| !arrows.iter().any(|(x, y)| x == *a && ran.contains(y)))
                .count();
            let backward = ran
                .iter()
                .filter(|b| !arrows.iter().any(|(x, y)| y == *b && dom.contains(x)))
                .count();
            let predicted = forward + backward;
            let emitted = root_owes.owed_by(r);
            assert_eq!(
                emitted,
                predicted,
                "{rel}: {} owes {emitted} and the declarations predict {predicted} \
                 (|dom| {} , |ran| {} , arrows {})",
                terms.sym(r),
                dom.len(),
                ran.len(),
                arrows.len(),
            );
            audited += 1;
        }
    }
    assert!(
        audited >= 10,
        "only {audited} relations audited — the file list stopped reaching bijections"
    );
}

// ── what must not have moved ───────────────────────────────────────

/// **No obligation rule is in the saturation agenda**, and the corpus is what
/// says so rather than the type system.
///
/// S1d.2.3 made this structural — a registry of its own that neither the
/// saturator nor `hypgen` walks — but recorded that the corpus half of the
/// claim was *vacuous* at that stage, because no entry used the atom. The
/// duals ship here, so it has something to hold across: on a program that
/// activates them, every `fire` event names a rule from `program.rules` and
/// none names an obligation.
#[test]
fn an_obligation_rule_never_reaches_the_firing_stream() {
    for rel in [
        "examples/zebra2.ein",
        "tests/stdlib/algebra/23_total_owed.ein",
        "tests/stdlib/slots/09_owed_room.ein",
    ] {
        let buffer = Buffer::new();
        let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
        // `--single`: root, then every node the obligation tree saturates,
        // each narrated with its own firings and its own tally.
        let mode = Mode {
            which: Which::All,
            ask: Ask::Single,
        };
        let (terms, kb, _) = ask_with(rel, mode, &mut events);
        let obligations: BTreeSet<String> = kb
            .program()
            .obligations
            .keys()
            .map(|s| terms.sym(s).to_string())
            .collect();
        assert!(!obligations.is_empty(), "{rel}: no obligation rule loaded");
        let log = buffer.to_string_lossy();
        let (mut fires, mut owes) = (0usize, 0usize);
        for line in log.lines() {
            let ev: serde_json::Value = serde_json::from_str(line).expect("event line");
            let rule = ev["rule"].as_str().unwrap_or("");
            match ev["e"].as_str().unwrap_or("") {
                "fire" => {
                    fires += 1;
                    assert!(
                        !obligations.contains(rule),
                        "{rel}: obligation rule {rule} reached the firing stream"
                    );
                }
                "owe" => {
                    owes += 1;
                    assert!(
                        obligations.contains(rule),
                        "{rel}: {rule} emitted an owe and is not an obligation rule"
                    );
                }
                _ => {}
            }
        }
        // `owes` on every one; `fires` only where the program has saturation
        // rules at all — two of the three are an obligation and its activators
        // and nothing else, which is what makes them isolate the rule.
        assert!(owes > 0, "{rel}: no owe reached the stream");
        assert!(
            fires > 0 || kb.program().rules.is_empty(),
            "{rel}: {} saturation rules and none fired",
            kb.program().rules.len()
        );
    }
}

/// **The new activators sit behind the same hypothesis scoping the six
/// existing ones do** — a fixture, not an argument, and the fixture found a
/// bug.
///
/// `bijective-setup` now derives two more activator facts per declaration, and
/// an activator fact is a *stored* fact. Two things could go wrong and only
/// one of them was obvious.
///
/// **The obvious one**: could the enumerator propose `(total-owed seats
/// is-a)` itself? No — it builds candidates only for relations carrying a
/// `(relation …)` signature, and the obligation activators carry none, exactly
/// as `total`, `surjective`, `domain-elimination`, `range-elimination` and the
/// two negative-completion activators carry none.
///
/// **The one this fixture found**: the *name* `total-owed` was landing in the
/// candidate **object** pool, so the enumerator proposed `(seats total-owed
/// C1)` — the name of a rule as a puzzle value, 3 502 of the 6 231 proposals
/// on this program. `hypgen::candidate_objects` keeps every name that
/// categorises as `Object`, and `Program::categorise` read `self.rules` alone,
/// so the S1d.2.3 registry split had quietly made an obligation rule's name
/// not-a-rule. The six existing activators were never in that state, which is
/// what "the same scoping" has to mean. Fixed in `categorise`; this is the
/// check.
#[test]
fn a_blind_run_never_proposes_an_obligation_activator() {
    let buffer = Buffer::new();
    let mut events = Events::to(Box::new(buffer.clone()), Level::Verbose);
    // Every candidate of H is narrated as a `hyp` line when the route builds
    // the universe, before any node: describe's stream holds all of them.
    let mode = Mode {
        which: Which::All,
        ask: Ask::Describe,
    };
    let _ = ask_with(
        "tests/stdlib/bijection/06_blind_enumeration.ein",
        mode,
        &mut events,
    );
    let names = [
        "total-owed",
        "surjective-owed",
        "slot-owed-room",
        "slot-owed-fill",
    ];
    let log = buffer.to_string_lossy();
    let mut proposals = 0usize;
    for line in log.lines() {
        let ev: serde_json::Value = serde_json::from_str(line).expect("event line");
        if ev["e"] != "hyp" {
            continue;
        }
        proposals += 1;
        let fact = ev["fact"].as_str().unwrap_or("");
        for n in names {
            assert!(
                !fact.starts_with(&format!("({n} ")),
                "the blind enumerator proposed an obligation activator: {fact}"
            );
            // As an argument value, which is the half that was broken.
            for token in fact
                .trim_matches(|c| c == '(' || c == ')')
                .split(' ')
                .skip(1)
            {
                assert_ne!(
                    token, n,
                    "an obligation rule's name reached the candidate object pool: {fact}"
                );
            }
        }
    }
    assert!(proposals > 0, "nothing was proposed — the check is vacuous");
}

/// Every fixture named above is a real file, so a rename cannot quietly turn
/// this suite into a no-op.
#[test]
fn the_fixtures_this_suite_names_exist() {
    for rel in [
        "tests/stdlib/algebra/23_total_owed.ein",
        "tests/stdlib/algebra/24_total_owed_satisfied.ein",
        "tests/stdlib/algebra/25_surjective_owed.ein",
        "tests/stdlib/algebra/26_surjective_owed_satisfied.ein",
        "tests/stdlib/slots/09_owed_room.ein",
        "tests/stdlib/slots/10_owed_room_satisfied.ein",
        "tests/stdlib/slots/11_owed_fill.ein",
        "tests/stdlib/slots/12_owed_fill_satisfied.ein",
        "examples/zebra2-minus-15.ein",
    ] {
        let p: &Path = &repo_root().join(rel);
        assert!(p.exists(), "{rel} is gone");
    }
}
