//! M1e S1e.3.3 — **the M1 alive-set invariant, evaluated over the corpus.**
//!
//! [`ein_infer::invariant`] is the check; this is what it says, banked. The
//! finding it closes is `ST-M1`: the invariant licenses the per-KB `alive`
//! recompute, the `state_key` dedup that produces `k`, and — since M1d — the
//! tree's exhaustiveness-by-discharge argument, and until this stage nothing
//! evaluated it.
//!
//! # Three readings, and which one the engine's soundness rests on
//!
//! | | claim | corpus |
//! |---|---|---|
//! | **R0** | statically, no rule's `:assert` names a constant the loaded program did not | **2** programs break it |
//! | **R1** | nothing a run derives names what the loaded program did not | the same 2, at 3 sites |
//! | **R2** | nothing the **search** reaches names what **root's fixpoint** did not | **1** program breaks it |
//!
//! R0 ⇒ R1 ⇒ R2, and R2 is the one that can cost an answer: `alive₀` is taken
//! at root's fixpoint and the lattice enumerates subsets of it, so a name that
//! arrives later extends a candidate space nothing will revisit. R0 is the one
//! worth *checking*, because it is free, it runs at load, and it answers for
//! every run the program could have rather than for the one that happened —
//! and, measured here, it finds every breach the dynamic sweep finds.
//!
//! # What a breach costs
//!
//! Nothing, on both corpus programs, and an **answer** on the pair this stage
//! added: `examples/ein-bugs/alive-set-fresh-name.ein` reports `k = 0,
//! exhausted = true` — a refutation, not a lower bound — where a model exists,
//! and `…-declared.ein` is the same file plus one fact naming the invented
//! constant and answers `Solution k = 1` over exactly that model.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_infer::events::Events;
use ein_infer::invariant::Universe;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_infer::verdict::Answer;
use ein_ir::{Ast, load_file};

/// **The whole of what the corpus does wrong**, as `(path, rendered breach)`.
///
/// Banked as a set rather than asserted empty, for `refutation_under_absent`'s
/// reason: the two entries are real and neither is a mistake to fix here. A
/// program that joins them fails this test, which is the point.
///
/// - `mixed-type-hypothesis.ein` — `Ann` appears in an `hrule`'s `:assert` and
///   in no fact. It is also the only corpus program that breaks **R2**: the
///   name arrives inside the search. It costs nothing there because that
///   query drives the hrule rung, whose candidate set already knew `Ann` —
///   the blind enumerator is what reads `kb.names()`.
/// - `07_schroder.ein` — `G` appears in the probe rule `probe-undecided`'s
///   `:assert` and in no fact. It arrives during **root** saturation, which is
///   before `alive₀` is taken, so it cannot cost anything either.
const KNOWN: [(&str, &str); 2] = [
    (
        "examples/ein-bugs/mixed-type-hypothesis.ein",
        "new-object: `Ann`, asserted by hrule `guess`",
    ),
    (
        "tests/stdlib/algebra/07_schroder.ein",
        "new-object: `G`, asserted by rule `probe-undecided`",
    ),
];

/// The pair this stage added, which is in `KNOWN`'s shape and not in `KNOWN`:
/// it is checked by [`the_pair_differs_by_one_name_and_one_verdict`] instead,
/// where the verdicts are the claim.
const PAIR: [&str; 2] = [
    "examples/ein-bugs/alive-set-fresh-name.ein",
    "examples/ein-bugs/alive-set-fresh-name-declared.ein",
];

fn states_of(a: &Answer) -> Vec<&ein_infer::Solution> {
    match a {
        Answer::Verdict(v) => v.states(),
        Answer::Aborted { .. } => Vec::new(),
    }
}

