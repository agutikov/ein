# S1e.3.6 — Tests (Medium)

**Phase:** [P1e.3](README.md) (Medium)
**Estimate:** 5 days
**Depends on:** [S1e.3.4](s1e.3.4_architecture.md) T1 for `TE-M8` (the
gate/CI diff is one of `AR-M1`'s four pairs).
**Findings:** [`TE-M1`](../review/tests/medium.md) …
[`TE-M8`](../review/tests/medium.md).

## Context

Eight findings about the gate — the largest topic in the review, and the one
where the repo's own standards are highest. None of them is *a test is
wrong*. All eight are **a test's sensitivity is lower than it looks**, which
is a harder class to see and the class this project has already been bitten
by twice.

The two precedents are both in the tree's own record. `dot_wellformed` was
converted from skip to fail because a skip is a pass nobody reads; the oracle
ledger § 2 records *41 tests passing on a SKIP line nobody read*. And
S1a.9.0 exists because a `slow = true` flag that stopped being true was
invisible.

`TE-M1` reproduces the first precedent exactly: `gen_zebra2_variants.py
--check` is invoked from a cargo test, and when `python3` is absent or exits
127 the test **eprintlns and passes**
([`cli_semantics.rs:269-293`](../../../ein.rs/crates/ein-cli/tests/cli_semantics.rs)),
while `CLAUDE.md` says flatly *`--check` is in the gate*.

`TE-M3` reproduces the second: the default sweep contains no slow entries, so
the wall-clock floor is skipped for them in every per-commit run and every
default local gate; a `slow = true` that stops being true is visible only to
nightly plus a reader of nightly.

`TE-M2` is decay by construction: `assert_census` requires `checked >= 55`
over what is now **197** corpus files, and `every_positive_entry_answers`
floors at `>= 60` against ~143 eligible entries with a triply-stale inline
comment. Roughly half the positive corpus could stop being swept before
either assertion fired. The floor-not-exact pattern exists so the corpus can
grow — and nothing ever re-tightens it, so sensitivity decays monotonically
while the number of files grows.

The remaining four are localized: an exit-code coupling that is safe only by
accident (`TE-M4`), two tests that assert almost nothing (`TE-M5`), a
hand-taken mutation sweep with no instrument (`TE-M6`), and the NAF
boundary's invalidation machinery with no direct unit test (`TE-M7`).

## Acceptance

- **No test in the workspace passes on a missing dependency.** `TE-M1`'s skip
  becomes a failure, and a grep confirms it is the only one of its shape.
- **The vacuity floors are derived**, not constant: from the manifest's
  eligible count minus a small stated slack, so they track the corpus.
- **The mutation sweep is an instrument** — `utils/stdlib_mutants.py` — and
  the one survivor it found has a fixture that kills it.
- **The NAF boundary's fast path has two direct tests** (a stalled guard not
  re-judged; a fork-inherited candidate re-judged exactly when its watched
  extent grew).
- **The gate and CI step lists are diffed by a test** (with
  [S1e.3.4](s1e.3.4_architecture.md)).
- Every claim this stage changes about the suite's size is quoted from a run.
- `./run_tests.sh` green, and the derived floors are run as a **report** over
  the corpus before becoming assertions.

## Tasks

### Task T1e.3.6.1 — `TE-M1`: fail, don't skip

Convert the `python3`-absent path in
[`cli_semantics.rs:269-293`](../../../ein.rs/crates/ein-cli/tests/cli_semantics.rs)
from `eprintln!` + pass to a failure, as `dot_wellformed` already does for
Graphviz — the precedent, the wording and the justification all exist.

Then sweep for siblings: any test in the workspace that tolerates a missing
external tool, a missing env var, or a non-zero exit from a helper by
printing and passing. The review found one; the shape is worth confirming
absent elsewhere, and the grep is minutes.

Note what stays true even with the skip: the *structural* diff still runs, and
CI installs Python. The exposure is a **local** gate on a machine without
`python3` reporting a pass on a rule-body drift in a generated variant — which
is the same local-gate-lies failure `run_tests.sh`'s own header warns about.

### Task T1e.3.6.2 — `TE-M2`: derive the floors

Two assertions, two constants:

- [`lattice_semantics.rs:272-293`](../../../ein.rs/crates/ein-infer/tests/lattice_semantics.rs)
  — `checked >= 55` over 197 files;
- [`corpus_cli.rs:451-458`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs)
  — `>= 60` against ~143 eligible entries, with a stale comment.

Replace both with a value computed from `ein-corpus`'s manifest — the crate
exists for exactly this and already exposes the entries. `eligible_count -
slack`, with `slack` small, named, and commented with *why* a slack exists at
all (entries legitimately skipped, and the list of reasons).

Order matters: **run it as a report first.** Print the derived floor and the
actual count for both assertions over today's corpus, and look at the gap. If
the gap is large, something is not being swept that should be, and that is a
finding to report before it is a constant to change.

### Task T1e.3.6.3 — `TE-M3`: assert the slow entries still exist

Keep the timing direction nightly — the review agrees, and a wall-clock
assertion in the per-commit tier is [TE-L1](../p1e.4_low/s1e.4.5_tests.md)'s
problem. What the default run *can* cheaply assert:

- the two `slow = true` entries still exist in the manifest by name;
- each carries a `cost_ms`;
- and the count is exactly two, so an entry silently gaining the flag is
  visible.

That converts an invisible-for-a-day rot into a per-commit signal for the part
that costs nothing to check. The direction that genuinely needs a stopwatch
stays where it is, and
[`corpus_cost.md`](../../../docs/history/m1a_rust/measurements/corpus_cost.md)
stays the instrument that re-prices.

