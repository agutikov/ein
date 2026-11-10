# S1f.5.27 — The nightly reduced: long-running steps to the dev host, critical ones kept in CI

**Phase:** [P1f.5](README.md) (The no-owner items)
**Status:** specified 2026-11-10 on the user's ruling on the close review's finding 8
([close_review.md § Findings with no owner](close_review.md#findings-with-no-owner)). **Done 2026-11-10**, in the
working tree on `566c9e3d` for the coordinator's commit: T1f.5.27.1, the timings re-taken here and read off three
nightlies on the runner, and the fuzz job's cost attributed: it is the parse, five properties paying it each, and three
quarters of the parse is the recovery search of 2.6 % of the refused inputs; S1f.5.10's plan property adds 1 %
([§ T1 record](#t1-record-2026-11-10)); T1f.5.27.2, the tiers: the nightly at twelve steps, 768.1 s here, the fuzzer
at 20 000 inputs a seed, and `./run_tests.sh --dev-host`, seventeen steps, 6 712.9 s (1 h 51 min 53 s) here, both
green, held together by four new tests in `nightly_steps.rs` ([§ T2 record](#t2-record-2026-11-10)); T1f.5.27.3, the
pages
([§ T3 record](#t3-record-2026-11-10)). The classification is confirmed row by row but the fuzzer's budget, moved from
100 000 to 20 000 a seed, which the rule's 5 min on the runner sets. Five choices taken by default, Q-M1f.102 to
Q-M1f.106, and Q-M1f.51 taken, (b) by tier. No answer, golden or register row moved: 249 fixed, 13 open and 1 not wrong
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10). No wrong answer was found. The stage changes
a CI workflow, so `utils/ci_container.sh --nightly` on its commit is the coordinator's run (Q-M1f.77)
([§ Acceptance, re-taken](#acceptance-re-taken-2026-11-10)).
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

## T1 record, 2026-11-10

T1f.5.27.1, on `566c9e3d` with the stage's tree. Logs under `build/s1f.5.27/` and `build/` (gitignored). Every local
time is under `utils/bench_env.sh`, which prints the machine state at each log's head (`--cores` with all 32 CPUs for a
tier's run; `--core 4`, one P-core, for a probe; `--cores PT:4`, two P-cores' four threads, the runner's count).

### The runner

Each step's wall on GitHub's runner, from three nightlies (`gh run view <id> --json jobs`, 2026-11-10; the step's build
included, the fuzz and bench jobs building the release profile with no cache). One step's time varies between runs by
up to 2×, with the machine and the code: the 11-10 `deep-corpus` job was 1.1× to 2× faster than the two before in seven
of the nine steps they share, and slower in `answer_dump` and `stdlib_mutants.py` than on 11-08.

| step | 11-08, `76318fd6` (42469293951) | 11-09, `8b1bcabb` (42605782048) | 11-10, `f454de3b` (42802597417) |
|---|---:|---:|---:|
| `corpus_cli`, the slow entries | 185 s | 164 s | 121 s |
| `id_order_invariance`, eight seeds | 505 s | 601 s | 452 s |
| `answer_dump` | 40 s | 72 s | 47 s |
| `answer_round_trip`, the slow and `heavy_dump` entries | not yet a step | not yet a step | 225 s |
| `lever_invariance`, the slow cells | 36 s | 42 s | 27 s |
| `corpus_cores`, the slow entries | 428 s | 436 s | 256 s |
| `stdlib_census.py --check` | 642 s | 642 s | 380 s |
| `review_bundle.py` | 752 s | 740 s, red (Q-M1f.52) | 372 s |
| `doc_audit.py --links --check build/review` | 20 s | skipped | 13 s |
| `stdlib_mutants.py --check` | 28 s | skipped | 42 s |
| the fuzzer, seed 1, a million | 81 s | 2 572 s | 4 472 s |
| the fuzzer, seed 987654321, a million | 107 s | 3 020 s | 3 548 s |
| the bench smoke | 98 s | 96 s | 100 s |

The fuzz job grew 28× to 55× across S1f.5.5's parser swap, which landed between `76318fd6` and `8b1bcabb`.

### Here

The steps the nightly keeps, from the reduced nightly's run (§ T2 record), and the steps that left or never ran in CI,
from the dev host's run, which runs every old nightly step at its old budget; the old nightly's own run here, S1f.5.19's
of 2026-11-09, is the stage file's table.

| step | the nightly here | the dev host here | S1f.5.19, here |
|---|---:|---:|---:|
| `corpus_cli`, the slow entries | 103.5 s | 101.5 s | 97.9 s |
| `id_order_invariance`, eight seeds | — | 531.3 s | 481.3 s |
| `answer_dump` | 41.5 s | 40.5 s | 39.0 s |
| `answer_round_trip`, the slow and `heavy_dump` entries | 154.2 s | 150.0 s | 148.6 s |
| `lever_invariance`, the slow cells | 18.8 s | 16.6 s | 16.0 s |
| `corpus_cores`, the slow entries | 206.6 s | 205.4 s | 203.7 s |
| `region`, the slow half | 35.7 s | 32.3 s | — |
| `obligation_rung`, the slow half | 71.6 s | 70.8 s | — |
| `route_invariance`, the slow half | — | 812.0 s | — |
| `check_test_scratch.py` | 0.0 s | 0.0 s | — |
| `stdlib_census.py --check` | — | 439.0 s | 429 s |
| `review_bundle.py` | — | 325.0 s | 366 s, with its link check |
| `doc_audit.py --links --check build/review` | — | 10.2 s | |
| `stdlib_mutants.py --check` | 42.3 s | 42.4 s | 42 s |
| the fuzzer, seed 1 | 47.8 s, 20 000 | 1 966.8 s, a million | 1 532.7 s, a million |
| the fuzzer, seed 987654321 | 44.5 s, 20 000 | 1 967.5 s, a million | 1 538.5 s, a million |
| the bench smoke | 1.0 s | 0.9 s | 5 s |

The dev host's million took 1 966.8 s a seed where S1f.5.19's took 1 532.7 s; S1f.5.21 has added a seventh property
since, another that parses the whole stream, and what else the gap is was not measured.

### The fuzz job's cost, attributed

The stage asked for the property's cost per input split between the parse and S1f.5.10's plan property. Two scratch
probes, appended to a copy of `fuzz_properties.rs` and deleted after (`build/s1f.5.27/t1_probe_seed1.log`,
`…seed987654321.log`, `t1_probe2_seed1.log`), each phase timed per input over the stream at 20 000 inputs, release, one
P-core:

| phase | seed 1 | seed 987654321 |
|---|---:|---:|
| generating an input | 2.8 µs an input | 2.8 µs |
| **the parse** | **1 940 µs an input**: 15 702 refused at 2 464 µs each, 4 298 accepted at 25.8 µs | **1 838 µs**: 15 624 at 2 345 µs, 4 376 at 26.5 µs |
| loading a parsed program | 95 µs a program, 20 µs an input | 94 µs a program |
| **S1f.5.10's plan property** (`unbound_conclusion`: its own parse, load and compile) | 105 µs a loaded program, **20.4 µs an input** | 106 µs, 21.3 µs an input |
| S1f.5.21's compile property (`compile_refusal`, the same) | 100 µs a loaded program, 19.4 µs an input | 101 µs, 20.3 µs |
| the compile alone, every plan once | 1.0 µs a loaded program | 1.0 µs |
| the round trip, an accepted program | 49 µs | 50 µs |

**The plan property is 1 % of an input's cost, and the parse 99 %.** Each property alone, seed 1, 20 000 inputs, one
P-core (`build/s1f.5.27/t1_fuzz_per_property_seed1.log`): the five that read the whole stream take 38.99 s to 41.46 s
each, `no_firing_meets_an_unbound_variable`, S1f.5.10's, 40.39 s against 40.48 s for
`generated_input_is_always_parsed_or_diagnosed`, which parses and does nothing else; the refusal property, which parses
one input in four twice, 20.90 s; the seeds' table 0.07 s. So the job's cost is the parse, paid once by each of the five
stream properties, each of which generates and parses the stream for itself.

**And the parse's cost is the recovery search.** By the steps lrpar's recovery took (`ParseError::recovery_steps`),
seed 1:

| refused inputs | inputs | each | of the 37.4 s |
|---|---:|---:|---:|
| under 100 steps | 8 986 | 70 µs | 0.6 s |
| 100 to 999 | 5 157 | 410 µs | 2.1 s |
| 1 000 to 9 999 | 1 156 | 5.2 ms | 6.0 s |
| 10 000 or more, not cut | 263 | 34.8 ms | 9.2 s |
| cut at `RECOVERY_STEP_BUDGET` (30 000) | 140 | 139 ms | 19.5 s |

The 403 refused inputs that take 10 000 steps or more, 2.6 % of the refused, are 77 % of the parse time, and the 140 cut
at the budget 52 %. The input's length does not explain it: a refused input under 128 bytes costs 2.2 ms on average and
one of 512 to 1 023 bytes 2.9 ms, where an accepted one costs 6 µs and 101 µs. `fuzz_properties.rs`' refusal property
said a refusal's milliseconds were *most of them lexing*; its comment now says what this measured.

**What remains of S1f.5.28's item 10**, *"The fuzz property costs about 27 s per 20 000 inputs"* (S1f.5.10's record):
not S1f.5.10's property, which adds 0.4 s per 20 000 inputs to the 40 s each stream property pays here. What is left is
two things, neither this stage's: the test's shape, five properties each generating and parsing the same stream, so an
input is parsed five times where once would serve; and the recovery search's cost at its budget, the parser's
([Q-M1f.8](../open_questions.md#q-m1f8--the-parsers-remaining-cost-within-grmtools-or-past-it), S1f.5.5's F11 budget).
S1f.5.28 triages them.

### The nightly's fuzz budget, read on the runner

On the runner a million took 4 410.06 s and 3 494.14 s a seed in `cargo test`'s own count (run 42802597417, six
properties then, on four vCPUs), its four stream properties side by side, each finishing at 4.28 to 4.32 ms an input
(seed 1) and 3.40 to 3.43 ms (seed 987654321). Here, on four P-core threads, the runner's count, 20 000 inputs take
106.29 s and 100.07 s for the seven properties (`build/s1f.5.27/t1_fuzz_pt4_seed*.log`), the four side by side
finishing at 3.13 to 3.22 ms an input and 2.95 to 3.05 ms. So an input costs 1.34× and 1.13× more on the runner, and the
step there comes to about 196 s and 160 s at 20 000 with its build (53.82 s and 47.07 s), and about 770 s and 610 s at
the stage file's 100 000. 100 000 is past the rule's 5 min, so the budget moved to 20 000
([Q-M1f.103](../open_questions.md#q-m1f103--the-fuzzers-budget-in-the-ci-nightly)). The first nightly after the push
measures it.

### The classification, confirmed or moved

| step | what it guards that no per-commit step does | over 5 min on the runner? | tier | |
|---|---|---|---|---|
| `corpus_cli`, the slow entries | the 14 `slow` entries' cells and the flag's timing band | no: 121 to 185 s | CI nightly | confirmed |
| `answer_dump` | the per-commit set again, in release | no: 40 to 72 s | CI nightly | confirmed, as the stage file had it |
| `answer_round_trip`, the slow and `heavy_dump` entries | 19 entries' dumps reloaded | no: 225 s | CI nightly | confirmed |
| `lever_invariance`, the slow cells | the config principle on the zebra2 pair | no: 27 to 42 s | CI nightly | confirmed |
| `corpus_cores`, the slow entries | the class-4 claim on the zebra family | 256 s on 11-10, 428 and 436 s before | CI nightly | confirmed, on the line ([Q-M1f.104](../open_questions.md#q-m1f104--a-critical-step-the-runners-spread-puts-on-the-line)) |
| `region`, the slow half | the state route held to route 1 on `zebra2-minus-15` through layer 6 | not run there; 35.7 s here | CI nightly, added | confirmed |
| `obligation_rung`, the slow half | the theory held to the hrule's 32 states on `zebra2-minus-15` | not run there; 71.6 s here | CI nightly, added | confirmed |
| `stdlib_mutants.py --check` | the suite's sensitivity | no: 28 to 42 s | CI nightly | confirmed |
| the bench smoke | nothing: the gate's last step | no: about 100 s, a cold build | CI nightly | confirmed, as the stage file had it |
| the fuzzer | the parser's, loader's and compiler's properties past the per-commit 2 000 | yes: 2 572 to 4 472 s a seed | CI nightly at 20 000 a seed, the dev host at a million | **moved**: 20 000, not 100 000 ([Q-M1f.103](../open_questions.md#q-m1f103--the-fuzzers-budget-in-the-ci-nightly)) |
| `id_order_invariance`, eight seeds | nothing: the per-commit tier runs one permutation of every file | yes: 452 to 601 s | dev host | confirmed |
| `stdlib_census.py --check` | nothing: a measurement, `stdlib_coverage` its per-commit gate | yes: 380 to 642 s | dev host | confirmed |
| `review_bundle.py` and its link check | the review's pages, not an answer; the per-commit `--smoke` reads every row and builds four cases (two when the stage file was written; S1f.5.25 added `W75/solve`) | yes: 372 to 752 s | dev host | confirmed ([Q-M1f.106](../open_questions.md#q-m1f106--the-whole-review-bundle-on-the-runners-graphviz)) |
| `route_invariance`, the slow half | route 1's agreement at `--region-budget` 2²⁵ and the rule's premise, a measurement of work; the state route's answers on its programs are digested per commit (`corpus_shapes`) | not run there; 812.0 s here | dev host | confirmed |

`read_out_examples`, the fourth target with a slow branch and no tier
([Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)), still has nothing in it:
the branch reads only the page's blocks at `--region-budget 33554432`, and the page has none (`grep -c 33554432
docs/kernel/inference/read_out_examples.md`, 0, 2026-11-10). `every_slow_half_runs_on_a_tier` checks that premise.

## T2 record, 2026-11-10

T1f.5.27.2.

**The tiers.**
- [`nightly.yml`](../../../.github/workflows/nightly.yml): `deep-corpus` loses `id_order_invariance`'s step,
  `stdlib_census.py --check`, `review_bundle.py` and its link check, and gains `region`'s and `obligation_rung`'s slow
  halves under `EIN_CORPUS_SLOW: "1"`, before the scratch check; the fuzz job sets `EIN_FUZZ_ITERS: "20000"`; the header
  states the rule and names the dev host, and each moved step's comment says where it went and why. Twelve steps.
- [`run_tests.sh`](../../../run_tests.sh): `--nightly` is the same twelve, in order. **`--dev-host`** is new: it takes no
  other argument, runs from the repo root under a checked `build/gate-tmp-<pid>`, and runs seventeen steps under
  `dev_host_step "…"` banners: the nightly's twelve in its order, the fuzzer at `EIN_FUZZ_ITERS="1000000"`, with
  `id_order_invariance`'s eight seeds, `stdlib_census.py --check`, the bundle and its link check where they ran in the
  old nightly, and `route_invariance`'s slow half beside the other slow halves, before the scratch check.
- The fuzz budget is an environment value each tier sets: `EIN_FUZZ_ITERS`, which the harness already read; no new
  name, so `config_reference.rs`' census of what the shipped binary reads is unmoved.

**The pins**, in [`nightly_steps.rs`](../../../ein.rs/crates/ein-cli/tests/nightly_steps.rs)
([Q-M1f.105](../open_questions.md#q-m1f105--how-the-dev-hosts-list-is-held)):
- `the_dev_host_runs_the_nightly_and_what_left_it`: every nightly step, matrix expanded, is a `--dev-host` step, in
  order, its fuzz budget read as the dev host's; the other steps are exactly `DEV_HOST_ONLY`'s five, each with why; and
  the list holds at least 17.
- `the_fuzzer_runs_at_each_tiers_budget`: each tier's two fuzz steps set `EIN_FUZZ_ITERS`, the nightly's above the
  per-commit 2 000 and below the dev host's 1 000 000, the dev host's exactly that.
- `every_dev_host_test_step_runs_under_the_checked_tmpdir`: every `cargo test` before the dev host's scratch check runs
  under `TMPDIR="${GATE_TMP}"`, none after it, and `DEV_HOST_ONLY`'s test steps are among them.
- `every_slow_half_runs_on_a_tier`, Q-M1f.51's (b): every `tests/*.rs` target whose code reads `EIN_CORPUS_SLOW`, eight
  today, has a step on one of the two lists that runs it under `--release` with the flag, or is `NO_SLOW_TIER`'s
  `read_out_examples`, whose premise it checks on the page.

**Each pin bites**, by eight scratch mutations of the two files, each restored after (`cmp` identical) and the test
binary run on each: the mutation census out of the nightly's two copies and left on the dev host
(`the_dev_host_runs…` fails, unnamed, and the nightly's floor of 12); the nightly's fuzz budget dropped from both copies
(`the_dev_host_runs…`, `the_fuzzer_runs…`); `id_order_invariance` dropped from the dev host (`the_dev_host_runs…`,
`every_dev_host_test_step…`); `route_invariance` moved after the scratch check (`every_dev_host_test_step…`); the bench
smoke out of all three copies (the dev host's floor and the nightly's); the dev host's fuzz at 20 000
(`the_fuzzer_runs…`, `the_dev_host_runs…`); `region`'s slow half out of all three (`every_slow_half_runs_on_a_tier` and
both floors); and a `33554432` appended to the read-out page (`every_slow_half_runs_on_a_tier`).

**No per-commit step moved.** `per-commit.yml` and the gate's `step "…"` banners are untouched (`git diff --stat`), and
`gate_steps.rs` passes.

**The two tiers, run once each, green** (`utils/bench_env.sh --cores 0,…,31`, every line stamped with `ts -s`):
- `./run_tests.sh --nightly`: **exit 0, 768.1 s (12 min 48 s)**, `build/nightly-s1f.5.27.log`, each step's time in § T1
  record's table. Every `cargo test` passed: `region` 28 of 28 and `obligation_rung` 6 of 6 under the flag, each fuzz
  seed 7 of 7; the scratch check found nothing; `stdlib_mutants.py --check` matched its bank.
- `./run_tests.sh --dev-host`: **exit 0, 6 712.9 s (1 h 51 min 53 s)**, `build/dev-host-s1f.5.27.log`. Every `cargo test` passed, the old nightly's at their old budgets: `id_order_invariance` 2 of 2 at
  eight seeds in 530.25 s, `route_invariance` 1 of 1 at its slow depth in 811.09 s, each fuzz seed 7 of 7 at a million
  in 1 966.70 s and 1 967.41 s; the scratch check found nothing; `stdlib_census.py --check` found 77 of 77 rules
  activated and its zero set empty; the bundle built 263 cases, 0 failed and 0 DOTs refused, in 324.8 s, and its link
  check read 7 113 pages, 0 findings; `stdlib_mutants.py --check` killed 167 of 217, its bank.

## T3 record, 2026-11-10

T1f.5.27.3, the pages:
- [`run_tests.sh`](../../../run_tests.sh)'s header: the usage line for `--dev-host`; the `--nightly` paragraph's
  *because they are cheap* replaced by the rule, the fuzz job's budget and what it cost on the runner; a `--dev-host`
  paragraph; `--slow`'s paragraph names both tiers.
- [`AGENTS.md`](../../../AGENTS.md)'s *Running the gate*: the command block (`--dev-host`, the fuzz line at the
  nightly's budget, the id seeds the dev host's), and a paragraph after the `--nightly` one on what each tier holds; its
  `stdlib_census.py` sentence (*it runs nightly*).
- [`corpus/README.md`](../../../corpus/README.md): a § Tiers with the table, test by tier, which the stage named and the
  page did not have; § `heavy_dump` names `--dev-host`.
- [`README.md`](../../../README.md)'s development loop, [`utils/README.md`](../../../utils/README.md)'s cadences
  (`stdlib_census.py`, the bundle, `check_test_scratch.py`, `doc_audit.py`, `ci_container.sh`),
  [`configuration.md`](../../../docs/kernel/configuration.md)'s `EIN_FUZZ_ITERS`, `EIN_ID_SEEDS` and
  `EIN_CORPUS_SLOW` rows (the last now naming its eight readers), and `utils/ci_container.sh`'s usage.
- The comments that named a moved step's tier: `stdlib_coverage.rs`, `stdlib_census.py`, `review_bundle.py`'s
  docstring, `fuzz_properties.rs`' budget and its refusal property's cost, `id_order_invariance.rs`, `region.rs`,
  `obligation_rung.rs`, `route_invariance.rs`, `steps/mod.rs`.
- [Q-M1f.51](../open_questions.md#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)'s row and section, taken;
  [Q-M1f.102](../open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs) to
  [Q-M1f.106](../open_questions.md#q-m1f106--the-whole-review-bundle-on-the-runners-graphviz), defaulted; the phase
  README's row; this file.

**Found, none of them a wrong answer:**
- `corpus_cores`' slow half is over the rule's line on a slow runner, 428 s and 436 s, and under it on the latest, 256 s
  ([Q-M1f.104](../open_questions.md#q-m1f104--a-critical-step-the-runners-spread-puts-on-the-line), kept whole).
- The refusal property's comment put a refusal's cost on lexing; T1's probe puts it on the recovery search. Corrected in
  place, being the paragraph the stage edits.
- The nightly's `answer_dump` step reruns the per-commit set under `--release` and adds no entry, as S1e2.3.6 noted;
  kept, as the stage file's row has it.

## Acceptance, re-taken 2026-11-10

On the working tree at `566c9e3d`, item by item:

- **The CI nightly holds only the critical steps, at the stated budgets, and its wall time is recorded.** Twelve steps,
  each guarding what no per-commit step does (§ T1 record's last table): every step the runner has run is under 5 min
  on its latest run, `corpus_cores` among them, which was over it on the two runs before (Q-M1f.104), and the two slow
  halves the stage added take 35.7 s and 71.6 s here; the fuzzer runs at 20 000 a seed, the budget the rule's line sets
  on the runner's rate (Q-M1f.103). Its wall: 768.1 s here (§ T2 record); on the runner it
  is the next nightly's to measure. **Met**, the runner's time open until the user's next push.
- **Every step it dropped runs in `./run_tests.sh --dev-host`, pinned by a test. Both tiers ran green once.** The four
  steps that left and `route_invariance`'s slow half are `DEV_HOST_ONLY`'s five, held by
  `the_dev_host_runs_the_nightly_and_what_left_it` and checked by mutation; `--nightly` exit 0 in 768.1 s,
  `--dev-host` exit 0 in 6 712.9 s. **Met.**
- **No per-commit step moved.** `per-commit.yml` and the gate's banners untouched, `gate_steps.rs` green. **Met.**
- **`./run_tests.sh` green.** Green, 17 min 4 s (`./run_tests.sh`, `build/gate-s1f.5.27.log`): 1 026 tests passed and
  0 failed, `nightly_steps.rs`' nine and `gate_steps.rs`' three among them; the smoke built its 4 cases with 0 DOTs
  refused; the link check read 409 pages, 0 findings. The edits after it started are this file's prose, re-checked by
  `doc_audit.py --links --check` on the final tree. **Met.**

## T4 record, 2026-11-10: the nightly in the runner's image

The stage changed a CI workflow, so it is done only when `utils/ci_container.sh` is green on its commit
([Q-M1f.77](../open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done)). The coordinator ran
`utils/ci_container.sh --nightly` on `089511fe`, the stage's commit, and it exited 0
(`build/ci-container-nightly-089511fe.log`, 2026-11-10). The image was Ubuntu 26.04.1 with Graphviz 14.1.2,
`libpathplan4` amd64v3 and the `fdp` layout present. It ran the nightly's twelve steps and no others: no
`id_order_invariance`, `stdlib_census.py` or `review_bundle.py` step. Each test step reported 0 failed. The test
times were the ones `cargo test` printed in that log:

| step | `finished in` |
|---|---:|
| corpus_cli, slow | 100.23 s |
| answer_dump | 40.79 s |
| answer_round_trip, slow | 150.96 s |
| lever_invariance, slow | 16.54 s |
| corpus_cores, slow | 205.99 s |
| region, slow | 25.55 s |
| obligation_rung, slow | 71.19 s |
| fuzz, seed 1, 20 000 inputs | 41.12 s |
| fuzz, seed 987654321, 20 000 inputs | 38.17 s |

The mutation census reported 167 of 217 killed, matching the bank. The bench smoke passed. The runner's own times are
the first GitHub nightly's after the user's push.
