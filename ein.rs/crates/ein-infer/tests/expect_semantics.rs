//! A query's `:expect` — the comparison, since M1e2 S1e2.1a.5 over
//! **projection classes** (`region_language.md` § 5).
//!
//! A query's `:expect` lists its classes: each `(model …)` is the goal's atoms
//! under one class of rows over the set its `:over` names, `(model)` the class
//! of states where the goal has no row, and `(false)` no class at all. The
//! classes compare as a set, and the read-out checks them against the run that
//! computed them ([`ein_infer::read_out`]). What a query *could* state before —
//! the whole state, with naming a relation closing it — is the region claim's
//! now, `(solve … :expect (region …))`, and `claim_round_trip.rs` and
//! `tests/region/` are its tests. The walk's comparison, `expect::check`, has
//! no caller since the switch.
//!
//! **Every rule here is tested in the failing direction**, because a checker
//! that reports success on a broken expectation is the worst outcome the
//! language has, and a control is not trusted until it has been seen to fail.

use ein_core::Terms;
use ein_infer::expect::Outcome;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out::{self, Query};
use ein_ir::expect::Expectation;
use ein_ir::{Ast, Node};

/// Two people, two seats, all-different both ways, and a restrictions-only
/// program: R is seven states — ∅, the four single seatings, the two full
/// ones — and its maximal line the two full ones.
const SEATS: &str = r#"
(relation instance Thing Type :why "{?1} is an instance of {?2}")
(relation seat Person Slot :why "{?1} sits in {?2}")
(instance Ann Person) (instance Bob Person)
(instance S1 Slot)    (instance S2 Slot)
(rule one-per-person (?R) :match (and (?R ?a ?b) (?R ?a ?c) (neq ?b ?c))
      :assert (false) :why "{?R} gives {?a} both {?b} and {?c}, and {?a} may have only one")
(rule one-per-seat (?R) :match (and (?R ?a ?c) (?R ?b ?c) (neq ?a ?b))
      :assert (false) :why "{?R} gives {?c} to both {?a} and {?b}, and {?c} may go to only one")
(one-per-person seat) (one-per-seat seat)
(hrule guess (?p ?s) :match (instance ?p Person) :assert (seat ?p ?s) :why "guess that {?p} sits in {?s}")
(solve :hrules (guess (Ann S1) (Ann S2) (Bob S1) (Bob S2)))
"#;

/// A program whose root is dead: R is empty, every node dead.
const DEAD: &str = r#"
(relation p T :why "p holds of {?1}")
(rule no () :match (p A) :assert (false) :why "p holds of A, which cannot be")
(p A)
"#;

const BOTH_FULL: &str = "(or (model (seat Ann S1) (seat Bob S2)) \
                             (model (seat Ann S2) (seat Bob S1)))";

/// Load `body` plus `query`, ask `mode`, and hand back the query's answer.
fn answer_with(body: &str, query: &str, mode: Mode, budget: Option<u64>) -> Result<Query, String> {
    let src = format!("{body}\n{query}\n");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, &src, None).map_err(|e| e.to_string())?;
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, None).map_err(|e| e.0)?;
    let mut opts = explore::Options::new(mode);
    if let Some(b) = budget {
        opts.budget = b;
    }
    let r = read_out::run(&mut kb, &mut terms, &ast, &opts).map_err(|e| e.to_string())?;
    Ok(r.queries
        .into_iter()
        .next()
        .expect("the query was answered"))
}

const DESCRIBE: Mode = Mode {
    which: Which::All,
    ask: Ask::Describe,
};

fn answer(body: &str, query: &str) -> Query {
    answer_with(body, query, DESCRIBE, None).expect("loads and runs")
}

fn outcome(q: &Query) -> Outcome {
    q.claim
        .as_ref()
        .expect("the query carries an :expect")
        .outcome
}

fn lines(q: &Query) -> Vec<String> {
    q.claim
        .as_ref()
        .expect("the query carries an :expect")
        .lines
        .clone()
}

// ── What a class is ────────────────────────────────────────────────

