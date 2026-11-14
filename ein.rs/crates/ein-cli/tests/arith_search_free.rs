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
//! | monotone: **one `absent` read where the program imports `std.math.decimal-arith`**, statically, over `let`, which nothing derives, and none dynamically; two strata there, and none and one elsewhere (S1f.6.12) | `monotonicity`, and `ein saturate --dump`'s `;; Strata` line |
//! | **the same answer under `ein solve`, `ein saturate` and `ein test`** | the result facts of the solve's one state file, the saturation's dump, and the claims `ein test` holds |
//!
//! **Which programs.** Every `.ein` under [`ROOTS`]: the tests of the
//! theories, `std.math.decimal-arith`, `std.math.num9` (S1f.6.13), `std.math.int` (S1f.6.7) and
//! `std.math.rat` (S1f.6.8), and their examples. A
//! program joins by being put there, and an operation by adding its result
//! relation to [`RESULTS`]; nothing else here names a file.
//!
//! **A program that divides by zero** (S1f.6.6) is one kind whose R is not
//! {∅}: it claims [`DEAD`], and [`dead`] holds what it is held to instead,
//! an empty R decided by root alone. A rational with a denominator of zero
//! is such a division (S1f.6.8). **A malformed program is the other kind**
//! (S1f.6.12): a request the theory cannot read, a `let` it cannot read, or
//! a name no `let` states, each a flag and `(false)` from it, held as a
//! division by zero is ([`FLAGS`]).
//!
//! What this file does not ask is whether the answers are *right*: that is
//! `ein-infer/tests/arith_theory.rs`, against Rust's integers.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, ein_files_under, repo_root};

/// Where the arithmetic programs live: `std.math.decimal-arith`'s tests, `std.math.num9`'s
/// (S1f.6.13), `std.math.int`'s (S1f.6.7), `std.math.rat`'s (S1f.6.8), and the examples
/// of all.
const ROOTS: [&str; 5] = [
    "tests/stdlib/math/decimal-arith",
    "tests/stdlib/math/num9",
    "tests/stdlib/math/int",
    "tests/stdlib/math/rat",
    "examples/arith",
];

/// The relations an answer is a fact of: `std.math.decimal-arith`'s five results, and
/// `std.math.num9`'s five, its overflow flag and its conversions' answers, and
/// `std.math.int`'s six and `std.math.rat`'s seven; and `div-done`, the hand-over of
/// `std.math.decimal-arith`'s tagged division, which is the answer a rule of a program's
/// own reads where it calls the division itself (S1f.6.24's `41`).
const RESULTS: [&str; 31] = [
    "div-done",
    "sum-of",
    "diff-of",
    "cmp-of",
    "product-of",
    "quot-of",
    "sum9-of",
    "diff9-of",
    "cmp9-of",
    "product9-of",
    "quot9-of",
    "overflow",
    "as9",
    "big",
    "of9",
    "int-simple-of",
    "int-neg-of",
    "int-sum-of",
    "int-diff-of",
    "int-product-of",
    "int-cmp-of",
    "int-quot-euclid-of",
    "int-quot-trunc-of",
    "int-quot-floor-of",
    "rat-simple-of",
    "rat-neg-of",
    "rat-sum-of",
    "rat-diff-of",
    "rat-product-of",
    "rat-quot-of",
    "rat-cmp-of",
];

/// The theories: a program under [`ROOTS`] imports one, or several.
const THEORIES: [&str; 4] = [
    "(import std.math.decimal-arith ",
    "(import std.math.num9 ",
    "(import std.math.int ",
    "(import std.math.rat ",
];

/// What a division by zero can come from, by the start of its text: a dead
/// program's core names exactly one. The four kinds of request, and, since
/// S1f.6.24 flags a division at the call whoever made it, the fact a rule of
/// the program's own makes its tagged call from: `(share A B)`, of
/// `tests/stdlib/math/decimal-arith/39_div_call_by_zero.ein` and `40`.
const DIVIDES: [&str; 5] = ["(div ", "(div9 ", "(rat-", "(int-div-", "(share "];

/// What else such a core may hold: the `let` of a named operand, and the
/// table rows a zero written with digits was read through, `md` where
/// `std.math.decimal-arith` puts a divisor into normal form, `cl` where `std.math.rat`
/// compares a denominator with zero, `fs` where `std.math.int` simplifies a
/// divisor's numerator, or both operands of a division of integers, and `nz`
/// where that simplification drops no high zero.
const RESTS_ON: [&str; 5] = ["(let ", "(md ", "(cl ", "(fs ", "(nz "];

/// The requests and the `let` a malformed program's flag can name, by the
/// start of their text (S1f.6.12): `std.math.decimal-arith`'s five requests.
const READS: [&str; 6] = ["(add ", "(sub ", "(cmp ", "(mul ", "(div ", "(let "];

