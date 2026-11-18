//! **The kernel's dunder is never an object of the blind enumerator** — what
//! [`ein_infer::hypgen::candidate_objects`] takes for an object where a
//! program holds `__closed__`.
//!
//! The KB's names index holds the head of every fact, and the registry the
//! head of every fact a file writes. So a dunder whose every fact a rule
//! derives was in the index and in no registry, and the blind enumerator
//! proposed facts about it: one closure stated two ways, by a fact and by a
//! rule, gave two answers. A dunder a program writes in an argument position
//! was an object the same way, by the list of the program's names. This file
//! holds that neither is.
//!
//! 1. **Two ways of stating one closure have one H and one
//!    R** ([`pairs`]): the form whose rule derives the dunder's fact answers
//!    what the form that writes the fact answers, candidate for candidate and
//!    commitment for commitment, by the cube
//!    ([`ein_infer::region::compute`], `ein oracle`'s library) and by a bare
//!    describe ([`ein_infer::explore::run`]). An invariance, so it does not
//!    rest on a number: the regression's witness and its control are one of the
//!    pairs, read from their files.
//! 2. **A dunder written as a value is no object**
//!    ([`a_dunder_written_as_a_value_is_no_object`]): the program that hands
//!    `__closed__` to a rule as a relation parameter answers what it answers
//!    with a fact of the dunder written beside it.
//! 3. **No candidate names a dunder**, on any of those programs and on
//!    `std.math.arith` beside an open relation.
//! 4. **The list of the program's names is what it was**
//!    ([`ein_infer::invariant::program_names`]): it holds a dunder exactly
//!    where a program writes one in an argument position, which stays legal,
//!    and the alive-set invariant reads that list. Only the enumerator's
//!    objects leave the dunder out.

use std::collections::BTreeSet;

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode as Question, Which};
use ein_infer::oblgen::Mode;
use ein_infer::region::{self, Outcome};
use ein_ir::{Ast, load, parse};

/// A program, by the name a failure reports it under.
struct Program {
    name: String,
    text: String,
}

fn inline(name: &str, text: &str) -> Program {
    Program {
        name: name.to_string(),
        text: text.to_string(),
    }
}

fn file(path: &str) -> Program {
    let full = repo_root().join(path);
    Program {
        name: path.to_string(),
        text: std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{path}: {e}")),
    }
}

fn loaded(p: &Program) -> (Kb, Terms, Ast) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms =
        parse(&mut ast, &p.text, Some(&p.name)).unwrap_or_else(|e| panic!("{}: {e}", p.name));
    let kb = load(&mut ast, &mut terms, &forms, None).unwrap_or_else(|e| panic!("{}: {e}", p.name));
    (kb, terms, ast)
}

/// What a program answers: the rung that built H, H in its order, R's
/// commitments by the cube, and how many states they make.
#[derive(Debug, PartialEq, Eq)]
struct Answered {
    rung: Mode,
    universe: Vec<String>,
    members: BTreeSet<Vec<String>>,
    states: usize,
}

/// The program's answer by the cube, held to a bare describe's counts on the
/// way.
fn answered(p: &Program) -> Answered {
    let (mut kb, mut terms, ast) = loaded(p);
    let checked = explore::check_monotone(
        &mut kb,
        &mut terms,
        &ast,
        &mut ein_infer::events::Events::off(),
    )
    .unwrap_or_else(|e| panic!("{}: {e}", p.name));
    let rung = checked.root.hypgen.rung.mode;

    let (mut kb, mut terms, ast) = loaded(p);
    let out = region::compute(
        &mut kb,
        &mut terms,
        &ast,
        &region::Options::default(),
        &mut |_| {},
    )
    .unwrap_or_else(|e| panic!("{}: {e}", p.name));
    let Outcome::Region(r) = out else {
        panic!("{}: the cube is within the oracle's limit", p.name);
    };
    let n = r.n();
    let universe: Vec<String> = r.candidates.iter().map(|c| c.text.clone()).collect();
    assert_eq!(universe.len(), checked.n, "{}: one H on both paths", p.name);
    let in_r: Vec<u64> = (0..1u64 << n).filter(|&m| r.in_r(m)).collect();
    let members: BTreeSet<Vec<String>> = in_r
        .iter()
        .map(|&m| {
            (0..n)
                .filter(|i| m >> i & 1 == 1)
                .map(|i| universe[i].clone())
                .collect()
        })
        .collect();
    let states: BTreeSet<u64> = in_r.iter().map(|&m| r.nodes[m as usize].class).collect();

    let (mut kb, mut terms, ast) = loaded(p);
    let describe = Question {
        which: Which::All,
        ask: Ask::Describe,
    };
    let a = explore::run(&mut kb, &mut terms, &ast, &explore::Options::new(describe))
        .unwrap_or_else(|e| panic!("{}: {e}", p.name));
    assert!(a.exhausted, "{}: describe exhausts R", p.name);
    let counts = a
        .counts
        .unwrap_or_else(|| panic!("{}: describe counts R", p.name));
    assert_eq!(
        (counts.k_c, counts.k_s),
        (members.len() as u64, states.len() as u64),
        "{}: describe's R is the cube's",
        p.name
    );
    Answered {
        rung,
        universe,
        members,
        states: states.len(),
    }
}

const DUNDERS: [&str; 1] = [ein_core::terms::CLOSED];

/// The candidates of `universe` that name a kernel dunder in an argument
/// position.
fn naming_a_dunder(universe: &[String]) -> Vec<&String> {
    universe
        .iter()
        .filter(|c| {
            c.split([' ', '(', ')'])
                .filter(|w| !w.is_empty())
                .skip(1)
                .any(|w| DUNDERS.contains(&w))
        })
        .collect()
}

