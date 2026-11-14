//! The corpus, and the plumbing every crate's tests share to read it.
//!
//! **Not part of the engine.** `publish = false`, and nothing outside a
//! `[dev-dependencies]` block or a bench depends on it. Three things live here
//! because all three are "test inputs, and how a test finds them":
//!
//! | | |
//! |---|---|
//! | [`manifest`] | `corpus/corpus.toml` — one entry per `.ein` file, the runs it is exercised under, and the completeness check that fails on a file with no entry |
//! | [`plan`] | a run name → an `ein` argv, which is what makes a manifest row executable |
//! | [`mod@register`] | `corpus/wrong_answers.toml` — one row per witness of every known wrong answer, with today's answer and the right one (M1e2 S1e2.1.1) |
//! | this module | [`repo_root`], [`corpus_files`], [`golden`], [`golden_path`] — found from the compiling crate, so a test runs the same from anywhere; and [`Scratch`], the one way a test makes a directory it does not keep |
//!
//! What is *not* here is a `skip` helper. A skip that prints to stderr —
//! which `cargo test` captures for a passing test — reports a pass while
//! asserting nothing. Nothing in this crate can be skipped: the corpus is
//! checked in, and a fixture that cannot be found is a failure.

pub mod manifest;
pub mod mutants;
pub mod plan;
pub mod register;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use manifest::{Corpus, Entry};
pub use register::{Register, Row};

/// The repo root, found from the compiling crate rather than the working
/// directory, so a test runs the same from anywhere.
pub fn repo_root() -> PathBuf {
    // crates/<crate> → crates → ein.rs → repo
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
}

/// **The imports a test program writes, under M1f S1f.6.35's full names**:
/// for each `(module, entries)`, one `(import MODULE :symbols (…))` line
/// listing the entries and every name the module declares that `body` writes,
/// in the order `body` first writes them; a module with nothing listed writes
/// no line. An import brings its module whole since S1f.6.35 and a list only
/// names, so a program lists the names it writes from the module that
/// declares them (`std.math.int`'s tests list `n` from `std.math.decimal-arith`). A name two of
/// the modules declare is the first one's.
///
/// `module` is a `std.*` name, read from the checkout's `stdlib/`; a
/// declaration is a line that starts `(relation`, `(rule`, `(hrule` or
/// `(macro`, which is how every stdlib module writes one.
pub fn import_lines(modules: &[(&str, &[&str])], body: &str) -> String {
    let declared: Vec<std::collections::BTreeSet<String>> = modules
        .iter()
        .map(|(m, _)| {
            let rel = m
                .strip_prefix("std.")
                .expect("a std.* module")
                .replace('.', "/");
            let text =
                std::fs::read_to_string(repo_root().join("stdlib").join(format!("{rel}.ein")))
                    .unwrap_or_else(|e| panic!("{m}: {e}"));
            text.lines()
                .filter_map(|l| {
                    let rest = ["(relation ", "(rule ", "(hrule ", "(macro "]
                        .iter()
                        .find_map(|d| l.strip_prefix(d))?;
                    rest.split_whitespace().next().map(str::to_string)
                })
                .collect()
        })
        .collect();
    let mut lists: Vec<Vec<String>> = modules
        .iter()
        .map(|(_, entries)| entries.iter().map(|e| e.to_string()).collect())
        .collect();
    // The names `body` writes, outside strings and comments, in order.
    let mut tokens: Vec<String> = Vec::new();
    let mut chars = body.chars().peekable();
    let mut tok = String::new();
    while let Some(c) = chars.next() {
        if c == '"' {
            while let Some(d) = chars.next() {
                if d == '\\' {
                    chars.next();
                } else if d == '"' {
                    break;
                }
            }
        } else if c == ';' {
            for d in chars.by_ref() {
                if d == '\n' {
                    break;
                }
            }
        } else if c.is_whitespace() || c == '(' || c == ')' {
            if !tok.is_empty() {
                tokens.push(std::mem::take(&mut tok));
            }
        } else {
            tok.push(c);
        }
    }
    if !tok.is_empty() {
        tokens.push(tok);
    }
    for t in tokens {
        if lists.iter().any(|l| l.contains(&t)) {
            continue;
        }
        if let Some(i) = declared.iter().position(|d| d.contains(&t)) {
            lists[i].push(t);
        }
    }
    modules
        .iter()
        .zip(lists)
        .filter(|(_, l)| !l.is_empty())
        .map(|((m, _), l)| format!("(import {m} :symbols ({}))\n", l.join(" ")))
        .collect()
}

