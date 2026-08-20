//! The corpus manifest — `conformance/corpus.toml`.
//!
//! One entry per `.ein` file the harness knows about, with the run matrix it
//! is exercised under. The manifest is the mechanical version of the rule
//! `examples/README.md` states in prose: a file with no entry is a coverage
//! hole, and a completeness check in both test suites fails on one.
//!
//! Format and the group vocabulary: `conformance/README.md`.

use serde::Deserialize;
use std::path::Path;

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
    /// One of the groups in `conformance/README.md`.
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

pub const SCHEMA: &str = "ein-corpus/1";

/// The group vocabulary — `conformance/README.md`.
pub const GROUPS: [&str; 7] = [
    "positive",
    "parse-negative",
    "load-negative",
    "stdlib",
    "golden",
    "generated",
    "crash-parity",
];

/// The repo root, found by walking up from the crate directory. Test-only:
/// at runtime the root is found by walking up from the *working* directory
/// (`main::find_repo`), so a checked-out harness and an installed one behave
/// the same.
#[cfg(test)]
pub fn repo_root() -> std::path::PathBuf {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("conformance/corpus.toml").is_file() {
        assert!(dir.pop(), "no conformance/corpus.toml above the crate");
    }
    dir
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn corpus() -> Corpus {
        Corpus::load(&repo_root().join("conformance/corpus.toml")).expect("corpus")
    }

    /// Every `.ein` the corpus must cover, repo-root-relative.
    fn tracked() -> BTreeSet<String> {
        let repo = repo_root();
        let mut out = BTreeSet::new();
        let mut stack = vec![repo.join("examples")];
        // The stdlib is not a tree; S1a.0.3 moves it to repo-root `stdlib/`.
        for dir in ["stdlib", "ein.py/src/ein/stdlib"] {
            let p = repo.join(dir);
            if p.is_dir() {
                stack.push(p);
                break;
            }
        }
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

    /// The completeness check, Rust side. Its twin is
    /// `ein.py/tests/test_corpus_manifest.py` — both suites, because either
    /// one alone can be the suite nobody ran.
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

    /// T1a.10.1.1 — the four claims the manifest check made **only** on the
    /// Python side, ported so the corpus's contract survives the suite that
    /// held it. `ein.py/tests/test_corpus_manifest.py` had nine tests and this
    /// module had five; these are the other four, and the count is the point:
    /// "the completeness check is duplicated in both suites" was true of the
    /// completeness check and not of the manifest's other invariants.
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

    /// `broken/*.ein` fail at parse; `broken/load/*.ein` fail at load; the
    /// split is what lets P1a.1 gate on one and P1a.2 on the other. A file
    /// that loads and *then* crashes the engine is neither — it is a
    /// well-formed input the engine mishandles, so it lives with the other
    /// bug-repro puzzles under `crash-parity`.
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
                &["crash-parity", "positive"]
            } else if path.starts_with("examples/broken/") {
                &["parse-negative"]
            } else if path.starts_with("examples/") {
                &["positive", "crash-parity"]
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
