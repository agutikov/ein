//! M1e S1e.3.4 — **one count per answer**, on every surface that prints one.
//!
//! [`AR-M2`] is that `Verdict` is constructed once, in `finalise`, and *read
//! out* by three crates that each chose their own number. The review found two
//! wrong choices; a third was found while fixing them, and it is the one worth
//! keeping in front of a reader:
//!
//! ```text
//! $ ein solve tests/stdlib/slots/03_fill.ein --stats
//!   solutions (k)   0        ← the table: models, and an open state is not one
//!   ...
//! stats
//!   solutions (k)    1       ← --stats: what the search recorded
//! ```
//!
//! Same label, two numbers, one screen, on a shipped corpus entry. Neither
//! number was wrong; the label was, and no test could see it because every
//! test that read one of the two read only one.
//!
//! So the property here is not *the numbers are right* — the corpus goldens
//! say that. It is **one label, one number, per invocation**, plus the number
//! printed being the summary's own. Since M1e2 S1e2.1a.5 the numbers are R's:
//! `states (k_s)` and `commitments (k_c)` under a description, each qualified
//! when the run did not exhaust, and the verdict word under `--single` alone —
//! which is the walk's `solutions (k)` / `solution_nodes` collision made
//! impossible by construction, since a count of states and a count of
//! recorded nodes are no longer both printed. The cells below are every
//! empty kind, a partial run, and each word.
//!
//! [`AR-M2`]: `docs/history/m1e_review_processing/README.md#s1e34--architecture-medium`

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    out: String,
    summary: serde_json::Value,
}

