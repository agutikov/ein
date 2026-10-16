//! The CLI's argument surface — S1a.5.4's acceptance.
//!
//! `--help` *layout* is not on the byte gate. What is checked is *structure*:
//! every parser rendered as `{command → {option → short, metavar, arity,
//! default, choices, group, help}}`, so a renamed short key, a changed
//! default, a dropped option or a new one fails on its own line.
//!
//! The two floors below count the options — but a count is a weak check on
//! its own, so the rendering itself is checked in as a golden. That is not
//! "the surface is right"; it is "the surface has not moved without someone
//! saying so", which is the same claim `corpus_shapes.md5` makes about every
//! other rendering.

// Both are the golden test's, and the golden is of the *default* surface —
// see `the_argument_surface_is_stable`.
#[cfg(feature = "einb")]
use ein_corpus::{golden, golden_path};

/// The surface, counted from the parsers themselves — 64 options across 12
/// parsers, `-h` and `-V` excluded as parser furniture (see `help_shape`'s
/// skip). An extractor that silently returned nothing would pass an empty
/// diff; it does not pass this.
///
/// `ein test`'s nine are M1c
/// [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test)'s
/// (plus M1d S1d.4.1's `--json-report`), and `ein kb save --saturate` exists
/// only under the `einb` feature, so each is counted apart from the rest.
///
/// `saturate` has 5 options rather than 3 because `--events` and
/// `--events-level` are on it too.
const EXPECTED: [(&str, usize); 12] = [
    ("COMMAND ein\n", 0),
    // Among them P1a.7's `--jobs`
    // ([T1a.7.2.1](../../../../docs/history/m1a_rust/README.md#s1a72--level-1-parallel-enterings)),
    // M1d S1d.3.3's `--models`, M1d P1d.10's `--layer-progress` and M1e2's
    // `--dump-answer` (S1e2.1.2), `--dump-lattice` (S1e2.1.3),
    // `--diff-models` (S1e2.1.4), and S1e2.1a.1's `--dump-region`,
    // `--region-limit` and `--region-states`. And S1e2.1a.9's four region
    // flags, `--minimal`, `--maximal`, `--single` and `--region-budget`
    // ([`solve_interface.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md)):
    // the walk's options go at S1e2.1a.5's switch, not here.
    ("COMMAND ein solve\n", 42),
    // S1e2.1a.9's `ein oracle`: `--region-limit`, `--dump`, `--states`,
    // `--jobs`, `--stats`.
    ("COMMAND ein oracle\n", 5),
    ("COMMAND ein saturate\n", 5),
    ("COMMAND ein render\n", 0),
    ("COMMAND ein render rules\n", 1),
    ("COMMAND ein render rule\n", 2),
    ("COMMAND ein render constraints\n", 0),
    ("COMMAND ein render lattice\n", 2),
    // M1e2 S1e2.1.3's two doors: `--levi` and `--rule-mode`; `--since`,
    // `--colour-by`, `--no-types`, `--no-instances`.
    ("COMMAND ein render program\n", 2),
    ("COMMAND ein render kb\n", 4),
    // M1e2 S1e2.1a.2's `--view` over a `--dump-region` directory.
    ("COMMAND ein render region\n", 1),
];

/// M1c [S1c.1.3](../../../../docs/history/m1c_external_validation/README.md#s1c13--ein-test)'s
/// parser. **Eight**, and
/// the number is the design: the three that decide whether a run can finish
/// (`-m`, `-T`, `-E`), the two that decide how much of a passing run to print
/// (`-v`, `-q`), and the three file-only artefact ones the stage asked for by
/// name (`--events`, `--events-level`, `--json-summary`), and M1d S1d.4.1's
/// `--json-report`. There is deliberately no `-n` and no `--exhaustive`: an
/// expectation is a claim about the exhausted answer, so exhausting is the
/// behaviour rather than a flag, and a *missing* option is exactly the kind of
/// thing this count exists to notice.
const TESTER: [(&str, usize); 1] = [("COMMAND ein test\n", 9)];

/// P1a.8's surface, registered only under the `einb` feature, and here for
/// the same reason the other parsers are — an extractor that stopped seeing a
/// parser would otherwise pass.
#[cfg(feature = "einb")]
const CONTAINER: [(&str, usize); 2] = [("COMMAND ein kb\n", 0), ("COMMAND ein kb save\n", 1)];
#[cfg(not(feature = "einb"))]
const CONTAINER: [(&str, usize); 0] = [];

fn options_under(shape: &str, header: &str) -> usize {
    let at = shape
        .find(header)
        .unwrap_or_else(|| panic!("no {header:?} in the shape"));
    shape[at + header.len()..]
        .lines()
        .take_while(|l| !l.starts_with("COMMAND "))
        .filter(|l| l.starts_with("  OPTION "))
        .count()
}

#[test]
fn the_extractor_finds_the_whole_surface() {
    let shape = ein_cli::help_shape::help_shape();
    let mut total = 0;
    for (header, n) in EXPECTED.iter().chain(TESTER.iter()).chain(CONTAINER.iter()) {
        let found = options_under(&shape, header);
        assert_eq!(found, *n, "{header:?} has {found} options, expected {n}");
        total += found;
    }
    assert_eq!(
        total,
        64 + TESTER.iter().map(|(_, n)| n).sum::<usize>()
            + CONTAINER.iter().map(|(_, n)| n).sum::<usize>(),
        "64 options across the twelve parsers of `ein`, `solve`, `oracle`, \
         `saturate` and `render`, plus `ein test`'s nine and `ein kb save --saturate`"
    );
}

