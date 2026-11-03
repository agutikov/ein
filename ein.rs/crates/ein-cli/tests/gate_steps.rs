//! `./run_tests.sh` runs what CI runs — as a **test**, not as a convention.
//!
//! M1e `AR-M1`'s fourth pair, and the one that has already cost something:
//! `run_tests.sh` and `.github/workflows/per-commit.yml` are two hand-written
//! copies of one list, and M1c S1c.1.5 found them apart after **three red
//! commits** — clippy and the two Python greps ran only in CI, so the local
//! script reported a pass over findings it could not see. Both files carry
//! that story in their own headers, and *"keep the two lists the same"* was
//! the whole of the mechanism keeping them the same.
//!
//! ## What is compared, and what is not
//!
//! The **commands**, in order, after a stated normalisation — not labels, not
//! step names. Nothing here is a third copy of the list: each side's entry is
//! read out of the file that runs it.
//!
//! | side | what a step is | normalisation |
//! |---|---|---|
//! | `run_tests.sh` | the statement following a `step "…"` banner — that function exists to announce exactly these | strip `if ! ` … `; then`, join `\` continuations, drop `"${SCRIPT_DIR}/"`, `--manifest-path "${MANIFEST}"`, `-q`, `${ARGS[@]…}` |
//! | `per-commit.yml` | a `- run:` marked `# gate-step` | fold a following `env: { K: "V" }` back into a `K="V" ` prefix |
//!
//! The marker on the workflow side carries no content — it says *this is a
//! gate step*, not *this is which one* — so it cannot go stale the way a
//! duplicated command would. Provisioning steps (checkout, toolchain, cache,
//! Python, Graphviz, the tests' `TMPDIR`) are excluded, and [`NOT_GATE_STEPS`]
//! is where that exclusion is written down: an unmarked `run:` the test does
//! not recognise **fails**, because the failure this file exists to prevent is
//! CI gaining a step the gate does not have.
//!
//! **Its sibling is `nightly_steps.rs`** (M1e2 S1e2.3.6), which holds
//! `run_tests.sh --nightly` to `nightly.yml` the same way, with the shell parse
//! and the normalisation shared from `steps/mod.rs`. The nightly's banners are
//! `nightly_step "…"`, which this file's parse does not read.
//!
//! ## What it does not catch
//!
//! A step whose *flags* drift within one command is caught; a step that
//! differs only in the environment it runs under is not, beyond the one `env:`
//! block the workflow uses.
//!
//! `--tests-only` **is** in scope since M1e S1e.3.6 T8, which asked for it:
//! [`what_tests_only_skips_is_what_the_script_guards`] reads the set out of the
//! script rather than restating it, so the flag's own list cannot drift from
//! the two above. Since M1e S1e.4.5
//! ([TE-L3](../../../../docs/history/m1e_review_processing/README.md#s1e45--tests-low))
//! it holds the script's **header** to that set as well — the count and the
//! bench smoke — because the header is the copy that drifts, and it drifted
//! while this test watched: T1e.3.8.4 corrected the paragraph at
//! `run_tests.sh:91` when the link check joined the gate and left the usage
//! line at `:5` saying *five*, so one header said five and the other six about
//! the same guard.

use ein_corpus::repo_root;

// The shell parse and the normalisation, shared with `nightly_steps.rs`
// (M1e2 S1e2.3.6), so the two diffs read `run_tests.sh` the same way.
#[allow(dead_code)]
mod steps;
use steps::{env_prefix, normalise, shell_steps};

/// The workflow's `- run:` lines that are **not** gate steps, with the reason.
///
/// Provisioning: CI has to install what a developer's machine already has.
/// `run_tests.sh`'s counterpart is a `command -v dot` that exits 127 — the
/// same requirement, expressed as a check rather than as an install, which is
/// why it is not a step on either list.
const NOT_GATE_STEPS: &[(&str, &str)] = &[
    (
        "sudo apt-get update && sudo apt-get install -y graphviz",
        "provisioning: the gate checks for `dot` and exits 127 instead",
    ),
    (
        "mkdir -p \"${GATE_TMP}\"",
        "provisioning: the tests' TMPDIR (M1e2 S1e2.3.5), which the gate makes \
         itself just before its test step",
    ),
];

/// The gate steps of `per-commit.yml`, in order, plus every `run:` it saw.
fn workflow_steps(text: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<&str> = text.lines().collect();
    let (mut steps, mut all) = (Vec::new(), Vec::new());
    for (i, line) in lines.iter().enumerate() {
        let Some(cmd) = line.trim().strip_prefix("- run: ") else {
            continue;
        };
        all.push(cmd.trim().to_string());
        if lines[..i]
            .iter()
            .rev()
            .find(|l| !l.trim().is_empty())
            .map(|l| l.trim() != "# gate-step")
            .unwrap_or(true)
        {
            continue;
        }
        // `env: { RUSTDOCFLAGS: "-D warnings" }` on the following line is the
        // shell's `RUSTDOCFLAGS="-D warnings" cargo …` prefix in YAML.
        let env = lines
            .get(i + 1)
            .copied()
            .and_then(env_prefix)
            .unwrap_or_default();
        steps.push(normalise(format!("{env}{cmd}")));
    }
    (steps, all)
}