/// One `ein solve`, with `--stats` and a summary beside it — the two surfaces
/// whose labels collided, plus the machine copy that says what the answer was.
fn solve(args: &[&str]) -> Run {
    // A directory per run, not a path derived from the argv: `cargo test` runs
    // a file's tests as threads of one process, and two of the three below
    // solve the same cells.
    let dir = Scratch::new("ein-read-out");
    let json = dir.join("summary.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .args(args)
        .args(["--stats", "--json-summary"])
        .arg(&json)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let text = std::fs::read_to_string(&json).expect("a summary was written");
    Run {
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        summary: serde_json::from_str(&text).expect("the summary parses"),
    }
}

/// Every `  <label>   <value>` row carrying `label`, value trimmed.
fn rows(run: &Run, label: &str) -> Vec<String> {
    run.out
        .lines()
        .filter(|l| l.trim_start().starts_with(label))
        .map(|l| l.trim_start()[label.len()..].trim().to_string())
        .collect()
}

/// `(file, extra flags)` — the cells: one state, several, a partial run, the
/// two empty kinds, and a root that owes beside a state that does not.
const CELLS: &[(&str, &[&str])] = &[
    ("examples/branching/01_saturate_only.ein", &[]),
    ("examples/branching/04_two_levels.ein", &[]),
    (
        "examples/saturation/type-exclusivity/colors.ein",
        &["--region-budget", "50"],
    ),
    ("examples/ein-bugs/zebra2-bad.ein", &[]),
    ("tests/stdlib/slots/03_fill.ein", &[]),
    ("examples/features/13_mixed_solution_and_open.ein", &[]),
];

/// **Each count is printed once and is the summary's**, with the qualifier
/// exactly when the run did not exhaust — under describe and under `--count`
/// (M1e2 T1e2.1c.4.6), whose two counts are describe's.
#[test]
fn each_count_is_printed_once_and_is_the_summarys() {
    for ((file, flags), count) in CELLS.iter().flat_map(|c| [(c, false), (c, true)]) {
        let mut argv = vec![*file];
        argv.extend_from_slice(flags);
        if count {
            argv.push("--count");
        }
        let run = solve(&argv);
        let where_ = format!("{file} {}", argv[1..].join(" "));
        if count {
            let described = solve(&argv[..argv.len() - 1]);
            for key in ["k_s", "k_c", "exhausted"] {
                assert_eq!(
                    run.summary["region"][key], described.summary["region"][key],
                    "{where_}: --count's {key} is not describe's"
                );
            }
            assert_eq!(run.summary["asked"]["ask"], "count", "{where_}");
            assert!(run.summary["region"]["states"].is_null(), "{where_}");
        }
        let region = &run.summary["region"];
        let exhausted = region["exhausted"] == true;
        for (label, key) in [("states (k_s)", "k_s"), ("commitments (k_c)", "k_c")] {
            let printed = rows(&run, label);
            assert_eq!(printed.len(), 1, "{where_}: `{label}` printed {printed:?}");
            let n = region[key].as_u64().expect("a count");
            let mut words = printed[0].split("   ");
            let value = words.next().unwrap().replace(' ', "");
            assert_eq!(value, n.to_string(), "{where_}: {label}");
            assert_eq!(
                words
                    .next()
                    .is_some_and(|q| q.starts_with("(a lower bound")),
                !exhausted,
                "{where_}: {label} is qualified exactly when the run did not exhaust"
            );
        }
        assert!(
            rows(&run, "solutions (k)").is_empty(),
            "{where_}: the walk's label"
        );
        assert!(
            rows(&run, "verdict").is_empty(),
            "{where_}: a word without --single"
        );
    }
}

/// **The word is printed once, under `--single`, and is the summary's.**
#[test]
fn the_word_is_printed_once_and_is_the_summarys() {
    for (file, word) in [
        ("examples/branching/01_saturate_only.ein", "Solution"),
        ("examples/branching/04_two_levels.ein", "Ambiguity"),
        ("examples/ein-bugs/zebra2-bad.ein", "Contradiction"),
        ("tests/stdlib/slots/03_fill.ein", "Open"),
    ] {
        let run = solve(&[file, "--single"]);
        assert_eq!(run.summary["verdict"]["word"], word, "{file}");
        let printed = rows(&run, "verdict");
        assert_eq!(printed.len(), 1, "{file}: {printed:?}");
        assert!(
            printed[0].starts_with(&format!("{word} — ")),
            "{file}: {printed:?}"
        );
    }
}

/// **The mixed regime** — a root that owes beside one state that does not.
///
/// `examples/features/13_mixed_solution_and_open.ein` is the fixture `CO-M2`
/// asked for: the walk recorded two complete nodes, one discharged and one
/// open, and the count beside the word had to be the models' and not the
/// recorded nodes'. Under R the open node is ∅, which owes, so it is not a
/// state of R: R is one state, `∅ not in R` says why the other is not, and
/// `--single` answers `Solution`.
#[test]
fn an_owing_node_is_not_a_state() {
    let run = solve(&["examples/features/13_mixed_solution_and_open.ein"]);
    assert_eq!(run.summary["region"]["k_s"], 1, "one state");
    assert_eq!(run.summary["region"]["root"], "owing");
    assert_eq!(run.summary["region"]["root_owes"], 1);
    assert_eq!(rows(&run, "∅"), vec!["not in R — root owes 1 instance"]);
    let single = solve(&[
        "examples/features/13_mixed_solution_and_open.ein",
        "--single",
    ]);
    assert_eq!(single.summary["verdict"]["word"], "Solution");
}

/// **An empty R of the owing kind** reports zero states, says which kind of
/// empty it is, and lists what is owed — where the walk printed
/// `solutions (k) 0` beside `open states 1` and a third count, `solution_nodes
/// 1`, under `--stats`.
#[test]
fn an_owing_empty_r_reports_zero_states_and_what_is_owed() {
    let run = solve(&["tests/stdlib/slots/03_fill.ein"]);
    assert_eq!(run.summary["region"]["k_s"], 0);
    assert_eq!(run.summary["region"]["empty"], "owing");
    assert_eq!(rows(&run, "states (k_s)"), vec!["0"]);
    assert_eq!(
        rows(&run, "empty"),
        vec!["every alive node owes — no commitment pays every debt"]
    );
    assert_eq!(rows(&run, "owing"), vec!["∅ — owes 10"]);
    assert!(rows(&run, "open states").is_empty() && rows(&run, "solution_nodes").is_empty());
}

/// **`--count` prints describe's rows and reads no state**, on every corpus
/// program and register witness — M1e2 T1e2.1c.4.6, the user's ruling of
/// 2026-11-03 that a count is describe's question with the listing left out.
///
/// In-process, at the corpus sweep's small budget and cube limit (2¹⁴ facts,
/// 2¹⁰ nodes, `corpus_shapes.rs`'s), so that partial runs of every kind are
/// among what is compared and the whole sweep is cheap. Each program is asked
/// both questions, and:
///
/// - **every row `--count` prints is describe's**, byte for byte, but `asked`
///   and the warning's `paths` row, which says the spot check was not run;
///   so are its `note:` lines for a partial run;
/// - **it reads no state**: no state is listed or kept, no query has a
///   class, and every claim the program states is `NOT CHECKED`, the same
///   number of claims describe judges;
/// - **it exits 0**, and refuses a program exactly where describe does, with
///   the same error.
///
/// Its counts against the oracle are `ein-infer/tests/explore.rs`'s, and at
/// the default budget under every lever `lever_invariance.rs`'s.
#[test]
fn count_prints_describes_rows_and_reads_no_state() {
    use ein_infer::expect::Outcome;
    use ein_infer::explore::{self, Ask, Mode, Which};
    use ein_infer::read_out::Listing;

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
    let ask = |path: &std::path::Path, ask: Ask| {
        let text = std::fs::read_to_string(path).ok()?;
        let mut ast = ein_ir::Ast::new();
        let mut terms = ein_core::Terms::new();
        let forms = ein_ir::parse(&mut ast, &text, path.to_str()).ok()?;
        let mut kb = ein_ir::load(&mut ast, &mut terms, &forms, path.parent()).ok()?;
        let mut opts = explore::Options::new(Mode {
            which: Which::All,
            ask,
        });
        opts.budget = 1 << 14;
        opts.limit = 10;
        let file = path.to_string_lossy().to_string();
        Some(
            ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts).map(|r| {
                let t = ein_render::read_out::render(
                    &r,
                    &ein_render::read_out::Options {
                        command: "solve",
                        argv: &file,
                        file: &file,
                        budget: opts.budget,
                        uncut: false,
                    },
                );
                (r, t)
            }),
        )
    };
    let jobs = std::thread::available_parallelism().map_or(1, |n| n.get().min(8));
    let next = std::sync::atomic::AtomicUsize::new(0);
    let tally = std::sync::Mutex::new((Vec::<String>::new(), 0usize, 0usize, 0usize));
    std::thread::scope(|scope| {
        for _ in 0..jobs {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(path) = files.get(i) else { break };
                    let rel = path
                        .strip_prefix(repo_root())
                        .unwrap_or(path)
                        .display()
                        .to_string();
                    let (Some(d), Some(c)) = (ask(path, Ask::Describe), ask(path, Ask::Count))
                    else {
                        continue;
                    };
                    let mut bad: Vec<String> = Vec::new();
                    let (mut compared, mut partial, mut claims) = (0, 0, 0);
                    match (d, c) {
                        (Err(x), Err(y)) if x.to_string() == y.to_string() => {}
                        (Err(x), y) => bad.push(format!(
                            "describe refuses ({x}), --count: {:?}",
                            y.err().map(|e| e.to_string())
                        )),
                        (Ok(_), Err(y)) => {
                            bad.push(format!("--count refuses ({y}), describe does not"))
                        }
                        (Ok((dr, dt)), Ok((cr, ct))) => {
                            compared = 1;
                            partial = usize::from(!cr.answer.exhausted);
                            let shown: std::collections::BTreeSet<&str> =
                                dt.stdout.lines().collect();
                            for l in ct.stdout.lines().skip(3) {
                                if l.is_empty() {
                                    break;
                                }
                                if !l.starts_with("    paths ") && !shown.contains(l) {
                                    bad.push(format!("a row describe does not print: {l}"));
                                }
                            }
                            for label in [
                                "states (k_s)",
                                "commitments (k_c)",
                                "exhausted",
                                "route",
                                "work",
                            ] {
                                let row = |t: &str| {
                                    t.lines()
                                        .find(|l| l.trim_start().starts_with(label))
                                        .map(str::to_string)
                                };
                                if row(&ct.stdout).is_none() || row(&ct.stdout) != row(&dt.stdout) {
                                    bad.push(format!("the `{label}` row is not describe's"));
                                }
                            }
                            let notes = |t: &ein_render::read_out::Text| -> Vec<String> {
                                t.stderr
                                    .iter()
                                    .filter(|l| l.contains(": partial — "))
                                    .cloned()
                                    .collect()
                            };
                            if notes(&ct) != notes(&dt) {
                                bad.push("the partial note is not describe's".to_string());
                            }
                            if !cr.states.is_empty()
                                || cr.listing != Listing::Counted
                                || cr.queries.iter().any(|q| !q.classes.is_empty())
                            {
                                bad.push("a state was kept, listed or answered from".to_string());
                            }
                            let judged = |r: &ein_infer::read_out::ReadOut| {
                                r.queries
                                    .iter()
                                    .filter_map(|q| q.claim.as_ref())
                                    .chain(r.claim.as_ref())
                                    .map(|c| c.outcome)
                                    .collect::<Vec<_>>()
                            };
                            claims = judged(&cr).len();
                            if claims != judged(&dr).len()
                                || judged(&cr).iter().any(|o| *o != Outcome::NotChecked)
                            {
                                bad.push(format!(
                                    "claims {:?} against describe's {:?}",
                                    judged(&cr),
                                    judged(&dr)
                                ));
                            }
                            if ct.code != 0 {
                                bad.push(format!("exit {}", ct.code));
                            }
                        }
                    }
                    let mut t = tally.lock().expect("one writer at a time");
                    t.0.extend(bad.into_iter().map(|b| format!("{rel}: {b}")));
                    t.1 += compared;
                    t.2 += partial;
                    t.3 += claims;
                }
            });
        }
    });
    let (bad, compared, partial, claims) = tally.into_inner().expect("joined");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    eprintln!(
        "{compared} programs compared, {partial} partial at 2^14, {claims} claims not checked"
    );
    // 206 programs, 21 of them partial at 2^14, and 81 claims, on 2026-11-03.
    assert!(
        compared >= 200 && partial >= 15 && claims >= 70,
        "{compared} compared, {partial} partial, {claims} claims"
    );
}

