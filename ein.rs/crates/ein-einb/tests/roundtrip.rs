//! The round trip — [design/10
//! §6](../../../../docs/history/m1a_rust/design/10_binary_format.md#6-acceptance-for-this-design),
//! P1a.8's acceptance.
//!
//! `save(kb) → open()` has to be **T1-identical**: the same facts in the same
//! order, the same provenance, the same indexes, in both the empty-interner
//! and shared-interner cases. Two comparators, because the two cases can not
//! be asked the same question:
//!
//! - Into an **empty** interner every id comes back the one it was, so
//!   [`Kb::diff`] compares the KBs themselves — fact order, belief set,
//!   negated set, all seven indexes, the primary map and the alternative
//!   lists, and the no-good set.
//! - Into a **shared** one the ids necessarily move — that is the whole point
//!   of the remap — so the comparator is [`ein_core::shape`], which names
//!   facts by their position in the fact list and everything else by text.
//!   Byte equality of two shapes over different id spaces is exactly "the same
//!   KB, renumbered".

use std::path::Path;

use ein_core::{Kb, Terms, shape};
use ein_corpus::{corpus_files, repo_root};
use ein_einb::{KbState, OpenOptions, SaveOptions, save_to_vec};
use ein_ir::{Ast, parse};

/// Load a corpus file the way the CLI does. `None` for the third of the corpus
/// that is a load-negative fixture.
fn load(path: &Path) -> Option<(Ast, Terms, Kb, Vec<ein_ir::NodeId>)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((ast, terms, kb, forms))
}

/// A `Terms` that is **not** the identity for the file about to be opened:
/// other content, interned first, so every table the reader builds is a
/// permutation and the linear remap is the path taken.
fn crowded() -> Terms {
    let mut terms = Terms::new();
    for i in 0..37 {
        let rel = terms.intern_text(&format!("crowd-{i}")).expect("room");
        let arg = terms.intern_text(&format!("other-{i}")).expect("room");
        terms.intern_int(&format!("{}", 1_000 + i)).expect("room");
        terms
            .intern_fact(rel, &[ein_core::Value::sym(arg)])
            .expect("room");
    }
    terms
}

/// The first line two shapes disagree on. A whole `assert_eq!` of two 400-line
/// dumps is a wall of text with the answer somewhere in it.
fn same_shape(a: &str, b: &str) -> Result<(), String> {
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return Err(format!("shape line {i}:\n  was {x}\n  now {y}"));
        }
    }
    let (n, m) = (a.lines().count(), b.lines().count());
    if n != m {
        return Err(format!("shape line counts: {n} vs {m}"));
    }
    Ok(())
}

fn saved(kb: &Kb, terms: &Terms, ast: &mut Ast, forms: &[ein_ir::NodeId], path: &Path) -> Vec<u8> {
    save_to_vec(
        kb,
        terms,
        ast,
        forms,
        path.parent(),
        &SaveOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}: save: {e}", path.display()))
}

#[test]
fn every_corpus_entry_round_trips_into_an_empty_and_a_shared_interner() {
    let (mut checked, mut remapped) = (0usize, 0usize);
    for path in &corpus_files() {
        let Some((mut ast, terms, kb, forms)) = load(path) else {
            continue;
        };
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        let bytes = saved(&kb, &terms, &mut ast, &forms, path);

        let mut fresh = Terms::new();
        let opened = ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default())
            .unwrap_or_else(|e| panic!("{name}: open: {e}"));
        assert!(
            !opened.remapped,
            "{name}: an empty interner should have taken the identity fast path"
        );
        if let Err(why) = kb.diff(&opened.kb) {
            panic!("{name}: {why}");
        }
        if let Err(why) = same_shape(&shape(&kb, &terms), &shape(&opened.kb, &fresh)) {
            panic!("{name}: {why}");
        }

        let mut shared = crowded();
        let opened = ein_einb::open_bytes(&bytes, &mut shared, &OpenOptions::default())
            .unwrap_or_else(|e| panic!("{name}: open (shared): {e}"));
        // The crowd is interned *after* the kernel names, so a file whose
        // tables are the bare kernel prefix — `examples/syntax/config.ein` is
        // one: a `(config …)` block and nothing else — still re-interns to its
        // own ids and takes the fast path honestly.
        let bare = terms.syms.len() == Terms::new().syms.len()
            && terms.ints.is_empty()
            && terms.facts.is_empty();
        assert_eq!(
            opened.remapped,
            !bare,
            "{name}: remapped={} for a file whose tables are {}the kernel prefix",
            opened.remapped,
            if bare { "" } else { "not " }
        );
        remapped += usize::from(opened.remapped);
        if let Err(why) = same_shape(&shape(&kb, &terms), &shape(&opened.kb, &shared)) {
            panic!("{name} (remapped): {why}");
        }
        checked += 1;
    }
    // `load_parity`'s floor, which this inherits with the op: a sweep that
    // loaded nothing agrees about nothing.
    assert!(checked >= 60, "only {checked} corpus files loaded");
    // The remap is half of what is being tested; a sweep in which every file
    // took the fast path would have tested one half twice.
    assert!(
        remapped >= 60,
        "only {remapped} of {checked} files exercised the remap"
    );
    eprintln!("einb: {checked} corpus files round-tripped, {remapped} through the remap");
}

