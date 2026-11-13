//! M1e2 S1e2.1.4 — **a model you can read**: `ein solve --models grid` and
//! `--diff-models FILE`.
//!
//! The grid's one claim that matters is that it is *the published table*.
//! [`ein_corpus::ZEBRA_GRID`] is the Wikipedia answer, the 25 cells
//! `ein-infer`'s `acceptance.rs` already holds both Zebra encodings' **solver**
//! to, and here the **renderer** is held to the same cells: `zebra.ein` through
//! its `slot-partition`, `zebra2.ein` through its five bijections onto `House`.
//! Two licences, one answer, one independent source.
//!
//! The diff's claims are the three shapes `ein-render::models`' unit tests pin,
//! seen through the CLI: a missing model, a spurious one, and one a fact off.
//! Until M1e2 S1e2.1a.5 they were the register's own witnesses, the walk's
//! wrong answers; the region read-out answers those rows right, so the shapes
//! are made here by writing the file wrong, over W1's witness.

use std::collections::BTreeSet;
use std::process::Command;

use ein_corpus::{Scratch, ZEBRA_GRID, repo_root};

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

/// The first grid in `out`, as `(row, cell value)` pairs — every value of a
/// multi-valued cell its own pair, so a cell holding two shows as two.
fn grid_cells(out: &str) -> (String, BTreeSet<(String, String)>) {
    let mut lines = out
        .lines()
        .skip_while(|l| !l.trim_start().starts_with("grid —"));
    let licence = lines.next().expect("a grid line").trim().to_string();
    let mut cells = BTreeSet::new();
    for line in lines
        .map(str::trim)
        .take_while(|l| l.starts_with('|'))
        .skip(2)
    {
        let cols: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        for cell in &cols[1..] {
            for v in cell.split(" / ") {
                cells.insert((cols[0].to_string(), v.to_string()));
            }
        }
    }
    (licence, cells)
}

fn published() -> BTreeSet<(String, String)> {
    ZEBRA_GRID
        .iter()
        .flat_map(|(h, vs)| vs.iter().map(move |v| (h.to_string(), v.to_string())))
        .collect()
}

#[test]
fn the_grid_is_the_published_table_in_both_encodings() {
    for (file, licence) in [
        ("examples/zebra.ein", "slot-partition"),
        ("examples/zebra2.ein", "injective color-loc"),
    ] {
        let r = ein(&["solve", file, "--models", "grid"]);
        assert_eq!(r.code, 0, "{file}: {}", r.err);
        let (lic, cells) = grid_cells(&r.out);
        assert!(lic.contains(licence), "{file}: licensed by {lic:?}");
        assert_eq!(
            cells,
            published(),
            "{file}: the grid is not the published table"
        );
    }
}

