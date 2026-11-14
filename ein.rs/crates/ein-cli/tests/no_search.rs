//! **`--no-search`: root alone, and R not searched** — M1f
//! [S1f.6.36](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.36_no_search.md),
//! the user's words of 2026-11-14: *"flag that disables search doesn't change
//! R, it disables discovering of R. R is not an output, it is complete result
//! when we do full R search."*
//!
//! The command's half; `lever_invariance.rs` holds, over the corpus, that what
//! `--no-search` reports of root is what the full run reaches at root and that
//! it states no R. Here: which commands and options take the flag
//! (Q-M1f.258), what `ein solve` prints and writes under it (Q-M1f.255,
//! Q-M1f.259), and what `ein test` decides and exits with (Q-M1f.256,
//! Q-M1f.257).

use std::path::Path;
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

/// The value of the read-out row `label`, at the top level.
fn row<'a>(out: &'a str, label: &str) -> Option<&'a str> {
    out.lines().find_map(|l| {
        let rest = l.strip_prefix("  ")?.strip_prefix(label)?;
        rest.starts_with(' ').then(|| rest.trim())
    })
}

/// The rows that count or list R, which no `--no-search` run prints.
const R_ROWS: [&str; 11] = [
    "states (k_s)",
    "commitments (k_c)",
    "exhausted",
    "empty",
    "minimal line",
    "maximal line",
    "components",
    "sub-cubes",
    "merged",
    "N",
    "E",
];

/// **`ein oracle` refuses it, and so does every question flag of `ein
/// solve`**: the oracle is R by brute force, and each question is about R or
/// about the program. Usage errors, exit 2, before any load.
#[test]
fn the_oracle_and_the_question_flags_refuse_it() {
    let file = "tests/region/04_owing.ein";
    let r = ein(&["oracle", "--no-search", file]);
    assert_eq!(r.code, 2, "{}", r.err);
    assert!(
        r.err
            .starts_with("error: ein oracle computes R by brute force")
            && r.err
                .contains("`ein solve FILE --no-search` reports root alone"),
        "{}",
        r.err
    );
    for q in [
        &["-n", "1"][..],
        &["--single"],
        &["--minimal"],
        &["--maximal"],
        &["--count"],
        &["--monotone"],
    ] {
        let mut argv = vec!["solve", "--no-search"];
        argv.extend_from_slice(q);
        argv.push(file);
        let r = ein(&argv);
        assert_eq!(r.code, 2, "{argv:?}: {}", r.err);
        assert!(r.err.contains("--no-search"), "{argv:?}: {}", r.err);
        assert!(r.out.is_empty(), "{argv:?}: {}", r.out);
    }
}

