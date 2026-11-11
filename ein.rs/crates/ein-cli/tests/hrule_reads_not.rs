//! **An hrule that reads `not` gets one answer from both builds** — M1f
//! S1f.7.2 (W85, Q-M1f.129).
//!
//! Until S1f.7.2 `Hrules::candidates` held a `debug_assert!` that no hrule's
//! `:match` read `not`, *which hypgen's kill cache writes mid-enumeration*, so
//! a build with debug assertions stopped, exit 101, on a program a release
//! build answered, and answered as `ein oracle` does. No command enumerates
//! with the kill cache on since M1e2 S1e2.3.1, so the assertion guarded a
//! write that no shipped path makes; where an hrule reads `not`, every hrule
//! is now matched before any candidate is emitted, so the answer is the same
//! with the cache on, as the shape digest's `hyp` op has it.
//!
//! `cargo test` builds with debug assertions, so this file is the debug
//! build's run of the three shapes: a stored negation read under `(absent
//! …)`, read as a premise, and through `std.macro`'s `unknown`. Each is held
//! on every question `ein solve` takes, on `ein oracle` and on `ein test`,
//! whose `:expect` states the classes.

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

/// `states (k_s)      N` of a read-out.
fn states(out: &str) -> Option<usize> {
    out.lines()
        .find_map(|l| l.trim_start().strip_prefix("states (k_s)"))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|n| n.parse().ok())
}

/// The register's three witnesses, and the state each has beside ∅.
const WITNESSES: [(&str, &str); 3] = [
    ("hrule-reads-absent-not", "(model (paid Ann))"),
    ("hrule-reads-not", "(model (owes Bob))"),
    ("hrule-reads-unknown", "(model (paid Ann))"),
];

#[test]
fn an_hrule_that_reads_not_is_answered_as_the_oracle_answers() {
    let scratch = Scratch::new("ein-hrule-reads-not");
    let events = scratch.path().join("events.jsonl").display().to_string();
    let mut bad = Vec::new();
    for (stem, model) in WITNESSES {
        let f = format!("examples/ein-bugs/wrong-answers/{stem}.ein");
        let einb = scratch.path().join(format!("{stem}.einb"));
        let einb = einb.display().to_string();
        let runs: [Vec<&str>; 13] = [
            vec!["saturate", &f],
            vec!["solve", &f],
            vec!["solve", "--minimal", &f],
            vec!["solve", "--maximal", &f],
            vec!["solve", "--single", &f],
            vec!["solve", "--count", &f],
            vec!["solve", "-n", "1", &f],
            vec!["solve", "--events", &events, &f],
            vec!["oracle", &f],
            vec!["test", &f],
            vec!["render", "lattice", &f],
            vec!["kb", "save", &f, &einb],
            vec!["solve", &einb],
        ];
        for argv in runs {
            let r = ein(&argv);
            if r.code != 0 {
                bad.push(format!(
                    "{stem}: ein {argv:?}: exit {}: {}",
                    r.code,
                    r.err.lines().find(|l| !l.is_empty()).unwrap_or("")
                ));
            }
        }
        let (solve, oracle) = (ein(&["solve", &f]), ein(&["oracle", &f]));
        for (cmd, r) in [("solve", &solve), ("oracle", &oracle)] {
            if states(&r.out) != Some(2) || !r.out.contains(model) {
                bad.push(format!(
                    "{stem}: ein {cmd}: {:?} states, or no {model}: want ∅ and the one candidate",
                    states(&r.out)
                ));
            }
        }
    }
    // The fuzzer's finding of 2026-08-21, the program the assertion was first
    // met on: its hrule matches nothing, and the run answers.
    let fuzz = ein(&["solve", "corpus/fuzz_findings/hrule-reads-not.ein"]);
    if fuzz.code != 0 {
        bad.push(format!(
            "corpus/fuzz_findings/hrule-reads-not.ein: exit {}: {}",
            fuzz.code,
            fuzz.err.lines().find(|l| !l.is_empty()).unwrap_or("")
        ));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// Two hrules, of which the second reads the negation the first could be
/// asked about: both are matched against the state as it was asked, so the
/// second proposes of Bob alone, and `ein solve` counts what `ein oracle`
/// counts.
#[test]
fn every_hrule_reads_the_state_it_was_asked_about() {
    let scratch = Scratch::new("ein-hrule-reads-not-two");
    let file = scratch.path().join("two.ein");
    std::fs::write(
        &file,
        "(relation paid Guest :why \"{?1} has paid\")\n\
         (relation owes Guest :why \"{?1} owes\")\n\
         (is-a Ann Guest) (is-a Bob Guest)\n\
         (not (paid Bob))\n\
         (hrule suppose () :match (and (is-a ?g Guest) (absent (not (paid ?g)))) \
         :assert (paid ?g) :why \"suppose {?g} has paid\")\n\
         (hrule charge () :match (and (is-a ?g Guest) (not (paid ?g))) \
         :assert (owes ?g) :why \"suppose {?g} owes\")\n\
         (query :goal (owes ?g))\n",
    )
    .expect("written");
    let f = file.display().to_string();
    let (solve, oracle) = (ein(&["solve", &f]), ein(&["oracle", &f]));
    assert_eq!(solve.code, 0, "{}", solve.err);
    assert_eq!(oracle.code, 0, "{}", oracle.err);
    // H is (paid Ann) and (owes Bob), independent: four states.
    assert_eq!(states(&solve.out), Some(4), "{}", solve.out);
    assert_eq!(states(&oracle.out), Some(4), "{}", oracle.out);
    assert!(solve.out.contains("2 candidates"), "{}", solve.out);
}
