//! When not to believe a file — T1a.8.1.5, [design/10
//! §4](../../../../docs/history/m1a_rust/design/10_binary_format.md#4-versioning-and-invalidation).
//!
//! The rule the tests here pin is that **a stale cache can never change a
//! verdict**. Two ways that could happen and neither is allowed: believing
//! derived state an older engine produced, and believing a KB built from text
//! that has since been edited. The answer to the first is to fall back to the
//! program, re-loaded; the answer to the second is to *say so*, because only
//! the caller knows whether it still has the `.ein` to go back to.

use ein_core::Terms;
use ein_corpus::repo_root;
use ein_einb::{Freshness, KbState, Meta, OpenOptions, SaveOptions, Source, save_to_vec};
use ein_ir::{Ast, parse};

struct Fixture {
    bytes: Vec<u8>,
    loaded_facts: usize,
    saturated_facts: usize,
}

fn saturated_zebra() -> Fixture {
    let path = repo_root().join("examples/zebra.ein");
    let text = std::fs::read_to_string(&path).expect("zebra");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("parses");
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("loads");
    let loaded_facts = kb.n_facts();
    ein_infer::saturate_events(&ast, &mut terms, &mut kb).expect("saturates");
    let saturated_facts = kb.n_facts();
    assert!(saturated_facts > loaded_facts, "saturation derived nothing");
    Fixture {
        bytes: save_to_vec(
            &kb,
            &terms,
            &mut ast,
            &forms,
            path.parent(),
            &SaveOptions {
                state: KbState::Saturated,
                sources: vec![Source::of(&path).expect("hashable")],
                ..SaveOptions::default()
            },
        )
        .expect("saves"),
        loaded_facts,
        saturated_facts,
    }
}

#[test]
fn a_fresh_file_is_believed_whole() {
    let f = saturated_zebra();
    let mut terms = Terms::new();
    let path = repo_root().join("examples/zebra.ein");
    let opened = ein_einb::open_bytes(
        &f.bytes,
        &mut terms,
        &OpenOptions {
            sources: vec![Source::of(&path).expect("hashable")],
            ..OpenOptions::default()
        },
    )
    .expect("opens");
    assert_eq!(opened.freshness, Freshness::Fresh);
    assert!(!opened.derived_dropped);
    assert_eq!(opened.kb.n_facts(), f.saturated_facts);
}

/// The one with teeth: another engine's saturated fact set is not believed,
/// and what comes back instead is the program re-loaded — which is exactly
/// what reading the `.ein` would have produced.
#[test]
fn another_engine_keeps_the_program_and_drops_the_derived_sections() {
    let f = saturated_zebra();
    let mut terms = Terms::new();
    let opened = ein_einb::open_bytes(
        &f.bytes,
        &mut terms,
        &OpenOptions {
            engine: "0.0.0-not-this-one".to_string(),
            ..OpenOptions::default()
        },
    )
    .expect("opens");
    assert_eq!(opened.freshness, Freshness::OtherEngine);
    assert!(opened.derived_dropped);
    assert_eq!(
        opened.kb.n_facts(),
        f.loaded_facts,
        "the derived facts should be gone and the loaded ones should not"
    );
    assert!(
        !opened.kb.program().rules.is_empty(),
        "PROGRAM is what survives a version mismatch"
    );
    assert!(opened.solutions.is_none());
}

/// A source that has been edited since: reported, not refused, and not
/// silently believed either — the caller has the `.ein` and this is the
/// signal to go back to it.
#[test]
fn an_edited_source_is_a_cache_miss_and_says_so() {
    let f = saturated_zebra();
    let mut terms = Terms::new();
    let opened = ein_einb::open_bytes(
        &f.bytes,
        &mut terms,
        &OpenOptions {
            sources: vec![Source {
                path: repo_root().join("examples/zebra.ein").display().to_string(),
                digest: [7; 32],
            }],
            ..OpenOptions::default()
        },
    )
    .expect("opens");
    assert_eq!(opened.freshness, Freshness::StaleSource);
    assert!(
        opened.derived_dropped,
        "a saturated KB of text that has changed is doubly wrong"
    );
    assert_eq!(opened.kb.n_facts(), f.loaded_facts);
}

/// The stdlib is the input whose divergence nothing else would notice: the
/// puzzle text is unchanged, and `std.*` decides what it *means*.
#[test]
fn a_stdlib_that_moved_under_the_file_is_a_cache_miss() {
    let meta = Meta {
        engine: ein_einb::engine_version().to_string(),
        writer: "test".to_string(),
        created_unix: 0,
        state: KbState::Saturated,
        config: None,
        sources: Vec::new(),
        stdlib: ein_einb::stdlib_digest(),
    };
    assert_eq!(
        meta.freshness(ein_einb::engine_version(), &[], &ein_einb::stdlib_digest()),
        Freshness::Fresh
    );
    // What editing a checked-out copy of `stdlib/` produces: the manifest's
    // bytes change, so its digest does. Hashing an edited manifest is the same
    // observation as pointing `$EIN_STDLIB` at an edited tree, without a
    // process-global environment mutation in the middle of a test binary —
    // `ein-cli/tests/einb_cli.rs` does it the other way, with a child process.
    let manifest = ein_ir::stdlib::resolve_default()
        .read(ein_ir::stdlib::MARKER)
        .expect("a stdlib manifest");
    let edited = *blake3::hash(format!("{manifest}\n; edited\n").as_bytes()).as_bytes();
    assert_ne!(edited, ein_einb::stdlib_digest());
    assert_eq!(
        meta.freshness(ein_einb::engine_version(), &[], &edited),
        Freshness::StaleStdlib
    );
}

/// A caller that did not re-hash anything is not told the file is stale.
#[test]
fn not_looking_at_the_sources_is_not_evidence_that_they_moved() {
    let f = saturated_zebra();
    let mut terms = Terms::new();
    let opened =
        ein_einb::open_bytes(&f.bytes, &mut terms, &OpenOptions::default()).expect("opens");
    assert_eq!(opened.freshness, Freshness::Fresh);
    assert_eq!(opened.kb.n_facts(), f.saturated_facts);
}
