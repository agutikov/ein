//! **[`14-natural-language.md`](../../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)
//! is held to the engine**: a page is pinned by something that runs.
//!
//! Three of its claims are checked here:
//!
//! - its § 3 block, between `<!-- trace: FILE -->` and `<!-- /trace -->`, is
//!   lines of the trace `ein solve FILE --trace OUT.md` writes, each one
//!   verbatim;
//! - its § 4 has a row for each of the four registers, and the `kernel`
//!   row's names are [`ein_core::RESERVED`], name for name;
//! - every decision id it cites links the decision's own file under
//!   `docs/decisions/`.

use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const PAGE: &str = "docs/ein/user/03-syntax-and-semantics/14-natural-language.md";
const OPEN: &str = "<!-- trace: ";
const CLOSE: &str = "<!-- /trace -->";

fn page() -> String {
    std::fs::read_to_string(repo_root().join(PAGE)).expect("the page")
}

#[test]
fn the_pages_trace_lines_are_a_runs() {
    let page = page();
    let start = page.find(OPEN).expect("the page marks a trace block");
    let rest = &page[start + OPEN.len()..];
    let (file, rest) = rest.split_once(" -->").expect("the marker closes");
    let block = &rest[..rest.find(CLOSE).expect("the block closes")];
    let lines: Vec<&str> = block
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with("```"))
        .collect();
    assert!(lines.len() >= 3, "the block shows {} lines", lines.len());

    let scratch = Scratch::new("ein-nl-page");
    let trace = scratch.path().join("trace.md");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", file, "--trace", &trace.to_string_lossy(), "-G"])
        .current_dir(repo_root())
        .output()
        .expect("the binary runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let written = std::fs::read_to_string(&trace).expect("a trace");
    let written: Vec<&str> = written.lines().collect();
    for l in lines {
        assert!(
            written.contains(&l),
            "{PAGE}'s § 3 shows a line `ein solve {file} --trace OUT.md` does not write:\n{l}\n\
             (re-take it from OUT.md)"
        );
    }
}

#[test]
fn the_pages_registers_are_four_and_the_kernel_is_reserved() {
    let page = page();
    for reg in ["puzzle", "stdlib", "kernel", "undeclared"] {
        assert!(
            page.contains(&format!("| **{reg}** |")),
            "{PAGE}: no register row {reg}"
        );
    }
    let row = page
        .lines()
        .find(|l| l.starts_with("| **kernel** |"))
        .expect("the kernel row");
    let names = row.split('|').nth(2).expect("the row's second column");
    let mut kernel: Vec<String> = names
        .split('`')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, s)| s.to_string())
        .collect();
    kernel.sort();
    let mut reserved: Vec<String> = ein_core::RESERVED.iter().map(|s| s.to_string()).collect();
    reserved.sort();
    assert_eq!(
        kernel, reserved,
        "{PAGE}'s kernel register is not ein_core::RESERVED"
    );
}

/// Every `[Q-M1f.N](…)` the page cites links a file under `docs/decisions/`
/// whose name begins with the id.
#[test]
fn every_decision_the_page_cites_links_its_file() {
    let page = page();
    let dir = repo_root().join(PAGE);
    let dir = dir.parent().expect("the page's directory");
    let mut cited = 0;
    for (i, _) in page.match_indices("[Q-M1f.") {
        let rest = &page[i + 1..];
        let id: String = rest.chars().take_while(|&c| c != ']').collect();
        let target = rest[id.len()..]
            .strip_prefix("](")
            .and_then(|t| t.split(')').next())
            .unwrap_or_else(|| panic!("{PAGE} cites {id} with no link"));
        let file = dir.join(target);
        let name = file
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        cited += 1;
        assert!(
            file.is_file() && name.starts_with(&format!("{id}-")) && target.contains("decisions/"),
            "{PAGE} cites {id} by {target}, which is not its decision's file"
        );
    }
    assert!(cited > 0, "{PAGE} cites no question");
}
