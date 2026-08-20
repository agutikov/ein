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
//!
//! **T1a.10.2.3 — this is now the whole acceptance gate.** `ein.py/acceptance/`
//! is 21 tests in four files and ~40 s under PyPy; what P1a.4 ported was the
//! three-class core, and this stage ports the rest. What arrived here:
//!
//! | Python | what it asserts | here |
//! |---|---|---|
//! | `test_zebra_three_classes.py` | the k = 1 / k ≥ 2 / k = 0 matrix | `the_three_readings_agree_with_the_input`, extended below with **exhaustion**, **distinctness** and the injected fact's **provenance** |
//! | `test_mode_consistency.py` | the gaps view and the contradictions view are *readings of one result*, not second opinions | `the_gaps_view_is_the_verdict_counted` / `the_contradictions_view_is_the_verdict_explained` |
//! | `test_zebra_two_ontologies.py` | two encodings, one model | `the_generic_link_encoding_is_the_unique_solution`, `both_ontologies_reach_the_same_model` |
//! | `test_bench_solve_mode.py` | the CLI prints the answer, `k`, `exhausted` | the CLI's surface, not the engine's — `ein-cli/tests/` |
//!
//! The history worth keeping is `test_mode_consistency.py`'s: on 2026-06-16
//! three *separate* entry points disagreed about the same input — `ein search`
//! said `Solution` while `ein lattice --gaps` fabricated a second model and
//! `ein lattice --contradictions` produced an 81-fact "core" for a satisfiable
//! puzzle — because the verdict tracked **the function called** rather than the
//! input. The buggy entries were deleted and one engine kept. These are the
//! guard on that, and they are the reason a single-implementation repo still
//! has something that is not a self-golden: they assert the *answer*.

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

/// The same, with the sound `LatticeProof` attached — what `--trace` and
/// `--dump-states` set, and what the gaps and contradictions views read.
fn run_with_proof(rel: &str, stop_after: Option<u64>) -> (Solved, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    let mut events = Events::off();
    let opts = SolveOptions {
        stop_after,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let solved =
        solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts).expect("solves");
    (solved, terms)
}

/// The three fixtures and the class each one is supposed to land in — the
/// matrix `test_mode_consistency.py` pins, as data.
///
/// `stop_after` exhausts the lattice for the unique and unsat cases, because
/// only an exhausted search *certifies* them; two distinct nodes already prove
/// ambiguity, so the middle one stops at 2.
const CLASSES: [(&str, &str, Option<u64>); 3] = [
    ("examples/zebra2.ein", "Solution", None),
    ("examples/zebra2-minus-15.ein", "Ambiguity", Some(2)),
    ("examples/ein-bugs/zebra2-bad.ein", "Contradiction", None),
];

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

// ── T1a.10.2.3 — the rest of `ein.py/acceptance/` ──────────────────

/// **Uniqueness is a claim about the *search*, not only about `k`.**
///
/// `both_ontologies_reach_the_same_model` above runs `zebra2` with
/// `stop_after = 1`, which finds *a* model and says nothing about whether
/// there is another. The Python gate ran the same puzzle exhaustively and
/// asserted `exhausted`, and that is the assertion that turns "one model was
/// found" into "one model exists". It costs 29 ms here and ~21 s there, which
/// is the whole reason it lived in a separate phase.
#[test]
fn the_typed_encoding_is_unique_and_the_search_that_says_so_was_exhausted() {
    let (solved, terms) = run("examples/zebra2.ein", None);
    assert_eq!(solved.stats.solution_nodes, 1, "k must be 1");
    assert!(
        solved.stats.exhausted,
        "uniqueness requires an exhausted search, not a satisfied `stop_after`"
    );
    // The four cells the puzzle is actually asked about, named rather than
    // counted: a model with 25 cells in the wrong places would pass a count.
    let kb = model(&solved);
    for (rel, a, b) in [
        ("drink-loc", "Water", "House-1"),
        ("pet-loc", "Zebra", "House-5"),
        ("nation-loc", "Norwegian", "House-1"),
        ("nation-loc", "Japanese", "House-5"),
    ] {
        assert!(
            holds(kb, &terms, rel, a, b),
            "the published answer has ({rel} {a} {b})"
        );
    }
}