/// **A core holding a nested fact prints it as an s-expression** — M1e2
/// S1e2.3.4's pin of P1e2.3's S8, on the user's ruling (a) of 2026-11-03.
/// `nested-core.ein`'s core is the given `(not (p A))` and `(q A)`; the walk
/// printed the first as `(not Fact(relation_name='p', args=('A',)))`, a
/// struct repr, and nothing checked the spelling once the region read-out
/// replaced it. The line is the read-out's, and the summary's `region.core`
/// carries the same two strings.
#[test]
fn a_nested_fact_in_a_core_prints_as_an_s_expression() {
    let r = solve(&["examples/ein-bugs/nested-core.ein"]);
    assert!(
        r.out
            .lines()
            .any(|l| l == "  unsat core        (not (p A))"),
        "the core's nested fact is not spelled `(not (p A))`:\n{}",
        r.out
    );
    assert_eq!(rows(&r, "unsat core"), ["(not (p A))", "(q A)"]);
    assert_eq!(rows(&r, "refuted by"), ["boom"]);
    assert_eq!(
        r.summary["region"]["core"],
        serde_json::json!(["(not (p A))", "(q A)"])
    );
    assert!(
        !r.out.contains("Fact("),
        "a struct repr is back:\n{}",
        r.out
    );
}
