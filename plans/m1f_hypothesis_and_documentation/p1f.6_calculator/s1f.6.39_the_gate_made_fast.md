# S1f.6.39 — The gate made fast: each check once, in parallel, at release speed; then what each invariance needs

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-15, on the user's word of that day ([§ The instruction](#the-instruction)). T1 to T5
keep every check the gate makes and run on defaults; **T6 stops for the user** with the audit of what each sweep
guards and needs; T7 applies what the user rules.
**Depends on:** nothing of the calculator. It runs after [S1f.6.38](s1f.6.38_a_number_is_well_formed_from_the_start.md)
and before [S1f.6.16](s1f.6.16_binary_arithmetic.md)'s build, since every later stage runs the gate it changes,
as [S1f.6.21](s1f.6.21_the_gate_reduced.md) ran first of its day for the same reason.

## The instruction

The user, 2026-11-15, after gate #3 (S1f.6.38's batch: `./run_tests.sh` in the runner's image 99 min, the nightly
79 min):

> the total testing time is insane; show me 1 - list of top level test suits (nightly, ...) 2 - what they contain
> with explanation what they do and time consumed 3 - why not test release build? 4 - analyse parallelism, if we
> can save time running in 16 parallell processes 5 - analyze importance of test packs, maybe we have too much
> duplications, I don't beleave Ein already have so many features to test them 3+ hours

