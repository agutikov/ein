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
use ein_einb::{
    EinbError, Freshness, KbState, Meta, OpenOptions, SaveOptions, Source, save_to_vec,
};
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

/// The sections of a file this writer wrote, in its order: each kind and its
/// bytes. The writer's own layout, read back through the public header and
/// section-table readers.
fn sections_of(bytes: &[u8]) -> Vec<(u32, Vec<u8>)> {
    let mut r = ein_einb::wire::Reader::new(bytes);
    let head = ein_einb::header::Header::read(&mut r).expect("a header");
    (0..head.n_sections)
        .map(|_| {
            let e = ein_einb::header::Entry::read(&mut r).expect("an entry");
            let (off, len) = (e.off as usize, e.len as usize);
            (e.kind, bytes[off..off + len].to_vec())
        })
        .collect()
}

/// A file of `sections`, laid out as `save_to_vec` lays one out — the header,
/// the section table, each body aligned — and digested, so that the reader's
/// default path, which checks the digest, is the one a refusal is on.
fn file_of(sections: &[(u32, Vec<u8>)]) -> Vec<u8> {
    use ein_einb::header::{ALIGN, ENTRY_LEN, Entry, HEADER_LEN, Header};
    use ein_einb::wire::Writer;
    let mut table = Writer::new();
    let mut body = Writer::new();
    let start = HEADER_LEN + sections.len() * ENTRY_LEN;
    for (kind, bytes) in sections {
        body.align(ALIGN);
        Entry {
            kind: *kind,
            flags: 0,
            off: (start + body.len()) as u64,
            len: bytes.len() as u64,
            raw_len: bytes.len() as u64,
        }
        .write(&mut table);
        body.bytes(bytes);
    }
    body.align(ALIGN);
    let mut after = table.into_vec();
    after.extend_from_slice(body.as_slice());
    let mut out = Writer::new();
    Header {
        major: ein_einb::FORMAT_MAJOR,
        minor: ein_einb::FORMAT_MINOR,
        flags: 0,
        n_sections: sections.len() as u32,
        digest: *blake3::hash(&after).as_bytes(),
    }
    .write(&mut out);
    out.bytes(&after);
    out.into_vec()
}

/// **A file that holds an integer is refused, never misread** — M1f S1f.5.4,
/// at format 2.0.
///
/// Ein has had no int since the kind was retired: a run of digits is a name
/// since S1f.5.5's swap. Through format 1.2 an empty `INTS` section (kind 3)
/// stayed in every file; 2.0 dropped it (Q-M1f.10 (b)), so what this writer
/// writes has no section of kind 3. What a writer before S1f.5.4 wrote from a
/// program with an int is refused on two grounds, each built here on a file
/// this writer wrote, its digest good: its header says 1.x, refused with the
/// version line; and a value tagged `0b01` in `FACTS`, refused with
/// [`EinbError::Int`]'s sentence, where reading it as a name or a fact would
/// have been a wrong answer. The file as written opens, and a name of digits in
/// it is a name.
#[test]
fn a_file_that_holds_an_integer_is_refused() {
    let text = "(relation p T :why \"p holds of {?1}\")\n(p 7)\n(p 07)\n";
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, text, Some("<digits>")).expect("parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, None).expect("loads");
    let bytes =
        save_to_vec(&kb, &terms, &mut ast, &forms, None, &SaveOptions::default()).expect("saves");

    let mut fresh = Terms::new();
    let opened = ein_einb::open_bytes(&bytes, &mut fresh, &OpenOptions::default())
        .expect("the file as written opens");
    let p = fresh.intern_text("p").expect("room");
    let mut stored: Vec<String> = opened.kb.facts_of(p).map(|f| fresh.fact_text(f)).collect();
    stored.sort();
    assert_eq!(stored, ["(p 07)", "(p 7)"], "`7` and `07` are two names");

    let sections = sections_of(&bytes);
    assert!(
        sections.iter().all(|(k, _)| *k != 3),
        "no INTS section (kind 3) since format 2.0"
    );

    // A 1.x file: this one, its header's version set to the last 1.x, 1.2.
    // The digest is over what follows the header, so it stays good.
    let mut old = bytes.clone();
    old[8..10].copy_from_slice(&1u16.to_le_bytes());
    old[10..12].copy_from_slice(&2u16.to_le_bytes());
    let mut terms = Terms::new();
    match ein_einb::open_bytes(&old, &mut terms, &OpenOptions::default()) {
        Err(e @ EinbError::Version { major: 1, minor: 2 }) => assert_eq!(
            e.to_string(),
            format!(
                ".einb format 1.2 — this engine reads {}.x",
                ein_einb::FORMAT_MAJOR
            )
        ),
        Err(e) => panic!("a 1.x file: refused for another reason: {e}"),
        Ok(_) => panic!("a 1.x file: opened"),
    }

    // A value tagged `0b01` in `FACTS`: the first argument of the first row,
    // its two tag bits an int's (`n_rows`, `n_args`, the 12-byte rows, then
    // the argument arena).
    let facts = ein_einb::Kind::Facts as u32;
    let with_tag: Vec<(u32, Vec<u8>)> = sections
        .iter()
        .map(|(k, b)| {
            let mut b = b.clone();
            if *k == facts {
                let n_rows = u32::from_le_bytes(b[0..4].try_into().expect("four bytes")) as usize;
                let at = 8 + 12 * n_rows;
                let word = u32::from_le_bytes(b[at..at + 4].try_into().expect("four bytes"));
                b[at..at + 4].copy_from_slice(&((word & !(3 << 30)) | 1 << 30).to_le_bytes());
            }
            (*k, b)
        })
        .collect();
    let file = file_of(&with_tag);
    let mut terms = Terms::new();
    match ein_einb::open_bytes(&file, &mut terms, &OpenOptions::default()) {
        Err(e @ EinbError::Int) => assert_eq!(
            e.to_string(),
            ".einb holds an integer, which Ein has not had since M1f S1f.5.4 (a run of \
             digits is a name): save it again from its .ein"
        ),
        Err(e) => panic!("a value tagged 0b01: refused for another reason: {e}"),
        Ok(_) => panic!("a value tagged 0b01: opened"),
    }
}