/// The saturated case — the one the format exists for, and the one with
/// derived provenance, alternative justifications and a `(not …)` index in it.
#[test]
fn a_saturated_kb_round_trips_with_its_derivations() {
    for rel in [
        "examples/zebra2.ein",
        "examples/zebra.ein",
        "examples/features/01_not_and_absent.ein",
        "examples/features/03_forall.ein",
        "examples/zebra2-hints.ein",
    ] {
        let path = repo_root().join(rel);
        if !path.is_file() {
            continue;
        }
        let Some((mut ast, mut terms, mut kb, forms)) = load(&path) else {
            panic!("{rel} does not load");
        };
        ein_infer::saturate_events(&ast, &mut terms, &mut kb)
            .unwrap_or_else(|e| panic!("{rel}: saturate: {e}"));
        let bytes = save_to_vec(
            &kb,
            &terms,
            &mut ast,
            &forms,
            path.parent(),
            &SaveOptions {
                state: KbState::Saturated,
                ..SaveOptions::default()
            },
        )
        .expect("save");

        let mut fresh = Terms::new();
        let opened =
            ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default()).expect("open");
        assert_eq!(opened.meta.state, KbState::Saturated);
        assert!(!opened.derived_dropped);
        if let Err(why) = kb.diff(&opened.kb) {
            panic!("{rel}: {why}");
        }
        if let Err(why) = same_shape(&shape(&kb, &terms), &shape(&opened.kb, &fresh)) {
            panic!("{rel}: {why}");
        }

        let mut shared = crowded();
        let opened =
            ein_einb::open_bytes(&bytes, &mut shared, &OpenOptions::default()).expect("open");
        if let Err(why) = same_shape(&shape(&kb, &terms), &shape(&opened.kb, &shared)) {
            panic!("{rel} (remapped): {why}");
        }
    }
}

/// design/10 §6's two numbers: a saturated `zebra2` under 64 KB, and a cold
/// open under a millisecond.
///
/// The open is timed as a *median* of eleven, because a single one on a shared
/// runner measures the runner. It is still the whole job — digest check,
/// re-intern, re-parse of `PROGRAM`, registry rebuild, fact replay, index
/// rebuild — and not a subset chosen to make the number.
#[test]
fn a_saturated_zebra2_is_small_and_opens_cold_in_under_a_millisecond() {
    let path = repo_root().join("examples/zebra2.ein");
    let (mut ast, mut terms, mut kb, forms) = load(&path).expect("zebra2 loads");
    ein_infer::saturate_events(&ast, &mut terms, &mut kb).expect("saturate");
    let bytes = save_to_vec(
        &kb,
        &terms,
        &mut ast,
        &forms,
        path.parent(),
        &SaveOptions {
            state: KbState::Saturated,
            ..SaveOptions::default()
        },
    )
    .expect("save");
    assert!(
        bytes.len() < 64 * 1024,
        "a saturated zebra2 is {} bytes, over design/10 §6's 64 KB",
        bytes.len()
    );

    let breakdown: Vec<String> = ein_einb::section_sizes(&bytes)
        .expect("a readable section table")
        .iter()
        .map(|(k, n)| format!("{k:?}={n}"))
        .collect();

    let mut times: Vec<f64> = Vec::new();
    for _ in 0..11 {
        let mut fresh = Terms::new();
        let t = std::time::Instant::now();
        let opened =
            ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default()).expect("open");
        times.push(t.elapsed().as_secs_f64() * 1000.0);
        std::hint::black_box(opened.kb.n_facts());
    }
    times.sort_by(f64::total_cmp);
    let median = times[times.len() / 2];
    // design/10 §6's millisecond is about the **shipped** engine. `cargo test`
    // builds the dev profile — `opt-level = 1` with debug assertions on, which
    // is 0.83 ms here against release's 0.61 — and CI runs it on a shared
    // machine, so gating the design's number on an unoptimised build on a
    // borrowed core would be measuring the runner. The number is printed
    // either way, which is what a regression is read off; what differs is the
    // ceiling that fails the test.
    let budget = if cfg!(debug_assertions) { 5.0 } else { 1.0 };
    eprintln!(
        "einb: saturated zebra2 = {} bytes ({}), {} facts, cold open {median:.3} ms",
        bytes.len(),
        breakdown.join(" "),
        kb.n_facts()
    );
    // The message names the budget, the profile and the suspect — M1e
    // S1e.4.5, `TE-L1`, which found this site while auditing two others and
    // did not list it. It is the workspace's second-tightest wall clock
    // (measured 0.96 ms against the 5.0 ms dev budget, 2026-09-01, so 5.2×)
    // and its failure said only what it took.
    assert!(
        median < budget,
        "cold open took {median:.3} ms against a {budget} ms {} budget \
         — a real regression, or machine load?",
        if cfg!(debug_assertions) {
            "dev"
        } else {
            "release"
        }
    );
}

