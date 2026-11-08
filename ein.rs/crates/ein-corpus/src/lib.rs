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
pub mod plan;
pub mod register;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use manifest::{Corpus, Entry};
pub use register::{Register, Row};

/// **The corpus files the parser refuses between M1f S1f.5.3 and S1f.5.4.**
///
/// Each writes a range, `1..5` or `1..3`. S1f.5.3 retired the `RANGE` token
/// (Q-M1e2.24: Ein has no numbers and no ranges), so `1` is an `INT` and
/// `..5` no token, and the parse stops at the `.`; S1f.5.4 makes `1..5` one
/// name, and both files load again, each as it is written. Until then a test
/// whose subject is a file that parses, loads or answers leaves these out by
/// name, never by a silent skip, and `ein-cli`'s
/// `corpus_cli::the_files_refused_until_digits_are_names_are_refused` holds
/// the list both ways: every run of each is a parse error, so S1f.5.4 empties
/// it in the commit that makes them load.
pub const REFUSED_UNTIL_S1F_5_4: &[&str] = &[
    "examples/ein-bugs/wrong-answers/range-in-match.ein",
    "examples/syntax/arg-kinds.ein",
];

/// The repo root, found from the compiling crate rather than the working
/// directory, so a test runs the same from anywhere.
pub fn repo_root() -> PathBuf {
    // crates/<crate> → crates → ein.rs → repo
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repo root")
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
