//! S1a.4.1 acceptance — the enumerator proposes the same things in the same
//! order, and drops them for the same named reasons.
//!
//! The comparison is the `hyp` / `hypskip` event stream at `verbose` plus the
//! `--hyp-stats` report, which is the whole of what a `HypGenStats` difference
//! could be: `raw`, `emitted`, every `filtered.*` key and every
//! `pre_candidate.*` key, and the *order* the candidates were built in. Order
//! matters twice over — it becomes `layer_1`'s singleton order and therefore
//! the whole traversal, and the pipeline's order decides which counter a drop
//! is attributed to, so a reordering that changed nothing about *which*
//! candidates survive would still be a parity failure.
//!
//! `n` is compared as a **position, not a field**, for the reason
//! `saturate_parity` gives: one extra event renumbers every line after it.
//!
//! S1a.4.2 adds two more surfaces over the same corpus: the generator asked
//! again **after the auto-closure pass** — a regime that moves `raw` from
//! 4 489 candidates to 3 022 and is what `--hyp-stats` reports — and the
//! static NAF dependency map with its warning text.

use ein_core::Terms;
use ein_ir::{Ast, load_file};
use ein_oracle::{Answer, IR_ORACLE, Oracle, corpus_files, repo_root, skip};
use std::path::Path;

fn rust_hyp(path: &Path) -> Option<Answer> {
    rust_hyp_with(path, false)
}

fn rust_hyp_with(path: &Path, closed: bool) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(
        match ein_infer::hyp_shape_with(&ast, &mut terms, &mut kb, closed) {
            Ok(text) => Answer::Ok(text),
            Err(msg) => Answer::Err {
                kind: "HypGenError".into(),
                msg,
            },
        },
    )
}

fn rust_naf(path: &Path) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(match ein_infer::naf_map(&ast, &mut terms, &mut kb) {
        Ok(text) => Answer::Ok(text),
        Err(e) => Answer::Err {
            kind: "SaturateError".into(),
            msg: e.to_string(),
        },
    })
}

/// One op over the whole corpus, comparing both sides line for line.
fn sweep(
    label: &str,
    op: serde_json::Value,
    rust: impl Fn(&Path) -> Option<Answer>,
    count: impl Fn(&str) -> usize,
    floors: (usize, usize),
) {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip(label);
    };
    let (mut bad, mut compared, mut items) = (Vec::new(), 0, 0usize);
    for path in &corpus_files() {
        let Some(got) = rust(path) else { continue };
        let mut req = op.clone();
        req["path"] = serde_json::json!(path.to_str().expect("utf-8"));
        let want = py.ask(req);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                compared += 1;
                items += count(a);
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { .. }, Answer::Err { .. }) => {}
            _ => bad.push(format!(
                "{name}\n  rs: {}\n  py: {}",
                brief(&got),
                brief(&want)
            )),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} files differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("{label}: {compared} files, {items} items, 0 differences");
    // A gate that passes because nothing ran is not a gate; the floors exist
    // only to catch a harness that silently stopped looking.
    assert!(
        compared >= floors.0 && items >= floors.1,
        "only {compared} files / {items} items compared"
    );
}

/// The same corpus after `emit_closed` — S1a.4.2's regime, and the one
/// `--hyp-stats` and the JSON summary report.
///
/// It is not the same check with a flag flipped: closing a relation removes
/// its candidates *before* the whitelist and blacklist are consulted, which is
/// why `no_hypothesis_relation` goes to zero here and is only reachable at all
/// in the other regime. Between them the two runs are the only way every
/// pre-candidate counter is exercised.
#[test]
fn the_whole_corpus_generates_the_same_hypotheses_after_auto_closure() {
    sweep(
        "hyp+closed",
        serde_json::json!({"op": "hyp-shape", "closed": true}),
        |p| rust_hyp_with(p, true),
        |a| a.lines().filter(|l| l.contains("\"hyp\"")).count(),
        (60, 1500),
    );
}

/// The static NAF dependency map, over a saturated cache.
#[test]
fn the_whole_corpus_reports_the_same_naf_dependencies() {
    sweep(
        "naf",
        serde_json::json!({"op": "naf-map"}),
        rust_naf,
        |a| a.lines().filter(|l| l.starts_with("NAF ")).count(),
        (60, 200),
    );
}

