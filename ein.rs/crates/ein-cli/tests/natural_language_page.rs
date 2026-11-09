//! **[`09_natural_language.md`](../../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)
//! is held to the engine** — M1f S1f.5.14's T1f.5.14.4, under the phase's rule
//! that a page the stage adds is pinned by something that runs.
//!
//! Three of its claims are checked here:
//!
//! - its § 3 block, between `<!-- state-file: FILE -->` and `<!--
//!   /state-file -->`, is lines of the state file `ein solve FILE
//!   --dump-answer DIR` writes, each one verbatim;
//! - its § 4 registers are the four `utils/nl_census.py` assigns, and the
//!   census's `kernel` register is [`ein_core::RESERVED`], name for name;
//! - every `Q-M1f.<n>` it cites is a heading of M1f's `open_questions.md`.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const PAGE: &str = "docs/kernel/ir/03-ein-lang/09_natural_language.md";
const CENSUS: &str = "utils/nl_census.py";
const OPEN: &str = "<!-- state-file: ";
const CLOSE: &str = "<!-- /state-file -->";

fn page() -> String {
    std::fs::read_to_string(repo_root().join(PAGE)).expect("the page")
}

#[test]
fn the_pages_state_file_lines_are_a_runs() {
    let page = page();
    let start = page.find(OPEN).expect("the page marks a state-file block");
    let rest = &page[start + OPEN.len()..];
    let (file, rest) = rest.split_once(" -->").expect("the marker closes");
    let block = &rest[..rest.find(CLOSE).expect("the block closes")];
    let lines: Vec<&str> = block.lines().filter(|l| l.starts_with('(')).collect();
    assert!(lines.len() >= 3, "the block shows {} lines", lines.len());

    let scratch = Scratch::new("ein-nl-page");
    let dir = scratch.path().join("answer");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--dump-answer", &dir.to_string_lossy()])
        .current_dir(repo_root())
        .output()
        .expect("the binary runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let state = std::fs::read_to_string(dir.join("models").join("01.ein")).expect("a state file");
    let written: Vec<&str> = state.lines().collect();
    for l in lines {
        assert!(
            written.contains(&l),
            "{PAGE}'s § 3 shows a line `ein solve {file} --dump-answer DIR` does not write:\n{l}\n\
             (re-take it from DIR/models/01.ein)"
        );
    }
}

#[test]
fn the_pages_registers_are_the_census_and_the_kernel_is_reserved() {
    let page = page();
    let census = std::fs::read_to_string(repo_root().join(CENSUS)).expect("the census");
    for reg in ["puzzle", "stdlib", "kernel", "undeclared"] {
        assert!(
            page.contains(&format!("| **{reg}** |")),
            "{PAGE}: no register row {reg}"
        );
        assert!(
            census.contains(&format!("return \"{reg}\"")),
            "{CENSUS}: no register {reg}"
        );
    }
    let line = census
        .lines()
        .skip_while(|l| !l.starts_with("KERNEL = {"))
        .take_while(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let mut kernel: Vec<String> = line
        .split('"')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, s)| s.to_string())
        .collect();
    kernel.sort();
    let mut reserved: Vec<String> = ein_core::RESERVED.iter().map(|s| s.to_string()).collect();
    reserved.sort();
    assert_eq!(
        kernel, reserved,
        "{CENSUS}'s KERNEL is not ein_core::RESERVED"
    );
}

#[test]
fn every_question_the_page_cites_is_open_questions_heading() {
    let page = page();
    let questions = std::fs::read_to_string(
        repo_root().join("plans/m1f_hypothesis_and_documentation/open_questions.md"),
    )
    .expect("M1f's questions");
    let mut cited = 0;
    for (i, _) in page.match_indices("[Q-M1f.") {
        let id: String = page[i + 1..].chars().take_while(|&c| c != ']').collect();
        cited += 1;
        assert!(
            questions.contains(&format!("### {id} — ")),
            "{PAGE} cites {id}, which open_questions.md has no heading for"
        );
    }
    assert!(cited > 0, "{PAGE} cites no question");
}
