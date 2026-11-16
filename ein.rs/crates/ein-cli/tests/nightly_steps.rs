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
//! the steps that left. Its banners are `dev_host_step "…"`, read
//! the same way. [`the_dev_host_runs_the_nightly_and_what_left_it`] holds the
//! two lists together: every nightly step is in the dev host's, in order, the
//! fuzz budget read as the dev host's ([`DEV_HOST_FUZZ_ITERS`]), and every
//! other step is one [`DEV_HOST_ONLY`] names with why. So a step taken out of
//! the nightly and left in `--dev-host` fails until it is named there, and one
//! taken out of all three copies lowers the list under its floor.
//! [`the_fuzzer_runs_at_each_tiers_budget`] pins the two budgets, which the
//! diff reads past, and [`every_slow_half_runs_on_a_tier`] fails on a test
//! target that reads `EIN_CORPUS_SLOW` and that neither list runs with it, as
//! three did until S1f.5.27.
//!
//! ## The shards (M1f S1f.6.39)
//!
//! A nightly step over five
//! minutes at the runner's four threads is split into parallel jobs, each a
//! shard of it, `EIN_TEST_SHARD=k/n` (`ein_corpus::shard`), a job's
//! `matrix: { shard: [...] }` naming its shards. `--nightly` runs a sharded
//! job once per shard, one after another, as GitHub runs its matrix side by
//! side, and `--dev-host` runs every shard too, at its budget.
//! [`every_sharded_step_runs_every_shard`] holds a sharded step to all of
//! its `n` shards, so no program is left in a shard nothing runs.
//!
//! ## What it does not catch
//!
//! The environment a job gives every step beyond its `env:` blocks: the
//! runner's image, `GATE_TMP`'s spelling (each side names its own directory),
//! the `working-directory` (the script runs from the repo root and names
//! `ein.rs/Cargo.toml`). And a flag dropped from both copies at once, which
//! is why [`the_slow_cells_run_under_the_gate_profile_with_the_flag`] pins the
//! one the corpus sweep's timing band needs.

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
/// runner or measures rather than checks. A step that leaves the
/// nightly and stays in `--dev-host` is named here, or
/// [`the_dev_host_runs_the_nightly_and_what_left_it`] fails.
const DEV_HOST_ONLY: &[(&str, &str)] = &[
    (
        "EIN_ID_SEEDS=\"8\" EIN_CORPUS_SLOW=\"1\" TMPDIR=\"${GATE_TMP}\" cargo test -p ein-render --profile gate --test id_order_invariance",
        "left the nightly at S1f.5.27: 452 s on the runner (run 42802597417, \
         2026-11-10); the nightly runs one permutation of every corpus \
         program, in shards, and the per-commit tier one of every program \
         outside tests/ that searches (S1f.6.39)",
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
/// nightly ran until S1f.5.27 and runs at a budget that fits since.
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

/// **The slow cells run at release speed, with `EIN_CORPUS_SLOW`.** The
/// timing half of `corpus_cli::the_slow_flag_still_describes_the_sweep` exists
/// only under that flag, and is calibrated on release codegen, which the
/// gate profile is (`--release` until M1f S1f.6.39, which gave every test
/// step the gate profile: release codegen with the debug checks); if both
/// copies lost the flag together, the diff above would pass and the band
/// would stop being asserted anywhere.
#[test]
fn the_slow_cells_run_under_the_gate_profile_with_the_flag() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    assert!(
        nightly.iter().any(|s| s.contains("EIN_CORPUS_SLOW=\"1\"")
            && s.ends_with("cargo test -p ein-cli --profile gate --test corpus_cli")),
        "no nightly step runs corpus_cli under --profile gate with EIN_CORPUS_SLOW=\"1\": \
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
            && s.ends_with("cargo test -p ein-cli --profile gate --test answer_round_trip")),
        "no nightly step runs answer_round_trip under --profile gate with \
         EIN_CORPUS_SLOW=\"1\": the heavy_dump and slow entries' dumps would be \
         round-tripped by no CI tier: {nightly:#?}"
    );
}

