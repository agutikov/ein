//! The corpus manifest — `corpus/corpus.toml`.
//!
//! One entry per `.ein` file, with the run matrix it is exercised under. The
//! manifest is the mechanical version of the rule `examples/README.md` states
//! in prose: a file with no entry is a coverage hole, and the completeness
//! check below fails on one.
//!
//! Format and the group vocabulary: `corpus/README.md`.
//!
//! A `runs` entry is an invocation the file is **exercised** under:
//! [`crate::plan::argv`] turns one into an argv and
//! `ein-cli/tests/corpus_cli.rs` runs it.

use serde::Deserialize;
use std::path::Path;

/// The manifest's path, repo-root-relative. One location, named once: a
/// corpus that can be pointed elsewhere is a completeness check that can be
/// pointed at an empty file.
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
    /// express today.
    #[serde(default)]
    pub levers: Vec<String>,
    /// Left out of the default sweep — see [`SLOW_MS`], which is the
    /// threshold it is a claim about, and [`Entry::cost_ms`], which is the
    /// measurement.
    #[serde(default)]
    pub slow: bool,
    /// **What this entry's declared runs cost, together, in milliseconds** —
    /// the evidence behind [`Entry::slow`], measured rather than guessed.
    ///
    /// The sum rather than the slowest run, because the flag's job is the
    /// *sweep's* budget and the sum is what an entry costs it. The two rules
    /// part on an entry of several runs each under the line and together over
    /// it, and `corpus/README.md` § `slow` is where the choice is written
    /// down.
    ///
    /// Recorded for the tail and omitted where it would be noise: an entry
    /// with no `cost_ms` is one the default sweep runs in milliseconds, and
    /// the sweep itself is what holds that claim up
    /// (`corpus_cli::the_slow_flag_still_describes_the_sweep`). The figure is
    /// a measurement taken outside the tests and banked here.
    #[serde(default)]
    pub cost_ms: Option<u64>,
    /// **This entry's describe dump writes at least [`HEAVY_DUMP_FILES`]
    /// state files.** `ein-cli/tests/answer_round_trip.rs` runs every corpus
    /// program's `ein solve FILE --dump-answer DIR` and reloads each state
    /// file it writes, so a dump of tens of thousands of files is most of
    /// that test's time; by default it leaves a flagged entry out, and under
    /// `EIN_CORPUS_SLOW=1` it takes it.
    ///
    /// Not [`Entry::slow`], which is a claim about the entry's *declared*
    /// runs: the round trip's dump is none of them, and the flagged entries
    /// that are not `slow` declare runs that cost milliseconds. It is a
    /// count, not a time, so it holds on any machine, and the round trip
    /// holds it in both directions wherever it dumps an entry: an unflagged
    /// entry that writes [`HEAVY_DUMP_FILES`] or more fails, and so does a
    /// flagged one that writes fewer. A `slow` entry is held to it too, under
    /// `EIN_CORPUS_SLOW=1`, so the flag says the same thing of every entry.
    /// `the_heavy_dump_set_is_exactly_these_five` pins the set.
    #[serde(default)]
    pub heavy_dump: bool,
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

/// Version 2 names one group `regression` that version 1 named otherwise, and
/// holds the compile-error fixtures as a group of their own,
/// `compile-negative`. The version is what refuses a
/// schema-1 manifest: an unknown group is an error, but a *renamed* one would
/// otherwise reach [`Corpus::load`]'s vocabulary check as a typo, and the
/// reader should say which of the two it is.
pub const SCHEMA: &str = "ein-corpus/2";

/// **The `slow` threshold**, in milliseconds of an entry's declared runs
/// summed.
///
/// A flag with a number behind it can rot; a flag with no number cannot even
/// be checked. [`Entry::cost_ms`] is the number, and `slow_matches_the_recorded_cost`
/// holds the flag to it against this line.
///
/// One second is where a single entry stops being part of a default sweep and
/// starts being the reason it is slow.
/// [03-design/02-shared-assets.md § 5](../../../../docs/ein/ein.rs/03-design/02-shared-assets.md#5-the-corpus)
/// states the field and `corpus/README.md` § `slow` the rule.
pub const SLOW_MS: u64 = 1000;

