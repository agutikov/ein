//! **C4 under every lever** — M1e2
//! [S1e2.1a.6](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)
//! T1e2.1a.6.2, the config principle as a test.
//!
//! [`cli.md` § 1](../../../../docs/ein/user/reference/cli.md#1-the-contract)'s
//! C4: *no lever changes what is printed, cost counters aside — that includes
//! which states `-n` prints*. The user's ruling of 2026-10-15 says the same of
//! every configuration knob: *config may only affect search speed*. What a
//! mode answers is held to the oracle at default levers by
//! `ein-infer/tests/explore.rs`; this file holds each mode's answer under a
//! lever to its answer at the default, which closes the square.
//!
//! Two tests, because a lever reaches a run by two routes:
//!
//! - **[`every_declared_lever_cell_moves_no_line_of_any_mode`]**, through the
//!   binary. Every `levers` cell `corpus/corpus.toml` declares, and every CLI
//!   lever a `corpus/wrong_answers.toml` row's `settings` names, run under each
//!   of `ein solve`'s sixteen questions (three parts of R × describe, `-n 1`,
//!   `-n 2`, `-n k_s+1`, `--single`, and `--count` over all of R) and
//!   compared with the same question at
//!   the default: the exit code, stdout, stderr and the `--json-summary`. The
//!   summary's `config` object echoes the lever, so it is left out of the
//!   comparison and checked to carry it: a lever that stopped reaching the run
//!   cannot pass as one that changes nothing. No environment lever is left:
//!   `EIN_TRAVERSAL=tree` went with the walk at M1e2 S1e2.1c.5, and
//!   `EIN_OBLIGATION_CHOICE=off|fail-first`, swept here until then, was retired
//!   at S1e2.3.1 once the summary's `root` block, its last reader, stopped
//!   running the walk's generator. The one CLI lever left, `-y`, is the read-out's
//!   (the detector's spot check, unbounded, on every program — the user's
//!   ruling of 2026-11-03, which retired `-L` and `-K`): it adds its `paths`
//!   row and the summary's `monotonicity.paths`, which a `-y` cell leaves out
//!   on both sides ([`without_spot`]), and moves nothing else.
//! - **[`no_config_flag_moves_a_line_of_any_mode`]**, in-process. Every flag
//!   a `(config …)` can set, flipped from the program's own resolved value in
//!   an appended block (the loader keeps the last block whole), on every
//!   corpus program and register witness whose R is known, under the same
//!   sixteen questions: the read-out the binary prints, rendered uncut, with
//!   its stderr lines and exit code, and the summary. This is the route a flag
//!   takes into the saturator, which every mode's every node runs. Each
//!   (program, setting, question) is a unit of the pool of its own, so one
//!   program's questions run side by side. Each question is asked as `ein
//!   solve` asks it ([`explore::Options::new`], the read-out's cores alone),
//!   which [`the_librarys_question_does_the_binarys_work`] holds on the cell
//!   that once ran for most of an hour.
//!
//! **One exemption** (M1e2 S1e2.2.11, the user's ruling of 2026-11-04 on
//! Q-M1e2.11 (ii)): `:record-alternative-justifications false` makes the proof
//! a tree, and may change which core a dead node's refutation names, the
//! read-out's `unsat core` and `source` rows and the summary's `region.core`,
//! and since M1e2 S1e2.2.23 whether its search was cut, the `core search` row
//! and `region.core_status`; and since M1e2 S1e2.3.10, where the core moved,
//! the rule its derivation ends in, the `refuted by` row and
//! `region.refuted_by`; and nothing else ([`CORE_FLAG`], [`without_core`]).
//! [`the_core_exemption_is_one_flag_and_the_core_rows`] holds it that narrow.
//!
//! **And one register row** (M1f S1f.5.15): W36's witness prints another
//! `refuted by` under that flag
//! beside the same core, which the exemption does not cover. It is a wrong
//! answer, `open` and owned by F22, and its program is compared less that row
//! until F22 fixes it ([`REFUTED_BY_OPEN`]);
//! [`w36_is_exempt_from_refuted_by_until_f22`] holds the row open and the
//! difference there.
//!
//! **Byte for byte, cost counters included**, because no route reads a lever
//! today. The day one does, its `work` row and block may move, and this file
//! is where that is relaxed, on purpose (`jobs_invariance.rs`'s stance).
//!
//! **What R is known means here**: a bare describe exhausts at the default
//! `--region-budget`, so the answer the lever is compared under is the whole
//! of R and not a part a budget chose. The corpus's `slow` entries are the
//! programs it does not (a bare describe runs to the budget, which is why they
//! are slow since S1e2.1a.5's switch); `EIN_CORPUS_SLOW=1` adds their declared
//! cells to the first test, where a cut answer must still be the same cut
//! answer under every lever (C3 and C4 together), and the nightly runs it so.
//!
//! **What this cannot find**, said once here and on the stage's page: a
//! wrong answer on a program neither the oracle nor route 1 reaches, and a
//! wrong answer the saturator causes, because the oracle saturates with the
//! same code (`solution_semantics.md` § 6.6). A lever that moves nothing
//! printed on a program R is not known for is outside both tests.
//!
//! ```text
//! cargo test -p ein-cli --test lever_invariance
//! EIN_CORPUS_SLOW=1 cargo test -p ein-cli --release --test lever_invariance   # + the slow entries' cells
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use ein_core::SolverConfig;
use ein_core::config::{FLAGS, FieldKind, rendered_fields};
use ein_corpus::{Scratch, corpus, register, repo_root};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load, parse};
use serde_json::Value as J;

/// The sixteen questions of `ein solve`'s two axes, as flags: which part of R
/// (all, the minimal line, the maximal line) × what is asked of it (describe,
/// `-n 1`, `-n 2`, `-n k_s+1`, `--single`), and `--count`, over all of R
/// only (M1e2 T1e2.1c.4.6). `-n k_s+1` asks for more states than R holds, so
/// it is the run that lists all of them; where that is `-n 2` already, it is
/// asked once, and there are thirteen.
fn questions(k_s: u64) -> Vec<(Vec<String>, Mode)> {
    let mut out = Vec::new();
    for (wflag, which) in [
        (None, Which::All),
        (Some("--minimal"), Which::Minimal),
        (Some("--maximal"), Which::Maximal),
    ] {
        let n = |k: u64| (vec!["-n".to_string(), k.to_string()], Ask::Find(k));
        let mut asks = vec![(vec![], Ask::Describe), n(1), n(2)];
        if k_s + 1 > 2 {
            asks.push(n(k_s + 1));
        }
        asks.push((vec!["--single".to_string()], Ask::Single));
        for (aflags, ask) in asks {
            let mut flags: Vec<String> = wflag.iter().map(|s| s.to_string()).collect();
            flags.extend(aflags);
            out.push((flags, Mode { which, ask }));
        }
    }
    out.push((
        vec!["--count".to_string()],
        Mode {
            which: Which::All,
            ask: Ask::Count,
        },
    ));
    out
}

