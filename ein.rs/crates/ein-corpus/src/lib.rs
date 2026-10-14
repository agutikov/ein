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
//! | this module | [`repo_root`], [`corpus_files`], [`golden`], [`golden_path`] — found from the compiling crate, so a test runs the same from anywhere |
//!
//! What is *not* here is a `skip` helper. A skip that prints to stderr —
//! which `cargo test` captures for a passing test — reports a pass while
//! asserting nothing. Nothing in this crate can be skipped: the corpus is
//! checked in, and a fixture that cannot be found is a failure.

pub mod manifest;
pub mod plan;
pub mod register;

use std::path::{Path, PathBuf};

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
