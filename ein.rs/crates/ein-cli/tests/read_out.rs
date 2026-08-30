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
//! say that. It is **one label, one number, per invocation**, plus the count
//! the table prints being the verdict's own. Seven cells: every verdict word,
//! qualified and unqualified, which is every arm of
//! [`ein_infer::verdict::ReadOut`] that a `.ein` program can reach. The eighth
//! — a truncated `Open` — no program reaches, and it is pinned in
//! `verdict.rs`'s own unit test instead.
//!
//! [`AR-M2`]: `plans/m1e_review_processing/p1e.3_medium/s1e.3.4_architecture.md`

use std::process::Command;

use ein_corpus::repo_root;

struct Run {
    out: String,
    summary: serde_json::Value,
}

/// One `ein solve`, with `--stats` and a summary beside it — the two surfaces
/// whose labels collided, plus the machine copy that says what the answer was.
fn solve(args: &[&str]) -> Run {
    // A counter, not the argv: `cargo test` runs a file's tests as threads of
    // one process, and two of the three below solve the same cells — a path
    // derived from the arguments is one both would write and one delete.
    static NTH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let nth = NTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("ein-read-out-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch dir");
    let json = dir.join(format!("{nth}.json"));
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .args(args)
        .args(["--stats", "--json-summary"])
        .arg(&json)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let text = std::fs::read_to_string(&json).expect("a summary was written");
    let _ = std::fs::remove_file(&json);
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

/// `(file, extra flags, verdict word, qualifier)` — the seven reachable cells.
///
/// One per `(word, exhausted)` pair the corpus can produce. `branching/02`
/// appears twice on purpose: the same program is a `Solution` at the default
/// stop and an `Ambiguity` under `-e`, so the pair holds the two arms whose
/// qualifiers differ from each other rather than only from silence.
const CELLS: &[(&str, &[&str], &str, &str)] = &[
    (
        "examples/branching/01_saturate_only.ein",
        &[],
        "Solution",
        "",
    ),
    (
        "examples/branching/02_one_dead_one_alive.ein",
        &[],
        "Solution",
        "(not certified — pass --exhaustive)",
    ),
    (
        "examples/branching/04_two_levels.ein",
        &["-e"],
        "Ambiguity",
        "",
    ),
    (
        "examples/branching/02_one_dead_one_alive.ein",
        &["-e"],
        "Ambiguity",
        "(a lower bound — the search did not exhaust)",
    ),
    (
        "examples/branching/16_lookahead_two_step_off.ein",
        &[],
        "Contradiction",
        "",
    ),
    (
        "examples/branching/07_lookahead_off.ein",
        &[],
        "Contradiction",
        "(none found — the search did not exhaust)",
    ),
    ("tests/stdlib/algebra/23_total_owed.ein", &[], "Open", ""),
];

/// The table prints the **verdict's** `k`, with the **verdict's** qualifier.
///
/// `Solution` printed `stats.solution_nodes` here until S1e.3.4 ([`CO-M2`]) —
/// invisibly, because the only regime where the two differ is one no corpus
/// entry reaches. The fix is structural rather than arithmetic: the count is
/// no longer an argument of `render_solution_table`, so an arm has nothing to
/// choose with.
///
/// [`CO-M2`]: `plans/m1e_review_processing/p1e.3_medium/s1e.3.1_correctness.md`
#[test]
fn the_table_prints_the_verdicts_own_count() {
    for (file, flags, word, qualifier) in CELLS {
        let mut argv = vec![*file];
        argv.extend_from_slice(flags);
        let run = solve(&argv);
        let where_ = format!("{file} {}", flags.join(" "));

        assert_eq!(
            run.summary["verdict"]["type"], *word,
            "{where_}: the cell claims the wrong verdict"
        );
        let k = run.summary["verdict"]["k"]
            .as_u64()
            .expect("verdict.k is a number");

        let printed = rows(&run, "solutions (k)");
        assert_eq!(
            printed.len(),
            1,
            "{where_}: `solutions (k)` must be printed once and by one surface, got {printed:?}"
        );
        let want = if qualifier.is_empty() {
            k.to_string()
        } else {
            format!("{k}   {qualifier}")
        };
        assert_eq!(printed[0], want, "{where_}");
    }
}

/// `--stats` prints the **counter**, under the counter's name.
///
/// The block is engine counters and `solution_nodes` belongs in it; what did
/// not belong was the label `solutions (k)`, which is the answer's. Both rows
/// are asserted against the summary so that relabelling one of them back would
/// fail here rather than in a reader's terminal.
#[test]
fn stats_prints_what_the_search_recorded_under_its_own_name() {
    for (file, flags, _, _) in CELLS {
        let mut argv = vec![*file];
        argv.extend_from_slice(flags);
        let run = solve(&argv);
        let where_ = format!("{file} {}", flags.join(" "));

        let nodes = run.summary["stats"]["solution_nodes"]
            .as_u64()
            .expect("stats.solution_nodes is a number");
        assert_eq!(
            rows(&run, "solution_nodes"),
            vec![nodes.to_string()],
            "{where_}"
        );
    }
}

/// **The mixed regime, measured** — one discharged model beside one open
/// state.
///
/// `finalise` partitions the recorded nodes and reads the verdict off the
/// discharged ones alone, with its own comment saying *"no corpus entry is in
/// that regime today … defined rather than measured"*. The `Solution` arm
/// printed `stats.solution_nodes` until M1e S1e.3.4, so a program in that
/// regime would have printed **`solutions (k) 2` beside `verdict Solution`**:
/// the count saying two models and the word beside it saying one.
///
/// `examples/features/13_mixed_solution_and_open.ein` is that program — the
/// fixture `CO-M2` asked for, because a regime nobody has seen a read-out from
/// is one nobody can check. It is the whole of why the count is the verdict's
/// now: with the two numbers separated, this run says 1 and 2 in the two
/// places they belong.
#[test]
fn an_open_state_is_recorded_and_not_counted() {
    let run = solve(&["examples/features/13_mixed_solution_and_open.ein", "-e"]);
    assert_eq!(run.summary["verdict"]["type"], "Solution");
    assert_eq!(run.summary["verdict"]["k"], 1, "one *model*");
    assert_eq!(
        run.summary["stats"]["solution_nodes"], 2,
        "two complete states were recorded — the regime this fixture exists for"
    );
    assert_eq!(rows(&run, "solutions (k)"), vec!["1"]);
    assert_eq!(rows(&run, "solution_nodes"), vec!["2"]);
    // `owes.models` is where both nodes are still visible: the discharged one
    // and the one the verdict declined to call a model.
    let owed: Vec<u64> = run.summary["owes"]["models"]
        .as_array()
        .expect("a per-model tally")
        .iter()
        .map(|m| m["total"].as_u64().expect("a total"))
        .collect();
    assert_eq!(owed, vec![0, 1], "one model owes nothing and one owes one");
}

/// The entry where the two numbers really differ, printed side by side.
///
/// Twelve corpus entries answer `Open`, and every one of them has
/// `verdict.k = 0` with `stats.solution_nodes = 1`: the search recorded a node
/// and the read-out declines to call it a model (M1d S1d.2.6). That is the
/// only regime in the corpus where a surface printing the wrong one is
/// *visible*, which is why it is the regression rather than `CO-M2`'s own
/// mixed-`Solution` regime — that one nothing reaches, and
/// `s1e.3.1_correctness.md` T2 owes it a fixture.
#[test]
fn an_open_verdict_reports_zero_models_and_one_recorded_node() {
    let run = solve(&["tests/stdlib/slots/03_fill.ein"]);
    assert_eq!(run.summary["verdict"]["k"], 0);
    assert_eq!(run.summary["stats"]["solution_nodes"], 1);
    assert_eq!(rows(&run, "solutions (k)"), vec!["0"]);
    assert_eq!(rows(&run, "open states"), vec!["1"]);
    assert_eq!(rows(&run, "solution_nodes"), vec!["1"]);
}