/// **The gaps view is the verdict counted, not a second opinion.**
///
/// `LatticeProof::solutions` is what `ein lattice --gaps` reads. The contract
/// is that it holds exactly `k` *distinct* models — distinct by `state_key`,
/// because two records of the same model reached by different commitments are
/// one model and counting them twice is precisely the 2026-06-16 bug.
#[test]
fn the_gaps_view_is_the_verdict_counted() {
    for (rel, class, stop) in CLASSES {
        let (solved, _) = run_with_proof(rel, stop);
        assert_eq!(solved.answer.as_str(), class, "{rel} must read as {class}");
        let proof = solved
            .proof
            .as_ref()
            .expect("store_lattice attaches a proof");
        let mut keys: Vec<Vec<u32>> = proof
            .solutions
            .iter()
            .map(|s| {
                ein_infer::state_key(&s.kb)
                    .iter()
                    .map(|f| f.0)
                    .collect::<Vec<u32>>()
            })
            .collect();
        keys.sort();
        let before = keys.len();
        keys.dedup();
        assert_eq!(
            keys.len(),
            solved.stats.solution_nodes as usize,
            "{rel}: the gaps view has {} distinct models ({before} records) \
             where the verdict counted {}",
            keys.len(),
            solved.stats.solution_nodes
        );
        // …and for the ambiguous one, "distinct" has to mean something: two
        // records that dedup to one would satisfy the count above only by
        // making k wrong too, so assert the shape directly.
        if class == "Ambiguity" {
            assert!(
                keys.len() >= 2,
                "{rel}: an ambiguous puzzle must reach models that actually differ"
            );
        }
    }
}

/// **The contradictions view is the verdict explained, not a second opinion.**
///
/// An unsat core is reported for a `Contradiction` and for nothing else — the
/// 81-fact "core" the removed `contradictions_solve` produced for a
/// *satisfiable* puzzle is what this forbids — and when there is one it names
/// the culprit rather than the puzzle.
#[test]
fn the_contradictions_view_is_the_verdict_explained() {
    for (rel, class, stop) in CLASSES {
        let (solved, terms) = run_with_proof(rel, stop);
        let core: Vec<String> = match &solved.answer {
            Answer::Verdict(Verdict::Contradiction { unsat_core }) => unsat_core
                .iter()
                .map(|&f| ein_infer::events::sexpr(&terms, f))
                .collect(),
            _ => Vec::new(),
        };
        assert_eq!(
            !core.is_empty(),
            class == "Contradiction",
            "{rel}: a core is reported exactly for a Contradiction, got {} entries",
            core.len()
        );
        if class == "Contradiction" {
            assert!(
                core.iter().any(|c| c == "(color-loc Green House-1)"),
                "{rel}: the core must name the injected fact, got {core:?}"
            );
            assert!(
                solved.stats.exhausted,
                "{rel}: UNSAT requires the search to have run out, not stopped"
            );
            // A refutation that refutes nothing is not one.
            let proof = solved.proof.as_ref().expect("proof");
            assert!(
                !proof.dead_commitments.is_empty(),
                "{rel}: a contradiction must have killed at least one commitment"
            );
        }
    }
}

