//! The CLI's argument surface — S1a.5.4's acceptance, without `argparse`.
//!
//! [Q-M1a.13](../../../../plans/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity)
//! took `--help` *layout* off the byte gate. What replaced it was a
//! comparison of *structure*: both parsers rendered as `{command → {option →
//! short, metavar, arity, default, choices, group, help}}`, and the texts
//! diffed, so a renamed short key, a changed default, a dropped option or a
//! new one failed on its own line.
//!
//! [S1a.10.2](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
//! removes the second parser. The
//! [ledger](../../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md)
//! calls the row **retired**, and the two oracle-free floors below are what it
//! keeps — but a count is a weak successor to a diff, so the rendering itself
//! is checked in as a golden. That is not "the surface is right"; it is "the
//! surface has not moved without someone saying so", which is the same claim
//! `corpus_shapes.md5` makes about every other rendering and the strongest one
//! available with a single implementation.

// Both are the golden test's, and the golden is of the *default* surface —
// see `the_argument_surface_is_stable`.
#[cfg(feature = "einb")]
use ein_corpus::{golden, golden_path};

/// The surface, counted from the parsers themselves — 40 options across 8
/// parsers, `-h` and `-V` excluded as parser furniture (see `help_shape`'s
/// skip). An extractor that silently returned nothing would pass an empty
/// diff; it does not pass this.
///
/// **39 of them are ein.py's**, and the fortieth is `solve --jobs`
/// ([T1a.7.2.1](../../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md#task-t1a721--snapshot-and-fan-out)),
/// which has no counterpart there for the same reason `ein kb save` does not:
/// it is a knob for something ein.py never had. The split is kept in the sum
/// below rather than folded away, because "the surface has not moved" is a
/// claim about the *ported* surface and a new option is a different event from
/// a changed one.
///
/// 39, not the "~40, every one with a short key" the stage plan carried: 26 of
/// `solve`'s 29 have short keys, and none of `saturate`'s 5 or `render`'s 5
/// do. `saturate` has 5 rather than 3 because `_events.add_arguments` puts
/// `--events` and `--events-level` on it too.
const EXPECTED: [(&str, usize); 8] = [
    ("COMMAND ein\n", 0),
    // 29 of ein.py's, plus P1a.7's `--jobs`.
    ("COMMAND ein solve\n", 30),
    ("COMMAND ein saturate\n", 5),
    ("COMMAND ein render\n", 0),
    ("COMMAND ein render rules\n", 1),
    ("COMMAND ein render rule\n", 2),
    ("COMMAND ein render constraints\n", 0),
    ("COMMAND ein render lattice\n", 2),
];

/// P1a.8's surface, which has no ein.py counterpart at all: `.einb` is a new
/// format and `ein kb` is a new command, so these two parsers were never in
/// the `argparse` diff and are here for the same reason the other eight are —
/// an extractor that stopped seeing a parser would otherwise pass.
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
    for (header, n) in EXPECTED.iter().chain(CONTAINER.iter()) {
        let found = options_under(&shape, header);
        assert_eq!(found, *n, "{header:?} has {found} options, expected {n}");
        total += found;
    }
    assert_eq!(
        total,
        39 + 1 + CONTAINER.iter().map(|(_, n)| n).sum::<usize>(),
        "39 options across ein.py's eight parsers, plus `solve --jobs` and \
         `ein kb save --saturate` — neither of which ein.py has"
    );
}

/// **The whole rendering, checked in.**
///
/// 40 options across 8 parsers, each with its short key, metavar, arity,
/// default, choices, group and help — the same text the diff against
/// `argparse` consumed, blessed from a tree where that diff was green. A flag
/// added, removed, renamed or re-defaulted shows up as a line.
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
