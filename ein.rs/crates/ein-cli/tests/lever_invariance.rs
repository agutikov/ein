//! **C4 under every lever** — M1e2
//! [S1e2.1a.6](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/s1e2.1a.6_engine_against_oracle.md)
//! T1e2.1a.6.2, the config principle as a test.
//!
//! [`solve_interface.md` § 1](../../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#1-the-contract)'s
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
//!   of `ein solve`'s fifteen questions (three parts of R × describe, `-n 1`,
//!   `-n 2`, `-n k_s+1`, `--single`) and compared with the same question at
//!   the default: the exit code, stdout, stderr and the `--json-summary`. The
//!   summary's `config` object echoes the lever, so it is left out of the
//!   comparison and checked to carry it: a lever that stopped reaching the run
//!   cannot pass as one that changes nothing. The three `EIN_*` levers the
//!   walk read (`EIN_OBLIGATION_CHOICE=off|fail-first`, `EIN_TRAVERSAL=tree`)
//!   reach a run the same way, as a process, and nothing declares them, so
//!   they are swept over the same files. The CLI shadows `-L`, `-K` and `-y`
//!   reach `-c`, the `run` event and the echo and nothing else
//!   (`ein-cli/src/modes.rs`), so what this holds is the process path; the day
//!   P1e2.1c wires one into a route, this is the test that has to stay green.
//! - **[`no_config_flag_moves_a_line_of_any_mode`]**, in-process. Every flag
//!   a `(config …)` can set, flipped from the program's own resolved value in
//!   an appended block (the loader keeps the last block whole), on every
//!   corpus program and register witness whose R is known, under the same
//!   fifteen questions: the read-out the binary prints, rendered uncut, with
//!   its stderr lines and exit code, and the summary. This is the route a flag
//!   takes into the saturator, which every mode's every node runs.
//!
//! **Byte for byte, cost counters included**, because no route reads a lever
//! today. The day one does, its `work` row and block may move, and this file
//! is where that is relaxed, on purpose (`jobs_invariance.rs`'s stance).
//!
//! **Three fields of the summary are left out**, `root.facts`,
//! `root.facts_by_relation` and `root.hypgen`, and the reason is a defect
//! this test found and does not own. `summary::root_block` runs the **walk's**
//! hypothesis generator on a fork of root, which reads the program's
//! lookahead and kill-cache flags and `EIN_OBLIGATION_CHOICE`, and counts
//! root's facts after it — so the block counts the kill cache's `(not h)`
//! writes as root facts (27 where root's fixpoint holds 23 on
//! `branching/02_one_dead_one_alive.ein`), describes a generator no route
//! runs, and moves under `(config :enable-pre-branch-lookahead false)` while
//! `-L` leaves it alone. Nothing R is made of moves: the read-out, the states,
//! the classes and every other block are identical. It is surface **S4** of
//! the proposed P1e2.3 (`plans/m1e2_correctness/README.md`), where
//! T1e2.1a.6.2 parked it.
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
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use ein_core::SolverConfig;
use ein_core::config::{FLAGS, FieldKind, rendered_fields};
use ein_corpus::{corpus, register, repo_root};
use ein_infer::explore::{self, Ask, Mode, Which};
use ein_ir::{Ast, load, parse};
use serde_json::Value as J;

/// The fifteen questions of `ein solve`'s two axes, as flags: which part of R
/// (all, the minimal line, the maximal line) × what is asked of it (describe,
/// `-n 1`, `-n 2`, `-n k_s+1`, `--single`). `-n k_s+1` asks for more states
/// than R holds, so it is the run that lists all of them; where that is
/// `-n 2` already, it is asked once, and there are twelve.
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
    out
}

fn workers() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().min(8))
}

/// Run `work` over `items` on [`workers`] threads; the results in `items`'
/// order.
fn par_map<T: Sync, R: Send>(items: &[T], work: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..workers() {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(i) else { break };
                    let r = work(item);
                    out.lock().expect("a worker panicked")[i] = Some(r);
                }
            });
        }
    });
    out.into_inner()
        .expect("a worker panicked")
        .into_iter()
        .map(|r| r.expect("every item ran"))
        .collect()
}

