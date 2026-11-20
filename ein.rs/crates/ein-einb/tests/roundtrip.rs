//! The round trip — [03-design/09-binary-format.md § 7](../../../../docs/ein/ein.rs/03-design/09-binary-format.md#7-tests).
//!
//! `save(kb) → open()` has to be **identical**: the same facts in the same
//! order, the same provenance, the same indexes, in both the empty-interner
//! and shared-interner cases. Two comparators, because the two cases can not
//! be asked the same question:
//!
//! - Into an **empty** interner every id comes back the one it was, so
//!   [`Kb::diff`] compares the KBs themselves — fact order, belief set,
//!   all six indexes, the primary map and the alternative
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
        terms.intern_text(&format!("{}", 1_000 + i)).expect("room");
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
    // On the corpus sweeps' pool: a file a unit, its
    // `Some(remapped)` or `None` where it does not load, read back in order.
    let files = corpus_files();
    let done = ein_corpus::pool::map(
        "einb::roundtrip",
        &files,
        |p| {
            (
                p.strip_prefix(repo_root())
                    .unwrap_or(p)
                    .display()
                    .to_string(),
                String::new(),
            )
        },
        |path| {
            let (mut ast, terms, kb, forms) = load(path)?;
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
            let bare = terms.syms.len() == Terms::new().syms.len() && terms.facts.is_empty();
            assert_eq!(
                opened.remapped,
                !bare,
                "{name}: remapped={} for a file whose tables are {}the kernel prefix",
                opened.remapped,
                if bare { "" } else { "not " }
            );
            if let Err(why) = same_shape(&shape(&kb, &terms), &shape(&opened.kb, &shared)) {
                panic!("{name} (remapped): {why}");
            }
            Some(opened.remapped)
        },
    );
    for r in done.into_iter().flatten() {
        remapped += usize::from(r);
        checked += 1;
    }
    // The floor the golden's sweep (`id_order_invariance`) holds `load` to,
    // over every corpus program in the nightly: a sweep that loaded nothing
    // agrees about nothing.
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
/// `alt-cap-core.ein` is the one whose full list keeps an arrival beside it
/// (`CLOSURE`), which `Kb::diff` compares.
#[test]
fn a_saturated_kb_round_trips_with_its_derivations() {
    for rel in [
        "examples/zebra2.ein",
        "examples/zebra.ein",
        "examples/features/01_not_and_absent.ein",
        "examples/features/03_forall.ein",
        "examples/zebra2-hints.ein",
        "examples/ein-bugs/alt-cap-core.ein",
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
        if rel.ends_with("alt-cap-core.ein") {
            assert!(
                opened
                    .kb
                    .closures()
                    .iter()
                    .any(|(f, _, best)| best
                        .is_some_and(|b| !opened.kb.alternatives(*f).contains(&b))),
                "{rel}: no arrival kept beside a full list came back"
            );
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

/// A solved KB, its models stored as deltas, and a model put back together.
///
/// The model is R's one state, as `ein solve --single` decides it. No command
/// writes the section, so this is its one writer.
#[test]
fn a_stored_solution_reconstitutes_the_model_it_was_taken_from() {
    use ein_infer::explore::{self, Ask, Mode, Verdict, Which};

    let path = repo_root().join("examples/zebra.ein");
    let (mut ast, mut terms, mut kb, forms) = load(&path).expect("zebra loads");
    let a = explore::run(
        &mut kb,
        &mut terms,
        &ast,
        &explore::Options::new(Mode {
            which: Which::All,
            ask: Ask::Single,
        }),
    )
    .expect("solve");
    assert_eq!(a.verdict, Some(Verdict::Solution), "zebra has one model");
    let facts: Vec<ein_core::FactId> = a.states[0]
        .set
        .iter()
        .map(|&b| a.candidates[b as usize].fact)
        .collect();
    let base = kb.fork();
    let (stored, models) = {
        let mut nodes = ein_infer::region::Nodes::new(&mut kb, &mut terms, &ast, &a.candidates)
            .expect("prepares");
        nodes
            .with_fixpoints(&[facts], |kbs, terms| {
                let model = &kbs[0];
                (
                    ein_einb::Solutions::of(
                        &base,
                        terms,
                        &ast,
                        ein_einb::solutions::VerdictKind::Solution,
                        &[model],
                        Vec::new(),
                        ein_einb::MonotonicStats::default(),
                    ),
                    vec![ein_infer::state_key(model)],
                )
            })
            .expect("saturates")
    };
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

/// `ein solve` must not be able to *read* a stored answer — a stored answer
/// memoises the puzzle rather than improving the reasoner, and the
/// mitigation is structural rather than a note.
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
            // named `solutions` on any type trips it, so the solve path names
            // no field so, and a rename is the right price for a guard this
            // cheap.
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

/// **A node's container holds its commitment, and a root's holds none**
/// (Q-M1f.413). A saved file with a `(commit …)`, loaded as every command
/// loads one and saved, writes a `COMMIT` section of its choices, which an
/// open, into an empty interner and into a crowded one, sets as the stored
/// program's commitment, as the saved file's load sets it; saved again, it is
/// the same bytes. The program the node is of, saved, writes no `COMMIT`
/// section and opens with no commitment, and so does its saved root file.
#[test]
fn a_nodes_commitment_is_stored_and_a_roots_is_not() {
    let load_saved = |rel: &str| {
        let path = repo_root().join(rel);
        let text = std::fs::read_to_string(&path).expect("the corpus file");
        let mut ast = Ast::new();
        let mut terms = Terms::new();
        let loaded = ein_ir::saved::load_text(
            &mut ast,
            &mut terms,
            &text,
            path.to_str(),
            path.parent(),
            0,
            &ein_ir::LoadOptions::default(),
        )
        .unwrap_or_else(|e| panic!("{rel}: {e}"));
        let carried = ein_ir::grammar::opens_saved(&text).then(|| ein_einb::sections::Carried {
            file: path.display().to_string(),
            modules: loaded
                .kb
                .program()
                .sources
                .as_ref()
                .map(|s| s.modules.clone())
                .unwrap_or_default(),
        });
        save_to_vec(
            &loaded.kb,
            &terms,
            &mut ast,
            &loaded.forms,
            path.parent(),
            &SaveOptions {
                carried,
                ..SaveOptions::default()
            },
        )
        .expect("saves")
    };
    let kinds = |bytes: &[u8]| -> Vec<ein_einb::Kind> {
        ein_einb::section_sizes(bytes)
            .expect("a section table")
            .into_iter()
            .map(|(k, _)| k)
            .collect()
    };

    let node = load_saved("tests/saved/04_a_saved_node.ein");
    assert!(
        kinds(&node).contains(&ein_einb::Kind::Commit),
        "{:?}",
        kinds(&node)
    );
    for mut terms in [Terms::new(), crowded()] {
        let opened = ein_einb::open_bytes(&node, &mut terms, &OpenOptions::default())
            .expect("the node's container opens");
        assert_eq!(
            opened.kb.program().commit.as_deref(),
            Some(&["(note (bad a))".to_string()][..]),
            "the node's choices"
        );
    }
    // Saved again, from what it opened to: the same bytes (Q-M1f.110).
    let mut terms = Terms::new();
    let mut opened =
        ein_einb::open_bytes(&node, &mut terms, &OpenOptions::default()).expect("opens");
    let again = save_to_vec(
        &opened.kb,
        &terms,
        &mut opened.ast,
        &opened.forms,
        None,
        &SaveOptions::default(),
    )
    .expect("saves again");
    assert!(
        again == node,
        "a node's container saved again is other bytes"
    );

    let program = load_saved("tests/saved/02_a_choice_with_a_fact_inside_it.ein");
    assert!(
        !kinds(&program).contains(&ein_einb::Kind::Commit),
        "{:?}",
        kinds(&program)
    );
    let mut terms = Terms::new();
    let opened =
        ein_einb::open_bytes(&program, &mut terms, &OpenOptions::default()).expect("opens");
    assert_eq!(opened.kb.program().commit, None, "a root has no commitment");
}
