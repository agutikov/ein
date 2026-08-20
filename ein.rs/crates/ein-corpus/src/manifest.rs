//! The corpus manifest — `corpus/corpus.toml`.
//!
//! One entry per `.ein` file, with the run matrix it is exercised under. The
//! manifest is the mechanical version of the rule `examples/README.md` states
//! in prose: a file with no entry is a coverage hole, and the completeness
//! check below fails on one.
//!
//! Format and the group vocabulary: `corpus/README.md`.
//!
//! The reader survived `ein-conformance`, which was retired with the second
//! engine at
//! [S1a.10.3](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md).
//! What changed with it is what a `runs` entry *means*: an invocation the file
//! is **exercised** under, not one two implementations are **compared** under.
//! [`crate::plan::argv`] turns one into an argv and
//! `ein-cli/tests/corpus_cli.rs` runs it.

use serde::Deserialize;
use std::path::Path;

/// The manifest's path, repo-root-relative. One location, named once: a
/// `--corpus` override was the harness's, and a corpus that can be pointed
/// elsewhere is a completeness check that can be pointed at an empty file.
pub const MANIFEST: &str = "corpus/corpus.toml";

#[derive(Debug, Deserialize)]
pub struct Corpus {
    pub schema: String,
    #[serde(default)]
    pub entry: Vec<Entry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Repo-root-relative path to the `.ein` file.
    pub path: String,
    /// One of the groups in `corpus/README.md`.
    pub group: String,
    /// Run names — see [`crate::plan::argv`] for how each becomes an argv.
    #[serde(default)]
    pub runs: Vec<String>,
    /// Extra flag strings, each appended to a plain `solve` to make one more
    /// run. The `SolverConfig` lever matrix, restricted to what the CLI can
    /// express today (Q-M1a.16).
    #[serde(default)]
    pub levers: Vec<String>,
    /// Nightly tier only.
    #[serde(default)]
    pub slow: bool,
    /// Free-text; why this entry is interesting, when that is not obvious.
    #[serde(default)]
    pub note: Option<String>,
}

impl Entry {
    /// Every run for this entry: the declared ones, then one `solve <lever>`
    /// per lever.
    pub fn all_runs(&self) -> Vec<String> {
        let mut out = self.runs.clone();
        for lever in &self.levers {
            out.push(format!("solve {lever}"));
        }
        out
    }
}

impl Corpus {
    pub fn load(path: &Path) -> Result<Corpus, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let corpus: Corpus =
            toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if corpus.schema != SCHEMA {
            return Err(format!(
                "{}: schema {:?}, expected {SCHEMA:?}",
                path.display(),
                corpus.schema
            ));
        }
        // A typo'd group would silently drop the entry out of every
        // `--group` selection, which reads as "that file is clean".
        for e in &corpus.entry {
            if !GROUPS.contains(&e.group.as_str()) {
                return Err(format!(
                    "{}: {}: unknown group {:?} (expected one of {GROUPS:?})",
                    path.display(),
                    e.path,
                    e.group
                ));
            }
        }
        Ok(corpus)
    }

    /// Entries matching the selection, in manifest order.
    pub fn select(&self, groups: &[String], filter: Option<&str>, slow: bool) -> Vec<&Entry> {
        self.entry
            .iter()
            .filter(|e| groups.is_empty() || groups.contains(&e.group))
            .filter(|e| filter.is_none_or(|f| e.path.contains(f)))
            .filter(|e| slow || !e.slow)
            .collect()
    }
}

/// Bumped to 2 at S1a.10.3, which renamed `crash-parity` (a claim about
/// ein.py's exception classes) to `regression` and split the compile-error
/// fixtures out as `compile-negative`. The version is what refuses a
/// schema-1 manifest: an unknown group is an error, but a *renamed* one would
/// otherwise reach [`Corpus::load`]'s vocabulary check as a typo, and the
/// reader should say which of the two it is.
pub const SCHEMA: &str = "ein-corpus/2";

