//! T1a.10.1.2 — **the T3 bytes, banked.**
//!
//! `ein-conformance --tier T3` compared every artefact of every corpus cell
//! between two engines: stdout, stderr, the exit code, the `--trace`
//! markdown, the `--dump-states` tree, every DOT view.
//! [P1a.10](../../../../plans/m1a_rust/p1a.10_single_implementation/README.md)
//! removes the other engine, and this is what takes over: the same sweep,
//! against a checked-in digest of what ein.rs produced **while the oracle was
//! still there and still agreeing**. That timing is the whole provenance
//! argument — the manifest was blessed in a tree where `cargo test
//! --workspace` was green with the differential half running, so every line
//! in it is a byte string ein.py had signed off on.
//!
//! It sweeps the *shape* functions rather than the CLI, because they are a
//! superset: `render rules` is one of `dot_shape`'s seventeen views, `solve`'s
//! stdout is `trace_shape`'s `--- table`, and `plan_shape` / `match_shape`
//! have no CLI surface at all. 4 228 pairs against the CLI's 505 cells.
//!
//! ## What a digest can and cannot say
//!
//! It says **that** a rendering moved, on which file, in which view, and by
//! how many lines. It does not say **what** moved, and that is a real loss
//! against a byte golden — accepted here for one reason: the full tree is
//! **38.0 MB**, two orders of magnitude past what belongs in a repo, and a
//! golden nobody can review is not more reviewable than a hash. What recovers
//! the diff is that the engine is right there:
//!
//! ```text
//! cargo test -p ein-render --test corpus_shapes          # what moved
//! EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes   # accept it
//! ```
//!
//! Twelve renderings *are* checked in whole
//! ([S1a.6.11](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.11_fixture_goldens.md),
//! `tests/golden/`), chosen because they are the ones the parity contract
//! stopped comparing. This manifest is the other 4 216, and the split is on
//! the ledger rather than in a comment.
//!
//! ## What it is not
//!
//! It is not evidence that the bytes are *right*. Nothing in a
//! single-implementation repo is: a self-golden says "still what it was", and
//! the claim "and what it was is what the semantics say" belongs to the
//! acceptance fixtures and to
//! [P1a.11](../../../../plans/m1a_rust/p1a.11_stdlib_conformance/README.md).
//! The manifest's job is to make a semantic change **visible**, not to judge
//! it.

mod corpus_ops;

use corpus_ops::{ops, run};
use ein_core::Terms;
use ein_oracle::{corpus_files, golden, golden_path, repo_root};

/// Sixteen hex digits of `md5`, the same digest and the same truncation
/// `render::hashed_id` uses for a DOT node id. Not a security claim: the
/// question is whether a rendering changed, and 64 bits answers it.
fn digest(text: &str) -> String {
    use md5::Digest;
    let mut h = md5::Md5::new();
    h.update(text.as_bytes());
    format!("{:x}", h.finalize())[..16].to_string()
}

/// One manifest line. The line count rides along because it is the cheapest
/// thing that turns "it moved" into "it grew by 40 lines", which is often the
/// whole diagnosis.
fn line(name: &str, text: &str) -> String {
    format!("{}  {:>7}L  {name}", digest(text), text.lines().count())
}

#[test]
fn every_corpus_rendering_reproduces_its_digest() {
    let ops = ops();
    let mut lines: Vec<String> = Vec::new();
    let mut bytes = 0usize;
    for path in &corpus_files() {
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        for op in &ops {
            let mut terms = Terms::new();
            let Some(text) = run(&mut terms, path, *op) else {
                continue;
            };
            bytes += text.len();
            lines.push(line(&format!("{}::{op}", rel.display()), &text));
        }
    }
    // The sweep's own floor: a manifest that shrank because a file stopped
    // parsing would otherwise be blessed as progress.
    assert!(
        lines.len() >= 3_500,
        "only {} renderings were produced — the sweep stopped looking",
        lines.len()
    );
    eprintln!(
        "corpus shapes: {} renderings, {:.1} MB of text, {} KB of manifest",
        lines.len(),
        bytes as f64 / 1e6,
        (lines.iter().map(String::len).sum::<usize>() + lines.len()) / 1024,
    );
    let manifest = format!("{}\n", lines.join("\n"));
    if let Some(msg) = golden(&golden_path("ein-render", "corpus_shapes.md5"), &manifest) {
        panic!("{msg}\n{}", first_moved(&manifest));
    }
}

/// The lines that moved, named, so a failure reads as a list of renderings
/// rather than as a line number in a 4 228-line file.
fn first_moved(got: &str) -> String {
    let Ok(want) = std::fs::read_to_string(golden_path("ein-render", "corpus_shapes.md5")) else {
        return String::new();
    };
    let index = |t: &str| -> std::collections::BTreeMap<String, String> {
        t.lines()
            .filter_map(|l| {
                l.split_once("L  ")
                    .map(|(a, b)| (b.to_string(), a.to_string()))
            })
            .collect()
    };
    let (g, w) = (index(got), index(&want));
    let mut out = Vec::new();
    for (name, gv) in &g {
        match w.get(name) {
            None => out.push(format!("  + {name} (new)")),
            Some(wv) if wv != gv => out.push(format!("  ~ {name}\n      was {wv}\n      now {gv}")),
            _ => {}
        }
    }
    for name in w.keys() {
        if !g.contains_key(name) {
            out.push(format!("  - {name} (gone)"));
        }
    }
    format!(
        "{} of {} renderings moved:\n{}",
        out.len(),
        w.len(),
        out.iter().take(25).cloned().collect::<Vec<_>>().join("\n")
    )
}
