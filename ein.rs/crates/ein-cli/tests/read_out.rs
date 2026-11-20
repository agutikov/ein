//! **One count per answer**, on every surface that prints one.
//!
//! An answer read out by several crates, each choosing its own number, can
//! print one label with two numbers on one screen — a count of models in the
//! table and a count of recorded nodes under `--stats`, both called
//! `solutions (k)` — and no test sees it if every test reads only one of the
//! two.
//!
//! So the property here is not *the numbers are right* — the corpus goldens
//! say that. It is **one label, one number, per invocation**, plus the number
//! printed being the summary's own. The numbers are R's: `states (k_s)` and
//! `commitments (k_c)` under a description, each qualified when the run did
//! not exhaust, and the verdict word under `--single` alone, so a count of
//! states and a count of recorded nodes are never both printed. The cells
//! below are every empty kind, a partial run, and each word.

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
    ("tests/region/04_owing.ein", &[]),
    ("examples/features/13_mixed_solution_and_open.ein", &[]),
];

/// **Each count is printed once and is the summary's**, with the qualifier
/// exactly when the run did not exhaust — under describe and under `--count`
/// (Q-M1f.218), whose two counts are describe's.
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
        ("tests/region/04_owing.ein", "Open"),
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
/// `examples/features/13_mixed_solution_and_open.ein` has owing nodes beside
/// discharged ones, and the count is the states' and not the nodes'. ∅ and
/// `{(choice Bob)}` owe, so neither is a state of R: R is three states, the
/// hypothesis rule's `(choice Ann)` and what the obligation proposes beside it
/// (Q-M1f.570), `∅ not in R` says why root is not, and `--single` answers
/// `Ambiguity`.
#[test]
fn an_owing_node_is_not_a_state() {
    let run = solve(&["examples/features/13_mixed_solution_and_open.ein"]);
    assert_eq!(run.summary["region"]["k_s"], 3, "three states");
    assert_eq!(run.summary["region"]["root"], "owing");
    assert_eq!(run.summary["region"]["root_owes"], 1);
    assert_eq!(rows(&run, "∅"), vec!["not in R — root owes 1 instance"]);
    // And what it owes is named under the row, the summary's own string.
    assert_eq!(rows(&run, "owed"), vec!["likes owes Ann a Food."]);
    assert_eq!(
        run.summary["region"]["root_owed"],
        serde_json::json!(["likes owes Ann a Food."])
    );
    let single = solve(&[
        "examples/features/13_mixed_solution_and_open.ein",
        "--single",
    ]);
    assert_eq!(single.summary["verdict"]["word"], "Ambiguity");
}

/// **An empty R of the owing kind** reports zero states, says which kind of
/// empty it is, and lists what is owed — never `solutions (k) 0` beside
/// `open states 1` and a third count under `--stats`.
#[test]
fn an_owing_empty_r_reports_zero_states_and_what_is_owed() {
    // Not `tests/stdlib/slots/03_fill.ein`: a `:no-hypothesis` that names a
    // relation an obligation owes is refused at load (Q-M1f.245).
    let run = solve(&["tests/region/04_owing.ein"]);
    assert_eq!(run.summary["region"]["k_s"], 0);
    assert_eq!(run.summary["region"]["empty"], "owing");
    assert_eq!(rows(&run, "states (k_s)"), vec!["0"]);
    assert_eq!(
        rows(&run, "empty"),
        vec!["every alive node owes — no commitment pays every debt"]
    );
    assert_eq!(rows(&run, "owing"), vec!["∅ — owes 1"]);
    assert!(rows(&run, "open states").is_empty() && rows(&run, "solution_nodes").is_empty());
}

