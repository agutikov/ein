//! T1a.10.2.3 — the acceptance gate's **CLI** half.
//!
//! These tests do not ask the engine anything: they run `ein solve <puzzle>`
//! and read what a person would see. The acceptance gate's engine claims live
//! in `ein-infer/tests/acceptance.rs`; the split is by *what is being
//! asserted*.
//!
//! | test | what it asserts |
//! |---|---|
//! | [`solve_zebra2_prints_the_answer_the_puzzle_asked_for`] | bare `ein solve` lists R's one state with the four cells the puzzle asks about, H is the five hypothesis relations, and `zebra2` answers **in English**, from the file's `:goal-text` and `:why` templates |
//! | [`solve_single_certifies_that_the_answer_is_the_only_one`] | `--single` answers `Solution — R is one state`, decided, which is what turns the one state into uniqueness |
//! | [`solve_on_an_unsat_puzzle_says_no_solution_and_names_the_culprit`] | the unsat fixture says R is empty, says why, and **names the injected `:source`** — and still exits 0, because classifying an unsat puzzle correctly is a success |
//! | [`solve_zebra_prints_the_same_answer_through_the_generic_encoding`] | the same, through the *other* encoding, so the English cannot be hardcoded per ontology |
//!
//! Since M1e2 S1e2.1a.5's switch bare `ein solve` describes R, and on both
//! puzzles it stops at the default `--region-budget` with R's one state found:
//! an answer, so exit 0, with one `note:` on stderr saying it is partial. That
//! note is the only stderr a successful run here may write.
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
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.lines().all(|l| l.starts_with("note: ")) || !out.status.success(),
        "a successful solve writes only notes to stderr, got: {err}"
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
/// drinks water and who owns the zebra — and lists the state.
#[test]
fn solve_zebra2_prints_the_answer_the_puzzle_asked_for() {
    // `-P` and `-H` are two surfaces nothing else in the suite runs end to end
    // on the puzzle: the state's positive facts beyond root, and H.
    let (code, out) = ein(&["solve", "examples/zebra2.ein", "-P", "-H"]);
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
    assert_eq!(field(&out, "states (k_s)"), Some("1".into()), "one state");
    // H is what the commitments are made of: the five typed projections and
    // nothing else, which is what the fixture is for.
    let relations: Vec<&str> = out
        .split("h — the universe")
        .nth(1)
        .expect("-H's block")
        .lines()
        .skip(1)
        .take_while(|l| l.starts_with("  "))
        .filter(|l| !l.starts_with("    "))
        .filter_map(|l| l.split_whitespace().next())
        .collect();
    assert_eq!(
        relations,
        [
            "color-loc",
            "drink-loc",
            "nation-loc",
            "pet-loc",
            "smoke-loc"
        ],
        "exactly the five hypothesis-target relations"
    );
}

/// `--single` is what makes the one state mean *unique*: it decides that R
/// holds no other.
#[test]
fn solve_single_certifies_that_the_answer_is_the_only_one() {
    let (code, out) = ein(&["solve", "examples/zebra2.ein", "--single"]);
    assert_eq!(code, 0);
    must_contain(&out, &["solution — r is one state"], "the word");
    assert_eq!(
        field(&out, "decided"),
        Some("yes".into()),
        "--single certifies uniqueness by deciding R holds no other state"
    );
    // The control: the default run finds the same one state and does *not*
    // certify it. Without this, the test above would pass on a build that
    // hardcoded the word.
    let (_, plain) = ein(&["solve", "examples/zebra2.ein"]);
    assert_eq!(field(&plain, "exhausted"), Some("no".into()));
}

/// The *other* encoding reaches the same English. The answer is rendered from
/// the file's own `:goal-text` and its relations' `:why` templates, so a
/// hardcoded per-ontology string would pass the previous test and fail this
/// one — which is exactly why the gate has both.
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

/// An unsat puzzle is a *correct classification*, not a failure: exit 0, R
/// empty because every node is dead, the reason, and the `:source` of the fact
/// that caused it — which is the whole point of `zebra2-bad.ein`.
#[test]
fn solve_on_an_unsat_puzzle_says_no_solution_and_names_the_culprit() {
    let (code, out) = ein(&["solve", "examples/ein-bugs/zebra2-bad.ein"]);
    assert_eq!(
        code, 0,
        "classifying an unsat puzzle correctly is a success"
    );
    must_contain(
        &out,
        &[
            "every node is dead",
            "unsat core        (color-loc green house-1)",
            ":source \"injected contradiction\"",
        ],
        "the contradiction report",
    );
}