/// Run `work` over `items` on the corpus sweeps' pool, `ein_corpus::pool`
/// (M1f S1f.6.39; its own eight threads until then); the results in `items`'
/// order. `name` names an item as `(program, output)` for the pool's
/// instrument.
fn par_map<T: Sync, R: Send>(
    sweep: &str,
    items: &[T],
    name: impl Fn(&T) -> (String, String) + Sync,
    work: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    ein_corpus::pool::map(sweep, items, name, work)
}

/// A program's path from the repo root, as the pool's instrument names it.
fn rel(p: &Path) -> String {
    p.strip_prefix(repo_root())
        .unwrap_or(p)
        .display()
        .to_string()
}

/// The first line two texts differ on, for a failure message.
fn first_difference(a: &str, b: &str) -> String {
    let (mut la, mut lb) = (a.lines(), b.lines());
    for n in 1.. {
        match (la.next(), lb.next()) {
            (None, None) => return "identical".to_string(),
            (x, y) if x == y => continue,
            (x, y) => {
                return format!(
                    "line {n}:\n      default: {}\n      lever:   {}",
                    x.unwrap_or("<end>"),
                    y.unwrap_or("<end>")
                );
            }
        }
    }
    unreachable!()
}

/// The summary as compared: less the `config` echo, which is returned.
fn scrub(summary: &mut J) -> J {
    let Some(o) = summary.as_object_mut() else {
        return J::Null;
    };
    o.remove("config").unwrap_or(J::Null)
}

// ── 1. The declared cells, through the binary ──────────────────────────

/// What each CLI lever writes into the resolved config, which the summary's
/// `config` object echoes: the evidence that the lever reached the run.
/// `-L` and `-K` were here until M1e2 S1e2.1c.5 retired them (the user's
/// ruling of 2026-11-03); a register row's `settings` that names one records
/// the walk, and is not swept.
const SHADOWS: [(&str, &str, bool); 1] = [("-y", "lattice-sanity-check", true)];

/// `-y`'s own output, which is the one thing it may add (M1e2 S1e2.1c.5): the
/// spot check's `paths` row, under the `lattice` row, and the summary's
/// `monotonicity.paths`. Both are left out of a `-y` comparison, on both sides
/// — on a flagged program the bounded check prints the same row by default.
/// Everything else, the `work` row and block included, must not move.
fn without_spot(text: &str) -> String {
    text.lines()
        .filter(|l| !l.starts_with("    paths "))
        .map(|l| format!("{l}\n"))
        .collect()
}

fn summary_without_spot(summary: &str) -> String {
    let Ok(mut j) = serde_json::from_str::<J>(summary) else {
        return summary.to_string();
    };
    if let Some(m) = j.get_mut("monotonicity").and_then(J::as_object_mut) {
        m.remove("paths");
    }
    serde_json::to_string_pretty(&j).expect("serialises")
}

/// Every declared lever cell, by file: the corpus's `levers` columns, and the
/// CLI lever each register row's `settings` names (`lever: -L, …`).
///
/// `slow_too`: the slow entries' cells too, the nightly's, which the one test
/// that reads `EIN_CORPUS_SLOW` passes (M1f S1f.6.39: the nightly runs that
/// test by name, its other tests being the per-commit tier's).
fn declared(slow_too: bool) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for e in corpus().entry {
        if !e.levers.is_empty() && (slow_too || !e.slow) {
            out.entry(e.path.clone()).or_default().extend(e.levers);
        }
    }
    let slow: BTreeSet<String> = corpus()
        .entry
        .into_iter()
        .filter(|e| e.slow)
        .map(|e| e.path)
        .collect();
    for row in register().case {
        let Some(rest) = row.settings.strip_prefix("lever: -") else {
            continue;
        };
        let lever: String = std::iter::once('-')
            .chain(rest.chars().take_while(|c| c.is_ascii_alphanumeric()))
            .collect();
        if !SHADOWS.iter().any(|(s, _, _)| *s == lever) {
            continue;
        }
        if slow_too || !slow.contains(&row.witness) {
            out.entry(row.witness).or_default().insert(lever);
        }
    }
    out
}

/// One process's answer: the exit code, stdout, stderr, the summary as
/// [`scrub`] leaves it, and its `config` echo.
#[derive(PartialEq)]
struct Got {
    code: i32,
    stdout: String,
    stderr: String,
    summary: String,
    config: J,
}

