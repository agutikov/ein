//! **The loud warning on a program that is not monotone**
//! ([Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md),
//! [08-read-out.md § 5.1](../../../../docs/ein/user/04-reference/08-read-out.md#51-under-lattice-and-monotone-the-warning)).
//!
//! A program the static check flags loads and is answered, and every command
//! that answers about a commitment says, in capitals, that the program is
//! not monotone and that nothing is claimed of the answer: the read-out's
//! first row, `WARNING`; the first line on stderr; and the summary's
//! `monotonicity.warning`. A monotone program gets none of the three.
//!
//! What is held here is the warning, on every corpus program. What a flagged
//! program answers is held nowhere.

use std::process::Command;

use ein_corpus::{Scratch, corpus_files, repo_root};
use ein_infer::detector::{WARNING, warning_line};

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

/// The read-out's `WARNING` row: its first line, as printed.
fn row() -> String {
    format!("  WARNING           {WARNING}.")
}

/// Whether `out`, a read-out, opens with the warning: the header, the rule,
/// then the `WARNING` row before `asked`.
fn opens_with_the_warning(out: &str) -> bool {
    let lines: Vec<&str> = out.lines().collect();
    lines.get(2) == Some(&row().as_str())
        && lines
            .iter()
            .position(|l| l.starts_with("  asked "))
            .is_some_and(|at| at > 2)
}

