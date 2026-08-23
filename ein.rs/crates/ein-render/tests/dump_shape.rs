//! The `--dump-states` tree's own invariants — S1a.5.3's acceptance, without
//! the oracle.
//!
//! Five modes per corpus entry. The tree has no line protocol to diff over, so
//! it is *rendered* as one text — every file, sorted by path, with its bytes —
//! and until
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite)
//! that text was diffed against ein.py's. The bytes are now
//! `corpus_shapes.md5`'s (5 modes × 107 files) plus `golden_dump.rs`'s two
//! whole trees, which is strictly more than the diff covered: the manifest
//! digests the tree *unnarrowed*, where the diff applied D3's narration cut.
//!
//! What has no owner in a digest is the **policy**, and that is what is left
//! here: on a run the budget cuts, the timeline is flushed and the summary is
//! not. A digest would record whichever files happen to contain — this records
//! why.
//!
//! - **`monotonic`** / **`lattice`** — the two file dumpers, whose layouts
//!   differ (`layers/layer_NN_pre.ein` versus `layers/layer_NN/pre.ein`, and
//!   the per-commitment `enterings/` tree only the second writes).
//! - **`progress`** — the live `-v` view *with* an `out_dir`, because the two
//!   compose and the live view is the file dumper plus a stream.
//! - **`abort`** — a budget small enough to trip mid-search, under the raising
//!   policy: `summary.json` must be **absent** and the timeline flushed anyway.
//! - **`snapshot`** — the `LatticeSnapshotV1` projection and the lattice DOT
//!   rendered from it.

use ein_core::Terms;
use ein_corpus::{corpus_files, repo_root};
use ein_ir::{Ast, parse};
use ein_render::shape::dump_shape;
use std::path::Path;

fn rendered(path: &Path, mode: &str) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    dump_shape(&mut ast, &mut terms, &forms, path.parent(), mode, 1).ok()
}

/// **A budget abort leaves a timeline and no summary.**
///
/// The policy, on every corpus file the budget actually cuts. `summary.json`
/// is written from the *verdict*, and an aborted run has none — so a summary
/// here would be a record of a conclusion that was never reached, which is
/// worse than no record. The timeline is the opposite: it is appended as the
/// search goes, and the run that most needs one is the run that did not
/// finish, so it must survive the abort path rather than be flushed at the
/// end.
///
/// The floor is the abort *count*, and it is the part that would rot silently:
/// the budget is a fixed number of enterings, so a search that got faster
/// stops tripping it, and a sweep over a corpus that never aborts asserts
/// nothing while still passing. `dump_parity`'s own floor was ten, over the
/// same fixed budget; today 36 of the 73 files that render a dump trip it.
#[test]
fn a_budget_abort_leaves_a_timeline_and_no_summary() {
    let (mut aborts, mut swept) = (0usize, 0usize);
    for path in &corpus_files() {
        let Some(text) = rendered(path, "abort") else {
            continue;
        };
        swept += 1;
        if !text.starts_with("ABORTED True") {
            continue;
        }
        aborts += 1;
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        assert!(
            !text.contains("=== summary.json"),
            "{name}: a budget abort left a summary.json behind"
        );
        assert!(
            text.contains("=== 00_timeline.jsonl"),
            "{name}: a budget abort left no timeline behind"
        );
    }
    assert!(
        swept >= 70 && aborts >= 10,
        "only {aborts} of {swept} files tripped the budget — \
         the abort path is no longer being exercised"
    );
}

/// `dump_shape` elides the `enterings/` subtree where it is produced, so its
/// marker is the one thing about the rule that lives outside `ein-parity`.
/// This is what keeps that from drifting.
#[test]
fn the_produced_marker_is_the_one_the_contract_expects() {
    assert_eq!(ein_parity::NARRATED, "<narrated>");
}