/// **The read-out names what root owes, beside the states that pay it.**
/// Where root is alive, owes and R is not empty, the `∅` row counts the
/// instances and the `owed` rows under it name them, each through its rule's
/// `:why` as an empty R's `owing` rows do, on every surface that repeats the
/// read-out's rows, not on `--events`' `owe` lines alone.
///
/// The program: 120 things each owe a mark, and one supposition marks them
/// all, so R is one state and root owes 120 instances.
///
/// - **stdout**: four `owed` rows, in the order the obligation reports its
///   instances, and one line for the other 116 that names `--dump-answer
///   DIR`; they do not take the listings' 100 lines, so R is still printed as
///   its `(region …)`;
/// - **`--dump-answer`'s `answer.txt`**: all 120, and no such line;
/// - **`--trace`**: its headline is stdout's rows;
/// - **`--json-summary`**: `region.root_owed` holds the 120, the count row's
///   `region.root_owes` beside it;
/// - **`ein oracle`**: the same four rows, its line naming the command that
///   writes the rest;
/// - **`--count`** prints the `∅` row and no `owed` row, and its summary holds
///   the instances all the same; **a mode** prints neither row, and its
///   summary's `root_owed` is `null`, as `root_owes` is.
///
/// R, the claim and the exit code are what they were: the query's one class
/// is every mark, on the one state. The obligation's witness has no guard, so
/// it proposes no payment at root, and H is the hypothesis rule's one candidate
/// (Q-M1f.570).
#[test]
fn the_read_out_names_what_root_owes() {
    let dir = Scratch::new("ein-read-out-owed");
    let file = dir.join("owes120.ein");
    let things: Vec<String> = (1..=120).map(|i| format!("(thing t{i:03})")).collect();
    std::fs::write(
        &file,
        format!(
            "(relation thing T :why \"{{?3}} is a thing\")\n\
             (relation go :why \"every thing is marked\")\n\
             (relation ink T :why \"{{?3}} is an ink\")\n\
             (relation mark T T :why \"{{?3}} is marked in {{?4}}\")\n\
             {}\n(ink red)\n\
             (rule guess () :match () :assert (open (go)) :why \"suppose every thing is marked\")\n\
             (rule mark-all () :match (and (go) (thing ?x)) :assert (mark ?x red) \
             :why \"{{?x}} is marked, every thing being so\")\n\
             (rule owe-mark () :match (and (thing ?x) (absent (mark ?x ?c))) \
             :assert (open mark) :why \"{{?x}} owes a mark\")\n\
             (query :goal (go) :expect (model (go)))\n",
            things.join(" ")
        ),
    )
    .expect("the program is written");
    let file = file.to_string_lossy().to_string();
    let run = |args: &[&str]| {
        let out = Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(args)
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs");
        assert_eq!(
            out.status.code(),
            Some(0),
            "{args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let owed = |text: &str| -> Vec<String> {
        text.lines()
            .filter_map(|l| l.strip_prefix("    owed            "))
            .map(str::to_string)
            .collect()
    };
    let all: Vec<String> = (1..=120).map(|i| format!("t{i:03} owes a mark")).collect();
    const ROOT: &str = "  ∅                 not in R — root owes 120 instances";
    let rest = |by: &str| {
        format!(
            "                    … 116 more instances: {by} writes every one, to DIR/answer.txt"
        )
    };

    // Stdout: four, and the line; the listing is whole.
    let json = dir.join("s.json");
    let trace = dir.join("t.md");
    let answer = dir.join("answer");
    let stdout = run(&[
        "solve",
        &file,
        "--json-summary",
        &json.to_string_lossy(),
        "--trace",
        &trace.to_string_lossy(),
        "--dump-answer",
        &answer.to_string_lossy(),
    ]);
    let lines: Vec<&str> = stdout.lines().collect();
    let at = lines
        .iter()
        .position(|l| *l == ROOT)
        .unwrap_or_else(|| panic!("no `∅` row:\n{stdout}"));
    assert_eq!(owed(&stdout), all[..4], "{stdout}");
    assert_eq!(
        lines[at + 1..at + 6].join("\n"),
        format!(
            "{}\n{}",
            all[..4]
                .iter()
                .map(|w| format!("    owed            {w}"))
                .collect::<Vec<_>>()
                .join("\n"),
            rest("--dump-answer DIR")
        ),
        "the rows under `∅`:\n{stdout}"
    );
    assert!(lines[at + 6].starts_with("  minimal line "), "{stdout}");
    assert!(
        stdout.contains("\n  (region\n") && !stdout.contains("not shown"),
        "the `owed` rows took the listing's lines:\n{stdout}"
    );
    assert!(stdout.contains("\n  :expect           holds\n"), "{stdout}");

    // `answer.txt`: every one, uncut.
    let whole = std::fs::read_to_string(answer.join("answer.txt")).expect("answer.txt");
    assert_eq!(owed(&whole), all, "{whole}");
    assert!(!whole.contains("more instance"), "{whole}");

    // The trace's headline is stdout's rows.
    let traced = std::fs::read_to_string(&trace).expect("the trace");
    assert_eq!(owed(&traced), all[..4], "{traced}");
    assert!(traced.contains(&rest("--dump-answer DIR")), "{traced}");

    // The summary holds them all, beside the count.
    let summary: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).expect("a summary")).expect("JSON");
    assert_eq!(summary["region"]["root_owes"], 120);
    assert_eq!(summary["region"]["root_owed"], serde_json::json!(all));
    assert_eq!(summary["region"]["k_s"], 1);

    // `ein oracle`: the same rows, and the command that writes the rest.
    let oracle = run(&["oracle", &file]);
    assert_eq!(owed(&oracle), all[..4], "{oracle}");
    assert!(
        oracle.contains(&rest("ein solve FILE --dump-answer DIR")),
        "{oracle}"
    );

    // `--count`: the `∅` row alone; the summary all the same.
    let cj = dir.join("c.json");
    let count = run(&[
        "solve",
        &file,
        "--count",
        "--json-summary",
        &cj.to_string_lossy(),
    ]);
    assert!(count.lines().any(|l| l == ROOT), "{count}");
    assert!(owed(&count).is_empty(), "{count}");
    let cs: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&cj).expect("a summary")).expect("JSON");
    assert_eq!(cs["region"]["root_owed"], serde_json::json!(all));

    // A mode prints no `∅` row, so no `owed` row, and its summary states
    // neither number.
    for mode in [
        &["--single"][..],
        &["--minimal"],
        &["--maximal"],
        &["-n", "2"],
    ] {
        let mj = dir.join("m.json");
        let mut args = vec!["solve", file.as_str()];
        args.extend_from_slice(mode);
        let mj_s = mj.to_string_lossy().to_string();
        args.extend_from_slice(&["--json-summary", &mj_s]);
        let text = run(&args);
        assert!(
            owed(&text).is_empty() && !text.contains("\n  ∅ "),
            "{mode:?}:\n{text}"
        );
        let ms: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&mj).expect("a summary")).expect("JSON");
        assert!(
            ms["region"]["root_owed"].is_null() && ms["region"]["root_owes"].is_null(),
            "{mode:?}"
        );
    }
}