/// **On every corpus program, flagged exactly when warned**: under
/// `--no-search`, which reads root alone, the read-out opens with the
/// `WARNING` row, stderr's first line is the warning's, and the summary's
/// `monotonicity.warning` is the sentence, each exactly where the summary's
/// `monotonicity.monotone` is `false`; and a monotone program has none of the
/// three. Every answer exits 0.
#[test]
fn every_flagged_corpus_program_is_warned_of_and_no_monotone_one() {
    let dir = Scratch::new("ein-non-monotone-warning");
    let root = repo_root();
    let files: Vec<String> = corpus_files()
        .iter()
        .map(|p| p.strip_prefix(&root).unwrap_or(p).display().to_string())
        .collect();
    let indexed: Vec<(usize, &String)> = files.iter().enumerate().collect();
    let found = ein_corpus::pool::map(
        "non_monotone_warning",
        &indexed,
        |(_, file)| ((*file).clone(), "--no-search".to_string()),
        |&(i, file)| {
            let summary = dir.join(format!("{i}.json"));
            let r = ein(&[
                "solve",
                file,
                "--no-search",
                "--json-summary",
                summary.to_str().expect("utf-8"),
            ]);
            // A program that does not load writes no summary: nothing to hold.
            let Ok(text) = std::fs::read_to_string(&summary) else {
                return (None, Vec::new());
            };
            let s: serde_json::Value = serde_json::from_str(&text).expect("the summary parses");
            let flagged = s["monotonicity"]["monotone"] == false;
            let mut bad = Vec::new();
            if r.code != 0 && !r.err.contains(":expect") {
                bad.push(format!("{file}: exit {}: {}", r.code, r.err));
            }
            let first = r.err.lines().next().unwrap_or("");
            if flagged {
                if !opens_with_the_warning(&r.out) {
                    bad.push(format!(
                        "{file}: flagged, and no WARNING row first:\n{}",
                        r.out
                    ));
                }
                if first != warning_line(file) {
                    bad.push(format!("{file}: flagged, and stderr opens with {first:?}"));
                }
                if s["monotonicity"]["warning"] != WARNING {
                    bad.push(format!(
                        "{file}: flagged, and monotonicity.warning is {}",
                        s["monotonicity"]["warning"]
                    ));
                }
                if !r.out.contains("\n  lattice           not monotone") {
                    bad.push(format!(
                        "{file}: flagged, and no `lattice  not monotone` row"
                    ));
                }
            } else {
                if r.out.contains("WARNING") || r.err.contains("WARNING") {
                    bad.push(format!("{file}: monotone, and warned of"));
                }
                if !s["monotonicity"]["warning"].is_null() {
                    bad.push(format!("{file}: monotone, and monotonicity.warning is set"));
                }
            }
            (Some(flagged), bad)
        },
    );
    let mut bad = Vec::new();
    let (mut flagged, mut monotone) = (0usize, 0usize);
    for (verdict, b) in found {
        bad.extend(b);
        match verdict {
            Some(true) => flagged += 1,
            Some(false) => monotone += 1,
            None => {}
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // Floors under what the sweep finds: one that read no flagged program
    // would pass the loop above.
    assert!(
        flagged >= 40 && monotone >= 500,
        "{flagged} flagged, {monotone} monotone"
    );
}

/// **Every question, and every command that answers about a commitment**: on
/// the probe of a refutation resting on an `absent`, a bare `ein solve`,
/// `--minimal`, `--maximal`, `--single`, `-n 2`, `--count`, `--no-search` and
/// `ein oracle` each open with the row and write the line first on stderr;
/// `--monotone`, whose answer is the verdict itself, prints neither; `ein
/// show hyps` keeps stdout to its facts and writes the line; `ein render
/// lattice` keeps stdout to its drawing and writes the line; and `ein test`
/// writes it once for a flagged file whose claim it checks.
#[test]
fn every_command_that_answers_about_a_commitment_warns() {
    const PROBE: &str = "examples/ein-bugs/naf-upward-closure.ein";
    let line = warning_line(PROBE);
    for flags in [
        &[][..],
        &["--minimal"],
        &["--maximal"],
        &["--single"],
        &["-n", "2"],
        &["--count"],
        &["--no-search"],
    ] {
        let mut args = vec!["solve", PROBE];
        args.extend_from_slice(flags);
        let r = ein(&args);
        assert_eq!(r.code, 0, "{flags:?}: {}", r.err);
        assert!(opens_with_the_warning(&r.out), "{flags:?}:\n{}", r.out);
        assert_eq!(r.err.lines().next(), Some(line.as_str()), "{flags:?}");
        // Its two further lines, under the row.
        assert!(
            r.out
                .contains("IT IS NOT CLAIMED TO BE RIGHT, SOUND OR COMPLETE (Q-M1f.564)."),
            "{flags:?}:\n{}",
            r.out
        );
    }
    let oracle = ein(&["oracle", PROBE]);
    assert_eq!(oracle.code, 0, "{}", oracle.err);
    assert!(opens_with_the_warning(&oracle.out), "{}", oracle.out);
    assert_eq!(oracle.err.lines().next(), Some(line.as_str()));

    let asked = ein(&["solve", PROBE, "--monotone"]);
    assert_eq!(asked.code, 0, "{}", asked.err);
    assert!(!asked.out.contains("WARNING"), "{}", asked.out);
    assert_eq!(asked.err, "");

    let hyps = ein(&["show", "hyps", PROBE]);
    assert_eq!(hyps.code, 0, "{}", hyps.err);
    assert!(!hyps.out.contains("WARNING"), "{}", hyps.out);
    assert!(hyps.out.lines().all(|l| l.starts_with('(')), "{}", hyps.out);
    assert_eq!(hyps.err.lines().next(), Some(line.as_str()));

    let lattice = ein(&["render", "lattice", PROBE]);
    assert_eq!(lattice.code, 0, "{}", lattice.err);
    assert!(lattice.out.starts_with("digraph"), "{}", lattice.out);
    assert!(!lattice.out.contains("WARNING"), "{}", lattice.out);
    assert_eq!(lattice.err.lines().next(), Some(line.as_str()));

    // `ein test` checks a flagged file's claim when asked, and says what it
    // is checking: the claim's outcome is not held here.
    const CLAIMS: &str = "examples/lattice/17_flagged_region_claim.ein";
    let test = ein(&["test", CLAIMS]);
    assert_eq!(
        test.err
            .lines()
            .filter(|l| l.starts_with("WARNING: "))
            .count(),
        1,
        "{}",
        test.err
    );
    assert_eq!(test.err.lines().next(), Some(warning_line(CLAIMS).as_str()));

    // And a monotone program is warned of nowhere.
    const MONOTONE: &str = "examples/lattice/02_genuine_3set_death.ein";
    for args in [
        &["solve", MONOTONE][..],
        &["oracle", MONOTONE],
        &["show", "hyps", MONOTONE],
        &["render", "lattice", MONOTONE],
    ] {
        let r = ein(args);
        assert_eq!(r.code, 0, "{args:?}: {}", r.err);
        assert!(
            !r.out.contains("WARNING") && !r.err.contains("WARNING"),
            "{args:?}"
        );
    }
}

/// **The warning is stderr's first line before a load's `note:`**: a
/// flagged program whose load notes something, a rule that concludes a
/// hypothesis beside `(solve :no-search)` and a rule that reads `open`,
/// writes the warning first and the note after it, on every command that
/// answers about a commitment. Only the order of the lines is held; what the
/// program answers is not.
#[test]
fn the_warning_comes_before_a_loads_note() {
    let dir = Scratch::new("ein-warning-before-note");
    let file = dir.join("noted.ein");
    std::fs::write(
        &file,
        "(relation s Thing :why \"s holds of {?3}\")\n\
         (relation seen Fact :why \"{?3} is supposed\")\n\
         (thing a)\n\
         (rule guess (?R) :match (thing ?x) :assert (open (?R ?x)) :why \"suppose {?R} of {?x}\")\n\
         (guess s)\n\
         (rule noted () :match (open ?f) :assert (seen ?f) :why \"{?f} is supposed\")\n\
         (solve :no-search)\n",
    )
    .expect("written");
    let f = file.to_str().expect("utf-8");
    let line = warning_line(f);
    for args in [
        &["solve", f][..],
        &["solve", f, "--no-search"],
        &["oracle", f],
        &["show", "hyps", f],
        &["render", "lattice", f],
    ] {
        let r = ein(args);
        assert_eq!(r.code, 0, "{args:?}: {}", r.err);
        let lines: Vec<&str> = r.err.lines().collect();
        assert_eq!(lines.first(), Some(&line.as_str()), "{args:?}:\n{}", r.err);
        let note = lines
            .iter()
            .position(|l| l.starts_with("note: ") && l.contains("solve: its :no-search"));
        assert!(note.is_some_and(|at| at > 0), "{args:?}:\n{}", r.err);
    }
}
