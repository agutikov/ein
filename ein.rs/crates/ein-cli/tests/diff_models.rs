//! **`--diff-models FILE`**: the states a run listed against the models a
//! file lists.
//!
//! The claims are the three shapes `ein-render::models`' unit tests pin, seen
//! through the CLI: a missing model, a spurious one, and one a fact off. The
//! region read-out answers the regressions right, so the shapes are made here
//! by writing the file wrong, over `branching/15`.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

struct Run {
    code: i32,
    out: String,
    err: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        out: String::from_utf8_lossy(&out.stdout).into_owned(),
        err: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn diff(file: &str, flags: &[&str], expected: &str) -> String {
    let dir = Scratch::new("ein-diff-models");
    let path = dir.join(format!("{}.models", file.replace('/', "_")));
    std::fs::write(&path, expected).expect("writes");
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    let p = path.to_string_lossy().into_owned();
    args.extend(["--diff-models", &p]);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{file}: {}", r.err);
    r.out
}

/// The three shapes, each on `branching/15` — whose maximal line is `{(p A)}`
/// and `{(p B), (p C)}` — against a file
/// written wrong; and the empty model, against the minimal line of a program
/// whose ∅ owes.
#[test]
fn the_diff_names_the_missing_the_spurious_and_the_one_fact_off() {
    const FILE: &str = "examples/branching/15_lookahead_two_step_on.ein";
    let out = diff(
        FILE,
        &["--maximal"],
        "closes p\nmodel\n(p A)\nmodel\n(p B)\n(p C)\nmodel\n(p C)\n(p D)\n",
    );
    assert!(
        out.contains("2 matched, 0 near, 1 missing, 0 spurious"),
        "{out}"
    );
    assert!(
        out.contains("- missing: model 3 of the file [(p C) (p D)]"),
        "{out}"
    );
    let out = diff(FILE, &["--maximal"], "closes p\nmodel\n(p A)\n");
    assert!(
        out.contains("1 matched, 0 near, 0 missing, 1 spurious"),
        "{out}"
    );
    assert!(out.contains("+ spurious: state 2 of 2"), "{out}");
    let out = diff(
        FILE,
        &["--maximal"],
        "closes p\nmodel\n(p A)\n(p B)\nmodel\n(p B)\n(p C)\n",
    );
    assert!(
        out.contains("1 matched, 1 near, 0 missing, 0 spurious"),
        "{out}"
    );
    assert!(out.contains("lacks (p B)"), "{out}");
    // The model with no fact, which no state of the minimal line is.
    let out = diff(
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        &["--minimal"],
        "closes seats\nmodel\n",
    );
    assert!(
        out.contains("- missing: model 1 of the file [no facts]"),
        "{out}"
    );
}

#[test]
fn a_file_the_diff_cannot_read_is_a_usage_error_before_anything_runs() {
    let dir = Scratch::new("ein-diff-models-bad");
    let bad = dir.join("bad.models");
    std::fs::write(&bad, "(p A)\n").expect("writes");
    for path in [
        bad.to_string_lossy().into_owned(),
        dir.join("absent").to_string_lossy().into_owned(),
    ] {
        let r = ein(&["solve", "examples/zebra2.ein", "--diff-models", &path]);
        assert_eq!(r.code, 2, "{path}: {}", r.err);
        assert!(r.err.contains("--diff-models"), "{}", r.err);
        assert!(r.out.is_empty(), "it ran anyway");
    }
}
