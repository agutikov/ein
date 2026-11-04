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
//! `EIN_CORPUS_SLOW=1` adds the programs route 1 describes only past the
//! default budget — the zebra family and the six `square-*` cubes — at
//! `--region-budget` 2²⁵, where both finish. `syntax/arg-kinds` was one until
//! M1e2 S1e2.2.14: its R is the whole cube of 41 candidates since, 2⁴¹
//! commitments, which no budget of 2²⁵ facts describes, so it is skipped.
//!
//! ```text
//! cargo test -p ein-cli --test route_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release --test route_invariance
//! ```

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// Every corpus file and register witness, once. A debug build left out W7's
/// two witnesses, which it aborted on, until M1e2 S1e2.2.7 fixed W7.
fn programs() -> Vec<PathBuf> {
    let mut files = ein_corpus::corpus_files();
    for row in register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files
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
/// never does more**, on every monotone program both finish.
#[test]
fn the_state_route_and_route_1_print_one_read_out() {
    let files = programs();
    let budgets: Vec<u64> = if slow_too() {
        vec![ein_infer::downset::DEFAULT_BUDGET, 1 << 25]
    } else {
        vec![ein_infer::downset::DEFAULT_BUDGET]
    };
    let next = AtomicUsize::new(0);
    let bad: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let compared = AtomicUsize::new(0);
    let workers = std::thread::available_parallelism().map_or(1, |n| n.get().min(8));
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let rel = path
                        .strip_prefix(repo_root())
                        .unwrap_or(path)
                        .display()
                        .to_string();
                    for &budget in &budgets {
                        let Some(one) = printed(path, Route::DownSet, budget) else {
                            break;
                        };
                        if !one.monotone {
                            break;
                        }
                        if !one.exhausted {
                            continue;
                        }
                        let Some(states) = printed(path, Route::States, budget) else {
                            bad.lock().unwrap().push(format!(
                                "{rel}: route 1 describes R and the state route does not run"
                            ));
                            break;
                        };
                        compared.fetch_add(1, Ordering::Relaxed);
                        let mut b = bad.lock().unwrap();
                        if !states.exhausted {
                            b.push(format!(
                                "{rel}: route 1 exhausts R at --region-budget {budget} and the \
                                 state route does not"
                            ));
                        } else if one.text != states.text {
                            b.push(format!(
                                "{rel}: {}",
                                first_difference(&one.text, &states.text)
                            ));
                        }
                        if states.saturations > one.saturations || states.facts > one.facts {
                            b.push(format!(
                                "{rel}: the state route did more, {} saturations and {} facts \
                                 against route 1's {} and {}",
                                states.saturations, states.facts, one.saturations, one.facts
                            ));
                        }
                        break;
                    }
                }
            });
        }
    });
    let bad = bad.into_inner().unwrap();
    let compared = compared.into_inner();
    eprintln!("{compared} programs compared");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // 2026-11-03: 171 monotone programs route 1 describes at the default
    // budget. A sweep that compared none would pass the loop above.
    assert!(compared >= 160, "compared {compared} programs");
}
