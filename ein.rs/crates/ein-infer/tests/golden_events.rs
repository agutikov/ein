//! The `--events` stream at `verbose`, against ein.rs's own goldens —
//! [T1a.6.11.4](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md).
//!
//! **What these are for is the half T2 stopped reading.** Since
//! [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
//! the cross-engine comparison reads a segment's *derivation* — the facts its
//! non-redundant firings produced and the rules that produced them — and
//! elides the scheduling traffic that got it there: `enqueue`, `park`,
//! `admit`, `retire`, `quiesce`, `alt`, `compile`, and every redundant firing.
//! That is the right cut between two engines that deliberately narrate
//! differently
//! ([D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)),
//! and it means nothing else would notice ein.rs's own scheduling changing.
//! These goldens are what notices.
//!
//! Kept **small on purpose**: a fixture whose stream is thousands of lines is
//! a golden nobody reads and everybody regenerates. Three from
//! `examples/features/` and `examples/branching/`, never the zebra puzzles —
//! exhaustive `zebra2` alone emits 68 670 events.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-infer
//! ```

use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_infer::events::{Buffer, Events, Level};
use ein_infer::solve::{NoDumper, OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use std::collections::BTreeMap;

/// `(golden name, corpus entry, exhaustive)` — one per shape of *scheduling*,
/// which is what the elided half is made of.
///
/// | golden | events | why this one |
/// |---|---:|---|
/// | `symmetric-native` | 43 | the native arg-swap mirror: `mirror` events, which are firings reported under another name and appear nowhere else in the corpus this cheaply |
/// | `naf-boundary` | 279 | the only small fixture that emits the whole scheduling vocabulary — `park`, `admit`, `retire`, `alt`, `compile`, `enqueue`, `quiesce` — because it forks *and* runs a lookahead guard to retirement |
/// | `unconditional` | 124 | a long root derivation that never forks: eleven redundant firings with no fork boundary to blame them on |
///
/// Between them, 446 lines, every elided class, and the assertion below is
/// what keeps that true rather than remembered.
const STREAMS: [(&str, &str, bool); 3] = [
    (
        "symmetric-native",
        "examples/features/06_symmetric_native.ein",
        false,
    ),
    (
        "naf-boundary",
        "examples/branching/12_typed_blind_solve.ein",
        false,
    ),
    ("unconditional", "examples/domain_elim/ab.ein", false),
];

fn stream(rel: &str, exhaustive: bool) -> String {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    let mut kb =
        ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("the fixture loads");
    let buf = Buffer::new();
    // No `impl` / `file` / `argv` on the `run` event: those are the CLI's, and
    // two of the three are what `ein_parity::events::comparable` drops
    // anyway. A golden that carried an absolute path would fail in every
    // other checkout.
    let mut events = Events::to(Box::new(buf.clone()), Level::Verbose);
    let opts = SolveOptions {
        stop_after: if exhaustive { None } else { Some(1) },
        max_set_size: 5,
        on_budget: OnBudget::Verdict,
        ..SolveOptions::default()
    };
    solve(&mut kb, &mut terms, &ast, &mut events, &mut NoDumper, &opts)
        .expect("the fixture solves");
    buf.to_string_lossy()
}

fn classes(log: &str) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for line in log.lines().filter(|l| !l.trim().is_empty()) {
        let v: serde_json::Value = serde_json::from_str(line).expect("one JSON object per line");
        let kind = v["e"].as_str().unwrap_or("<no e>").to_string();
        *out.entry(kind).or_insert(0) += 1;
    }
    out
}

#[test]
fn every_event_stream_reproduces_its_golden() {
    let mut bad: Vec<String> = Vec::new();
    for (name, rel, exhaustive) in STREAMS {
        let got = stream(rel, exhaustive);
        assert!(
            got.lines().count() < 400,
            "{rel} emits {} events — too many for a golden anyone reads",
            got.lines().count()
        );
        if let Some(e) = golden(
            &golden_path("ein-infer", &format!("events_{name}.jsonl")),
            &got,
        ) {
            bad.push(e);
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
}

/// The goldens have to *contain* the elided classes, or they are not covering
/// the half the parity contract stopped reading — they are just three more
/// files that happen to be stable.
#[test]
fn between_them_the_goldens_cover_every_elided_class() {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut redundant = 0usize;
    for (_, rel, exhaustive) in STREAMS {
        let log = stream(rel, exhaustive);
        for (k, n) in classes(&log) {
            *seen.entry(k).or_insert(0) += n;
        }
        redundant += log
            .lines()
            .filter(|l| l.contains("\"e\": \"fire\"") && l.contains("\"redundant\": true"))
            .count();
    }
    for kind in ein_parity::events::SCHEDULING {
        assert!(
            seen.contains_key(kind),
            "no golden emits a `{kind}` event, so nothing pins it: {seen:?}"
        );
    }
    assert!(
        redundant > 0,
        "no golden contains a redundant firing — the largest elided class is unpinned"
    );
    assert!(
        seen.contains_key("mirror"),
        "no golden emits a `mirror`, and a mirror is a firing under another name"
    );
}