/// **A query over an empty R says which empty.** Each query's `over` row
/// tells a contradiction from a debt, as root says it, under describe,
/// `--single` and `-n` alike, the three that print the row: not *R is empty:
/// no state to answer from* whether root was dead or alive and owing.
///
/// **No claim moves** (Q-M1f.219): `as :expect` is
/// `(false)` under both wordings, and a query's `:expect (false)` holds on an
/// R that is empty by owing as it did, exit 0, on
/// `examples/binary_logic/unsatisfiable.ein`, whose root owes three instances
/// and is not on the owing frontier.
#[test]
fn a_query_over_an_empty_r_says_which_empty() {
    const DEAD: &str = "all of R — R is empty, root is dead: no state to answer from";
    let owing = |n: &str| {
        format!("all of R — R is empty, root is alive and owes {n}: no state to answer from")
    };
    for (file, want) in [
        ("tests/region/04_owing.ein", owing("1 instance")),
        ("tests/region/05_empty.ein", owing("1 instance")),
        ("examples/ein-bugs/zebra2-bad.ein", DEAD.to_string()),
        (
            "tests/stdlib/algebra/09_irreflexive_violated.ein",
            DEAD.to_string(),
        ),
    ] {
        for mode in [&[][..], &["--single"], &["-n", "2"]] {
            let mut argv = vec![file];
            argv.extend_from_slice(mode);
            let run = solve(&argv);
            assert_eq!(rows(&run, "over"), vec![want.clone()], "{file} {mode:?}");
            assert_eq!(rows(&run, "as :expect"), vec!["(false)"], "{file} {mode:?}");
            assert!(
                !run.out.contains("R is empty: no state"),
                "{file} {mode:?}: the bare wording:\n{}",
                run.out
            );
        }
    }
    // The claim under the row is what it was.
    let run = solve(&["examples/binary_logic/unsatisfiable.ein"]);
    assert_eq!(run.summary["region"]["empty"], "owing");
    assert_eq!(run.summary["region"]["root_owes"], 3);
    assert_eq!(rows(&run, "over"), vec![owing("3 instances")]);
    assert_eq!(rows(&run, ":expect"), vec!["holds"]);
    assert_eq!(run.summary["queries"][0]["expect"]["outcome"], "held");
    // The solve claim is about the owing frontier, which the default route,
    // the tree and the classes, does not compute: not checked, and the state
    // route holds it.
    assert_eq!(rows(&run, "(solve :expect)"), vec!["NOT CHECKED"]);
    let states = solve(&[
        "examples/binary_logic/unsatisfiable.ein",
        "--describe-route",
        "states",
    ]);
    assert_eq!(rows(&states, "(solve :expect)"), vec!["holds"]);
}

