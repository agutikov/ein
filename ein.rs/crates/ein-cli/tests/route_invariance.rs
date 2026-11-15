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
//! **Where route 1 is cut and the state route is not**, at the default
//! budget, the read-outs are not compared and the premise is held by the
//! cut: the state route finished in no more saturations and facts than route
//! 1 spent before it stopped, so no more than route 1 would have spent to
//! finish. The test names each such program on stderr with its counts:
//! `examples/zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein` and the
//! register's `count-past-2-64.ein` and `count-past-2-128.ein`
//! (`EIN_CORPUS_SLOW=1 cargo test -p ein-cli --profile gate --test
//! route_invariance -- --nocapture`, 2026-11-15). Where neither route
//! finishes nothing is read, and the test names those too: the six
//! `square-bwd` and `square-fwd` programs, `send_more_money.ein`, and
//! `zebra2-minus-15.ein` with its `-obligations` twin, the same day.
//! `syntax/arg-kinds`, skipped so until S1f.6.39 for a cube of 41 candidates
//! no budget described, is compared: its R is {∅} today.
//!
//! **No run here passes the default budget** (Q-M1f.304 ruled (d), the user,
//! 2026-11-15). Until then a second test, the dev host's, compared the
//! programs route 1 describes only past the default at `--region-budget` 2²⁵:
//! `explore::describe_by` at 32 times the 2²⁰ the user ruled on 2026-11-03,
//! after 2²⁴ ran the gate out of memory, on a route no command sends a
//! monotone program.
//!
//! **Per commit, every program but the manifest's `slow` entries** (M1f
//! S1f.6.39, the user's ruling of 2026-11-15 on Q-M1f.291): the zebra family
//! and `send_more_money` were most of the per-commit time.
//! `EIN_CORPUS_SLOW=1` reads them too; the CI nightly runs the test with the
//! flag, by name.
//!
//! ```text
//! cargo test -p ein-cli --profile gate --test route_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --profile gate --test route_invariance -- the_state_route_and_route_1_print_one_read_out
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
/// entries unless `slow`; this run's shard of them. A debug
/// build left out W7's two witnesses, which it aborted on, until M1e2
/// S1e2.2.7 fixed W7, and W16's order pair until M1e2 S1e2.2.16 fixed W16.
fn programs(slow_too: bool) -> Vec<PathBuf> {
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
    files.retain(|f| slow_too || !slow.contains(f));
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
/// `EIN_CORPUS_SLOW=1` every one, the nightly's. Where the state route finishes
/// and route 1 does not, the premise is held by the cut and the read-outs are
/// not compared; those programs are counted and named on stderr.
#[test]
fn the_state_route_and_route_1_print_one_read_out() {
    let budget = ein_infer::downset::DEFAULT_BUDGET;
    // Read here, in the one test, so the nightly's step that names it runs
    // all of the target's slow half (`nightly_steps.rs`).
    let slow = std::env::var_os("EIN_CORPUS_SLOW").is_some();
    let Compared {
        bad,
        compared,
        cut,
        neither,
    } = compare(budget, slow);
    let shard = ein_corpus::shard::current().map_or(String::new(), |s| format!(" [shard {s}]"));
    eprintln!("{compared} programs compared{shard}");
    eprintln!(
        "{} programs the state route describes and route 1 does not at --region-budget \
         {budget}, held by the cut, their read-outs not compared{}",
        cut.len(),
        cut.iter().map(|c| format!("\n  {c}")).collect::<String>()
    );
    eprintln!(
        "{} monotone programs neither route describes at --region-budget {budget}, \
         skipped{}",
        neither.len(),
        neither
            .iter()
            .map(|c| format!("\n  {c}"))
            .collect::<String>()
    );
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // 2026-11-03: 171 monotone programs route 1 describes at the default
    // budget. A sweep that compared none would pass the loop above.
    assert!(
        compared >= ein_corpus::shard::share(160),
        "compared {compared} programs"
    );
}

/// What [`compare`] found: the failures, how many programs it compared, the
/// programs held by the cut alone, each with its counts, and those it skipped.
struct Compared {
    bad: Vec<String>,
    compared: usize,
    cut: Vec<String>,
    neither: Vec<String>,
}

/// The comparison over [`programs`] at one budget, the `slow` entries too if
/// `slow`.
///
/// Each (program, route) is a unit of its own on the pool since M1f
/// S1f.6.39, so a program's two routes run side by side; the fold reads them
/// in the programs' order. A program on which neither route finishes is
/// skipped.
fn compare(budget: u64, slow: bool) -> Compared {
    let files = programs(slow);
    let routes = [Route::DownSet, Route::States];
    let units: Vec<(usize, usize)> = (0..files.len())
        .flat_map(|f| (0..routes.len()).map(move |r| (f, r)))
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
        |&(f, r)| (rel(f), format!("{:?} at {budget}", routes[r])),
        |&(f, r)| printed(&files[f], routes[r], budget),
    )
    .into_iter();
    let mut out = Compared {
        bad: Vec::new(),
        compared: 0,
        cut: Vec::new(),
        neither: Vec::new(),
    };
    for f in 0..files.len() {
        let one = done.next().expect("a unit per route");
        let states = done.next().expect("a unit per route");
        let rel = rel(f);
        let Some(one) = one else {
            continue;
        };
        if !one.monotone {
            continue;
        }
        if !one.exhausted {
            // Route 1 is cut. Where the state route finished, the premise
            // holds by the cut: it describes R within the budget route 1
            // spent without finishing, so it saturated no more nodes and
            // derived no more facts than route 1 would have. Where neither
            // finishes, nothing is read.
            let Some(states) = states.filter(|s| s.exhausted) else {
                out.neither.push(rel);
                continue;
            };
            if states.facts > budget
                || states.facts > one.facts
                || states.saturations > one.saturations
            {
                out.bad.push(format!(
                    "{rel}: route 1 is cut at --region-budget {budget} and the state \
                     route finished past what it spent: {} saturations and {} facts \
                     against route 1's {} and {} at the cut",
                    states.saturations, states.facts, one.saturations, one.facts
                ));
            }
            out.cut.push(format!(
                "{rel}: the state route {} saturations and {} facts, route 1 cut at {} \
                 and {}",
                states.saturations, states.facts, one.saturations, one.facts
            ));
            continue;
        }
        let Some(states) = states else {
            out.bad.push(format!(
                "{rel}: route 1 describes R and the state route does not run"
            ));
            continue;
        };
        out.compared += 1;
        if !states.exhausted {
            out.bad.push(format!(
                "{rel}: route 1 exhausts R at --region-budget {budget} and the state \
                 route does not"
            ));
        } else if one.text != states.text {
            out.bad.push(format!(
                "{rel}: {}",
                first_difference(&one.text, &states.text)
            ));
        }
        if states.saturations > one.saturations || states.facts > one.facts {
            out.bad.push(format!(
                "{rel}: the state route did more, {} saturations and {} facts \
                 against route 1's {} and {}",
                states.saturations, states.facts, one.saturations, one.facts
            ));
        }
    }
    out
}
