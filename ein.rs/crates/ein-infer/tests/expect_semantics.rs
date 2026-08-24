//! `:expect` — the comparison, and the three rules it implements.
//!
//! M1c
//! [S1c.1.2](../../../../docs/history/m1c_external_validation/README.md#s1c12--how-a-program-states-what-it-expects)
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
    check_stopping(body, query, None)
}

/// The same, with a `-n` cap — one of the two ways to reach a *non-exhausted*
/// answer, which is the state a verdict claim cannot be checked against.
fn check_stopping(
    body: &str,
    query: &str,
    stop_after: Option<u64>,
) -> Result<expect::Report, String> {
    check_with(body, query, stop_after, 5)
}

/// The **other** way: a lattice depth cap with the frontier still alive at it.
/// `-n` can only ever find too few models; a cap can leave *none*, which is a
/// `Contradiction` that has proved nothing — the case
/// [`a_contradiction_from_a_truncated_search_is_not_checked`] pins.
fn check_capped(body: &str, query: &str, max_set_size: u32) -> Result<expect::Report, String> {
    check_with(body, query, None, max_set_size)
}

fn check_with(
    body: &str,
    query: &str,
    stop_after: Option<u64>,
    max_set_size: u32,
) -> Result<expect::Report, String> {
    let src = format!("{body}\n{query}\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, &src, None).map_err(|e| e.to_string())?;
    let mut kb: Kb = ein_ir::load(&mut ast, &mut terms, &forms, None).map_err(|e| e.0)?;
    let opts = SolveOptions {
        stop_after,
        max_set_size,
        ..SolveOptions::default()
    };
    let mut events = Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .map_err(|e| e.to_string())?;
    let node = expect_node(&ast, &kb).expect("the query carries an :expect");
    let expectation = ein_ir::expect::parse(&ast, node)?;
    Ok(expect::check(
        &ast,
        &mut terms,
        &expectation,
        &solved.answer,
        solved.stats.exhausted,
    ))
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
    check(body, query).expect("loads and solves").passed()
}

fn why(body: &str, query: &str) -> Vec<String> {
    let report = check(body, query).expect("loads and solves");
    assert!(!report.passed(), "expected a failure, and it held");
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
/// which fact was unexpected — and, since M1c
/// [T1c.1.3.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test),
/// where it came from on the line under it. `(p B H2)` is authored here, so
/// the provenance is the program's own text; `a_surplus_fact_names_the_rule_that_derived_it`
/// is the case that matters.
#[test]
fn a_surplus_fact_in_a_named_relation_fails_and_is_named() {
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))",
    );
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[0].contains("(p B H2)"), "{lines:?}");
    assert!(
        lines[0].contains("naming a relation closes it"),
        "{lines:?}"
    );
    assert!(lines[1].contains("in the program's own text"), "{lines:?}");
}

/// **The line that is worth the plumbing.** A surplus fact a *rule* put there
/// is the shape of the bug this milestone is written around —
/// `disjunctive-prune` derived one for a year — and the next question after
/// "there is an extra fact here" is always "which rule?". One level of
/// premises, because the rest of the chain is `--trace`'s.
#[test]
fn a_surplus_fact_names_the_rule_that_derived_it() {
    let lines = why(
        "(import std.algebra :symbols (symmetric))\n\
         (relation p Thing Thing)\n(symmetric p)\n(p A B)\n",
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A B)))",
    );
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[0].contains("(p B A)"), "the mirrored edge: {lines:?}");
    assert!(
        lines[1].contains("derived by symmetric from (p A B)"),
        "the rule and its premise: {lines:?}"
    );
}

#[test]
fn a_missing_fact_fails_and_is_named() {
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) \
         :expect (model (p A H1) (p B H2) (p C H3)))",
    );
    assert!(
        lines
            .iter()
            .any(|l| l.contains("(p C H3)") && l.contains("no such fact")),
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
fn bottom_is_the_contradiction_spelling() {
    let body = "(relation p Thing Place)\n(p A H1)\n\
                (rule no (?R) :match (?R ?a ?b) :assert (false) :priority 250 :why \"no\")\n\
                (no p)\n";
    assert!(holds(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (false))"
    ));
    let lines = why(
        DETERMINATE,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (false))",
    );
    assert!(lines[0].contains("expected (false)"), "{lines:?}");
    // …and the other direction, with the message that says what to write.
    let lines = why(
        body,
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))",
    );
    assert!(lines[0].contains("`:expect (false)`"), "{lines:?}");
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

