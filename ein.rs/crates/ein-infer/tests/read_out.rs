//! The read-out's data against the oracle — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md)'s
//! third step.
//!
//! [`read_out::run`] reads every state it lists through the node function,
//! answers every query from one solve, and checks the program's claim against
//! the R it described. Held here, on every corpus file and register witness
//! whose cube the oracle enumerates:
//!
//! - **each state's `(model …)` is the oracle's state**: the positive facts of
//!   the candidates' relations in the oracle's own fixpoint of that node;
//! - **the query classes partition the states they are over**, each class
//!   once, sorted;
//! - **the claim checked against this run's R says what a check that computes
//!   R again says** (`claim::check`).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ein_core::{Kb, Terms};
use ein_corpus::repo_root;
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_infer::read_out::{self, Listing};
use ein_infer::region::{self, Outcome, Region};
use ein_ir::{Ast, load, parse};

fn loaded(path: &Path) -> Option<(Kb, Terms, Ast)> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    Some((kb, terms, ast))
}

fn corpus_and_register() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in ein_corpus::register().case {
        for (_, a) in row.answers() {
            if cfg!(debug_assertions) && a.debug_panic.is_some() {
                continue;
            }
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
}

fn mask(set: &[u32]) -> u64 {
    set.iter().fold(0, |m, &b| m | 1 << b)
}

fn relation_of(fact: &str) -> &str {
    fact.trim_start_matches('(')
        .split(|c: char| c.is_whitespace() || c == ')')
        .next()
        .unwrap_or("")
}

/// The oracle's model of a node: root's facts and the node's own beyond
/// them, positive, over the candidates' relations, sorted.
fn oracle_model(r: &Region, root: &BTreeSet<String>, m: u64) -> Vec<String> {
    let named: BTreeSet<&str> = r.candidates.iter().map(|c| relation_of(&c.text)).collect();
    let beyond = &r.states.as_ref().expect("kept")[m as usize];
    let mut v: Vec<String> = root
        .iter()
        .chain(beyond.iter())
        .filter(|t| !t.starts_with("(not ") && named.contains(relation_of(t)))
        .cloned()
        .collect();
    v.sort();
    v.dedup();
    v
}

#[test]
fn every_state_is_the_oracles_and_every_claim_is_checked_once() {
    let mut failures: Vec<String> = Vec::new();
    let mut models = 0usize;
    let mut claims = 0usize;
    let mut classes = 0usize;
    let oracle = region::Options {
        keep_states: true,
        ..region::Options::default()
    };
    for path in corpus_and_register() {
        let rel = path
            .strip_prefix(repo_root())
            .unwrap_or(&path)
            .to_string_lossy()
            .to_string();
        let Some((mut kb, mut terms, ast)) = loaded(&path) else {
            continue;
        };
        let Ok(Outcome::Region(r)) =
            region::compute(&mut kb, &mut terms, &ast, &oracle, &mut |_| {})
        else {
            continue;
        };
        let root: BTreeSet<String> = kb
            .facts()
            .map(|f| ein_infer::events::sexpr(&terms, f))
            .collect();
        for which in [Which::All, Which::Maximal] {
            let mut opts = explore::Options::new(Mode {
                which,
                ask: Ask::Describe,
            });
            opts.budget = u64::MAX;
            let (mut kb, mut terms, ast) = loaded(&path).expect("loaded once");
            let out = read_out::run(&mut kb, &mut terms, &ast, &opts)
                .unwrap_or_else(|e| panic!("{rel}: {e}"));
            let tag = format!("{rel} {which:?}");
            for s in &out.states {
                let m = mask(&s.rep);
                if !r.in_r(m) {
                    failures.push(format!("{tag}: a state is not in R"));
                    continue;
                }
                let want = oracle_model(&r, &root, m);
                if s.model != want {
                    failures.push(format!(
                        "{tag}: the model of {:?} is {:?}, the oracle's {:?}",
                        s.rep, s.model, want
                    ));
                }
                models += 1;
            }
            if let Listing::Owing(frontier) = &out.listing {
                for f in frontier {
                    let want = oracle_model(&r, &root, mask(&f.node.set));
                    if f.model != want {
                        failures.push(format!("{tag}: an owing model differs from the oracle's"));
                    }
                }
            }
            for q in &out.queries {
                let mut seen: Vec<usize> =
                    q.classes.iter().flat_map(|c| c.states.clone()).collect();
                seen.sort_unstable();
                if seen != (0..out.states.len()).collect::<Vec<_>>() {
                    failures.push(format!(
                        "{tag}: {}'s classes do not partition the states",
                        q.goal
                    ));
                }
                let keys: BTreeSet<&Vec<String>> = q.classes.iter().map(|c| &c.facts).collect();
                if keys.len() != q.classes.len() {
                    failures.push(format!("{tag}: {} has a class twice", q.goal));
                }
                classes += q.classes.len();
            }
            if which == Which::All
                && let Some(checked) = &out.claim
            {
                let (mut kb, mut terms, ast) = loaded(&path).expect("loaded once");
                let solve = kb.program().solve().cloned().expect("a claim has a solve");
                let node = ein_infer::solve_value(&ast, &solve, "expect").expect("an :expect");
                let claim = ein_ir::claim::parse(&ast, node).expect("loaded");
                let again = ein_infer::claim::check(
                    &mut kb,
                    &mut terms,
                    &ast,
                    &claim,
                    &ein_infer::claim::Options {
                        budget: u64::MAX,
                        ..Default::default()
                    },
                )
                .expect("checks");
                if again.report.outcome != checked.outcome {
                    failures.push(format!(
                        "{tag}: the claim is {:?} against this run's R and {:?} against R \
                         computed again",
                        checked.outcome, again.report.outcome
                    ));
                }
                claims += 1;
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // Not vacuous: 17 130 models, 7 claims and 359 classes on 2026-10-16.
    assert!(
        models >= 15_000 && claims >= 7 && classes >= 300,
        "{models} {claims} {classes}"
    );
}