/// A closed table beside an open `total`.
const DIGITS: &str = "\
(relation digit Digit :why \"{?3} is a digit\")
(digit 0) (digit 1) (digit 2)
(relation total Number :why \"the total is {?3}\")
";

/// Each pair: one program whose rule derives the dunder's fact, and the same
/// program with the fact written.
fn pairs() -> Vec<(&'static str, Program, Program)> {
    vec![
        (
            "the regression's witness and its control",
            file("examples/ein-bugs/wrong-answers/dunder-object-closed-by-rule.ein"),
            file("examples/ein-bugs/wrong-answers/dunder-object-closed-by-fact.ein"),
        ),
        (
            "a closure, by a rule with an empty :match and by a fact",
            inline(
                "closed by a rule",
                &format!(
                    "{DIGITS}(rule digits-closed () :match () :assert (__closed__ digit) \
                     :why \"the digits are all written\")"
                ),
            ),
            inline("closed by a fact", &format!("{DIGITS}(__closed__ digit)")),
        ),
        (
            "a closure, by a rule that reads the table and by a fact",
            inline(
                "closed by a rule over the table",
                &format!(
                    "{DIGITS}(rule digits-closed () :match (digit 0) :assert (__closed__ digit) \
                     :why \"the table that holds 0 is all written\")"
                ),
            ),
            inline("closed by a fact", &format!("{DIGITS}(__closed__ digit)")),
        ),
    ]
}

/// **One closure stated two ways is one closure.** The form whose rule
/// derives the dunder's fact and the form that writes it have the same H, in
/// the same order, and the same R, commitment for commitment and state for
/// state; neither H names a dunder.
#[test]
fn a_closure_by_a_rule_answers_what_the_closure_by_a_fact_answers() {
    for (name, by_rule, by_fact) in pairs() {
        let (rule, fact) = (answered(&by_rule), answered(&by_fact));
        assert_eq!(rule, fact, "{name}: the two forms answer alike");
        assert_eq!(rule.rung, Mode::Blind, "{name}: the blind enumerator's H");
        assert!(
            !rule.universe.is_empty(),
            "{name}: the pair proposes something, or it compares nothing"
        );
        let named = naming_a_dunder(&rule.universe);
        assert!(named.is_empty(), "{name}: H names a dunder: {named:?}");
    }
}

/// **`std.math.arith` beside an open relation proposes nothing.** The
/// module's closing rule derives its closures and it writes no `(__closed__
/// R)` fact; the program writes no name, so the one thing the enumerator had
/// to propose about was the dunder. It answers what the program answers that
/// closes `total` itself: no candidate, and R is {∅}.
#[test]
fn the_shipped_closing_rule_leaves_no_dunder_to_propose() {
    let open = answered(&file(
        "examples/ein-bugs/wrong-answers/dunder-object-arith.ein",
    ));
    let closed = answered(&inline(
        "std.math.arith, total closed",
        "(import std.math.arith)\n(relation total Number :why \"the total is {?3}\")\n\
         (__closed__ total)",
    ));
    assert_eq!(
        open, closed,
        "nothing is left to propose about `total` but the kernel's word"
    );
    assert!(open.universe.is_empty(), "{:?}", open.universe);
    assert_eq!(open.states, 1, "R is {{∅}}");
}

/// A dunder written as a value, as `std.algebra`'s `(imply1 functional
/// __closed__)` writes one: the rule `lift` is handed the dunder and derives
/// `(__closed__ thing)`. The regressions' fourth witness.
const AS_VALUE: &str = "examples/ein-bugs/wrong-answers/dunder-object-as-value.ein";

/// **A dunder a program writes as a value is no object either.** It is a
/// constant in an argument position of a fact, and still no thing the puzzle
/// is about. The program answers what it answers with a fact of the dunder
/// written beside it, which registers the dunder as a relation and changes
/// nothing else: one H and one R, and neither names the dunder.
#[test]
fn a_dunder_written_as_a_value_is_no_object() {
    let witness = file(AS_VALUE);
    let as_value = answered(&witness);
    let registered = answered(&inline(
        "a dunder as a value, and a fact of it written",
        &format!("{}\n(__closed__ thing)\n", witness.text),
    ));
    assert_eq!(as_value, registered, "the two forms answer alike");
    assert_eq!(as_value.rung, Mode::Blind);
    assert_eq!(as_value.universe, ["(total a)"]);
    assert_eq!(as_value.states, 2, "the whole cube over the one candidate");
}

/// **The list of the program's names does not move**: it holds a dunder
/// exactly where a program writes it in an argument position, so the
/// alive-set invariant's baseline is what it was. What leaves the dunder out
/// is the enumerator's own list of objects.
#[test]
fn the_programs_names_hold_a_dunder_where_it_is_written_as_a_value() {
    let dunder_in = |p: &Program| -> (bool, bool) {
        let (kb, terms, ast) = loaded(p);
        let named = ein_infer::invariant::program_names(&kb, &terms, &ast)
            .iter()
            .any(|n| DUNDERS.contains(&terms.sym(n.name)));
        let object = ein_infer::candidate_objects(&kb, &terms, &ast)
            .iter()
            .any(|&o| DUNDERS.contains(&terms.sym(o)));
        (named, object)
    };
    for (name, by_rule, by_fact) in pairs() {
        assert_eq!(dunder_in(&by_rule), (false, false), "{name}, by a rule");
        assert_eq!(dunder_in(&by_fact), (false, false), "{name}, by a fact");
    }
    assert_eq!(
        dunder_in(&file(AS_VALUE)),
        (true, false),
        "a name of the program, and no object"
    );
}
