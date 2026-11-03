//! P1a.4's acceptance gate — the three zebra fixtures, solved and checked
//! against the published answer.
//!
//! These prove the answer is the **right** one. That distinction matters at
//! exactly one point: a self-golden can pin a wrong model, and the corpus
//! digest would be green. So these assert the Zebra puzzle's published answer,
//! read through each encoding's own vocabulary.
//!
//! Each is a few dozen saturations on the routes, so they run with the
//! ordinary suite.
//!
//! **T1a.10.2.3 — this is the whole acceptance gate.** It covers four
//! questions:
//!
//! | what it asserts | here |
//! |---|---|
//! | the k = 1 / k ≥ 2 / k = 0 matrix | `the_three_readings_agree_with_the_input`, extended below with **exhaustion**, **distinctness** and the injected fact's **provenance** |
//! | the listing and the contradictions view are *readings of one result*, not second opinions | `the_listing_is_the_verdict_counted` / `the_contradictions_view_is_the_verdict_explained` |
//! | two encodings, one model | `the_generic_link_encoding_is_the_unique_solution`, `both_ontologies_reach_the_same_model` |
//! | the CLI prints the answer, `k`, `exhausted` | the CLI's surface, not the engine's — `ein-cli/tests/` |
//!
//! The history worth keeping is the second row's: on 2026-06-16
//! three *separate* entry points disagreed about the same input — `ein search`
//! said `Solution` while `ein lattice --gaps` fabricated a second model and
//! `ein lattice --contradictions` produced an 81-fact "core" for a satisfiable
//! puzzle — because the verdict tracked **the function called** rather than the
//! input. The buggy entries were deleted and one entry point kept. These are
//! the guard on that, and they are the reason the repo has something that is
//! not a self-golden: they assert the *answer*.
//!
//! **Since M1e2 S1e2.1c.5 (2026-11-03) they ask the routes**
//! ([`ein_infer::explore`]): the walk they were written against was deleted
//! on the user's ruling of that day. *Unique* is `--single`'s `Solution`,
//! which is decided only when R is exactly one state; a model is that state's
//! fixpoint, read through the node function every route uses
//! ([`ein_infer::region::Nodes`]); and the unsat case is an empty R whose root
//! is dead, with root's core. The three readings are still one engine's.

use ein_core::{FactId, Kb, Terms, Value};
use ein_infer::described::Described;
use ein_infer::explore::{self, Answer, Ask, Mode, Verdict, Which};
use ein_infer::region::Nodes;
use ein_ir::{Ast, load_file};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

fn load(rel: &str) -> (Ast, Terms, Kb) {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let kb = load_file(&mut ast, &mut terms, &repo_root().join(rel)).expect("loads");
    (ast, terms, kb)
}

/// What a question about one file answered, and the first state it lists,
/// read whole: `read` gets that state's fixpoint.
fn ask<R>(
    rel: &str,
    which: Which,
    q: Ask,
    read: impl FnOnce(&Answer, Option<(&Kb, &Terms)>) -> R,
) -> R {
    let (ast, mut terms, mut kb) = load(rel);
    let a = explore::run(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(Mode { which, ask: q }),
    )
    .expect("solves");
    let Some(first) = a.states.first() else {
        return read(&a, None);
    };
    let facts: Vec<FactId> = first
        .set
        .iter()
        .map(|&b| a.candidates[b as usize].fact)
        .collect();
    let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).expect("prepares");
    let mut read = Some(read);
    nodes
        .with_fixpoint(&facts, |kb, terms| {
            (read.take().expect("once"))(&a, Some((kb, terms)))
        })
        .expect("saturates")
}

/// `ein solve FILE --single`'s word.
fn single(rel: &str) -> Verdict {
    ask(rel, Which::All, Ask::Single, |a, _| {
        a.verdict.expect("--single decides on these fixtures")
    })
}

/// `ein solve FILE`: R described.
fn describe(rel: &str) -> Described {
    let (ast, mut terms, mut kb) = load(rel);
    let a = explore::run(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Describe,
        }),
    )
    .expect("solves");
    *a.described.expect("describe describes")
}

/// The three fixtures and the word each one is supposed to land on — the
/// matrix, as data. `--single` decides each: one state, two, or none.
const CLASSES: [(&str, Verdict); 3] = [
    ("examples/zebra2.ein", Verdict::Solution),
    ("examples/zebra2-minus-15.ein", Verdict::Ambiguity),
    ("examples/ein-bugs/zebra2-bad.ein", Verdict::Contradiction),
];

// The Wikipedia answer, as the 25 `(attribute, house)` cells — vocabulary
// independent, which is the whole point of checking two encodings against it.
// Lives in `ein-corpus` since M1e2 S1e2.1.4, because `ein solve --models grid`
// is held to the same 25 cells (`ein-cli/tests/models_grid.rs`): the solver
// and the renderer checked against one independent source.
use ein_corpus::ZEBRA_GRID as GRID;

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

