//! P1a.4's acceptance gate — the three fixtures `ein.py/acceptance/` runs,
//! against ein.rs.
//!
//! `hypgen_parity.rs` proves the two implementations *agree*; these prove the
//! answer is the **right** one. That distinction matters at exactly one point:
//! a port can agree with an oracle about a wrong model, and the corpus diff
//! would be green. So these assert the Zebra puzzle's published answer, read
//! through each encoding's own vocabulary.
//!
//! The Python originals live outside `ein.py/tests/` because an exhaustive
//! `zebra.ein` solve is ~21 s under PyPy and much worse under CPython. Here it
//! is 0.6 s, so they run with the ordinary suite.

use ein_core::{Kb, Terms, Value};
use ein_infer::solve::{NoDumper, SolveOptions, solve};
use ein_infer::verdict::{Answer, Verdict};
use ein_infer::{Events, Solved};
use ein_ir::{Ast, load_file};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

fn run(rel: &str, stop_after: Option<u64>) -> (Solved, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut events = Events::off();
    let opts = SolveOptions {
        stop_after,
        ..SolveOptions::default()
    };
    let solved =
        solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).expect("solves");
    (solved, terms)
}

fn model(solved: &Solved) -> &Kb {
    match &solved.answer {
        Answer::Verdict(Verdict::Solution(s)) => &s.kb,
        other => panic!("expected a Solution, got {}", other.as_str()),
    }
}

/// The Wikipedia answer, as the 25 `(attribute, house)` cells — vocabulary
/// independent, which is the whole point of checking two encodings against it.
const GRID: [(&str, [&str; 5]); 5] = [
    ("House-1", ["Yellow", "Norwegian", "Kools", "Water", "Fox"]),
    (
        "House-2",
        ["Blue", "Ukrainian", "Chesterfields", "Tea", "Horse"],
    ),
    (
        "House-3",
        ["Red", "Englishman", "Old_Gold", "Milk", "Snail"],
    ),
    (
        "House-4",
        ["Ivory", "Spaniard", "Lucky_Strike", "Juice", "Dog"],
    ),
    (
        "House-5",
        ["Green", "Japanese", "Parliament", "Coffee", "Zebra"],
    ),
];

/// `co-located` is symmetric, so either argument order counts.
fn co_located(kb: &Kb, terms: &Terms, a: &str, b: &str) -> bool {
    let rel = terms.syms.get("co-located").expect("declared");
    let (Some(a), Some(b)) = (terms.syms.get(a), terms.syms.get(b)) else {
        return false;
    };
    [[a, b], [b, a]].iter().any(|pair| {
        terms
            .probe_fact(rel, &pair.map(Value::sym))
            .is_some_and(|f| kb.contains(f))
    })
}

/// `zebra.ein` — one generic `co-located` equivalence — reaches the published
/// grid, and the search that found it was exhausted, so `k = 1` is
/// *uniqueness* and not merely "a model".
#[test]
fn the_generic_link_encoding_is_the_unique_solution() {
    let (solved, terms) = run("examples/zebra.ein", None);
    assert_eq!(solved.stats.solution_nodes, 1, "k must be 1");
    assert!(
        solved.stats.exhausted,
        "uniqueness requires an exhausted search"
    );
    let kb = model(&solved);
    let missing: Vec<String> = GRID
        .iter()
        .flat_map(|(house, values)| values.iter().map(move |v| (*house, *v)))
        .filter(|(house, value)| !co_located(kb, &terms, value, house))
        .map(|(house, value)| format!("{value} in {house}"))
        .collect();
    assert!(missing.is_empty(), "grid cells unplaced: {missing:?}");
}

/// `zebra2.ein` — five typed projections — reaches the same 25 cells, read
/// through *its* vocabulary. The two encodings differ ontologically, not
/// semantically, and that is the only reason the second one is kept.
#[test]
fn both_ontologies_reach_the_same_model() {
    let (solved, terms) = run("examples/zebra2.ein", Some(1));
    let kb = model(&solved);
    let mut cells: Vec<(String, String)> = Vec::new();
    for rel in [
        "color-loc",
        "nation-loc",
        "drink-loc",
        "smoke-loc",
        "pet-loc",
    ] {
        let Some(rel) = terms.syms.get(rel) else {
            continue;
        };
        for f in kb.facts_of(rel) {
            let args = terms.facts.args(f);
            if args.len() == 2 {
                cells.push((terms.display(args[0]), terms.display(args[1])));
            }
        }
    }
    cells.sort();
    cells.dedup();
    assert_eq!(cells.len(), 25, "zebra2's model must fill all 25 cells");
    let mut want: Vec<(String, String)> = GRID
        .iter()
        .flat_map(|(house, values)| {
            values
                .iter()
                .map(move |v| (v.to_string(), house.to_string()))
        })
        .collect();
    want.sort();
    assert_eq!(cells, want, "the two ontologies disagree");
}

/// One engine, three answers, selected by the **input** and never by which
/// function was called. The regression guard for the three separate entries
/// that used to disagree on the same puzzle.
#[test]
fn the_three_readings_agree_with_the_input() {
    // k = 1 — a unique model, and no core.
    let (solved, _) = run("examples/zebra2.ein", None);
    assert!(matches!(
        solved.answer,
        Answer::Verdict(Verdict::Solution(_))
    ));
    assert_eq!(solved.stats.solution_nodes, 1);

    // k ≥ 2 — under-determined, so the gaps view has that many models.
    let (solved, _) = run("examples/zebra2-minus-15.ein", Some(2));
    assert!(
        matches!(solved.answer, Answer::Verdict(Verdict::Ambiguity(ref b)) if b.len() >= 2),
        "15 clues short of unique must read as Ambiguity, got {}",
        solved.answer.as_str()
    );

    // k = 0 — unsat, and the core names the culprit rather than the puzzle.
    let (solved, terms) = run("examples/ein-bugs/zebra2-bad.ein", None);
    let Answer::Verdict(Verdict::Contradiction { unsat_core }) = &solved.answer else {
        panic!("an injected clash must read as Contradiction");
    };
    assert_eq!(solved.stats.solution_nodes, 0);
    let core: Vec<String> = unsat_core
        .iter()
        .map(|&f| ein_infer::events::sexpr(&terms, f))
        .collect();
    assert!(
        core.iter().any(|c| c == "(color-loc Green House-1)"),
        "the core must name the injected fact, got {core:?}"
    );
}
