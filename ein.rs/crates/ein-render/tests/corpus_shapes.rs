//! T1a.10.1.2 — **the T3 bytes, banked.**
//!
//! `ein-conformance --tier T3` compared every artefact of every corpus cell
//! between two engines: stdout, stderr, the exit code, the `--trace`
//! markdown, the `--dump-states` tree, every DOT view.
//! [P1a.10](../../../../docs/history/m1a_rust/README.md#p1a10--one-implementation)
//! removes the other engine, and this is what takes over: the same sweep,
//! against a checked-in digest of what ein.rs produced **while the oracle was
//! still there and still agreeing**. That timing is the whole provenance
//! argument — the manifest was blessed in a tree where `cargo test
//! --workspace` was green with the differential half running, so every line
//! in it is a byte string ein.py had signed off on.
//!
//! **One edit to the banked bytes has happened since**: the 107 lines whose
//! rendering quotes a *resolved path* were re-blessed with the checkout root
//! replaced by `<repo>` ([`portable`]). Nothing semantic moved — the elided
//! prefix was the blessing machine's directory, never the engine's answer,
//! and it is what made this test green where it was blessed and red on CI.
//!
//! It sweeps the *shape* functions rather than the CLI, because they are a
//! superset: `render rules` is one of `dot_shape`'s seventeen views, `solve`'s
//! stdout is `trace_shape`'s `--- table`, and `plan_shape` / `match_shape`
//! have no CLI surface at all. 7 462 pairs against the CLI's 901 cells.
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
//! ## The two ops that arrived with floors
//!
//! `load` and `saturate` were added by
//! [S1a.10.2](../../../../docs/history/m1a_rust/README.md#s1a102--port-the-python-test-suite),
//! and they are the only two surfaces whose sole owner had been a differential
//! sweep: `ein-ir/tests/load_parity.rs` diffed `ein_core::shape` against
//! `ir_oracle.py`'s `kb-shape`, and `ein-infer/tests/saturate_parity.rs`
//! diffed the whole verbose event stream. Both sweeps carried a **coverage
//! floor** — "at least 60 files actually loaded", "at least 50 files and 3 000
//! events" — because a sweep that compares nothing agrees about nothing, and
//! those floors move here with the ops rather than dying with the tests.
//!
//! Twelve renderings *are* checked in whole
//! ([S1a.6.11](../../../../docs/history/m1a_rust/README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing),
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
//! [P1c.1](../../../../plans/m1c_external_validation/p1c.1_stdlib_conformance/README.md).
//! The manifest's job is to make a semantic change **visible**, not to judge
//! it.

mod corpus_ops;

use corpus_ops::{ops, run};
use ein_core::Terms;
use ein_corpus::{corpus_files, golden, golden_path, repo_root};

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

/// The checkout directory, out of the rendering.
///
/// Two of the sweep's surfaces quote a *resolved* path back at the reader: a
/// `ParseError` names the file it failed in, and `import cycle: …` names the
/// chain, which `Resolver::locate` canonicalises so that two spellings of one
/// file are one node in the cycle graph. Digested raw, both bank the blessing
/// machine's checkout into a checked-in golden — green where it was blessed
/// and red everywhere else, which is what CI caught on 107 of these lines
/// while `cargo test --workspace` was clean locally.
///
/// `corpus_cli` gets the same normalisation for free by running the binary
/// with the repo as its working directory and naming its cells relatively;
/// this sweep calls the crates in-process with the absolute path
/// [`corpus_files`] hands it, so it does the substitution itself. Only the
/// root goes: a rendering that starts naming a *different* file still moves,
/// because everything below `<repo>` is still in the digest.
fn portable(root: &str, text: &str) -> String {
    text.replace(root, "<repo>")
}

#[test]
fn every_corpus_rendering_reproduces_its_digest() {
    let ops = ops();
    let mut lines: Vec<String> = Vec::new();
    let mut bytes = 0usize;
    // The two ops S1a.10.2 inherited from a differential sweep carry that
    // sweep's own coverage floors — see [`SWEPT_FLOORS`].
    let (mut loads, mut saturates, mut events) = (0usize, 0usize, 0usize);
    let root = repo_root();
    let root_str = root.display().to_string();
    // Twenty-one of the renderings below name the stdlib root, because
    // `(import std.nope)` reports the path it looked at — and
    // `stdlib::resolve_default` finds that root by walking up from the test
    // executable. A target directory outside the checkout, or an
    // `$EIN_STDLIB`, therefore resolves a *different* stdlib and moves all
    // twenty-one. That is a difference in what ran rather than drift in what
    // it produced, so it is said once here instead of arriving as twenty-one
    // digests for someone to bless.
    let stdlib = ein_ir::stdlib::resolve_default();
    assert_eq!(
        stdlib,
        ein_ir::stdlib::Source::Checkout(root.join("stdlib")),
        "the manifest is of the checkout's stdlib; this run resolved {} — \
         build inside the checkout and leave $EIN_STDLIB unset",
        stdlib.describe()
    );
    for path in &corpus_files() {
        let rel = path.strip_prefix(&root).unwrap_or(path);
        for op in &ops {
            let mut terms = Terms::new();
            let Some(text) = run(&mut terms, path, *op) else {
                continue;
            };
            match op {
                corpus_ops::Op::Load if !text.starts_with("<refused>") => loads += 1,
                corpus_ops::Op::Saturate => {
                    saturates += 1;
                    events += text.lines().count();
                }
                _ => {}
            }
            bytes += text.len();
            let text = portable(&root_str, &text);
            lines.push(line(&format!("{}::{op}", rel.display()), &text));
        }
    }
    // The sweep's own floor: a manifest that shrank because a file stopped
    // parsing would otherwise be blessed as progress.
    assert!(
        lines.len() >= 4_000,
        "only {} renderings were produced — the sweep stopped looking",
        lines.len()
    );
    assert!(
        loads >= 60,
        "only {loads} corpus files loaded — `load_parity`'s floor was 60"
    );
    assert!(
        saturates >= 50 && events >= 3_000,
        "only {saturates} files / {events} events saturated — \
         `saturate_parity`'s floor was 50 and 3 000"
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
/// rather than as a line number in a 7 462-line file.
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
