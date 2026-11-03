//! `./run_tests.sh --nightly` runs what the nightly runs — M1e2 S1e2.3.6.
//!
//! `gate_steps.rs`' sibling, for the other workflow. Until
//! S1e2.3.6 nothing local ran `.github/workflows/nightly.yml`'s steps, and the
//! nightly failed on every run from 2026-10-17 to 2026-11-03 while the
//! per-commit gate here stayed green: it reports to GitHub and to nothing in
//! this tree. `--nightly` is the local copy of its step list, and this file is
//! what keeps the copy a copy.
//!
//! ## What is compared
//!
//! The **commands**, in order, after the normalisation `gate_steps.rs` uses
//! (`steps/mod.rs`): every `nightly_step "…"` banner's statement in
//! `run_tests.sh`, against every `- run:` of `nightly.yml`, job by job in the
//! file's order, with a step's `env: { K: "V", … }` folded back into a
//! `K="V" … ` prefix. A job with a `matrix:` runs every step once per value,
//! `${{ matrix.KEY }}` replaced, as GitHub runs it; so the fuzz job is two
//! steps here, one per seed.
//!
//! **There is no marker.** `per-commit.yml` marks its gate steps `# gate-step`
//! and excludes the rest by name; here every `run:` is a step unless
//! [`NOT_NIGHTLY_STEPS`] names it with a reason, so a step added to the
//! workflow and not to the script fails [`the_nightly_runs_what_the_workflow_runs`]
//! with no marker to forget. A `run: |` block is refused: a step is one line
//! here, so the shell's copy of it is one statement.
//!
//! ## What it does not catch
//!
//! The environment a job gives every step beyond its `env:` blocks: the
//! runner's image, `GATE_TMP`'s spelling (each side names its own directory),
//! the `working-directory` (the script runs from the repo root and names
//! `ein.rs/Cargo.toml`). And a flag dropped from both copies at once, which
//! is why [`the_slow_cells_run_under_release_with_the_flag`] pins the one the
//! corpus sweep's timing band needs.

use ein_corpus::repo_root;

#[allow(dead_code)]
mod steps;
use steps::{env_prefix, normalise, shell_steps};

/// The workflow's `- run:` lines that are **not** steps of `--nightly`, with
/// the reason. The same two as `gate_steps.rs`' `NOT_GATE_STEPS`, for the same
/// reasons.
const NOT_NIGHTLY_STEPS: &[(&str, &str)] = &[
    (
        "sudo apt-get update && sudo apt-get install -y graphviz",
        "provisioning: run_tests.sh checks for `dot` and exits 127 instead",
    ),
    (
        "mkdir -p \"${GATE_TMP}\"",
        "provisioning: the tests' TMPDIR, which --nightly makes itself before \
         its first step",
    ),
];

/// One job of the workflow: its name, its matrix, and its `run:` commands
/// with their `env:` folded in, in order.
struct Job {
    name: String,
    matrix: Option<(String, Vec<String>)>,
    runs: Vec<String>,
}

/// The jobs of a workflow, in order. Enough YAML for this file's shapes and
/// no more: a job is a two-space key under `jobs:`, a step's `env:` and
/// `matrix:` are flow maps on one line, and anything else fails loudly.
fn jobs(text: &str) -> Vec<Job> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
        .iter()
        .position(|l| *l == "jobs:")
        .expect("the workflow has a `jobs:` key");
    let mut out: Vec<Job> = Vec::new();
    for (i, line) in lines.iter().enumerate().skip(start + 1) {
        let indent = line.len() - line.trim_start().len();
        let t = line.trim();
        if indent == 2 && t.ends_with(':') && !t.starts_with('#') {
            out.push(Job {
                name: t.trim_end_matches(':').to_string(),
                matrix: None,
                runs: Vec::new(),
            });
            continue;
        }
        let Some(job) = out.last_mut() else { continue };
        if let Some(m) = t.strip_prefix("matrix: { ") {
            let (key, list) = m
                .strip_suffix(" }")
                .and_then(|m| m.split_once(": "))
                .expect("a matrix is a one-line flow map of one key");
            let values = list
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|v| v.trim().trim_matches('"').to_string())
                .collect();
            job.matrix = Some((key.to_string(), values));
            continue;
        }
        let Some(cmd) = t.strip_prefix("- run: ") else {
            continue;
        };
        assert!(
            !cmd.starts_with('|') && !cmd.starts_with('>'),
            "nightly.yml's job `{}` has a multi-line `run:` at line {}: a step \
             here is one command, so that `run_tests.sh --nightly` can run it \
             as one statement",
            job.name,
            i + 1
        );
        // The step's own keys, on the lines after `run:` that are indented
        // past its `- `: `env:` is the one that changes the command.
        let env = lines[i + 1..]
            .iter()
            .take_while(|l| {
                let t = l.trim();
                !t.is_empty()
                    && !t.starts_with('#')
                    && !t.starts_with("- ")
                    && l.len() - l.trim_start().len() > indent
            })
            .copied()
            .find_map(env_prefix)
            .unwrap_or_default();
        job.runs.push(normalise(format!("{env}{cmd}")));
    }
    out
}

