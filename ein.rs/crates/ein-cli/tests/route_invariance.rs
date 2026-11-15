//! **Two routes, one read-out** — M1e2
//! [S1e2.1c.2](../../../../plans/m1e2_correctness/p1e2.1c_the_region_search/s1e2.1c.2_the_lines_by_duality.md)
//! T1e2.1c.2.4, in the form of C4
//! ([`solve_interface.md` § 1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#1-the-contract)).
//!
//! Since this stage, describe on a monotone program takes the state route
//! ([`ein_infer::states`]) and route 1 ([`ein_infer::downset`]) answers it no
//! more. The rule that picks it reads the static verdict and nothing else
//! (`explore`'s § *Describe's route*), so no option can make the two meet on
//! one program. This file makes them meet, through
//! [`explore::describe_by`], and holds two things on every monotone corpus
//! program and register witness where **both** describe all of `R` at the
//! default budget:
//!
//! - **one read-out**: what `ein solve` prints, uncut, with its stderr lines
//!   and exit code, and its `--json-summary`, are identical apart from the
//!   `route` and `work` rows and the summary's `route`, `licences` and `work`
//!   — the user's *one tool, consistent answers* (2026-10-16);
//! - **the premise of the rule**: the state route saturates no more nodes and
//!   derives no more facts than route 1. The day a program breaks it, the rule
//!   is revisited, which is what lets the rule rest on a measurement
//!   (`standard_of_proof.md`).
//!
//! **Per commit, every program but the manifest's `slow` entries** (M1f
//! S1f.6.39, the user's ruling of 2026-11-15 on Q-M1f.291): the zebra family
//! and `send_more_money` were most of the per-commit half's time.
//! `EIN_CORPUS_SLOW=1`, the slow half, reads them too, and a second test adds
//! the programs route 1 describes only past the default budget — the zebra
//! family and the six `square-*` cubes — at `--region-budget` 2²⁵, where both
//! finish. `syntax/arg-kinds` was one until M1e2 S1e2.2.14: its R is the
//! whole cube of 41 candidates since, 2⁴¹ commitments, which no budget of
//! 2²⁵ facts describes, so it is skipped. Since S1f.6.39 the CI nightly runs
//! the first test with the flag, by name, the `slow` entries at the default
//! budget; the second, the 2²⁵ half, stays the dev host's, as the whole slow
//! half was from M1f S1f.5.27 (Q-M1f.51): one program's route 1 there is
//! past any shard's five minutes (Q-M1f.304).
//!
//! ```text
//! cargo test -p ein-cli --profile gate --test route_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --profile gate --test route_invariance -- the_state_route_and_route_1_print_one_read_out
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --profile gate --test route_invariance   # + 2^25, the dev host's
//! ```

use std::path::{Path, PathBuf};

use ein_corpus::{register, repo_root};
use ein_infer::explore::{self, Ask, Mode, Route, Which};
use ein_ir::{Ast, load, parse};
use serde_json::Value as J;

const DESCRIBE: Mode = Mode {
    which: Which::All,
    ask: Ask::Describe,
};

fn slow_too() -> bool {
    std::env::var_os("EIN_CORPUS_SLOW").is_some()
}

/// What one route printed, and what it cost.
struct Printed {
    /// The read-out uncut, its stderr lines and exit code, then the summary,
    /// less the rows and keys a route may move.
    text: String,
    exhausted: bool,
    monotone: bool,
    saturations: u64,
    facts: u64,
}

fn printed(path: &Path, route: Route, budget: u64) -> Option<Printed> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let forms = parse(&mut ast, &text, path.to_str()).ok()?;
    let mut kb = load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
    let mut opts = explore::Options::new(DESCRIBE);
    opts.budget = budget;
    let a = explore::describe_by(&mut kb, &mut terms, &ast, &opts, route).ok()?;
    let (exhausted, monotone) = (a.exhausted, a.monotonicity.is_monotone());
    let (saturations, facts) = (a.saturations, a.facts);
    let r = ein_infer::read_out::of_answer(&mut kb, &mut terms, &ast, a, &opts).ok()?;
    let file = path.to_string_lossy().to_string();
    let t = ein_render::read_out::render(
        &r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: &file,
            file: &file,
            budget,
            uncut: true,
        },
    );
    let rows: Vec<&str> = t
        .stdout
        .lines()
        .filter(|l| !l.starts_with("  route ") && !l.starts_with("  work "))
        .collect();
    let cfg = kb.program().config.clone().unwrap_or_default();
    let summary = match ein_cli::summary::build_region(&ast, &mut terms, &mut kb, &r, &file, &cfg) {
        Ok(s) => {
            let mut j: J = serde_json::from_str(&ein_cli::summary::text(&s)).expect("parses");
            if let Some(o) = j.as_object_mut() {
                o.remove("work");
                if let Some(region) = o.get_mut("region").and_then(J::as_object_mut) {
                    region.remove("route");
                    region.remove("licences");
                }
            }
            serde_json::to_string_pretty(&j).expect("serialises")
        }
        Err(e) => format!("<summary> {e}"),
    };
    Some(Printed {
        text: format!(
            "{}\n--- stderr\n{}\n--- exit {}\n--- summary\n{summary}\n",
            rows.join("\n"),
            t.stderr.join("\n"),
            t.code
        ),
        exhausted,
        monotone,
        saturations,
        facts,
    })
}

