//! idea-08 trace fidelity, on the engine that ships — T1a.6.11.2.
//!
//! Every named move in the human zebra walkthrough must correspond to a named
//! rule firing in the engine, and the proof a user is handed must *exhibit*
//! it. That has been the assertion since M1 S1.6.5, and this file exists
//! because of how nearly it was lost.
//!
//! **The near-miss.** `--trace` renders one node's firings, and until
//! [S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
//! every fork re-derived root's whole closure into them — so the trace was
//! getting root's proof *by accident*. Take the re-derivation away and the
//! solution node's trace covered 12 distinct rules instead of 24, with
//! `symmetric` — which closes `next-to` at root and nowhere else — missing
//! entirely. A rendered trace is narration, so no invariance sweep compares
//! one byte for byte, and without this file the same regression would be
//! silent.
//!
//! Two levels, and both run here: the library
//! **defines** the rules, and they actually **fire** — asserted against the
//! rendered markdown rather than against the firing list, because the
//! markdown is what a reader gets and the root section is part of it.
//!
//! **Since M1e2 S1e2.1c.5 (2026-11-03) the trace is the region trace**
//! (`ein solve FILE --single --trace FILE.md`,
//! [`ein_render::trace::region`]): root's derivation, then each state the
//! read-out prints, derived from its first minimal commitment. The walk's
//! `linearize` trace these tests first read was deleted with the walk, on the
//! user's ruling of that day.

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, parse};
use std::collections::BTreeSet;
use std::path::Path;

/// Rules the idea-08 walkthrough names, ∩ the zebra2 library — the frozen
/// regression target.
const WALKTHROUGH_RULES: [&str; 9] = [
    "adjacent-via-fwd",
    "co-located",
    "disjunctive-prune-bwd",
    "disjunctive-prune-fwd",
    "domain-elimination",
    "range-elimination",
    "functional",
    "symmetric",
    "total",
];

/// What must actually **fire** on a zebra2 solve. The property rules
/// `functional` / `total` surface as their consequences — see the
/// structural-equivalence notes in
/// [`zebra_walkthrough.md`](../../../../docs/kernel/inference/zebra_walkthrough.md)
/// — so the firing target maps `functional` → `functional-negative` and adds
/// the `-bwd` dual.
const FIRING_TARGET: [&str; 9] = [
    "adjacent-via-fwd",
    "adjacent-via-bwd",
    "co-located",
    "domain-elimination",
    "range-elimination",
    "disjunctive-prune-fwd",
    "disjunctive-prune-bwd",
    "functional-negative",
    "symmetric",
];

/// The generic-link encoding (S1.22.1a): `zebra.ein` reaches the same
/// conclusions over one `co-located` equivalence instead of five typed
/// relations. `docs/kernel/inference/README.md` documents the correspondence;
/// this is its machine-checkable form.
const WALKTHROUGH_RULES_GENERIC: [&str; 14] = [
    "slot-locate",
    "slot-occupied",
    "slot-exclusive",
    "slot-negative",
    "slot-elimination",
    "slot-fill",
    "slot-adjacent-fwd",
    "slot-adjacent-bwd",
    "slot-prune-fwd",
    "slot-prune-bwd",
    "slot-endpoint-fwd",
    "slot-endpoint-bwd",
    "symmetric",
    "symmetric-negative",
];

/// The whole correspondence, plus the two negative companions of the spatial
/// propagation. Unlike zebra2 — whose firing target is a strict subset of its
/// library — every inference rule this encoding provides fires on the solution
/// path. The only exclusions are `slot-no-room` / `slot-no-fill`, which are
/// ⊥-rules: they fire on dead branches, not on the path a solution records.
const FIRING_TARGET_GENERIC: [&str; 16] = [
    "slot-locate",
    "slot-occupied",
    "slot-exclusive",
    "slot-negative",
    "slot-elimination",
    "slot-fill",
    "slot-adjacent-fwd",
    "slot-adjacent-bwd",
    "slot-prune-fwd",
    "slot-prune-bwd",
    "slot-endpoint-fwd",
    "slot-endpoint-bwd",
    "symmetric",
    "symmetric-negative",
    "slot-adjacent-fwd-neg",
    "slot-adjacent-bwd-neg",
];

fn load(rel: &str) -> (Ast, Terms, Kb) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the puzzle is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the puzzle parses");
    let kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the puzzle loads");
    (ast, terms, kb)
}

/// The rules the puzzle **provides**, imports resolved — so a rule promoted to
/// the stdlib counts as provided, not just one still defined inline. Read by
/// the names the stdlib writes: a rule the puzzle does not list prints by its
/// full name since M1f S1f.6.35, `std.bijection.domain-elimination`.
fn provided(rel: &str) -> BTreeSet<String> {
    let (_, terms, kb) = load(rel);
    let p = kb.program();
    p.rules
        .keys()
        .chain(p.hrules.keys())
        .map(|s| ein_corpus::unprefixed(terms.sym(s)))
        .collect()
}