/// **What `ein solve --no-search` prints**: root's standing as describe
/// prints it, the `R` row, no row of R's, and `route none`, on a root in R, an
/// owing root and a dead one; every run exits 0.
#[test]
fn solve_reports_root_and_says_r_was_not_searched() {
    for (file, standing) in [
        (
            "tests/region/01_states.ein",
            "in R — root is alive and owes nothing",
        ),
        (
            "tests/region/04_owing.ein",
            "not in R — root owes 1 instance",
        ),
        (
            "tests/region/06_false.ein",
            "dead — root's own fixpoint derives (false)",
        ),
    ] {
        let r = ein(&["solve", file, "--no-search"]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        assert_eq!(
            row(&r.out, "asked"),
            Some("root alone — --no-search: root's fixpoint, and R is not searched"),
            "{file}:\n{}",
            r.out
        );
        assert_eq!(row(&r.out, "∅"), Some(standing), "{file}:\n{}", r.out);
        assert!(
            row(&r.out, "R").is_some_and(|v| v.starts_with("not searched")),
            "{file}:\n{}",
            r.out
        );
        for label in R_ROWS {
            assert_eq!(row(&r.out, label), None, "{file}: {label}:\n{}", r.out);
        }
        assert!(
            row(&r.out, "route").is_some_and(|v| v.starts_with("none")),
            "{file}:\n{}",
            r.out
        );
        assert_eq!(
            row(&r.out, "work"),
            Some("1 saturation · 0 facts beyond root"),
            "{file}:\n{}",
            r.out
        );
    }
    // The owing root's instances, as describe names them.
    let r = ein(&["solve", "tests/region/04_owing.ein", "--no-search"]);
    assert!(
        r.out.contains("    owed            likes owes Ann a Food."),
        "{}",
        r.out
    );
}

/// **The summary and the answer directory**: `asked.ask` is `no-search`,
/// `region` and `verdict` are `null`, root's standing is the `root` block's,
/// and `--dump-answer` writes root's state as a loadable `root.ein`, beside
/// the read-out and the summary, and no state of R.
#[test]
fn the_summary_states_no_region_and_the_dump_writes_root() {
    let scratch = Scratch::new("ein-no-search-dump");
    let json = scratch.path().join("s.json");
    let dir = scratch.path().join("answer");
    let r = ein(&[
        "solve",
        "tests/region/04_owing.ein",
        "--no-search",
        "-p",
        "--json-summary",
        json.to_str().expect("utf-8"),
        "--dump-answer",
        dir.to_str().expect("utf-8"),
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    let s: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&json).expect("a summary"))
            .expect("the summary parses");
    assert_eq!(s["asked"]["ask"], "no-search");
    assert!(s["region"].is_null() && s["verdict"].is_null(), "{s}");
    assert_eq!(s["root"]["dead"], false);
    assert_eq!(s["root"]["owes"], 1);
    assert_eq!(s["root"]["owed"][0], "likes owes Ann a Food.");
    assert_eq!(s["claim"]["outcome"], "not-checked");
    let names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory")
        .map(|e| {
            e.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    for want in ["answer.txt", "answer.json", "root.ein", "forms"] {
        assert!(names.iter().any(|n| n == want), "{want} in {names:?}");
    }
    for not in ["models", "owing", "index.json", "nogoods.json"] {
        assert!(!names.iter().any(|n| n == not), "{not} in {names:?}");
    }
    assert_eq!(
        std::fs::read_to_string(dir.join("answer.json")).expect("answer.json"),
        std::fs::read_to_string(&json).expect("the summary")
    );
    let root = dir.join("root.ein");
    let text = std::fs::read_to_string(&root).expect("root.ein");
    assert!(
        text.contains(";;; root:       not in R — owes 1")
            && text.contains(";;; R:          not searched"),
        "{text}"
    );
    // Root's state loads, and its fixpoint owes what root owes.
    let again = ein(&["solve", root.to_str().expect("utf-8"), "--no-search"]);
    assert_eq!(again.code, 0, "{}", again.err);
    assert_eq!(
        row(&again.out, "∅"),
        Some("not in R — root owes 1 instance")
    );
}

/// **What `ein test --no-search` decides, and how it exits** (Q-M1f.256,
/// Q-M1f.257): a dead root decides `(false)`, as the full run does; a claim
/// root does not decide is NOT CHECKED and moves no exit code; a query whose
/// class at ∅ the claim does not list is FAILED, exit 1.
#[test]
fn test_decides_what_root_decides_and_a_claim_not_checked_exits_0() {
    let r = ein(&["test", "-v", "--no-search", "tests/region/06_false.ein"]);
    assert_eq!(r.code, 0, "{}{}", r.out, r.err);
    assert!(
        r.out
            .contains("solve · :expect (false) — holds (--no-search: root alone)"),
        "{}",
        r.out
    );
    for file in [
        "tests/region/01_states.ein",
        "tests/region/04_owing.ein",
        "tests/stdlib/algebra/24_total_owed_satisfied.ein",
    ] {
        let full = ein(&["test", file]);
        assert_eq!(full.code, 0, "{file}: {}", full.out);
        let r = ein(&["test", "--no-search", file]);
        assert_eq!(r.code, 0, "{file}: {}{}", r.out, r.err);
        assert!(r.out.starts_with("NOT CHECKED"), "{file}: {}", r.out);
        assert!(
            r.out.contains("--no-search reads root alone"),
            "{file}: {}",
            r.out
        );
    }
    // ∅ is in R and the query's claim lists another class.
    let scratch = Scratch::new("ein-no-search-test");
    let file = scratch.path().join("refuted.ein");
    std::fs::write(
        &file,
        "(relation at Color House :why \"{?1} is at {?2}\")\n\
         (is-a Red Color) (is-a Blue Color) (is-a H1 House) (is-a H2 House)\n\
         (query :goal (at ?c ?h) :expect (model (at Red H1)))\n",
    )
    .expect("written");
    let f = file.to_str().expect("utf-8");
    let r = ein(&["test", "--no-search", f]);
    assert_eq!(r.code, 1, "{}", r.out);
    assert!(
        r.out.starts_with("FAILED") && r.out.contains("root's state refutes the claim"),
        "{}",
        r.out
    );
    assert_eq!(ein(&["test", f]).code, 1);
    assert!(Path::new(f).exists());
}

/// **Under `(solve :no-search)` no commitment holds a fact**, so the
/// loader's checks of what a commitment could bind read none: a rule that
/// concludes `(not ?x)` with `?x` bound only by a declared relation the blind
/// enumerator would guess is refused without the keyword (M1f S1f.7.4, W84:
/// a commitment could give `?x` a name) and loads with it.
#[test]
fn no_commitment_binds_a_name_under_the_keyword() {
    let scratch = Scratch::new("ein-no-search-commitments");
    let program = "(relation r T :why \"{?1} is r\")\n\
                   (rule neg () :match (r ?x) :assert (not ?x) :why \"not {?x}\")\n";
    let without = scratch.path().join("without.ein");
    let with = scratch.path().join("with.ein");
    std::fs::write(&without, format!("{program}(solve)\n")).expect("written");
    std::fs::write(&with, format!("{program}(solve :no-search)\n")).expect("written");
    let r = ein(&["solve", without.to_str().expect("utf-8")]);
    assert_eq!(r.code, 1, "{}", r.err);
    assert!(r.err.contains("a commitment can hold a name"), "{}", r.err);
    let r = ein(&["solve", with.to_str().expect("utf-8")]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert_eq!(
        row(&r.out, "∅"),
        Some("in R — root is alive and owes nothing")
    );
}
