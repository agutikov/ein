//! M1d S1d.3.3 — **how `ein solve` reports a model *set***: the guarantee
//! rule, and the determining key that is the set's compact form.
//!
//! [P1d.3](../../../../plans/m1d_satisfiability/p1d.3_model_sets/README.md)
//! answered *print or describe* with **both**, and the two halves are tested
//! here because they are one surface: an `Ambiguity` block. The rule first,
//! since the key inherits it.
//!
//! ## The rule (T1d.3.3.2)
//!
//! | the search | what a report of the model set may claim |
//! |---|---|
//! | `exhausted = true` | *these are the models* |
//! | `exhausted = false` | *these are models **found*** — a further model may exist |
//!
//! A `Solution` has qualified its own `k` since ein.py — *"(not certified —
//! pass --exhaustive)"* — and the verdict that reports a model **set** did
//! not, which is the wrong way round: `k = 1` unqualified is a guess about
//! uniqueness, and `k = 5` unqualified is a **wrong count**.
//! `examples/saturation/type-exclusivity/colors.ein` is the proof and is one
//! of the two fixtures below: at the default depth `solve -e` finds 5 models
//! and at `-m 6` there are **9**. `exhausted` was printed by `--stats` and by
//! nothing else, so a reader of the count had no way to know it was a lower
//! bound.
//!
//! ## The key (T1d.3.3.4)
//!
//! `--models key` is **additional output, never a replacement**: it chooses
//! the *projection* of the same model set that goes to stdout, and reaches
//! nothing recorded. The third test is that claim, checked the way
//! [S1d.2.5](../../../../plans/m1d_satisfiability/p1d.2_obligations/hypotheses_from_obligations.md)
//! checked its lever — run the entry both ways and diff everything outside
//! the cells that are allowed to move.

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

