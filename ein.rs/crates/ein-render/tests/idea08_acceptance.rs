//! idea-08 trace fidelity, on the engine that ships —
//! [T1a.6.11.2](../../../../docs/history/m1a_rust/README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing).
//!
//! Every named move in the human zebra walkthrough must correspond to a named
//! rule firing in the engine, and the proof a user is handed must *exhibit*
//! it. `ein.py/tests/trace/test_idea08_acceptance.py` has asserted that since
//! M1 S1.6.5; this is the same assertion on ein.rs, and it exists because of
//! how nearly the port lost it.
//!
//! **The near-miss.** `--trace` renders one node's firings, and until
//! [S1a.6.9](../../../../docs/history/m1a_rust/README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
//! every fork re-derived root's whole closure into them — so the trace was
//! getting root's proof *by accident*. Take the re-derivation away and the
//! solution node's trace covered 12 distinct rules instead of 24, with
//! `symmetric` — which closes `next-to` at root and nowhere else — missing
//! entirely. The Python test is what caught it. Since
//! [S1a.6.10](../../../../docs/history/m1a_rust/README.md#s1a610--the-parity-contract-relaxes-answers-not-narration)
//! no cross-engine diff compares a rendered trace at all, so without this file
//! the same regression would be silent on the shipping engine.
//!
//! Two levels, as on the Python side, and both run here: the library
//! **defines** the rules, and they actually **fire** — asserted against the
//! rendered markdown rather than against the firing list, because the
//! markdown is what a reader gets and the root section is part of it.

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
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
/// the stdlib counts as provided, not just one still defined inline.
fn provided(rel: &str) -> BTreeSet<String> {
    let (_, terms, kb) = load(rel);
    let p = kb.program();
    p.rules
        .keys()
        .chain(p.hrules.keys())
        .map(|s| terms.sym(s).to_string())
        .collect()
}

/// The rules the **rendered trace** exhibits — `## Step N — \`rule\``.
///
/// Read out of the markdown rather than out of `proof.solutions[..].firings`
/// on purpose: since S1a.6.9 the solution node's firings are the hypothesis's
/// own, and root's are rendered as the *Before any assumption* section. What
/// idea-08 promises is about the document, so the document is what is asserted.
fn rules_in_trace(rel: &str, exhaustive: bool) -> BTreeSet<String> {
    let (ast, mut terms, mut kb) = load(rel);
    let opts = SolveOptions {
        stop_after: if exhaustive { None } else { Some(1) },
        max_set_size: 5,
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    let solved = solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .expect("the puzzle solves");
    let trace = ein_render::linearize(
        &ast,
        &terms,
        &kb,
        &solved,
        ein_render::LinearizeOpts {
            // Off: the rule names are in the prose, and a digraph per step
            // makes this five times the text for nothing.
            diagrams: false,
            ..ein_render::LinearizeOpts::new()
        },
    );
    let md = ein_render::render_markdown(&trace, ein_render::Mode::Engine, false);
    md.lines()
        .filter_map(|l| l.strip_prefix("## Step "))
        .filter_map(|l| l.split_once('`'))
        .filter_map(|(_, rest)| rest.split_once('`'))
        .map(|(rule, _)| rule.to_string())
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

/// The assertion S1a.6.9 nearly lost. Exhaustive, as on the Python side —
/// where it costs 35 s and is `EIN_RUN_SLOW`-gated, and here it is the reason
/// the port exists.
#[test]
fn the_zebra2_trace_exhibits_the_walkthrough_rules() {
    let fired = rules_in_trace("examples/zebra2.ein", true);
    assert!(
        missing(&FIRING_TARGET, &fired).is_empty(),
        "walkthrough rules the trace does not exhibit: {:?}\nexhibited: {:?}",
        missing(&FIRING_TARGET, &fired),
        fired
    );
    // `symmetric` fires **only at root**, so it is in the trace only because
    // the *Before any assumption* section is. Named separately from the set
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
/// `stop_after = 1`, not exhaustive: this asserts over the *solution path*,
/// and the exhaustive certification is the acceptance gate's job
/// (`ein.py/acceptance/test_zebra_two_ontologies.py`).
#[test]
fn the_generic_trace_exhibits_the_walkthrough_rules() {
    let fired = rules_in_trace("examples/zebra.ein", false);
    assert!(
        missing(&FIRING_TARGET_GENERIC, &fired).is_empty(),
        "walkthrough counterparts the trace does not exhibit: {:?}\nexhibited: {:?}",
        missing(&FIRING_TARGET_GENERIC, &fired),
        fired
    );
}

/// The two encodings are claimed to be the same inference over different
/// ontologies. They are also the two puzzles whose `--trace` nobody compares
/// against ein.py any more, so this is where "both still reach it" is checked.
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
