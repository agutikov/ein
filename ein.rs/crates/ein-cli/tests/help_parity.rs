//! S1a.5.4 acceptance — the CLI's argument surface, against `argparse`'s.
//!
//! [Q-M1a.13](../../../../plans/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity)
//! took `--help` *layout* off the byte gate. This is what it put in its place:
//! both parsers rendered as `{command → {option → short, metavar, arity,
//! default, choices, group, help}}`, and the texts diffed. A renamed short
//! key, a changed default, a dropped option or a new one fails on its own
//! line.
//!
//! Two floors keep it from passing vacuously: the counted surface below, and
//! `a_renamed_short_key_fails`, which mutates the Rust side and asserts the
//! rendering carries what the diff would have to notice.

use ein_oracle::{Answer, IR_ORACLE, Oracle};

/// The surface, counted from the parsers themselves — 39 options across 8
/// parsers, `-h` excluded. An extractor that silently returned nothing would
/// pass an empty diff; it does not pass this.
///
/// 39, not the "~40, every one with a short key" the stage plan carried: 26 of
/// `solve`'s 29 have short keys, and none of `saturate`'s 5 or `render`'s 5
/// do. `saturate` has 5 rather than 3 because `_events.add_arguments` puts
/// `--events` and `--events-level` on it too.
const EXPECTED: [(&str, usize); 8] = [
    ("COMMAND ein\n", 0),
    ("COMMAND ein solve\n", 29),
    ("COMMAND ein saturate\n", 5),
    ("COMMAND ein render\n", 0),
    ("COMMAND ein render rules\n", 1),
    ("COMMAND ein render rule\n", 2),
    ("COMMAND ein render constraints\n", 0),
    ("COMMAND ein render lattice\n", 2),
];

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
    for (header, n) in EXPECTED {
        let found = options_under(&shape, header);
        assert_eq!(found, n, "{header:?} has {found} options, expected {n}");
        total += found;
    }
    assert_eq!(total, 39, "39 options across 8 parsers");
}

#[test]
fn the_surface_matches_argparse() {
    let mut oracle = Oracle::start(IR_ORACLE).expect("oracle starts");
    let want = match oracle.ask(serde_json::json!({"op": "help-shape"})) {
        Answer::Ok(s) => s,
        other => panic!("oracle refused help-shape: {other:?}"),
    };
    let got = ein_cli::help_shape::help_shape();
    if want != got {
        let w: Vec<&str> = want.lines().collect();
        let g: Vec<&str> = got.lines().collect();
        let mut shown = 0;
        for i in 0..w.len().max(g.len()) {
            let (a, b) = (
                w.get(i).copied().unwrap_or(""),
                g.get(i).copied().unwrap_or(""),
            );
            if a != b {
                eprintln!("line {i}:\n  py: {a}\n  rs: {b}");
                shown += 1;
                if shown == 12 {
                    break;
                }
            }
        }
        panic!("the argument surface differs from argparse's");
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
