//! T1a.10.3.2 — **the corpus, through the CLI.**
//!
//! `ein-conformance run --impl-a … --impl-b … --tier T3` took every entry of
//! `corpus/corpus.toml`, ran it under every declared run as a *process* on
//! both implementations, and diffed everything the two processes produced.
//! [S1a.10.3](../../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md)
//! retires the second operand. What replaces the harness is not a diff, it is
//! a **sweep**: the same 660 cells, one engine, run to see that they run.
//!
//! ## Why a sweep is not nothing
//!
//! Almost everything else in this workspace tests the engine as a *library*.
//! `corpus_shapes.md5` digests 4 228 renderings without starting a process;
//! `summary_properties.rs` solves 176 cells in-process. They are a superset of
//! this file on *content* and they cannot see any of what it sees:
//!
//! - that the manifest's `runs` column still names invocations the CLI
//!   accepts — a renamed flag or a dropped subcommand is a usage error here
//!   and invisible everywhere else;
//! - the **exit code**, which was T0's and which nothing else asserts over the
//!   whole corpus;
//! - that a refused input says *why* on stderr rather than dying quietly;
//! - that `--trace`, `--dump-states` and `--json-summary` write the files they
//!   promise, at the paths the caller chose.
//!
//! ## What is banked, and what is a rule
//!
//! The exit codes are a **golden** (`tests/golden/corpus_exits.txt`, one line
//! per cell) because they are not predictable from the group: `render rules`
//! never loads the KB, so ten of the thirty load-negatives render their rules
//! and exit 0; seventeen `positive` entries have no rule forms and exit 1 on
//! that same run; and `regression` holds ten fixtures of which seven answer.
//! A group-shaped rule would have needed four carve-outs and would have been a
//! worse description of the truth than the table.
//!
//! Everything else here is a **rule**, because a rule does not rot:
//! [`no_cell_crashes`] and the three structural claims below hold for any
//! correct engine and do not have to be looked up.
//!
//! ```text
//! cargo test -p ein-cli --test corpus_cli                    # 542 cells, ~3 s
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test corpus_cli  # 660 cells, ~4 min
//! EIN_BLESS=1       cargo test -p ein-cli --test corpus_cli  # re-bank (implies SLOW)
//! ```
//!
//! ## The timeout is not decoration
//!
//! Every cell runs under `EIN_CORPUS_TIMEOUT` seconds (default 300, which is
//! what `ein-conformance --timeout` defaulted to) and a cell that outlives it
//! is killed and recorded as `-2`. Without it, the failure mode of a change
//! that makes some corpus program stop terminating is not a red gate but a
//! **hung** one, with no output and no name — and this file is the only test
//! in the workspace that runs unbounded search from a process it did not
//! write the arguments for. The slowest cell today is
//! `square-unique/cul-de-sac.ein :: render lattice` at 83 s, which is a blind
//! enumerator over a domain the demo never bounds; it is why that entry is
//! `slow`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use ein_corpus::{corpus, golden, golden_path, plan, repo_root};

/// One cell's result.
struct Cell {
    path: String,
    run: String,
    /// The process's exit code, or `-1` for death by signal and `-2` for a
    /// cell the timeout killed. Neither collides with an exit code.
    code: i32,
    stderr: String,
    out: PathBuf,
}

impl Cell {
    /// `0  examples/zebra2.ein :: solve -e` — the golden's line format.
    ///
    /// Code first, so a column of codes reads down the left edge and a cell
    /// that changed is visible without reading the name.
    fn line(&self) -> String {
        format!("{:<4} {} :: {}", self.code, self.path, self.run)
    }
}