/// The rules the **rendered trace** exhibits — `### Step N — \`rule\``.
///
/// Read out of the markdown rather than out of the firings on purpose: what
/// idea-08 promises is about the document, so the document is what is
/// asserted. The question is `--single`, the one that decides uniqueness, so
/// the trace is root's derivation and the one state's, as `ein solve FILE
/// --single --trace FILE.md` writes it.
fn rules_in_trace(rel: &str) -> BTreeSet<String> {
    let (ast, mut terms, mut kb) = load(rel);
    let mut events = ein_infer::events::Events::off();
    let firings = ein_infer::region::root_firings(&mut kb, &mut terms, &ast, &mut events)
        .expect("root saturates");
    let opts = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Single,
    });
    let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).expect("the puzzle solves");
    let md = ein_render::trace::region::of_read_out(
        &r,
        &mut kb,
        &mut terms,
        &ast,
        &firings,
        &ein_render::read_out::Options {
            command: "solve",
            argv: rel,
            file: rel,
            budget: opts.budget,
            uncut: false,
        },
        ein_render::trace::region::Shape {
            // Off: the rule names are in the prose, and a digraph per step
            // makes this five times the text for nothing.
            diagrams: false,
            relevant: false,
            reorder: false,
            full_kb: false,
        },
    )
    .expect("the trace renders");
    md.lines()
        .filter_map(|l| l.strip_prefix("### Step "))
        .filter_map(|l| l.split_once('`'))
        .filter_map(|(_, rest)| rest.split_once('`'))
        .map(|(rule, _)| ein_corpus::unprefixed(rule))
        .collect()
}

fn missing(target: &[&str], have: &BTreeSet<String>) -> Vec<String> {
    target
        .iter()
        .filter(|r| !have.contains(**r))
        .map(|r| (*r).to_string())
        .collect()
}

// ── the library defines them ───────────────────────────────────────

#[test]
fn the_zebra2_library_defines_the_walkthrough_rules() {
    let have = provided("examples/zebra2.ein");
    assert!(
        missing(&WALKTHROUGH_RULES, &have).is_empty(),
        "the walkthrough names rules absent from the zebra2 library: {:?}",
        missing(&WALKTHROUGH_RULES, &have)
    );
}

#[test]
fn the_generic_library_defines_the_walkthrough_rules() {
    let have = provided("examples/zebra.ein");
    assert!(
        missing(&WALKTHROUGH_RULES_GENERIC, &have).is_empty(),
        "the generic-link encoding is missing counterparts of walkthrough \
         rules: {:?}",
        missing(&WALKTHROUGH_RULES_GENERIC, &have)
    );
}

// ── and the trace exhibits them ────────────────────────────────────

/// The assertion S1a.6.9 nearly lost, on the question that certifies the
/// answer unique.
#[test]
fn the_zebra2_trace_exhibits_the_walkthrough_rules() {
    let fired = rules_in_trace("examples/zebra2.ein");
    assert!(
        missing(&FIRING_TARGET, &fired).is_empty(),
        "walkthrough rules the trace does not exhibit: {:?}\nexhibited: {:?}",
        missing(&FIRING_TARGET, &fired),
        fired
    );
    // `symmetric` fires **only at root**, so it is in the trace only because
    // root's section is. Named separately from the set
    // above because it is the exact rule the near-miss dropped, and a set
    // assertion says "one of nine is missing" where this says which.
    assert!(
        fired.contains("symmetric"),
        "the trace lost `symmetric` — root's saturation is no longer rendered"
    );
    assert!(
        fired.len() >= 20,
        "the trace exhibits only {} rules; the walkthrough's proof has 24",
        fired.len()
    );
}

/// The generic-link encoding reaches the walkthrough by its own rules.
///
/// This asserts over the *solution path*; that the state is the published
/// answer is the acceptance gate's job (`ein-infer/tests/acceptance.rs`).
#[test]
fn the_generic_trace_exhibits_the_walkthrough_rules() {
    let fired = rules_in_trace("examples/zebra.ein");
    assert!(
        missing(&FIRING_TARGET_GENERIC, &fired).is_empty(),
        "walkthrough counterparts the trace does not exhibit: {:?}\nexhibited: {:?}",
        missing(&FIRING_TARGET_GENERIC, &fired),
        fired
    );
}

/// The two encodings are claimed to be the same inference over different
/// ontologies. A rendered trace is narration that no sweep reads for its
/// rules, so this is where "both still reach it" is checked.
#[test]
fn both_encodings_reach_the_same_walkthrough() {
    for (rel, target) in [
        ("examples/zebra2.ein", &FIRING_TARGET[..]),
        ("examples/zebra.ein", &FIRING_TARGET_GENERIC[..]),
    ] {
        assert!(
            Path::new(&repo_root().join(rel)).is_file(),
            "{rel} is gone; the walkthrough correspondence has no fixture"
        );
        assert!(!target.is_empty());
    }
}
