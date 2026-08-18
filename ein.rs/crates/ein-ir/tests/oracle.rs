//! Shared plumbing for the differential tests: a long-lived `ein.py` process
//! answering frontend questions, and the repo paths both sides read.
//!
//! Compiled into each integration test that needs it via `#[path]`, because
//! Rust gives every `tests/*.rs` its own crate.

#![allow(dead_code)]

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};

pub fn repo_root() -> PathBuf {
    // crates/ein-ir → crates → ein.rs → repo
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// `ein.py`'s frontend, kept warm. Building the Lark grammar costs about half
/// a second, which is why this is one process and not one per question.
pub struct Oracle {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Ok(String),
    Err { kind: String, msg: String },
}

impl Oracle {
    /// `None` when ein.py is not importable here — a pure-Rust checkout can
    /// still run `cargo test`, it just cannot run the parity half. Every
    /// caller reports the skip loudly rather than passing silently.
    pub fn start() -> Option<Oracle> {
        let script = repo_root().join("utils/ir_oracle.py");
        let mut child = Command::new("python3")
            .arg(&script)
            .current_dir(repo_root())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()?;
        let stdin = child.stdin.take().expect("stdin");
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        let mut oracle = Oracle {
            child,
            stdin,
            stdout,
        };
        // A round-trip proves the imports worked; a broken environment fails
        // here rather than on the first interesting question.
        match oracle.try_ask(&serde_json::json!({"op": "accept", "text": "(a b)"})) {
            Some(Answer::Ok(_)) => Some(oracle),
            _ => None,
        }
    }

    fn try_ask(&mut self, req: &serde_json::Value) -> Option<Answer> {
        writeln!(self.stdin, "{req}").ok()?;
        self.stdin.flush().ok()?;
        let mut line = String::new();
        if self.stdout.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let v: serde_json::Value = serde_json::from_str(&line).ok()?;
        Some(if v["ok"].as_bool().unwrap_or(false) {
            Answer::Ok(v["out"].as_str().unwrap_or("").to_string())
        } else {
            Answer::Err {
                kind: v["kind"].as_str().unwrap_or("?").to_string(),
                msg: v["err"].as_str().unwrap_or("").to_string(),
            }
        })
    }

    pub fn ask(&mut self, req: serde_json::Value) -> Answer {
        self.try_ask(&req).expect("the oracle answered")
    }

    /// `op` over literal source text, with an optional filename.
    pub fn text(&mut self, op: &str, text: &str, filename: Option<&str>) -> Answer {
        self.ask(serde_json::json!({"op": op, "text": text, "filename": filename}))
    }

    /// `op` over a file, addressed absolutely so both sides name it the same
    /// way in `Loc`s and error messages.
    pub fn file(&mut self, op: &str, path: &Path) -> Answer {
        self.ask(serde_json::json!({"op": op, "path": path.to_str().expect("utf-8")}))
    }
}

impl Drop for Oracle {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Every `.ein` under `examples/` and `stdlib/`, sorted — the same file set
/// `conformance/corpus.toml` enumerates, discovered rather than listed so a
/// new fixture is covered the moment it lands.
pub fn corpus_files() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ["examples", "stdlib"] {
        collect(&root.join(dir), &mut out);
    }
    out.sort();
    out
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect(&p, out);
        } else if p.extension().is_some_and(|x| x == "ein") {
            out.push(p);
        }
    }
}

/// Print a skip that a human will actually see. `cargo test` swallows stdout
/// for passing tests, so this goes to stderr.
pub fn skip(what: &str) {
    eprintln!(
        "SKIP {what}: ein.py is not runnable here \
         (need `python3` + lark; try `pip install -e ein.py`)"
    );
}