/// **The tests' scratch is checked** (S1e2.3.6, after S1e2.3.5). In every
/// job that runs `check_test_scratch.py`, every `cargo test` runs under
/// `TMPDIR="${GATE_TMP}"`, and the check comes after the last of them; and
/// every job with a `cargo test` under that `TMPDIR` runs the check, so the
/// slow cells and the shards (S1f.6.39), which no per-commit run reaches,
/// leave nothing either.
#[test]
fn every_test_step_runs_under_the_checked_tmpdir() {
    let all = jobs(&read(".github/workflows/nightly.yml"));
    let check = "python3 utils/check_test_scratch.py \"${GATE_TMP}\"";
    let mut checked = 0usize;
    for job in &all {
        let tests: Vec<usize> = (0..job.runs.len())
            .filter(|&i| job.runs[i].contains("cargo test"))
            .collect();
        let Some(at) = job.runs.iter().position(|r| r == check) else {
            assert!(
                tests
                    .iter()
                    .all(|&i| !job.runs[i].contains("TMPDIR=\"${GATE_TMP}\"")),
                "`{}` runs a cargo test under the tests' TMPDIR and never checks it: {:#?}",
                job.name,
                job.runs
            );
            continue;
        };
        checked += 1;
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
            tests.iter().all(|&i| i < at),
            "`{}` checks its TMPDIR before its last cargo test: {:#?}",
            job.name,
            job.runs
        );
    }
    assert!(
        checked > 0,
        "no nightly job runs utils/check_test_scratch.py"
    );
}

/// `step` with its `EIN_TEST_SHARD="k/n" ` taken out, and the shard.
fn unsharded(step: &str) -> (String, Option<(usize, usize)>) {
    let key = "EIN_TEST_SHARD=\"";
    let Some(at) = step.find(key) else {
        return (step.to_string(), None);
    };
    let rest = &step[at + key.len()..];
    let end = rest.find('"').expect("a closing quote");
    let shard = rest[..end]
        .split_once('/')
        .and_then(|(k, n)| Some((k.parse().ok()?, n.parse().ok()?)));
    let shard = shard.unwrap_or_else(|| panic!("`{step}` names no shard k/n"));
    (
        normalise(format!("{}{}", &step[..at], &rest[end + 1..])),
        Some(shard),
    )
}

/// **Every sharded step runs every one of its shards** (M1f S1f.6.39): a
/// step the nightly runs with `EIN_TEST_SHARD="k/n"` is run for
/// each `k` from 1 to `n`, one `n`, once each, so every program its sweep
/// deals is in a shard a job runs. The dev host's list is held to the
/// nightly's by [`the_dev_host_runs_the_nightly_and_what_left_it`].
#[test]
fn every_sharded_step_runs_every_shard() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    let mut shards: std::collections::BTreeMap<String, Vec<(usize, usize)>> = Default::default();
    for s in &nightly {
        if let (cmd, Some(shard)) = unsharded(s) {
            shards.entry(cmd).or_default().push(shard);
        }
    }
    assert!(
        !shards.is_empty(),
        "no nightly step is sharded: the steps split into shards are back in one job each"
    );
    for (cmd, mut got) in shards {
        got.sort();
        let n = got[0].1;
        let want: Vec<(usize, usize)> = (1..=n).map(|k| (k, n)).collect();
        assert_eq!(
            got, want,
            "`{cmd}` is run on the shards {got:?}, where a sharding into {n} runs each \
             of 1/{n} to {n}/{n} once"
        );
    }
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
    // three slow halves it placed.
    assert!(
        dev.len() >= 17,
        "only {} steps parsed out of run_tests.sh's --dev-host: {dev:#?}",
        dev.len()
    );
}

/// **Each tier sets the fuzzer's budget** (M1f S1f.5.27). The diff
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
/// `cargo test` before its last `check_test_scratch.py` runs under
/// `TMPDIR="${GATE_TMP}"` and is followed by a check, nothing after the last
/// one runs under it, and the steps that left the nightly or never ran in it
/// are among the checked ones, so the slow halves only the dev host runs
/// leave nothing either. Several checks since M1f S1f.6.39: one a job, as
/// the nightly's jobs have, the shards' among them.
#[test]
fn every_dev_host_test_step_runs_under_the_checked_tmpdir() {
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    let check = "python3 utils/check_test_scratch.py \"${GATE_TMP}\"";
    let last = dev
        .iter()
        .rposition(|r| r == check)
        .expect("--dev-host runs utils/check_test_scratch.py on the tests' TMPDIR");
    for (i, s) in dev[..last].iter().enumerate() {
        if s.contains("cargo test") {
            assert!(
                s.contains("TMPDIR=\"${GATE_TMP}\" cargo test"),
                "--dev-host runs `{s}` outside the checked TMPDIR"
            );
            assert!(
                dev[i..].iter().any(|r| r == check),
                "--dev-host runs `{s}` with no scratch check after it"
            );
        }
    }
    for s in &dev[last..] {
        assert!(
            !s.contains("TMPDIR=\"${GATE_TMP}\""),
            "--dev-host runs `{s}` under the tests' TMPDIR after it was last checked"
        );
    }
    for (cmd, _) in DEV_HOST_ONLY
        .iter()
        .filter(|(c, _)| c.contains("cargo test"))
    {
        assert!(
            dev[..last].iter().any(|s| *s == normalise(cmd)),
            "`{cmd}` runs on the dev host after its last scratch check, or not at all"
        );
    }
}