/// **The whole rendering, checked in.**
///
/// Every option of every parser — 74 across fifteen at M1e2 S1e2.1a.9, the count
/// [`the_extractor_finds_the_whole_surface`] holds — each with its short key, metavar, arity,
/// default, choices, group and help. A flag added, removed, renamed or
/// re-defaulted shows up as a line.
///
/// ```text
/// EIN_BLESS=1 cargo test -p ein-cli --test help_surface
/// ```
/// The golden is of the **default** surface, container included. A
/// `--no-default-features` build renders a smaller one on purpose and has
/// nothing to compare it against, so it does not compare.
#[cfg(feature = "einb")]
#[test]
fn the_argument_surface_is_stable() {
    let shape = ein_cli::help_shape::help_shape();
    if let Some(msg) = golden(&golden_path("ein-cli", "help_shape.txt"), &shape) {
        panic!("{msg}");
    }
}

/// The mutation floor: the short key has to be *in* the rendering, or a
/// renamed one could not fail the comparison.
#[test]
fn a_renamed_short_key_fails() {
    let shape = ein_cli::help_shape::help_shape();
    let mutated = shape.replacen("--solutions -n ", "--solutions -N ", 1);
    assert_ne!(shape, mutated, "the short key must be in the rendering");
}

/// **The `--view full` fallback says the same thing in the DOT and in the
/// help** — M1e S1e.4.6, `CD-L3`.
///
/// The two surfaces a user meets in one session disagreed: `--view`'s help
/// gave the real reason (*solve doesn't store the per-commitment SetNode
/// DAG*), while the DOT the same command printed said *"no stored lattice
/// (store_lattice=False)"* — false twice over, since every caller that can
/// reach that line sets the flag `true` and `LatticeProof` has no `kb_index`
/// for any setting of it to fill.
///
/// The stage's acceptance is *"the fallback message states the real reason,
/// and agrees with `--view`'s help text"*, and agreement by inspection is what
/// let them part in the first place. `ein-render` cannot see `cmdline.rs`, so
/// the check lives here, where the constant and the golden are both in scope.
#[test]
fn the_full_view_fallback_agrees_with_the_help_text() {
    let note = ein_render::FULL_VIEW_FALLBACK;
    assert!(
        note.contains("per-commitment SetNode DAG"),
        "the DOT note no longer names what is missing: {note:?}"
    );
    assert!(
        !note.contains("store_lattice"),
        "the DOT note is back to naming a flag that decides nothing: {note:?}"
    );
    let shape = std::fs::read_to_string(golden_path("ein-cli", "help_shape.txt"))
        .expect("golden/help_shape.txt");
    assert!(
        shape.contains("per-commitment SetNode DAG"),
        "`ein render lattice --view`'s help no longer names the per-commitment \
         SetNode DAG, so the two surfaces have parted again (CD-L3)"
    );
}

/// **Every artefact `ein solve` can write, the review bundle drives** — M1e2
/// S1e2.1.5.
///
/// The bundle's promise is *every artefact the engine can write for a run*,
/// rendered, so that a reviewer never meets one they were not shown. A new
/// artefact flag is the way that promise breaks silently, so the flags are
/// read off the checked-in surface rather than listed: every `ein solve`
/// option whose value is a path (`metavar=FILE…` or `DIR`) has to appear in
/// `utils/review/run.py`, the module that runs the invocations.
#[test]
#[cfg(feature = "einb")]
fn every_artefact_flag_of_solve_is_driven_by_the_review_bundle() {
    let golden = std::fs::read_to_string(ein_corpus::golden_path("ein-cli", "help_shape.txt"))
        .expect("help_shape.txt");
    let solve = golden
        .split("COMMAND ein solve\n")
        .nth(1)
        .and_then(|s| s.split("\nCOMMAND ").next())
        .expect("the solve parser");
    let driver = std::fs::read_to_string(ein_corpus::repo_root().join("utils/review/run.py"))
        .expect("utils/review/run.py");
    let mut flags = Vec::new();
    let mut missing = Vec::new();
    for line in solve
        .lines()
        .filter(|l| l.trim_start().starts_with("OPTION "))
    {
        let mut words = line.split_whitespace().skip(1);
        let Some(flag) = words.next() else { continue };
        let path_valued = line.contains("metavar=DIR") || line.contains("metavar=FILE");
        if !path_valued {
            continue;
        }
        flags.push(flag.to_string());
        if !driver.contains(&format!("\"{flag}\"")) {
            missing.push(flag.to_string());
        }
    }
    assert!(
        flags.len() >= 6,
        "found only {flags:?} — the extractor stopped looking"
    );
    assert!(
        missing.is_empty(),
        "`ein solve` writes artefacts the review bundle never asks for: {missing:?}. Drive them in \
         utils/review/run.py, or the bundle is no longer every artefact"
    );
}
