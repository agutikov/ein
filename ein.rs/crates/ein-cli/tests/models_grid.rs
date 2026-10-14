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
//! seen through the CLI on the register's own witnesses: a missing model, a
//! spurious one, and one a fact off.

use std::collections::BTreeSet;
use std::process::Command;

use ein_corpus::{ZEBRA_GRID, repo_root};

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

/// Where nothing licenses a grid the run says so in one line and the models
/// are still printed — `key`'s *unaffordable → print the models* precedent.
#[test]
fn no_licence_is_one_line_and_the_models_still_print() {
    let file = "examples/lattice/02_genuine_3set_death.ein";
    let list = ein(&["solve", file, "-e"]);
    let grid = ein(&["solve", file, "-e", "--models", "grid"]);
    assert_eq!(grid.code, 0, "{}", grid.err);
    let none: Vec<&str> = grid
        .out
        .lines()
        .filter(|l| l.contains("grid — none:"))
        .collect();
    assert_eq!(none.len(), 3, "one line per model:\n{}", grid.out);
    let without: String = grid
        .out
        .lines()
        .filter(|l| !l.contains("grid — none:"))
        .collect::<Vec<_>>()
        .join("\n");
    // Each grid line came with a blank line before it; the listing is the same.
    let squeeze = |s: &str| {
        s.lines()
            .filter(|l| !l.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(squeeze(&without), squeeze(&list.out));
}

/// An open state gets its grid too — and the cell its debt is about is the
/// empty one.
#[test]
fn an_open_state_has_a_grid_with_its_debt_as_an_empty_cell() {
    let r = ein(&[
        "solve",
        "tests/stdlib/closure/03_closed_and_owing.ein",
        "-e",
        "--models",
        "grid",
    ]);
    assert_eq!(r.code, 0, "{}", r.err);
    assert!(r.out.contains("Open — owes"), "{}", r.out);
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
    let plain = ein(&["solve", "examples/zebra2.ein"]);
    let list = ein(&["solve", "examples/zebra2.ein", "--models", "list"]);
    assert_eq!(plain.out, list.out);
    assert!(!plain.out.contains("grid —"));
}

fn diff(file: &str, flags: &[&str], expected: &str) -> String {
    let dir = std::env::temp_dir().join(format!("ein-models-grid-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch");
    let path = dir.join(format!("{}.models", file.replace('/', "_")));
    std::fs::write(&path, expected).expect("writes");
    let mut args = vec!["solve", file];
    args.extend_from_slice(flags);
    let p = path.to_string_lossy().into_owned();
    args.extend(["--diff-models", &p]);
    let r = ein(&args);
    assert_eq!(r.code, 0, "{file}: {}", r.err);
    let _ = std::fs::remove_file(&path);
    r.out
}

/// The three shapes, each on the register's own witness and its right answer.
#[test]
fn the_diff_names_the_missing_the_spurious_and_the_one_fact_off() {
    // W1: the engine records one of the two models.
    let out = diff(
        "examples/branching/15_lookahead_two_step_on.ein",
        &["-e"],
        "closes p\nmodel\n(p A)\nmodel\n(p B)\n(p C)\n",
    );
    assert!(
        out.contains("1 matched, 0 near, 1 missing, 0 spurious"),
        "{out}"
    );
    assert!(
        out.contains("- missing: model 1 of the file [(p A)]"),
        "{out}"
    );
    // W5: a superset of a model, recorded as a third.
    let out = diff(
        "examples/ein-bugs/wrong-answers/owed-seats.ein",
        &["-e"],
        "closes seats\nmodel\n(seats Ann C1)\n(seats Bob C2)\nmodel\n(seats Ann C2)\n(seats Bob C1)\n",
    );
    assert!(
        out.contains("2 matched, 0 near, 0 missing, 1 spurious"),
        "{out}"
    );
    // W3: the right model lacks one fact.
    let out = diff(
        "examples/ein-bugs/naf-upward-closure.ein",
        &["-e"],
        "closes p q\nmodel\n(p A)\n(q A)\n",
    );
    assert!(
        out.contains("0 matched, 1 near, 0 missing, 0 spurious"),
        "{out}"
    );
    assert!(out.contains("lacks (p A)"), "{out}");
    // W1 at layer 0: no model where the right answer is one with no `q`.
    let out = diff(
        "examples/ein-bugs/wrong-answers/forced-2-two.ein",
        &["-e"],
        "closes q\nmodel\n",
    );
    assert!(
        out.contains("- missing: model 1 of the file [no facts]"),
        "{out}"
    );
}

#[test]
fn a_file_the_diff_cannot_read_is_a_usage_error_before_anything_runs() {
    let dir = std::env::temp_dir().join(format!("ein-models-grid-bad-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch");
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
    let _ = std::fs::remove_dir_all(&dir);
}
