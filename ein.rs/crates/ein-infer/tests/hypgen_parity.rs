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

fn rust_lattice(path: &Path) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(match ein_infer::lattice_shape(&ast, &mut terms, &mut kb) {
        Ok(text) => Answer::Ok(text),
        Err(e) => Answer::Err {
            kind: "SaturateError".into(),
            msg: e.to_string(),
        },
    })
}

fn rust_commit(path: &Path, fail_fast: bool) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(
        match ein_infer::commit_shape(&ast, &mut terms, &mut kb, fail_fast) {
            Ok(text) => Answer::Ok(text),
            Err(e) => Answer::Err {
                kind: "SaturateError".into(),
                msg: e.to_string(),
            },
        },
    )
}

fn rust_explain(path: &Path, alts: bool) -> Option<Answer> {
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let mut kb = load_file(&mut ast, &mut terms, path).ok()?;
    Some(
        match ein_infer::explain_shape(&ast, &mut terms, &mut kb, alts) {
            Ok(text) => Answer::Ok(text),
            Err(e) => Answer::Err {
                kind: "SaturateError".into(),
                msg: e.to_string(),
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
///
/// `divergent` names the repo-relative paths where ein.py is **expected** to
/// raise and ein.rs to answer — the accepted ledger entries. They are
/// asserted, not tolerated: a file listed here that stops diverging fails just
/// as loudly as one that starts, because a ledger entry nobody can reproduce
/// is not a decision.
fn sweep(
    label: &str,
    op: serde_json::Value,
    rust: impl Fn(&Path) -> Option<Answer>,
    count: impl Fn(&str) -> usize,
    floors: (usize, usize),
    divergent: &[&str],
) {
    let Some(mut py) = Oracle::start(IR_ORACLE) else {
        return skip(label);
    };
    let (mut bad, mut compared, mut items) = (Vec::new(), 0, 0usize);
    let mut seen_divergent: Vec<String> = Vec::new();
    for path in &corpus_files() {
        let Some(got) = rust(path) else { continue };
        let mut req = op.clone();
        req["path"] = serde_json::json!(path.to_str().expect("utf-8"));
        let want = py.ask(req);
        let rel = path.strip_prefix(repo_root()).unwrap_or(path);
        let name = rel.display();
        let expected = divergent.contains(&rel.to_str().unwrap_or_default());
        match (&got, &want) {
            (Answer::Ok(_), Answer::Err { .. }) if expected => {
                seen_divergent.push(rel.to_str().unwrap_or_default().to_string());
            }
            _ if expected => bad.push(format!(
                "{name} is a ledger entry and no longer diverges\n  rs: {}\n  py: {}",
                brief(&got),
                brief(&want)
            )),
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
    seen_divergent.sort();
    let mut want_divergent: Vec<String> = divergent.iter().map(|s| s.to_string()).collect();
    want_divergent.sort();
    assert_eq!(
        seen_divergent, want_divergent,
        "the ledger's divergent files are not the ones that diverged"
    );
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
        &[],
    );
}

/// The lattice's arithmetic — the join, both ordering modes, and the no-good
/// store's subsumption bookkeeping.
///
/// This is the sweep that reaches
/// [D2](../../../../plans/m1a_rust/divergences.md): `layer_1`'s `sorted(alive)`
/// is the one comparison in the engine that ein.py cannot always make.
#[test]
fn the_whole_corpus_joins_the_same_layers() {
    sweep(
        "lattice",
        serde_json::json!({"op": "lattice-shape"}),
        rust_lattice,
        |a| a.lines().filter(|l| l.contains("\"nogood\"")).count(),
        (60, 300),
        // D2 — `apriori.layer_1`'s `sorted(alive)` raises on mixed-type args
        // and `Value` is totally ordered, so ein.rs answers where ein.py
        // crashes. This is the *only* op that reaches it, and the only file
        // that can produce one.
        &["examples/ein-bugs/mixed-type-hypothesis.ein"],
    );
}

/// The commitment primitive, in both fail-fast regimes.
///
/// `enable_fail_fast_fork` is the engine's one pure speed lever, so the two
/// runs must agree on every *verdict* and differ only in how much of a dying
/// fork's saturation was done — which is exactly what the `kind` / `firings` /
/// `facts` columns say.
#[test]
fn the_whole_corpus_enters_the_same_commitments() {
    sweep(
        "commit",
        serde_json::json!({"op": "commit-shape"}),
        |p| rust_commit(p, true),
        |a| a.lines().filter(|l| l.starts_with("ENTER ")).count(),
        (60, 100),
        // D2 again: `layer_1` is how this op picks its candidates.
        &["examples/ein-bugs/mixed-type-hypothesis.ein"],
    );
}

#[test]
fn the_whole_corpus_enters_the_same_commitments_without_fail_fast() {
    sweep(
        "commit-slow",
        serde_json::json!({"op": "commit-shape", "fail-fast": false}),
        |p| rust_commit(p, false),
        |a| a.lines().filter(|l| l.starts_with("ENTER ")).count(),
        (60, 100),
        &["examples/ein-bugs/mixed-type-hypothesis.ein"],
    );
}

/// The three searches over the AND/OR justification graph.
///
/// Run twice, because `record_alternative_justifications` decides whether the
/// graph *is* an OR-graph: with it off a fact has one derivation and the
/// search degenerates to the pre-S1.21.7 recorded-primary walk, which is a
/// different code path and a supported configuration.
#[test]
fn the_whole_corpus_explains_the_same_way() {
    sweep(
        "explain",
        serde_json::json!({"op": "explain-shape"}),
        |p| rust_explain(p, true),
        |a| a.lines().filter(|l| l.starts_with("EXPLAIN ")).count(),
        (60, 150),
        &[],
    );
}

#[test]
fn the_whole_corpus_explains_the_same_way_without_alternatives() {
    sweep(
        "explain-noalts",
        serde_json::json!({"op": "explain-shape", "alts": false}),
        |p| rust_explain(p, false),
        |a| a.lines().filter(|l| l.starts_with("EXPLAIN ")).count(),
        (60, 150),
        &[],
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
        &[],
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

/// `score-sum` must actually order differently from `lex` somewhere.
///
/// The two modes are compared line for line by the lattice sweep, but under
/// `hypgen_scoring = "most-constrained"` every score is `0.0` and `score-sum`
/// *is* `lex` — and a KB with no `(config …)` block takes that path (53 of the
/// corpus's files do). If no file differentiated them, the sweep would be
/// comparing `lex` twice and the mode's parity would be untested.
#[test]
fn score_sum_orders_differently_from_lex_somewhere_in_the_corpus() {
    let mut differing = 0;
    for path in &corpus_files() {
        let Some(Answer::Ok(text)) = rust_lattice(path) else {
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