/// The `(attribute, house)` cells zebra2's five typed projections hold.
fn typed_cells(kb: &Kb, terms: &Terms) -> Vec<(String, String)> {
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
    cells
}

/// `zebra.ein` — one generic `co-located` equivalence — reaches the published
/// grid, and `--single` decided that R is that one state, so it is
/// *uniqueness* and not merely "a model".
#[test]
fn the_generic_link_encoding_is_the_unique_solution() {
    let missing = ask("examples/zebra.ein", Which::All, Ask::Single, |a, state| {
        assert_eq!(a.verdict, Some(Verdict::Solution), "R must be one state");
        assert!(a.exhausted, "uniqueness is a decided --single");
        let (kb, terms) = state.expect("the one state");
        GRID.iter()
            .flat_map(|(house, values)| values.iter().map(move |v| (*house, *v)))
            .filter(|(house, value)| !co_located(kb, terms, value, house))
            .map(|(house, value)| format!("{value} in {house}"))
            .collect::<Vec<String>>()
    });
    assert!(missing.is_empty(), "grid cells unplaced: {missing:?}");
}

/// `zebra2.ein` — five typed projections — reaches the same 25 cells, read
/// through *its* vocabulary. The two encodings differ ontologically, not
/// semantically, and that is the only reason the second one is kept.
#[test]
fn both_ontologies_reach_the_same_model() {
    let cells = ask(
        "examples/zebra2.ein",
        Which::All,
        Ask::Find(1),
        |_, state| {
            let (kb, terms) = state.expect("a state");
            typed_cells(kb, terms)
        },
    );
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
    for (rel, word) in CLASSES {
        assert_eq!(single(rel), word, "{rel} must read as {}", word.word());
    }
    // k = 0 — unsat, and the core names the culprit rather than the puzzle.
    let d = describe("examples/ein-bugs/zebra2-bad.ein");
    assert_eq!(d.k_s(), 0);
    assert!(
        d.root.core.iter().any(|c| c == "(color-loc Green House-1)"),
        "the core must name the injected fact, got {:?}",
        d.root.core
    );
}

// ── T1a.10.2.3 — the rest of the acceptance gate ──────────────────

/// **Uniqueness is a claim about the *search*, not only about `k`.**
///
/// `both_ontologies_reach_the_same_model` above asks `-n 1`, which finds *a*
/// state and says nothing about whether there is another. This asks
/// `--single`, whose `Solution` is decided only when R is exactly one state,
/// and that is the assertion that turns "one model was found" into "one model
/// exists".
#[test]
fn the_typed_encoding_is_unique_and_the_search_that_says_so_was_exhausted() {
    let held = ask(
        "examples/zebra2.ein",
        Which::All,
        Ask::Single,
        |a, state| {
            assert_eq!(a.verdict, Some(Verdict::Solution), "R must be one state");
            assert!(
                a.exhausted,
                "uniqueness requires a decided --single, not a satisfied -n"
            );
            let (kb, terms) = state.expect("the one state");
            // The four cells the puzzle is actually asked about, named rather than
            // counted: a model with 25 cells in the wrong places would pass a count.
            [
                ("drink-loc", "Water", "House-1"),
                ("pet-loc", "Zebra", "House-5"),
                ("nation-loc", "Norwegian", "House-1"),
                ("nation-loc", "Japanese", "House-5"),
            ]
            .iter()
            .map(|&(rel, a, b)| (rel, a, b, holds(kb, terms, rel, a, b)))
            .collect::<Vec<_>>()
        },
    );
    for (rel, a, b, ok) in held {
        assert!(ok, "the published answer has ({rel} {a} {b})");
    }
}

/// **The listing is the verdict counted, not a second opinion.**
///
/// `--single` lists the states that decided it: one for `Solution`, two for
/// `Ambiguity`, none for `Contradiction`. The contract is that they are
/// *distinct* states, by `state_key`, because two records of the same model
/// reached by different commitments are one model and counting them twice is
/// precisely the 2026-06-16 bug.
#[test]
fn the_listing_is_the_verdict_counted() {
    for (rel, word) in CLASSES {
        let (ast, mut terms, mut kb) = load(rel);
        let a = explore::run(
            &mut kb,
            &mut terms,
            &ast,
            &explore::Options::new(Mode {
                which: Which::All,
                ask: Ask::Single,
            }),
        )
        .expect("solves");
        assert_eq!(a.verdict, Some(word), "{rel} must read as {}", word.word());
        let sets: Vec<Vec<FactId>> = a
            .states
            .iter()
            .map(|s| {
                s.set
                    .iter()
                    .map(|&b| a.candidates[b as usize].fact)
                    .collect()
            })
            .collect();
        let mut nodes = Nodes::new(&mut kb, &mut terms, &ast, &a.candidates).expect("prepares");
        let mut keys: Vec<Vec<u32>> = nodes
            .with_fixpoints(&sets, |kbs, _| {
                kbs.iter()
                    .map(|k| ein_infer::state_key(k).iter().map(|f| f.0).collect())
                    .collect()
            })
            .expect("saturates");
        keys.sort();
        let before = keys.len();
        keys.dedup();
        let want = match word {
            Verdict::Solution => 1,
            Verdict::Ambiguity => 2,
            _ => 0,
        };
        assert_eq!(
            (keys.len(), before),
            (want, want),
            "{rel}: --single listed {before} states, {} distinct, where {} decides {want}",
            keys.len(),
            word.word()
        );
    }
}

