# S1a.7.5 — The `--jobs` contract

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Status:** ✅ **closed 2026-08-23** — five tasks shipped and one declined.
The corpus-wide sweep is the headline: **20 712 (file, op, jobs) cells at
`--jobs {2,4,8,16}` over 128 files × 45 ops, 13 920 of them running a solve,
0 moved**, in 30 s against the retired T3 harness's 738. `--jobs auto` and the
ruling that jobs stays out of `SolverConfig`; `Terms::lend`, so a worker panic
cannot leave the tables lent; the failure-mode rulings; the scaling row in
[design/README § Measured](../design/README.md#measured). **`--unordered` is
declined** — in this fan-out it is worth 0 %, so the contract is two modes and
both are the same computation.
**Depends on:** [S1a.7.2](s1a.7.2_parallel_enterings.md),
~~[S1a.7.3](s1a.7.3_parallel_boundary.md)~~,
~~[S1a.7.4](s1a.7.4_parallel_enqueue.md)~~ — **both declined 2026-08-23**
([scaling.md §9](scaling.md#9-levels-2-and-3-measured-before-they-are-built)),
so this stage is the phase's last and the "three parallel levels" its Context
assumes are **one**: level 1, plus level 4, which needed nothing
**Implements:** [design/08](../design/08_parallelism.md) §1
**Decides:** Q-M1a.7

## Context

> **One level exists, not three** — [S1a.7.3](s1a.7.3_parallel_boundary.md) and
> [S1a.7.4](s1a.7.4_parallel_enqueue.md) were declined on measured premises,
> which changes this stage in exactly one place: T1a.7.5.2's "threaded to the
> three levels" is a `SolveOptions` field read at one site. The contract, the
> matrix and `--unordered` were unaffected by *that*, because they were always
> about what the flag promises rather than about how many sites honour it —
> though `--unordered` then went the same way for a reason of its own
> (T1a.7.5.4).

Three parallel levels exist by now; this stage turns them into a *user
contract* — one flag, three documented modes, and the conformance matrix
that keeps the promise honest.

The default matters more than the speedup. `--jobs 1` stays the default
because a benchmark or a golden run must never silently become a
different computation, and because the whole parity apparatus assumes it.

## Acceptance

> **Restated 2026-08-22.** Every "T*n*-identical" below named
> `ein-conformance`, which [P1a.10](../p1a.10_single_implementation/README.md)
> retired with the second engine. The successor per half is the phase
> [README § The acceptance, restated](README.md#the-acceptance-restated); the
> promise is unchanged and in one place stronger, because the cut names which
> differences are admitted where a byte diff could only say that there was one.

- `--jobs N` (deterministic) is **the same computation** as `--jobs 1` for
  N ∈ {1, 2, 4, 8, 16} across the whole corpus, with only wall-clock
  fields normalised.
- ~~`--unordered` keeps the **answer** … and is therefore the one mode that
  must **fail** `jobs_invariance`.~~ **The mode is declined** (T1a.7.5.4,
  2026-08-23), so there is nothing to exempt: every mode the engine has is the
  same computation, and `jobs_invariance` has no arm that is supposed to fail.
  A criterion for a mechanism that is not built is a category error, not a
  weaker criterion, so it is struck through rather than deleted.
- The CLI surface and the `SolverConfig` interaction agree on the same
  semantics, and **the crate API exposes the same knob under the same name** —
  restated 2026-08-22: this read "the embedding API (P1a.9)", and
  [P1a.9 deferred the PyO3 binding](../p1a.9_release/README.md) on 2026-08-21
  for want of a consumer. The embedding surface that is real is the crates,
  and [S1a.9.4](../p1a.9_release/s1a.9.4_documentation.md) is where it is
  written down.
- The guarantee table from [design/08](../design/08_parallelism.md) §1 lands in
  user-facing documentation, not only in the plan. **Not `docs/api/`** — that
  tree describes an engine that no longer exists and is kept in reserve; the
  flag belongs wherever S1a.9.4 puts the CLI's contract.
- Q-M1a.7 closed with the measured re-validation rate.

## Tasks

### Task T1a.7.5.1 — CLI and config ✅

`-j/--jobs N` (default 1; `auto` = available parallelism) and
`--unordered`. Decide whether jobs belongs in `SolverConfig` — argument
against: it is an execution knob, not a semantics knob, and every
`SolverConfig` field is printed by `--dump-config` and parsed from
`(config …)`, which would let a *puzzle file* set the thread count.
Recommendation: keep it CLI/API-only, not in `SolverConfig`.

> **Shipped 2026-08-23.** `-j/--jobs N` was already there (T1a.7.2.1); this
> added **`--jobs auto`** and the ruling.
>
> - **`auto` is a sentinel, not a spelling.** The value parser maps the word to
>   `0` and `crate::solve::resolve_jobs` turns that into
>   `std::thread::available_parallelism`, which respects a cgroup quota and a
>   `taskset` mask. A literal `--jobs 0` is **refused** — "0 threads" reads as
>   *none* at least as often as *all of them*, and a flag with two ways to say
>   one thing has two ways to be misread. `--jobs xyz` and `--jobs 1.5` are
>   refused with the same message shape.
> - **`auto` is the machine's answer, not the best one**, and the help says so
>   rather than pretending otherwise: `available_parallelism` counts *logical*
>   CPUs, so on this dev box it is **32** — 8 P-cores, their SMT siblings and
>   16 E-cores — and [scaling.md §8](scaling.md#8-t1a721--the-fan-out-and-the-three-things-it-costs)
>   measures `--jobs 16` as *slower* than `--jobs 8` there. `auto` is a
>   convenience; a measurement passes a number.
> - **Not in `SolverConfig`, and the reason is the parity contract.** Every
>   `SolverConfig` field is printed by `--dump-config` and parsed from
>   `(config …)`, so a thread count there would let a **puzzle file** set it —
>   a `.ein` that reads differently on an 8-core machine than on a 4-core one,
>   through a field `--json-summary` compares. Jobs is an execution knob and
>   `SolverConfig` is the semantics. The recommendation below is taken as
>   written.
> - **The batch size does not become a second knob**, because the
>   batch-synchronous route was not taken ([S1a.7.2](s1a.7.2_parallel_enterings.md)
>   § The decision). `BATCH_PER_WORKER` is a measured constant — re-taken from
>   32 to **512** on 2026-08-23 — and `EIN_BATCH_PER_WORKER` is the seam for
>   re-taking it, not a user flag: it changes no observable.
>
> `cli_semantics.rs`'s `jobs_takes_a_count_or_auto_and_nothing_else` is the
> test, and it also holds the half that keeps every existing `--stats` run
> byte-identical: at `--jobs 1` the block is not printed at all.

**The precedent is already set.**
[S1a.7.0](s1a.7.0_speculation_audit.md) T1a.7.0.5 put
`SolveOptions::integrate_every` — the batch barrier — exactly there, for
exactly this reason, and left it without a flag on purpose: the mode it
exposes is `--jobs`'s to name. If the batch-synchronous route wins in
[S1a.7.2](s1a.7.2_parallel_enterings.md), the **batch size** is a second knob
this task has to surface, and it is not a thread count: `--jobs 8` with a
barrier every 10 000 enterings and `--jobs 8` with one per layer are different
computations with the same worker count.

### Task T1a.7.5.2 — Mode plumbing ✅ — and it is one field, not a policy

`SolveOptions::jobs` is the whole of it, and `ExecPolicy { jobs, ordered }`
would be a struct with one live field: `ordered` is `--unordered`'s and that is
declined above, and "the three levels" are one since
[S1a.7.3](s1a.7.3_parallel_boundary.md) and
[S1a.7.4](s1a.7.4_parallel_enqueue.md) were.

**What the task actually asks for is already true and is asserted.**
`jobs == 1` takes the sequential path rather than a one-worker pool — the pool
is built only when `opts.jobs > 1`, so a default run **creates no thread at
all**, and `Run::phase2`'s `fan_out` predicate is
`fanned_out && jobs > 1 && cfg!(feature = "parallel")`. The sequential path is
therefore the same lines it was before the phase, which is what "must not
bit-rot" asks: it is not a mode, it is the loop, and `--jobs N` is a branch
around it.

What follows is the task as written.

One `ExecPolicy { jobs, ordered }` threaded to the three levels, with
`jobs == 1` short-circuiting to the sequential code path (not a
one-worker pool — the sequential path must stay the reference
implementation and must not bit-rot).

### Task T1a.7.5.3 — The cross-jobs matrix ✅

**Re-aimed 2026-08-22.** "Extend the harness" named `ein-conformance`; what
this builds instead is `ein-render/tests/jobs_invariance.rs` — the third sweep
over [`corpus_ops`](../../../ein.rs/crates/ein-render/tests/corpus_ops/mod.rs),
beside `corpus_shapes` (digest once) and `id_order_invariance` (twice, ids
permuted). Every corpus file × every op × every `jobs` value, in one process,
cut by `ein-parity`. This is the mechanism that keeps the promise; without it,
"deterministic parallel" decays within a month.

**Its first line is a jobs axis on `solve_shape`**, and that is worth knowing
before the sweep is written: `Op::Solve` reaches `ein_infer::solve_shape`,
which builds its own `SolveOptions` and therefore pins `jobs` at 1. Until it
takes one, *every* `corpus_ops` sweep — `corpus_shapes` and
`id_order_invariance` included — is a `--jobs 1` sweep, so the "`--jobs`
composes with the permuted id space" the phase
[README](README.md#the-acceptance-restated)'s last table row wants is
downstream of this and not already true. What *is* already true is the
generated-input form, [S1a.7.2](s1a.7.2_parallel_enterings.md) T1a.7.2.6's
`jobs` property in `utils/fuzz_ein.py`: 10 000 paired `--jobs 8` runs through
the CLI, which compares processes rather than the 45 ops. The two are
complements.

**Nightly may not be needed.** The tier was nightly because two processes per
corpus cell cost 738 s; `id_order_invariance` does the whole corpus twice in
seconds. If the `jobs` axis lands in the same envelope it belongs in
`cargo test --workspace`, and a gate that runs is worth more than a tier that
is read on Mondays.

> **Shipped 2026-08-23, and nightly is not needed.**
> `ein-render/tests/jobs_invariance.rs`: **5 178 cells in 12 s** at the default
> `--jobs 2`, **20 712 in 30 s** at `EIN_JOBS_SWEEP=2,4,8,16` — which is the
> acceptance's whole matrix, 25× cheaper than the harness it replaces, and in
> `cargo test --workspace` rather than on a schedule. **0 moved**, and 13 920
> of the cells ran a solve, which is asserted with a floor so the sweep cannot
> go green by ceasing to reach them.
>
> Four things are worth carrying forward.
>
> - **The jobs axis is a parameter, not a global.** `solve_shape`, `dot_shape`,
>   `trace_shape` and `dump_shape` take a `jobs: usize`; `corpus_ops::run_with`
>   passes it and `run` is `run_with(…, 1)`. A global would have been three
>   lines shorter and would have made the sweep's two runs share it.
> - **The test is stricter than the contract, on purpose.** The contract admits
>   narration movement — a firing count, an event ordinal, a dying fork's
>   stopping point — and `id_order_invariance` measures 51 of 3 160 such
>   movements under a permuted id space. Under a job count there are **none**,
>   because a worker's events get their ordinals at the ordered commit
>   (T1a.7.2.2), so the sweep asserts byte equality and classifies any
>   difference through `ein-parity`'s cut only to say *which half* broke. A
>   narration difference fails too, with a message saying the contract would
>   have allowed it and that relaxing the assertion is a deliberate edit.
> - **It was made to fail before it was trusted.** Committing a batch's results
>   in reverse order — correct pairing, wrong order — turns it red on **179 of
>   5 178** cells, and every one of them is classified as an *answer*
>   difference rather than a narration one.
> - **The default sweep is `--jobs 2` and that is a measurement decision**, in
>   `EIN_ID_SEEDS`' shape: what makes a fan-out wrong is committing out of
>   order or handing a worker the wrong root, and two threads reach both.
>   `EIN_JOBS_SWEEP` is the seam, and `2,4,8,16` is what a release run uses.

### Task T1a.7.5.4 — ~~`--unordered`~~ ✗ **declined — there is nothing for it to buy back**

> **Measured and structural, 2026-08-23, and declined**; unlike
> S1a.7.3 and S1a.7.4 this one does not even need a workload to change its
> mind: it needs a different fan-out. The row is struck from
> [design/08 §1](../design/08_parallelism.md#1-the-contract-first)'s contract
> table with the reason beside it, so the flag is not promised anywhere.

The mode exists in [design/08 §1](../design/08_parallelism.md#1-the-contract-first)'s
table because "there are workloads where the last 20 % of determinism costs
2×". In *this* fan-out it costs **0 %**, and the reason is the shape rather
than the numbers:

```rust
let results = self.fan_out(root, lent.get(), …, &candidates[i..end]);  // returns when every worker is done
for (c, r) in candidates[i..end].iter().zip(results) { self.commit_entering(…) }
```

**`fan_out` is a barrier.** By the time the commit loop starts there is no
outstanding work and no worker waiting, so consuming the results in completion
order rather than in candidate order changes *which* order serial work happens
in and not *when* it happens. There is no "commit-on-completion" to relax to: a
`Vec<Speculated>` has no completion order left in it.

**The version that would be worth something is a different mechanism.** To
overlap the commit with speculation, `commit_entering` would have to run while
workers are still going — and it takes `&mut Terms` (the provenance arena,
`record_node`'s promotion) and `&mut Kb` (root's no-good store), where a worker
holds `&Terms` and `&Kb`. That is the lend seam
[T1a.7.2.1](s1a.7.2_parallel_enterings.md#task-t1a721--snapshot-and-fan-out)
built and [S1a.7.1](s1a.7.1_sync_shared_state.md) chose deliberately, having
measured the alternative — `Arc<T>` in both states — at **4 % slower**. So
`--unordered` is not a relaxation of the ordered commit; it is a concurrent
interner, which this phase declined on its own measurement.

**And the ceiling is small anyway.** The ordered commit is what such a mode
could overlap, and [§ Where the other 1.5× is](scaling.md#where-the-other-15-is)
prices it at `--jobs 8`: **4.0 / 2.7 / 53.6 / 3.2 ms** of Phase-2 totals near
49 / 76 / 550 / 58 — 3.6 % to 9.8 %, and only if the overlap were free.

**What it would cost is the whole point of the phase.** Every counter identity,
`jobs_invariance`'s 20 712 cells, `summary_properties`' thirteen identities and
the fuzzer's `jobs` property are written against "the same computation". A flag
that exempts itself from all of them, for ≤ 9.8 % it cannot actually collect,
is a promise nobody should be able to opt into by accident.

What follows is the task as written.

Relax the ordered commit (level 1) to commit-on-completion, keeping only
solution-node recording and no-good emission (both order-insensitive in
their *effect*, if not in their counters). Verify **the answer** holds —
verdict, `k`, `exhausted`, the model as a fact set, the goal bindings and the
unsat core, which is [oracle ledger](../p1a.10_single_implementation/oracle_ledger.md)
row 1.1 and is asserted by `ein-infer/tests/acceptance.rs` and
`ein-cli/tests/summary_properties.rs` — and be
explicit in the docs that entering counts, nogood counts and traversal
order may differ, so nobody benchmarks with it by accident.

### Task T1a.7.5.5 — Diagnostics and scaling report ✅

> **Both halves shipped, and half the list it asks for has nothing to count.**
>
> - **The diagnostics are [T1a.7.2.5](s1a.7.2_parallel_enterings.md#task-t1a725--diagnostics)'s**,
>   printed by `--stats` when and only when `--jobs > 1`: workers, speculated
>   with its committed / handed-back / wasted split, and the sequential count.
> - **The case-2 / case-3 validation counters have nothing to count**, because
>   there is no validation: a layer is fanned out iff it cannot write to root.
>   **The boundary chunk stats have nothing to count either**, because
>   [S1a.7.3](s1a.7.3_parallel_boundary.md) is declined. What replaced both is
>   `JobStats::sequential`, which is 0 on three of the four measurement-set
>   workloads and 204 on the fourth — Amdahl's numerator per run, and a build
>   where it grew would be a build where the fan-out predicate had changed.
> - **The scaling table is in
>   [design/README.md § Measured](../design/README.md#measured)**, dated
>   2026-08-23, taken through `bench_env.sh --cores P:8` so its "8 cores" names
>   one machine. It carries the measurement set's 3.17–4.40×, the two zebras as
>   parity cells at 1.23× / 1.43×, where the missing 1.5× is, and what the two
>   declined levels were declined on.

What follows is the task as written.

A `--stats` addition (or a sibling flag) reporting: workers used,
speculative vs committed enterings, case-2/case-3 validation counts,
boundary chunk stats, and parallel-pass fraction. Publish a scaling
table (1/2/4/8/16 cores × the phase's measurement set) in
[design/README.md § Measured](../design/README.md#measured) — **through
[`utils/bench_env.sh`](../../../utils/bench_env.sh) `--cores`**, which shipped
at S1a.7.1 and is what makes the table's "8 cores" mean one machine rather
than three (`P:8`, `PT:8` and `E:8` are all "8 cores" and none of them is the
others). "both puzzles" was the pre-S1a.7.0 target set and is superseded.

### Task T1a.7.5.6 — Failure modes ✅

**Decided 2026-08-23**, one ruling each, and one of the three needed code.

- **A worker panic aborts the solve, and no candidate is silently dropped.**
  `rayon` catches a worker's panic and resumes unwinding on the thread that
  called `install`, payload intact — so the message is the worker's own and the
  solve ends the way a sequential panic would end it. Nothing is committed from
  that batch, because the fan-out is a barrier: a panic anywhere in it means
  the commit loop is never reached, rather than that one candidate's result
  went missing.
- **…and the one thing that survived a panic badly is now structural.**
  `Terms::share` and `Terms::reclaim` have to come in pairs, and the window
  between them is where a panic — or a future `?` — would leave without the
  second half. A lent `Terms` is not a crash: it is a table that has silently
  stopped growing, so **every later entering hands itself back, for ever, at
  full speed**. `Terms::lend` now returns a guard that reclaims on `Drop`, and
  reclaims *quietly* while `std::thread::panicking()`, because the assertion
  that is right on the ordinary path would be a second panic here and would
  take the first one's message with it.
  `worker_view.rs`'s `a_panic_inside_the_lend_window_gives_the_tables_back` is
  the test, and it fails with a bare `share()`.
- **`max_time` is checked at the commit point, so its cut is deterministic in
  *where* and not in *when*.** `before_commit` calls `check_budget` once per
  committed entering, in candidate order, so an expired budget stops the run at
  a candidate index rather than mid-batch — the in-flight batch is finished and
  then discarded, and what that discards is bounded by T1a.7.2.4's ramp: never
  more than the enterings already committed. `max_time` is why the ramp covers
  three flags and not two.
- **Cancellation is not built, and that is a decision rather than an
  omission.** The shape is known and small — an `Arc<AtomicBool>` on
  `SolveOptions`, checked in `before_commit` beside the budget, giving the same
  deterministic cut — and what is missing is a caller. The embedding surface is
  the crates ([S1a.9.4](../p1a.9_release/s1a.9.4_documentation.md)), the PyO3
  binding was deferred for want of a consumer
  ([Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)),
  and `max_time` already stops a runaway solve. Building it now would be API
  with a test and no user; the two lines are written down here for the day one
  arrives.

## What the stage decided

One line each, because the reasons are in the tasks:

| | |
|---|---|
| `-j/--jobs N`, default **1** | unchanged, and the default is a correctness-of-measurement choice |
| `--jobs auto` | `available_parallelism`; `--jobs 0` refused; the help says it is the machine's answer, not the best one |
| jobs in `SolverConfig` | **no** — a puzzle file must not set a thread count |
| a batch-size flag | **no** — `BATCH_PER_WORKER` is a measured constant, `EIN_BATCH_PER_WORKER` the seam |
| `ExecPolicy` | **no** — one `SolveOptions` field; `jobs == 1` takes the sequential path and builds no pool |
| `--unordered` | **declined** — 0 % in this fan-out, ≤ 9.8 % in one that does not exist |
| worker panic | propagates with its payload; the fan-out's barrier means nothing is half-committed |
| the lend window | a `Drop` guard, quiet while panicking |
| `max_time` in flight | cut at the commit point, waste bounded by T1a.7.2.4's ramp |
| cancellation | **not built** — two lines, no caller; `max_time` is the mechanism today |

## Notes

- Resist adding a `--jobs auto` *default* later "because it is faster".
  The default is a correctness-of-measurement choice, and the
  conformance harness, the benchmarks and the acceptance gate all depend
  on it.
- An embedder that drives many solves can reasonably default its own
  jobs to `auto`; that is a caller-side decision and does not change the
  CLI default.
