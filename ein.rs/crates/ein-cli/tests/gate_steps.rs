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
//! Python, Graphviz) are excluded, and [`NOT_GATE_STEPS`] is where that
//! exclusion is written down: an unmarked `run:` the test does not recognise
//! **fails**, because the failure this file exists to prevent is CI gaining a
//! step the gate does not have.
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
//! the two above. What that test does *not* do is fix the header — the script
//! says the flag skips *the static checks* and it also skips the bench smoke,
//! which is [TE-L3](../../../../plans/m1e_review_processing/p1e.4_low/s1e.4.5_tests.md)'s
//! one-line wording change. The check is here first on purpose: a header
//! nobody verifies is how the divergence arose.

use ein_corpus::repo_root;

/// The workflow's `- run:` lines that are **not** gate steps, with the reason.
///
/// Provisioning: CI has to install what a developer's machine already has.
/// `run_tests.sh`'s counterpart is a `command -v dot` that exits 127 — the
/// same requirement, expressed as a check rather than as an install, which is
/// why it is not a step on either list.
const NOT_GATE_STEPS: &[(&str, &str)] = &[(
    "sudo apt-get update && sudo apt-get install -y graphviz",
    "provisioning: the gate checks for `dot` and exits 127 instead",
)];

/// The gate steps of `run_tests.sh`, in order.
fn shell_steps(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.trim_start().starts_with("step \"") {
            continue;
        }
        // The command is the next statement — one logical line, which may be
        // spread over several physical ones by a trailing backslash.
        let mut j = i + 1;
        let mut cmd = String::new();
        while j < lines.len() {
            let piece = lines[j].trim();
            cmd.push_str(piece.strip_suffix('\\').unwrap_or(piece));
            if !piece.ends_with('\\') {
                break;
            }
            cmd.push(' ');
            j += 1;
        }
        let cmd = cmd
            .trim_start_matches("if ! ")
            .trim_end_matches("; then")
            .replace("--manifest-path \"${MANIFEST}\" ", "")
            .replace(" -q ", " ")
            .replace("${ARGS[@]+\"${ARGS[@]}\"}", "");
        // `"${SCRIPT_DIR}/utils/x.py"` is `utils/x.py` with the quoting a
        // shell needs and a workflow does not. Rewritten per token, so the
        // quotes that are *semantic* — `RUSTDOCFLAGS="-D warnings"` — survive
        // and are still compared.
        out.push(normalise(
            cmd.split_whitespace()
                .map(|t| {
                    t.strip_prefix("\"${SCRIPT_DIR}/")
                        .and_then(|t| t.strip_suffix('"'))
                        .unwrap_or(t)
                })
                .collect::<Vec<_>>()
                .join(" "),
        ));
    }
    out
}

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
            .and_then(|l| l.trim().strip_prefix("env: { "))
            .and_then(|l| l.strip_suffix(" }"))
            .and_then(|kv| kv.split_once(": "))
            .map(|(k, v)| format!("{k}={v} "))
            .unwrap_or_default();
        steps.push(normalise(format!("{env}{cmd}")));
    }
    (steps, all)
}

/// One space between words, nothing at the ends.
fn normalise(cmd: impl AsRef<str>) -> String {
    cmd.as_ref()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The two lists are the same list.
#[test]
fn the_gate_runs_what_ci_runs() {
    let root = repo_root();
    let shell = std::fs::read_to_string(root.join("run_tests.sh")).expect("run_tests.sh");
    let yml = std::fs::read_to_string(root.join(".github/workflows/per-commit.yml"))
        .expect("per-commit.yml");

    let gate = shell_steps(&shell);
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
        "the set `--tests-only` skips moved. Note the sixth: the flag skips the \
         **bench smoke** as well as the five static checks, and `run_tests.sh`'s \
         header says only the first — TE-L3, whose fix is that line and whose \
         check is this one"
    );
}
