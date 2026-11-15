# S1f.6.39 — The gate made fast: each check once, in parallel, at release speed; then what each invariance needs

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-15, on the user's word of that day ([§ The instruction](#the-instruction)). T1 to T5
keep every check the gate makes and run on defaults; **T6 stops for the user** with the audit of what each sweep
guards and needs; T7 applies what the user rules. **T1 to T5 built 2026-11-15, and T6's audit waits for the user**
([§ Record](#record-2026-11-15-t1-to-t5-built-t6s-audit-and-the-stop)): Q-M1f.287 to .294.
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

## Record, 2026-11-15: T1 to T5 built, T6's audit, and the stop

Commits: `13cdfbb1` (T4, the pool and the pooled sweeps), `d5cb8930` (T2 and T3), `f137d1cc` (T5, the gate
profile), `6d258219` (T1's two tools, and `574fa8d3` two fixes to one), `b8c931f3` (T8, the pages); this record and the audit
in the commit after. Every run below is on this machine, an i9-14900HX, **unpinned on all 32 CPUs** (CPUs 0–15 the
eight P-cores' threads, 16–31 the sixteen E-cores) unless it names its cores, **on AC throughout**: `utils/bench_env.sh --cores 0-31 --report` before each run
(`build/s1f.6.39/*env*.txt`, every one `power AC (ACAD online=1)`), and `utils/gate_times.py run`'s once-a-second
samples of the mains supply, which read `1` in every sample of every log below. Logs and TSVs are under
`build/s1f.6.39/`; `python3 utils/gate_times.py report LOG` re-reads each. The machine held a 6.4 GB tmpfs leftover
of an interrupted `stdlib_census.py` run of 2026-11-14 (`/tmp/ein-stdlib-census-5f5avsni`) throughout, not touched.

### T1 — the baseline, on AC

**The tools.** `utils/bench_env.sh` reports the power supply beside the cores (every `Mains` supply's `online`,
each battery's state), says `ON BATTERY` and warns there, and `--require-ac` exits 3 on battery before the command.
`utils/gate_times.py` is the measurement: `run --out LOG -- CMD` stamps every line and samples the machine (memory in
use, the mains supply, and the run's own process tree's resident memory); `report LOG` gives every step and every test
binary with its wall time and peak memory, and reads a GitHub log too.

**Before, on AC** (`d7352fce`, 2026-11-15):

- **The gate in the runner's image**, `utils/ci_container.sh` (Ubuntu 26.04.1 amd64v3, Graphviz 14.1.2, its target
  warm): **68.3 min** (4 097.8 s), exit 0; `cargo test --workspace` 4 079.0 s of it, the six static checks 14.4 s, the
  smoke and the bench 3.8 s (`build/s1f.6.39/before-ci.log`). Gate #3's 99 min was the battery's: its first 38 min,
  on AC, read as this run does (`explore` 376.4 s here, 378 s there; `import_invariance` 163.8 s both).
- **The nightly here**, `./run_tests.sh --nightly` from a clone at `d7352fce` (release, warm): the eight
  deep-corpus test steps **895.6 s** together (`build/s1f.6.39/before-nightly-deep.log`); the mutation census,
  serial, did 72 of its 572 mutants in 376.2 s, 5.2 s a mutant, about 50 min for all, and was stopped there to free
  the machine for the build (gate #3's whole census took about 49 min).
- **The runner's cores**: no step prints `nproc` (`gh run view 43628514197 --log`, the per-commit run of
  `723dfd6e`); the log's own header names the image, `ubuntu-26.04` 20261111.82.6, and the host, and GitHub's
  standard runner for a public repository is four vCPUs and 16 GB, the four Q-M1f.103's measurement took as the
  runner's count. The pool's cap makes four its worker count there.

**What each (program, output) costs** — `EIN_TEST_JOBS=1 EIN_TEST_UNITS=… cargo test -p ein-render --test
corpus_shapes` at the dev profile, the pool's instrument, one line a unit (`build/s1f.6.39/units-dev-shapes.tsv`):
33 497 units, 428.6 s. By output: the five core sets 19.5 %, the five read-outs 19.4 %, the fourteen DOT views 15.0 %,
the region trace 7.0 %, the states 5.6 %, the lattice 4.0 %, the four IR modes 3.9 %, saturate 3.2 %, the two explains
5.8 %, the two hypothesis shapes 4.8 %, load 2.6 %, the oracle 2.5 %, match, NAF and plan 6.7 %: **the fourteen
outputs a job count reaches are 58.1 %**. The longest unit is 2.40 s (`zebra2-hints.ein`'s maximal read-out). Of a
pass, **import resolution is most of every loading output's cost**: `ir[resolve]`, which reads and parses the
program's stdlib modules and nothing more, is 8.3 s of the pass, where `plan` (resolve, load, the plan) is 9.1 s and
`naf` 9.7 s — about 30 of the 41 outputs resolve again (Q-M1f.292). At the gate profile the same pass is 338.1 s
(`units-gate-j1.tsv`), 1.27 times faster serially.

**Peak memory.** In-process, one worker, no `corpus_shapes` unit took the test process past 272 MB (`VmHWM`). Through
the CLI, `build/s1f.6.39/cli_rss.py` (`os.wait4`'s `ru_maxrss`, `ein solve FILE` at the default region budget and
`ein oracle FILE`, every corpus program): `send_more_money` 529 MB, `features/04_open` 266 MB, every other program
187 MB or less. Those two are the heavy lane, `pool::HEAVY`; the bound it keeps is loose (below).

### T2 — `jobs_invariance`, the outputs a job count reaches, against the golden

`corpus_ops::Op::takes_jobs` names the fourteen outputs a job count reaches — the five read-outs, the region trace,
the states, the lattice, the five core sets and the oracle — and `corpus_ops::run_with` **dispatches on it**: an
output inside goes to `region_op` with the count, an output outside to `run_frontend`, which has no count to read. The
set the count reaches is the code's, not a list beside it. The DOT views took a count until M1e2 S1e2.1c.5; the
test's `solves()` still counted them as solving.

The sweep runs those fourteen at each `EIN_JOBS_SWEEP` count, a (file, output, count) a unit, and digests each run as
`corpus_shapes` does — the same bytes, narration and all, the checkout made portable (`corpus_ops::digest`,
`portable` and `manifest_line`, moved there so the three sweeps share one definition) — against the golden's line for
the pair (`corpus_ops::golden_digests`). Equal: the same bytes as `--jobs 1`, which is not run again. Unequal or
missing: `--jobs 1` is computed and the first difference reported through the answer/narration cut, as before. The
premise, that the golden is today's, is `corpus_shapes`' to fail on in the same gate; a stale golden costs this test
time and never a verdict, since a mismatch is what computes the baseline (the argument is beside the code, in the
file's note, as the standard of proof asks). `the_sweep_is_not_vacuous` is kept, and the floor of 1 000 cells now
counts cells of the fourteen. New: `an_op_outside_the_predicate_ignores_jobs` (the 27 others render alike at `--jobs
1` and `--jobs 7` on five programs that search, refuse and fail to parse, and the predicate holds fourteen) and
`a_job_count_that_reaches_an_output_is_caught` (below).

### T3 — `id_order_invariance`: the plain run stays (E-72)

The shuffled id space is a permutation of the **plain run's own** `Terms`, every name and fact that run interned,
the facts its forks derived among them. No digest holds that table, so the shuffled run cannot start without the
plain one, and the plain run's rendering comes with it: holding the shuffled run to the golden saves nothing. A
permutation of a smaller table, the load's, would leave the derived facts in their derivation order under a uniform
offset, which the file's own note calls no permutation, and M1e2 S1e2.2.12's leak was in those facts. So T3 is the
pool alone: a (file, output) a unit, every count, floor and the first twenty reports the serial loop's;
`EIN_ID_SEEDS`, `EIN_ID_FILES` and the register's id-order rows unchanged (E-72, Q-M1f.286 (a) defaulted). What
would remove a pass is to digest this plain run against the golden and drop `corpus_shapes`' own pass, which changes
what a binary checks: T6's Q-M1f.287.

**T2 and T3 by sensitivity**, the two probes of `build/s1f.6.39/probes/apply_probe.py`, each a splice into
`corpus_ops/mod.rs` of a scratch tree, never committed:

| probe | the test before (`d7352fce`, release) | the test after (`b8c931f3`, gate profile) |
|---|---|---|
| the describe read-out writes `--- probe: jobs N` when N > 1 | `jobs_invariance` fails: *575 of 26 665 (file, op, jobs) cells changed the ANSWER* | fails: *575 of 8 316 (file, op, jobs) cells changed the ANSWER* — the same 575, of the fourteen outputs' cells |
| the load's rendering writes the name holding the first id after the kernel's | `id_order_invariance` fails: *591 of 26 665 (file, op) pairs move when the ids do* | fails: *591 of 26 665 (file, op) pairs move* |

Kept as tests, on one program each: `jobs_invariance::a_job_count_that_reaches_an_output_is_caught` (an answer
moved, with the golden and without it) and `id_order_invariance::an_id_order_that_reaches_an_output_is_caught` (the
honest run moves nothing, the leaky one moves the answer). Outputs: `build/s1f.6.39/probe-{before,after}-{jobs,id}.txt`.

### T4 — the worker pool

`ein_corpus::pool::map(sweep, units, key, f)` (`ein-corpus/src/pool.rs`, std threads, no crate added): `f` over every
unit on `EIN_TEST_JOBS` workers, default 16, never more than the machine's threads (a value that is not a count of one
or more is a panic, not the default), and the results **in the units' order**. A sweep's loop body became `f`, an
`Fn` that cannot write what another unit reads, and its fold is the serial loop's: every count, floor, failure line
and its order. One semaphore a process, so pooled tests of one binary run at most `EIN_TEST_JOBS` units between them;
a `map` inside a unit runs inline. A panicking unit is held by a hook installed once (other threads' panics go to the
hook that was there) and re-raised after every unit ran, as the first in order, its message printed once. Units start
heavy first (`pool::HEAVY`, a lane of a quarter of the workers), then `pool::LONG` (the 18 programs with a unit of 5 s
or more at one worker, `units-gate-j1.tsv`), then the manifest's `cost_ms`; no order reaches a result.
`EIN_TEST_UNITS=<file>` writes each unit's milliseconds and, at one worker, its own peak (`/proc/self/clear_refs`,
then `VmHWM` less the resident set it started from). Five tests in `pool.rs`: the order, the first panicking unit,
the inline map, and the two lists name corpus files. `EIN_TEST_JOBS` and `EIN_TEST_UNITS` are rows of
`configuration.md`'s harness class, which `config_reference.rs` now pins against what the two dev-only crates read.

On the pool: the three A sweeps; `explore`'s three corpus sweeps; `import_invariance`; ein-infer's `read_out`;
`summary_properties`; `lever_invariance`'s four maps; `corpus_cli`; `answer_dump`; `answer_round_trip`'s eight shards;
`corpus_cores`' two; `region`'s five oracle sweeps; einb's `roundtrip`; `arith_search_free` (a scratch a program: two
programs of one stem shared one file name); `route_invariance`; `region_modes`' verdict sweep; `claim_routes`;
`claim_round_trip`; `alive_invariant`; `interning`; `kb_semantics`' and `load_semantics`' corpus sweeps — every corpus
sweep of 20 s or more in the before-run but `dot_wellformed`, which already ran its files on its own ordered threads,
and the nightly's deep-corpus tests, which are the same binaries. Three of them listed failures in the order their
own threads finished (E-73). `import_invariance` also split every prefix into characters at every boundary of every
string: `features/04_open.ein` was 127.3 s of its 129 s at sixteen workers, 11.3 s since (E-73).
`utils/stdlib_mutants.py --jobs N` (default `$EIN_TEST_JOBS`, else 16, at most the machine's threads), each mutant in
its own copy of `stdlib/`, results in the mutants' order before anything is printed or compared, the work directory
removed however the sweep ends.

**No verdict depends on the count.** The whole workspace at the gate profile, `cargo test --profile gate --workspace
--no-fail-fast`, at `EIN_TEST_JOBS=1` (65.5 min, `try-1.log`) and at 16 (16.5 min, `try-16.log`): **1 207 tests, each
`ok` in both**, no verdict different. **A failure reads the same at any count**: `build/s1f.6.39/probes/same_text.py`
splices a panic into every parse of two programs (`examples/branching/02_one_dead_one_alive.ein`,
`examples/lattice/02_genuine_3set_death.ein`) and runs each of the 23 pooled sweep binaries over a 44-program set at
one worker and at sixteen: 22 fail, `arith_search_free` holds neither program, and **every failing test's own text is
the same at both counts** (`probes/compare_text.py`, 2026-11-15). What differs between two runs of a binary, at one
count as at two, and is set aside by the comparison: the order in which libtest lists failing tests (the order they
finished), the default hook's one `RUST_BACKTRACE` note (after the first test to panic), cargo's own build lines, and
the seconds `claim_routes` prints beside a slow program.

**The pool's memory.** Per binary at sixteen workers, the run's own process tree (`gate_times.py`): `id_order_invariance`
1.80 GiB, `corpus_shapes` 1.61, `lever_invariance` 1.52, `explore` 1.28, every other pooled sweep under 1.2 GiB; the
gate's largest binaries are feature tests that the pool does not run (`arith_theory` 3.64 GiB, `int_theory` 3.25,
`arith_batch` 2.77), and the run's peak, 7.65 GiB, was rustc's, compiling. The nightly's peak, 4.24 GiB, is
`answer_round_trip`'s slow half writing its `heavy_dump` dumps. All under the 24 GB bound by more than a factor of
three (Q-M1f.284 (a) defaulted).

### T5 — the gate profile

`[profile.gate]`: `inherits = "release"`, `debug-assertions = true`, `overflow-checks = true`, and snmalloc's package
override (release's allocator). Every `cargo test` of `run_tests.sh`, `--nightly` and `--dev-host`, `per-commit.yml`
and `nightly.yml` is `cargo test --profile gate`; `gate_steps.rs` and `nightly_steps.rs` keep the lists equal, their
`--release` pins read `--profile gate`. `review_bundle.py --smoke` drives `target/gate/ein`, and the nightly's census
`--bin ein.rs/target/gate/ein` (Q-M1f.285 (a) defaulted). `ci_container.sh` needed nothing. `rust-cache`'s key hashes
every `Cargo.toml`, per its documentation, so the first CI run after the profile builds `target/gate/` cold and caches
it. `release.yml` still tests `--release`, the binary that ships.

- **The corpus answers alike**: every corpus program, `ein solve FILE --json-summary` (stdout, stderr, exit, the
  summary) and `ein test FILE` (stdout less its wall clock, exit), with `target/debug/ein` and `target/gate/ein`:
  **818 of 818 alike** (`build/s1f.6.39/profiles_alike.py`, 2026-11-15).
- **The checks fire**: a probe at the top of `explore::run_with`, in a scratch tree, under an environment variable —
  `debug_assert!(false, …)` exits 101 with its message from the gate binary and 0 from the release one; `u8::MAX + 1`
  exits 101 (*attempt to add with overflow*) from the gate binary and prints `255 + 1 = 0` from the release one.
- **The build**: every test target at the gate profile, cold, 55.0 s here and 37.8 s in the image; serially the
  corpus is 1.27 times faster than at the dev profile (`corpus_shapes` at one worker 444.8 s before, 349.9 s at the
  gate profile).

### Before and after, on AC

**The gate**, `./run_tests.sh`:

| | before | after |
|---|---:|---:|
| in the runner's image (`utils/ci_container.sh`) | 68.3 min (4 097.8 s), `d7352fce` | **15.2 min** (910.5 s), `b8c931f3`, its gate build cold (37.8 s) |
| here | — (not re-run; the image's run is this machine's) | **15.0 min** (899.1 s), `b8c931f3`, 33.0 s of rebuild |
| of which the tests | 4 079.0 s | 881.4 s in the image, 886.8 s here |

Per binary, the 24 slowest before (`python3 build/s1f.6.39/compare.py`):

| binary | before, image | after, image | after, here | its peak here |
|---|---:|---:|---:|---:|
| `jobs_invariance` | 900.8 s | 19.5 s | 20.2 s | 1.66 GiB |
| `id_order_invariance` | 854.4 s | 65.5 s | 65.1 s | 1.80 GiB |
| `corpus_shapes` | 444.8 s | 30.4 s | 30.6 s | 1.61 GiB |
| `explore` | 376.4 s | 46.7 s | 50.4 s | 1.28 GiB |
| `import_invariance` | 163.8 s | 12.1 s | 10.2 s | 0.97 GiB |
| `read_out` (ein-infer) | 100.5 s | 16.1 s | 14.9 s | 0.69 GiB |
| `summary_properties` | 98.4 s | 6.2 s | 6.0 s | 0.27 GiB |
| `int_theory` | 90.5 s | 65.2 s | 65.5 s | 3.25 GiB |
| `lever_invariance` | 78.5 s | 45.1 s | 45.0 s | 1.52 GiB |
| `corpus_cli` | 73.6 s | 4.2 s | 4.2 s | 0.53 GiB |
| `answer_dump` | 61.4 s | 34.1 s | 33.8 s | 0.76 GiB |
| `corpus_cores` | 50.8 s | 41.8 s | 42.9 s | 0.74 GiB |
| `region` | 50.4 s | 20.7 s | 24.3 s | 0.92 GiB |
| `arith_batch` | 49.2 s | 35.9 s | 36.0 s | 2.77 GiB |
| `roundtrip` (einb) | 46.2 s | 3.0 s | 3.0 s | 0.28 GiB |
| `arith_search_free` | 37.9 s | 2.6 s | 2.6 s | 0.42 GiB |
| `route_invariance` | 34.0 s | 26.8 s | 26.8 s | 1.13 GiB |
| `read_out_reference` | 27.4 s | 23.5 s | 23.3 s | 0.69 GiB |
| `answer_round_trip` | 25.8 s | 10.6 s | 12.2 s | 0.97 GiB |
| `claim_round_trip` | 24.1 s | 2.0 s | 2.2 s | 0.48 GiB |
| `kb_semantics` | 24.0 s | 1.6 s | 1.6 s | 0.19 GiB |
| `load_semantics` | 23.7 s | 7.0 s | 7.0 s | 0.19 GiB |
| `alive_invariant` | 23.6 s | 6.9 s | 6.9 s | 0.26 GiB |
| `read_out_examples` | 23.3 s | 18.7 s | 18.7 s | 0.21 GiB |

All 135 binaries: 4 074 s before, 839 s after in the image, 849 s here. **The slowest single unit** of the gate is
`corpus_cores`' `statuses` on `tests/stdlib/bijection/06_blind_enumeration.ein`, 36.0 s at one worker
(`units-gate-j1.tsv`), which is most of that binary's 42.9 s; before the prefix fix it was `import_invariance` on
`features/04_open.ein`, 127.3 s. What is left long per commit is the feature tests the pool does not reach
(`int_theory`, `arith_batch`, `read_out_reference`, `read_out_examples`, `arith_theory`: 162 s together) and the
sweeps whose one program dominates (`corpus_cores`, `answer_dump`'s `square-unique/corner-house` dump, 33.2 s,
`route_invariance`'s zebra family, 21 to 25 s a program): T6.

**The nightly**, `./run_tests.sh --nightly`, here (`after-nightly.log`, exit 0): **16.7 min** (1 000.9 s), the census
461 of 572 mutants killed and the banked set held. Before, its test steps, fuzz and bench were 932.0 s together
(`before-nightly-deep.log`, and `before-nightly-tail.log` for the fuzzer and the bench, run from the clone after the
fact), and its census about 50 min more: about 66 min.

| step | before (release) | after (gate profile, the pool) |
|---|---:|---:|
| `corpus_cli`, the slow entries | 169.6 s | 22.7 s |
| `answer_dump` | 45.4 s | 35.1 s |
| `answer_round_trip`, slow and `heavy_dump` | 167.7 s | 116.5 s |
| `lever_invariance`, the slow cells | 55.9 s | 47.2 s |
| `corpus_cores`, the slow entries | 222.3 s | 224.5 s |
| `region`, the slow half | 57.0 s | 53.9 s |
| `obligation_rung`, the slow half | 71.6 s | 76.7 s |
| `import_invariance`, the slow entries | 106.1 s | 53.4 s |
| `stdlib_mutants.py --check` | about 50 min serial (72 of 572 in 376.2 s) | 332.8 s, sixteen at once |
| the fuzzer, seeds 1 and 987654321 | 24.4 s and 11.0 s | 25.6 s and 11.4 s |
| the bench smoke | 1.0 s | 1.0 s |

### T6 — the audit of what each sweep guards, and what it needs

**The sweeps.** § Today counted 31 by cost (gate #3's binaries of 10 s or more); read off the code, the corpus sweeps
are the binaries with a test that walks every corpus program, every manifest cell or every register witness. Each
row: what it claims; what a change must touch to break it; the programs it walks; its cost after T2 to T5 (`./run_tests.sh`
here at `b8c931f3`, `after-gate.tsv`).

| sweep | claims | breaks on a change to | walks | after |
|---|---|---|---|---:|
| `corpus_shapes` | every rendering of 41 outputs is the banked one: a change detector, not a judge | any writer an output reads: parser and dumpers, loader, saturation, the routes, `ein-render` | every corpus file | 30.6 s |
| `id_order_invariance` | no rendering depends on the order ids were assigned in | an iteration or sort by `Symbol`/`FactId` number that reaches an output | every file, every output that interns | 65.1 s |
| `jobs_invariance` | `--jobs N` renders `--jobs 1`'s bytes | the fan-out and its ordered commit (`region::fan_out`, the routes over it, `Events::replay`) | every file, the 14 outputs a count reaches | 20.2 s |
| `explore` (3 sweeps) | every cell of `ein solve`'s axes is the oracle's; a run without cores answers alike; describe without N is describe less N | the routes (`downset`, `states`, `tree`), `explore`, the oracle | corpus + register within the oracle's limit | 50.4 s |
| `region` (5 sweeps) | route 1, the state route and L5 are the oracle; fact grain never calls a violated cube monotone; monotone makes saturation a closure | `downset`, `states`, `oblgen`, `monotone`, the oracle | corpus + register within the limit | 24.3 s |
| `route_invariance` | route 1 and the state route print one read-out, the state route never doing more | `downset`, `states`, the read-out | every monotone program both finish, the zebra family's included | 26.8 s |
| `claim_routes` | every claim is decided alike by its least question, the description and the oracle | `claim.rs`'s routing, the routes | corpus + register | 15.3 s |
| `region_modes` (corpus test) | `--monotone`'s verdict and sites are the read-out's; `ein test` agrees | `--monotone`, the lattice row | every file | 11.7 s |
| `lever_invariance` | no config flag or declared lever moves a line of any mode; `--no-search` reports root as the full run reaches it | anything that reads `SolverConfig` on the way to an output | every program whose R is known; the declared cells | 45.0 s |
| `read_out` (ein-infer) | each listed state's model is the oracle's; classes partition; a claim checked twice says one thing | `read_out.rs`, `claim.rs` | corpus + register within the limit | 14.9 s |
| `corpus_cores` | every core reported names its commitment and derives the contradiction; none printed unqualified from a cut search | core extraction and its printers | corpus + register, not `slow` | 42.9 s |
| `summary_properties` | the summary's identities on five questions | `summary.rs`, the routes' counts | every file | 6.0 s |
| `import_invariance` | an importing program answers alike under every import form | `imports.rs`, the loader's names, the summary's spellings | the ~270 programs that import, not `slow` | 10.2 s |
| `corpus_cli` | every declared cell's exit code is banked; no crash; artefacts written; the `slow` flag holds | the CLI, its exit codes; every command | every manifest cell, not `slow` | 4.2 s |
| `answer_dump` | every recorded run's state files are fixpoints of their program | the state writer | the register's runs, seven cases | 33.8 s |
| `answer_round_trip` | every state file reloads to its own facts | the dump writer, the parser, the loader | every entry, not `slow` or `heavy_dump` | 12.2 s |
| `claim_round_trip` | a printed region reads back as its own claim | `claim::print`, `claim::check` | files with no claim | 2.2 s |
| `wrong_answers`, `register_region` | every register row answers what it records; its region is its witness's R | whatever a row's question reads | the register | 14.0, 6.4 s |
| `roundtrip` (einb), `kb_semantics`, `load_semantics`, `alive_invariant`, `interning`, `detector`, `line_modes_past_the_limit`, `arith_search_free`, `test_cli`, `dot_wellformed`, and the floors (`hypgen_coverage`, `refutation_under_absent`, `compile_limits`, `compile_semantics`, `template_wellformedness`, `stdlib_coverage`) | a container reopens to its KB; the KB's shape and every refusal is id-free; `:why` decides loading only; no run reaches a name the program lacks; the search interns nothing; the detector's halves; line modes past the limit; arithmetic is computed; `ein test`'s rows; Graphviz accepts every view; each floor's coverage | `ein-einb`, the loader, hypgen, the detector, the calculator, the DOT writers, the front end | every file (`stdlib_coverage`: `tests/`; `arith_search_free`: the arithmetic roots) | 1.6 to 15.2 s each |

**The probes.** Six engine mutations, each a splice into a scratch clone of `b8c931f3`
(`build/s1f.6.39/probes/mutations.py`, never committed), each run against every sweep above over the whole corpus,
and the render and determinism sweeps over two reduced sets (`probes/sets.py`: **the 188 programs that search**,
whose describe saturates more than root, and **the 597 outside `tests/`**), by `probes/t6.sh` and `run_probe.py`, the
outputs in `build/s1f.6.39/t6-out/`. A mark is a failing test of that binary:

| sweep | `id-leak` | `jobs-leak` | `neq-nested` | `route1-maximal` | `states-maximal` | `maximal-drop` |
|---|:-:|:-:|:-:|:-:|:-:|:-:|
| `corpus_shapes` | ✗ 273 renderings | | ✗ 651 | | ✗ | |
| `id_order_invariance` | ✗ 483 of 26 665 pairs | | | | | |
| `jobs_invariance` | | ✗ 987 of 8 316 cells | | | (✗ its probe test, by the golden) | |
| `kb_semantics` | ✗ 556 of 817 files | | | | | |
| `explore` | | ✗ (at 4 and 8 jobs) | | | ✗ | |
| `region` | | ✗ (`jobs_change_nothing`, two oracle sweeps at 8 jobs) | | ✗ route 1 = oracle | ✗ state route = oracle | |
| `register_region` | | ✗ | | | ✗ | |
| `region_modes` | | ✗ (`jobs_change_no_line_of_a_mode`) | | | ✗ | |
| `line_modes_past_the_limit` | | ✗ | | | | |
| `route_invariance` | | | | ✗ (programs past the oracle's limit) | ✗ | |
| `claim_routes` | | | | | ✗ | |
| `corpus_cli`, `test_cli` | | | ✗ (`ein test` exits) | | ✗ | |
| `arith_search_free` | | | ✗ | | | |
| the 18 others | | | | | | |

- `id-leak`: `ein_core::shape` lists relations in `Symbol` order, not declaration order.
- `jobs-leak`: `region::fan_out` swaps a fanned-out step's first two nodes when a pool runs it.
- `neq-nested`: `(neq F F)` holds for two equal nested facts — a wrong answer every route shares.
- `route1-maximal`: route 1's description drops the first state of its maximal line.
- `states-maximal`: the state route's description drops the first state of its maximal line when it holds two.
- `maximal-drop`: a line mode past the limit on a flagged program drops its first maximal state. **Caught by
  nothing, and equivalent**: that path certifies no maximal state at all (`explore.rs`' note on it), so the
  mutant changes no output; kept in the record as the control that a mark means something.

**What the probes show.** The invariance sweeps catch exactly their own kind and nothing else: no invariance sweep
caught `neq-nested`, a wrong answer the routes share, which the goldens (`corpus_shapes`) and the claims (`corpus_cli`'s
and `test_cli`'s `ein test` cells, `arith_search_free`) caught. A job count's leak is caught by six binaries, five of
them on fixed programs at four or eight jobs, and over the corpus only by `jobs_invariance`. An id leak is caught by
`id_order_invariance`, `kb_semantics` (the load, every file) and the golden. A state-route bug is caught by nine;
route 1's by two, and by `route_invariance` on programs past the oracle's limit, where nothing else holds route 1.
`lever_invariance`, `import_invariance`, `summary_properties`, `answer_dump`, `answer_round_trip`, `corpus_cores`,
`claim_round_trip` and `read_out` caught none of these six: each guards a claim of its own (config, import forms,
identities, dumps, cores, claim printing), and a probe of each kind is what would show which of them share one.

**Over the reduced sets** (the sweeps' own failures, each set's golden and Graphviz skip list set aside: over a subset
`corpus_shapes` and `dot_wellformed` fail on the lines the set leaves out, and a cut re-banks both):

| probe | `id_order_invariance` | `jobs_invariance` | `corpus_shapes` (renderings moved) |
|---|---|---|---|
| `id-leak` | all 483 of 26 665; **search** 144 of 7 708; **no-tests** 269 of 17 645 | — | moved in all three |
| `jobs-leak` | — | all 987 of 8 316; **search 983 of 2 632**; **no-tests** 942 of 5 236 | — |
| `neq-nested` | — | — | moved in all three (`examples/arith`, `beyond_arith`) |
| `states-maximal` | — | (the probe test) | moved in all three |

Every probe a full-corpus determinism or render sweep catches, it still catches over both reduced sets; and 983 of
the 987 cells a job count's leak moves are in the 188 programs that search. What the sets lose is not a probe's
answer but coverage no probe here exercised: the id order of the 33 frontend and shape outputs on the 629
programs that do not search (the load's is `kb_semantics`' too), and `tests/`'s 220 programs under the renderings,
which `corpus_cli`, `test_cli` and the census check by their claims.

**The costs the sets leave**, the reduced sets run on the unmutated tree (`t6.tsv`, the `none` rows): `id_order_invariance`
65.4 s → 14.6 s over the search set, 30.7 s outside `tests/`; `jobs_invariance` 20.9 → 7.5 and 11.7 s; `corpus_shapes`
29.5 → 7.4 and 15.7 s; `dot_wellformed` 13.4 → 3.7 and 8.4 s. Per unit at one worker (`units-gate-j1.tsv`), the
programs under `tests/` are 48 % of `corpus_shapes`' work and 52 % of `id_order_invariance`'s; the programs that
search 23 % and 24 %. Of a `corpus_shapes` pass, **import resolution is about 61 %** (`ir[resolve]` 6.2 s of the
gate profile's pass, and about 30 of the 41 outputs resolve again), the shared-load candidate below.

**The nightly against Q-M1f.102's five minutes.** The runner has four vCPUs, so a step's time there is read here on
four P-core threads with four workers, as Q-M1f.103 measured the fuzzer: `taskset -c 0-3 env EIN_TEST_JOBS=4
./run_tests.sh --nightly` (`build/s1f.6.39/nightly-4threads.log`, cores `0-3`, two physical P-cores, AC throughout):

| step | four P-core threads, four workers | over 5 min? |
|---|---:|---|
| `corpus_cli`, the slow entries | 77.6 s | no |
| `answer_dump` | 35.8 s | no |
| `answer_round_trip`, slow and `heavy_dump` | 353.0 s | **yes** |
| `lever_invariance`, the slow cells | 202.6 s | no; about 230 to 270 s at the runner's 1.13 to 1.34 times |
| `corpus_cores`, the slow entries | 376.3 s | **yes** |
| `region`, the slow half | 271.7 s | **on the runner, by that factor**: about 307 to 364 s |
| `obligation_rung`, the slow half | 76.3 s | no |
| `import_invariance`, the slow entries | 86.9 s | no |
| `stdlib_mutants.py --check` | 1 382.9 s | **yes**, 23 min |
| the fuzzer, each seed | 24.7 s, 21.5 s | no |
| the bench smoke | 1.0 s | no |

The deep-corpus job is 47.7 min of its 48.5 (the three jobs run side by side on GitHub). Every test step is one
binary, and what keeps the four over the line is a handful of programs each: the zebra family's cores and dumps, the
`heavy_dump` entries' tens of thousands of state files, `region`'s zebra2-minus-15 family; and the census's 572 runs
of `ein test tests/`, 9.7 s each on a worker of four. Before the change the census alone was about 50 min, serial.

**The proposed rulings**, each a question with its default unset — **T6 stops here; T7 applies what the user rules**:

| question | the cut, in a line | what it leaves (here, `b8c931f3`) |
|---|---|---|
| [Q-M1f.287](../open_questions.md#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance) | `id_order_invariance` digests its plain run against the golden; `corpus_shapes`' own pass goes | the two 96 s → about 65 s |
| [Q-M1f.288](../open_questions.md#q-m1f288--jobs_invariance-over-the-programs-that-search) | `jobs_invariance` over the 188 programs that search | 20.2 s → 7.5 s |
| [Q-M1f.289](../open_questions.md#q-m1f289--id_order_invariance-over-the-programs-that-search) | `id_order_invariance` over the programs that search (or its region outputs only) | 65.1 s → 14.6 s |
| [Q-M1f.290](../open_questions.md#q-m1f290--tests-out-of-the-render-and-determinism-sweeps) | `tests/` out of `corpus_shapes`, the two determinism sweeps and `dot_wellformed` | the four 129.5 s → 66.5 s |
| [Q-M1f.291](../open_questions.md#q-m1f291--the-five-sweeps-that-set-modes-and-routes-against-one-another) | keep the five mode and route sweeps; `route_invariance`'s `slow` entries to its slow half | 26.8 s → about 8 s |
| [Q-M1f.292](../open_questions.md#q-m1f292--one-load-a-program-shared-by-the-outputs-that-do-not-change-it) | one load a program, cloned for each output (`Clone` on `Ast` and `Terms`) | the A sweeps' 116 s → about half |
| [Q-M1f.293](../open_questions.md#q-m1f293--the-nightlys-steps-against-five-minutes) | the nightly's four steps over five minutes on the runner (the census, `corpus_cores`' and `answer_round_trip`'s slow halves, `region`'s) | the deep-corpus job 47.7 min at the runner's count → under 15 min, by the option |
| [Q-M1f.294](../open_questions.md#q-m1f294--answer_dumps-heavy_dump-witness-and-the-feature-tests-the-pool-does-not-reach) | `answer_dump`'s `heavy_dump` witness to the nightly; the five feature tests on the pool | 33.8 s → about 5 s; 160 s → less |

Taken together (.287, .288, .289, .290's `dot_wellformed`, .291, .292 and .294, the overlaps counted once) they would
take about 3.5 min off the 14.8 min of the per-commit tests here, the rest being the 135 binaries cargo runs one after
another, a few seconds each: cargo runs a crate's test binaries in sequence, and running them side by side would need
a runner from outside the workspace, which the stage file rules out.

### Defaults, findings, and what the stage did not do

- **Defaulted, each the user's to revise**: [Q-M1f.284](../open_questions.md#q-m1f284--the-worker-pools-count-its-heavy-lane-and-its-memory-bound)
  (a), sixteen workers, a quarter of them the heavy lane, a 24 GB bound;
  [Q-M1f.285](../open_questions.md#q-m1f285--the-nightlys-and-the-dev-hosts-test-steps-on-the-gate-profile) (a), every
  test step of every tier on the gate profile; [Q-M1f.286](../open_questions.md#q-m1f286--id_order_invariance-keeps-its-plain-run)
  (a), `id_order_invariance` keeps its plain run.
- **For the user's ruling (T6)**: Q-M1f.287 to .294, above. No check left a tier before them.
- **Findings**: [E-72](findings.md), the plain run is the permutation's source (open, Q-M1f.286, .287); [E-73](findings.md),
  three sweeps listed failures in thread order and one normalised its strings at quadratic cost (closed by T4). No wrong
  answer was met: every probe above was a deliberate mutation, in a scratch tree.
- **Not done**: the local gate was not re-run before the change (the image's run is this machine's, on AC); the
  nightly's census before the change was measured on 72 of its 572 mutants and not to its end; the runner's own
  times wait for the next push, which is the user's.
