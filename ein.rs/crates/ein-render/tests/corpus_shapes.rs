//! T1a.10.1.2 — **every rendering of every corpus file, banked.**
//!
//! Every observable surface of every corpus file — the solve table and the
//! `--trace` markdown, the `--dump-states` tree, every DOT view, and the
//! shapes behind them — is rendered and digested against a checked-in
//! manifest, `tests/golden/corpus_shapes.md5`.
//!
//! **The 107 lines whose rendering quotes a *resolved path*** are banked with
//! the checkout root replaced by `<repo>` ([`portable`]). Nothing semantic is
//! elided — the prefix is the blessing machine's directory, never the
//! engine's answer, and it is what made this test green where it was blessed
//! and red on CI.
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
//! `load` — `ein_core::shape` of the loaded KB — and `saturate` — the whole
//! verbose event stream — each carry a **coverage floor**: "at least 60 files
//! actually loaded", "at least 50 files and 3 000 events", because a sweep
//! that compares nothing agrees about nothing.
//!
//! Twelve renderings *are* checked in whole as well (S1a.6.11,
//! `tests/golden/`), chosen because they are narration: they are what moves
//! when the engine changes how much of a derivation it narrates, and there a
//! digest saying *that* it moved is not enough.
//!
//! ## What it is not
//!
//! It is not evidence that the bytes are *right*. No golden is: a golden says
//! "still what it was", and
//! the claim "and what it was is what the semantics say" belongs to the
//! acceptance fixtures and to
//! [P1c.1](../../../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance).
//! The manifest's job is to make a semantic change **visible**, not to judge
//! it.

mod corpus_ops;

use corpus_ops::{manifest_line, ops, pair_name, portable, run};
use ein_core::Terms;
use ein_corpus::{corpus_files, golden, golden_path, pool, repo_root};

/// What one (file, op) pair contributes to the manifest and to the floors.
struct Rendered {
    line: String,
    bytes: usize,
    loaded: bool,
    events: Option<usize>,
}

#[test]
fn every_corpus_rendering_reproduces_its_digest() {
    let ops = ops();
    let mut lines: Vec<String> = Vec::new();
    let mut bytes = 0usize;
    // `load` and `saturate` carry their own coverage floors — see the
    // module note.
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
    // The pairs, on the pool (M1f S1f.6.39 T1f.6.39.4): each rendered on
    // its own `Terms` as before, the lines and the floors read in the pairs'
    // order, so the manifest is the one a serial loop wrote.
    let files = corpus_files();
    let units: Vec<(usize, usize)> = (0..files.len())
        .flat_map(|f| (0..ops.len()).map(move |o| (f, o)))
        .collect();
    let rel = |f: usize| {
        files[f]
            .strip_prefix(&root)
            .unwrap_or(&files[f])
            .to_path_buf()
    };
    let rendered = pool::map(
        "corpus_shapes",
        &units,
        |&(f, o)| (rel(f).display().to_string(), ops[o].to_string()),
        |&(f, o)| {
            let op = ops[o];
            let mut terms = Terms::new();
            let text = run(&mut terms, &files[f], op)?;
            Some(Rendered {
                loaded: matches!(op, corpus_ops::Op::Load) && !text.starts_with("<refused>"),
                events: matches!(op, corpus_ops::Op::Saturate).then(|| text.lines().count()),
                bytes: text.len(),
                line: manifest_line(&pair_name(&rel(f), op), &portable(&root_str, &text)),
            })
        },
    );
    for r in rendered.into_iter().flatten() {
        loads += usize::from(r.loaded);
        if let Some(n) = r.events {
            saturates += 1;
            events += n;
        }
        bytes += r.bytes;
        lines.push(r.line);
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
        "only {loads} corpus files loaded — the floor is 60"
    );
    assert!(
        saturates >= 50 && events >= 3_000,
        "only {saturates} files / {events} events saturated — \
         the floor is 50 and 3 000"
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