/// **The `heavy_dump` threshold**, in state files an entry's describe dump
/// writes: what `answer_round_trip`'s `check` reloads, and
/// what `EIN_ROUND_TRIP_LIST=1` prints beside each program.
///
/// Set in the gap of the distribution
/// (`EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release
/// --test answer_round_trip -- --nocapture`): 296 of the 417 entries write a
/// state file, five write 64 514 to 65 214, the next six 23 390 each (the
/// `slow` `square-fwd` and `square-bwd`), then 11 339
/// (`type-exclusivity/pets`) and 4 096 or fewer. Forty thousand is 1.6×
/// under the five and 1.7× over the six, and it flags the five
/// `the_heavy_dump_set_is_exactly_these_five` names, one of them `slow`. A count
/// depends on the engine and the corpus, never on the machine, so the line
/// moves only when a dump does, and the round trip says which.
pub const HEAVY_DUMP_FILES: usize = 40_000;

/// The group vocabulary — `corpus/README.md`.
///
/// A corpus entry is a file the engine is *permanently* exercised over, so a
/// batch of generated programs, which lives for milliseconds, has no group.
pub const GROUPS: [&str; 6] = [
    "positive",
    "stdlib",
    "parse-negative",
    "load-negative",
    "compile-negative",
    "regression",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{corpus, repo_root};
    use std::collections::BTreeSet;

    /// Every `.ein` the corpus must cover, repo-root-relative.
    ///
    /// `stdlib/` is named, not searched for: a search with a fallback would
    /// turn "the stdlib directory is gone" into "the check passes over fewer
    /// files".
    ///
    /// `tests/` is the third root: the stdlib conformance programs, under
    /// [`tests/README.md`](../../../../tests/README.md), are a suite rather than a set of
    /// examples, and they are walked here for the same reason the other two
    /// roots are — a test file with no corpus entry is a file the sweep never
    /// runs, which reads as coverage nobody has.
    fn tracked() -> BTreeSet<String> {
        let repo = repo_root();
        let mut out = BTreeSet::new();
        let mut stack = vec![
            repo.join("examples"),
            repo.join("stdlib"),
            repo.join("tests"),
        ];
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

    /// The completeness check. The claims below it are the manifest's other
    /// invariants.
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

    /// **No declared run may carry a wall-clock budget**: it is the one place
    /// a stopwatch would reach a **golden** rather than a test.
    ///
    /// [`corpus/README.md`](../../../../corpus/README.md) blesses `-T` /
    /// `--max-time` in the run vocabulary as the equal of `-E`, and
    /// `no_cell_crashes` allows an exit 2 wherever a run names one. But `-E`
    /// counts enterings and `-T` counts seconds: the same argv exits **2** at
    /// `-T 0.001` and **0** at `-T 60`, and
    /// `ein-cli/tests/golden/corpus_exits.txt` is compared line by line. So a
    /// `-T` cell would put a stopwatch inside a banked artefact — which is
    /// the one place in this repo the clock is otherwise always scrubbed
    /// before a comparison (`golden_dump.rs`, `shape.rs`).
    #[test]
    fn no_declared_run_budgets_by_wall_clock() {
        let bad: Vec<String> = corpus()
            .entry
            .iter()
            .flat_map(|e| {
                e.runs
                    .iter()
                    .chain(e.levers.iter())
                    .filter(|r| r.split_whitespace().any(|w| w == "-T" || w == "--max-time"))
                    .map(move |r| format!("  {}: `{r}`", e.path))
            })
            .collect();
        assert!(
            bad.is_empty(),
            "a declared run budgets by wall clock, so its exit code is a \
             stopwatch and `corpus_exits.txt` banks it. Use `--region-budget \
             FACTS`, which counts facts and is deterministic:\n{}",
            bad.join("\n")
        );
    }

    /// **The `slow` set, by name.** Two things it catches that nothing else
    /// does. An entry *gaining* the flag — which silently removes it from the
    /// default sweep, so every assertion about it keeps passing on a cell
    /// that no longer runs — and an entry *losing* it, which is the
    /// measurement working and is worth a line rather than a silent diff.
    /// Whether a recorded `slow = true` is still true wants a stopwatch, and
    /// is `corpus_cli::the_slow_flag_still_describes_the_sweep`'s under
    /// `EIN_CORPUS_SLOW=1`; what a default run can assert for nothing is the
    /// **set**.
    ///
    /// It is `assert_eq!` on the names rather than a floor on the count,
    /// because a floor over a growing corpus decays.
    #[test]
    fn the_slow_set_is_exactly_these_sixteen() {
        let manifest = corpus();
        let slow: Vec<&str> = manifest
            .entry
            .iter()
            .filter(|e| e.slow)
            .map(|e| e.path.as_str())
            .collect();
        assert_eq!(
            slow,
            // The entries whose declared runs cost a second or more
            // together, as each one's `cost_ms` records, in manifest order.
            [
                "examples/features/01_not_and_absent.ein",
                "examples/beyond_arith/send_more_money.ein",
                "examples/beyond_arith/send_more_money_written.ein",
                "examples/lattice/06_friends_tables.ein",
                "examples/lattice/07_two_pairs_two_tables.ein",
                "examples/lattice/14_seats_nothing_proposes.ein",
                "examples/saturation/square-bwd/floors.ein",
                "examples/saturation/square-bwd/houses.ein",
                "examples/saturation/square-bwd/meetings.ein",
                "examples/saturation/square-fwd/floors.ein",
                "examples/saturation/square-fwd/houses.ein",
                "examples/saturation/square-fwd/meetings.ein",
                "examples/zebra2-minus-15.ein",
                "examples/zebra2-minus-15-obligations.ein",
                "examples/zebra2-obligations.ein",
                "examples/zebra2.ein",
            ],
            "the `slow` set moved. An entry that gained the flag has left the \
             default sweep — check that its recorded cost is real before \
             banking it here; one that lost it is back in the default sweep"
        );
    }

    /// **`slow` is a measured claim, and this is what holds it to the
    /// threshold**: [`SLOW_MS`] against a recorded [`Entry::cost_ms`], in
    /// both directions — no engine run, no wall clock, no flake.
    ///
    /// The half this cannot see is whether `cost_ms` is still *true*, and
    /// that half is `corpus_cli::the_slow_flag_still_describes_the_sweep`,
    /// which compares it against the sweep it has just run. Two checks, one
    /// claim: this one is exact and always runs, that one measures and needs
    /// a tolerance, and asserts its drift only under `EIN_CORPUS_SLOW`,
    /// because a tolerance tighter than the machine fails on the machine.
    #[test]
    fn slow_matches_the_recorded_cost() {
        let mut bad: Vec<String> = Vec::new();
        for e in &corpus().entry {
            match (e.slow, e.cost_ms) {
                (true, None) => bad.push(format!(
                    "{}: slow = true with no cost_ms — the flag has no measurement behind it",
                    e.path
                )),
                (true, Some(ms)) if ms < SLOW_MS => bad.push(format!(
                    "{}: slow = true, but cost_ms = {ms} is under the {SLOW_MS} ms threshold",
                    e.path
                )),
                (false, Some(ms)) if ms >= SLOW_MS => bad.push(format!(
                    "{}: cost_ms = {ms} is at or over the {SLOW_MS} ms threshold, but slow is unset",
                    e.path
                )),
                _ => {}
            }
            if e.cost_ms == Some(0) {
                bad.push(format!("{}: cost_ms = 0 — no run costs nothing", e.path));
            }
        }
        assert!(
            bad.is_empty(),
            "the `slow` flag and the recorded cost disagree \
             (re-measure the entry's runs and record its cost_ms):\n  {}",
            bad.join("\n  ")
        );
    }

    /// **The `heavy_dump` set, by name**, as
    /// [`the_slow_set_is_exactly_these_sixteen`] pins `slow`'s. Whether the
    /// flag is *true* is `answer_round_trip`'s to say, which dumps the
    /// unflagged entries by default and the flagged ones only under
    /// `EIN_CORPUS_SLOW=1`; so an entry that gained the flag would leave the
    /// default round trip with nothing in a default run to notice. This does.
    #[test]
    fn the_heavy_dump_set_is_exactly_these_five() {
        let manifest = corpus();
        let heavy: Vec<&str> = manifest
            .entry
            .iter()
            .filter(|e| e.heavy_dump)
            .map(|e| e.path.as_str())
            .collect();
        assert_eq!(
            heavy,
            // The entries whose describe dump writes at least
            // `HEAVY_DUMP_FILES` state files, the distribution in that
            // constant's doc; `features/01_not_and_absent` is `slow` too.
            [
                "examples/features/01_not_and_absent.ein",
                "examples/features/04_open.ein",
                "examples/saturation/square-unique/corner-house.ein",
                "examples/saturation/square-unique/cul-de-sac.ein",
                "examples/saturation/square-unique/terminus.ein",
            ],
            "the `heavy_dump` set moved. An entry that gained the flag has left \
             the per-commit round trip: check its dump writes at least \
             HEAVY_DUMP_FILES state files (`EIN_ROUND_TRIP_LIST=1 \
             EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release --test \
             answer_round_trip -- --nocapture`) before banking it here"
        );
    }

    #[test]
    fn every_entry_has_at_least_one_run() {
        let corpus = corpus();
        for e in &corpus.entry {
            assert!(!e.all_runs().is_empty(), "{}: no runs", e.path);
        }
    }

    /// One of the manifest's other invariants: "every file has an entry" says
    /// nothing about whether a file has two.
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
    /// `broken/load/*.ein` at load, `broken/compile/*.ein` at compile — so a
    /// group names the stage that refuses its files — and `ein-bugs/*.ein`
    /// are the bug-repro puzzles, which fail at no fixed point: some of them
    /// do not fail at all.
    ///
    /// That last group is defined by where a file lives, not by what it does:
    /// some of its members answer and some are refused, which is why the
    /// group does not predict an exit code and the sweep's golden does.
    #[test]
    fn negatives_are_grouped_by_where_they_fail() {
        for e in &corpus().entry {
            let (path, group) = (e.path.as_str(), e.group.as_str());
            let want: &[&str] = if path.starts_with("examples/broken/load/") {
                &["load-negative"]
            } else if path.starts_with("examples/broken/compile/") {
                // They parse and load, then the compiler refuses.
                // `activator_arity` is the exception: the activators' arity filter
                // makes its error unreachable through the engine, so its run
                // succeeds and it is an ordinary `positive`.
                &["compile-negative", "positive"]
            } else if path.starts_with("examples/broken/") {
                &["parse-negative"]
            } else if path.starts_with("examples/ein-bugs/") {
                // The whole directory, so the group is a fact about where a
                // file lives rather than about what it does. What it does is
                // mixed: some answer, some are refused.
                &["regression"]
            } else if path.starts_with("examples/") {
                &["positive"]
            } else if path.starts_with("tests/stdlib/") {
                // The stdlib conformance programs. They are in
                // the `stdlib` group with the modules they exercise, because
                // the group names a *subject* here rather than a failure
                // point: what a `tests/stdlib/` entry is about is a rule of
                // `stdlib/`, and nothing else in the corpus is.
                &["stdlib"]
            } else {
                continue;
            };
            assert!(
                want.contains(&group),
                "{path}: {group} (want one of {want:?})"
            );
        }
    }

    /// The compile-negative fixtures: every `examples/broken/compile/*.ein` is a
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
    /// entry has its `.expected` beside it — the group checked against the
    /// fixture directory.
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

    /// Does `text` state a claim: `:expect` outside every comment and string?
    fn states_a_claim(text: &str) -> bool {
        let (mut in_string, mut in_comment, mut escaped) = (false, false, false);
        let mut code = String::new();
        for c in text.chars() {
            if in_comment {
                in_comment = c != '\n';
            } else if in_string {
                in_string = escaped || c != '"';
                escaped = !escaped && c == '\\';
            } else if c == ';' {
                in_comment = true;
            } else if c == '"' {
                in_string = true;
            } else {
                code.push(c);
                continue;
            }
            code.push(' ');
        }
        code.contains(":expect")
    }

    /// Whether the static check flags the program at `rel`: root's fixpoint,
    /// the universe and the check, as `ein solve FILE --monotone` asks.
    /// `None` where the program does not load or root does not saturate.
    fn flagged(rel: &str) -> Option<bool> {
        let path = repo_root().join(rel);
        let mut ast = ein_ir::Ast::new();
        let mut terms = ein_core::Terms::new();
        let mut kb = ein_ir::load_file(&mut ast, &mut terms, &path).ok()?;
        let checked = ein_infer::explore::check_monotone(
            &mut kb,
            &mut terms,
            &ast,
            &mut ein_infer::events::Events::off(),
        )
        .ok()?;
        Some(!checked.monotonicity.is_monotone())
    }

    /// **No suite checks a claim of a program that is not monotone**
    /// (Q-M1f.564). Of a program the static check flags nothing is claimed,
    /// so the corpus holds none to a claim it states: no flagged program is
    /// under `tests/`, whose programs are each a claim `ein test tests/`
    /// checks; no flagged entry declares the `test` run; and a flagged entry
    /// whose program states an `:expect` declares only runs that check no
    /// claim: `saturate`, a `render` view, `solve --count` and
    /// `solve --monotone`. The claim stays in the file, where `ein test FILE`
    /// checks it for whoever asks. Computed from each program, never a list.
    #[test]
    fn a_flagged_program_declares_no_run_that_checks_a_claim() {
        let checks_no_claim = |run: &str| {
            run.starts_with("saturate")
                || run.starts_with("render ")
                || run == "solve --count"
                || run == "solve --monotone"
        };
        let (mut bad, mut seen, mut claiming) = (Vec::new(), 0usize, 0usize);
        for e in &corpus().entry {
            if flagged(&e.path) != Some(true) {
                continue;
            }
            seen += 1;
            if e.path.starts_with("tests/") {
                bad.push(format!(
                    "{}: flagged, and under tests/, whose programs `ein test tests/` checks",
                    e.path
                ));
            }
            let runs = e.all_runs();
            if runs.iter().any(|r| r == "test") {
                bad.push(format!("{}: flagged, and declares the `test` run", e.path));
            }
            let text = std::fs::read_to_string(repo_root().join(&e.path)).expect("an entry's file");
            if !states_a_claim(&text) {
                continue;
            }
            claiming += 1;
            for run in runs.iter().filter(|r| !checks_no_claim(r)) {
                bad.push(format!(
                    "{}: flagged and states an `:expect`, and its run `{run}` checks a claim",
                    e.path
                ));
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
        // Floors under what the sweep finds: one that read no flagged program,
        // or none that states a claim, would pass the loop above.
        assert!(
            seen >= 40 && claiming >= 3,
            "{seen} flagged, {claiming} of them stating a claim"
        );
    }

    #[test]
    fn levers_become_extra_solve_runs() {
        let e = Entry {
            path: "x".into(),
            group: "positive".into(),
            runs: vec!["solve".into()],
            levers: vec!["-L".into(), "-o score-sum".into()],
            slow: false,
            cost_ms: None,
            heavy_dump: false,
            note: None,
        };
        assert_eq!(e.all_runs(), ["solve", "solve -L", "solve -o score-sum"]);
    }
}