/// **R0 — the static check, over every corpus program.**
///
/// Free: it reads the registered rules' `:assert` constants once, at load, and
/// never runs the search. That is what makes it a statement about the
/// *program* — a rule that would introduce a name only on an input nobody has
/// written is reported all the same.
#[test]
fn the_corpus_breaks_the_invariant_in_exactly_the_places_named_here() {
    let mut found: Vec<(String, String)> = Vec::new();
    let mut programs = 0usize;
    for path in &corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .display()
            .to_string();
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        programs += 1;
        if PAIR.contains(&rel.as_str()) {
            continue;
        }
        let u = Universe::of(&kb, &terms);
        for b in u.rule_breaches(&kb, &terms, &ast) {
            found.push((rel.clone(), b.render(&terms)));
        }
    }
    assert!(
        programs >= 160,
        "only {programs} corpus files loaded — the sweep stopped looking"
    );
    found.sort();
    let mut want: Vec<(String, String)> = KNOWN
        .iter()
        .map(|(p, b)| (p.to_string(), b.to_string()))
        .collect();
    want.sort();
    assert_eq!(
        found, want,
        "the set of corpus programs that break the M1 alive-set invariant \
         moved. A new one is a finding; a missing one means the program was \
         fixed and this list has to say so."
    );
}

/// **R1 and R2 — what a run actually reaches, against what R0 predicted.**
///
/// The dynamic half exists to confirm the induction the static half assumes:
/// a *variable* leaf can only carry a value the KB already holds, so nothing
/// but a rule's constant can introduce a name. Measured over the corpus, the
/// dynamic sweep finds **no name R0 did not**, which is what licenses shipping
/// the free check rather than a per-fixpoint scan.
///
/// It also separates the two readings by measurement rather than by argument:
/// three sites break R1 and **one** breaks R2, and R2 is the one that can cost
/// an answer.
#[test]
fn no_run_reaches_a_name_the_static_check_did_not_predict() {
    let mut r1: Vec<(String, String)> = Vec::new();
    let mut r2: Vec<(String, String)> = Vec::new();
    let mut solved = 0usize;
    for path in &corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(path)
            .display()
            .to_string();
        if PAIR.contains(&rel.as_str()) {
            continue;
        }
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let Ok(mut kb) = load_file(&mut ast, &mut terms, path) else {
            continue;
        };
        let loaded = Universe::of(&kb, &terms);
        let at_load = kb.n_facts();
        let predicted: Vec<String> = loaded
            .rule_breaches(&kb, &terms, &ast)
            .iter()
            .map(|b| terms.sym(b.name).to_string())
            .collect();
        let mut events = Events::off();
        if ein_infer::saturate_events(&ast, &mut terms, &mut kb).is_err() {
            continue;
        }
        let at_root = kb.n_facts();
        let root_universe = Universe::of(&kb, &terms);

        let collect = |u: &Universe,
                       k: &ein_core::Kb,
                       terms: &Terms,
                       from: usize,
                       into: &mut Vec<(String, String)>| {
            let mut b = Vec::new();
            u.breaches(k, terms, from, &mut b);
            for x in b {
                let name = terms.sym(x.name).to_string();
                assert!(
                    predicted.contains(&name),
                    "{rel}: the run named `{name}`, which the static check did \
                     not predict — the induction that a variable can only carry \
                     a value the KB already holds is false, or the baseline is \
                     wrong"
                );
                let row = (rel.clone(), name);
                if !into.contains(&row) {
                    into.push(row);
                }
            }
        };
        collect(&loaded, &kb, &terms, at_load, &mut r1);

        let opts = SolveOptions {
            stop_after: None,
            max_enterings: Some(60),
            on_budget: OnBudget::Verdict,
            ..SolveOptions::default()
        };
        let Ok(answer) = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts) else {
            continue;
        };
        solved += 1;
        for s in states_of(&answer.answer) {
            collect(&loaded, &s.kb, &terms, at_load, &mut r1);
            collect(&root_universe, &s.kb, &terms, at_root, &mut r2);
        }
    }
    assert!(
        solved >= 155,
        "only {solved} corpus files reached a solve — the sweep stopped looking"
    );
    r1.sort();
    r2.sort();
    assert_eq!(
        r1,
        vec![
            (
                "examples/ein-bugs/mixed-type-hypothesis.ein".to_string(),
                "Ann".to_string()
            ),
            (
                "tests/stdlib/algebra/07_schroder.ein".to_string(),
                "G".to_string()
            ),
        ],
        "R1 — what a run derives, against the loaded program's names"
    );
    assert_eq!(
        r2,
        vec![(
            "examples/ein-bugs/mixed-type-hypothesis.ein".to_string(),
            "Ann".to_string()
        )],
        "R2 — what the search reaches, against root's fixpoint. This is the \
         reading `alive` monotonicity rests on"
    );
}