fn slow_too() -> bool {
    std::env::var("EIN_CORPUS_SLOW").as_deref() == Ok("1")
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

/// The summary as compared: less the `config` echo, and less the three
/// fields of `root` that read the walk's generator (S4, the module note).
/// The echo is returned.
fn scrub(summary: &mut J) -> J {
    let Some(o) = summary.as_object_mut() else {
        return J::Null;
    };
    if let Some(root) = o.get_mut("root").and_then(J::as_object_mut) {
        for k in ["facts", "facts_by_relation", "hypgen"] {
            root.remove(k);
        }
    }
    o.remove("config").unwrap_or(J::Null)
}

// ── 1. The declared cells, through the binary ──────────────────────────

/// What each CLI lever writes into the resolved config, which the summary's
/// `config` object echoes: the evidence that the lever reached the run.
const SHADOWS: [(&str, &str, bool); 3] = [
    ("-L", "enable-pre-branch-lookahead", false),
    ("-K", "enable-lookahead-kill-cache", false),
    ("-y", "lattice-sanity-check", true),
];

/// The walk's three environment levers (`configuration.md` § 4). Nothing
/// declares them; they are swept over the files that declare a CLI lever.
const ENV: [(&str, &str); 3] = [
    ("EIN_OBLIGATION_CHOICE", "off"),
    ("EIN_OBLIGATION_CHOICE", "fail-first"),
    ("EIN_TRAVERSAL", "tree"),
];

/// A lever: a CLI flag, or an environment variable set for the process.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Lever {
    Flag(String),
    Env(&'static str, &'static str),
}

impl std::fmt::Display for Lever {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lever::Flag(l) => write!(f, "{l}"),
            Lever::Env(k, v) => write!(f, "{k}={v}"),
        }
    }
}

