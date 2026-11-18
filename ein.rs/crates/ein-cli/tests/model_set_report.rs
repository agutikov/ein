//! **How `ein solve` reports a set of states**: every count says what it
//! guarantees.
//!
//! [What a count of models claims](../../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#6-a-set-of-models-and-what-its-count-claims):
//!
//! | the run | what a report of the set may claim |
//! |---|---|
//! | exhausted | *these are the states* — of R, or of the line asked |
//! | not exhausted | *these are states **found*** — the set may hold more |
//!
//! A bare *5* for `examples/saturation/type-exclusivity/colors.ein`, whose
//! maximal line has **9** states, would break the rule. The region read-out carries the
//! qualifier on every count: a describe's `k_s` says *a lower bound* and
//! where it is complete up to, and a mode's line says *as asked — may hold
//! more*.

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
/// maximal line nine, asked by `--maximal`.
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
