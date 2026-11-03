//! **The read-out's golden is the page the user read** — M1e2
//! [T1e2.1a.5.2](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.5_the_read_out.md).
//!
//! [`read_out_examples.md`](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md)
//! holds real runs: each block is captioned with the `ein solve` command that
//! printed it, and is that command's stdout, then its stderr lines as
//! `stderr: …`, then `exit N`. `utils/read_out_examples.py` writes the page
//! from the binary. This test runs every caption again and compares the
//! block byte for byte, so the page cannot drift from what `ein solve` prints.
//! A caption with `--dump-answer DIR` is compared with the `answer.txt` it
//! writes.
//!
//! Skipped: a block whose program is not in the repository (the page's one
//! scratch variant), a block that quotes only part of a run (`, the query
//! block`), and — unless `EIN_CORPUS_SLOW=1` — zebra2 at `--region-budget
//! 33554432`, 22 s at `-j 8` in a release build.

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const PAGE: &str = "plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md";

struct Block {
    line: usize,
    args: Vec<String>,
    body: String,
}

/// Every block whose caption is a whole `ein solve` run.
fn blocks(page: &str) -> Vec<Block> {
    let lines: Vec<&str> = page.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        let whole = l.starts_with("`ein solve ")
            && l[1..].find('`').is_some_and(|end| {
                let rest = &l[end + 2..];
                rest.is_empty() || rest.starts_with(" — ")
            });
        if whole && lines.get(i + 2) == Some(&"```text") {
            let end = l[1..].find('`').expect("closed") + 1;
            let args: Vec<String> = l["`ein solve ".len()..end]
                .split_whitespace()
                .map(str::to_string)
                .collect();
            let mut j = i + 3;
            let mut body = Vec::new();
            while lines[j] != "```" {
                body.push(lines[j]);
                j += 1;
            }
            out.push(Block {
                line: i + 1,
                args,
                body: body.join("\n"),
            });
            i = j;
        }
        i += 1;
    }
    out
}

/// The block a run prints: stdout, the stderr lines, the exit code — the
/// composition `utils/read_out_examples.py` writes.
fn printed(args: &[String]) -> String {
    let mut run: Vec<String> = Vec::new();
    // A `--dump-answer` goes to a fresh `answer/` in a scratch directory that
    // is removed when this returns, or panics.
    let tmp = Scratch::new("ein-read-out-examples");
    let mut dump: Option<PathBuf> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--dump-answer" {
            it.next();
            let d = tmp.join("answer");
            run.push(a.clone());
            run.push(d.to_string_lossy().to_string());
            dump = Some(d);
        } else {
            run.push(a.clone());
        }
    }
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .args(&run)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let stdout = match &dump {
        // What `--dump-answer DIR` wrote, with DIR as the page spells it.
        Some(d) => std::fs::read_to_string(d.join("answer.txt"))
            .expect("answer.txt")
            .replace(&d.to_string_lossy().to_string(), "DIR"),
        None => String::from_utf8_lossy(&out.stdout).into_owned(),
    };
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let mut body = stdout.trim_end_matches('\n').to_string();
    if !stderr.trim().is_empty() {
        body.push_str("\n\n");
        let lines: Vec<String> = stderr.lines().map(|l| format!("stderr: {l}")).collect();
        body.push_str(&lines.join("\n"));
    }
    body.push_str(&format!("\n\nexit {}", out.status.code().unwrap_or(-1)));
    body
}

#[test]
fn every_block_is_what_ein_solve_prints() {
    let page = std::fs::read_to_string(repo_root().join(PAGE)).expect("the page");
    let slow = std::env::var("EIN_CORPUS_SLOW").is_ok_and(|v| v == "1");
    let mut checked = 0usize;
    let mut failures = Vec::new();
    for b in blocks(&page) {
        let Some(file) = b.args.last() else { continue };
        if !Path::new(&repo_root().join(file)).exists() {
            continue;
        }
        if !slow && b.args.iter().any(|a| a == "33554432") {
            continue;
        }
        let got = printed(&b.args);
        if got != b.body {
            let diff: Vec<String> = got
                .lines()
                .zip(b.body.lines())
                .enumerate()
                .filter(|(_, (g, w))| g != w)
                .take(3)
                .map(|(k, (g, w))| format!("    line {}: printed {g:?}\n    page    {w:?}", k + 1))
                .collect();
            failures.push(format!(
                "{PAGE}:{}: `ein solve {}` ({} lines printed, {} on the page)\n{}",
                b.line,
                b.args.join(" "),
                got.lines().count(),
                b.body.lines().count(),
                diff.join("\n")
            ));
        }
        checked += 1;
    }
    assert!(
        failures.is_empty(),
        "{}\n\nre-write the page with `python3 utils/read_out_examples.py`",
        failures.join("\n")
    );
    assert!(checked >= 20, "{checked} blocks checked");
}