/// **The injected clash carries the provenance that names it.**
///
/// `examples/ein-bugs/zebra2-bad.ein` is `zebra2` plus one line —
/// `(color-loc Green House-1 :source "injected contradiction")` — and the
/// point of the fixture is that the engine blames *that line*. A core naming
/// the right fact with the wrong provenance would still be useless to a
/// reader, which is why the Python gate asserted the `:source` and not only
/// the fact.
#[test]
fn the_injected_clash_carries_the_source_that_names_it() {
    let (solved, terms) = run("examples/ein-bugs/zebra2-bad.ein", None);
    let Answer::Verdict(Verdict::Contradiction { unsat_core }) = &solved.answer else {
        panic!("an injected clash must read as Contradiction");
    };
    let culprit = unsat_core
        .iter()
        .find(|&&f| ein_infer::events::sexpr(&terms, f) == "(color-loc Green House-1)")
        .copied()
        .expect("the core names the injected fact");
    // Provenance for an *ingested* fact is written once, at load, into root —
    // a fork inherits the record rather than rewriting it — so the root KB is
    // where to ask, and asking it separately also proves the fact is a stated
    // one rather than something the search derived.
    let (root, root_terms) = just_load("examples/ein-bugs/zebra2-bad.ein");
    let same = root_terms
        .syms
        .get("color-loc")
        .and_then(|rel| {
            let (g, h) = (
                root_terms.syms.get("Green")?,
                root_terms.syms.get("House-1")?,
            );
            root_terms.probe_fact(rel, &[Value::sym(g), Value::sym(h)])
        })
        .expect("the injected fact exists in root");
    assert!(
        root.contains(same),
        "the injected fact is stated, not derived"
    );
    let prov = root_terms
        .provs
        .get(root.primary(same).expect("a stated fact has a record"));
    assert_eq!(
        prov.source
            .map(|s| root_terms.sym(s).to_string())
            .as_deref(),
        Some("injected contradiction"),
        "the injected fact must carry its `:source`"
    );
    // …and the culprit in the core is that same fact, not a homonym.
    assert_eq!(
        ein_infer::events::sexpr(&terms, culprit),
        "(color-loc Green House-1)"
    );
}

/// **The hard soundness invariant: the classification never inverts.**
///
/// A satisfiable puzzle is never a `Contradiction` and an unsatisfiable one is
/// never a `Solution`. Weaker than the two tests above and deliberately kept
/// separate: it is the one that would still fail if the gaps and contradictions
/// views were deleted tomorrow, and it is the shape of the 2026-06-16 bug.
#[test]
fn a_satisfiable_puzzle_is_never_unsat_and_an_unsat_one_is_never_solved() {
    let (sat, terms) = run("examples/zebra2.ein", None);
    assert!(
        !matches!(sat.answer, Answer::Verdict(Verdict::Contradiction { .. })),
        "zebra2 is satisfiable and must not read as a contradiction"
    );
    assert!(matches!(sat.answer, Answer::Verdict(Verdict::Solution(_))));
    let cells = typed_cells(model(&sat), &terms);
    assert_eq!(cells, 25, "a solution must fill all 25 grid cells");

    let (unsat, _) = run("examples/ein-bugs/zebra2-bad.ein", None);
    assert!(
        matches!(unsat.answer, Answer::Verdict(Verdict::Contradiction { .. })),
        "an injected clash is unsat and must not read as a solution"
    );
    assert_eq!(unsat.stats.solution_nodes, 0);
}

// ── helpers for the four tests above ───────────────────────────────

/// Is `(rel a b)` believed? By name, so a test reads as the puzzle does.
fn holds(kb: &Kb, terms: &Terms, rel: &str, a: &str, b: &str) -> bool {
    let (Some(rel), Some(a), Some(b)) = (terms.syms.get(rel), terms.syms.get(a), terms.syms.get(b))
    else {
        return false;
    };
    terms
        .probe_fact(rel, &[Value::sym(a), Value::sym(b)])
        .is_some_and(|f| kb.contains(f))
}

/// How many `(attribute, house)` cells the five typed projections hold.
fn typed_cells(kb: &Kb, terms: &Terms) -> usize {
    [
        "color-loc",
        "nation-loc",
        "drink-loc",
        "smoke-loc",
        "pet-loc",
    ]
    .iter()
    .filter_map(|r| terms.syms.get(r))
    .map(|rel| kb.facts_of(rel).count())
    .sum()
}

/// Load without solving — the root KB, as the loader left it.
fn just_load(rel: &str) -> (Kb, Terms) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    (kb, terms)
}