/// **`--count` prints describe's rows and reads no state**, on every corpus
/// program and regression witness: a count is describe's question with the
/// listing left out (Q-M1f.218).
///
/// In-process, at the corpus sweep's small budget and cube limit (2¹⁴ facts,
/// 2¹⁰ nodes, the golden's sweep's, `corpus_ops`'), so that partial runs of every kind are
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
    for row in ein_corpus::regressions().case {
        files.push(repo_root().join(&row.witness));
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
                            // After the header and the rule: a flagged
                            // program's first row is the warning, which
                            // describe prints too, so `asked` is found by
                            // its label.
                            for l in ct.stdout.lines().skip(2) {
                                if l.is_empty() {
                                    break;
                                }
                                if !l.starts_with("  asked ")
                                    && !l.starts_with("    paths ")
                                    && !shown.contains(l)
                                {
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
    // A floor, so that a sweep that compared few programs cannot pass.
    assert!(
        compared >= 190 && partial >= 15 && claims >= 70,
        "{compared} compared, {partial} partial, {claims} claims"
    );
}

/// **A core holding a nested fact prints it as an s-expression.**
/// `nested-core.ein`'s core is the given `(not (p A))` and `(q A)`, the first
/// never a struct repr, `(not Fact(relation_name='p', args=('A',)))`. The
/// line is the read-out's, and the summary's `region.core`
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

/// **`ein oracle` names no option it refuses.**
///
/// `ein oracle` has no `--dump-answer`: it refuses the flag, exit 2. So each
/// line past 100 that names where the rest is names `ein solve FILE
/// --dump-answer DIR`, as the forms cut on `ein oracle` do, and `as
/// :expect`'s *it is in
/// DIR/answer.txt* says which command writes that DIR. The command it names
/// exits 0 and writes the whole of what the lines left out, over the oracle's
/// R. `ein solve`'s own lines still name the flag alone, and `ein oracle`
/// gained no flag. `03_forall.ein` carries all three kinds of line.
#[test]
fn the_oracles_read_out_names_the_command_that_writes_its_rest() {
    const FORALL: &str = "examples/features/03_forall.ein";
    const NAMED: &str = "ein solve FILE --dump-answer DIR";
    let run = |args: &[&str]| {
        let out = Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(args)
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs");
        (
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stdout).into_owned(),
        )
    };
    let k_s = |text: &str| {
        text.lines()
            .find_map(|l| l.trim_start().strip_prefix("states (k_s)"))
            .map(|v| v.trim().to_string())
    };
    let (code, oracle) = run(&["oracle", FORALL]);
    assert_eq!(code, 0, "{oracle}");
    let named: Vec<&str> = oracle.lines().filter(|l| l.contains("DIR")).collect();
    for l in &named {
        assert_eq!(
            l.matches("--dump-answer").count(),
            l.matches(NAMED).count(),
            "a line names --dump-answer without `ein solve FILE`: {l}"
        );
    }
    assert!(
        named.iter().any(|l| l.starts_with("  ;; not shown: ")
            && l.ends_with(&format!(
                "— {NAMED} writes the whole (region …) to DIR/answer.txt"
            ))),
        "the region's cut:\n{oracle}"
    );
    assert!(
        named.iter().any(|l| l
            .trim_start()
            .starts_with(&format!("… 3 more classes: {NAMED} writes them"))),
        "the query's cut classes:\n{oracle}"
    );
    assert!(
        named
            .iter()
            .any(|l| l.trim_start().starts_with("as :expect")
                && l.ends_with(&format!("it is in DIR/answer.txt, which {NAMED} writes"))),
        "as :expect:\n{oracle}"
    );
    assert_eq!(named.len(), 3, "{oracle}");

    let dir = Scratch::new("ein-read-out-oracle");
    let d = dir.join("d");
    let d = d.to_string_lossy();
    // No new flag: `ein oracle` still refuses it, before anything runs.
    assert_eq!(run(&["oracle", FORALL, "--dump-answer", &d]).0, 2);
    // The command the lines name exits 0, and writes R whole: the oracle's
    // states, wrapped in `(region …)`, no listing cut.
    let (code, solve) = run(&["solve", FORALL, "--dump-answer", &d]);
    assert_eq!(code, 0, "{solve}");
    let answer = std::fs::read_to_string(dir.join("d/answer.txt")).expect("answer.txt");
    assert_eq!(k_s(&answer), k_s(&oracle), "{answer}");
    assert!(answer.contains("\n  (region\n"), "{answer}");
    for cut in ["not shown", "more classes", "not printed"] {
        assert!(
            !answer.contains(cut),
            "answer.txt is cut ({cut}):\n{answer}"
        );
    }
    // `ein solve`'s own lines are unchanged: the flag alone.
    assert!(
        solve.contains("— --dump-answer DIR writes the whole (region …) to DIR/answer.txt")
            && !solve.contains(NAMED),
        "{solve}"
    );
}

/// **What the universe leaves out is one sentence, on every surface that
/// reports R** (Q-M1e2.3): R is scoped, and the read-out says so. R is
/// over H, and the
/// read-out names each relation the program can conclude or ask about, does
/// not scope out itself, and of which the rung that built H proposes no fact
/// (`ein_infer::scope`). Every question prints the same `outside` row under
/// `universe`, `ein oracle` and its dump say it, `ein test` says it beside a
/// claim, and the summary lists the relations in `root.hypgen.outside`, on
/// `--monotone` too. One witness per rung: `uncovered.ein`'s obligations,
/// `arity3.ein`'s blind enumerator (whose `is-a`, scoped out by
/// `:no-hypothesis`, is not named), `examples/lattice/15_union_label_dead.ein`'s
/// hypothesis rules, which states a claim; and a program that leaves nothing out prints no row and an
/// empty list.
#[test]
fn the_universes_scope_is_one_sentence_on_every_surface() {
    const TAIL: &str = "R holds of it only what the program states and saturation derives";
    let cases: [(&str, &[&str], String); 4] = [
        (
            "examples/ein-bugs/wrong-answers/uncovered.ein",
            &["knows"],
            format!("knows — no candidate: no obligation owes it. {TAIL}"),
        ),
        (
            "examples/ein-bugs/wrong-answers/arity3.ein",
            &["r3"],
            format!(
                "r3 — no candidate: it has arity 3, and the blind enumerator proposes arity 1 \
                 and 2 only. {TAIL}"
            ),
        ),
        (
            "examples/lattice/15_union_label_dead.ein",
            &["flag"],
            format!("flag — no candidate: no hypothesis rule proposes it. {TAIL}"),
        ),
        ("tests/region/01_states.ein", &[], String::new()),
    ];
    let run = |args: &[&str]| {
        let out = Command::new(env!("CARGO_BIN_EXE_ein"))
            .args(args)
            .current_dir(repo_root())
            .output()
            .expect("the `ein` binary runs");
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        assert_eq!(out.status.code(), Some(0), "ein {args:?}:\n{stdout}");
        stdout
    };
    let rows = |text: &str| -> Vec<String> {
        text.lines()
            .filter_map(|l| l.strip_prefix("    outside         "))
            .map(str::to_string)
            .collect()
    };
    let listed = |json: &serde_json::Value| -> Vec<String> {
        json.as_array()
            .expect("an array on every run")
            .iter()
            .map(|o| o["relation"].as_str().expect("a relation").to_string())
            .collect()
    };
    for (file, relations, sentence) in &cases {
        let want: Vec<String> = if sentence.is_empty() {
            Vec::new()
        } else {
            vec![sentence.clone()]
        };
        let tmp = Scratch::new("ein-scope");
        let questions: [&[&str]; 7] = [
            &[],
            &["--maximal"],
            &["--minimal"],
            &["--single"],
            &["-n", "1"],
            &["--count"],
            &["--monotone"],
        ];
        for (i, flags) in questions.iter().enumerate() {
            let json = tmp.join(format!("summary-{i}.json"));
            let json = json.to_str().expect("a UTF-8 path");
            let mut args = vec!["solve", file];
            args.extend_from_slice(flags);
            args.extend_from_slice(&["--json-summary", json]);
            let out = run(&args);
            assert_eq!(rows(&out), want, "ein solve {file} {flags:?}:\n{out}");
            let summary: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(json).expect("a summary"))
                    .expect("it parses");
            assert_eq!(
                listed(&summary["root"]["hypgen"]["outside"]),
                *relations,
                "{file} {flags:?}: root.hypgen.outside"
            );
        }
        let dump = tmp.join("cube");
        let dump = dump.to_str().expect("a UTF-8 path");
        let out = run(&["oracle", file, "--dump", dump]);
        assert_eq!(rows(&out), want, "ein oracle {file}:\n{out}");
        let md = std::fs::read_to_string(tmp.join("cube/region.md")).expect("region.md");
        assert_eq!(
            md.contains("**Outside it**: "),
            !sentence.is_empty(),
            "{file}: region.md"
        );
        if !sentence.is_empty() {
            assert!(md.contains(&format!("**Outside it**: {sentence}.")), "{md}");
        }
        let cube: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(tmp.join("cube/region.json")).expect("region.json"),
        )
        .expect("it parses");
        assert_eq!(listed(&cube["outside"]), *relations, "{file}: region.json");
    }
    // `ein test`, beside the claim it prints.
    let out = run(&["test", "-v", "examples/lattice/15_union_label_dead.ein"]);
    assert!(
        out.contains(&format!("    outside the universe: {}\n", cases[2].2)),
        "{out}"
    );
}