/// **The contradictions view is the verdict explained, not a second opinion.**
///
/// An unsat core is reported for a `Contradiction` and for nothing else — the
/// 81-fact "core" the removed `contradictions_solve` produced for a
/// *satisfiable* puzzle is what this forbids — and when there is one it names
/// the culprit rather than the puzzle. A core is root's, and only a dead root
/// has one, so on the two satisfiable fixtures root must be alive.
#[test]
fn the_contradictions_view_is_the_verdict_explained() {
    for (rel, word) in CLASSES {
        let root_dead = ask(rel, Which::All, Ask::Find(1), |a, _| a.root_dead);
        assert_eq!(
            root_dead,
            word == Verdict::Contradiction,
            "{rel}: root is dead exactly for a Contradiction"
        );
    }
    let rel = "examples/ein-bugs/zebra2-bad.ein";
    let d = describe(rel);
    assert!(
        d.exhausted,
        "{rel}: UNSAT requires R to be known, not a run stopped"
    );
    assert!(!d.root.alive && d.k_s() == 0 && !d.any_alive);
    assert!(
        d.root.core.iter().any(|c| c == "(color-loc Green House-1)"),
        "{rel}: the core must name the injected fact, got {:?}",
        d.root.core
    );
    // A refutation that names no rule is not one.
    assert!(
        !d.root.refuted_by.is_empty(),
        "{rel}: a contradiction must have been derived by some rule"
    );
}

/// **The injected clash carries the provenance that names it.**
///
/// `examples/ein-bugs/zebra2-bad.ein` is `zebra2` plus one line —
/// `(color-loc Green House-1 :source "injected contradiction")` — and the
/// point of the fixture is that the engine blames *that line*. A core naming
/// the right fact with the wrong provenance would still be useless to a
/// reader, which is why this asserts the `:source` and not only the fact.
#[test]
fn the_injected_clash_carries_the_source_that_names_it() {
    let d = describe("examples/ein-bugs/zebra2-bad.ein");
    assert!(
        d.root.core.iter().any(|c| c == "(color-loc Green House-1)"),
        "the core names the injected fact"
    );
    assert!(
        d.root.sources.iter().any(
            |s| s.contains("(color-loc Green House-1)") && s.contains("injected contradiction")
        ),
        "the injected fact must carry its `:source` in the report, got {:?}",
        d.root.sources
    );
    // Provenance for an *ingested* fact is written once, at load, into root,
    // so the loaded KB is where to ask, and asking it separately also proves
    // the fact is a stated one rather than something the search derived.
    let (_, root_terms, root) = load("examples/ein-bugs/zebra2-bad.ein");
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
}

/// **The hard soundness invariant: the classification never inverts.**
///
/// A satisfiable puzzle is never a `Contradiction` and an unsatisfiable one is
/// never a `Solution`. Weaker than the two tests above and deliberately kept
/// separate: it is the one that would still fail if the listing and the core
/// were dropped tomorrow, and it is the shape of the 2026-06-16 bug.
#[test]
fn a_satisfiable_puzzle_is_never_unsat_and_an_unsat_one_is_never_solved() {
    let (word, cells) = ask(
        "examples/zebra2.ein",
        Which::All,
        Ask::Single,
        |a, state| {
            let (kb, terms) = state.expect("a state");
            (a.verdict, typed_cells(kb, terms).len())
        },
    );
    assert_ne!(
        word,
        Some(Verdict::Contradiction),
        "zebra2 is satisfiable and must not read as a contradiction"
    );
    assert_eq!(word, Some(Verdict::Solution));
    assert_eq!(cells, 25, "a solution must fill all 25 grid cells");

    let unsat = single("examples/ein-bugs/zebra2-bad.ein");
    assert_eq!(
        unsat,
        Verdict::Contradiction,
        "an injected clash is unsat and must not read as a solution"
    );
}

// ── helpers ────────────────────────────────────────────────────────

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
