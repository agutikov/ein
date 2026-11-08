//! M1f S1f.5.15's T4 (T1f.5.15.4) — **the static non-stratifiability
//! diagnostic**, through the binary.
//!
//! A program whose rules' dependency graph has a cycle through an `absent`
//! has no stratification, so its root fixpoint may depend on the boundary's
//! admission order and need not be unique: a possibility the graph licenses,
//! never a fact about the run (a cycle whose rules never fire changes
//! nothing). `ein solve` says so in a `strata`
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
/// A program whose only cycle through an `absent` never fires.
const NEVER_FIRES: &str = "examples/syntax/rule-guards.ein";

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
    // What the graph licenses, and no more: a possibility, never a fact
    // about this run.
    assert!(
        row.contains(
            "root's fixpoint may depend on the boundary's admission order, and need not be unique"
        ),
        "{row}"
    );
    assert!(
        out.lines().any(|l| l.contains(CYCLE)
            && l.contains("several fixpoints may exist")
            && l.contains("may depend")),
        "{TRIPS}: the cycle's parity is stated as a possibility\n{out}"
    );

    // `syntax/rule-guards.ein`'s odd cycle never fires (the program has no
    // facts), so its fixpoint is unique: the row may say only what may be.
    let (code, out) = ein(&["solve", NEVER_FIRES]);
    assert_eq!(code, 0, "{out}");
    let strata: Vec<&str> = out
        .lines()
        .filter(|l| l.starts_with("  strata ") || l.starts_with("    negative cycle "))
        .collect();
    assert_eq!(
        strata.len(),
        2,
        "{NEVER_FIRES}: one row and one cycle\n{out}"
    );
    assert!(
        strata[1].contains("odd: a fixpoint supported at the boundary need not exist"),
        "{strata:?}"
    );
    for l in &strata {
        for overclaim in ["one of several", "is the one", "several fixpoints,"] {
            assert!(
                !l.contains(overclaim),
                "{NEVER_FIRES}: {l:?} claims {overclaim:?}"
            );
        }
    }

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
        out.contains(
            ";; Strata — not stratifiable: 1 negative cycle; the fixpoint below may depend on \
             the boundary's admission order, and need not be unique"
        ),
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
