//! **`einaf outcome` and `einaf feedback`**, the command: one line a
//! reading, or the object as JSON; every outcome exits 0, a usage error 2.

use std::process::Command;

fn einaf(args: &[&str]) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_einaf"))
        .args(args)
        .current_dir(ein_corpus::repo_root())
        .output()
        .expect("einaf runs");
    (
        out.status.code().expect("an exit code"),
        String::from_utf8(out.stdout).expect("UTF-8"),
        String::from_utf8(out.stderr).expect("UTF-8"),
    )
}

#[test]
fn the_command_prints_one_line_a_reading() {
    let (code, out, _) = einaf(&["outcome", "examples/zebra2.ein", "--single"]);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "einaf outcome · examples/zebra2.ein --single\n  \
         one           over all of R           Solution: one state\n"
    );
    let (code, out, _) = einaf(&["outcome", "examples/zebra2.ein"]);
    assert_eq!(code, 0);
    assert_eq!(out.lines().count(), 4, "{out}");
}

#[test]
fn every_outcome_exits_zero() {
    for (args, word) in [
        (
            vec!["outcome", "examples/broken/unclosed_paren.ein", "--single"],
            "invalid",
        ),
        (
            vec![
                "outcome",
                "examples/lattice/04_obligation_trigger_hole.ein",
                "--single",
            ],
            "flagged",
        ),
        (
            vec!["outcome", "examples/zebra2.ein", "--region-budget", "1"],
            "not decided",
        ),
        (
            vec!["outcome", "examples/ein-bugs/zebra2-bad.ein", "--single"],
            "inconsistent",
        ),
        (
            vec!["outcome", "tests/region/04_owing.ein", "--single"],
            "incomplete",
        ),
        (
            vec!["outcome", "examples/zebra2-minus-15.ein", "--maximal"],
            "several",
        ),
    ] {
        let (code, out, _) = einaf(&args);
        assert_eq!(code, 0, "{args:?}");
        let second = out.lines().nth(1).unwrap_or("");
        assert!(second.trim_start().starts_with(word), "{args:?}: {out}");
    }
}

#[test]
fn a_usage_error_exits_two() {
    let (code, _, err) = einaf(&["outcome", "examples/zebra2.ein", "--single", "--count"]);
    assert_eq!(code, 2);
    assert!(err.starts_with("error: "), "{err}");
    let (code, _, _) = einaf(&["solve"]);
    assert_eq!(code, 2);
    let (code, _, _) = einaf(&["feedback", "examples/zebra2.ein", "--max-steps", "0"]);
    assert_eq!(code, 2);
}

/// **`einaf feedback` prints the object**, `einaf-feedback/1`, which reads
/// back as itself, with the bounds it was asked under; `einaf outcome` reads
/// the same object's outcome.
#[test]
fn the_feedback_command_prints_the_object() {
    let (code, out, err) = einaf(&["feedback", "examples/zebra2.ein", "--single", "-T", "60"]);
    assert_eq!(code, 0, "{err}");
    let f = einaf::kernel::Feedback::from_json(&out).expect("the object");
    assert_eq!(f.schema, einaf::kernel::SCHEMA);
    assert_eq!(f.asked.question, "--single");
    assert_eq!(f.asked.clock_us, Some(60_000_000));
    assert_eq!(f.asked.max_steps, Some(einaf::kernel::DEFAULT_MAX_STEPS));
    let r = f
        .outcome_over(einaf::kernel::Part::All)
        .expect("its reading");
    assert_eq!(r.outcome, einaf::kernel::Outcome::One);

    // A refused program: an object, invalid, its findings the diagnostics.
    let (code, out, _) = einaf(&["feedback", "examples/broken/unclosed_paren.ein"]);
    assert_eq!(code, 0);
    let f = einaf::kernel::Feedback::from_json(&out).expect("the object");
    assert!(
        f.outcome
            .iter()
            .flatten()
            .all(|r| r.outcome == einaf::kernel::Outcome::Invalid)
    );
    assert_eq!(f.diagnostics[0].kind, "parse");
}
