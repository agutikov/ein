//! M1d S1d.3.3 — **how `ein solve` reports a model *set***: the guarantee
//! rule, and the determining key that is the set's compact form, read since
//! M1e2 S1e2.1a.5's switch over the states a run lists.
//!
//! [P1d.3](../../../../docs/history/m1d_satisfiability/README.md#p1d3--model-sets)
//! answered *print or describe* with **both**, and the two halves are tested
//! here because they are one surface. The rule first, since the key inherits
//! it.
//!
//! ## The rule (T1d.3.3.2)
//!
//! | the run | what a report of the set may claim |
//! |---|---|
//! | exhausted | *these are the states* — of R, or of the line asked |
//! | not exhausted | *these are states **found*** — the set may hold more |
//!
//! The walk's `Ambiguity` printed a bare *5* for
//! `examples/saturation/type-exclusivity/colors.ein`, whose maximal line has
//! **9** states — the proof the rule needed. The region read-out carries the
//! qualifier on every count: a describe's `k_s` says *a lower bound* and
//! where it is complete up to, and a mode's line says *as asked — may hold
//! more*.
//!
//! ## The key (T1d.3.3.4)
//!
//! `--models key` is **additional output, never a second answer**: it adds a
//! projection of the listed states to stdout and reaches nothing recorded. The
//! third test is that claim, checked the way
//! [S1d.2.5](../../../../docs/history/m1d_satisfiability/hypotheses_from_obligations.md)
//! checked its lever — run the entry both ways and diff everything outside
//! the block that is allowed to move.

use std::process::Command;

use ein_corpus::repo_root;

struct Run {
    out: String,
    code: i32,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        code: out.status.code().unwrap_or(-1),
    }
}

/// The `  <label>   <rest>` row, whole and trimmed.
fn row(run: &Run, label: &str) -> String {
    run.out
        .lines()
        .find(|l| l.trim_start().starts_with(label))
        .unwrap_or_else(|| panic!("no `{label}` row in:\n{}", run.out))
        .trim()
        .to_string()
}

const COLORS: &str = "examples/saturation/type-exclusivity/colors.ein";

/// **exhausted-says-these-are-the-states.** Row one of the rule, on a real
/// multi-state entry: `colors.ein`'s R is 169 states, exhausted, and its
/// maximal line nine — the walk's nine models, asked by `--maximal`.
#[test]
fn an_exhausted_run_reports_the_states() {
    let all = ein(&["solve", COLORS]);
    assert_eq!(all.code, 0);
    assert_eq!(row(&all, "states (k_s)"), "states (k_s)      169");
    assert_eq!(
        row(&all, "exhausted"),
        "exhausted         yes — every commitment is classified: this is all of R"
    );
    assert_eq!(row(&all, "maximal line"), "maximal line      9 states");
    let max = ein(&["solve", COLORS, "--maximal"]);
    assert_eq!(
        row(&max, "maximal line"),
        "maximal line      9 states — all of them"
    );
    assert_eq!(
        row(&max, "exhausted"),
        "exhausted         yes — this is all of the maximal line"
    );
}

/// **not-exhausted-says-these-are-states-found.** Row two, and the one that
/// fails if someone later drops the qualifier: on a part asked (`-n 5`) and
/// on a description its budget stopped. Every number is the same kind of
/// number as above, and every one is a lower bound.
#[test]
fn an_unexhausted_run_reports_states_found() {
    let part = ein(&["solve", COLORS, "--maximal", "-n", "5"]);
    assert_eq!(part.code, 0);
    assert_eq!(
        row(&part, "maximal line"),
        "maximal line      5 states, as asked — the maximal line may hold more"
    );
    assert!(
        row(&part, "exhausted").starts_with("exhausted         no — stopped as asked"),
        "{}",
        part.out
    );
    let stopped = ein(&["solve", COLORS, "--region-budget", "50"]);
    assert_eq!(stopped.code, 0);
    assert_eq!(
        row(&stopped, "states (k_s)"),
        "states (k_s)      37   (a lower bound: complete up to states of 1 candidate)"
    );
    assert!(
        row(&stopped, "exhausted").starts_with("exhausted         no — the state route stopped")
    );
}

