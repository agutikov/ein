//! The enumerator's **coverage floors** — S1a.4.1's acceptance, without the
//! oracle.
//!
//! `hypgen_parity` was eleven whole-corpus sweeps: hypothesis generation in
//! both closure regimes, the lattice join, the solve in three regimes,
//! commitment entry in both fail-fast regimes, explanation with and without
//! alternatives, and the static NAF map. Every one of those bytes is now
//! `corpus_shapes.md5`'s — 78 files each for `hyp`, `hyp+closed`, `lattice`,
//! `solve[default|exhaustive|shuffled]`, `commit`, `commit-nofailfast`,
//! `explain`, `explain-noalts` and `naf`, digested *unnarrowed*, which is
//! strictly more than the sweeps compared.
//!
//! What a digest cannot inherit is the pair of tests below, and they are the
//! reason this file still exists. Both answer the question a comparison
//! cannot ask of itself: **was there anything to compare?** A counter that is
//! zero on both sides agrees for the wrong reason, and two ordering modes that
//! are secretly the same comparison agree perfectly. Those were floors under a
//! diff; they are floors under a digest for exactly the same reason.

use ein_core::Terms;
use ein_ir::{Ast, load_file};
use ein_oracle::corpus_files;
use std::path::Path;

fn hyp_shape(path: &Path) -> Option<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    ein_infer::hyp_shape_with(&ast, &mut terms, &mut kb, false).ok()
}

fn lattice_shape(path: &Path) -> Option<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    ein_infer::lattice_shape(&ast, &mut terms, &mut kb).ok()
}

/// The eight counters the stage names, each exercised somewhere in the corpus.
///
/// The stats block is digested per file, but a key that is **zero everywhere**
/// is pinned as zero and stays pinned: a filter that was deleted outright
/// would leave every digest unchanged. This asserts each one actually fires,
/// which is what makes the digests evidence about the filters rather than
/// about their absence.
#[test]
fn every_filter_and_skip_fires_somewhere_in_the_corpus() {
    const KEYS: [&str; 8] = [
        "filtered.fact_already_exists",
        "filtered.lookahead_killed",
        "filtered.negated_fact",
        "filtered.seen_in_call",
        "pre.closed_relation",
        "pre.no_hypothesis_relation",
        "pre.relation_not_whitelisted",
        "pre.self_edge",
    ];
    let mut seen: Vec<&str> = Vec::new();
    for path in &corpus_files() {
        let Some(text) = hyp_shape(path) else {
            continue;
        };
        for key in KEYS {
            if text.contains(key) && !seen.contains(&key) {
                seen.push(key);
            }
        }
    }
    seen.sort_unstable();
    assert_eq!(
        seen, KEYS,
        "a counter never fired on the corpus, so its digest pins a zero"
    );
}

/// `score-sum` must actually order differently from `lex` somewhere.
///
/// Under `hypgen_scoring = "most-constrained"` every score is `0.0` and
/// `score-sum` *is* `lex` — and a KB with no `(config …)` block takes that
/// path (53 of the corpus's files do). If no file differentiated them, the
/// `lattice` digest would be pinning `lex` twice and the second mode would be
/// unowned.
#[test]
fn score_sum_orders_differently_from_lex_somewhere_in_the_corpus() {
    let mut differing = 0;
    for path in &corpus_files() {
        let Some(text) = lattice_shape(path) else {
            continue;
        };
        let line = |k: &str| {
            text.lines()
                .find(|l| l.starts_with(k))
                .map(|l| l[k.len()..].to_string())
        };
        if let (Some(lex), Some(score)) = (line("ORDER lex "), line("ORDER score-sum "))
            && lex != score
        {
            differing += 1;
        }
    }
    assert!(
        differing >= 5,
        "score-sum differed from lex on only {differing} files"
    );
}