/// **A test program's text with its leading `std.*` imports rewritten to name
/// what it writes** ([`import_lines`]): each `(import std.M :symbols (…))` at
/// the head of `src` keeps its entries, and the modules each imports, read
/// off the stdlib's own `(import …)` lines, follow with nothing of their own;
/// every one then lists the names `src`'s body writes that it declares. A
/// text that starts with no import is returned as it is. What a test that
/// built a program on `(import std.math.decimal-arith :symbols (decimal-arith))` writes since M1f
/// S1f.6.35, where a list only names.
pub fn listed(src: &str) -> String {
    let mut modules: Vec<(String, Vec<String>)> = Vec::new();
    let mut rest = src;
    loop {
        let trimmed = rest.trim_start();
        let Some(form) = trimmed.strip_prefix("(import ") else {
            break;
        };
        // The form's end: its closing paren, the list's counted in.
        let mut depth = 1;
        let mut end = None;
        for (i, c) in form.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { break };
        let head = &form[..end];
        let module = head.split_whitespace().next().unwrap_or("").to_string();
        let entries: Vec<String> = head
            .split_once(":symbols (")
            .map(|(_, l)| {
                l.trim_end_matches(')')
                    .split_whitespace()
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        modules.push((module, entries));
        rest = &form[end + 1..];
    }
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    // Only a head of `std.*` selections is rewritten: a whole import names
    // nothing, and a file-relative module is not the stdlib's to read.
    if modules.is_empty()
        || modules
            .iter()
            .any(|(m, e)| !m.starts_with("std.") || m.ends_with(')') || e.is_empty())
    {
        return src.to_string();
    }
    let mut i = 0;
    while i < modules.len() {
        let rel = modules[i].0["std.".len()..].replace('.', "/");
        let text = std::fs::read_to_string(repo_root().join("stdlib").join(format!("{rel}.ein")))
            .unwrap_or_default();
        for l in text.lines() {
            // A bare import, `(import std.math.decimal-arith)`, closes on the
            // module's name.
            if let Some(m) = l
                .strip_prefix("(import ")
                .and_then(|r| r.split_whitespace().next())
                .map(|m| m.trim_end_matches(')'))
                && !modules.iter().any(|(x, _)| x == m)
            {
                modules.push((m.to_string(), Vec::new()));
            }
        }
        i += 1;
    }
    let refs: Vec<(&str, Vec<&str>)> = modules
        .iter()
        .map(|(m, e)| (m.as_str(), e.iter().map(String::as_str).collect()))
        .collect();
    let args: Vec<(&str, &[&str])> = refs.iter().map(|(m, e)| (*m, e.as_slice())).collect();
    format!("{}{rest}", import_lines(&args, rest))
}

/// Every spelling a stdlib name may print in since M1f S1f.6.35: its short
/// name, where the program lists it, and its full name under each stdlib
/// module, where it does not. A test that reads a relation by name looks for
/// each in turn.
///
/// Every module at any depth: `stdlib/math/decimal-arith.ein` is
/// `std.math.decimal-arith` (M1f S1f.6.31).
pub fn spellings(short: &str) -> Vec<String> {
    let mut out = vec![short.to_string()];
    let dir = repo_root().join("stdlib");
    let mut modules: Vec<String> = ein_files_under(&dir)
        .into_iter()
        .filter_map(|p| {
            let rel = p.strip_prefix(&dir).ok()?.with_extension("");
            let parts: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect();
            Some(parts.join("."))
        })
        .collect();
    modules.sort();
    out.extend(modules.into_iter().map(|m| format!("std.{m}.{short}")));
    out
}

/// `text` with every stdlib module prefix taken off where it starts a name,
/// `(std.math.decimal-arith.done (std.math.decimal-arith.sum a b) c)` read
/// `(done (sum a b) c)`: what a test that compares facts in their short
/// spelling reads (M1f S1f.6.35).
pub fn unprefixed(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut at_start = true;
    while !rest.is_empty() {
        if at_start && let Some(after) = rest.strip_prefix("std.") {
            // `std.<segment>.…<segment>.`, a segment a run of name characters
            // with no dot: a module's canonical name is a dot a directory
            // (`std.math.decimal-arith.`, M1f S1f.6.31), and the name after
            // it holds none.
            let segment = |s: &str| -> Option<usize> {
                let dot = s.find('.')?;
                (dot > 0
                    && s[..dot]
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'))
                .then_some(dot)
            };
            if let Some(dot) = segment(after) {
                let mut tail = &after[dot + 1..];
                while let Some(dot) = segment(tail) {
                    tail = &tail[dot + 1..];
                }
                rest = tail;
                at_start = false;
                continue;
            }
        }
        let c = rest.chars().next().expect("non-empty");
        out.push(c);
        at_start = matches!(c, '(' | ')' | ' ' | '"' | '\n' | '`' | '\'' | ',' | '[');
        rest = &rest[c.len_utf8()..];
    }
    out
}

/// The manifest, loaded from its one location.
pub fn corpus() -> Corpus {
    let path = repo_root().join(manifest::MANIFEST);
    Corpus::load(&path).unwrap_or_else(|e| panic!("{e}"))
}

/// **The Zebra puzzle's published answer** — the Wikipedia grid, as the 25
/// `(house, values)` cells in the order Color, Nationality, Cigarette, Drink,
/// Pet. Vocabulary independent, which is the point: `ein-infer`'s
/// `acceptance.rs` checks both encodings' *solver* against it, and
/// `ein-cli/tests/models_grid.rs` checks `ein solve --models grid`'s
/// *renderer* against it (M1e2 S1e2.1.4). One independent source for both.
pub const ZEBRA_GRID: [(&str, [&str; 5]); 5] = [
    ("House-1", ["Yellow", "Norwegian", "Kools", "Water", "Fox"]),
    (
        "House-2",
        ["Blue", "Ukrainian", "Chesterfields", "Tea", "Horse"],
    ),
    (
        "House-3",
        ["Red", "Englishman", "Old_Gold", "Milk", "Snail"],
    ),
    (
        "House-4",
        ["Ivory", "Spaniard", "Lucky_Strike", "Juice", "Dog"],
    ),
    (
        "House-5",
        ["Green", "Japanese", "Parliament", "Coffee", "Zebra"],
    ),
];

/// The wrong-answer register, loaded from its one location.
pub fn register() -> Register {
    let path = repo_root().join(register::REGISTER);
    Register::load(&path).unwrap_or_else(|e| panic!("{e}"))
}

/// Every `.ein` under `examples/`, `stdlib/` and `tests/`, sorted — the file
/// set [`manifest`] enumerates, discovered rather than listed so a new fixture
/// is covered the moment it lands.
///
/// The two views are kept the same set by
/// `manifest::tests::every_ein_file_has_an_entry`, which is what lets a
/// sweep walk the *files* and the CLI sweep walk the *rows* without either one
/// having to trust the other.
///
/// `tests/` is M1c S1c.1.4's — the stdlib conformance programs, which are a
/// suite rather than a set of examples and so are not under `examples/`.
pub fn corpus_files() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ["examples", "stdlib", "tests"] {
        collect(&root.join(dir), &mut out);
    }
    out.sort();
    out
}

/// Every `.ein` under one directory, sorted — [`corpus_files`] over a
/// directory the caller names.
///
/// Its one caller is `ein-render/tests/id_order_invariance.rs`'s
/// `EIN_ID_FILES` seam, which points that sweep at generated input instead of
/// at the corpus. Public here rather than duplicated there so that "what
/// counts as an input file" has one answer.
pub fn ein_files_under(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out);
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

/// A checked-in golden: compare, or rewrite under `EIN_BLESS=1`.
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

/// Where a golden lives: `<crate>/tests/golden/<name>`.
pub fn golden_path(krate: &str, name: &str) -> PathBuf {
    repo_root()
        .join("ein.rs/crates")
        .join(krate)
        .join("tests/golden")
        .join(name)
}

/// **A scratch directory that removes itself** (M1e2 S1e2.3.5).
///
/// `<prefix>-<pid>-<n>` under [`std::env::temp_dir`], so `$TMPDIR` moves it.
/// The pid keeps two test binaries, or two gate runs, apart; `n`, a counter
/// of this process, keeps apart two tests of one binary, which are threads of
/// one process. It is created empty, and removed with everything in it when
/// the value is dropped, by a test that panics too, since the panic unwinds
/// through the drop. [`Scratch::keep`] leaves it behind on purpose, for a
/// human to read after a failure.
///
/// Before it each test file made its own scratch, and seven removed it on no
/// path at all, nine more only when the test passed: 1 098 `/tmp/ein-*`
/// directories had piled up by 2026-11-03, eleven more per gate run
/// (S1e2.3.5's sweep). `./run_tests.sh` now runs the tests under a fresh
/// `TMPDIR` and fails on anything left in it.
///
/// It dereferences to its [`Path`], so `scratch.join("x.ein")` and
/// `scratch.display()` read as they would on a `PathBuf`.
pub struct Scratch {
    path: PathBuf,
    keep: bool,
}

impl Scratch {
    /// A fresh, empty `<prefix>-<pid>-<n>` under the temp directory.
    ///
    /// A directory of that name left by an earlier process with the same pid
    /// is removed first, so what a test finds in it is what it wrote.
    pub fn new(prefix: &str) -> Scratch {
        static NTH: AtomicUsize = AtomicUsize::new(0);
        let n = NTH.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path)
            .unwrap_or_else(|e| panic!("a scratch directory {}: {e}", path.display()));
        Scratch { path, keep: false }
    }

    /// The directory.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Leave the directory where it is, and return its path: the one way a
    /// test keeps its output for a human. Nothing removes it afterwards, and
    /// the gate's scratch check fails on it if the run is otherwise green.
    pub fn keep(mut self) -> PathBuf {
        self.keep = true;
        self.path.clone()
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if !self.keep {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

impl std::ops::Deref for Scratch {
    type Target = Path;
    fn deref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for Scratch {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::Scratch;

    /// Named `<prefix>-<pid>-<n>`, empty, distinct per call, gone on drop,
    /// and still there after `keep`.
    #[test]
    fn a_scratch_directory_is_fresh_and_removes_itself() {
        let (a, b) = (
            Scratch::new("ein-corpus-scratch"),
            Scratch::new("ein-corpus-scratch"),
        );
        assert_ne!(a.path(), b.path());
        let name = a.file_name().unwrap().to_string_lossy().into_owned();
        let pid = std::process::id().to_string();
        let mut parts = name.rsplitn(3, '-');
        let (n, p, prefix) = (parts.next(), parts.next(), parts.next());
        assert!(n.is_some_and(|n| n.parse::<usize>().is_ok()), "{name}");
        assert_eq!(
            (prefix, p),
            (Some("ein-corpus-scratch"), Some(pid.as_str()))
        );
        assert!(a.is_dir() && std::fs::read_dir(&*a).unwrap().next().is_none());

        std::fs::write(a.join("f"), "x").unwrap();
        let gone = a.to_path_buf();
        drop(a);
        assert!(!gone.exists(), "{} survived its drop", gone.display());

        let kept = b.keep();
        assert!(kept.is_dir(), "keep() removed it");
        std::fs::remove_dir_all(&kept).unwrap();
    }
}
