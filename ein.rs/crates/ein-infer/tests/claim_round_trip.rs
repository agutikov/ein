//! **A printed region reads back as its own claim** — M1e2
//! [S1e2.1a.4](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.4_the_language.md),
//! ruling 1 of 2026-10-16: one form for both directions.
//!
//! For every corpus program whose R a route describes, [`claim::print`] writes
//! R as a `(region …)` (or `(false)`), the claim is spliced into the program's
//! `(solve …)` (or a new one), the result is loaded again, and
//! [`claim::check`] must say it holds. That holds the printer and the checker
//! to each other on every region the corpus has: the states, the closure over
//! the candidates' relations, every `(merged …)` and every `(owing …)`.
//!
//! ```text
//! cargo test -p ein-infer --test claim_round_trip
//! ```

use std::path::Path;

use ein_core::{Kb, Terms};
use ein_corpus::{corpus_files, repo_root};
use ein_infer::claim::{self, Options};
use ein_infer::expect::Outcome;
use ein_ir::{Ast, load, parse};

fn loaded(src: &str, dir: Option<&Path>) -> Option<(Kb, Terms, Ast)> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, src, None).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, dir).ok()?;
    Some((kb, terms, ast))
}

/// The byte range of the top-level `(solve …)` form, comments and strings
/// honoured.
fn solve_span(src: &str) -> Option<(usize, usize)> {
    let b = src.as_bytes();
    let (mut i, mut depth, mut start) = (0, 0usize, None);
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += if b[i] == b'\\' { 2 } else { 1 };
                }
            }
            b'(' => {
                if depth == 0 && src[i + 1..].starts_with("solve") {
                    let after = src[i + 6..].chars().next();
                    if after.is_none_or(|c| c.is_whitespace() || c == ')') {
                        start = Some(i);
                    }
                }
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0
                    && let Some(s) = start
                {
                    return Some((s, i));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[test]
fn every_printed_region_holds_as_its_own_claim() {
    // A quarter of the default budget's reach, so the sweep stays a gate test
    // (it was 23 s at 2¹⁹). The regions it gives up are the corpus's largest,
    // the zebra family declining and pets' 11 339 states printed, which are the
    // code paths below at a size, not new ones.
    let opts = Options {
        jobs: 4,
        budget: 1 << 16,
        ..Options::default()
    };
    let mut checked = 0usize;
    let mut kinds = [0usize; 3]; // (false), (region (owing …)), (region …)
    let mut bad = Vec::new();
    for path in corpus_files() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .display()
            .to_string();
        if rel.contains("/broken/") {
            continue;
        }
        let Ok(src) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Some((mut kb, mut terms, ast)) = loaded(&src, path.parent()) else {
            continue;
        };
        // A program that states its own claim is held by `ein test` already.
        if kb
            .program()
            .solve()
            .is_some_and(|s| ein_infer::solve_value(&ast, s, "expect").is_some())
        {
            continue;
        }
        let printed = match claim::print(&mut kb, &mut terms, &ast, &opts) {
            Ok(Some(p)) => p,
            Ok(None) => continue, // no route described R
            Err(_) => continue,   // the program does not compile or saturate
        };
        let spliced = match solve_span(&src) {
            Some((_, close)) => format!("{} :expect {printed}{}", &src[..close], &src[close..]),
            None => format!("{src}\n(solve :expect {printed})\n"),
        };
        let Some((mut kb, mut terms, ast)) = loaded(&spliced, path.parent()) else {
            bad.push(format!(
                "{rel}: the printed claim does not load:\n  {printed}"
            ));
            continue;
        };
        let node = kb
            .program()
            .solve()
            .and_then(|s| ein_infer::solve_value(&ast, s, "expect"))
            .expect("spliced");
        let c = ein_ir::claim::parse(&ast, node).expect("loaded");
        let got = claim::check(&mut kb, &mut terms, &ast, &c, &opts).expect("checks");
        checked += 1;
        kinds[if printed == "(false)" {
            0
        } else if printed.starts_with("(region (owing") {
            1
        } else {
            2
        }] += 1;
        if got.report.outcome != Outcome::Held {
            bad.push(format!(
                "{rel}: its printed region does not hold:\n  {printed}\n  {}",
                got.report.lines.join("\n  ")
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {checked} printed regions do not read back:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    // Not vacuous: the corpus has every kind, and most of it is reached.
    assert!(
        checked >= 150,
        "only {checked} programs were printed and checked"
    );
    assert!(
        kinds.iter().all(|&k| k > 0),
        "a kind of claim never occurred: (false) {}, owing {}, region {}",
        kinds[0],
        kinds[1],
        kinds[2]
    );
}
