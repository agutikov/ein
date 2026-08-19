//! `ein.py` as a long-lived oracle — test plumbing, shared by every crate
//! that has to prove it renders what the Python implementation renders.
//!
//! **Not part of the engine.** It is a dev-dependency only (`publish = false`,
//! nothing in the workspace depends on it outside `[dev-dependencies]`), and
//! it exists because the alternative is the same sixty lines of process
//! plumbing copied into every `tests/` directory from
//! [P1a.1](../../../plans/m1a_rust/p1a.1_ir_frontend/README.md) to
//! [P1a.5](../../../plans/m1a_rust/p1a.5_presentation/README.md).
//!
//! The conformance harness cannot serve here: it compares two `ein` CLIs, and
//! most of what needs checking during the port — the AST, the dumper,
//! `repr()`, a float's field width — has no CLI surface. Two scripts back it:
//!
//! | script | answers |
//! |---|---|
//! | [`IR_ORACLE`] | `ein.py`'s frontend: parse, dump, resolve imports, minimise |
//! | [`PY_ORACLE`] | CPython itself: `repr()` and `format()` |
//!
//! Both speak one JSON object per line in each direction, and both are kept
//! *warm*: building the Lark grammar costs about half a second, and the
//! differential fuzzer sends a million inputs.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};

/// `ein.py`'s IR frontend.
pub const IR_ORACLE: &str = "utils/ir_oracle.py";
/// CPython's `repr()` / `format()`.
pub const PY_ORACLE: &str = "utils/py_oracle.py";

/// The repo root, found from the compiling crate rather than the working
/// directory, so a test runs the same from anywhere.
pub fn repo_root() -> PathBuf {
    // crates/<crate> → crates → ein.rs → repo
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// One warm oracle process.
pub struct Oracle {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

/// What an oracle answered. `Ok` carries the rendering; `Err` carries the
/// message ein.py would print, which is usually the thing under test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Ok(String),
    Err { kind: String, msg: String },
}

impl Answer {
    /// The rendering, or a panic naming the failure — for the many call sites
    /// where an error is not an expected outcome.
    pub fn unwrap(&self) -> &str {
        match self {
            Answer::Ok(s) => s,
            Answer::Err { kind, msg } => panic!("oracle failed: {kind}: {msg}"),
        }
    }
}

impl Oracle {
    /// Start `script` (one of [`IR_ORACLE`] / [`PY_ORACLE`]), or `None` when
    /// Python cannot run it here.
    ///
    /// A pure-Rust checkout can still `cargo test`; it just cannot run the
    /// parity half. Every caller reports that as a skip a human will see —
    /// [`skip`] — because a gate that passes when it did not run is not a
    /// gate.
    pub fn start(script: &str) -> Option<Oracle> {
        let mut child = Command::new("python3")
            .arg(repo_root().join(script))
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
        // A round-trip proves the imports worked, so a broken environment
        // fails here rather than on the first interesting question.
        let probe = if script == PY_ORACLE {
            serde_json::json!({"op": "repr", "v": {"s": "probe"}})
        } else {
            serde_json::json!({"op": "accept", "text": "(a b)"})
        };
        match oracle.try_ask(&probe) {
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

    /// An [`IR_ORACLE`] op over literal source text.
    pub fn text(&mut self, op: &str, text: &str, filename: Option<&str>) -> Answer {
        self.ask(serde_json::json!({"op": op, "text": text, "filename": filename}))
    }

    /// An [`IR_ORACLE`] op over a file, addressed **absolutely** so both sides
    /// name it the same way in `Loc`s and in error messages.
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

/// Every `.ein` under `examples/` and `stdlib/`, sorted — the file set
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

/// Print a skip a human will actually see. `cargo test` swallows stdout for
/// passing tests, so this goes to stderr.
pub fn skip(what: &str) {
    eprintln!(
        "SKIP {what}: the oracle is not runnable here \
         (need `python3` + lark; try `pip install -e ein.py`)"
    );
}

/// A checked-in **ein.rs** golden: compare, or rewrite under `EIN_BLESS=1`.
///
/// Distinct from the goldens under `ein.py/tests/golden/`, which are the
/// *oracle's* and are read-only here — a port that shipped its own copy of the
/// expected bytes would prove only that it agrees with itself. These are the
/// other kind, and since
/// [S1a.6.10](../../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
/// they are the whole regression coverage of everything the parity contract
/// stopped comparing: a shipping engine is compared against fixtures, not
/// against an oracle.
///
/// Returns `None` when the golden matches (or was just written), and
/// otherwise the message to fail with — naming the first differing line and
/// the one command that regenerates it, because a golden without a documented
/// regeneration gets edited by hand and drifts.
///
/// ```text
/// EIN_BLESS=1 cargo test -p ein-render
/// ```
pub fn golden(path: &Path, got: &str) -> Option<String> {
    let how = "regenerate with EIN_BLESS=1 cargo test";
    if std::env::var("EIN_BLESS").as_deref() == Ok("1") {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        std::fs::write(path, got).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        return None;
    }
    let want = match std::fs::read_to_string(path) {
        Ok(w) => w,
        Err(e) => return Some(format!("{}: {e}\n  {how}", path.display())),
    };
    if got == want {
        return None;
    }
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let at = got
        .lines()
        .zip(want.lines())
        .position(|(g, w)| g != w)
        .unwrap_or_else(|| got.lines().count().min(want.lines().count()));
    Some(format!(
        "{name} differs at line {}:\n  got  {:?}\n  want {:?}\n  {how}",
        at + 1,
        got.lines().nth(at).unwrap_or("<end of file>"),
        want.lines().nth(at).unwrap_or("<end of file>"),
    ))
}

/// Where an ein.rs golden lives: `<crate>/tests/golden/<name>`.
pub fn golden_path(krate: &str, name: &str) -> PathBuf {
    repo_root()
        .join("ein.rs/crates")
        .join(krate)
        .join("tests/golden")
        .join(name)
}
