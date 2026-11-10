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
//! ## The dev host's list (M1f S1f.5.27)
//!
//! The user's ruling of 2026-11-10, *"reduce nightly job, remove long running
//! tests, we will eventually run them on dev host, retain critical tests in
//! CI"*: the nightly keeps the steps that guard an answer or a contract no
//! per-commit step guards, each at a budget that fits in 5 min on the runner,
//! and `./run_tests.sh --dev-host` runs every one of them at full budget and
//! the steps that left (Q-M1f.102). Its banners are `dev_host_step "…"`, read
//! the same way. [`the_dev_host_runs_the_nightly_and_what_left_it`] holds the
//! two lists together: every nightly step is in the dev host's, in order, the
//! fuzz budget read as the dev host's ([`DEV_HOST_FUZZ_ITERS`]), and every
//! other step is one [`DEV_HOST_ONLY`] names with why. So a step taken out of
//! the nightly and left in `--dev-host` fails until it is named there, and one
//! taken out of all three copies lowers the list under its floor.
//! [`the_fuzzer_runs_at_each_tiers_budget`] pins the two budgets, which the
//! diff reads past, and [`every_slow_half_runs_on_a_tier`] fails on a test
//! target that reads `EIN_CORPUS_SLOW` and that neither list runs with it, as
//! three did until S1f.5.27 (Q-M1f.51).
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
        "sudo apt-get update && sudo apt-get install -y graphviz libgvplugin-neato-layout8",
        "provisioning: run_tests.sh checks that `dot` and `fdp` lay a graph out, \
         and exits 127 instead",
    ),
    (
        "mkdir -p \"${GATE_TMP}\"",
        "provisioning: the tests' TMPDIR, which --nightly makes itself before \
         its first step",
    ),
];

/// The dev host's steps that the nightly does not run, with why: each left the
/// nightly at M1f S1f.5.27, or never ran in it, and is over 5 min on the
/// runner or measures rather than checks (Q-M1f.102). A step that leaves the
/// nightly and stays in `--dev-host` is named here, or
/// [`the_dev_host_runs_the_nightly_and_what_left_it`] fails.
const DEV_HOST_ONLY: &[(&str, &str)] = &[
    (
        "EIN_ID_SEEDS=\"8\" TMPDIR=\"${GATE_TMP}\" cargo test -p ein-render --release --test id_order_invariance",
        "left the nightly at S1f.5.27: 452 s on the runner (run 42802597417, \
         2026-11-10), and the per-commit tier runs one permutation of every \
         corpus file",
    ),
    (
        "EIN_CORPUS_SLOW=\"1\" TMPDIR=\"${GATE_TMP}\" cargo test -p ein-cli --release --test route_invariance",
        "no tier ran it until S1f.5.27 (Q-M1f.51): 857 s here at \
         S1f.5.13, a second route held to answers `corpus_shapes` digests per \
         commit",
    ),
    (
        "python3 utils/stdlib_census.py --check",
        "left the nightly at S1f.5.27: a measurement, 380 s on the runner; \
         `stdlib_coverage` is its per-commit gate",
    ),
    (
        "python3 utils/review_bundle.py",
        "left the nightly at S1f.5.27: 372 s on the runner; the per-commit \
         `--smoke` reads every register row and builds four cases",
    ),
    (
        "python3 utils/doc_audit.py --links --check build/review",
        "left the nightly at S1f.5.27 with the bundle it reads",
    ),
];

/// The fuzzer's budget on the dev host: P1a.1's acceptance number, which the
/// nightly ran until S1f.5.27 and runs at a budget that fits since
/// (Q-M1f.103).
const DEV_HOST_FUZZ_ITERS: u64 = 1_000_000;

/// The per-commit tier's budget, `EIN_FUZZ_ITERS`' default in
/// `ein-ir/tests/fuzz_properties.rs`.
const PER_COMMIT_FUZZ_ITERS: u64 = 2_000;

/// `step` with its `EIN_FUZZ_ITERS="…"`, if it sets one, read as `iters`.
fn at_budget(step: &str, iters: u64) -> String {
    let key = "EIN_FUZZ_ITERS=\"";
    let Some(at) = step.find(key) else {
        return step.to_string();
    };
    let rest = &step[at + key.len()..];
    let end = rest.find('"').expect("a closing quote");
    format!("{}{key}{iters}{}", &step[..at], &rest[end..])
}

/// The budget a step sets, `EIN_FUZZ_ITERS="N"`'s `N`.
fn budget_of(step: &str) -> Option<u64> {
    let rest = step.split_once("EIN_FUZZ_ITERS=\"")?.1;
    rest.split('"').next()?.parse().ok()
}

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
    // seeds, one bench), fourteen at S1f.5.13, and twelve again at S1f.5.27,
    // which moved four steps to the dev host and added two; a parse that
    // matched nothing against nothing would pass the assertion above.
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