/// The three ways an arithmetic program is dead, by the rule that concludes
/// `(false)`: the start of its flag's text, and what the flag names.
const FLAGS: [(&str, &str, &[&str]); 3] = [
    ("division-by-zero-is-false", "(division-by-zero ", &DIVIDES),
    ("malformed-is-false", "(malformed ", &READS),
    ("undeclared-is-false", "(undeclared ", &READS),
];

/// The rules that derive a malformed program's flag.
const MALFORMED_BY: [&str; 6] = [
    "bad-request-malformed",
    "bad-request-undeclared",
    "let-twice",
    "let-zero",
    "let-digit",
    "let-digit-zero",
];

/// The rules that derive the flag, each with the start of the flag it
/// derives. Since S1f.6.24 the first is the flag of a call, `std.math.decimal-arith`'s
/// `(div-call TAG X Y)` with a divisor that is zero, whoever asserted it; a
/// public `(div X Y)` has that flag beside its own, and the modules' rules
/// below flag their request before any call is made, so theirs is one flag.
const FLAGGERS: [(&str, &str); 8] = [
    ("(division-by-zero (div-call ", "div-call-by-zero"),
    ("(division-by-zero (int-div-", "int-div-by-zero"),
    ("(division-by-zero (div ", "div-by-zero"),
    ("(division-by-zero (div9 ", "div9-by-zero"),
    ("(division-by-zero (rat-", "rat-div-zero"),
    ("(division-by-zero (rat-", "rat-zero-only"),
    ("(division-by-zero (rat-", "rat-zero-first"),
    ("(division-by-zero (rat-", "rat-zero-second"),
];

/// The relations a division's answer is a fact of: a dead program stores
/// none for the request that killed it. A program of rationals is asked one
/// other request, a sum, so that its state is seen to be computed.
const QUOTIENTS: [&str; 6] = [
    "(quot-of ",
    "(quot9-of ",
    "(rat-quot-of ",
    "(int-quot-euclid-of ",
    "(int-quot-trunc-of ",
    "(int-quot-floor-of ",
];

/// What a program that divides by zero claims: root is dead and R is empty.
const DEAD: &str = "(solve :expect (false))";

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

