//! **Two routes, one read-out**, in the form of C4
//! ([`05-cli.md` § 1](../../../../docs/ein/user/04-reference/05-cli.md#1-the-contract)).
//!
//! On a monotone program describe takes the state route
//! ([`ein_infer::states`]), with `--describe-route states` or where the tree
//! and the classes have nothing to branch on, and route 1
//! ([`ein_infer::downset`]) answers no describe there. The rule that picks
//! route 1 reads the static verdict and nothing else
//! (`explore`'s § *Describe's route*), so no option can make the two meet on
//! one program. This file makes them meet, through
//! [`explore::describe_by`], and holds two things on every monotone corpus
//! program and regression witness where **both** describe all of `R` at the
//! default budget:
//!
//! - **one read-out**: what `ein solve` prints, uncut, with its stderr lines
//!   and exit code, and its `--json-summary`, are identical apart from the
//!   `route` and `work` rows and the summary's `route`, `licences` and `work`
//!   — no two commands disagree (Q-M1e2.32);
//! - **the premise of the rule**: the state route saturates no more nodes and
//!   derives no more facts than route 1. The day a program breaks it, the rule
//!   is revisited, which is what lets the rule rest on a measurement
//!   (Q-M1e.2).
//!
//! **Where route 1 is cut and the state route is not**, at the default
//! budget, the read-outs are not compared and the premise is held by the
//! cut: the state route finished in no more saturations and facts than route
//! 1 spent before it stopped, so no more than route 1 would have spent to
//! finish. The test names each such program on stderr with its counts:
//! the zebra family and the regressions' `count-past-2-64.ein` and
//! `count-past-2-128.ein` among them (`EIN_CORPUS_SLOW=1 cargo test -p
//! ein-cli --profile gate --test route_invariance -- --nocapture` lists
//! them). Where neither route finishes nothing is read, and the test names
//! those too: the `square-bwd` and `square-fwd` programs,
//! `send_more_money.ein` and `zebra2-minus-15.ein` with its `-obligations`
//! twin among them.
//!
//! **No run here passes the default budget**, as no test does (Q-M1e2.35):
//! a program route 1 describes only past it is not compared.
//!
//! **By default, every program but the manifest's `slow` entries**: the zebra
//! family and `send_more_money` would be most of the time.
//! `EIN_CORPUS_SLOW=1` reads them too, by name.
//!
//! ```text
//! cargo test -p ein-cli --profile gate --test route_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --profile gate --test route_invariance -- the_state_route_and_route_1_print_one_read_out
//! ```

use std::path::{Path, PathBuf};

use ein_corpus::{regressions, repo_root};
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

/// Every corpus file and regression witness, once, but the manifest's `slow`
/// entries unless `slow`; this run's shard of them, the `binding-key-*` and
/// `activator-text-*` witnesses among them in a debug build too.
fn programs(slow_too: bool) -> Vec<PathBuf> {
    let root = repo_root();
    let slow: std::collections::BTreeSet<PathBuf> = ein_corpus::corpus()
        .entry
        .iter()
        .filter(|e| e.slow)
        .map(|e| root.join(&e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in regressions().case {
        files.push(root.join(&row.witness));
    }
    files.sort();
    files.dedup();
    files.retain(|f| slow_too || !slow.contains(f));
    ein_corpus::shard::keep(files, |f| {
        f.strip_prefix(&root).unwrap_or(f).display().to_string()
    })
}

/// **The programs on which the state route counts more facts than route 1**,
/// exactly: a state is a closure, and a candidate another one implies is in
/// it, so the state route's dead extension commits it where route 1's fork
/// from root, stopped at its first contradiction, has not derived it yet. In
/// `13_mixed_solution_and_open.ein` `(choice Ann)` implies `(likes Ann Soup)`,
/// a candidate beside it under the union (Q-M1f.570). The saturations are no
/// more on these, and the read-out is one.
const COMMITS_AN_IMPLIED_CANDIDATE: [&str; 1] = ["examples/features/13_mixed_solution_and_open.ein"];

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
    // all of the target's slow half.
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
    // A floor: a sweep that compared none would pass the loop above.
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
/// Each (program, route) is a unit of its own on the pool, so a program's
/// two routes run side by side; the fold reads them
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
    // On the corpus sweeps' pool, read back in the units' order, not in the
    // order the failures finished.
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
        let more_facts = states.facts > one.facts;
        let named = COMMITS_AN_IMPLIED_CANDIDATE.contains(&rel.as_str());
        if states.saturations > one.saturations || (more_facts && !named) {
            out.bad.push(format!(
                "{rel}: the state route did more, {} saturations and {} facts \
                 against route 1's {} and {}",
                states.saturations, states.facts, one.saturations, one.facts
            ));
        }
        if named && !more_facts {
            out.bad.push(format!(
                "{rel}: COMMITS_AN_IMPLIED_CANDIDATE names it, and the state route did no more \
                 facts than route 1: drop it from the list"
            ));
        }
    }
    out
}