/// Every declared lever cell, by file: the corpus's `levers` columns, and the
/// CLI lever each register row's `settings` names (`lever: -L, …`).
fn declared() -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for e in corpus().entry {
        if !e.levers.is_empty() && (slow_too() || !e.slow) {
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
        if slow_too() || !slow.contains(&row.witness) {
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

fn solve(file: &str, flags: &[String], env: Option<(&str, &str)>) -> Got {
    static RUN: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!("ein-lever-invariance-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    let json = dir.join(format!("{}.json", RUN.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_file(&json);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ein"));
    cmd.arg("solve")
        .arg(file)
        .args(flags)
        .arg("--json-summary")
        .arg(&json)
        .current_dir(repo_root());
    // The default is the default whatever the caller's shell exports.
    for (k, _) in ENV {
        cmd.env_remove(k);
    }
    if let Some((k, v)) = env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("the `ein` binary runs");
    let mut summary: J = std::fs::read_to_string(&json)
        .ok()
        .map(|s| serde_json::from_str(&s).expect("the summary parses"))
        .unwrap_or(J::Null);
    let _ = std::fs::remove_file(&json);
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
/// default prints**, and so does each environment lever on the same files.
#[test]
fn every_declared_lever_cell_moves_no_line_of_any_mode() {
    let cells = declared();
    // Each file's describe at the default, for its k_s and whether R is known.
    let files: Vec<String> = cells.keys().cloned().collect();
    let bases = par_map(&files, |f| solve(f, &[], None));
    let mut known = 0usize;
    let mut cut = Vec::new();
    let mut jobs: Vec<(String, Vec<String>, Option<Lever>)> = Vec::new();
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
        let levers = cells[file]
            .iter()
            .map(|l| Lever::Flag(l.clone()))
            .chain(ENV.iter().map(|&(k, v)| Lever::Env(k, v)));
        let levers: Vec<Lever> = levers.collect();
        for (flags, _) in questions(k_s) {
            jobs.push((file.clone(), flags.clone(), None));
            for lever in &levers {
                jobs.push((file.clone(), flags.clone(), Some(lever.clone())));
            }
        }
    }
    let got = par_map(&jobs, |(file, flags, lever)| match lever {
        None => solve(file, flags, None),
        Some(Lever::Flag(l)) => {
            let mut argv = flags.clone();
            argv.push(l.clone());
            solve(file, &argv, None)
        }
        Some(Lever::Env(k, v)) => solve(file, flags, Some((k, v))),
    });
    let mut defaults: BTreeMap<(&str, &[String]), &Got> = BTreeMap::new();
    for ((file, flags, lever), g) in jobs.iter().zip(&got) {
        if lever.is_none() {
            defaults.insert((file, flags), g);
        }
    }
    let mut bad = Vec::new();
    let (mut declared_cells, mut env_cells) = (0usize, 0usize);
    for ((file, flags, lever), g) in jobs.iter().zip(&got) {
        let Some(lever) = lever else { continue };
        let at = format!("{file} {} {lever}", flags.join(" "));
        match lever {
            Lever::Flag(l) => {
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
            }
            Lever::Env(..) => env_cells += 1,
        }
        let d = defaults[&(file.as_str(), flags.as_slice())];
        for (what, x, y) in [
            ("exit", d.code.to_string(), g.code.to_string()),
            ("stdout", d.stdout.clone(), g.stdout.clone()),
            ("stderr", d.stderr.clone(), g.stderr.clone()),
            ("summary", d.summary.clone(), g.summary.clone()),
        ] {
            if x != y {
                bad.push(format!("{at}: {what} moved, {}", first_difference(&x, &y)));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "{} of {} lever cells printed something the default does not (C4). A \
         lever that changes an answer is a wrong answer and a register row, \
         never a weaker mode:\n\n{}",
        bad.len(),
        declared_cells + env_cells,
        bad.join("\n\n")
    );
    eprintln!(
        "{} files ({known} with R known), {declared_cells} declared cells, {env_cells} \
         environment cells",
        files.len()
    );
    // The selection did not collapse: twelve corpus entries declare levers,
    // and ten of them, every one outside the slow tier, have a known R
    // (2026-11-01): 348 declared cells — fifteen questions each, twelve where
    // k_s = 1 — and 432 environment cells beside them.
    if !slow_too() {
        assert!(cut.is_empty(), "R is not known for {cut:?}");
        assert!(
            known >= 10 && declared_cells >= 348 && env_cells >= 432,
            "{known} files, {declared_cells} declared cells, {env_cells} environment cells"
        );
    }
}

// ── 2. Every `(config …)` flag, in-process ────────────────────────────

/// Every corpus program and register witness, less the slow tier (whose R a
/// bare describe does not reach) and, in a debug build, the witnesses it
/// aborts on (W7's two), as `explore.rs` leaves them out.
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
            if cfg!(debug_assertions) && a.debug_panic.is_some() {
                continue;
            }
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

/// The disagreements this test found and a later task of the stage removes,
/// each pinned so that it cannot go quiet unnoticed: every question on the
/// program must still disagree under the flag.
///
/// `enable-symmetric-mirror` off takes the mirror's three facts out of
/// root's fixpoint on the one program that marks a relation
/// `(__symmetric__ R)`, so the query goal `(knows Bob Ann)` is no longer
/// derived — R moves. The user ruled on 2026-11-01 that the flag retires;
/// T1e2.1a.6.3 retires it, files the register row, and deletes this line.
const KNOWN: [(&str, &str); 1] = [(
    "examples/features/06_symmetric_native.ein",
    "enable-symmetric-mirror",
)];

/// **No `(config …)` flag moves a line of any mode**, on any program whose R
/// is known.
#[test]
fn no_config_flag_moves_a_line_of_any_mode() {
    let files = programs();
    let reached = par_map(&files, |p| reach(p));
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
    // flag flipped — so that a program with a large R is spread over the
    // workers rather than run on one.
    let mut items: Vec<(usize, Option<&str>)> = Vec::new();
    for (i, _) in known.iter().enumerate() {
        items.push((i, None));
        items.extend(FLAGS.iter().map(|(f, _)| (i, Some(*f))));
    }
    let outputs = par_map(&items, |&(i, flag)| {
        let k = &known[i];
        let text = match flag {
            None => k.text.clone(),
            Some(f) => format!("{}{}", k.text, flipped(&k.resolved, f)),
        };
        k.questions
            .iter()
            .map(|(flags, mode)| printed(&k.path, &text, flags, *mode))
            .collect::<Vec<String>>()
    });
    let mut defaults: BTreeMap<usize, &Vec<String>> = BTreeMap::new();
    for (&(i, flag), out) in items.iter().zip(&outputs) {
        if flag.is_none() {
            defaults.insert(i, out);
        }
    }
    let (mut cells, mut bad) = (0usize, Vec::new());
    let mut pinned: BTreeMap<(String, &str), (usize, usize)> = BTreeMap::new();
    for (&(i, flag), out) in items.iter().zip(&outputs) {
        let Some(flag) = flag else { continue };
        let k = &known[i];
        let rel = k
            .path
            .strip_prefix(repo_root())
            .unwrap_or(&k.path)
            .display()
            .to_string();
        let is_known = KNOWN.contains(&(rel.as_str(), flag));
        for (((flags, _), want), got) in k.questions.iter().zip(defaults[&i]).zip(out) {
            cells += 1;
            if is_known {
                let p = pinned.entry((rel.clone(), flag)).or_default();
                p.0 += 1;
                p.1 += usize::from(got != want);
                continue;
            }
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
         budget, {none} with no read-out",
        known.len()
    );
    assert!(
        bad.is_empty(),
        "{} of {cells} (program, flag, question) cells printed something the \
         default does not. No config option may change the answer \
         (configuration.md § 2.5) — this is a register row:\n\n{}",
        bad.len(),
        bad.join("\n\n")
    );
    for (path, flag) in KNOWN {
        match pinned.get(&(path.to_string(), flag)) {
            Some(&(asked, moved)) if asked > 0 && asked == moved => {}
            got => bad.push(format!(
                "{path} with :{flag} flipped is a known disagreement, and it moved on \
                 {got:?} (questions, disagreements): fixed, or no longer swept — \
                 take it out of KNOWN"
            )),
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
    // 180 on 2026-11-01, of 255 files: 12 partial, 63 with no read-out.
    assert!(known.len() >= 180, "{} programs compared", known.len());
}