/// Every `run:` the workflow executes, in order, a matrix job expanded once
/// per value; and, beside it, every `run:` as written.
fn workflow_steps(text: &str) -> (Vec<String>, Vec<String>) {
    let (mut steps, mut all) = (Vec::new(), Vec::new());
    for job in jobs(text) {
        all.extend(job.runs.iter().cloned());
        let runs: Vec<&String> = job
            .runs
            .iter()
            .filter(|r| !NOT_NIGHTLY_STEPS.iter().any(|(c, _)| normalise(c) == **r))
            .collect();
        match &job.matrix {
            None => steps.extend(runs.into_iter().cloned()),
            Some((key, values)) => {
                for v in values {
                    for r in &runs {
                        steps.push(r.replace(&format!("${{{{ matrix.{key} }}}}"), v));
                    }
                }
            }
        }
    }
    (steps, all)
}

fn read(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The two lists are the same list.
#[test]
fn the_nightly_runs_what_the_workflow_runs() {
    let local = shell_steps(&read("run_tests.sh"), "nightly_step");
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    assert_eq!(
        local, nightly,
        "`run_tests.sh --nightly` and nightly.yml no longer run the same steps \
         in the same order.\n  --nightly: {local:#?}\n  nightly.yml: {nightly:#?}"
    );
    // Not vacuous: twelve at S1e2.3.6 (nine in `deep-corpus`, two fuzz
    // seeds, one bench), and a parse that matched nothing against nothing
    // would pass the assertion above.
    assert!(
        local.len() >= 12,
        "only {} steps parsed out of run_tests.sh — the parse, not the \
         nightly, is what changed: {local:#?}",
        local.len()
    );
    assert!(
        nightly.iter().all(|s| !s.contains("${{")),
        "a `${{{{ … }}}}` expression is left in a step after the matrix was \
         expanded, so the shell cannot spell it: {nightly:#?}"
    );
}

/// Every exception names a `run:` the workflow has, so the list cannot keep a
/// reason for a step that is gone.
#[test]
fn every_exception_is_a_step_the_workflow_has() {
    let (_, all) = workflow_steps(&read(".github/workflows/nightly.yml"));
    for (cmd, why) in NOT_NIGHTLY_STEPS {
        assert!(
            all.iter().any(|r| *r == normalise(cmd)),
            "NOT_NIGHTLY_STEPS excuses `{cmd}` ({why}), and nightly.yml runs no \
             such step: {all:#?}"
        );
    }
}

/// **The slow cells run under `--release`, with `EIN_CORPUS_SLOW`.** The
/// timing half of `corpus_cli::the_slow_flag_still_describes_the_sweep` exists
/// only under that flag, and is calibrated on that profile; if both copies
/// lost the flag together, the diff above would pass and the band would stop
/// being asserted anywhere.
#[test]
fn the_slow_cells_run_under_release_with_the_flag() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    assert!(
        nightly.iter().any(|s| s.contains("EIN_CORPUS_SLOW=\"1\"")
            && s.ends_with("cargo test -p ein-cli --release --test corpus_cli")),
        "no nightly step runs corpus_cli under --release with EIN_CORPUS_SLOW=\"1\": \
         {nightly:#?}"
    );
}

/// **The tests' scratch is checked** (S1e2.3.6, after S1e2.3.5). Every
/// `cargo test` of the job that runs `check_test_scratch.py` runs under
/// `TMPDIR="${GATE_TMP}"`, and the check comes after the last of them; so the
/// slow cells, which no per-commit run reaches, leave nothing either.
#[test]
fn every_test_step_runs_under_the_checked_tmpdir() {
    let all = jobs(&read(".github/workflows/nightly.yml"));
    let job = all
        .iter()
        .find(|j| j.runs.iter().any(|r| r.contains("check_test_scratch.py")))
        .expect("a nightly job runs utils/check_test_scratch.py");
    let check = job
        .runs
        .iter()
        .position(|r| r == "python3 utils/check_test_scratch.py \"${GATE_TMP}\"")
        .expect("the check reads the tests' TMPDIR");
    let tests: Vec<usize> = (0..job.runs.len())
        .filter(|&i| job.runs[i].contains("cargo test"))
        .collect();
    assert!(!tests.is_empty(), "`{}` runs no cargo test", job.name);
    for &i in &tests {
        assert!(
            job.runs[i].contains("TMPDIR=\"${GATE_TMP}\" cargo test"),
            "`{}` runs `{}` outside the checked TMPDIR",
            job.name,
            job.runs[i]
        );
    }
    assert!(
        tests.iter().all(|&i| i < check),
        "`{}` checks its TMPDIR before its last cargo test: {:#?}",
        job.name,
        job.runs
    );
}