/// **The round trip's slow half runs in the nightly** (M1f S1f.5.13, the
/// user's ruling of 2026-11-08 on P-16). Per commit `answer_round_trip` leaves
/// out the `slow` and `heavy_dump` entries, and only `EIN_CORPUS_SLOW` brings
/// them back, so a step that lost the flag in both copies would round-trip
/// them nowhere, as no tier did before the stage, and the diff above would
/// pass.
#[test]
fn the_round_trips_slow_half_runs_with_the_flag() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    assert!(
        nightly.iter().any(|s| s.contains("EIN_CORPUS_SLOW=\"1\"")
            && s.ends_with("cargo test -p ein-cli --release --test answer_round_trip")),
        "no nightly step runs answer_round_trip under --release with \
         EIN_CORPUS_SLOW=\"1\": the heavy_dump and slow entries' dumps would be \
         round-tripped by no CI tier: {nightly:#?}"
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

/// **The dev host runs the nightly, and every step that left it** (M1f
/// S1f.5.27). Every step of the nightly, matrix expanded, is a step of
/// `--dev-host`, in the nightly's order and at the dev host's fuzz budget; the
/// others are exactly [`DEV_HOST_ONLY`], in its order.
#[test]
fn the_dev_host_runs_the_nightly_and_what_left_it() {
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    let want: Vec<String> = nightly
        .iter()
        .map(|s| at_budget(s, DEV_HOST_FUZZ_ITERS))
        .collect();
    let (mut k, mut extras) = (0usize, Vec::new());
    for s in &dev {
        if want.get(k) == Some(s) {
            k += 1;
        } else {
            extras.push(s.clone());
        }
    }
    assert!(
        k == want.len(),
        "`run_tests.sh --dev-host` does not run the nightly's step `{}`, in the \
         nightly's order and at the dev host's budget: the dev host runs every \
         nightly step.\n  --dev-host: {dev:#?}\n  nightly.yml: {want:#?}",
        want[k]
    );
    let named: Vec<String> = DEV_HOST_ONLY.iter().map(|(c, _)| normalise(c)).collect();
    assert_eq!(
        extras, named,
        "the dev host's steps that the nightly does not run are not the ones \
         DEV_HOST_ONLY names with why, in its order: a step that left the \
         nightly is named there, and a name with no step goes"
    );
    // A floor, so a step taken out of all three copies is seen: seventeen at
    // S1f.5.27, the nightly's fourteen of that day at their budgets and the
    // three slow halves Q-M1f.51 placed.
    assert!(
        dev.len() >= 17,
        "only {} steps parsed out of run_tests.sh's --dev-host: {dev:#?}",
        dev.len()
    );
}

/// **Each tier sets the fuzzer's budget** (M1f S1f.5.27, Q-M1f.103). The diff
/// above reads past the budget, so it is pinned here: every fuzz step of the
/// nightly sets one above the per-commit default and below the dev host's, and
/// every fuzz step of the dev host sets P1a.1's million. A budget dropped from
/// a step would run it at the per-commit tier's 2 000 with every list still
/// agreeing.
#[test]
fn the_fuzzer_runs_at_each_tiers_budget() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    for (tier, steps) in [("nightly.yml", &nightly), ("--dev-host", &dev)] {
        let fuzz: Vec<&String> = steps
            .iter()
            .filter(|s| s.contains("--test fuzz_properties"))
            .collect();
        assert_eq!(
            fuzz.len(),
            2,
            "{tier} runs the fuzzer once per seed: {fuzz:#?}"
        );
        for s in fuzz {
            let n = budget_of(s).unwrap_or_else(|| {
                panic!("{tier}'s `{s}` sets no EIN_FUZZ_ITERS, so it runs at the per-commit 2 000")
            });
            if tier == "--dev-host" {
                assert_eq!(n, DEV_HOST_FUZZ_ITERS, "{tier}'s `{s}`");
            } else {
                assert!(
                    n > PER_COMMIT_FUZZ_ITERS && n < DEV_HOST_FUZZ_ITERS,
                    "{tier}'s `{s}`: the nightly's budget is above the per-commit \
                     tier's and below the dev host's"
                );
            }
        }
    }
}

