//! **Arithmetic is computed and never guessed** — the search-free test of
//! M1f P1f.6, which
//! [S1f.6.3](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md)
//! writes and every later operation joins.
//!
//! The user's instruction of 2026-11-08: *"solving simple arithmetic will
//! utilize only saturation with hardcoded … tables like it is done in
//! electronics, so search will not be involved at all"*. In Ein's terms, on
//! every arithmetic program:
//!
//! | claim | read off |
//! |---|---|
//! | H is empty: the universe has no candidate, and nothing is left outside it | `--json-summary`'s `region.universe` and `root.hypgen` |
//! | k_s is 1 and R is {∅}: one state, root itself, alive and owing nothing | `region.k_s`, `k_c`, `root`, `states` |
//! | one saturation, no fork | `work.saturations` |
//! | monotone: no `absent` read, one stratum | `monotonicity`, and `ein saturate --dump`'s `;; Strata` line |
//! | **the same answer under `ein solve`, `ein saturate` and `ein test`** | the result facts of the solve's one state file, the saturation's dump, and the claims `ein test` holds |
//!
//! **Which programs.** Every `.ein` under [`ROOTS`]: the theory's tests and
//! its examples. A program joins by being put there, and an operation by
//! adding its result relation to [`RESULTS`]; nothing else here names a file.
//!
//! **A program that divides by zero** (S1f.6.6) is the one kind whose R is
//! not {∅}: it claims [`DEAD`], and [`division_by_zero`] holds what it is
//! held to instead, an empty R decided by root alone.
//!
//! What this file does not ask is whether the answers are *right*: that is
//! `ein-infer/tests/arith_theory.rs`, against Rust's integers.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, ein_files_under, repo_root};

/// Where the arithmetic programs live.
const ROOTS: [&str; 2] = ["tests/stdlib/arith", "examples/arith"];

/// The relations an answer is a fact of.
const RESULTS: [&str; 5] = ["sum-of", "diff-of", "cmp-of", "product-of", "quot-of"];

/// What a program that divides by zero claims: root is dead and R is empty.
const DEAD: &str = "(solve :expect (false))";

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn programs() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ROOTS {
        let found = ein_files_under(&root.join(dir));
        assert!(!found.is_empty(), "no program under {dir}");
        out.extend(found);
    }
    out
}

/// The result facts in a text that writes one fact a line: the line's
/// leading form, when its head is one of [`RESULTS`]. `cut` removes what the
/// surface writes after the fact's own arguments.
fn results(text: &str, cut: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let line = line.trim();
        if !RESULTS.iter().any(|r| line.starts_with(&format!("({r} "))) {
            continue;
        }
        let fact = line.find(cut).map_or(line, |i| &line[..i]);
        out.insert(balanced(fact));
    }
    out
}

/// A form's text closed at its own last parenthesis: the fact with exactly
/// the parentheses it opens.
fn balanced(text: &str) -> String {
    let mut depth = 0i32;
    for (i, ch) in text.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return text[..=i].to_string();
                }
            }
            _ => {}
        }
    }
    let mut out = text.to_string();
    for _ in 0..depth {
        out.push(')');
    }
    out
}