/// **exhausted-true-says-these-are-the-models.** Row one of the rule, on a
/// real multi-model entry.
///
/// `colors.ein` at `-m 6` exhausts with nine models, and that is the only
/// state in which the engine may print a bare count. Nothing qualifies it,
/// nothing says "found", and `--stats` agrees — which is the assertion that
/// keeps the two surfaces from drifting apart, since the qualifier is
/// computed from `stats.exhausted` and printed nowhere near it.
#[test]
fn an_exhausted_search_reports_the_models() {
    let run = ein(&[
        "solve",
        "examples/saturation/type-exclusivity/colors.ein",
        "-e",
        "-m",
        "6",
        "-s",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(row(&run, "exhausted"), "exhausted        true");
    assert_eq!(row(&run, "solutions (k)   "), "solutions (k)   9");
    assert_eq!(
        row(&run, "verdict"),
        "verdict         Ambiguous — distinct complete models; \
         the puzzle is under-determined"
    );
}

/// **exhausted-false-says-these-are-models-found.** Row two, and the one that
/// fails if someone later drops the qualifier.
///
/// The same file at the default cap. Every number on the surface is *the same
/// kind of number* as above and every one of them is a lower bound: the search
/// stopped at depth 5 with a non-empty frontier and the true count is 9. Two
/// separate marks carry that — the parenthesis on the count and the word
/// *found* in the verdict — because a reader who scans one line must not have
/// to have read the other.
#[test]
fn an_unexhausted_search_reports_models_found() {
    let run = ein(&[
        "solve",
        "examples/saturation/type-exclusivity/colors.ein",
        "-e",
        "-s",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(row(&run, "exhausted"), "exhausted        false");
    assert_eq!(
        row(&run, "solutions (k)   "),
        "solutions (k)   5   (a lower bound — the search did not exhaust)"
    );
    assert_eq!(
        row(&run, "verdict"),
        "verdict         Ambiguous — distinct complete models found; \
         the puzzle is under-determined"
    );
    // And the claim behind the claim: the count really is short. A fixture
    // that only asserted the wording would still pass if `colors.ein` grew a
    // clue and stopped being ambiguous at depth 6.
    let deeper = ein(&[
        "solve",
        "examples/saturation/type-exclusivity/colors.ein",
        "-e",
        "-m",
        "6",
        "-s",
    ]);
    assert_eq!(row(&deeper, "solutions (k)   "), "solutions (k)   9");
}

/// **the-key-is-a-projection-not-a-second-answer.** T1d.3.3.4's constraint,
/// as a diff.
///
/// `--models key` may move the model *blocks* on stdout and nothing else. So:
/// the same entry, both ways, with `--json-summary` written each time — the
/// summaries must be **byte-identical**, because `verdict.solutions` is what
/// every downstream consumer reads, and the two runs' stdout must agree on
/// every line that is not a model block.
#[test]
fn the_key_form_changes_stdout_and_nothing_recorded() {
    let dir = std::env::temp_dir().join(format!("ein-model-set-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let a = dir.join("list.json");
    let b = dir.join("key.json");
    let entry = "examples/lattice/02_genuine_3set_death.ein";

    let listed = ein(&["solve", entry, "-e", "--json-summary", &a.to_string_lossy()]);
    let keyed = ein(&[
        "solve",
        entry,
        "-e",
        "--models",
        "key",
        "--json-summary",
        &b.to_string_lossy(),
    ]);
    assert_eq!((listed.code, keyed.code), (0, 0));
    assert_eq!(
        std::fs::read_to_string(&a).expect("a summary"),
        std::fs::read_to_string(&b).expect("a summary"),
        "--models moved something a consumer reads"
    );
    // The verdict header is identical; only what follows it differs.
    let head = |r: &Run| {
        r.out
            .lines()
            .take_while(|l| !l.trim_start().starts_with("model 1/"))
            .take(4)
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    assert_eq!(head(&listed), head(&keyed));
    assert!(
        listed.out.contains("model 1/3") && !keyed.out.contains("model 1/3"),
        "the key form should stand in for the blocks:\n{}",
        keyed.out
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **the-key-determines-and-says-how.** The form itself, on the smallest
/// corpus entry that has one.
///
/// Three things a key table must carry, and each is a separate failure:
/// the *size* against the varying total, the *count* of equally minimal keys
/// — the answer to "why these" — and one row per model. The rows are the
/// joint projection, so their number is `k`; an envelope's would have been the
/// product.
#[test]
fn the_key_table_names_its_size_its_alternatives_and_one_row_per_model() {
    let run = ein(&[
        "solve",
        "examples/lattice/02_genuine_3set_death.ein",
        "-e",
        "--models",
        "key",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(row(&run, "solutions (k)   "), "solutions (k)   3");
    assert_eq!(
        row(&run, "determining key"),
        "determining key — 2 of 3 varying slots"
    );
    let table: Vec<&str> = run
        .out
        .lines()
        .skip_while(|l| !l.contains("-----"))
        .skip(1)
        .take_while(|l| !l.trim().is_empty())
        .collect();
    assert_eq!(table.len(), 3, "one row per model, got:\n{table:#?}");
    assert!(
        run.out.contains("rows, one per model."),
        "an exhausted key table should claim the models, not models found:\n{}",
        run.out
    );
}

/// **an-unaffordable-key-prints-the-models.** The fallback, and it is a
/// first-class answer rather than an error.
///
/// A minimum determining set is a minimum hitting set, which is NP-hard, and
/// `branching/06_lookahead_on.ein` is the corpus entry that shows it: 42
/// varying slots, 22 models, minimum key **8**, and `C(42, 8) = 118 030 185`
/// candidate keys — so the count that answers *"why these"* cannot be taken
/// and the form declines rather than printing an arbitrary basis. What it
/// falls back to is the enumeration, which was a legitimate winner of the
/// pricing all along, so the exit code is 0 and the models are all there.
#[test]
fn an_unaffordable_key_falls_back_to_the_enumeration() {
    let run = ein(&[
        "solve",
        "examples/branching/06_lookahead_on.ein",
        "-e",
        "--models",
        "key",
    ]);
    assert_eq!(run.code, 0);
    assert_eq!(
        row(&run, "determining key"),
        "determining key — none within budget"
    );
    // Whitespace-normalised, because the reason is wrapped to the page and
    // where it wraps is layout rather than content.
    let flat = run.out.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("C(42, 8) = 118 030 185 candidates is over the budget"),
        "the reason should name the number that made it unaffordable:\n{}",
        run.out
    );
    assert!(
        flat.contains("the models are printed instead") && run.out.contains("model 22/22"),
        "the fallback is the enumeration, whole:\n{}",
        run.out
    );
}

/// **only-a-model-set-has-a-key.** The flag is inert on the other verdicts.
///
/// A single model is its own smallest description and a refutation has no
/// model set at all, so `--models key` may not change one byte of either.
/// Checked rather than assumed, because the natural way to write the renderer
/// — read the flag once, at the top — would have got this wrong.
#[test]
fn the_flag_is_inert_where_there_is_no_model_set() {
    for entry in [
        "examples/branching/05_mini_zebra.ein",
        "examples/features/12_expect_false.ein",
    ] {
        let listed = ein(&["solve", entry, "-e"]);
        let keyed = ein(&["solve", entry, "-e", "--models", "key"]);
        assert_eq!(
            (listed.out, listed.code),
            (keyed.out, keyed.code),
            "--models key moved {entry}, which has no model set"
        );
    }
}
