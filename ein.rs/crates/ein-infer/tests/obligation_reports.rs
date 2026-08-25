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
//! [S1d.2.6]: `plans/m1d_satisfiability/p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md`
//! [P1d.4]: `plans/m1d_satisfiability/p1d.4_model_set_closure/README.md`

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{FactId, Kb, Symbol, Terms};
use ein_corpus::repo_root;
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::obligations::Owes;
use ein_infer::solve::{NoDumper, SolveOptions, Solved, solve};
use ein_ir::{Ast, load_file};

/// Solve one file the way `ein test` does, and hand back the run.
fn run(rel: &str) -> (Ast, Terms, Kb, Solved) {
    run_with(rel, &mut Events::off(), 5)
}

/// A file whose search is large enough that the cap is what keeps this suite
/// fast.
///
/// **Every claim here is about a fixpoint, and root's is reached in Phase 1**
/// — before the first candidate is generated — so a depth cap costs the tests
/// nothing and saves `zebra2-minus-15` its 618 076 enterings. The suite's own
/// fixtures are tiny and exhaust either way.
fn run_shallow(rel: &str) -> (Ast, Terms, Kb, Solved) {
    run_with(rel, &mut Events::off(), 1)
}

fn run_with(rel: &str, events: &mut Events, depth: u32) -> (Ast, Terms, Kb, Solved) {
    let path: PathBuf = repo_root().join(rel);
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
    let opts = SolveOptions {
        config: Some(kb.program().config.clone().unwrap_or_default()),
        max_set_size: depth,
        ..SolveOptions::default()
    };
    let solved = solve(&mut kb, &mut terms, &ast, events, &mut NoDumper, &opts)
        .unwrap_or_else(|e| panic!("{rel}: {e:?}"));
    (ast, terms, kb, solved)
}

/// The rendered `:why` of every outstanding obligation, in report order.
fn whys(owes: &Owes) -> Vec<String> {
    owes.instances().iter().map(|i| i.why.clone()).collect()
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
        let (_, _, _, solved) = run(rel);
        let got = whys(&solved.owes.root);
        assert_eq!(
            got.len(),
            want,
            "{rel}: root owes {} where the fixture states {want}:\n  {}",
            got.len(),
            got.join("\n  "),
        );
        // The model is a state of the same fixpoint, so the two tallies agree
        // — and a fixture whose only solution is root is the common case here.
        if let Some(first) = solved.owes.models.first() {
            assert_eq!(
                whys(first).len(),
                want,
                "{rel}: the recorded model owes a different number than root did"
            );
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
/// `23_total_owed.ein` is `consistent ∧ complete` by the only completeness
/// test the engine had — *does the generator propose anything?* — and it owes
/// Ann a Food. S1d.2.4 asserted `Solution` and said in its own doc comment
/// that [S1d.2.6] would have to change it. It did: `complete` in the verdict
/// read-out now means **discharged**, so the state is reported `Open`, the
/// word is `owes 1`, and the debt is the reason rather than a footnote beside
/// a model.
///
/// **The three numbers that must disagree**, because their disagreement is
/// the whole of what shipped:
///
/// | | | |
/// |---|---:|---|
/// | `stats.solution_nodes` | 1 | what the *search* recorded — unchanged, and the reason no counter moved |
/// | `verdict.k()` | 0 | what the *read-out* calls a model |
/// | `owes.root.total()` | 1 | why |
///
/// A regression that reverted the read-out would keep the first and the third
/// and lose the second, so the second is asserted explicitly rather than via
/// the word.
///
/// [S1d.2.6]: `plans/m1d_satisfiability/p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md`
#[test]
fn a_state_that_owes_is_open_and_not_a_solution() {
    let (_, _, _, solved) = run("tests/stdlib/algebra/23_total_owed.ein");
    assert_eq!(
        solved.answer.as_str(),
        "Open",
        "a state that owes is not a model — S1d.2.6"
    );
    assert_eq!(solved.owes.root.total(), 1, "and it owes one");
    assert_eq!(
        solved.owes.declared, 1,
        "in scope: it states one obligation"
    );
    let ein_infer::verdict::Answer::Verdict(v) = &solved.answer else {
        panic!("a verdict")
    };
    assert_eq!(v.k(), 0, "no model");
    assert_eq!(v.owed(), 1, "one debt, on the open state");
    assert_eq!(
        solved.stats.solution_nodes, 1,
        "the search still recorded the node — S1d.2.6 moved the read-out, not the traversal"
    );
}

/// **The scope rule, as a test**: the same shape with no obligation stated
/// keeps `Solution`.
///
/// `03_closed_and_owing.ein` was in exactly this position until S1d.2.6 — a
/// state owing something it can never pay, reported as a model — and what
/// moved it was one declaration, not the verdict change. So the rule needs a
/// witness on the other side: a program judged by *exhaustion* because it
/// never said what it owed. `02_closed_and_satisfied.ein` is the discharged
/// twin, and `01_infer_closure.ein` is the neighbour that states nothing.
#[test]
fn a_program_that_states_no_obligation_keeps_its_word() {
    let (_, _, _, solved) = run("tests/stdlib/closure/01_infer_closure.ein");
    assert_eq!(solved.owes.declared, 0, "states no obligation");
    assert!(
        !solved.owes.in_scope(),
        "so it is out of the read-out's scope"
    );
    assert_eq!(
        solved.answer.as_str(),
        "Solution",
        "and is judged by exhaustion, exactly as before P1d.2"
    );
}

/// The pair S1d.2.2 banked, cashed: one fact apart, and now one **word** apart.
///
/// Both declare `(total-owed r is-a)` since S1d.2.6 — which is what puts them
/// in scope at all — and `03` is `02` with the witness edge deleted. The
/// contract's rule holds: the word is `Open` and **not** `(false)`, because no
/// rule derived a refutation. That the debt is unpayable is said by the rung
/// (`mode=stuck`), not by the verdict.
#[test]
fn the_closed_and_owing_pair_now_differs_by_a_word() {
    let (_, _, _, sat) = run("tests/stdlib/closure/02_closed_and_satisfied.ein");
    assert_eq!(sat.answer.as_str(), "Solution", "the witness discharges it");
    assert_eq!(sat.owes.declared, 1, "and it is in scope");

    let (_, _, _, owing) = run("tests/stdlib/closure/03_closed_and_owing.ein");
    assert_eq!(
        owing.answer.as_str(),
        "Open",
        "the same file, one fact fewer"
    );
    assert_eq!(owing.owes.root.total(), 1, "a1 owes a B");
    assert_ne!(
        sat.answer.as_str(),
        owing.answer.as_str(),
        "the corner is what a pair reporting one word for two states was"
    );
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
    let (_, terms, _, solved) = run_shallow("examples/zebra2-minus-15.ein");
    let owes = &solved.owes.root;
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
/// is the [`layer_census`](../../../../plans/m1d_satisfiability/p1d.10_exhaustive_search/layer_census.md)
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
        // The audit reads root's tally and root's fact store, both of them
        // Phase 1's, so the depth cap is free here too.
        let (_, terms, kb, solved) = run_shallow(rel);
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
            let emitted = solved.owes.root.owed_by(r);
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
        let (_, terms, kb, _) = run_with(rel, &mut events, 5);
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
    let (_, _, _, _) = run_with(
        "tests/stdlib/bijection/06_blind_enumeration.ein",
        &mut events,
        5,
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