/// **A program that divides by zero is search-free too**, and dead: no
/// candidate, one saturation and no fork, and no state at all, since root's
/// own fixpoint derives `(false)`. The core is the request and nothing
/// else, `refuted by` names the rule that concludes `(false)` from the flag
/// (the user's ruling of 2026-11-11: two steps), `ein saturate` shows the
/// flag and `(false)` in root's state, each by its own rule, and `ein test`
/// holds the one claim such a program can make.
fn division_by_zero(file: &str, json: &serde_json::Value, dump: &Path) {
    let region = &json["region"];
    assert_eq!(region["k_s"], 0, "{file}: k_s");
    assert_eq!(region["k_c"], 0, "{file}: k_c");
    assert_eq!(region["exhausted"], true, "{file}");
    assert_eq!(region["root"], "dead", "{file}: root derives (false)");
    assert_eq!(
        region["states"],
        serde_json::json!([]),
        "{file}: R is empty"
    );
    assert_eq!(json["work"]["saturations"], 1, "{file}: one saturation");
    assert_eq!(json["monotonicity"]["monotone"], true, "{file}");
    // The core is the request, and what its divisor's zero rests on: nothing
    // where the divisor is written `z`; the `let` where it is a name; and the
    // rows of `md` that put a zero written with digits into normal form, since
    // a table row is a premise like any other fact (S1f.6.2's measured cost).
    let core: Vec<&str> = region["core"]
        .as_array()
        .expect("root's core")
        .iter()
        .map(|f| f.as_str().expect("a fact"))
        .collect();
    assert_eq!(
        core.iter().filter(|f| f.starts_with("(div ")).count(),
        1,
        "{file}: the core names the request: {core:?}"
    );
    assert!(
        core.iter()
            .all(|f| ["(div ", "(let ", "(md "].iter().any(|h| f.starts_with(h))),
        "{file}: the core is the request, a `let` and rows of `md`: {core:?}"
    );
    assert_eq!(
        region["refuted_by"],
        serde_json::json!(["division-by-zero-is-false"]),
        "{file}"
    );
    assert!(
        !dump.join("models").exists()
            || std::fs::read_dir(dump.join("models")).is_ok_and(|mut d| d.next().is_none()),
        "{file}: a dead root has no state file"
    );

    let saturate = ein(&["saturate", "--dump", file]);
    assert_eq!(
        saturate.code, 0,
        "{file}: ein saturate\n{}",
        saturate.stderr
    );
    let fact = |start: &str, rule: &str| {
        saturate.stdout.lines().any(|l| {
            let l = l.trim();
            l.starts_with(start) && l.ends_with(&format!(":rule {rule})"))
        })
    };
    assert!(
        fact("(division-by-zero (div ", "div-by-zero"),
        "{file}: the flag, by its own rule"
    );
    assert!(
        fact("(false", "division-by-zero-is-false"),
        "{file}: (false), by the second rule"
    );
    assert!(
        !saturate
            .stdout
            .lines()
            .any(|l| l.trim().starts_with("(quot-of ")),
        "{file}: a division by zero is never answered with a number"
    );

    let test = ein(&["test", file]);
    assert_eq!(
        test.code, 0,
        "{file}: ein test\n{}{}",
        test.stdout, test.stderr
    );
    assert!(
        test.stdout.contains("0 FAILED, 0 not checked, 0 errors"),
        "{file}: {}",
        test.stdout
    );
}