/// Over all of R the seat goal has seven classes, one per state, `(model)`
/// among them — ∅ holds no seat — and the claim that lists exactly those
/// holds.
#[test]
fn the_classes_over_all_of_r_hold_when_listed_whole() {
    let all = "(or (model) (model (seat Ann S1)) (model (seat Ann S2)) \
                (model (seat Bob S1)) (model (seat Bob S2)) \
                (model (seat Ann S1) (seat Bob S2)) (model (seat Ann S2) (seat Bob S1)))";
    let q = answer(SEATS, &format!("(query :goal (seat ?w ?s) :expect {all})"));
    assert!(q.exact && q.as_asked, "{q:?}");
    assert_eq!(q.classes.len(), 7);
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
}

/// The same two full seatings, asked `:over maximal`, are the whole answer;
/// asked over all of R they are two classes of seven, and the claim fails
/// naming a class it does not list.
#[test]
fn over_selects_the_states_and_a_missing_class_is_named() {
    let q = answer(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :over maximal :expect {BOTH_FULL})"),
    );
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
    let q = answer(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :expect {BOTH_FULL})"),
    );
    assert_eq!(outcome(&q), Outcome::Failed);
    let l = lines(&q);
    assert!(
        l[0].starts_with("the claim lists 2 classes, the query has 7: (model) is not listed"),
        "{l:?}"
    );
    assert!(
        l.iter()
            .any(|x| x == "(model (seat Ann S1)) is a class the claim does not list"),
        "{l:?}"
    );
}

/// A listed class the query does not have fails too, and says so.
#[test]
fn an_extra_class_is_named() {
    let q = answer(
        SEATS,
        "(query :goal (seat ?w ?s) :over maximal \
         :expect (or (model (seat Ann S1) (seat Bob S2)) \
                     (model (seat Ann S2) (seat Bob S1)) \
                     (model (seat Ann S1))))",
    );
    assert_eq!(outcome(&q), Outcome::Failed);
    let l = lines(&q);
    assert!(
        l[0].contains("the claim lists 3 classes, the query has 2")
            && l[0].contains("(model (seat Ann S1)) is no class of the maximal line"),
        "{l:?}"
    );
}

/// The classes compare as a set, and so do the facts in one.
#[test]
fn the_classes_and_their_facts_compare_as_sets() {
    let q = answer(
        SEATS,
        "(query :goal (seat ?w ?s) :over maximal \
         :expect (or (model (seat Bob S1) (seat Ann S2)) \
                     (model (seat Bob S2) (seat Ann S1))))",
    );
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
}

/// `(model)` is the class where the goal has no row: on the minimal line of a
/// restrictions-only program that is ∅ alone.
#[test]
fn an_undetermined_goal_is_the_empty_class() {
    let q = answer_with(
        SEATS,
        "(query :goal (seat ?w ?s) :over minimal :expect (model))",
        Mode {
            which: Which::Minimal,
            ask: Ask::Describe,
        },
        None,
    )
    .expect("runs");
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
    let q = answer(
        SEATS,
        "(query :goal (seat ?w ?s) :over minimal :expect (model))",
    );
    assert_eq!(
        outcome(&q),
        Outcome::Held,
        "the description's minimal line is ∅ too"
    );
}

/// `(false)` is no class: it holds exactly when the set is empty.
#[test]
fn false_is_no_class() {
    let q = answer(DEAD, "(query :goal (p ?x) :expect (false))");
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
    let q = answer(
        SEATS,
        "(query :goal (seat ?w ?s) :over maximal :expect (false))",
    );
    assert_eq!(outcome(&q), Outcome::Failed);
    assert!(lines(&q)[0].starts_with("the claim lists 0 classes, the query has 2"));
}

// ── When a claim is not checked ────────────────────────────────────

/// A claim is checked against the classes over the set it asks about, and
/// complete: a run that asked another set, or stopped, did not decide it.
#[test]
fn a_claim_the_run_did_not_decide_is_not_checked() {
    let claim = format!("(query :goal (seat ?w ?s) :over maximal :expect {BOTH_FULL})");
    // `--minimal` asked another line.
    let q = answer_with(
        SEATS,
        &claim,
        Mode {
            which: Which::Minimal,
            ask: Ask::Describe,
        },
        None,
    )
    .expect("runs");
    assert!(!q.as_asked);
    assert_eq!(outcome(&q), Outcome::NotChecked);
    assert_eq!(
        lines(&q),
        ["the claim is about the maximal line, and this run asked its minimal line"]
    );
    // `-n 1` asked one state.
    let q = answer_with(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :expect {BOTH_FULL})"),
        Mode {
            which: Which::All,
            ask: Ask::Find(1),
        },
        None,
    )
    .expect("runs");
    assert_eq!(outcome(&q), Outcome::NotChecked);
    assert!(
        lines(&q)[0].contains("1 of its states (-n 1)"),
        "{:?}",
        lines(&q)
    );
    // A budget that stops the description.
    let q = answer_with(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :expect {BOTH_FULL})"),
        DESCRIBE,
        Some(1),
    )
    .expect("runs");
    assert!(!q.exact);
    assert_eq!(outcome(&q), Outcome::NotChecked);
    assert_eq!(
        lines(&q),
        ["the claim is about all of R, and this run did not exhaust it"]
    );
}

