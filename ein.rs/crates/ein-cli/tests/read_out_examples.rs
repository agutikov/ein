//! **The read-out's golden is the reference's examples page**,
//! [`read-out-examples.md`](../../../../docs/ein/user/reference/read-out-examples.md),
//! beside the reference.
//!
//! The page holds real runs: each block is captioned with the `ein solve` or
//! `ein oracle` command that printed it, and is that command's stdout, then
//! its stderr lines as `stderr: …`, then `exit N`.
//! The page is generated from the binary. This test
//! runs every caption again and compares the block byte for byte, so the page
//! cannot drift from what the binary prints. A caption with
//! `--dump-answer DIR` is compared with the `answer.txt` it writes.
//!
//! Skipped: a block whose program is not in the repository (the page's one
//! scratch variant), a block that quotes only part of a run (`, the query
//! block`), and — unless `EIN_CORPUS_SLOW=1` — zebra2 at `--region-budget
//! 33554432`, the page's one slow run.
//!
//! What those skips and the prose leave, the gate holds: the page is its
//! generator's output whole, the variant's block among it, written with the
//! binary the tests ran.

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, repo_root};

const PAGE: &str = "docs/ein/user/reference/read-out-examples.md";

struct Block {
    line: usize,
    /// `solve` or `oracle`.
    command: String,
    args: Vec<String>,
    body: String,
}

/// Every block whose caption is a whole `ein solve` or `ein oracle` run.
fn blocks(page: &str) -> Vec<Block> {
    let lines: Vec<&str> = page.lines().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        let command = ["solve", "oracle"]
            .into_iter()
            .find(|c| l.starts_with(&format!("`ein {c} ")));
        let whole = command.is_some()
            && l[1..].find('`').is_some_and(|end| {
                let rest = &l[end + 2..];
                rest.is_empty() || rest.starts_with(" — ")
            });
        if whole && lines.get(i + 2) == Some(&"```text") {
            let end = l[1..].find('`').expect("closed") + 1;
            let command = command.expect("a whole run's").to_string();
            let args: Vec<String> = l[format!("`ein {command} ").len()..end]
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
                command,
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
/// composition the page's generator writes.
fn printed(command: &str, args: &[String]) -> String {
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
        .arg(command)
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
    let runs: Vec<Block> = blocks(&page)
        .into_iter()
        .filter(|b| {
            b.args
                .last()
                .is_some_and(|file| Path::new(&repo_root().join(file)).exists())
        })
        .filter(|b| slow || !b.args.iter().any(|a| a == "33554432"))
        .collect();
    // Every block's run on the corpus sweeps' pool, read back
    // in the page's order.
    let printed_all = ein_corpus::pool::map(
        "read_out_examples",
        &runs,
        |b| {
            (
                b.args.last().cloned().unwrap_or_default(),
                format!("ein {} {}", b.command, b.args.join(" ")),
            )
        },
        |b| printed(&b.command, &b.args),
    );
    for (b, got) in runs.iter().zip(printed_all) {
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
                "{PAGE}:{}: `ein {} {}` ({} lines printed, {} on the page)\n{}",
                b.line,
                b.command,
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
        "{}\n\nthe page is generated: write it again from the binary",
        failures.join("\n")
    );
    assert!(checked >= 30, "{checked} blocks checked");
    assert!(
        blocks(&page).iter().any(|b| b.command == "oracle"),
        "no `ein oracle` block"
    );
}
