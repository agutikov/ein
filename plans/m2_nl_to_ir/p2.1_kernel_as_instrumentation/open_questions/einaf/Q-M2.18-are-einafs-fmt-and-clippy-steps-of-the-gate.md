# Q-M2.18 — Are EinAf's fmt and clippy steps of the gate?

**Status:** open

**Raised 2026-11-25 by [S2.1.1](../../s2.1.1_tree_and_outcomes.md#defaults-taken), its default 3**, and the first of
the user's choices in [the phase report's § 7](../../report/README.md#7-the-defaults-taken-for-the-user-to-correct).

**The question.** Does the gate run `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` on EinAf's
package, as static steps beside the engine's, or does a stage run them by hand?

**The default taken**, in S2.1.1's words: *"One gate step, EinAf's tests, as the stage says: after the engine's tests,
under the gate's `TMPDIR`, before the scratch check, so the check reads what both left; in CI the same command from the
repository's root. No `cargo fmt` or `cargo clippy -D warnings` step for EinAf: both ran clean by hand (and `cargo doc
-D warnings`); two more static checks would move the count of static checks the gate's header,
`check_gate_steps.py` and `AGENTS.md` state, and the stage named one step. **Proposed for the user**: EinAf's fmt and
clippy as static steps beside the engine's."*

Since, each stage ran both by hand and found them clean: [S2.1.2](../../s2.1.2_diagnostics_surface.md) on `d13dc4d9`,
[S2.1.3](../../s2.1.3_feedback_object.md) on `0ab1aef0`.

**What the default costs.** EinAf's formatting and its lints are held by whoever remembers them: a file left
unformatted, or a lint, reaches master behind a green gate, as the engine's did before its own steps existed
(`run_tests.sh`'s header lists what each found the first time it ran: three files unformatted, a range loop indexing
a slice and four needless borrows).

**The alternatives.**

1. **Both, as static steps beside the engine's** (the proposal): `run_tests.sh` and `per-commit.yml` gain two steps
   each, and the count of static checks moves where it is written (the gate's header, `utils/check_gate_steps.py`,
   `AGENTS.md` § Running the gate). `--tests-only` skips them as it skips the engine's.
2. **`cargo fmt --check` alone as a step**, clippy by hand: formatting does not depend on a toolchain's lints.
3. **As now**, both by hand, each stage's record saying it ran them.

**What bears on it.** Clippy's lints are a toolchain's, and EinAf has none of its own: CI builds it with the runner's
stable and the host with its rustc 1.100, while the engine's steps take the pinned 1.97.1 in CI
([L-M2.1](../../findings/toolchain/L-M2.1-the-host-builds-with-rustc-1-100-not-the-pinned-1-97-1.md),
[L-M2.2](../../findings/toolchain/L-M2.2-einaf-has-no-toolchain-pin-ci-builds-it-with-the-runners.md)). A clippy step
of EinAf's would fail on a lint a newer stable adds, as the host's rustc raised three on the engine's code in this
phase (`acd04b13`); pinning EinAf's toolchain would be part of alternative 1.