/// The read-out above the `models —` block, without the header's echo of the
/// flags.
fn above_models(out: &str) -> String {
    out.lines()
        .skip(1)
        .take_while(|l| !l.starts_with("models — "))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

/// Where nothing licenses a grid the run says so in one line per state, and
/// the read-out above the block is the listing's — `key`'s *unaffordable →
/// say why* precedent.
#[test]
fn no_licence_is_one_line_and_the_models_still_print() {
    let file = "examples/lattice/02_genuine_3set_death.ein";
    let list = ein(&["solve", file, "--maximal"]);
    let grid = ein(&["solve", file, "--maximal", "--models", "grid"]);
    assert_eq!(grid.code, 0, "{}", grid.err);
    let none: Vec<&str> = grid
        .out
        .lines()
        .filter(|l| l.contains("grid — none:"))
        .collect();
    assert_eq!(none.len(), 3, "one line per state:\n{}", grid.out);
    assert_eq!(above_models(&grid.out), above_models(&list.out));
}

/// An owing node gets its grid too — and the cell its debt is about is the
/// empty one. On an empty R of the owing kind the frontier is what the
/// read-out lists, and since M1e2 S1e2.1a.5 it is what the forms draw, where
/// the walk drew its `Open` state.
///
/// The program was `tests/stdlib/closure/03_closed_and_owing.ein` until M1f
/// S1f.6.34 refused an obligation on a closed relation: `a1` owes `r` a `B`
/// and `r` is functional, which is what makes a row of the grid; here the
/// debt cannot be paid because both `B`s are stored false.
#[test]
fn an_owing_node_has_a_grid_with_its_debt_as_an_empty_cell() {
    let dir = Scratch::new("ein-owing-grid");
    let file = dir.path().join("owing.ein");
    std::fs::write(
        &file,
        "(import std.algebra :symbols (functional total-owed))\n\
         (relation r A B :why \"{?1} is assigned {?2}\")\n\
         (is-a a1 A) (is-a b1 B) (is-a b2 B)\n\
         (functional r)\n(total-owed r is-a)\n\
         (not (r a1 b1)) (not (r a1 b2))\n(solve :no-hypothesis (is-a))\n",
    )
    .expect("written");
    let r = ein(&["solve", file.to_str().expect("utf-8"), "--models", "grid"]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("every alive node owes"), "{}", r.out);
    assert!(
        r.out.contains("── owing node 1 of 1 — owes 1 ──"),
        "{}",
        r.out
    );
    assert!(r.out.contains("grid — rows:"), "{}", r.out);
    assert!(
        r.out.contains("| —"),
        "the owed cell is not empty:\n{}",
        r.out
    );
}

/// `list` is still what `ein solve` prints.
#[test]
fn list_stays_the_default() {
    let plain = ein(&["solve", "examples/zebra2.ein", "-n", "1"]);
    let list = ein(&[
        "solve",
        "examples/zebra2.ein",
        "-n",
        "1",
        "--models",
        "list",
    ]);
    assert_eq!(above_models(&plain.out), above_models(&list.out));
    assert!(!plain.out.contains("grid —"));
    assert!(!list.out.contains("models — "));
}

fn diff(file: &str, flags: &[&str], expected: &str) -> String {
    let dir = Scratch::new("ein-models-grid");
    let path = dir.join(format!("{}.models", file.replace('/', "_")));
    std::fs::write(&path, expected).expect("writes");
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    let p = path.to_string_lossy().into_owned();
    args.extend(["--diff-models", &p]);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{file}: {}", r.err);
    r.out
}

/// The three shapes, each on W1's witness — whose maximal line is `{(p A)}`
/// and `{(p B), (p C)}`, the right answer since the switch — against a file
/// written wrong; and the empty model, against the minimal line of a program
/// whose ∅ owes.
#[test]
fn the_diff_names_the_missing_the_spurious_and_the_one_fact_off() {
    const W1: &str = "examples/branching/15_lookahead_two_step_on.ein";
    let out = diff(
        W1,
        &["--maximal"],
        "closes p\nmodel\n(p A)\nmodel\n(p B)\n(p C)\nmodel\n(p C)\n(p D)\n",
    );
    assert!(
        out.contains("2 matched, 0 near, 1 missing, 0 spurious"),
        "{out}"
    );
    assert!(
        out.contains("- missing: model 3 of the file [(p C) (p D)]"),
        "{out}"
    );
    let out = diff(W1, &["--maximal"], "closes p\nmodel\n(p A)\n");
    assert!(
        out.contains("1 matched, 0 near, 0 missing, 1 spurious"),
        "{out}"
    );
    assert!(out.contains("+ spurious: state 2 of 2"), "{out}");
    let out = diff(
        W1,
        &["--maximal"],
        "closes p\nmodel\n(p A)\n(p B)\nmodel\n(p B)\n(p C)\n",
    );
    assert!(
        out.contains("1 matched, 1 near, 0 missing, 0 spurious"),
        "{out}"
    );
    assert!(out.contains("lacks (p B)"), "{out}");
    // The model with no fact, which no state of the minimal line is.
    let out = diff(
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        &["--minimal"],
        "closes seats\nmodel\n",
    );
    assert!(
        out.contains("- missing: model 1 of the file [no facts]"),
        "{out}"
    );
}

#[test]
fn a_file_the_diff_cannot_read_is_a_usage_error_before_anything_runs() {
    let dir = Scratch::new("ein-models-grid-bad");
    let bad = dir.join("bad.models");
    std::fs::write(&bad, "(p A)\n").expect("writes");
    for path in [
        bad.to_string_lossy().into_owned(),
        dir.join("absent").to_string_lossy().into_owned(),
    ] {
        let r = ein(&["solve", "examples/zebra2.ein", "--diff-models", &path]);
        assert_eq!(r.code, 2, "{path}: {}", r.err);
        assert!(r.err.contains("--diff-models"), "{}", r.err);
        assert!(r.out.is_empty(), "it ran anyway");
    }
}
