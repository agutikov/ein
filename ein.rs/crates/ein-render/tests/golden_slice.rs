//! The `slice` provenance cone, against ein.rs's own goldens —
//! [T1a.6.11.3](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md).
//!
//! `dot_parity.rs` compares seventeen DOT views of every corpus entry byte for
//! byte against ein.py. One of them — `slice` — renders a **derivation**, so
//! it moved with
//! [D3](../../../../plans/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
//! on sixteen entries and is on `ein-parity`'s narration list: still run on
//! both sides, both still have to answer, no longer byte-compared. This is
//! what replaces the byte check.
//!
//! Two entries, both from that sixteen, chosen for size: the cone is
//! `render_slice` over a solution's commitment and firings, so a puzzle with a
//! long proof makes a golden nobody reads.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-render
//! ```

use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_ir::{Ast, parse};
use ein_render::shape::dot_shape;

/// `(golden name, corpus entry)`. Both are on `dot_parity`'s
/// `NARRATED_SLICES`, which is the asserted list of entries whose cone D3
/// actually moves — so these two are pinning bytes nothing else looks at.
const SLICES: [(&str, &str); 2] = [
    ("forall", "examples/features/03_forall.ein"),
    ("two-level", "examples/branching/04_two_levels.ein"),
];

fn slice_view(rel: &str) -> String {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    dot_shape(&mut ast, &mut terms, &forms, path.parent(), "slice", 1)
        .expect("the slice view renders")
}

#[test]
fn every_slice_cone_reproduces_its_golden() {
    let mut bad: Vec<String> = Vec::new();
    for (name, rel) in SLICES {
        let got = slice_view(rel);
        // The cone is a *derivation*, so an empty one is the regression this
        // exists to catch — S1a.6.9's near-miss was exactly a proof that
        // quietly lost most of itself.
        assert!(
            got.contains("--- solution 0") && got.contains("digraph"),
            "{rel}: the slice view rendered no solution cone"
        );
        if let Some(e) = golden(
            &golden_path("ein-render", &format!("slice_{name}.dot")),
            &got,
        ) {
            bad.push(e);
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
}