fn solve(file: &str, flags: &[String]) -> Got {
    let dir = Scratch::new("ein-lever-invariance");
    let json = dir.join("summary.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .arg(file)
        .args(flags)
        .arg("--json-summary")
        .arg(&json)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let mut summary: J = std::fs::read_to_string(&json)
        .ok()
        .map(|s| serde_json::from_str(&s).expect("the summary parses"))
        .unwrap_or(J::Null);
    let config = scrub(&mut summary);
    Got {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        summary: serde_json::to_string_pretty(&summary).expect("serialises"),
        config,
    }
}

/// **Every declared lever cell, under every question, prints what the
/// default prints.**
#[test]
fn every_declared_lever_cell_moves_no_line_of_any_mode() {
    let slow_too = std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1");
    let cells = declared(slow_too);
    // Each file's describe at the default, for its k_s and whether R is known.
    let files: Vec<String> = cells.keys().cloned().collect();
    let bases = par_map(
        "lever_invariance::declared",
        &files,
        |f| (f.clone(), "solve".to_string()),
        |f| solve(f, &[]),
    );
    let mut known = 0usize;
    let mut cut = Vec::new();
    let mut jobs: Vec<(String, Vec<String>, Option<String>)> = Vec::new();
    for (file, base) in files.iter().zip(&bases) {
        let s: J = serde_json::from_str(&base.summary).expect("re-parses");
        assert_eq!(
            base.code, 0,
            "{file}: the default describe exits {}\n{}",
            base.code, base.stderr
        );
        if s["region"]["exhausted"] == J::Bool(true) {
            known += 1;
        } else {
            cut.push(file.clone());
        }
        let k_s = s["region"]["k_s"].as_u64().expect("a k_s");
        for (flags, _) in questions(k_s) {
            jobs.push((file.clone(), flags.clone(), None));
            for lever in &cells[file] {
                jobs.push((file.clone(), flags.clone(), Some(lever.clone())));
            }
        }
    }
    let got = par_map(
        "lever_invariance::declared",
        &jobs,
        |(file, flags, lever)| {
            (
                file.clone(),
                format!(
                    "solve {} {}",
                    flags.join(" "),
                    lever.as_deref().unwrap_or("")
                ),
            )
        },
        |(file, flags, lever)| match lever {
            None => solve(file, flags),
            Some(l) => {
                let mut argv = flags.clone();
                argv.push(l.clone());
                solve(file, &argv)
            }
        },
    );
    let mut defaults: BTreeMap<(&str, &[String]), &Got> = BTreeMap::new();
    for ((file, flags, lever), g) in jobs.iter().zip(&got) {
        if lever.is_none() {
            defaults.insert((file, flags), g);
        }
    }
    let mut bad = Vec::new();
    let mut declared_cells = 0usize;
    for ((file, flags, lever), g) in jobs.iter().zip(&got) {
        let Some(l) = lever else { continue };
        let at = format!("{file} {} {l}", flags.join(" "));
        declared_cells += 1;
        let Some(&(_, key, value)) = SHADOWS.iter().find(|(s, _, _)| s == l) else {
            bad.push(format!(
                "{at}: a lever this test does not know how to see reach a run"
            ));
            continue;
        };
        if g.config[key] != J::Bool(value) {
            bad.push(format!(
                "{at}: the summary's config does not carry the lever ({key} = {})",
                g.config[key]
            ));
        }
        let d = defaults[&(file.as_str(), flags.as_slice())];
        let spot = l == "-y";
        let out = |t: &str| if spot { without_spot(t) } else { t.to_string() };
        let sum = |t: &str| {
            if spot {
                summary_without_spot(t)
            } else {
                t.to_string()
            }
        };
        for (what, x, y) in [
            ("exit", d.code.to_string(), g.code.to_string()),
            ("stdout", out(&d.stdout), out(&g.stdout)),
            ("stderr", d.stderr.clone(), g.stderr.clone()),
            ("summary", sum(&d.summary), sum(&g.summary)),
        ] {
            if x != y {
                bad.push(format!("{at}: {what} moved, {}", first_difference(&x, &y)));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {declared_cells} lever cells printed something the default does not (C4). A \
         lever that changes an answer is a wrong answer and a register row, \
         never a weaker mode:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    eprintln!(
        "{} files ({known} with R known), {declared_cells} declared cells",
        files.len()
    );
    // The selection did not collapse: eight corpus entries declare a lever,
    // `-y`, since M1e2 S1e2.1c.5 retired `-L` and `-K` (twelve declared one
    // before), and six of them, every one outside the slow tier, have a known
    // R (2026-11-03): 93 declared cells — sixteen questions each, thirteen
    // where k_s = 1. The 186 environment cells beside them, the two
    // `EIN_OBLIGATION_CHOICE` settings over the same six files, went when
    // M1e2 S1e2.3.1 retired the variable.
    if !slow_too {
        assert!(cut.is_empty(), "R is not known for {cut:?}");
        assert!(
            known >= 6 && declared_cells >= 93,
            "{known} files, {declared_cells} declared cells"
        );
    }
}

// ── 2. Every `(config …)` flag, in-process ────────────────────────────

/// Every corpus program and register witness, less the slow tier (whose R a
/// bare describe does not reach). A debug build left out W7's two witnesses,
/// which it aborted on, until M1e2 S1e2.2.7 fixed W7, and W16's order pair
/// until M1e2 S1e2.2.16 fixed W16.
fn programs() -> Vec<PathBuf> {
    let slow: BTreeSet<PathBuf> = corpus()
        .entry
        .into_iter()
        .filter(|e| e.slow)
        .map(|e| repo_root().join(e.path))
        .collect();
    let mut files = ein_corpus::corpus_files();
    for row in register().case {
        for (_, a) in row.answers() {
            files.push(repo_root().join(row.witness_of(a)));
        }
    }
    files.sort();
    files.dedup();
    files.retain(|f| !slow.contains(f));
    files
}

/// The program's config as a `(config …)` block, every settable flag spelled
/// out, so that appended it replaces the program's own block whole.
fn block(c: &SolverConfig) -> String {
    let fields: BTreeMap<&str, String> = rendered_fields(c).into_iter().collect();
    let mut out = String::from("\n(config");
    for (flag, kind) in FLAGS {
        assert_eq!(
            *kind,
            FieldKind::Bool,
            "{flag}: every settable flag is a bool"
        );
        out.push_str(&format!(" :{flag} {}", fields[flag].to_lowercase()));
    }
    out.push_str(")\n");
    out
}

/// `c` as a block with `flag` flipped — checked to differ, so that a flag
/// the block does not spell cannot pass as one that changes nothing.
fn flipped(c: &SolverConfig, flag: &str) -> String {
    let b = block(c);
    let on = format!(":{flag} true");
    let off = format!(":{flag} false");
    let out = if b.contains(&on) {
        b.replace(&on, &off)
    } else {
        b.replace(&off, &on)
    };
    assert_ne!(out, b, ":{flag} is not in {b}");
    out
}

/// What `ein solve` prints for one question about the program `text`, read
/// at `path`: the read-out uncut, its stderr lines and its exit code, then
/// the `--json-summary` as [`scrub`] leaves it — or why there is none.
fn printed(path: &Path, text: &str, flags: &[String], mode: Mode) -> String {
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let file = path.to_string_lossy().to_string();
    let forms = match parse(&mut ast, text, path.to_str()) {
        Ok(f) => f,
        Err(e) => return format!("<parse> {e}"),
    };
    let mut kb = match load(&mut ast, &mut terms, &forms, path.parent()) {
        Ok(kb) => kb,
        Err(e) => return format!("<load> {}", e.0),
    };
    let opts = explore::Options::new(mode);
    let r = match ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts) {
        Ok(r) => r,
        Err(e) => return format!("<solve> {e}"),
    };
    let argv = format!("{file} {}", flags.join(" "));
    let t = ein_render::read_out::render(
        &r,
        &ein_render::read_out::Options {
            command: "solve",
            argv: argv.trim_end(),
            file: &file,
            budget: opts.budget,
            uncut: true,
        },
    );
    let cfg = kb.program().config.clone().unwrap_or_default();
    let summary = match ein_cli::summary::build_region(&ast, &mut terms, &mut kb, &r, &file, &cfg) {
        Ok(s) => {
            let mut j: J = serde_json::from_str(&ein_cli::summary::text(&s)).expect("parses");
            scrub(&mut j);
            serde_json::to_string_pretty(&j).expect("serialises")
        }
        Err(e) => format!("<summary> {e}"),
    };
    format!(
        "{}--- stderr\n{}\n--- exit {}\n--- summary\n{summary}\n",
        t.stdout,
        t.stderr.join("\n"),
        t.code
    )
}

/// The one flag the core is exempt under (M1e2 S1e2.2.11, Q-M1e2.11 (ii)):
/// off, a re-derivation is dropped rather than recorded, the explanation
/// search is offered each fact's first derivation alone, and the core it
/// names can be another. The classic zebra puzzle's own `(config …)` sets
/// it off for speed (0.95 s against 188 s, the same 458 cores, 2026-11-04;
/// `config.md` § 2.2); `alt-cap-core.ein` is where it changes a core.
const CORE_FLAG: &str = "record-alternative-justifications";

/// The programs on which [`CORE_FLAG`] moves `refuted by` beside a core that
/// did not move, each under a register row `open` and owned by F22:
/// `W36/alt-flag-refuted-by` (M1f S1f.5.15), on its witness and,
/// since S1f.5.15 took every `:priority` out, on `features/12`, the program
/// the witness was written from, which no typed order hides any more; and
/// `W36/alt-flag-two-derivations`, the witness no rule order hides. Each is
/// compared less the `refuted by` row and `region.refuted_by` under that flag,
/// and nothing else; an entry leaves with its row's fix.
const REFUTED_BY_OPEN: [(&str, &str); 3] = [
    (
        "W36/alt-flag-refuted-by",
        "examples/ein-bugs/wrong-answers/alt-flag-refuted-by.ein",
    ),
    (
        "W36/alt-flag-refuted-by",
        "examples/features/12_expect_false.ein",
    ),
    (
        "W36/alt-flag-two-derivations",
        "examples/ein-bugs/wrong-answers/alt-flag-two-derivations.ein",
    ),
];

/// The rows that name a dead node's core: the read-out's `unsat core` and
/// `source`, and since M1e2 S1e2.2.23 its `core search` row, which says the
/// core's search was cut: off, each fact's first derivation is all the search
/// is offered, so the search can be cut where it was not, or not where it was
/// (`core-search-cut.ein`'s `(A k)` has one environment off and 256 on).
fn is_core_row(line: &str) -> bool {
    line.starts_with("  unsat core ")
        || line.starts_with("  source ")
        || line.starts_with("  core search ")
}

/// The row that names the rules a dead node's core's derivation ends in
/// (M1e2 S1e2.3.10): it follows the core, so it may move where the core does.
fn is_refuted_row(line: &str) -> bool {
    line.starts_with("  refuted by ")
}

/// Whether the core moved between two [`printed`] texts: a core row
/// ([`is_core_row`]), or the summary's `region.core` or `region.core_status`.
fn core_moved(want: &str, got: &str) -> bool {
    let rows = |t: &str| -> Vec<String> {
        t.lines()
            .filter(|l| is_core_row(l))
            .map(str::to_string)
            .collect()
    };
    let summary = |t: &str| -> (J, J) {
        let s = t.split_once("--- summary\n").map_or("", |(_, s)| s);
        serde_json::from_str::<J>(s.trim_end()).map_or((J::Null, J::Null), |j| {
            (
                j["region"]["core"].clone(),
                j["region"]["core_status"].clone(),
            )
        })
    };
    rows(want) != rows(got) || summary(want) != summary(got)
}

/// [`printed`] less the core: its rows ([`is_core_row`]) and the summary's
/// `region.core` and `region.core_status`, and, where `refuted` (the core
/// moved, [`core_moved`]), the `refuted by` row and `region.refuted_by`.
/// Everything else, and `refuted by` beside a core that did not move, must
/// not move.
fn without_core(text: &str, refuted: bool) -> String {
    let (head, summary) = text.split_once("--- summary\n").unwrap_or((text, ""));
    let head: String = head
        .lines()
        .filter(|l| !is_core_row(l) && !(refuted && is_refuted_row(l)))
        .map(|l| format!("{l}\n"))
        .collect();
    let summary = match serde_json::from_str::<J>(summary.trim_end()) {
        Ok(mut j) => {
            if let Some(r) = j.get_mut("region").and_then(J::as_object_mut) {
                r.remove("core");
                r.remove("core_status");
                if refuted {
                    r.remove("refuted_by");
                }
            }
            serde_json::to_string_pretty(&j).expect("serialises")
        }
        Err(_) => summary.trim_end().to_string(),
    };
    format!("{head}--- summary\n{summary}\n")
}

/// [`printed`] less `-y`'s own output ([`without_spot`]): the `paths` row of
/// the read-out, and the summary's `monotonicity.paths`.
fn printed_without_spot(text: &str) -> String {
    let (head, summary) = text.split_once("--- summary\n").unwrap_or((text, ""));
    format!(
        "{}--- summary\n{}\n",
        without_spot(head),
        summary_without_spot(summary.trim_end())
    )
}

/// A program whose R is known: its text, its resolved config and its
/// questions.
struct Known {
    path: PathBuf,
    text: String,
    resolved: SolverConfig,
    questions: Vec<(Vec<String>, Mode)>,
}

/// Why a program is not swept, or what it is swept with.
enum Reach {
    /// It does not parse, load or solve: no read-out to compare.
    None,
    /// A bare describe stops at the default budget: R is not known.
    Partial,
    Known(Known),
}

fn reach(path: &Path) -> Reach {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Reach::None;
    };
    let mut ast = Ast::new();
    let mut terms = ein_core::Terms::new();
    let Ok(forms) = parse(&mut ast, &text, path.to_str()) else {
        return Reach::None;
    };
    let Ok(mut kb) = load(&mut ast, &mut terms, &forms, path.parent()) else {
        return Reach::None;
    };
    let resolved = kb.program().config.clone().unwrap_or_default();
    let describe = explore::Options::new(Mode {
        which: Which::All,
        ask: Ask::Describe,
    });
    let Ok(a) = explore::run(&mut kb, &mut terms, &ast, &describe) else {
        return Reach::None;
    };
    if !a.exhausted {
        return Reach::Partial;
    }
    let k_s = a.counts.map_or(0, |c| c.k_s);
    Reach::Known(Known {
        path: path.to_path_buf(),
        text,
        resolved,
        questions: questions(k_s),
    })
}

/// **No `(config …)` flag moves a line of any mode**, on any program whose R
/// is known.
///
/// It found one when it was written: `enable-symmetric-mirror` off took the
/// mirror's three facts out of root's fixpoint on
/// `examples/features/06_symmetric_native.ein`, on every question, so R
/// moved. T1e2.1a.6.3 retired the flag on the user's ruling of 2026-11-01 and
/// filed the case as the register's W13, fixed in the same commit. A program
/// that sets it is a load error now, so it is not among [`FLAGS`].
#[test]
fn no_config_flag_moves_a_line_of_any_mode() {
    let files = programs();
    let reached = par_map(
        "lever_invariance::reach",
        &files,
        |p| (rel(p), "reach".to_string()),
        |p| reach(p),
    );
    let (mut partial, mut none) = (0usize, 0usize);
    let mut known: Vec<Known> = Vec::new();
    for r in reached {
        match r {
            Reach::None => none += 1,
            Reach::Partial => partial += 1,
            Reach::Known(k) => known.push(k),
        }
    }
    // One item per program and setting — the program as written, and each
    // flag flipped — each asked its questions.
    let mut items: Vec<(usize, Option<&str>)> = Vec::new();
    for (i, _) in known.iter().enumerate() {
        items.push((i, None));
        items.extend(FLAGS.iter().map(|(f, _)| (i, Some(*f))));
    }
    // One unit per (program, setting, question), so that the questions of a
    // program with a large R run side by side rather than one after another
    // on one worker; the results are read back per item, in the items' order
    // and each item's questions in theirs, as one unit per item gave them.
    let units: Vec<(usize, usize)> = items
        .iter()
        .enumerate()
        .flat_map(|(n, &(i, _))| (0..known[i].questions.len()).map(move |q| (n, q)))
        .collect();
    let mut printed_all = par_map(
        "lever_invariance::flags",
        &units,
        |&(n, q)| {
            let (i, flag) = items[n];
            (
                rel(&known[i].path),
                format!(
                    "{} {}",
                    flag.unwrap_or("as written"),
                    known[i].questions[q].0.join(" ")
                )
                .trim_end()
                .to_string(),
            )
        },
        |&(n, q)| {
            let (i, flag) = items[n];
            let k = &known[i];
            let text = match flag {
                None => k.text.clone(),
                Some(f) => format!("{}{}", k.text, flipped(&k.resolved, f)),
            };
            let (flags, mode) = &k.questions[q];
            printed(&k.path, &text, flags, *mode)
        },
    )
    .into_iter();
    let outputs: Vec<Vec<String>> = items
        .iter()
        .map(|&(i, _)| {
            printed_all
                .by_ref()
                .take(known[i].questions.len())
                .collect()
        })
        .collect();
    let mut defaults: BTreeMap<usize, &Vec<String>> = BTreeMap::new();
    for (&(i, flag), out) in items.iter().zip(&outputs) {
        if flag.is_none() {
            defaults.insert(i, out);
        }
    }
    let (mut cells, mut bad, mut exempt) = (0usize, Vec::new(), BTreeSet::new());
    // `--count` at the default budget prints describe's rows, but `asked` and
    // the warning's `paths` (M1e2 T1e2.1c.4.6): one tool, one count, on every
    // program whose R is known. `read_out.rs` holds the same at a small
    // budget on every program, partial ones among them.
    for (&i, out) in &defaults {
        let k = &known[i];
        let at = |m: Mode| k.questions.iter().position(|(_, q)| *q == m);
        let (Some(d), Some(c)) = (
            at(Mode {
                which: Which::All,
                ask: Ask::Describe,
            }),
            at(Mode {
                which: Which::All,
                ask: Ask::Count,
            }),
        ) else {
            bad.push(format!(
                "{}: no describe or --count question",
                k.path.display()
            ));
            continue;
        };
        let shown: BTreeSet<&str> = out[d].lines().collect();
        for l in out[c].lines().skip(3).take_while(|l| !l.is_empty()) {
            if !l.starts_with("    paths ") && !shown.contains(l) {
                bad.push(format!(
                    "{}: --count prints a row describe does not: {l}",
                    k.path.display()
                ));
            }
        }
    }
    for (&(i, flag), out) in items.iter().zip(&outputs) {
        let Some(flag) = flag else { continue };
        let k = &known[i];
        let rel = k
            .path
            .strip_prefix(repo_root())
            .unwrap_or(&k.path)
            .display()
            .to_string();
        for (((flags, _), want), got) in k.questions.iter().zip(defaults[&i]).zip(out) {
            cells += 1;
            // `:lattice-sanity-check true` is `-y`: its row is its own.
            let (want, got) = if flag == "lattice-sanity-check" {
                (printed_without_spot(want), printed_without_spot(got))
            } else if flag == CORE_FLAG {
                if want != got {
                    exempt.insert(rel.clone());
                }
                // W36's program: `refuted by` too, until F22.
                let moved = core_moved(want, got) || REFUTED_BY_OPEN.iter().any(|(_, w)| *w == rel);
                (without_core(want, moved), without_core(got, moved))
            } else {
                (want.clone(), got.clone())
            };
            let (want, got) = (&want, &got);
            if got != want {
                bad.push(format!(
                    "{rel} {} with :{flag} flipped: {}",
                    flags.join(" "),
                    first_difference(want, got)
                ));
            }
        }
    }
    eprintln!(
        "{} programs whose R is known, {cells} cells; {partial} partial at the default \
         budget, {none} with no read-out; the core moved under :{CORE_FLAG} on {exempt:?}",
        known.len()
    );
    assert!(
        bad.is_empty(),
        "{} of {cells} (program, flag, question) cells printed something the \
         default does not. No config option may change the answer \
         (config.md § 2.5) — this is a register row:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    // 180 on 2026-11-01, of 255 files: 12 partial, 63 with no read-out.
    assert!(known.len() >= 180, "{} programs compared", known.len());
}

/// **The library's question does the binary's work**, on the cell
/// [`no_config_flag_moves_a_line_of_any_mode`] once ran for most of an hour.
///
/// `zebra.ein`'s own `(config …)` sets [`CORE_FLAG`] off, and says why: on,
/// a dead node's core is an explanation search over a densely re-derived
/// proof graph, some twenty seconds a node. The sweep flips it on. Its
/// questions were asked through [`explore::Options::new`], whose cores were
/// then every core any surface prints, a core at every dead node a route
/// visits, which only `--events` and `--dump-lattice` print and the sweep
/// does not read; `ein solve COPY --minimal`, which takes the read-out's
/// cores alone, answered in a second. `Options::new` asks the read-out's
/// cores now, as the binary does. So on `zebra.ein` with the flag on, every
/// question asked in process through `Options::new` answers before a
/// deadline that a core taken at each dead node passes after its first few
/// nodes, and prints what `ein solve` prints for it.
#[test]
fn the_librarys_question_does_the_binarys_work() {
    let path = repo_root().join("examples/zebra.ein");
    let Reach::Known(k) = reach(&path) else {
        panic!("zebra.ein: R is not known");
    };
    assert!(
        !k.resolved.record_alternative_justifications,
        "zebra.ein no longer sets :{CORE_FLAG} off: this test flips it on"
    );
    let dir = Scratch::new("ein-lever-library-question");
    let copy = dir.join("zebra-alternatives.ein");
    std::fs::write(
        &copy,
        format!("{}{}", k.text, flipped(&k.resolved, CORE_FLAG)),
    )
    .expect("the copy writes");
    let file = copy.to_string_lossy().to_string();
    let text = std::fs::read_to_string(&copy).expect("the copy reads");
    let mismatched = par_map(
        "lever_invariance::library_question",
        &k.questions,
        |(flags, _)| ("examples/zebra.ein".to_string(), flags.join(" ")),
        |(flags, mode)| {
            let at = format!("{file} {}", flags.join(" "));
            let argv = at.trim_end();
            let opts = explore::Options {
                deadline: Some(std::time::Instant::now() + std::time::Duration::from_secs(60)),
                ..explore::Options::new(*mode)
            };
            let mut ast = Ast::new();
            let mut terms = ein_core::Terms::new();
            let forms = parse(&mut ast, &text, Some(file.as_str())).expect("the copy parses");
            let mut kb = load(&mut ast, &mut terms, &forms, copy.parent()).expect("it loads");
            let slow = "in process, the question ran past a minute where `ein solve` answers \
                        at once: does `Options::new` take cores no surface it prints reads?";
            let r = ein_infer::read_out::run(&mut kb, &mut terms, &ast, &opts)
                .unwrap_or_else(|e| panic!("{argv}: {e}; {slow}"));
            assert_ne!(
                r.answer.stopped,
                Some(explore::Stopped::Deadline),
                "{argv}: {slow}"
            );
            let t = ein_render::read_out::render(
                &r,
                &ein_render::read_out::Options {
                    command: "solve",
                    argv,
                    file: &file,
                    budget: opts.budget,
                    uncut: false,
                },
            );
            let out = Command::new(env!("CARGO_BIN_EXE_ein"))
                .arg("solve")
                .arg(&file)
                .args(flags)
                .current_dir(repo_root())
                .output()
                .expect("the `ein` binary runs");
            let stdout = String::from_utf8_lossy(&out.stdout);
            (out.status.code() != Some(t.code) || stdout != t.stdout).then(|| {
                format!(
                    "{argv}: exit {:?} from the binary and {} in process; its stdout, as \
                     `default`, against the read-out in process, {}",
                    out.status.code(),
                    t.code,
                    first_difference(&stdout, &t.stdout)
                )
            })
        },
    );
    let bad: Vec<String> = mismatched.into_iter().flatten().collect();
    assert!(
        bad.is_empty(),
        "the library's question prints otherwise than `ein solve`'s:\n{}",
        bad.join("\n")
    );
    assert_eq!(
        explore::Options::new(k.questions[0].1).cores,
        ein_infer::region::Cores::READ_OUT,
        "the library's default cores are the read-out's"
    );
}

/// **W36 is exempt from `refuted by` until F22 fixes it** (M1f S1f.5.15).
///
/// Each [`REFUTED_BY_OPEN`] entry names a register row that is `open` and
/// owned by F22, and a program that is its witness or that its note names;
/// and on that program
/// [`CORE_FLAG`] still moves the `refuted by` row beside a core that did not
/// move, on some question, and nothing else. The day it stops, the row is
/// fixed and the entry goes with it.
#[test]
fn w36_is_exempt_from_refuted_by_until_f22() {
    let reg = register();
    for (id, witness) in REFUTED_BY_OPEN {
        let row = reg
            .case
            .iter()
            .find(|r| r.id == id)
            .unwrap_or_else(|| panic!("{id}: not a register row"));
        assert_eq!(
            (row.status.as_str(), row.owner.as_str()),
            ("open", "F22"),
            "{id}: the exemption is an open row of F22's"
        );
        assert!(
            row.witness == witness || row.note.as_deref().is_some_and(|n| n.contains(witness)),
            "{id}: {witness} is neither its witness nor named in its note"
        );
        let path = repo_root().join(witness);
        let Reach::Known(k) = reach(&path) else {
            panic!("{witness}: R is not known");
        };
        let flipped_text = format!("{}{}", k.text, flipped(&k.resolved, CORE_FLAG));
        let mut moved = 0usize;
        for (flags, mode) in &k.questions {
            let want = printed(&k.path, &k.text, flags, *mode);
            let got = printed(&k.path, &flipped_text, flags, *mode);
            let at = flags.join(" ");
            assert!(
                !core_moved(&want, &got),
                "{witness} {at}: the core moved, which the flag's own exemption covers"
            );
            assert_eq!(
                without_core(&want, true),
                without_core(&got, true),
                "{witness} {at}: :{CORE_FLAG} moved more than `refuted by`"
            );
            if want != got {
                moved += 1;
            }
        }
        assert!(
            moved >= 1,
            "{witness}: :{CORE_FLAG} no longer moves `refuted by` — {id} is fixed: flip \
             the row and take its entry out of REFUTED_BY_OPEN"
        );
    }
}

/// **The core exemption is one flag and the core rows** (M1e2 S1e2.2.11, the
/// user's ruling of 2026-11-04 on Q-M1e2.11 (ii)).
///
/// [`no_config_flag_moves_a_line_of_any_mode`] compares
/// [`CORE_FLAG`]'s cells less the core. This holds the exemption to what the
/// ruling grants: the flag is a `(config …)` flag, the rows taken out are the
/// core's and nothing else on any line, and on `alt-cap-core.ein`, the
/// program where the flag changes a core, every question prints the same
/// read-out and summary but those rows, while the rows do differ. The day
/// they stop differing there the exemption has no witness, and this fails
/// rather than let it outlive its reason.
#[test]
fn the_core_exemption_is_one_flag_and_the_core_rows() {
    assert!(
        FLAGS.iter().any(|(f, _)| *f == CORE_FLAG),
        ":{CORE_FLAG} is not a (config …) flag"
    );
    for (row, core) in [
        ("  unsat core        (w1 X)", true),
        (
            "  source            (color-loc Green House-1) :source \"x\"",
            true,
        ),
        (
            "  core search       cut at its budget of 64 environments per fact: a smaller core may exist",
            true,
        ),
        ("  refuted by        narrow", false),
        ("  refuted by", false),
        ("  unsat corel", false),
        ("    core search", false),
        ("    source", false),
        ("  universe          70 candidates, 70 dead alone", false),
    ] {
        assert_eq!(is_core_row(row), core, "{row:?}");
    }
    assert!(is_refuted_row("  refuted by        narrow") && !is_refuted_row("    refuted by"));

    let path = repo_root().join("examples/ein-bugs/alt-cap-core.ein");
    let Reach::Known(k) = reach(&path) else {
        panic!("alt-cap-core.ein: R is not known");
    };
    let flipped_text = format!("{}{}", k.text, flipped(&k.resolved, CORE_FLAG));
    let mut moved = 0usize;
    for (flags, mode) in &k.questions {
        let want = printed(&k.path, &k.text, flags, *mode);
        let got = printed(&k.path, &flipped_text, flags, *mode);
        let at = flags.join(" ");
        let core = core_moved(&want, &got);
        assert_eq!(
            without_core(&want, core),
            without_core(&got, core),
            "alt-cap-core.ein {at}: :{CORE_FLAG} moved more than the core and its rules"
        );
        let rows = |t: &str| -> Vec<String> {
            t.lines()
                .filter(|l| is_core_row(l))
                .map(str::to_string)
                .collect()
        };
        let summary_core = |t: &str| -> J {
            let s = t.split_once("--- summary\n").map_or("", |(_, s)| s);
            serde_json::from_str::<J>(s.trim_end())
                .map(|j| j["region"]["core"].clone())
                .unwrap_or(J::Null)
        };
        if want != got {
            moved += 1;
            assert_ne!(
                rows(&want),
                rows(&got),
                "{at}: moved, but not in a core row"
            );
            assert_eq!(
                (summary_core(&want), summary_core(&got)),
                (
                    serde_json::json!(["(w1 X)", "(w2 X)"]),
                    serde_json::json!(["(p1 Y01)", "(p2 Y01)", "(p3 Y01)"])
                ),
                "{at}: the summary's core, alternatives on and off"
            );
            // The rule each core's derivation ends in (M1e2 S1e2.3.10):
            // `wide` from `(w1 X) (w2 X)`, `narrow` from the chain.
            let refuted = |t: &str| -> Vec<String> {
                t.lines()
                    .filter(|l| is_refuted_row(l))
                    .map(str::to_string)
                    .collect()
            };
            assert_eq!(
                (refuted(&want), refuted(&got)),
                (
                    vec!["  refuted by        wide".to_string()],
                    vec!["  refuted by        narrow".to_string()]
                ),
                "{at}: refuted by, alternatives on and off"
            );
        }
    }
    // Describe and `--count` print the core; the lines and `-n` print no core
    // on an empty R.
    assert!(
        moved >= 1,
        "alt-cap-core.ein: :{CORE_FLAG} no longer changes its core, so the \
         exemption has no witness — retire it from no_config_flag_moves_a_line_of_any_mode"
    );
}

// ── 3. `--no-search`: R not discovered, never another R ───────────────

/// The rows that count or list R: none is printed under `--no-search`.
const R_ROWS: [&str; 11] = [
    "states (k_s)",
    "commitments (k_c)",
    "exhausted",
    "empty",
    "minimal line",
    "maximal line",
    "components",
    "sub-cubes",
    "merged",
    "N",
    "E",
];

/// One run's exit, stdout, stderr and summary (`Null` where it wrote none).
fn run_json(file: &Path, flags: &[&str]) -> (i32, String, String, J) {
    let dir = Scratch::new("ein-lever-no-search");
    let json = dir.join("summary.json");
    let out = Command::new(env!("CARGO_BIN_EXE_ein"))
        .arg("solve")
        .arg(file)
        .args(flags)
        .arg("--json-summary")
        .arg(&json)
        .current_dir(repo_root())
        .output()
        .expect("the `ein` binary runs");
    let summary = std::fs::read_to_string(&json)
        .ok()
        .map(|s| serde_json::from_str(&s).expect("the summary parses"))
        .unwrap_or(J::Null);
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        summary,
    )
}

/// A claim's outcome in a summary: the `(solve :expect)`'s, then each query's.
fn outcomes(s: &J) -> Vec<J> {
    let mut v = vec![s["claim"]["outcome"].clone()];
    if let Some(qs) = s["queries"].as_array() {
        v.extend(qs.iter().map(|q| q["expect"]["outcome"].clone()));
    }
    v
}

/// **`--no-search` reports root as the full run reaches it, and states no
/// R** — M1f
/// [S1f.6.36](../../../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.36_no_search.md),
/// the user's words of 2026-11-14, *"flag that disables search doesn't change
/// R, it disables discovering of R"*. The flag is no lever that a complete
/// search could answer another R under: it declines to search. So on every
/// program [`programs`] holds, against a bare `ein solve`:
///
/// - a program the full run refuses is refused alike, the same exit and
///   stderr;
/// - otherwise the full run's `region.root`, `region.root_owes` and
///   `region.root_owed` are the `--no-search` summary's `root.dead`,
///   `root.owes` and `root.owed`, the two `root` blocks are equal, its
///   `region` is `null` and `asked.ask` is `no-search`, and its stdout prints
///   no row that counts or lists R ([`R_ROWS`]);
/// - each claim's outcome is the full run's or `not-checked`, never another.
#[test]
fn no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r() {
    let files = programs();
    // One unit per (program, run), the full run and `--no-search` side by
    // side; folded per program, in the programs' order.
    let units: Vec<(usize, bool)> = (0..files.len())
        .flat_map(|f| [(f, false), (f, true)])
        .collect();
    let mut runs = par_map(
        "lever_invariance::no_search",
        &units,
        |&(f, no_search)| {
            (
                rel(&files[f]),
                if no_search { "--no-search" } else { "solve" }.to_string(),
            )
        },
        |&(f, no_search)| {
            let flags: &[&str] = if no_search { &["--no-search"] } else { &[] };
            run_json(&files[f], flags)
        },
    )
    .into_iter();
    let mut compared = 0usize;
    let mut bad: Vec<String> = Vec::new();
    for path in &files {
        let (fc, _, ferr, full) = runs.next().expect("the full run");
        let (nc, nout, nerr, ns) = runs.next().expect("the --no-search run");
        bad.extend(no_search_against_full(
            path,
            &mut compared,
            (fc, ferr, full),
            (nc, nout, nerr, ns),
        ));
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // A floor, so that a sweep that compared nothing cannot pass: 517 of the
    // 856 tracked programs wrote a summary on 2026-11-14 (S1f.6.36's T1),
    // the slow tier among them.
    assert!(compared >= 400, "only {compared} programs compared");
}

/// What [`no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r`]
/// finds wrong on one program: the full run's exit, stderr and summary
/// against `--no-search`'s exit, stdout, stderr and summary.
fn no_search_against_full(
    path: &Path,
    compared: &mut usize,
    (fc, ferr, full): (i32, String, J),
    (nc, nout, nerr, ns): (i32, String, String, J),
) -> Vec<String> {
    let at = path
        .strip_prefix(repo_root())
        .unwrap_or(path)
        .display()
        .to_string();
    let mut bad = Vec::new();
    if full.is_null() {
        if nc != fc || nerr != ferr || !ns.is_null() {
            bad.push(format!(
                "{at}: refused by the full run (exit {fc}), and --no-search exits {nc}:\n{nerr}"
            ));
        }
        return bad;
    }
    if ns.is_null() {
        bad.push(format!(
            "{at}: --no-search wrote no summary (exit {nc}): {nerr}"
        ));
        return bad;
    }
    *compared += 1;
    let r = &full["region"];
    let n = &ns["root"];
    let standing = if n["dead"] == J::Bool(true) {
        "dead"
    } else if n["owes"] == 0 {
        "in-r"
    } else {
        "owing"
    };
    if r["root"] != standing || r["root_owes"] != n["owes"] || r["root_owed"] != n["owed"] {
        bad.push(format!(
            "{at}: root is ({}, {}, {}) to the full run and ({standing}, {}, {}) to --no-search",
            r["root"], r["root_owes"], r["root_owed"], n["owes"], n["owed"]
        ));
    }
    if full["root"] != ns["root"] {
        bad.push(format!("{at}: the root blocks differ"));
    }
    if !ns["region"].is_null() || ns["asked"]["ask"] != "no-search" {
        bad.push(format!(
            "{at}: --no-search states a region, or asked {}",
            ns["asked"]
        ));
    }
    for label in R_ROWS {
        let printed = nout.lines().any(|l| {
            l.strip_prefix("  ")
                .and_then(|rest| rest.strip_prefix(label))
                .is_some_and(|rest| rest.starts_with(' '))
        });
        if printed {
            bad.push(format!("{at}: --no-search prints the row {label}"));
        }
    }
    for (want, got) in outcomes(&full).iter().zip(outcomes(&ns)) {
        if got != *want && got != "not-checked" {
            bad.push(format!(
                "{at}: a claim is {want} to the full run and {got} to --no-search"
            ));
        }
    }
    bad
}

/// The keys of a summary's `region` block that are `R`'s or around it, and
/// that both of describe's routes compute: every one but the route's own
/// (`route`, `licences`, `stopped`, `complete_up_to`), and `N` and an empty
/// `R`'s owing frontier, which the state route alone computes where root
/// owes an instance a licence takes and which are compared where both wrote
/// them.
const R_KEYS: [&str; 14] = [
    "k_s",
    "k_c",
    "exhausted",
    "root",
    "root_owes",
    "root_owed",
    "empty",
    "minimal",
    "maximal",
    "components",
    "subcubes",
    "merged",
    "universe",
    "states",
];

/// **`--describe-route` moves no part of R**: describe and `--count` take
/// the tree and the classes by default and the state route when asked, and
/// the option chooses a route and never R. On every corpus program and
/// register witness (the `slow` entries apart, whose state route the budget
/// stops), describe and `--count` under `--describe-route tree` and `states`
/// through the binary: where both runs exhausted R, every key of the
/// summary's `region` block that is R's ([`R_KEYS`]) is the same, `nogoods`
/// and `frontier` are the same wherever both wrote one, and every claim's
/// outcome is the same but one over an empty R's owing frontier that the
/// tree and the classes did not compute, which they leave not checked. Stdout differs in the `route`, `work` and `N` rows,
/// the owing listing, and a stopped run's rows and note alone, which this
/// does not read.
#[test]
fn the_describe_route_moves_no_part_of_r() {
    let files = programs();
    // One unit per (program, question, route), so that a program's four runs
    // go side by side; folded per (program, question), in the programs'
    // order and describe before `--count`, as one unit per program read them.
    const ROUTES: [&str; 2] = ["tree", "states"];
    let units: Vec<(usize, bool, &str)> = (0..files.len())
        .flat_map(|f| {
            [false, true]
                .into_iter()
                .flat_map(move |count| ROUTES.into_iter().map(move |r| (f, count, r)))
        })
        .collect();
    let mut runs = par_map(
        "lever_invariance::describe_route",
        &units,
        |&(f, count, route)| {
            (
                rel(&files[f]),
                format!(
                    "{}--describe-route {route}",
                    if count { "--count " } else { "" }
                ),
            )
        },
        |&(f, count, route)| {
            let mut flags: Vec<&str> = if count { vec!["--count"] } else { Vec::new() };
            flags.extend(["--describe-route", route]);
            run_json(&files[f], &flags)
        },
    )
    .into_iter();
    let mut compared = 0usize;
    let mut bad: Vec<String> = Vec::new();
    for path in &files {
        let at = rel(path);
        for count in [false, true] {
            let (tc, _, terr, tree) = runs.next().expect("the tree's run");
            let (sc, _, serr, states) = runs.next().expect("the state route's run");
            if tree.is_null() || states.is_null() {
                if tc != sc || terr != serr {
                    bad.push(format!(
                        "{at}: no summary, and the two routes exit {tc} and {sc}"
                    ));
                }
                continue;
            }
            let (t, s) = (&tree["region"], &states["region"]);
            if t.is_null() || s.is_null() {
                if t != s {
                    bad.push(format!("{at}: one route states a region and one does not"));
                }
                continue;
            }
            if t["exhausted"] != J::Bool(true) || s["exhausted"] != J::Bool(true) {
                continue;
            }
            compared += 1;
            for key in R_KEYS {
                if t[key] != s[key] {
                    bad.push(format!(
                        "{at}{}: region.{key} is {} by the tree and {} by the state route",
                        if count { " --count" } else { "" },
                        t[key],
                        s[key]
                    ));
                }
            }
            for key in ["nogoods", "frontier"] {
                if !t[key].is_null() && !s[key].is_null() && t[key] != s[key] {
                    bad.push(format!("{at}: region.{key} differs where both computed it"));
                }
            }
            // A claim over an empty R's owing frontier is not checked
            // where the tree and the classes did not compute it.
            let frontier_skipped = t["frontier"].is_null() && !s["frontier"].is_null();
            for (x, y) in outcomes(&tree).iter().zip(outcomes(&states)) {
                if *x != y && !(frontier_skipped && *x == "not-checked") {
                    bad.push(format!(
                        "{at}: a claim is {x} by the tree and {y} by the state route"
                    ));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    assert!(compared >= 800, "only {compared} runs compared");
}