/// The `--maximal` run asked the claim's own line, and decides it.
#[test]
fn a_line_mode_decides_a_claim_over_its_line() {
    let q = answer_with(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :over maximal :expect {BOTH_FULL})"),
        Mode {
            which: Which::Maximal,
            ask: Ask::Describe,
        },
        None,
    )
    .expect("runs");
    assert!(q.as_asked && q.exact, "{q:?}");
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
}

/// `--single` that decided one state has listed all of R, so a claim over
/// all of R, or over either line, is decided by it; one that found two
/// states has not, and its claim is not checked.
#[test]
fn a_decided_single_decides_a_claim_over_all_of_r() {
    // One guest who must sit, one seat: ∅ owes, and R is the one seating.
    const ONE: &str = r#"
(import std.algebra :symbols (total-owed))
(relation is-a T T :why "{?1} is a {?2}")
(relation seat Person Slot :why "{?1} sits in {?2}")
(is-a Ann Person) (is-a S1 Slot)
(total-owed seat is-a)
(hrule guess (?p ?s) :match (is-a ?p Person) :assert (seat ?p ?s) :why "guess that {?p} sits in {?s}")
(solve :hrules (guess (Ann S1)))
"#;
    let single = Mode {
        which: Which::All,
        ask: Ask::Single,
    };
    for over in ["", ":over maximal", ":over minimal"] {
        let q = answer_with(
            ONE,
            &format!("(query :goal (seat ?w ?s) {over} :expect (model (seat Ann S1)))"),
            single,
            None,
        )
        .expect("runs");
        assert!(q.as_asked && q.exact, "{over}: {q:?}");
        assert_eq!(outcome(&q), Outcome::Held, "{over}: {:?}", lines(&q));
    }
    let q = answer_with(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :expect {BOTH_FULL})"),
        single,
        None,
    )
    .expect("runs");
    assert!(!q.exact, "Ambiguity listed two states of seven");
    assert_eq!(outcome(&q), Outcome::NotChecked);
}

/// An `-n` run that exhausted its part found fewer states than it asked for,
/// so it listed the whole part and decides a claim over it.
#[test]
fn an_exhausted_find_decides_its_part() {
    let q = answer_with(
        SEATS,
        &format!("(query :goal (seat ?w ?s) :over maximal :expect {BOTH_FULL})"),
        Mode {
            which: Which::Maximal,
            ask: Ask::Find(5),
        },
        None,
    )
    .expect("runs");
    assert!(q.exact, "{q:?}");
    assert_eq!(outcome(&q), Outcome::Held, "{:?}", lines(&q));
}

// ── The rendering agreement ────────────────────────────────────────

/// `ein_ir::expect::render` and `ein_infer::events::sexpr` must agree byte for
/// byte, because the comparison holds one against the other. Facts compare by
/// *content* and never by `FactId` — `fork_audit`'s reason: two runs do not
/// share an interner, and an expectation is written by a person in the first
/// place.
#[test]
fn rendering_agrees_with_the_fact_dump() {
    let src = "(relation p Thing Place :why \"p holds of {?1} and {?2}\")\n(relation q Thing :why \"q holds of {?1}\")\n\
               (p A H1)\n(p A 42)\n(q B)\n(not (p A H2))\n";
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = ein_ir::parse(&mut ast, src, None).expect("parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");

    // The same facts, written as an expectation, rendered by the other half.
    // Parsed and not loaded: the facts are no query's classes, and the shape
    // is what is under test.
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
    let Expectation::One(model) = ein_ir::expect::parse(&ast2, value) else {
        panic!("one model");
    };
    let rendered: Vec<String> = model
        .facts
        .iter()
        .map(|&f| ein_ir::expect::fact(&ast2, f).rendered)
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