/// A run, read in short names: a name of the calculator a program does not
/// list prints by its full name since M1f S1f.6.35, `std.math.decimal-arith.strip`, and
/// this test reads every one as the module writes it.
fn ein(args: &[&str]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: ein_corpus::unprefixed(&String::from_utf8_lossy(&out.stdout)),
        stderr: ein_corpus::unprefixed(&String::from_utf8_lossy(&out.stderr)),
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
///
/// **A malformed program is held the same way** (S1f.6.12): `refuted by`
/// names `malformed-is-false` or `undeclared-is-false`; the core names what
/// the flag names, a request or a `let`, and beside it only `let`s and
/// table rows; and the request the flag names is never answered.
fn dead(file: &str, json: &serde_json::Value, dump: &Path) {
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
    let refuted_by = region["refuted_by"]
        .as_array()
        .and_then(|r| match r.as_slice() {
            [one] => one.as_str(),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{file}: refuted by one rule: {}", region["refuted_by"]));
    let (_, flag, names) = FLAGS
        .iter()
        .find(|(rule, _, _)| *rule == refuted_by)
        .unwrap_or_else(|| panic!("{file}: refuted by {refuted_by}, which concludes no flag"));
    let division = *flag == "(division-by-zero ";
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
    if division {
        assert_eq!(
            core.iter()
                .filter(|f| DIVIDES.iter().any(|h| f.starts_with(h)))
                .count(),
            1,
            "{file}: the core names the request: {core:?}"
        );
    } else {
        // A malformed program's core: the request or the `let` its flag
        // names, and it names at least one.
        assert!(
            core.iter().any(|f| names.iter().any(|h| f.starts_with(h))),
            "{file}: the core names what the flag names: {core:?}"
        );
    }
    assert!(
        core.iter()
            .all(|f| names.iter().chain(&RESTS_ON).any(|h| f.starts_with(h))),
        "{file}: the core is the request, a `let` and table rows: {core:?}"
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
        fact("(false", refuted_by),
        "{file}: (false), by the second rule"
    );
    if division {
        // Every flag in root's state is by the rule that derives its shape.
        let flags: Vec<&str> = saturate
            .stdout
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with(flag))
            .collect();
        assert!(!flags.is_empty(), "{file}: no flag in root's state");
        for line in &flags {
            assert!(
                FLAGGERS.iter().any(|(start, rule)| line.starts_with(start)
                    && line.ends_with(&format!(":rule {rule})"))),
                "{file}: the flag, by its own rule: {line}"
            );
        }
        // Where the flag comes from (S1f.6.24). A public `(div X Y)` is
        // flagged twice, the request and its call under `std.math.decimal-arith`'s tag; a
        // call a rule of the program's own made is flagged once, the call;
        // and a module that sees the zero before it calls flags its request
        // and no call.
        let calls = flags
            .iter()
            .filter(|l| l.starts_with("(division-by-zero (div-call "))
            .count();
        let named = core
            .iter()
            .find(|f| DIVIDES.iter().any(|h| f.starts_with(h)))
            .expect("the core names what divided");
        let (want_calls, want_flags) = if named.starts_with("(div ") {
            assert!(
                flags
                    .iter()
                    .any(|l| l.starts_with("(division-by-zero (div-call (quotient ")),
                "{file}: a request's call is under the tag (quotient X Y)"
            );
            (1, 2)
        } else if named.starts_with("(share ") {
            (1, 1)
        } else {
            (0, 1)
        };
        assert_eq!(
            (calls, flags.len()),
            (want_calls, want_flags),
            "{file}: the flags of one division by zero: {flags:?}"
        );
        assert!(
            !saturate
                .stdout
                .lines()
                .any(|l| QUOTIENTS.iter().any(|q| l.trim().starts_with(q))),
            "{file}: a division by zero is never answered with a number"
        );
    } else {
        // The flags, each by one of the rules that derive one, and the
        // request each names: `(malformed (add X Y) :rule …)` names
        // `(add X Y)`, and `(undeclared NAME (add X Y) :rule …)` the same.
        let flags: Vec<&str> = saturate
            .stdout
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with(flag))
            .collect();
        assert!(!flags.is_empty(), "{file}: no flag in root's state");
        for line in &flags {
            assert!(
                MALFORMED_BY
                    .iter()
                    .any(|rule| line.ends_with(&format!(":rule {rule})"))),
                "{file}: the flag, by its own rule: {line}"
            );
            let inner = &line[flag.len()..];
            let named = READS
                .iter()
                .filter_map(|h| inner.find(h))
                .min()
                .map(|i| balanced(&inner[i..]))
                .unwrap_or_else(|| panic!("{file}: the flag names a request or a `let`: {line}"));
            // A flagged request is never answered: no result fact repeats
            // its operands.
            if let Some(operands) = named
                .strip_suffix(')')
                .and_then(|r| r.split_once(' '))
                .map(|(_, o)| o)
                .filter(|_| !named.starts_with("(let "))
            {
                assert!(
                    !saturate.stdout.lines().map(str::trim).any(|l| RESULTS
                        .iter()
                        .any(|r| l.starts_with(&format!("({r} {operands} ")))),
                    "{file}: {named} is flagged and answered"
                );
            }
        }
    }

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

fn search_free(path: &Path, text: &str, scratch: &Scratch, dead: bool) {
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
    let json: serde_json::Value = serde_json::from_str(&ein_corpus::unprefixed(
        &std::fs::read_to_string(&summary).expect("the summary"),
    ))
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
        self::dead(file, &json, &dump);
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
    // One `absent` where the program holds `std.math.decimal-arith`'s rules, which since
    // M1f S1f.6.35 is every program here: `std.math.num9` imports `std.math.decimal-arith`, and
    // an import brings its module whole (until then a program of `std.math.num9`
    // alone held none of its rules and read none). The undeclared name's,
    // over `let` (S1f.6.12). It is read statically, a premise of one plan,
    // and never at a node: the theory closes `let` and no rule derives it, so
    // the static check calls the program monotone all the same.
    let mono = &json["monotonicity"];
    assert_eq!(mono["monotone"], true, "{file}");
    let absents = mono["absent_reads"]["static"].as_u64().expect("a count");
    assert!(
        THEORIES.iter().any(|t| text.contains(t)),
        "{file}: imports none"
    );
    assert_eq!(
        absents, 1,
        "{file}: the theory reads one `absent`, the undeclared name's"
    );
    for kind in ["dynamic_monotone", "dynamic_non_monotone"] {
        assert_eq!(
            mono["absent_reads"][kind], 0,
            "{file}: an `absent` is read at a node ({kind})"
        );
    }

    // ── ein saturate ──
    let saturate = ein(&["saturate", "--dump", file]);
    assert_eq!(
        saturate.code, 0,
        "{file}: ein saturate\n{}",
        saturate.stderr
    );
    // The rule that reads the `absent` is a stratum above the rest.
    let strata = if absents == 0 {
        ";; Strata — stratified, 1 stratum"
    } else {
        ";; Strata — stratified, 2 strata"
    };
    assert!(
        saturate.stdout.lines().any(|l| l.trim() == strata),
        "{file}: not `{strata}`"
    );

    // ── the same answer ──
    let state = ein_corpus::unprefixed(
        &std::fs::read_to_string(dump.join("models/01.ein")).expect("the one state's file"),
    );
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
            THEORIES.iter().any(|t| text.contains(t)),
            "{}: under an arithmetic root and imports no theory of the calculator",
            path.display()
        );
        search_free(path, &text, &scratch, text.contains(DEAD));
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