#[test]
fn the_whole_corpus_generates_the_same_hypotheses() {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip("the_whole_corpus_generates_the_same_hypotheses");
    };
    let (mut bad, mut compared, mut candidates) = (Vec::new(), 0, 0usize);
    for path in &corpus_files() {
        let Some(got) = rust_hyp(path) else { continue };
        let want = py.file("hyp-shape", path);
        let name = path.strip_prefix(repo_root()).unwrap_or(path).display();
        match (&got, &want) {
            (Answer::Ok(a), Answer::Ok(b)) => {
                compared += 1;
                candidates += a.lines().filter(|l| l.contains("\"hyp\"")).count();
                if a != b {
                    bad.push(format!("{name}\n{}", first_difference(a, b)));
                }
            }
            (Answer::Err { .. }, Answer::Err { .. }) => {}
            _ => bad.push(format!(
                "{name}\n  rs: {}\n  py: {}",
                brief(&got),
                brief(&want)
            )),
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {compared} files differ:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!("hyp: {compared} files, {candidates} candidates, 0 differences");
    // A gate that passes because nothing ran is not a gate. The corpus builds
    // ~3 000 candidates across ~70 loadable files; the floor is well under
    // that and exists only to catch a harness that silently stopped looking.
    assert!(
        compared >= 60 && candidates >= 2000,
        "only {compared} files / {candidates} candidates compared"
    );
}

/// Every counter the stage names, exercised somewhere in the corpus.
///
/// The stats block is compared line for line above, but a key that is zero on
/// **both** sides agrees for the wrong reason: it would also agree if neither
/// implementation had the filter at all. This asserts each one actually fires.
#[test]
fn every_filter_and_skip_fires_somewhere_in_the_corpus() {
    let mut seen: Vec<&str> = Vec::new();
    for path in &corpus_files() {
        let Some(Answer::Ok(text)) = rust_hyp(path) else {
            continue;
        };
        for key in [
            "filtered.fact_already_exists",
            "filtered.lookahead_killed",
            "filtered.negated_fact",
            "filtered.seen_in_call",
            "pre.closed_relation",
            "pre.no_hypothesis_relation",
            "pre.relation_not_whitelisted",
            "pre.self_edge",
        ] {
            if text.contains(key) && !seen.contains(&key) {
                seen.push(key);
            }
        }
    }
    seen.sort_unstable();
    assert_eq!(
        seen,
        [
            "filtered.fact_already_exists",
            "filtered.lookahead_killed",
            "filtered.negated_fact",
            "filtered.seen_in_call",
            "pre.closed_relation",
            "pre.no_hypothesis_relation",
            "pre.relation_not_whitelisted",
            "pre.self_edge",
        ],
        "a counter never fired on the corpus, so its parity is untested"
    );
}

fn brief(a: &Answer) -> String {
    match a {
        Answer::Ok(s) => format!("{} lines", s.lines().count()),
        Answer::Err { kind, msg } => format!("{kind}: {msg}"),
    }
}

/// The first differing line, with the four before it from each side.
fn first_difference(a: &str, b: &str) -> String {
    let (ours, theirs): (Vec<&str>, Vec<&str>) = (a.lines().collect(), b.lines().collect());
    for (i, (x, y)) in ours.iter().zip(theirs.iter()).enumerate() {
        if x != y {
            let from = i.saturating_sub(4);
            let context: Vec<String> = ours[from..i].iter().map(|l| format!("    {l}")).collect();
            return format!(
                "  at line {i}:\n{}\n  ein.py: {y}\n  ein.rs: {x}",
                context.join("\n")
            );
        }
    }
    let (extra, side) = if ours.len() > theirs.len() {
        (ours.get(theirs.len()), "ein.rs")
    } else {
        (theirs.get(ours.len()), "ein.py")
    };
    format!(
        "  same prefix, different length: ein.py {} lines, ein.rs {}\n  \
         first extra ({side}): {extra:?}",
        theirs.len(),
        ours.len(),
    )
}