/// `EIN_CORPUS_SLOW=1` — the 17 entries whose runs took 3 s or more under
/// CPython at T1a.0.1.1. They cost ~2 minutes of the sweep's ~3 seconds, so
/// they are nightly's.
///
/// `EIN_BLESS=1` implies it: a golden blessed from the default selection
/// would silently *shrink* by 118 lines, which is the one way a table like
/// this can be wrong without anyone noticing.
fn include_slow() -> bool {
    let on = |k: &str| std::env::var(k).as_deref() == Ok("1");
    on("EIN_CORPUS_SLOW") || on("EIN_BLESS")
}

/// `EIN_CORPUS_TIMEOUT`, in seconds.
fn timeout() -> Duration {
    Duration::from_secs(
        std::env::var("EIN_CORPUS_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300),
    )
}

/// Run one cell, bounded.
///
/// stdout and stderr go to **files** in the cell's own directory rather than
/// to pipes: a bounded run has to poll for completion, and a child that is
/// filling a 64 KB pipe nobody is draining would deadlock instead of finishing
/// — `render lattice` on a large puzzle writes far more than that.
fn run_cell(bin: &str, argv: &[String], repo: &Path, dir: &Path) -> (i32, String) {
    let file = |name: &str| {
        std::fs::File::create(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
    };
    let mut child = Command::new(bin)
        .args(argv)
        .current_dir(repo)
        .stdin(Stdio::null())
        .stdout(Stdio::from(file("stdout")))
        .stderr(Stdio::from(file("stderr")))
        .spawn()
        .unwrap_or_else(|e| panic!("{bin} {argv:?}: {e}"));
    let deadline = Instant::now() + timeout();
    let mut polls = 0u32;
    let code = loop {
        match child.try_wait().expect("try_wait") {
            Some(s) => break s.code().unwrap_or(-1),
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                break -2;
            }
            // Tight at first — most cells are done in under 20 ms and a fixed
            // poll interval would be most of the sweep's wall clock — then
            // coarse, because a cell still alive after a second is not about
            // to finish in the next millisecond.
            None => {
                polls += 1;
                std::thread::sleep(if polls < 200 {
                    Duration::from_millis(1)
                } else {
                    Duration::from_millis(25)
                });
            }
        }
    };
    let stderr = std::fs::read_to_string(dir.join("stderr")).unwrap_or_default();
    (code, stderr)
}

/// Run every declared cell, once.
///
/// Cells are sorted by `(path, run)` rather than left in manifest order, so
/// re-ordering the manifest — which is grouped for readers, not for machines —
/// does not churn the golden.
fn sweep() -> Vec<Cell> {
    let repo = repo_root();
    let bin = env!("CARGO_BIN_EXE_ein");
    let root = std::env::temp_dir().join(format!("ein-corpus-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);

    let mut cells: Vec<(String, String)> = Vec::new();
    for e in &corpus().entry {
        if e.slow && !include_slow() {
            continue;
        }
        for run in e.all_runs() {
            cells.push((e.path.clone(), run));
        }
    }
    cells.sort();

    let mut out = Vec::with_capacity(cells.len());
    for (i, (path, run)) in cells.into_iter().enumerate() {
        let dir = root.join(format!("{i:04}"));
        std::fs::create_dir_all(&dir).expect("cell output directory");
        let argv = plan::argv(&run, &path, &dir);
        let (code, stderr) = run_cell(bin, &argv, &repo, &dir);
        out.push(Cell {
            path,
            run,
            code,
            stderr,
            out: dir,
        });
    }
    out
}

/// The sweep, run once for the whole file. `OnceLock` rather than one sweep
/// per test: five tests × 542 processes would be five times the cost for the
/// same 542 answers.
fn cells() -> &'static [Cell] {
    static CELLS: std::sync::OnceLock<Vec<Cell>> = std::sync::OnceLock::new();
    CELLS.get_or_init(sweep)
}

#[test]
fn every_cell_reproduces_its_exit_code() {
    let cells = cells();
    let got: Vec<String> = cells.iter().map(Cell::line).collect();
    let text = format!("{}\n", got.join("\n"));
    let path = golden_path("ein-cli", "corpus_exits.txt");

    // The default selection is a *subset* of the banked table, so it is
    // compared line by line against the table's own lines rather than as a
    // whole file — otherwise every default run would report 118 missing cells.
    if include_slow() || std::env::var("EIN_BLESS").as_deref() == Ok("1") {
        if let Some(msg) = golden(&path, &text) {
            panic!("{msg}\n{}", moved(&got, &path));
        }
        return;
    }
    let want = std::fs::read_to_string(&path).expect("corpus_exits.txt");
    let banked: std::collections::BTreeSet<&str> = want.lines().collect();
    let missing: Vec<&String> = got
        .iter()
        .filter(|l| !banked.contains(l.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "{} of {} cells differ from the banked table \
         (EIN_BLESS=1 cargo test -p ein-cli --test corpus_cli to re-bank):\n{}",
        missing.len(),
        got.len(),
        moved(&got, &path)
    );
}

/// The differing lines, paired with what was banked for the same cell, so a
/// failure reads as "this cell's exit code moved" rather than as a line
/// number in a 660-line file.
fn moved(got: &[String], path: &Path) -> String {
    let Ok(want) = std::fs::read_to_string(path) else {
        return String::new();
    };
    let key = |l: &str| {
        l.split_once(' ')
            .map(|(a, b)| (b.trim().to_string(), a.to_string()))
    };
    let banked: BTreeMap<String, String> = want.lines().filter_map(key).collect();
    let mut out = Vec::new();
    for line in got {
        let Some((name, code)) = key(line) else {
            continue;
        };
        match banked.get(&name) {
            None => out.push(format!("  + {name} -> {code} (new cell)")),
            Some(w) if *w != code => out.push(format!("  ~ {name}: was {w}, now {code}")),
            _ => {}
        }
    }
    out.iter().take(25).cloned().collect::<Vec<_>>().join("\n")
}

/// **Nothing in the corpus crashes the binary.** The rule the golden cannot
/// state, because a crash would be banked as an exit code like any other.
///
/// The three codes `ein` is allowed to produce are 0 (it answered), 1 (it
/// refused, with a diagnostic) and 2 (a usage error). A usage error over a
/// manifest run means the `runs` column names an invocation the CLI no longer
/// accepts, which is a corpus defect rather than an engine one — and exactly
/// what nothing else in the workspace can see, since every other test builds
/// its own argv. Anything else is a panic (101) or a signal (-1).
#[test]
fn no_cell_crashes() {
    let bad: Vec<String> = cells()
        .iter()
        .filter(|c| !(0..=1).contains(&c.code))
        .map(|c| {
            format!(
                "  {} :: {} -> {}\n{}",
                c.path,
                c.run,
                match c.code {
                    -2 => format!("killed after {}s (EIN_CORPUS_TIMEOUT)", timeout().as_secs()),
                    -1 => "killed by a signal".to_string(),
                    2 => "usage error — the manifest names an argv the CLI refuses".to_string(),
                    n => format!("exit {n}"),
                },
                c.stderr.lines().take(4).collect::<Vec<_>>().join("\n"),
            )
        })
        .collect();
    assert!(bad.is_empty(), "{} cells:\n{}", bad.len(), bad.join("\n"));
}

/// **Every entry that is supposed to work, works** — the liveness check's
/// successor.
///
/// The harness asked "did either implementation ever exit 0?", because two
/// engines that both fail to start agree on every cell
/// ([found at S1a.1.3](../../../../plans/m1a_rust/p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md):
/// 438 cells, 0 DIFF, both sides `ModuleNotFoundError`). With one engine that
/// question is sharper and per-entry rather than per-run: every `positive` and
/// `stdlib` entry must answer under at least one of its runs. Not *all* of
/// them — seventeen have no rule forms, so `render rules` exits 1 on a file
/// that is otherwise fine.
#[test]
fn every_positive_entry_answers_under_at_least_one_run() {
    let manifest = corpus();
    let group: BTreeMap<&str, &str> = manifest
        .entry
        .iter()
        .map(|e| (e.path.as_str(), e.group.as_str()))
        .collect();
    let mut answered: BTreeMap<&str, bool> = BTreeMap::new();
    for c in cells() {
        let Some(g) = group.get(c.path.as_str()) else {
            continue;
        };
        if matches!(*g, "positive" | "stdlib") {
            *answered.entry(&c.path).or_insert(false) |= c.code == 0;
        }
    }
    let dead: Vec<&str> = answered
        .iter()
        .filter(|(_, ok)| !**ok)
        .map(|(p, _)| *p)
        .collect();
    assert!(
        dead.is_empty(),
        "entries that never exited 0 under any run: {dead:?}"
    );
    // 65 in the default selection, 82 with the slow entries: 75 `positive`
    // plus 7 `stdlib`, of which 17 are `slow`. A floor rather than either
    // number, because a corpus that grows must not have to edit a test.
    assert!(
        answered.len() >= 60,
        "only {} positive/stdlib entries were swept — the selection stopped looking",
        answered.len()
    );
}

/// **A refusal says why.** An exit 1 with an empty stderr is a run nobody can
/// diagnose, and it is the one failure shape a corpus sweep would otherwise
/// bank happily: the code is stable, so the golden is green.
#[test]
fn every_refusal_carries_a_diagnostic() {
    let mute: Vec<String> = cells()
        .iter()
        .filter(|c| c.code != 0 && c.stderr.trim().is_empty())
        .map(|c| format!("  {} :: {}", c.path, c.run))
        .collect();
    assert!(
        mute.is_empty(),
        "{} cells exited non-zero with nothing on stderr:\n{}",
        mute.len(),
        mute.join("\n")
    );
}

/// **The artefact flags write their artefacts, where the caller asked.**
///
/// `{out}` in a run name is the manifest's way of naming a file the run will
/// produce (`plan::argv`), and `solve` silently gains a `--json-summary`. All
/// three are additive surfaces whose whole contract is "a file appears and the
/// run is otherwise unchanged", so a sweep that never looked at the files
/// would be checking the weaker half of it.
///
/// The summary is *parsed*, not merely stat-ed: a truncated or malformed JSON
/// object is exactly what an additive writer gets wrong under a run that dies
/// part way through.
#[test]
fn every_artefact_run_leaves_its_artefact() {
    let (mut summaries, mut traces, mut dumps) = (0usize, 0usize, 0usize);
    let mut bad: Vec<String> = Vec::new();
    for c in cells() {
        if c.code != 0 {
            continue; // a refused run promises nothing.
        }
        if c.run.starts_with("solve") {
            let s = c.out.join("summary.json");
            match std::fs::read_to_string(&s) {
                Err(e) => bad.push(format!("  {} :: {} — no summary.json ({e})", c.path, c.run)),
                Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                    Err(e) => bad.push(format!("  {} :: {} — summary.json: {e}", c.path, c.run)),
                    Ok(v) => {
                        summaries += 1;
                        if v.get("verdict").is_none() {
                            bad.push(format!(
                                "  {} :: {} — summary has no verdict",
                                c.path, c.run
                            ));
                        }
                    }
                },
            }
        }
        if c.run.contains("--trace") {
            traces += 1;
            let t = c.out.join("trace.md");
            if !t.is_file() {
                bad.push(format!("  {} :: {} — no trace.md", c.path, c.run));
            }
        }
        if c.run.contains("--dump-states") {
            dumps += 1;
            let d = c.out.join("states");
            if !d.is_dir() {
                bad.push(format!("  {} :: {} — no states/ tree", c.path, c.run));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // Floors: a sweep that stopped producing artefacts would otherwise pass
    // this test by having nothing to check.
    assert!(summaries >= 100, "only {summaries} summaries were written");
    assert!(traces >= 5 && dumps >= 5, "{traces} traces, {dumps} dumps");
}
