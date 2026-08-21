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
//! | this module | [`repo_root`], [`corpus_files`], [`golden`], [`golden_path`] — found from the compiling crate, so a test runs the same from anywhere |
//!
//! # What used to be here
//!
//! This crate is `ein-oracle` with its subject removed. From
//! [P1a.1](../../../plans/m1a_rust/p1a.1_ir_frontend/README.md) to
//! [S1a.10.2](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
//! it held `ein.py` and CPython behind a JSON-Lines protocol, so that the 42
//! differential tests in the workspace did not each re-implement the process
//! plumbing. S1a.10.2 un-differentialled all 42 and the `Oracle` half became
//! dead code the same day;
//! [S1a.10.3](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
//! removed it and folded in the manifest reader, which was the surviving half
//! of `ein-conformance`.
//!
//! What is *not* here is a `skip` helper. The oracle's version printed to
//! stderr — which `cargo test` captures for a passing test — so 41 tests
//! reported a pass while asserting nothing, for two phases
//! ([the ledger §2](../../../plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md#2-the-finding--46--of-einrss-own-integration-tests-are-differential)).
//! Nothing in this crate can be skipped: the corpus is checked in, and a
//! fixture that cannot be found is a failure.

pub mod manifest;
pub mod plan;

use std::path::{Path, PathBuf};

pub use manifest::{Corpus, Entry};

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

/// Every `.ein` under `examples/` and `stdlib/`, sorted — the file set
/// [`manifest`] enumerates, discovered rather than listed so a new fixture is
/// covered the moment it lands.
///
/// The two views are kept the same set by
/// [`manifest::tests::every_ein_file_has_an_entry`], which is what lets a
/// sweep walk the *files* and the CLI sweep walk the *rows* without either one
/// having to trust the other.
pub fn corpus_files() -> Vec<PathBuf> {
    let root = repo_root();
    let mut out = Vec::new();
    for dir in ["examples", "stdlib"] {
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

/// A checked-in **ein.rs** golden: compare, or rewrite under `EIN_BLESS=1`.
///
/// Distinct from the nineteen under `tests/golden/from_ein_py/`, which are the
/// *other* implementation's own output and are read-only — a golden re-blessed
/// from ein.rs proves only that it agrees with itself. (They lived under
/// `ein.py/tests/golden/` until
/// [S1a.10.2](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.2_port_the_suite.md)
/// carried them across by `git mv`, which is the last independent provenance
/// the repo has.) These are the other kind, and since
/// [S1a.6.10](../../../plans/m1a_rust/p1a.6_performance/s1a.6.10_parity_contract.md)
/// they are the whole regression coverage of everything the parity contract
/// stopped comparing: a shipping engine is compared against fixtures.
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