### Task T1e.3.6.4 — `TE-M4`: exit 2 means more than one thing

`no_cell_crashes`
([`corpus_cli.rs:303-333`](../../../ein.rs/crates/ein-cli/tests/corpus_cli.rs))
hard-codes *exit 2 = the CLI refused the argv*. But `ein solve` exits 2 for a
**budget abort** and `ein test` for **load errors / nothing-to-check**
([`cli_semantics.rs:943-964`](../../../ein.rs/crates/ein-cli/tests/cli_semantics.rs),
[`test_cli.rs:332-396`](../../../ein.rs/crates/ein-cli/tests/test_cli.rs)).

It is safe today only because no corpus run declares `-E`/`-T` and `test`
runs are declared only on files that load and claim. Adding a budgeted run or
a `test` cell on a loader-negative would make an *intended* exit 2 fail the
crash check with a misleading message.

Two fixes and they compose: make the check consult the entry's group and
declared flags rather than the bare code, **and** state the constraint in
[`corpus/README.md`](../../../corpus/README.md)'s run-vocabulary section,
where it is currently unstated. Then add the run that would have broken it —
a budgeted cell on one entry — so the coupling is not merely documented but
exercised.

This is also half of [EH-M1](s1e.3.5_error_handling.md)'s exit-code
conversation; the `Q-M1e.<n>` filed there should carry this finding's
evidence.

### Task T1e.3.6.5 — `TE-M5`: make the or-matcher tests assert something

[`expect_semantics.rs:354-380`](../../../ein.rs/crates/ein-infer/tests/expect_semantics.rs):
`two_identical_disjuncts_do_not_cover_two_models` asserts only
`!lines.is_empty()` **after failure is already established** — so any failure
text passes — and `one_wrong_disjunct_fails` only checks that a line starts
with `"expectation "`.

The property the docs claim is that the augmenting-path matcher produces a
*specific, non-confusing* report — precisely the thing the greedy pairing did
not. A regression to the confusing message would pass both tests today.

Fix: assert the decisive substring of the report in both. Read the current
output first and pick the phrase that distinguishes the augmenting-path
report from the greedy one; if no such phrase exists, that is the more
interesting finding and the report needs it before the test can name it.

### Task T1e.3.6.6 — `TE-M6`: bank the mutation sweep

`tests/README.md:154-166` records a 51-mutant sweep the suite catches 50 of,
and the survivor — `slot-adjacent-bwd-neg`
([`stdlib/slots.ein:408-415`](../../../stdlib/slots.ein)) with its two
structure operands exchanged — is killable by nothing in the suite. The sweep
was **hand-taken, with no `utils/` script**, unlike every other census in the
repo.

Two deliverables:

1. **`utils/stdlib_mutants.py`.** The mutation vocabulary is already written
   down in `tests/README.md`; the script applies it to `stdlib/*.ein`, runs
   `ein test tests/` per mutant, and reports survivors. It joins the other
   four re-takable censuses and gets a line in
   [`utils/README.md`](../../../utils/README.md). Cost: it is 51 runs of a
   0.04 s suite, so it is cheap enough to be a nightly rather than a
   milestone-cadence instrument.
2. **A fixture that kills the survivor.** A program where
   `slot-adjacent-bwd-neg` with exchanged operands derives something the
   correct rule does not. That is a real stdlib soundness shape and the suite
   should hold it.

The second matters more than the number: the recorded 50/51 will rot silently
as rules are added, and a real defect of that exact shape passes `ein test
tests/`, `stdlib_coverage`, the corpus sweep and every golden.

### Task T1e.3.6.7 — `TE-M7`: two tests for the NAF fast path

The invalidation machinery that replaced a per-candidate stamp
([`saturator.rs:1112-1116, 1230-1252`](../../../ein.rs/crates/ein-infer/src/saturator.rs))
has no direct unit test: nothing asserts that a parked candidate whose watched
relations did not grow is skipped (the `judged_at` / `gs_epoch` fast path),
nor that `watched_sizes` carried across a resume behaves at the fork
boundary. The guarantee rides indirectly on corpus event goldens and counter
tests.

That is a weaker localization than the rest of the NAF boundary enjoys —
`naf_dropped`, one-admission and retirement all have named tests — and the
failure mode here is the quiet one: a missed re-judgement **under-derives**
(a `forall` flip never noticed) rather than crashing.

Write the two the review names:

1. a stalled-guard candidate is **not** re-judged when an unrelated relation
   grows;
2. a fork-inherited candidate **is** re-judged exactly when its watched extent
   grew in the fork.

Both are assertions about the epoch counters, so they can be written against
`--events` or against the saturator's own stats — prefer whichever the sibling
NAF tests already use, so the file stays one idiom.

### Task T1e.3.6.8 — `TE-M8`: diff the two step lists

Owned jointly with [S1e.3.4](s1e.3.4_architecture.md) T1, since it is one of
`AR-M1`'s four pairs. What this stage adds is the *test's* content: the
comparison must cover `--tests-only`'s list too, because
[TE-L3](../p1e.4_low/s1e.4.5_tests.md) is the same divergence one flag down —
the flag skips CI's last step (the bench smoke) while its header says it skips
only the static checks.

## Notes

Five days, and the two that can overrun are T1e.3.6.2 (if the derived floor
exposes unswept entries) and T1e.3.6.6 (if the survivor's fixture turns out to
need a stdlib change rather than a test). Both overruns are findings, not
delays — report them rather than absorbing them.

Nothing here loosens a check. If a derived floor or a converted skip makes the
gate red, the red is the information; the fix is upstream of the assertion,
never the assertion.