/// The group vocabulary — `corpus/README.md`.
pub const GROUPS: [&str; 7] = [
    "positive",
    "stdlib",
    "parse-negative",
    "load-negative",
    "compile-negative",
    "regression",
    "generated",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{corpus, repo_root};
    use std::collections::BTreeSet;

    /// Every `.ein` the corpus must cover, repo-root-relative.
    ///
    /// `stdlib/` is named, not searched for. It was searched for until
    /// S1a.10.3, with `ein.py/src/ein/stdlib` as a fallback — a build-time
    /// copy of the same files (design/11), and a fallback that would have
    /// turned "the stdlib directory is gone" into "the check passes over
    /// seven fewer files". The ledger's §4 lists it as the one relocation the
    /// removal still owed.
    fn tracked() -> BTreeSet<String> {
        let repo = repo_root();
        let mut out = BTreeSet::new();
        let mut stack = vec![repo.join("examples"), repo.join("stdlib")];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read_dir").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "ein") {
                    out.insert(
                        path.strip_prefix(&repo)
                            .expect("prefix")
                            .display()
                            .to_string(),
                    );
                }
            }
        }
        out
    }

    /// The completeness check. It had a Python twin
    /// (`ein.py/tests/test_corpus_manifest.py`) while there were two suites;
    /// the four claims that twin made and this module did not are the four
    /// below, ported at T1a.10.1.1.
    #[test]
    fn every_ein_file_has_an_entry() {
        let corpus = corpus();
        let listed: BTreeSet<String> = corpus.entry.iter().map(|e| e.path.clone()).collect();
        let missing: Vec<String> = tracked()
            .into_iter()
            .filter(|p| !listed.contains(p))
            .collect();
        assert!(missing.is_empty(), "no corpus entry for: {missing:?}");
    }

    #[test]
    fn every_entry_names_a_real_file() {
        let repo = repo_root();
        let corpus = corpus();
        let stale: Vec<&str> = corpus
            .entry
            .iter()
            .filter(|e| !repo.join(&e.path).is_file())
            .map(|e| e.path.as_str())
            .collect();
        assert!(stale.is_empty(), "entries naming missing files: {stale:?}");
    }

    #[test]
    fn groups_are_from_the_vocabulary() {
        let corpus = corpus();
        for e in &corpus.entry {
            assert!(
                GROUPS.contains(&e.group.as_str()),
                "{}: {}",
                e.path,
                e.group
            );
        }
    }

    #[test]
    fn every_entry_has_at_least_one_run() {
        let corpus = corpus();
        for e in &corpus.entry {
            assert!(!e.all_runs().is_empty(), "{}: no runs", e.path);
        }
    }

    /// T1a.10.1.1 — the first of the four claims the manifest check made
    /// **only** on the Python side, ported so the corpus's contract survives
    /// the suite that held it. `test_corpus_manifest.py` had nine tests and
    /// this module had five; these are the other four, and the count is the
    /// point: "the completeness check is duplicated in both suites" was true
    /// of the completeness check and not of the manifest's other invariants.
    #[test]
    fn paths_are_unique() {
        let corpus = corpus();
        let mut seen = BTreeSet::new();
        let dupes: Vec<&str> = corpus
            .entry
            .iter()
            .filter(|e| !seen.insert(e.path.as_str()))
            .map(|e| e.path.as_str())
            .collect();
        assert!(dupes.is_empty(), "duplicate entries: {dupes:?}");
    }

    /// **A group is a directory.** `broken/*.ein` fail at parse,
    /// `broken/load/*.ein` at load, `broken/compile/*.ein` at compile — the
    /// split is what lets P1a.1 gate on one, P1a.2 on the next and P1a.3 on
    /// the third — and `ein-bugs/*.ein` are the bug-repro puzzles, which fail
    /// at no fixed point because some of them no longer fail at all.
    ///
    /// That last group was `crash-parity` until S1a.10.3, and its membership
    /// rule was "ein.py raises an unhandled exception here", which is neither
    /// a directory nor a fact about the language. Two of its ten members
    /// answer in ein.rs ([D2](../../../plans/m1a_rust/divergences.md)), which
    /// is why the group no longer predicts an exit code and the sweep's
    /// golden does.
    #[test]
    fn negatives_are_grouped_by_where_they_fail() {
        for e in &corpus().entry {
            let (path, group) = (e.path.as_str(), e.group.as_str());
            let want: &[&str] = if path.starts_with("examples/broken/load/") {
                &["load-negative"]
            } else if path.starts_with("examples/broken/compile/") {
                // S1a.3.1 — they parse and load, then the compiler refuses.
                // `activator_arity` is the exception: the S1.22.0 arity filter
                // makes its error unreachable through the engine, so its run
                // succeeds and it is an ordinary `positive`.
                &["compile-negative", "positive"]
            } else if path.starts_with("examples/broken/") {
                &["parse-negative"]
            } else if path.starts_with("examples/ein-bugs/") {
                // S1a.10.3 — the whole directory, so the group is a fact about
                // where a file lives rather than about what it does. What it
                // does is mixed: seven answer, two are refused, one does both.
                &["regression"]
            } else if path.starts_with("examples/") {
                &["positive"]
            } else {
                continue;
            };
            assert!(
                want.contains(&group),
                "{path}: {group} (want one of {want:?})"
            );
        }
    }

    /// The other half of S1a.3.1: every `examples/broken/compile/*.ein` is a
    /// corpus entry and has its `.expected` beside it, and nothing else claims
    /// to be one.
    #[test]
    fn every_compile_negative_fixture_has_its_expected() {
        let dir = repo_root().join("examples/broken/compile");
        let stems = |ext: &str| -> BTreeSet<String> {
            std::fs::read_dir(&dir)
                .expect("examples/broken/compile")
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == ext))
                .map(|p| p.file_stem().expect("stem").to_string_lossy().into_owned())
                .collect()
        };
        let eins = stems("ein");
        assert!(!eins.is_empty(), "no fixtures in {}", dir.display());
        assert_eq!(eins, stems("expected"), "a fixture without its .expected");
        let listed: BTreeSet<String> = corpus().entry.into_iter().map(|e| e.path).collect();
        for stem in &eins {
            let path = format!("examples/broken/compile/{stem}.ein");
            assert!(listed.contains(&path), "{path} is not a corpus entry");
        }
    }

    /// Every load-negative fixture is a corpus entry, and every load-negative
    /// entry has its `.expected` beside it — the cross-check against the other
    /// half of S1a.0.1.
    #[test]
    fn the_load_negative_group_matches_the_fixture_directory() {
        let repo = repo_root();
        let dir = repo.join("examples/broken/load");
        let on_disk: BTreeSet<String> = std::fs::read_dir(&dir)
            .expect("examples/broken/load")
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "ein"))
            .map(|p| p.strip_prefix(&repo).expect("prefix").display().to_string())
            .collect();
        let listed: BTreeSet<String> = corpus()
            .entry
            .into_iter()
            .filter(|e| e.group == "load-negative")
            .map(|e| e.path)
            .collect();
        assert_eq!(
            listed, on_disk,
            "the load-negative group is not the directory"
        );
        for path in &listed {
            let expected = repo.join(path).with_extension("expected");
            assert!(expected.is_file(), "{path} has no .expected");
        }
    }

    #[test]
    fn levers_become_extra_solve_runs() {
        let e = Entry {
            path: "x".into(),
            group: "positive".into(),
            runs: vec!["solve".into()],
            levers: vec!["-L".into(), "-o score-sum".into()],
            slow: false,
            note: None,
        };
        assert_eq!(e.all_runs(), ["solve", "solve -L", "solve -o score-sum"]);
    }
}
