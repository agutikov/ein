//! **The regressions, as a test.**
//!
//! `corpus/regressions.toml` holds one row per program and run that was once
//! answered wrong: what the engine answers today, `[case.now]`, what it
//! should, `[case.right]`, and the witness's region, `[case.region]`.
//! `ein-corpus` holds, with no run, that each row's `[case.now]` answers its
//! `[case.right]`; this file runs every row's `[case.now]` through the binary
//! and holds the engine to it, and recomputes every row's region with the
//! library and holds the row to it (both [`held`]'s), so a run that stops
//! giving the right answer fails here.
//!
//! **Through the binary, as a process**, as `corpus_cli.rs` is: a row is a
//! claim about what `ein <run>` answers. Both builds hold every row.
//!
//! ```text
//! cargo test -p ein-cli --test regressions
//! EIN_BLESS=1 cargo test -p ein-cli --test regressions   # re-bank the region blocks
//! ```

mod held;

use std::path::Path;

use ein_corpus::regressions::REGRESSIONS;
use ein_corpus::{Scratch, regressions, repo_root};
use held::regions::bless;

#[test]
fn every_regression_answers_what_it_records() {
    let file = regressions();
    let scratch = Scratch::new("ein-regressions");
    let got = held::hold(Path::new(env!("CARGO_BIN_EXE_ein")), &file.case, &scratch);
    assert!(got.runs > 0, "the regressions recorded no runs");
    assert!(
        got.bad.is_empty(),
        "{} of {} runs disagree with corpus/regressions.toml:\n\n{}",
        got.bad.len(),
        got.runs,
        got.bad.join("\n\n")
    );
}

#[test]
fn every_row_states_its_witness_s_region() {
    let blessing = std::env::var("EIN_BLESS").as_deref() == Ok("1");
    let (checked, bad) = held::regions::check(&repo_root().join(REGRESSIONS), blessing);
    assert!(checked > 0, "the regressions have no rows to check");
    assert!(
        bad.is_empty(),
        "{} of {checked} rows state a region the library does not compute \
         (re-bank with EIN_BLESS=1 cargo test -p ein-cli --test regressions):\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
}

/// **The bless writes a region block before the row's `[case.right]`
/// table**, never into a string that names the table: a note ending
/// *`[case.right].derived` carries the count* stays whole, the block lands on
/// the line before the table's header, and the row still parses. A row that
/// states a `[case.region]` table has it replaced, a note that names it left
/// as it was.
#[test]
fn the_bless_writes_before_the_right_table_and_not_into_a_note() {
    let block = "[case.region]\nroute    = \"state route\"\nk_s      = 1";
    let row = |region: &str| {
        format!(
            "[[case]]\nid       = \"X/one\"\nnote     = \"`[case.region]` and `[case.right]`.derived \
             carry the counts\"\n\n[case.wrong]\nk = 2\n\n{region}[case.right]\nk = 1\n\n[[case]]\n\
             id       = \"X/two\"\n\n[case.right]\nk = 3\n"
        )
    };
    let note = "note     = \"`[case.region]` and `[case.right]`.derived carry the counts\"\n";
    for region in ["", "[case.region]\nroute    = \"cube\"\nk_s      = 9\n\n"] {
        let text = row(region);
        let got = bless(&text, "X/one", block);
        assert!(got.contains(note), "the note moved:\n{got}");
        assert!(
            got.contains(&format!("{block}\n\n[case.right]\nk = 1\n")),
            "the block is not before the table:\n{got}"
        );
        assert_eq!(got.matches("[case.region]\n").count(), 1, "{got}");
        assert!(!got.contains("\"cube\""), "the old block stayed:\n{got}");
        // The next row is untouched, and the whole still parses as TOML.
        assert!(got.ends_with("[[case]]\nid       = \"X/two\"\n\n[case.right]\nk = 3\n"));
        let parsed = toml::from_str::<toml::Table>(&got);
        assert!(parsed.is_ok(), "{parsed:?}\n{got}");
    }
}
