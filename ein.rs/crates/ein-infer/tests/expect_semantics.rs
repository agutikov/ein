//! `:expect` — the comparison, and the three rules it implements.
//!
//! M1c
//! [S1c.1.2](../../../../plans/m1c_external_validation/p1c.1_stdlib_conformance/s1c.1.2_test_form.md)
//! T1c.1.2.4. The *shape* is `ein-ir`'s and is tested there; this is what
//! happens when the shape meets an answer.
//!
//! **Every rule here is tested in the failing direction**, because a checker
//! that reports success on a broken expectation is the worst outcome the phase
//! has available to it — the
//! [S1a.6.6](../../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer)
//! lesson, whose fuzzer's own three controls each failed once first.

use ein_core::{Kb, Terms};
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::{Events, expect};
use ein_ir::expect::Expectation;
use ein_ir::{Ast, Node, NodeId};

/// A two-person, two-seat puzzle with exactly two models, and whatever
/// `:expect` the caller wants to try against it.
///
/// `-e`: the set comparison is about *every* model, so the search has to be
/// exhausted before there is a set to compare.
const AMBIGUOUS: &str = r#"
(relation seat Person Slot)
(relation instance Thing Type)
(instance Ann Person) (instance Bob Person)
(instance S1 Slot)    (instance S2 Slot)
(rule one-per-person (?R) :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
      :assert (false) :priority 250 :why "{?a} sits twice")
(rule one-per-seat (?R) :match (and (?R ?a ?c) (?R ?b ?c) (neq ?a ?b))
      :assert (false) :priority 250 :why "{?c} seats two")
(one-per-person seat) (one-per-seat seat)
(hrule guess (?p ?s) :match (instance ?p Person) :assert (seat ?p ?s) :why "guess")
"#;

/// A determinate one: two facts, no search at all.
const DETERMINATE: &str = r#"
(relation p Thing Place)
(p A H1)
(p B H2)
"#;

/// Load `body` plus one query, solve exhaustively, and check the query's
/// `:expect` — the whole pipeline, since half the rules are the loader's.
fn check(body: &str, query: &str) -> Result<expect::Report, String> {
    let src = format!("{body}\n{query}\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, &src, None).map_err(|e| e.to_string())?;
    let mut kb: Kb = ein_ir::load(&mut ast, &mut terms, &forms, None).map_err(|e| e.0)?;
    let opts = SolveOptions {
        stop_after: None,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut events,
        &mut NoDumper,
        &opts,
    )
    .map_err(|e| e.to_string())?;
    let node = expect_node(&ast, &kb).expect("the query carries an :expect");
    let expectation = ein_ir::expect::parse(&ast, node)?;
    Ok(expect::check(&ast, &terms, &expectation, &solved.answer))
}

fn expect_node(ast: &Ast, kb: &Kb) -> Option<NodeId> {
    let query = kb.program().query()?;
    for &pair in query.kw_pairs.iter() {
        if let Node::KwPair { key, value } = ast.node(NodeId(pair.0))
            && let Node::Keyword(name) = ast.node(key)
            && ast.sym(name) == "expect"
        {
            return Some(value);
        }
    }
    None
}

fn holds(body: &str, query: &str) -> bool {
    check(body, query).expect("loads and solves").passed
}

fn why(body: &str, query: &str) -> Vec<String> {
    let report = check(body, query).expect("loads and solves");
    assert!(!report.passed, "expected a failure, and it held");
    report.lines
}

// ── Rule 3: naming a relation closes it ────────────────────────────

#[test]
fn a_complete_extent_holds() {
    assert!(holds(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H2)))"
    ));
}

/// The rule's whole point, and the one a per-fact `:derives` cannot state: a
/// **surplus** fact in a named relation is a failure, and the message says
/// which fact was unexpected.
#[test]
fn a_surplus_fact_in_a_named_relation_fails_and_is_named() {
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))",
    );
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(lines[0].contains("(p B H2)"), "{lines:?}");
    assert!(lines[0].contains("naming a relation closes it"), "{lines:?}");
}

#[test]
fn a_missing_fact_fails_and_is_named() {
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) \
         :expect (model (p A H1) (p B H2) (p C H3)))",
    );
    assert!(
        lines.iter().any(|l| l.contains("(p C H3)") && l.contains("no such fact")),
        "{lines:?}"
    );
}

/// Relations the expectation never mentions are unconstrained — the other half
/// of closure, and what keeps a test off the 250 facts of `is-a*` and
/// activator noise a whole-state golden would pin.
#[test]
fn an_unnamed_relation_is_not_constrained() {
    assert!(
        holds(
            DETERMINATE,
            "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1) (p B H2)))"
        ),
        "`relation` and `instance` facts are in the model and are not listed"
    );
}

/// Closing a relation says nothing about the extent of its stored negatives —
/// otherwise every expectation drags in the negative-completion rules' whole
/// output, which on a Zebra puzzle is most of the model. A `(not …)` is still
/// listable, and is then checked for presence.
#[test]
fn stored_negatives_are_checked_for_presence_and_not_for_extent() {
    let body = "(relation p Thing Place)\n(p A H1)\n(not (p A H2))\n";
    assert!(
        holds(
            body,
            "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))"
        ),
        "the stored (not (p A H2)) is not surplus — closure is positive-only"
    );
    assert!(holds(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) \
         :expect (model (p A H1) (not (p A H2))))"
    ));
    let lines = why(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) \
         :expect (model (p A H1) (not (p A H3))))",
    );
    assert!(
        lines.iter().any(|l| l.contains("(not (p A H3))")),
        "{lines:?}"
    );
}