/// Every corpus file and register witness, once, but the manifest's `slow`
/// entries unless `EIN_CORPUS_SLOW` is set; this run's shard of them. A debug
/// build left out W7's two witnesses, which it aborted on, until M1e2
/// S1e2.2.7 fixed W7, and W16's order pair until M1e2 S1e2.2.16 fixed W16.
fn programs() -> Vec<PathBuf> {
    let root = repo_root();
    let slow: std::collections::BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| root.join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in register().case {
        for (_, a) in row.answers() {
            files.push(root.join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too() || !slow.contains(f));
    ein_corpus::shard::keep(files, |f| {
        f.strip_prefix(&root).unwrap_or(f).display().to_string()
    })
}

/// The first line on which two texts differ.
fn first_difference(a: &str, b: &str) -> String {
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return format!("line {}: route 1 `{x}`, the state route `{y}`", i + 1);
        }
    }
    format!("{} lines against {}", a.lines().count(), b.lines().count())
}

/// **The state route and route 1 print one read-out, and the state route
/// never does more**, on every monotone program both finish at the default
/// budget: per commit every program but the `slow` entries, under
/// `EIN_CORPUS_SLOW=1` every one, the nightly's.
#[test]
fn the_state_route_and_route_1_print_one_read_out() {
    let (bad, compared) = compare(&[ein_infer::downset::DEFAULT_BUDGET]);
    eprintln!(
        "{compared} programs compared{}",
        ein_corpus::shard::current().map_or(String::new(), |s| format!(" [shard {s}]"))
    );
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // 2026-11-03: 171 monotone programs route 1 describes at the default
    // budget. A sweep that compared none would pass the loop above.
    assert!(
        compared >= ein_corpus::shard::share(160),
        "compared {compared} programs"
    );
}

/// **The same where route 1 finishes only past the default budget**, at
/// `--region-budget` 2²⁵: the zebra family and the six `square-*` cubes.
/// `EIN_CORPUS_SLOW=1` only, and the dev host's: route 1 alone took 495 s at
/// `-j 16` to reach a 2²² ceiling on `zebra2-minus-15`
/// (`alive_set_reach.md` § 2.3, 2026-11-03), where this asks 2²⁵ of one
/// unit, and the whole slow half had not finished after 29 minutes on four
/// threads (M1f S1f.6.39, 2026-11-15); no shard by program fits the
/// nightly's five minutes (Q-M1f.304). The nightly runs the test above, by
/// name.
#[test]
fn route_1_and_the_state_route_agree_past_the_default_budget() {
    if !slow_too() {
        return;
    }
    let (bad, compared) = compare(&[ein_infer::downset::DEFAULT_BUDGET, 1 << 25]);
    eprintln!("{compared} programs compared at the first budget route 1 exhausts");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(
        compared >= ein_corpus::shard::share(160),
        "compared {compared} programs"
    );
}

/// The comparison over [`programs`], at each budget in turn until route 1
/// exhausts: the failures, and how many programs were compared.
///
/// Each (program, budget, route) is a unit of its own on the pool since M1f
/// S1f.6.39, so a program's two routes run side by side; the fold reads
/// them as the loop did, budget by budget, and compares at the first budget
/// route 1 exhausts. A unit no budget reaches is computed and not read.
fn compare(budgets: &[u64]) -> (Vec<String>, usize) {
    let files = programs();
    let routes = [Route::DownSet, Route::States];
    let units: Vec<(usize, usize, usize)> = (0..files.len())
        .flat_map(|f| {
            (0..budgets.len()).flat_map(move |b| (0..routes.len()).map(move |r| (f, b, r)))
        })
        .collect();
    let rel = |f: usize| {
        files[f]
            .strip_prefix(repo_root())
            .unwrap_or(&files[f])
            .display()
            .to_string()
    };
    // On the corpus sweeps' pool (M1f S1f.6.39; eight threads of its own
    // until then, its failures in the order they finished), read back in the
    // units' order.
    let mut done = ein_corpus::pool::map(
        "route_invariance",
        &units,
        |&(f, b, r)| (rel(f), format!("{:?} at {}", routes[r], budgets[b])),
        |&(f, b, r)| printed(&files[f], routes[r], budgets[b]),
    )
    .into_iter();
    let mut bad: Vec<String> = Vec::new();
    let mut compared = 0usize;
    for f in 0..files.len() {
        let runs: Vec<(Option<Printed>, Option<Printed>)> = (0..budgets.len())
            .map(|_| {
                let one = done.next().expect("a unit per route");
                let states = done.next().expect("a unit per route");
                (one, states)
            })
            .collect();
        let rel = rel(f);
        for (b, (one, states)) in runs.into_iter().enumerate() {
            let budget = budgets[b];
            let Some(one) = one else {
                break;
            };
            if !one.monotone {
                break;
            }
            if !one.exhausted {
                continue;
            }
            let Some(states) = states else {
                bad.push(format!(
                    "{rel}: route 1 describes R and the state route does not run"
                ));
                break;
            };
            compared += 1;
            if !states.exhausted {
                bad.push(format!(
                    "{rel}: route 1 exhausts R at --region-budget {budget} and the \
                     state route does not"
                ));
            } else if one.text != states.text {
                bad.push(format!(
                    "{rel}: {}",
                    first_difference(&one.text, &states.text)
                ));
            }
            if states.saturations > one.saturations || states.facts > one.facts {
                bad.push(format!(
                    "{rel}: the state route did more, {} saturations and {} facts \
                     against route 1's {} and {}",
                    states.saturations, states.facts, one.saturations, one.facts
                ));
            }
            break;
        }
    }
    (bad, compared)
}