/// **The dev host's tests' scratch is checked**, as the nightly's is: every
/// `cargo test` before its `check_test_scratch.py` runs under
/// `TMPDIR="${GATE_TMP}"`, and the steps that left the nightly or never ran in
/// it are among them, so the slow halves only the dev host runs leave nothing
/// either.
#[test]
fn every_dev_host_test_step_runs_under_the_checked_tmpdir() {
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    let check = dev
        .iter()
        .position(|r| r == "python3 utils/check_test_scratch.py \"${GATE_TMP}\"")
        .expect("--dev-host runs utils/check_test_scratch.py on the tests' TMPDIR");
    for s in &dev[..check] {
        if s.contains("cargo test") {
            assert!(
                s.contains("TMPDIR=\"${GATE_TMP}\" cargo test"),
                "--dev-host runs `{s}` outside the checked TMPDIR"
            );
        }
    }
    for s in &dev[check..] {
        assert!(
            !s.contains("TMPDIR=\"${GATE_TMP}\""),
            "--dev-host runs `{s}` under the tests' TMPDIR after it was checked"
        );
    }
    for (cmd, _) in DEV_HOST_ONLY
        .iter()
        .filter(|(c, _)| c.contains("cargo test"))
    {
        assert!(
            dev[..check].iter().any(|s| *s == normalise(cmd)),
            "`{cmd}` runs on the dev host after its scratch check, or not at all"
        );
    }
}

/// Test targets that read `EIN_CORPUS_SLOW` and whose slow half runs on no
/// tier, with why, each held to its premise by
/// [`every_slow_half_runs_on_a_tier`].
const NO_SLOW_TIER: &[(&str, &str)] = &[(
    "read_out_examples",
    "its slow branch adds only the read-out reference's blocks at \
     `--region-budget 33554432`, and the page has none; a block that adds one \
     needs a step",
)];

/// Every `tests/*.rs` target of the workspace whose code, comments aside,
/// reads `EIN_CORPUS_SLOW`, as `(package, target)`.
fn slow_readers() -> Vec<(String, String)> {
    let crates = repo_root().join("ein.rs/crates");
    let mut out = Vec::new();
    for c in std::fs::read_dir(&crates).expect("ein.rs/crates").flatten() {
        let Ok(files) = std::fs::read_dir(c.path().join("tests")) else {
            continue;
        };
        for f in files.flatten() {
            let path = f.path();
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            let reads = text
                .lines()
                .filter(|l| !l.trim_start().starts_with("//"))
                .any(|l| l.contains("\"EIN_CORPUS_SLOW\""));
            if reads {
                let package = c.file_name().to_string_lossy().to_string();
                let target = path.file_stem().unwrap().to_string_lossy().to_string();
                out.push((package, target));
            }
        }
    }
    out.sort();
    out
}

/// **Every slow half runs on a tier** (M1f S1f.5.27, Q-M1f.51's (b)). A test
/// target that reads `EIN_CORPUS_SLOW` has a step in the nightly or on the dev
/// host that runs it under `--release` with the flag, or is named in
/// [`NO_SLOW_TIER`] with a premise this test checks. Three targets' slow
/// halves ran on no tier until S1f.5.27, `route_invariance`, `region` and
/// `obligation_rung`, and only S1f.5.13's re-take noticed; a fourth would fail
/// here.
#[test]
fn every_slow_half_runs_on_a_tier() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    let readers = slow_readers();
    // Not vacuous: eight at S1f.5.27 (`grep -rln '"EIN_CORPUS_SLOW"'
    // ein.rs/crates/*/tests`, less the files that only name it in a comment).
    assert!(
        readers.len() >= 8,
        "only {} test targets read EIN_CORPUS_SLOW: {readers:#?}",
        readers.len()
    );
    for (package, target) in &readers {
        if let Some((_, why)) = NO_SLOW_TIER.iter().find(|(t, _)| t == target) {
            if target == "read_out_examples" {
                let page = read("docs/kernel/inference/read_out_examples.md");
                assert!(
                    !page.contains("33554432"),
                    "NO_SLOW_TIER excuses `{target}` ({why}), and the page has a \
                     block at --region-budget 33554432 now: give the target a step"
                );
            }
            continue;
        }
        let tail = format!("cargo test -p {package} --release --test {target}");
        let runs = |steps: &[String]| {
            steps
                .iter()
                .any(|s| s.contains("EIN_CORPUS_SLOW=\"1\"") && s.ends_with(&tail))
        };
        assert!(
            runs(&nightly) || runs(&dev),
            "{package}'s `{target}` reads EIN_CORPUS_SLOW, and neither the nightly \
             nor --dev-host runs it under --release with the flag, so its slow \
             half runs on no tier (Q-M1f.51)"
        );
    }
    for (target, why) in NO_SLOW_TIER {
        assert!(
            readers.iter().any(|(_, t)| t == target),
            "NO_SLOW_TIER excuses `{target}` ({why}), which reads no EIN_CORPUS_SLOW"
        );
    }
}
