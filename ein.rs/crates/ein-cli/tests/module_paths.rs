//! **A module is named as the program names it** (Q-M1f.93): a message, a
//! finding's place and the run summary name an imported module by the
//! program's directory as it was given, joined with the module's name as
//! written, and never by a path the program did not give, an absolute one
//! made from the working directory among them. So a program named three ways
//! — from the repository's root as `examples/x.ein` and as `./examples/x.ein`,
//! and from `ein.rs/` as `../examples/x.ein` — says the same three times but
//! for its own path as given.
//!
//! The programs are the corpus's refused ones, whose messages name the
//! modules an import cycle, a missing module and a module's parse error are
//! about, and every one that imports a module by its file-relative name.
//! What names the standard library's modules names the library the binary
//! found, which no program gives and no working directory moves.

use std::path::{Path, PathBuf};
use std::process::Command;

use ein_corpus::{Scratch, repo_root};
use serde_json::Value as J;

/// What one run said: stdout, stderr, and its summary's `source` and
/// `diagnostics`.
struct Said {
    code: i32,
    text: String,
}

/// `ein solve PROGRAM --no-search --json-summary F` in `cwd`, what it said
/// with the program's path as given written `{FILE}` and its directory as
/// given `{DIR}`, and the standard library the binary reads `{STDLIB}`.
fn said(cwd: &Path, program: &str, summary: &Path, stdlib: &str) -> Said {
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .args(["solve", program, "--no-search", "--json-summary"])
        .arg(summary)
        .current_dir(cwd)
        .output()
        .expect("the `ein` binary runs");
    let s: Option<J> = std::fs::read_to_string(summary)
        .ok()
        .map(|t| serde_json::from_str(&t).expect("the summary parses"));
    let dir = Path::new(program)
        .parent()
        .map(|d| d.display().to_string())
        .unwrap_or_default();
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&out.stdout));
    text.push_str("\n--- stderr\n");
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    if let Some(s) = s {
        text.push_str(&format!(
            "\n--- summary\n{}\n{}\n",
            s["source"], s["diagnostics"]
        ));
    }
    let text = text
        .replace(stdlib, "{STDLIB}")
        .replace(program, "{FILE}")
        .replace(&format!("{dir}/"), "{DIR}/");
    Said {
        code: out.status.code().unwrap_or(-1),
        text,
    }
}

/// The corpus's refused programs, and every program that imports a module by
/// its file-relative name, not `slow`.
fn programs() -> Vec<String> {
    let root = repo_root();
    ein_corpus::corpus()
        .entry
        .into_iter()
        .filter(|e| !e.slow)
        .filter(|e| {
            matches!(e.group.as_str(), "load-negative" | "parse-negative")
                || std::fs::read_to_string(root.join(&e.path)).is_ok_and(|t| {
                    t.lines().any(|l| {
                        l.trim_start()
                            .strip_prefix("(import ")
                            .is_some_and(|m| !m.starts_with("std."))
                    })
                })
        })
        .map(|e| e.path)
        .collect()
}

#[test]
fn a_module_is_named_as_the_program_names_it() {
    let root = repo_root();
    let stdlib: PathBuf = std::env::var_os("EIN_STDLIB")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("stdlib"));
    let stdlib = stdlib.display().to_string();
    let elsewhere = root.join("ein.rs");
    let programs = programs();
    assert!(
        programs.len() > 60,
        "only {} programs to read",
        programs.len()
    );
    let tmp = Scratch::new("ein-module-paths");
    let found: Vec<Result<bool, String>> = ein_corpus::pool::map(
        "module_paths",
        &programs.iter().enumerate().collect::<Vec<_>>(),
        |(_, p)| (p.to_string(), "solve --no-search".to_string()),
        |(i, rel)| {
            let ways = [
                (root.as_path(), rel.to_string()),
                (root.as_path(), format!("./{rel}")),
                (elsewhere.as_path(), format!("../{rel}")),
            ];
            let runs: Vec<Said> = ways
                .iter()
                .enumerate()
                .map(|(k, (cwd, program))| {
                    said(cwd, program, &tmp.join(format!("{i:04}-{k}.json")), &stdlib)
                })
                .collect();
            for (k, r) in runs.iter().enumerate().skip(1) {
                if r.code != runs[0].code || r.text != runs[0].text {
                    return Err(format!(
                        "{rel}: named {:?} from {}, it says another thing than named {rel:?} from \
                         the root:\n--- {rel}\n{}\n--- {}\n{}",
                        ways[k].1,
                        ways[k].0.display(),
                        runs[0].text,
                        ways[k].1,
                        r.text
                    ));
                }
            }
            if runs[0].text.contains(&root.display().to_string()) {
                return Err(format!(
                    "{rel}: names a path the program did not give:\n{}",
                    runs[0].text
                ));
            }
            // Whether a line names one of the program's modules.
            Ok(runs[0].text.contains("{DIR}/"))
        },
    );
    let mut named = 0usize;
    let mut bad = Vec::new();
    for f in found {
        match f {
            Ok(true) => named += 1,
            Ok(false) => {}
            Err(e) => bad.push(e),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n\n"));
    assert!(
        named >= 5,
        "only {named} programs said a line naming one of their modules"
    );
}