fn search_free(path: &Path, scratch: &Scratch, dead: bool) {
    let root = repo_root();
    let file = path
        .strip_prefix(&root)
        .unwrap_or(path)
        .to_str()
        .expect("utf-8");
    let stem = path.file_stem().expect("a stem").to_string_lossy();
    let summary = scratch.join(format!("{stem}.json"));
    let dump = scratch.join(format!("{stem}.answer"));

    // ── ein solve ──
    let solve = ein(&[
        "solve",
        file,
        "--json-summary",
        summary.to_str().expect("utf-8"),
        "--dump-answer",
        dump.to_str().expect("utf-8"),
    ]);
    assert_eq!(solve.code, 0, "{file}: ein solve\n{}", solve.stderr);
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&summary).expect("the summary"))
            .expect("the summary is JSON");
    let region = &json["region"];
    assert_eq!(
        region["universe"]["candidates"], 0,
        "{file}: H is not empty — a relation of the program is not closed"
    );
    assert_eq!(
        json["root"]["hypgen"]["outside"],
        serde_json::json!([]),
        "{file}: something is left outside the universe"
    );
    assert_eq!(json["root"]["hypgen"]["emitted"], 0, "{file}");
    if dead {
        division_by_zero(file, &json, &dump);
        return;
    }
    assert_eq!(region["k_s"], 1, "{file}: k_s");
    assert_eq!(region["k_c"], 1, "{file}: k_c");
    assert_eq!(region["exhausted"], true, "{file}");
    assert_eq!(
        region["root"], "in-r",
        "{file}: root is alive and owes nothing"
    );
    let states = region["states"].as_array().expect("the states");
    assert_eq!(states.len(), 1, "{file}: R has one state");
    assert_eq!(
        states[0]["facts"],
        serde_json::json!([]),
        "{file}: the one state is ∅, root itself"
    );
    assert_eq!(
        json["work"]["saturations"], 1,
        "{file}: one saturation, no fork"
    );
    let mono = &json["monotonicity"];
    assert_eq!(mono["monotone"], true, "{file}");
    for kind in ["static", "dynamic_monotone", "dynamic_non_monotone"] {
        assert_eq!(
            mono["absent_reads"][kind], 0,
            "{file}: the theory reads an `absent` ({kind})"
        );
    }

    // ── ein saturate ──
    let saturate = ein(&["saturate", "--dump", file]);
    assert_eq!(
        saturate.code, 0,
        "{file}: ein saturate\n{}",
        saturate.stderr
    );
    assert!(
        saturate
            .stdout
            .lines()
            .any(|l| l.trim() == ";; Strata — stratified, 1 stratum"),
        "{file}: not one stratum"
    );

    // ── the same answer ──
    let state = std::fs::read_to_string(dump.join("models/01.ein")).expect("the one state's file");
    let from_solve = results(&state, " ; ");
    let from_saturate = results(&saturate.stdout, " :rule ");
    assert!(!from_solve.is_empty(), "{file}: no answer in the state");
    assert_eq!(
        from_solve, from_saturate,
        "{file}: `ein solve` and `ein saturate` hold different answers"
    );

    // ── ein test ──
    let test = ein(&["test", file]);
    assert_eq!(
        test.code, 0,
        "{file}: ein test\n{}{}",
        test.stdout, test.stderr
    );
    assert!(
        test.stdout.contains("0 FAILED, 0 not checked, 0 errors"),
        "{file}: {}",
        test.stdout
    );
}

#[test]
fn every_arithmetic_program_is_search_free() {
    let scratch = Scratch::new("ein-arith-search-free");
    let programs = programs();
    for path in &programs {
        let text = std::fs::read_to_string(path).expect("readable");
        assert!(
            text.contains("(import std.arith "),
            "{}: under an arithmetic root and does not import std.arith",
            path.display()
        );
        search_free(path, &scratch, text.contains(DEAD));
    }
}

/// `utils/gen_arith_examples.py --check`, run here so that it is in the
/// gate (M1f S1f.6.15): `examples/arith/long_numbers.ein`,
/// `many_calculations.ein` and `factorial.ein` are generated, operands drawn
/// with a fixed seed and every claimed answer computed by Python's integers.
/// The test above then holds each of them search-free, and `ein test` holds
/// the claims to the engine's own answers. The per-commit tier installs a
/// `python3`, as for `utils/gen_arith_tables.py --check`
/// (`ein-ir/tests/digit_tables.rs`, Q-M1f.145).
#[test]
fn the_generated_examples_are_what_the_generator_writes() {
    let probe = Command::new("python3")
        .args(["-c", "pass"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    assert!(
        matches!(probe, Ok(s) if s.success()),
        "`python3` does not run ({probe:?}): three programs of `examples/arith/` are generated \
         and `utils/gen_arith_examples.py --check` is the only check that compares their \
         bytes with the script's — a missing gate, not a missing convenience"
    );
    let out = Command::new("python3")
        .args(["utils/gen_arith_examples.py", "--check"])
        .current_dir(repo_root())
        .output()
        .expect("python3 ran a moment ago");
    assert!(
        out.status.success(),
        "a generated example is stale — run `python3 utils/gen_arith_examples.py`\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}