/// **What the invariant is worth: one name, and the verdict inverts.**
///
/// The two files differ by the single fact `(seen Z)`, over a relation nothing
/// else mentions. It changes no rule, no constraint and no goal — all it does
/// is put `Z` in the name universe before `alive₀` is taken.
///
/// Without it the engine answers `Contradiction, k = 0, exhausted = true` — a
/// refutation, not a truncation — and `{(q A Z), (q B Z)}` is a model it never
/// enumerated. With it, that model is the answer.
///
/// And the control does **not** break the invariant at all: `(seen Z)` puts
/// `Z` in the ontology, which is exactly what clause 1 asks for. So the one
/// fact does two things at once and they are the same thing — it makes the
/// program conform, and it makes the answer right.
#[test]
fn the_pair_differs_by_one_name_and_one_verdict() {
    let run = |rel: &str| {
        let path = repo_root().join(rel);
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let mut kb =
            load_file(&mut ast, &mut terms, &path).unwrap_or_else(|e| panic!("{rel}: {e}"));
        let breaches: Vec<String> = Universe::of(&kb, &terms)
            .rule_breaches(&kb, &terms, &ast)
            .iter()
            .map(|b| b.render(&terms))
            .collect();
        let opts = SolveOptions {
            stop_after: None,
            config: Some(kb.program().config.clone().unwrap_or_default()),
            ..SolveOptions::default()
        };
        let mut events = Events::off();
        let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
            .unwrap_or_else(|e| panic!("{rel}: {e:?}"));
        let models: Vec<Vec<String>> = states_of(&solved.answer)
            .iter()
            .map(|s| {
                let mut v: Vec<String> =
                    s.kb.facts()
                        .map(|f| ein_infer::events::sexpr(&terms, f))
                        .filter(|t| t.starts_with("(q "))
                        .collect();
                v.sort();
                v
            })
            .collect();
        let word = solved.answer.as_str().to_string();
        (breaches, word, models, solved.stats.exhausted)
    };

    let (broken_breach, broken_word, broken_models, broken_exhausted) = run(PAIR[0]);
    let (control_breach, control_word, control_models, _) = run(PAIR[1]);

    // The same rule, the same invented name, in both.
    assert_eq!(
        broken_breach,
        vec!["new-object: `Z`, asserted by rule `spawn`".to_string()],
        "the fixture stopped breaking the invariant"
    );
    assert!(
        control_breach.is_empty(),
        "the control is supposed to *conform*: `(seen Z)` is what puts the \
         invented name in the ontology, which is clause 1 satisfied. It \
         reported {control_breach:?}"
    );

    assert_eq!(broken_word, "Contradiction");
    assert!(
        broken_exhausted,
        "the refutation is qualified after all, and the fixture's point is \
         that it is not"
    );
    assert!(broken_models.is_empty());

    assert_eq!(control_word, "Solution");
    assert_eq!(
        control_models,
        vec![vec!["(q A Z)".to_string(), "(q B Z)".to_string()]],
        "the control's model is not the one the twin cannot reach"
    );
}
