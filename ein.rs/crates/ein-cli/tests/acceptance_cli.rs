//! T1a.10.2.3 — the acceptance gate's **CLI** half.
//!
//! Five of `ein.py/acceptance/`'s twenty-one tests do not ask the engine
//! anything: they run `ein solve <puzzle>` and read what a person would see.
//! The other sixteen are engine claims and live in
//! `ein-infer/tests/acceptance.rs`; the split is by *what is being asserted*,
//! not by which Python file it came from.
//!
//! | Python | what it asserts |
//! |---|---|
//! | `test_bench_solve_mode.py::test_solve_solves_zebra2_correctly` | the table names the four cells the puzzle asks about, `k = 1`, and the `:hrules` line reports the hypothesis relations |
//! | `test_bench_solve_mode.py::test_solve_exhaustive_certifies_unique` | `-e` prints `exhausted true`, which is what turns `k = 1` into uniqueness |
//! | `test_zebra_three_classes.py::test_cli_solve_emits_answer_in_words` | `zebra2` answers **in English**, from the file's `:goal-text` and `:why` templates |
//! | `test_zebra_three_classes.py::test_cli_solve_contradiction_reports_no_solution` | the unsat fixture says "no solution", says why, and **names the injected `:source`** — and still exits 0, because classifying an unsat puzzle correctly is a success |
//! | `test_zebra_two_ontologies.py::test_cli_solve_zebra_emits_answer_in_words` | the same, through the *other* encoding, so the English cannot be hardcoded per ontology |
//!
//! They run the built binary (`CARGO_BIN_EXE_ein`) rather than calling the
//! library, because what is under test is the *presentation*: the exit code,
//! stdout, and the fact that no flag was needed to get any of it. Assertions
//! are on **content**, never on layout — the byte-exact form of every one of
//! these lines is already pinned by `ein-render`'s `corpus_shapes.md5`
//! (`trace[answer]`, `trace[table]`), and duplicating that here would make a
//! whitespace change fail twice and say nothing new.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// Run `ein <args…>` from the repo root and hand back `(code, stdout)`.
///
/// Lower-cased, because every assertion below is about *which words appear*
/// and the answer text is prose.
fn ein(args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    assert!(
        out.stderr.is_empty() || !out.status.success(),
        "a successful solve writes nothing to stderr, got: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).to_lowercase(),
    )
}

/// The value of a `  <label>   <value>` row in the `--stats` block.
///
/// By label rather than by position, and trimmed rather than matched with the
/// exact run of spaces — the layout is `corpus_shapes.md5`'s to pin.
fn field(out: &str, label: &str) -> Option<String> {
    out.lines()
        .find(|l| l.trim_start().starts_with(label))
        .and_then(|l| l.trim_start().strip_prefix(label))
        .map(|v| v.split_whitespace().next().unwrap_or("").to_string())
}

fn must_contain(out: &str, needles: &[&str], what: &str) {
    let missing: Vec<&str> = needles
        .iter()
        .copied()
        .filter(|n| !out.contains(n))
        .collect();
    assert!(
        missing.is_empty(),
        "{what}: {missing:?} missing from\n{}",
        &out[..out.len().min(2000)]
    );
}

/// The default solve answers both halves of the puzzle's own question — who
/// drinks water and who owns the zebra — and reports the table.
#[test]
fn solve_zebra2_prints_the_answer_the_puzzle_asked_for() {
    // The Python original's argv, kept: `--print-final-state` and
    // `--print-final-hfacts` are two surfaces nothing else in the suite runs
    // end to end, and `--stats` is where `exhausted` is reported. `k` is the
    // *table*'s row, which is what `field` finds first and what the assertion
    // below means — the two are different numbers on an `Open` verdict, and
    // since M1e S1e.3.4 they are under different labels (`AR-M2`).
    let (code, out) = ein(&[
        "solve",
        "examples/zebra2.ein",
        "--print-final-state",
        "--print-final-hfacts",
        "--stats",
    ]);
    assert_eq!(code, 0, "a solved puzzle exits 0");
    must_contain(
        &out,
        &["norwegian", "water", "japanese", "zebra"],
        "the answer in words",
    );
    must_contain(
        &out,
        &[
            "(drink-loc water house-1)",
            "(pet-loc zebra house-5)",
            "(nation-loc japanese house-5)",
            "(nation-loc norwegian house-1)",
        ],
        "the four named cells",
    );
    assert_eq!(
        field(&out, "solutions (k)"),
        Some("1".into()),
        "k must be 1"
    );
    // `--print-final-hfacts` reports which relations the blind enumerator was
    // pointed at. ein.py asserted `:hrules [.*'nation-loc'.*]`; the list is
    // the five typed projections and nothing else, which is stronger and is
    // what the fixture is for.
    let hrules = out
        .split(":hrules [")
        .nth(1)
        .and_then(|r| r.split(']').next())
        .unwrap_or("");
    assert_eq!(
        hrules, "'color-loc', 'drink-loc', 'nation-loc', 'pet-loc', 'smoke-loc'",
        "exactly the five hypothesis-target relations"
    );
}

/// `-e` is what makes `k = 1` mean *unique*: it says the lattice ran out.
#[test]
fn solve_exhaustive_certifies_that_the_answer_is_the_only_one() {
    let (code, out) = ein(&["solve", "examples/zebra2.ein", "--exhaustive", "--stats"]);
    assert_eq!(code, 0);
    assert_eq!(
        field(&out, "solutions (k)"),
        Some("1".into()),
        "k must be 1"
    );
    assert_eq!(
        field(&out, "exhausted"),
        Some("true".into()),
        "an exhaustive run certifies uniqueness by saying the lattice ran out"
    );
    // The control: without `--exhaustive` the same puzzle reports the same `k`
    // and does *not* certify it. Without this, the test above would pass on a
    // build that hardcoded `true`.
    let (_, plain) = ein(&["solve", "examples/zebra2.ein", "--stats"]);
    assert_eq!(field(&plain, "exhausted"), Some("false".into()));
}

/// The *other* encoding reaches the same English. The answer is rendered from
/// the file's own `:goal-text` and its relations' `:why` templates, so a
/// hardcoded per-ontology string would pass the previous test and fail this
/// one — which is exactly why the Python gate had both.
#[test]
fn solve_zebra_prints_the_same_answer_through_the_generic_encoding() {
    let (code, out) = ein(&["solve", "examples/zebra.ein"]);
    assert_eq!(code, 0);
    must_contain(
        &out,
        &["norwegian", "water", "japanese", "zebra"],
        "the answer in words, generic encoding",
    );
}

/// An unsat puzzle is a *correct classification*, not a failure: exit 0, the
/// words "no solution", the reason, and the `:source` of the fact that caused
/// it — which is the whole point of `zebra2-bad.ein`.
#[test]
fn solve_on_an_unsat_puzzle_says_no_solution_and_names_the_culprit() {
    let (code, out) = ein(&["solve", "examples/ein-bugs/zebra2-bad.ein"]);
    assert_eq!(
        code, 0,
        "classifying an unsat puzzle correctly is a success"
    );
    must_contain(
        &out,
        &["no solution", "contradict", "injected contradiction"],
        "the contradiction report",
    );
}
