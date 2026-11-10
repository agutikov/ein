# S1f.5.27 — The nightly reduced: long-running steps to the dev host, critical ones kept in CI

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 8
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). Not started.
**Depends on:** nothing in code. It edits `.github/workflows/nightly.yml`, `run_tests.sh` and the tests that diff
them, so it runs alone on the main tree.
**Takes:** [Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs), the three tests
whose slow half no tier runs, which the user accepted on 2026-11-10 as parked. This stage gives each a tier.

## The ruling

The close review's finding 8, as written on 2026-11-10:

> **`run_tests.sh` line 81 says the fuzz and bench jobs are in the nightly "because they are cheap"**, but the fuzz
> job takes about 25 min per seed since the parser swap.

The user, 2026-11-10: *"reduce nightly job, remove long running tests, we will eventually run them on dev host,
retain critical tests in CI"*.

## The case

The merged tree's first nightly, every step green, 81 min 54 s (S1f.5.19's T3 record, 2026-11-09):

| job | step | wall |
|---|---|---:|
| deep-corpus | `corpus_cli`, the 14 slow entries | 97.9 s |
| deep-corpus | `id_order_invariance`, eight seeds | 481.3 s |
| deep-corpus | `answer_dump` | 39.0 s |
| deep-corpus | `answer_round_trip`, the slow and `heavy_dump` entries | 148.6 s |
| deep-corpus | `lever_invariance`, the slow cells | 16.0 s |
| deep-corpus | `corpus_cores`, the slow entries | 203.7 s |
| deep-corpus | `utils/stdlib_census.py --check` | 429 s |
| deep-corpus | `utils/review_bundle.py` and its link check | 366 s |
| deep-corpus | `utils/stdlib_mutants.py --check` | 42 s |
| frontend-fuzz | `fuzz_properties`, 1 000 000 inputs, seed 1 | 1 532.7 s |
| frontend-fuzz | the same, seed 987654321 | 1 538.5 s |
| bench | `cargo bench --bench engine -- --test` | 5 s |

On 2026-11-03 both fuzz seeds and the bench smoke took 15 s together (S1e2.3.6), which is what `run_tests.sh`'s
*"because they are cheap"* records. The fuzz job is about 200× that now. Two things grew it: the swap's parser, and
S1f.5.10's property, which compiles every accepted program's plans, *about 27 s per 20 000 inputs*
(S1f.5.10's record). T1 attributes the cost between the two. Not run by any tier
([Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)): `route_invariance`'s
slow half 857 s, `region`'s 38 s and `obligation_rung`'s 71 s.

## What it changes

**The rule**, taken by default and recorded as a Q-M1f row:
- **Critical** means a step guards an answer or a contract that no per-commit step guards. That covers the slow
  corpus entries' answers, the config principle on the slow cells, the cores and dumps of the entries the
  per-commit tier leaves out, and the test suite's sensitivity (the mutants).
- **Long-running** means a step over 5 min on the runner, or a step that measures rather than checks.
- A critical step that is long-running stays in CI at a budget that fits, and runs whole on the dev host.

**The classification it gives** (T1 re-takes the timings and confirms each):

| step | tier |
|---|---|
| `corpus_cli` slow entries, `answer_dump`, `answer_round_trip` slow + `heavy_dump`, `lever_invariance` slow cells, `corpus_cores` slow entries, `stdlib_mutants.py --check`, the bench smoke | **CI nightly**, as today |
| `region`'s and `obligation_rung`'s slow halves (Q-M1f.51) | **CI nightly**, added: each guards answers and is under 5 min |
| the fuzzer | **CI nightly at a reduced budget** (default 100 000 inputs a seed, about 2.5 min); **1 000 000 a seed on the dev host** |
| `id_order_invariance`, eight seeds | **dev host** (the per-commit tier runs one seed) |
| `stdlib_census.py --check` | **dev host** (a measurement; the per-commit `stdlib_coverage` is its gate) |
| `review_bundle.py` and its link check | **dev host** (the per-commit `--smoke` reads every row and builds two cases) |
| `route_invariance`'s slow half (Q-M1f.51) | **dev host** |

**The dev-host tier.** `./run_tests.sh --dev-host` runs every step the CI nightly runs, at full budget, plus the
dev-host steps, from the repo root and under a checked `TMPDIR` as `--nightly` does. `--nightly` stays the CI
nightly's exact list. A test pins the dev-host list as `nightly_steps.rs` pins the nightly's, and a step that
leaves the nightly must appear in it. Nothing schedules it: the user runs it.

## Tasks

### Task T1f.5.27.1 — the timings re-taken, and the fuzz cost attributed

Each step on this machine (`./run_tests.sh --nightly` under `utils/bench_env.sh`), and the fuzz property's cost per
input split between the parse and the plan property. Confirm or move each row of the classification, and record it.

### Task T1f.5.27.2 — the tiers

`nightly.yml` reduced, `--dev-host` added to `run_tests.sh`, `nightly_steps.rs` and the new pin, the fuzz budget
an env value both tiers set. Run `--nightly` and `--dev-host` once each, green, with their times.

### Task T1f.5.27.3 — the pages

`run_tests.sh`'s header (the *cheap* sentence corrected), AGENTS.md's *Running the gate*, `corpus/README.md`'s
tier table, Q-M1f.51's row, the stage's records.

## Acceptance

- The CI nightly holds only the critical steps, at the stated budgets, and its wall time is recorded.
- Every step it dropped runs in `./run_tests.sh --dev-host`, pinned by a test. Both tiers ran green once.
- No per-commit step moved.
- `./run_tests.sh` green.