/// **A slow half run by name leaves nothing out in silence** (M1f
/// S1f.6.39): a nightly step that names the test it runs, `-- NAME`, names a
/// test of its target, and either that test is the one place the target's
/// code reads `EIN_CORPUS_SLOW` (`region`'s, `route_invariance`'s), or the
/// dev host runs the whole target with the flag, a step [`DEV_HOST_ONLY`]
/// names with why. So a second test that reads the flag runs on some tier.
/// The second way had one instance, `route_invariance`'s 2²⁵ half, until no
/// test ran past the default budget.
#[test]
fn a_slow_half_run_by_name_leaves_nothing_out() {
    let (nightly, _) = workflow_steps(&read(".github/workflows/nightly.yml"));
    let dev = shell_steps(&read("run_tests.sh"), "dev_host_step");
    let mut named = 0usize;
    for s in nightly.iter().filter(|s| s.contains("cargo test")) {
        let Some((head, name)) = s.split_once(" -- ") else {
            continue;
        };
        let (package, target) = head
            .split_once(" -p ")
            .and_then(|(_, r)| r.split_once(" --profile gate --test "))
            .unwrap_or_else(|| panic!("`{s}` names a test but no target"));
        let text = read(&format!("ein.rs/crates/{package}/tests/{target}.rs"));
        assert!(
            text.contains(&format!("fn {name}(")),
            "`{s}`: {target} has no test `{name}`"
        );
        let whole = format!(
            "EIN_CORPUS_SLOW=\"1\" TMPDIR=\"${{GATE_TMP}}\" cargo test -p {package} --profile gate --test {target}"
        );
        if dev.contains(&whole) && DEV_HOST_ONLY.iter().any(|(c, _)| normalise(c) == whole) {
            named += 1;
            continue;
        }
        let code: Vec<(usize, &str)> = text
            .lines()
            .enumerate()
            .filter(|(_, l)| !l.trim_start().starts_with("//"))
            .collect();
        let at = code
            .iter()
            .position(|(_, l)| l.trim_start().starts_with(&format!("fn {name}(")))
            .unwrap_or_else(|| panic!("`{s}`: {target} has no test `{name}`"));
        let next = code[at + 1..]
            .iter()
            .position(|(_, l)| l.starts_with("fn ") || l.starts_with("#[test]"))
            .map_or(code.len(), |k| at + 1 + k);
        let reads: Vec<usize> = (0..code.len())
            .filter(|&i| code[i].1.contains("\"EIN_CORPUS_SLOW\""))
            .collect();
        assert!(
            !reads.is_empty() && reads.iter().all(|&i| at < i && i < next),
            "`{s}` runs `{name}` alone, and {target} reads EIN_CORPUS_SLOW outside it \
             (lines {:?}): its slow half is more than the one test, and the dev host \
             does not run the target whole with the flag",
            reads.iter().map(|&i| code[i].0 + 1).collect::<Vec<_>>()
        );
        named += 1;
    }
    assert!(
        named > 0,
        "no nightly step names its test: region's slow half is by name"
    );
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

/// **Every slow half runs on a tier** (M1f S1f.5.27). A test
/// target that reads `EIN_CORPUS_SLOW` has a step in the nightly or on the dev
/// host that runs it under `--profile gate` with the flag, or is named in
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
                let page = read("docs/ein/user/reference/read-out-examples.md");
                assert!(
                    !page.contains("33554432"),
                    "NO_SLOW_TIER excuses `{target}` ({why}), and the page has a \
                     block at --region-budget 33554432 now: give the target a step"
                );
            }
            continue;
        }
        // The step may name the tests it runs, `-- NAME` (`region`'s, since
        // S1f.6.39: its one test that reads the flag).
        let tail = format!("cargo test -p {package} --profile gate --test {target}");
        let runs = |steps: &[String]| {
            steps.iter().any(|s| {
                s.contains("EIN_CORPUS_SLOW=\"1\"")
                    && (s.ends_with(&tail) || s.contains(&format!("{tail} -- ")))
            })
        };
        assert!(
            runs(&nightly) || runs(&dev),
            "{package}'s `{target}` reads EIN_CORPUS_SLOW, and neither the nightly \
             nor --dev-host runs it under --profile gate with the flag, so its slow \
             half runs on no tier"
        );
    }
    for (target, why) in NO_SLOW_TIER {
        assert!(
            readers.iter().any(|(_, t)| t == target),
            "NO_SLOW_TIER excuses `{target}` ({why}), which reads no EIN_CORPUS_SLOW"
        );
    }
}