/// **`is-a` is a relation like any other, on every surface that reports R**
/// (Q-M1e.12). The blind enumerator is untyped: `pets.ein`'s `Pet`, the atom
/// its `is-a` facts point at, is an argument of candidates of H like any
/// other guessable atom, and R holds them. And nothing reads the relation by
/// its spelling: the same program with `is-a` spelt `is-b` prints, name for
/// name, what it prints under every question and in its summary, in `ein
/// oracle`'s output and its dump, and in `ein test`'s lines and report, here
/// on a claim that the maximal line holds `(likes Ann Person)`. Each pair is
/// run from its own directory under one file name, so a path the outputs
/// echo is the same in both.
#[test]
fn is_a_reads_as_any_other_relation_on_every_surface() {
    let pets =
        std::fs::read_to_string(repo_root().join("examples/saturation/type-exclusivity/pets.ein"))
            .expect("pets.ein");
    let likes = "(relation is-a T T :why \"{?3} is a {?4}\")\n\
                 (relation likes T T :why \"{?3} likes {?4}\")\n(__closed__ is-a)\n\
                 (is-a Ann Person)\n\
                 (query :goal (likes Ann ?x) :over maximal\n\
                 \x20 :expect (model (likes Ann Person)))\n";
    assert!(!pets.contains("is-b") && !likes.contains("is-b"));
    let tmp = Scratch::new("ein-is-a");
    // Every surface's text for one spelling, the other spelt back as `is-a`.
    let surfaces = |spelling: &str| -> Vec<(String, String)> {
        let dir = tmp.join(spelling);
        std::fs::create_dir_all(&dir).expect("a directory");
        std::fs::write(dir.join("pets.ein"), pets.replace("is-a", spelling)).expect("pets");
        std::fs::write(dir.join("likes.ein"), likes.replace("is-a", spelling)).expect("likes");
        let run = |args: &[&str]| -> (Option<i32>, String) {
            let out = Command::new(env!("CARGO_BIN_EXE_ein"))
                .args(args)
                .current_dir(&dir)
                .output()
                .expect("the `ein` binary runs");
            let text = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            (out.status.code(), text)
        };
        let read = |name: &str| std::fs::read_to_string(dir.join(name)).expect(name);
        let mut got = Vec::new();
        let questions: [&[&str]; 7] = [
            &[],
            &["--maximal"],
            &["--minimal"],
            &["--single"],
            &["-n", "1"],
            &["--count"],
            &["--monotone"],
        ];
        for (i, flags) in questions.iter().enumerate() {
            let json = format!("summary-{i}.json");
            let mut args = vec!["solve", "pets.ein"];
            args.extend_from_slice(flags);
            args.extend_from_slice(&["--json-summary", &json]);
            let (code, out) = run(&args);
            assert_eq!(code, Some(0), "ein {args:?} ({spelling}):\n{out}");
            got.push((format!("ein solve {flags:?}"), out));
            got.push((format!("ein solve {flags:?}: the summary"), read(&json)));
        }
        let (code, out) = run(&["oracle", "pets.ein", "--dump", "cube"]);
        assert_eq!(code, Some(0), "ein oracle ({spelling}):\n{out}");
        got.push(("ein oracle".to_string(), out));
        got.push(("region.json".to_string(), read("cube/region.json")));
        // The cost line is wall-clock, and the one line of the dump that differs
        // between two runs.
        let md: String = read("cube/region.md")
            .lines()
            .filter(|l| !l.ends_with("differ between two runs."))
            .map(|l| format!("{l}\n"))
            .collect();
        got.push(("region.md".to_string(), md));
        let (code, out) = run(&["test", "-v", "likes.ein", "--json-report", "report.json"]);
        assert_eq!(code, Some(0), "ein test ({spelling}):\n{out}");
        // The tally's wall-clock, `  (0.00 s)`, is the one thing that differs.
        let out: String = out
            .lines()
            .map(|l| match l.rfind("  (") {
                Some(i) if l.ends_with(" s)") => format!("{}\n", &l[..i]),
                _ => format!("{l}\n"),
            })
            .collect();
        got.push(("ein test -v".to_string(), out));
        let mut report: serde_json::Value =
            serde_json::from_str(&read("report.json")).expect("the report parses");
        for row in report["rows"].as_array_mut().expect("rows") {
            if let Some(region) = row.get_mut("region").and_then(|r| r.as_object_mut()) {
                region.remove("ms");
            }
        }
        got.push(("the report".to_string(), report.to_string()));
        got.into_iter()
            .map(|(what, text)| (what, text.replace(spelling, "is-a")))
            .collect()
    };
    let is_a = surfaces("is-a");
    let is_b = surfaces("is-b");
    assert_eq!(is_a.len(), is_b.len());
    for ((what, a), (_, b)) in is_a.iter().zip(&is_b) {
        assert_eq!(a, b, "{what}: `is-b` reads otherwise than `is-a`");
    }
    // Untyped: `Pet` is an argument of candidates of H, and of states of R.
    let describe = &is_a[0].1;
    assert!(
        describe.contains("  universe          15 candidates, 0 dead alone\n")
            && describe.contains("(model (co-located Bird Pet) (is-a Bird Pet)"),
        "{describe}"
    );
    let summary: serde_json::Value = serde_json::from_str(&is_a[1].1).expect("it parses");
    assert_eq!(summary["region"]["universe"]["candidates"], 15);
    assert_eq!(summary["region"]["k_s"], 11_339);
    let test = &is_a[is_a.len() - 2].1;
    assert!(test.contains("  :goal (likes Ann ?x) — holds ("), "{test}");
}