// ── The verdict, implied ───────────────────────────────────────────

#[test]
fn none_is_the_contradiction_spelling() {
    let body = "(relation p Thing Place)\n(p A H1)\n\
                (rule no (?R) :match (?R ?a ?b) :assert (false) :priority 250 :why \"no\")\n\
                (no p)\n";
    assert!(holds(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect none)"
    ));
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect none)",
    );
    assert!(lines[0].contains("expected none (Contradiction)"), "{lines:?}");
    // …and the other direction, with the message that says what to write.
    let lines = why(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))",
    );
    assert!(lines[0].contains("`:expect none`"), "{lines:?}");
}

#[test]
fn k_is_implied_by_the_number_of_disjuncts() {
    let lines = why(
        AMBIGUOUS,
        "(query :goal (seat ?w ?s) :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (model (seat Ann S1) (seat Bob S2)))",
    );
    assert!(lines[0].contains("k = 1"), "{lines:?}");
    assert!(lines[0].contains("k = 2"), "{lines:?}");
}

// ── `(or …)` is a set ──────────────────────────────────────────────

/// The order the search finds models in is exactly what S1a.7.0's invariance
/// tests assert is not observable, so the comparison is over **sets**: the
/// same two disjuncts in either order hold.
#[test]
fn the_disjuncts_compare_as_a_set_not_a_sequence() {
    let q = |a: &str, b: &str| {
        format!(
            "(query :goal (seat ?w ?s) \
             :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
             :expect (or (model {a}) (model {b})))"
        )
    };
    let one = "(seat Ann S1) (seat Bob S2)";
    let two = "(seat Ann S2) (seat Bob S1)";
    assert!(holds(AMBIGUOUS, &q(one, two)));
    assert!(holds(AMBIGUOUS, &q(two, one)), "the other order too");
}

/// …and the facts inside one disjunct are a set as well.
#[test]
fn the_facts_in_a_model_compare_as_a_set() {
    assert!(holds(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p B H2) (p A H1)))"
    ));
}

/// One wrong disjunct fails even though the *count* is right and the other
/// disjunct is exact — the case a greedy pairing gets wrong.
#[test]
fn one_wrong_disjunct_fails() {
    let lines = why(
        AMBIGUOUS,
        "(query :goal (seat ?w ?s) :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (or (model (seat Ann S1) (seat Bob S2)) \
                     (model (seat Ann S2) (seat Bob S2))))",
    );
    assert!(
        lines.iter().any(|l| l.starts_with("expectation ")),
        "the report says which of the two: {lines:?}"
    );
}

/// Two identical disjuncts against two distinct models: the count matches and
/// every disjunct fits *a* model, but no perfect matching exists. Greedy would
/// pair the first and then fail on the second with a confusing message; the
/// augmenting-path search reports it as what it is.
#[test]
fn two_identical_disjuncts_do_not_cover_two_models() {
    let lines = why(
        AMBIGUOUS,
        "(query :goal (seat ?w ?s) :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (or (model (seat Ann S1) (seat Bob S2)) \
                     (model (seat Ann S1) (seat Bob S2))))",
    );
    assert!(!lines.is_empty(), "a failure with something to say");
}

// ── The rendering agreement ────────────────────────────────────────

/// `ein_ir::expect::render` and `ein_infer::events::sexpr` must agree byte for
/// byte, because the comparison holds one against the other. Facts compare by
/// *content* and never by `FactId` — `fork_audit`'s reason: two runs do not
/// share an interner, and an expectation is written by a person in the first
/// place.
#[test]
fn rendering_agrees_with_the_fact_dump() {
    let src = "(relation p Thing Place)\n(relation q Thing)\n\
               (p A H1)\n(p A 42)\n(q B)\n(not (p A H2))\n";
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, src, None).expect("parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");

    // The same facts, written as an expectation, rendered by the other half.
    let listed = "(query :goal (p A ?h) \
                  :expect (model (p A H1) (p A 42) (q B) (not (p A H2))))";
    let mut ast2 = Ast::new();
    let forms2 = ein_ir::parse(&mut ast2, listed, None).expect("parses");
    let value = {
        let mut found = None;
        for &a in ast2.form_args(forms2[0]) {
            if let Node::KwPair { key, value } = ast2.node(a)
                && let Node::Keyword(k) = ast2.node(key)
                && ast2.sym(k) == "expect"
            {
                found = Some(value);
            }
        }
        found.expect("an :expect")
    };
    let Expectation::One(model) = ein_ir::expect::parse(&ast2, value).expect("parses") else {
        panic!("one model");
    };
    let rendered: Vec<String> = model
        .facts
        .iter()
        .map(|&f| ein_ir::expect::fact(&ast2, f).expect("a fact").rendered)
        .collect();
    let dumped: Vec<String> = kb
        .facts()
        .map(|f| ein_infer::events::sexpr(&terms, f))
        .collect();
    for want in &rendered {
        assert!(
            dumped.contains(want),
            "{want} is not among the fact dump {dumped:?}"
        );
    }
}