/// …and the count is followed by **what the models were**, projected through
/// the query's own `:goal` — M1c
/// [T1c.1.3.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test).
/// "You said one and I found two" without saying what the second one was
/// leaves the reader to go and re-run the search by hand, which is the thing
/// this whole form is for not having to do.
#[test]
fn a_count_mismatch_says_what_the_models_were() {
    let lines = why(
        AMBIGUOUS,
        "(query :goal (seat ?w ?s) :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (model (seat Ann S1) (seat Bob S2)))",
    );
    assert_eq!(
        lines.len(),
        3,
        "the count, then one line per model: {lines:?}"
    );
    let rows: Vec<&String> = lines[1..].iter().collect();
    assert!(
        rows.iter()
            .all(|l| l.contains("model ") && l.contains("of 2")),
        "{rows:?}"
    );
    // Both seatings, each shown as the goal's own variables.
    let both = format!("{}{}", rows[0], rows[1]);
    for want in [
        "?w=Ann ?s=S1",
        "?w=Bob ?s=S2",
        "?w=Ann ?s=S2",
        "?w=Bob ?s=S1",
    ] {
        let cells: Vec<&str> = want.split(' ').collect();
        assert!(
            both.contains(cells[1]) && both.contains(cells[0]),
            "{want} is not in {rows:?}"
        );
    }
    // **Sorted**, because the row order a goal projection happens to produce
    // is `defined_behaviour.md` §6's under-determined one — a report that
    // inherited it could not be diffed.
    for row in &rows {
        let cells: Vec<&str> = row
            .split(": ")
            .nth(1)
            .expect("a projection")
            .split("; ")
            .collect();
        let mut sorted = cells.clone();
        sorted.sort();
        assert_eq!(cells, sorted, "{row}");
    }
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

// ── An expectation is a claim about the *exhausted* answer ─────────

const TWO_MODELS: &str = "(query :goal (seat ?w ?s) \
     :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
     :expect (or (model (seat Ann S1) (seat Bob S2)) \
                 (model (seat Ann S2) (seat Bob S1))))";

/// The hole this outcome exists to close. Stopped at two models, the run has
/// found both of the listed ones — and has proved only that there are *at
/// least* two. Reporting that as a pass would be a green result for the half
/// of the claim ("and no third") that nobody checked.
#[test]
fn an_unexhausted_search_does_not_confirm_a_verdict() {
    let stopped = check_stopping(AMBIGUOUS, TWO_MODELS, Some(2)).expect("solves");
    assert_eq!(stopped.outcome, expect::Outcome::NotChecked);
    assert!(!stopped.passed(), "NotChecked is not success");
    assert!(
        stopped.lines[0].contains("not exhausted") && stopped.lines[0].contains("lower bound"),
        "{:?}",
        stopped.lines
    );

    let exhausted = check_stopping(AMBIGUOUS, TWO_MODELS, None).expect("solves");
    assert_eq!(
        exhausted.outcome,
        expect::Outcome::Held,
        "the same claim, proved"
    );
}

/// …and it bites only where more searching could have changed the answer.
/// Finding **more** models than were claimed is a refutation whatever the
/// search did next, so it stays a failure rather than becoming "not checked".
#[test]
fn too_many_models_is_a_failure_not_an_unchecked_one() {
    let one = "(query :goal (seat ?w ?s) \
         :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (model (seat Ann S1) (seat Bob S2)))";
    let r = check_stopping(AMBIGUOUS, one, Some(2)).expect("solves");
    assert_eq!(r.outcome, expect::Outcome::Failed, "{:?}", r.lines);
}

/// **A `k = 0` from a truncated search is not a refutation either**, and it
/// used to be reported as one.
///
/// `MonotonicStats::exhausted`'s own words: a `k = 0` from a truncated run is
/// "no model within the cap", not proven unsat. The `Contradiction` arm short-
/// circuited above the shortfall check that says so, so a claim of two models
/// against a depth-1 search came back `FAILED` — a refutation on the strength
/// of a search that stopped. Found by M1c
/// [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test),
/// where `ein test` exhausts by default and `--max-set-size` is the only thing
/// left that can truncate a run.
#[test]
fn a_contradiction_from_a_truncated_search_is_not_checked() {
    // Both models seat two people, so nothing is complete at depth 1.
    let capped = check_capped(AMBIGUOUS, TWO_MODELS, 1).expect("solves");
    assert_eq!(
        capped.outcome,
        expect::Outcome::NotChecked,
        "{:?}",
        capped.lines
    );
    assert!(
        capped.lines[0].contains("no model within the cap"),
        "and says which cap to raise: {:?}",
        capped.lines
    );

    // The same claim at a cap that admits the answer.
    let deep = check_capped(AMBIGUOUS, TWO_MODELS, 5).expect("solves");
    assert_eq!(deep.outcome, expect::Outcome::Held);
}

/// …and an *exhausted* `Contradiction` against a model claim still fails, so
/// the guard above did not turn a refutation into a shrug.
#[test]
fn an_exhausted_contradiction_still_refutes_a_model_claim() {
    let r = check(
        "(relation p Thing Place)\n(p A H1)\n\
         (rule no (?R) :match (?R ?a ?b) :assert (false) :priority 250)\n(no p)\n",
        "(query :goal (p A ?h) :no-hypothesis (p) :expect (model (p A H1)))",
    )
    .expect("solves");
    assert_eq!(r.outcome, expect::Outcome::Failed, "{:?}", r.lines);
    assert!(r.lines[0].contains("`:expect (false)`"), "{:?}", r.lines);
}

/// A model that disagrees with the expectation it was matched to is a
/// disagreement about *content*, and no amount of further search unfinds it.
#[test]
fn a_wrong_model_is_a_failure_even_unexhausted() {
    let wrong = "(query :goal (seat ?w ?s) \
         :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)) \
         :expect (or (model (seat Ann S1) (seat Bob S1)) \
                     (model (seat Ann S2) (seat Bob S2))))";
    let r = check_stopping(AMBIGUOUS, wrong, Some(2)).expect("solves");
    assert_eq!(r.outcome, expect::Outcome::Failed, "{:?}", r.lines);
}