/// A solved KB, its models stored as deltas, and a model put back together.
#[test]
fn a_stored_solution_reconstitutes_the_model_it_was_taken_from() {
    use ein_infer::solve::{NoDumper, SolveOptions, solve};

    let path = repo_root().join("examples/zebra.ein");
    let (mut ast, mut terms, mut kb, forms) = load(&path).expect("zebra loads");
    let mut events = ein_infer::events::Events::off();
    let solved = solve(
        &mut kb,
        &mut terms,
        &ast,
        &mut events,
        &mut NoDumper,
        &SolveOptions::default(),
    )
    .expect("solve");
    let models: Vec<Box<[ein_core::FactId]>> = match &solved.answer {
        ein_infer::verdict::Answer::Verdict(ein_infer::verdict::Verdict::Solution(s)) => {
            vec![ein_infer::state_key(&s.kb)]
        }
        other => panic!("zebra should have one model, got {}", other.as_str()),
    };
    let stored = ein_einb::Solutions::of(&kb, &mut terms, &ast, &solved.answer, solved.stats);
    assert_eq!(stored.nodes.len(), 1);

    let bytes = save_to_vec(
        &kb,
        &terms,
        &mut ast,
        &forms,
        path.parent(),
        &SaveOptions {
            state: KbState::Solved,
            solutions: Some(stored.clone()),
            ..SaveOptions::default()
        },
    )
    .expect("save");

    let mut fresh = Terms::new();
    let mut opened =
        ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default()).expect("open");
    let back = opened.solutions.clone().expect("a SOLUTIONS section");
    assert_eq!(back, stored, "the stored solve moved");

    // base.fork() + the delta is the model again — same state key, which is
    // the identity the search itself dedups nodes by.
    let model = back.nodes[0].reconstitute(&mut opened.kb);
    assert_eq!(ein_infer::state_key(&model), models[0]);
    assert!(model.n_facts() > opened.kb.n_facts());
}

/// `ein solve` must not be able to *read* a stored answer — F9's hazard, and
/// the reason the mitigation is structural rather than a note.
#[test]
fn nothing_in_the_solve_path_reads_the_solution_store() {
    let cli = repo_root().join("ein.rs/crates/ein-cli/src");
    let mut offenders = Vec::new();
    for entry in std::fs::read_dir(&cli).expect("the CLI source") {
        let path = entry.expect("a dir entry").path();
        if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).expect("readable");
            // The *read* — the field access — not the word: `ein kb save`
            // mentions `solutions:` when it declines to store any, and that is
            // the writer, which is not the hazard.
            //
            // It is a textual check and therefore a coarse one: any field
            // named `solutions` on any type trips it. That has cost one
            // rename already — M1d S1d.2.4's `OwesReport` calls its per-model
            // tallies `models` for this reason, and its own doc says so — and
            // a rename is the right price for a guard this cheap.
            if text.contains(".solutions") {
                offenders.push(path.display().to_string());
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "the CLI reads a stored solution store: {offenders:?}"
    );
}