/// **the-key-is-a-projection-not-a-second-answer.** T1d.3.3.4's constraint,
/// as a diff.
///
/// `--models key` may add its block to stdout and nothing else. So: the same
/// entry, both ways, with `--json-summary` written each time — the summaries
/// must be **byte-identical**, because `region.states` is what every
/// downstream consumer reads, and stdout must agree on every line outside the
/// block and the header's echo of the flags.
#[test]
fn the_key_form_changes_stdout_and_nothing_recorded() {
    let dir = std::env::temp_dir().join(format!("ein-model-set-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let a = dir.join("list.json");
    let b = dir.join("key.json");
    let entry = "examples/lattice/02_genuine_3set_death.ein";

    let listed = ein(&[
        "solve",
        entry,
        "--maximal",
        "--json-summary",
        &a.to_string_lossy(),
    ]);
    let keyed = ein(&[
        "solve",
        entry,
        "--maximal",
        "--models",
        "key",
        "--json-summary",
        &b.to_string_lossy(),
    ]);
    assert_eq!((listed.code, keyed.code), (0, 0));
    assert_eq!(
        std::fs::read_to_string(&a).expect("a summary"),
        std::fs::read_to_string(&b).expect("a summary"),
        "--models moved something a consumer reads"
    );
    let body = |r: &Run| {
        r.out
            .lines()
            .skip(1)
            .take_while(|l| !l.starts_with("models — "))
            .map(str::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end()
            .to_string()
    };
    assert_eq!(body(&listed), body(&keyed));
    assert!(
        !listed.out.contains("determining key") && keyed.out.contains("determining key"),
        "the key form adds its block:\n{}",
        keyed.out
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **the-key-determines-and-says-how.** The form itself, on the smallest
/// corpus entry that has one.
///
/// Three things a key table must carry, and each is a separate failure:
/// the *size* against the varying total, the *count* of equally minimal keys
/// — the answer to "why these" — and one row per state. The rows are the
/// joint projection, so their number is the states'; an envelope's would have
/// been the product.
#[test]
fn the_key_table_names_its_size_its_alternatives_and_one_row_per_state() {
    let run = ein(&[
        "solve",
        "examples/lattice/02_genuine_3set_death.ein",
        "--maximal",
        "--models",
        "key",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(
        row(&run, "maximal line"),
        "maximal line      3 states — all of them"
    );
    assert_eq!(
        row(&run, "determining key"),
        "determining key — 2 of 3 varying slots"
    );
    let table: Vec<&str> = run
        .out
        .lines()
        .skip_while(|l| !l.contains("-----"))
        .skip(1)
        .take_while(|l| !l.trim().is_empty())
        .collect();
    assert_eq!(table.len(), 3, "one row per state, got:\n{table:#?}");
    assert!(
        run.out.contains("3 rows, one per state."),
        "an exhausted key table claims the states, not states found:\n{}",
        run.out
    );
}

/// **an-unaffordable-key-says-why.** The fallback, and it is a first-class
/// answer rather than an error.
///
/// A minimum determining set is a minimum hitting set, which is NP-hard, and
/// `branching/06_lookahead_on.ein` is the corpus entry that shows it: 42
/// varying slots over its maximal line's 22 states, minimum key **8**, and
/// `C(42, 8) = 118 030 185` candidate keys — so the count that answers *"why
/// these"* cannot be taken and the form declines rather than printing an
/// arbitrary basis. The states are listed above it either way, so the exit
/// code is 0 and nothing is withheld.
#[test]
fn an_unaffordable_key_says_why() {
    let run = ein(&[
        "solve",
        "examples/branching/06_lookahead_on.ein",
        "--maximal",
        "--models",
        "key",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(
        row(&run, "determining key"),
        "determining key — none within budget"
    );
    // Whitespace-normalised, because the reason is wrapped to the page and
    // where it wraps is layout rather than content.
    let flat = run.out.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("C(42, 8) = 118 030 185 candidates is over the budget"),
        "the reason should name the number that made it unaffordable:\n{}",
        run.out
    );
    // The states are listed above it, as far as stdout's 100 lines go, and
    // the cut names the file that holds the rest.
    assert!(
        run.out.contains(";; state 1 of 22"),
        "the states are listed:\n{}",
        run.out
    );
    assert!(
        run.out
            .contains(";; … 13 more states: --dump-answer DIR writes the listing uncut"),
        "{}",
        run.out
    );
}

/// **only-a-set-has-a-key.** The flag is inert where one state or none is
/// listed.
///
/// A single state is its own smallest description and an empty R has no set
/// at all, so `--models key` may not change one byte of either but the
/// header's echo of the flags. Checked rather than assumed, because the
/// natural way to write the renderer — read the flag once, at the top — would
/// have got this wrong, and did, until the switch's own test run.
#[test]
fn the_flag_is_inert_where_there_is_no_set() {
    for entry in [
        "examples/branching/05_mini_zebra.ein",
        "examples/features/12_expect_false.ein",
    ] {
        let listed = ein(&["solve", entry, "--maximal"]);
        let keyed = ein(&["solve", entry, "--maximal", "--models", "key"]);
        let body = |r: &Run| {
            r.out
                .lines()
                .skip(1)
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            (body(&listed), listed.code),
            (body(&keyed), keyed.code),
            "--models key moved {entry}, which has no set"
        );
    }
}
