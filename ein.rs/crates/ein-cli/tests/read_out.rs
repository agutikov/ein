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
/// exactly when the run did not exhaust.
#[test]
fn each_count_is_printed_once_and_is_the_summarys() {
    for (file, flags) in CELLS {
        let mut argv = vec![*file];
        argv.extend_from_slice(flags);
        let run = solve(&argv);
        let where_ = format!("{file} {}", flags.join(" "));
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