/// The two lists are the same list.
#[test]
fn the_gate_runs_what_ci_runs() {
    let root = repo_root();
    let shell = std::fs::read_to_string(root.join("run_tests.sh")).expect("run_tests.sh");
    let yml = std::fs::read_to_string(root.join(".github/workflows/per-commit.yml"))
        .expect("per-commit.yml");

    let gate = shell_steps(&shell, "step");
    let (ci, _) = workflow_steps(&yml);

    assert_eq!(
        gate, ci,
        "run_tests.sh and per-commit.yml no longer run the same steps in the \
         same order.\n  gate: {gate:#?}\n  ci:   {ci:#?}"
    );
    // Not vacuous: seven steps at S1e.3.4, and a parse that silently matched
    // nothing against nothing would pass the assertion above.
    assert!(
        gate.len() >= 7,
        "only {} steps parsed out of run_tests.sh — the parse, not the gate, \
         is what changed: {gate:#?}",
        gate.len()
    );
}

/// Every `run:` in the workflow is a gate step or a named exception.
///
/// The direction that matters. A step added to CI and not to the gate is the
/// failure of M1c S1c.1.5; if it is unmarked *and* unlisted here, this fails
/// and the author has to say which it is.
#[test]
fn every_ci_command_is_a_gate_step_or_says_why_not() {
    let root = repo_root();
    let yml = std::fs::read_to_string(root.join(".github/workflows/per-commit.yml"))
        .expect("per-commit.yml");
    let (steps, all) = workflow_steps(&yml);

    for cmd in &all {
        let normal = normalise(cmd);
        if steps.iter().any(|s| s.ends_with(&normal) || *s == normal) {
            continue;
        }
        assert!(
            NOT_GATE_STEPS.iter().any(|(c, _)| normalise(c) == normal),
            "per-commit.yml runs `{cmd}`, which is neither marked `# gate-step` \
             nor listed in NOT_GATE_STEPS with a reason — so either \
             ./run_tests.sh does not run it, or nothing says why it should not"
        );
    }
    assert_eq!(
        all.len(),
        steps.len() + NOT_GATE_STEPS.len(),
        "the workflow's `run:` lines are {} and the two lists account for {}",
        all.len(),
        steps.len() + NOT_GATE_STEPS.len()
    );
}

/// **What `--tests-only` skips is what the script guards** — M1e S1e.3.6 T8.
///
/// `TE-M8` is about two *lists* — the gate's and CI's — and `TE-L3` is the
/// same divergence one flag down: `--tests-only` sets `CHECKS=0`, and every
/// step inside an `if [[ "${CHECKS}" == 1 ]]` block is skipped with it. The
/// script's header says the flag skips "the static checks"; the **bench
/// smoke** is inside such a block too.
///
/// Read from the script rather than restated, for this file's own reason: a
/// second copy of a list is the thing that drifts. What is asserted is the
/// *set*, so a step that gains or loses the guard fails here — including, when
/// `TE-L3` lands, the header line that has to be corrected with it.
#[test]
fn what_tests_only_skips_is_what_the_script_guards() {
    let text = std::fs::read_to_string(repo_root().join("run_tests.sh")).expect("run_tests.sh");
    let mut guarded: Vec<String> = Vec::new();
    let mut depth = 0usize;
    // The depth the `CHECKS` guard opened at — the first block has nested
    // `if`s in it, so "closed by the next `fi`" is the wrong rule and was the
    // first thing this test got wrong.
    let mut checks_at: Option<usize> = None;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("if ") {
            depth += 1;
            if checks_at.is_none() && t.contains("${CHECKS}") && t.contains("== 1") {
                checks_at = Some(depth);
            }
        } else if t == "fi" {
            if checks_at == Some(depth) {
                checks_at = None;
            }
            depth = depth.saturating_sub(1);
        } else if checks_at.is_some()
            && let Some(rest) = t.strip_prefix("step \"")
            && let Some(name) = rest.strip_suffix('"')
        {
            guarded.push(name.to_string());
        }
    }
    assert!(
        depth == 0,
        "the shell parse is unbalanced — {depth} unclosed `if`"
    );
    assert_eq!(
        guarded,
        [
            "utils/stdlib_manifest.py",
            "utils/check_hashmap_iteration.py",
            "utils/doc_audit.py --links --check",
            "cargo fmt --all --check",
            "cargo clippy --workspace --all-targets -D warnings",
            "cargo doc --no-deps, -D warnings",
            "cargo bench --bench engine -- --test",
        ],
        "the set `--tests-only` skips moved. Note the last one: the flag skips \
         the **bench smoke** as well as the six static checks, which is what \
         `run_tests.sh`'s header says since M1e S1e.4.5 (`TE-L3`) and what the \
         assertions below hold it to"
    );

    // **The header is the other copy of this set**, and it is the copy that
    // drifts — see the module doc. Both assertions read the parse above rather
    // than a literal, so this file cannot restate the count wrongly either.
    const WORDS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let statics = guarded.len() - 1; // every guarded step but the bench smoke
    let header: String = text
        .lines()
        .take_while(|l| l.is_empty() || l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let usage = header
        .lines()
        .find(|l| l.contains("--tests-only"))
        .expect("run_tests.sh's header lists `--tests-only`");
    assert!(
        usage.contains("bench"),
        "the header's `--tests-only` line reads `{usage}` — the flag skips the \
         bench smoke too, which is the {}th of the {statics} + 1 steps this test \
         just read out of the guard (TE-L3)",
        guarded.len()
    );
    let word = WORDS[statics];
    assert!(
        header.contains(&format!("{word} static check")),
        "the header does not say `{word} static check…` anywhere, and the guard \
         holds {statics} of them plus the bench smoke. A count in this header is \
         the one copy of this set that no other test reads (TE-L3)"
    );
}
