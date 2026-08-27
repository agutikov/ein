# S1e.4.5 — Tests (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 2 days
**Depends on:** [Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) for
`TE-L5`; [S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md) T1 for `TE-L3`
(the same step-list diff, one flag down);
[T1e.1.6.1](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) for `TE-L2`'s
anchor list, which falls out of that grep.
**Findings:** [`TE-L1`](../review/tests/low.md) …
[`TE-L5`](../review/tests/low.md).

## Context

Five findings about the gate's edges. Two are about **tests that can fail for
a reason other than the code** (`TE-L1`), or **couple to files nobody warns
you about** (`TE-L2`); three are about **checks that do not run** — a flag
that skips more than it says (`TE-L3`), a census wired to nothing (`TE-L4`),
and a release matrix that has never executed (`TE-L5`).

`TE-L1` is the only place in the workspace where a test can fail on machine
load rather than behaviour:
[`test_cli.rs:203-225`](../../../ein.rs/crates/ein-cli/tests/test_cli.rs)
requires the features directory to finish in under 20 s, and
[`corpus_cli.rs:335-412`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)
asserts dev-profile wall clock at 4× tolerance. Both are deliberately
generous; on a badly overloaded runner they become flakes, and a flake in this
suite will be read as an engine regression, because everything else here is
deterministic by construction.

`TE-L5` is the one worth being blunt about: four platform legs, a
jobs-cross-diff and a `--no-default-features` leg, **none of which has ever
run**. The workflow's own header says so — *a green badge here is read as: the
first tag passed* — which is honest and is also exactly why a reader would
believe the badge.

## Acceptance

- No test can fail on machine load without saying so in its failure message,
  or the timing assertions are gated behind an env var.
- The world-anchor list exists in one place and both puzzle files point at it.
- `run_tests.sh --tests-only` either runs the bench smoke or its header stops
  claiming it skips only the static checks.
- `stdlib_census.py --check`'s cadence is stated — nightly, or
  milestone-cadence with that written down.
- The release matrix's status is resolved per
  [Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md).

## Tasks

### Task T1e.4.5.1 — `TE-L1`: make load-sensitivity legible

Two changes, and the second matters more than the first:

1. **Label the failures.** Both assertions' messages say *"machine load?"* and
   name the measured and expected values. A flake that announces itself as a
   possible flake costs a reader a minute instead of an hour.
2. **Decide whether they belong in the per-commit tier at all.** The repo has
   a precedent both ways: the wall-clock floors for slow entries are nightly
   ([TE-M3](../p1e.3_medium/s1e.3.6_tests.md)), and the bench smoke is in the
   gate. The cleanest split is to gate the *timing* on an env var (as
   `EIN_CORPUS_SLOW` already gates the slow entries) and keep the
   *completion* — that the features directory finishes at all — in the
   default run, which is the property that actually guards against a
   pathological regression.

Do not simply raise the margins. A tolerance raised until it never fires is a
test that has been deleted without anyone noticing.

### Task T1e.4.5.2 — `TE-L2`: one anchor list

Four crates' tests hard-code facts about `zebra.ein` and `zebra2.ein` —
[`embedding.rs:126-139`](../../../ein.rs/crates/ein-cli/tests/embedding.rs),
[`kb_semantics.rs:1101`](../../../ein.rs/crates/ein-ir/tests/kb_semantics.rs),
[`cli_semantics.rs:156-176, 304-330`](../../../ein.rs/crates/ein-cli/tests/cli_semantics.rs),
[`obligation_reports.rs:263-285`](../../../ein.rs/crates/ein-infer/tests/obligation_reports.rs)
— plus `docs/api/rust.md`. They are deliberate anchor tests and their docs say
so; only `embedding.rs` documents the cost. A reviewer changing `zebra2.ein`
has **no list** of what will fire.

The fix is a list, and the place for it is the puzzle files themselves: a
header comment in `zebra.ein` and `zebra2.ein` naming the anchors, since that
is what the person making the edit is looking at. The list comes free from
[T1e.1.6.1](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md)'s grep — take it
from there rather than re-deriving it.

Worth checking while assembling it: whether any anchor is a *number* that
S1d.2.4's activator facts already moved.
[`docs/api/rust.md`](../../../docs/api/rust.md) documents its own 434 → 444
move; the review notes other tables did **not** get the same audit
([DO-L2](s1e.4.7_documentation.md)).

### Task T1e.4.5.3 — `TE-L3`: `--tests-only` skips more than it says

`run_tests.sh:7` says the flag *skips the static checks*; `:186-189` shows it
also skips the **bench smoke**, which is CI's last step. So a green
`--tests-only` is a strict subset of CI — the precise property the script's
own header warns about, in miniature — and a bench that stops compiling is
invisible to `cargo test`.

Prefer **running the bench smoke under `--tests-only`**: it is a compile plus
one short run, the flag exists to skip the ~5 s of static checks, and the
whole point of the flag is to be a faster gate rather than a weaker one. If
that costs too much, amend the header instead — but then the step-list diff
from [S1e.3.4](../p1e.3_medium/s1e.3.4_architecture.md) T1 must compare the
flag's list too, or the divergence just moves.

### Task T1e.4.5.4 — `TE-L4`: state the census's cadence

`utils/stdlib_census.py --check` is wired to no gate and no workflow. That is
**by design** — the in-gate check is scoped to `tests/stdlib/`
([`stdlib_coverage.rs:28-35`](../../../ein.rs/crates/ein-infer/tests/stdlib_coverage.rs))
— but the corpus-wide census, with its numbers and its sole-activator table,
is then re-taken only when someone remembers, which is the failure the
coverage test's own doc comment names for scripts.

Two options: a nightly step (cheap — the census is 37 s over 180 entries), or
an explicit statement in [`utils/README.md`](../../../utils/README.md) that it
is milestone-cadence and why. Prefer the nightly: four of the repo's five
censuses are re-takable instruments and only this one has a `--check` mode
already written, so wiring it costs a YAML block.

Whichever, the sentence goes in `utils/README.md` beside the script's line, so
a reader knows whether the number they are reading is current.

### Task T1e.4.5.5 — `TE-L5`: resolve the release matrix

[Q10](../p1e.1_open_questions/s1e.1.6_coverage_gaps.md) took the decision —
run the matrix once on a pre-release tag, or accept until the first real tag.
Carry it out:

- **If run**: tag, watch the four platform legs, the jobs-cross-diff and the
  `--no-default-features` leg, and record what broke. Something usually does,
  and finding it on a pre-release tag rather than on the first real one is the
  entire value.
- **If accepted**: the sentence goes in
  [`docs/install.md`](../../../docs/install.md), which is the page that offers
  *a release binary* as a channel and is therefore where a reader would
  otherwise assume the matrix works.

## Notes

Two days, and `TE-L5` is the one that can consume all of it if the matrix is
actually run and actually breaks. That is a good use of the time and it is
also a reason to sequence it last in the stage: the other four are bounded,
and a broken Windows build is a finding for a followup, not a reason to hold
the phase.
