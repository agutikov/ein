//! The two halves of a `--dump-states` tree the cross-engine diff stopped
//! reading — [S1a.6.11](../../../../docs/history/m1a_rust/README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing).
//!
//! `dump_parity.rs` compares the whole tree against ein.py byte for byte,
//! with two exceptions, both
//! [D3](../../../../docs/history/m1a_rust/divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it):
//!
//! - the **`enterings/` subtree** — a fork's own firing list, its state dump
//!   in the fork's derivation order with the fork's `:rule` annotations, and,
//!   for a dying fork, the core of whichever clash its fail-fast prefix
//!   reached. Elided where it is *produced*, because `zebra2-hints` writes
//!   6.6 MiB of it, so the file set is all that survives the diff.
//! - the **snapshot's dead `state_key`s** and the two lattice DOTs the DAG
//!   merges by them.
//!
//! …plus the one field of `00_timeline.jsonl` that is blanked rather than
//! elided, its per-entering `firings` count.
//!
//! All three are pinned here instead, on a fixture small enough to read.
//!
//! ```text
//! EIN_BLESS=1 cargo test -p ein-render
//! ```

use ein_core::Terms;
use ein_corpus::{golden, golden_path, repo_root};
use ein_infer::solve::{OnBudget, SolveOptions, solve};
use ein_ir::{Ast, parse};
use ein_render::dump::LatticeDumper;
use ein_render::shape::dump_shape;
use std::path::{Path, PathBuf};

/// Small, forks, and **kills** three of its commitments — so the dead-fork
/// half of the rule has something to be pinned against.
const FIXTURE: &str = "examples/lattice/01_subset_pruned.ein";

fn parsed(rel: &str) -> (PathBuf, Ast, Terms, Vec<ein_ir::NodeId>) {
    let path = repo_root().join(rel);
    let text = std::fs::read_to_string(&path).expect("the fixture is checked in");
    let mut ast = Ast::new();
    let terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).expect("the fixture parses");
    (path, ast, terms, forms)
}

/// Solve `rel` under a `LatticeDumper` and hand back the tree it wrote. The
/// caller removes it.
fn run_dump(rel: &str) -> PathBuf {
    let (path, mut ast, mut terms, forms) = parsed(rel);
    let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).expect("it loads");
    let tmp = std::env::temp_dir().join(format!(
        "ein-golden-dump-{}",
        path.file_stem().unwrap_or_default().to_string_lossy()
    ));
    let _ = std::fs::remove_dir_all(&tmp);
    let out = tmp.join("states");
    let mut dumper = LatticeDumper::new(Some(&out)).expect("a dumper");
    let opts = SolveOptions {
        stop_after: None,
        max_set_size: 3,
        max_enterings: Some(60),
        on_budget: OnBudget::Verdict,
        store_lattice: true,
        ..SolveOptions::default()
    };
    let mut events = ein_infer::events::Events::off();
    solve(&mut kb, &mut terms, &ast, &mut events, &mut dumper, &opts).expect("it solves");
    out
}

/// The timeline and every file under `enterings/`, sorted, with their bytes —
/// the same dumb rendering `dump_shape` uses for the rest of the tree, so a
/// missing file, an extra file and a changed byte all read the same way.
///
/// The timeline is here for **one field**: its per-entering `firings` count,
/// which is the only part of `00_timeline.jsonl` the cross-engine diff blanks.
/// The rest of that file is byte-compared against ein.py by `dump_parity`, so
/// this pins it twice — cheap at a dozen lines, and the alternative was to
/// argue that the count is implied by the firing lists below, which is not
/// something this tree bears out: it has twelve `firings.jsonl` and ten
/// `entering` records.
fn enterings_tree(out: &Path) -> String {
    let mut files: Vec<PathBuf> = vec![out.join("00_timeline.jsonl")];
    collect(&out.join("enterings"), &mut files);
    files[1..].sort();
    let mut lines: Vec<String> = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(out).unwrap_or(f).to_string_lossy();
        lines.push(format!("=== {rel}"));
        let text = std::fs::read_to_string(f).unwrap_or_default();
        lines.extend(text.lines().map(mask_clock));
    }
    assert!(
        files.len() > 1,
        "no per-entering dumps were written — this golden pins nothing"
    );
    lines.join("\n") + "\n"
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// The clocks, which are on the normalisation list for the ordinary reason:
/// no two runs agree on them, and a golden that carried one would fail on
/// every machine including this one.
fn mask_clock(line: &str) -> String {
    let mut out = line.to_string();
    for key in ["\"ts_ms\": ", "\"elapsed_seconds\": ", "\"wall_seconds\": "] {
        while let Some(at) = out.find(key) {
            let start = at + key.len();
            let end = out[start..]
                .find(|c: char| !matches!(c, '0'..='9' | '.' | 'e' | 'E' | '+' | '-'))
                .map_or(out.len(), |j| start + j);
            out.replace_range(start..end, "<ts>");
            // Rewrite the key so the scan cannot find it again.
            out.replace_range(at..at + 1, "\u{1}");
        }
        out = out.replace('\u{1}', "\"");
    }
    out
}

#[test]
fn a_forks_own_dump_reproduces_its_golden() {
    let out = run_dump(FIXTURE);
    let got = enterings_tree(&out);
    let _ = std::fs::remove_dir_all(out.parent().unwrap_or(&out));
    assert!(
        got.contains("firings.jsonl") && got.contains("\"firings\": "),
        "the per-entering firing list, or the count the diff blanks, is not \
         in the tree any more"
    );
    assert!(
        got.contains("unsat_core.jsonl"),
        "no fork died, so a dying fork's core — the other half of the rule — \
         is not pinned here"
    );
    if let Some(e) = golden(
        &golden_path("ein-render", "dump_enterings_subset-pruned.txt"),
        &got,
    ) {
        panic!("{e}");
    }
}

#[test]
fn the_snapshot_projection_reproduces_its_golden() {
    let (path, mut ast, mut terms, forms) = parsed(FIXTURE);
    let got = dump_shape(&mut ast, &mut terms, &forms, path.parent(), "snapshot", 1)
        .expect("the snapshot renders");
    // The two things `ein-parity` blanks out of this text when it is compared
    // against ein.py — so if they were not here, this golden would be pinning
    // the part that was never in question.
    assert!(
        got.contains("  deads          [{"),
        "the snapshot has no dead state keys, which is the half this pins"
    );
    assert!(
        got.contains("=== dot solution\ndigraph"),
        "the snapshot's lattice DOT is not rendered"
    );
    if let Some(e) = golden(
        &golden_path("ein-render", "dump_snapshot_subset-pruned.txt"),
        &got,
    ) {
        panic!("{e}");
    }
}