and, on the analysis ([§ Today](#today-the-analysis-of-2026-11-15)):

> slowdown was because laptop worked on battery; Yes plan a stage, use all proposals

So all five proposals are the stage: (1) `jobs_invariance` at `--jobs N` only where the job count reaches, against
the golden; (2) `id_order_invariance` against the golden; (3) the sweeps and the mutation census split over a
worker pool; (4) a gate profile at release speed with the debug checks kept; (5) the audit of what each sweep
guards and which programs it needs, which changes what a commit guards and so is the user's to rule.

## Today: the analysis of 2026-11-15

Every number here is from gate #3's logs (`build/gate-batch3/`, `723dfd6e`, Ubuntu 26.04.1 image) and gate #2's
(`build/gate-batch2/`, `27197ea3`), or from runs of 2026-11-15 on this machine, an i9-14900HX (8 P-cores, CPUs
0–15; 16 E-cores, CPUs 16–31; 31 GB, no swap).

**Gate #3's mid-run slowdown was the machine: the laptop was on battery** (the user, 2026-11-15). From 38 min into
the run every binary took 2.1 to 2.9 times gate #2's, small ones too (`dot_wellformed` 7.6 → 21.9 s), where before
it the ratios were 0.93 to 1.5. The engine did not slow: four commits (`27197ea3`, `8340baf7`, `f555c7e0`,
`723dfd6e`) built in detached worktrees, every corpus program under `ein solve` and `ein oracle` with its own
stdlib, agree within ±2 % on the programs they share, in release and at the dev profile (`build/ab/times*.tsv`).
One serial thread on an E-core is 1.7 times slower than on a P-core (40 solves, `taskset -c 2` 1.46 s, `-c 20`
2.52 s, three times each).

**The tiers.**

| tier | command | what | gate #3 | gate #2 |
|---|---|---|---|---|
| per-commit gate (= `per-commit.yml`) | `./run_tests.sh` | six static checks (~1.5 min), `cargo test --workspace` (135 binaries, dev profile), scratch check, review-bundle smoke, bench smoke | 99 min | 57 min |
| nightly (= `nightly.yml`) | `./run_tests.sh --nightly` | deep corpus, 8 tests in release at their slow depth, 28 min; `stdlib_mutants.py --check`, ~49 min, serial; fuzz, 2 seeds × 20 000, 25 s; bench smoke | 79 min | 42 min (host) |
| dev host | `--dev-host` | the nightly at full budget and the steps that left it | — | — |

On GitHub the per-commit runs of 2026-11-13 and -14 took 58 to 82 min, the nightly 40 to 47 min (`gh run list`).

**Where the gate's 98 test minutes go** (gate #3; gate #2 in brackets):

| group | binaries | time | share |
|---|---|---|---|
| A. every corpus program (817) through all 41 outputs of `corpus_ops::ops()` | `jobs_invariance` 1 822 s [694], `id_order_invariance` 1 720 s [690], `corpus_shapes` 445 s [354] | 3 987 s | 68 % [52 %] |
| B. other whole-corpus sweeps | `explore` 378, `import_invariance` 164, `read_out` 100, `summary_properties` 98, `lever_invariance` 78, `corpus_cli` 73, `corpus_cores` 51, `region` 50, `ein-einb`'s `roundtrip` 46, `route_invariance` 34, and 18 more at 10 to 30 s | 1 395 s | 24 % |
| C. feature and unit tests | 104 binaries, `int_theory` 90 s the largest | 503 s | 9 % |

**Release.** The gate tests the dev profile (`opt-level = 1`, debug assertions and overflow checks on). The corpus
under `ein solve` is 1.58 times slower there than in release, under `ein oracle` 1.47 (`build/ab/`); a clean build
of `ein-cli` takes 22 to 26 s in either. What the dev profile buys is 25 `debug_assert!`s, one
`cfg(debug_assertions)` block (`ein-infer/src/engine.rs`) and overflow checks; the nightly's deep-corpus tests
already run in release, without them.

**Parallelism.** Cargo runs the binaries one after another; inside one the test functions run in parallel, but
every sweep of A and most of B is one `#[test]` with a serial loop over the corpus, so the gate keeps one of 32
hardware threads busy. The corpus under `ein solve` and `ein oracle`, release, through `xargs -P`: 1 process
100.5 s, 8 13.9 s, 16 14.5 s, 24 13.3 s. The floor is one program, `send_more_money.ein`, 12.0 s of the two: past
eight processes a split by program buys nothing, and a split by (program, output) is what the cores can use.

**Duplication, read from the code.**

- `jobs_invariance` runs all 41 outputs at `--jobs 1` and again at `--jobs 2`, but `corpus_ops::run_with` hands
  the job count only to the 14 region outputs (5 read-out modes, the region trace, the states, the lattice, 5 core
  sets, the oracle); the other 27 (parse and the IR modes, load, saturate, plan, match, hypotheses, NAF, explain,
  the 14 DOT views) are one computation run twice, as its own note says (*"the ops that cannot differ by job
  count — every frontend one"*).
- `id_order_invariance` and `jobs_invariance` each compute afresh the baseline `corpus_shapes` already computes
  and banks as a digest (`golden/corpus_shapes.md5`): two of the five passes over the corpus.
- Every output parses and loads its program again, stdlib modules included: a calculator program is loaded about
  205 times by the three sweeps.
- Every sweep takes every program: 339 witnesses under `examples/ein-bugs/`, 66 that do not parse, and the 220
  under `tests/`, each written for one stdlib rule's claim.
- The nightly's `stdlib_mutants.py --check` runs 572 mutants one after another, each `ein test tests/`: about
  49 min, against [Q-M1f.102](../open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)'s
  ruling that a nightly step takes at most 5 min on the runner.

## Tasks

1. **T1f.6.39.1 — the baseline, on AC power.** `utils/bench_env.sh` reports the power supply
   (`/sys/class/power_supply/*/online`) beside the cores, and warns on battery. Then, on AC: `./run_tests.sh`, the
   nightly and `utils/ci_container.sh` timed per binary and per step; the three A sweeps' cost per (program, output);
   the peak resident memory of each corpus program under its heaviest output, which sets the worker pool's cap; the
   runner's cores from a workflow log (`nproc`). This is the stage's *before*, recorded with the commands.
2. **T1f.6.39.2 — `jobs_invariance`, once each, against the golden.** Which outputs take the job count is read off
   the code, not written in a list: a predicate on `corpus_ops::Op` used by `run_with` itself, with a test that an
   output outside it ignores its `jobs` argument. Each such output runs at each `EIN_JOBS_SWEEP` count, its digest
   held to `corpus_shapes`' golden; only a mismatch computes the `--jobs 1` run, to report the first difference
   through the answer/narration cut, as today. `the_sweep_is_not_vacuous` kept.
3. **T1f.6.39.3 — `id_order_invariance` against the golden.** The shuffled run's digest held to the golden; a
   mismatch computes the ordinary run and applies today's cut (the narration differences it admits are found
   that way, as now). `EIN_ID_SEEDS`, `EIN_ID_FILES` (which has no golden: there the baseline is computed) and the
   register's id-order rows unchanged.
   **T2 and T3 are verified by sensitivity, not by argument** ([`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)):
   a probe that makes one output depend on the id order, and one that makes one depend on the job count, each fails
   the test before the change and after it, the probes kept as a test or banked in the record.
4. **T1f.6.39.4 — a worker pool.** One helper in `ein-corpus`: a sweep's work is its (program, output) units, run on
   `EIN_TEST_JOBS` workers (default 16, at most the machine's threads), results sorted by (program, output) before
   any report, so a failure's text is the same at any count; a program T1 measured as heavy runs in a capped lane so
   the pool stays under a stated memory bound. Applied to the A sweeps and to every B sweep of 20 s or more, and to
   the nightly's deep-corpus tests. `utils/stdlib_mutants.py --jobs N`, each mutant in its own tree, its report in
   the order of the mutants. No new crate outside the workspace's lock file. **A test's verdict does not depend on
   `EIN_TEST_JOBS`**: the gate run once at 1 and once at the default, every verdict equal, and a failure injected in
   each pooled sweep reported with the same text at both.
5. **T1f.6.39.5 — the gate profile.** `[profile.gate]` in `ein.rs/Cargo.toml`: `inherits = "release"`,
   `debug-assertions = true`, `overflow-checks = true` (snmalloc's package override as dev and profiling have it).
   `./run_tests.sh`, `per-commit.yml`, the nightly's steps and `ci_container.sh` run `cargo test --profile gate`;
   `gate_steps.rs` and `nightly_steps.rs` keep the lists equal; clippy and rustdoc unchanged; anything that names
   `target/debug` follows. Verified: the corpus answers alike under the dev build and the gate build
   (`ein solve --json-summary` and `ein test`, every corpus program), and a `debug_assert!` still fires under the
   gate profile (a probe).
6. **T1f.6.39.6 — the audit of what each sweep guards, and the stop.** For each of the 31 corpus sweeps of A and B
   and each nightly step: what it claims; what a change must touch to break it; which other sweep already holds the
   same claim (a matrix); its cost after T2 to T5; and which programs it needs, all or a named set (programs that
   search, read an `absent`, owe, or exercise a family). **Every proposed cut is shown by a probe, not argued**: for
   each sweep, faults it catches today (the register's witnesses, injected mutations of the engine), and whether the
   reduced set still catches each. The candidates, from [§ Today](#today-the-analysis-of-2026-11-15): the
   determinism sweeps over the programs that search rather than every claim fixture; `tests/` programs under
   `ein test`, the coverage census and the mutants, and not under the render and determinism sweeps; the five
   sweeps that set modes and routes against one another or the oracle (`explore`, `lever_invariance`,
   `route_invariance`, `region_modes`, `claim_routes`); one load per program shared by the outputs that do not
   change it; the nightly's steps against Q-M1f.102's five minutes. **Stops for the user** with the audit, each
   cut's probe, and the times it would leave.
7. **T1f.6.39.7 — the cuts, as ruled.** Written when the user has ruled on T6; a cut that moves a test to another
   tier names it in `nightly_steps.rs`' `DEV_HOST_ONLY` with why.
8. **T1f.6.39.8 — the pages and the record.** `AGENTS.md` § Running the gate (the profile, `EIN_TEST_JOBS`, the
   commands), `run_tests.sh`'s header, `configuration.md`'s census (`EIN_TEST_JOBS`, pinned by
   `config_reference.rs`), `tests/README.md`, `corpus/README.md`'s table of readers. The record: before and after,
   on AC, per binary and per step, here and in the runner's image; and the runner's times from the next pushes.

## Acceptance

- **Through T5, every check the gate made is still made**: each of T2 to T5 shown by a probe that fails before and
  after; the corpus's answers and every test's verdict unchanged.
- **No verdict depends on the worker count**, and a failure reads the same at any count.
- **The gate and the nightly are timed before and after on AC power**, here and in the runner's image, with the
  slowest single unit named; the pool's peak memory measured under its bound.
- **The nightly's steps are held to Q-M1f.102's five minutes**, or each step over it is named with the user's
  ruling at T6.
- After T7, the tiers as the user ruled.

## What it must not do

- **Not change the engine** or what any command answers: test code, scripts, workflows, the profile and pages only.
  A wrong answer met on the way is a register row, filed first.
- **Not drop a check before the user's ruling** (T6): T2 to T5 make the same checks cheaper, nothing more.
- **Not add a tool the gate must install** (no test runner from outside the workspace) and nothing upstream.
- **Not read timings taken on battery**, or on a core the record does not name.

## The standing rules

A wrong answer is a row of `corpus/wrong_answers.toml` with its witness, filed `open` in a commit of its own.
Findings go to [`findings.md`](findings.md) (E-72 the next free), questions to
[`open_questions.md`](../open_questions.md) (Q-M1f.284 the next free), each default the user's to revise. Commit only
on master and only the stage's own paths. Every number carries its command, its date or the test that pins it.
`utils/ci_container.sh` is part of done (the workflows change).
