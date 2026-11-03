//! The enumerator's **coverage floors** — S1a.4.1's acceptance.
//!
//! The enumerator's bytes are `corpus_shapes.md5`'s `hyp` and `hyp+closed`
//! rows, digested *unnarrowed*.
//!
//! What a digest cannot do is the test below, and it is the reason this file
//! exists. It answers the question a digest cannot ask of itself: **was there
//! anything to pin?** A counter that is zero everywhere is stable for the
//! wrong reason. Its sibling, that the walk's `score-sum` lattice order
//! differs from `lex` somewhere, went with the walk's lattice order at M1e2
//! S1e2.1c.5 (2026-11-03).

use ein_core::Terms;
use ein_corpus::corpus_files;
use ein_ir::{Ast, load_file};
use std::path::Path;

fn hyp_shape(path: &Path) -> Option<String> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    ein_infer::hyp_shape_with(&ast, &mut terms, &mut kb, false).ok()
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
