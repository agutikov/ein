//! M1f S1f.5.15's T4 (T1f.5.15.4) — **the static non-stratifiability
//! diagnostic**, through the binary.
//!
//! A program whose rules' dependency graph has a cycle through an `absent`
//! has no stratification, and its root fixpoint is one of several: the one
//! the boundary's admission order reaches. `ein solve` says so in a `strata`
//! row, and `ein saturate --dump` in its `;; Strata` line, each cycle named by
//! its relations and the rules on its negative edges. It is a diagnostic and
//! not a refusal: the run answers, and exits as it did. The fixtures are the
//! stage's: `examples/features/07_unstratifiable.ein` trips it, and
//! `examples/features/01_not_and_absent.ein`, an `absent` over a relation no
//! rule derives, does not.
//!
//! ```text
//! cargo test -p ein-cli --test strata_diagnostic
//! ```

use std::process::Command;

use ein_corpus::repo_root;

const TRIPS: &str = "examples/features/07_unstratifiable.ein";
const STRATIFIED: &str = "examples/features/01_not_and_absent.ein";
const CYCLE: &str = "p, q — derive-p: q →¬ p · derive-q: p →¬ q";

fn ein(args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// **`ein solve`**: a `strata` row naming the cycle on the fixture that trips
/// it, none on the one that does not, and an answer either way.
#[test]
fn solve_names_each_negative_cycle_and_still_answers() {
    let (code, out) = ein(&["solve", TRIPS]);
    assert_eq!(code, 0, "a diagnostic, not a refusal:\n{out}");
    let row = out
        .lines()
        .find(|l| l.starts_with("  strata "))
        .unwrap_or_else(|| panic!("{TRIPS}: no strata row\n{out}"));
    assert!(
        row.contains("not stratifiable: 1 cycle through an absent"),
        "{row}"
    );
    assert!(
        out.lines().any(|l| l.starts_with("    negative cycle ")
            && l.contains(CYCLE)
            && l.contains("even")),
        "{TRIPS}: the cycle is not named\n{out}"
    );
    assert!(
        out.contains("(region"),
        "{TRIPS}: the answer is printed\n{out}"
    );

    let (code, out) = ein(&["solve", STRATIFIED]);
    assert_eq!(code, 0, "{out}");
    assert!(
        !out.lines().any(|l| l.starts_with("  strata ")),
        "{STRATIFIED}: a stratified program prints no strata row\n{out}"
    );
}

/// **`ein saturate --dump`**: the `;; Strata` line, both ways.
#[test]
fn saturate_dump_says_whether_a_stratification_exists() {
    let (code, out) = ein(&["saturate", TRIPS, "--dump"]);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains(";; Strata — not stratifiable: 1 negative cycle"),
        "{out}"
    );
    assert!(
        out.contains(&format!(";;   negative cycle  {CYCLE}")),
        "{out}"
    );

    let (code, out) = ein(&["saturate", STRATIFIED, "--dump"]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains(";; Strata — stratified, "), "{out}");
    assert!(!out.contains("not stratifiable"), "{out}");
}
