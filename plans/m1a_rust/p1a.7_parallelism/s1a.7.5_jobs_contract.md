# S1a.7.5 — The `--jobs` contract

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Status:** in progress — **T1a.7.5.3 shipped 2026-08-23**, the corpus-wide
sweep: **20 712 (file, op, jobs) cells at `--jobs {2,4,8,16}` over 128 files ×
45 ops, 13 920 of them running a solve, 0 moved**, in 30 s against the retired
T3 harness's 738.
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
> matrix and `--unordered` are unaffected, because they were always about what
> the *flag* promises rather than about how many sites honour it.

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
- `--unordered` keeps the **answer** — same verdict, same model set, which is
  what the ledger's row 1.1 covers — and is documented as *not*
  counter-identical, with a fixture demonstrating a counter difference so the
  distinction is visible rather than theoretical. It is therefore the one mode
  that must **fail** `jobs_invariance`, and the test has to say so on purpose
  rather than by not being run.
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

### Task T1a.7.5.1 — CLI and config

`-j/--jobs N` (default 1; `auto` = available parallelism) and
`--unordered`. Decide whether jobs belongs in `SolverConfig` — argument
against: it is an execution knob, not a semantics knob, and every
`SolverConfig` field is printed by `--dump-config` and parsed from
`(config …)`, which would let a *puzzle file* set the thread count.
Recommendation: keep it CLI/API-only, not in `SolverConfig`.

**The precedent is already set.**
[S1a.7.0](s1a.7.0_speculation_audit.md) T1a.7.0.5 put
`SolveOptions::integrate_every` — the batch barrier — exactly there, for
exactly this reason, and left it without a flag on purpose: the mode it
exposes is `--jobs`'s to name. If the batch-synchronous route wins in
[S1a.7.2](s1a.7.2_parallel_enterings.md), the **batch size** is a second knob
this task has to surface, and it is not a thread count: `--jobs 8` with a
barrier every 10 000 enterings and `--jobs 8` with one per layer are different
computations with the same worker count.

### Task T1a.7.5.2 — Mode plumbing

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

### Task T1a.7.5.4 — `--unordered`

Relax the ordered commit (level 1) to commit-on-completion, keeping only
solution-node recording and no-good emission (both order-insensitive in
their *effect*, if not in their counters). Verify **the answer** holds —
verdict, `k`, `exhausted`, the model as a fact set, the goal bindings and the
unsat core, which is [oracle ledger](../p1a.10_single_implementation/oracle_ledger.md)
row 1.1 and is asserted by `ein-infer/tests/acceptance.rs` and
`ein-cli/tests/summary_properties.rs` — and be
explicit in the docs that entering counts, nogood counts and traversal
order may differ, so nobody benchmarks with it by accident.

### Task T1a.7.5.5 — Diagnostics and scaling report

A `--stats` addition (or a sibling flag) reporting: workers used,
speculative vs committed enterings, case-2/case-3 validation counts,
boundary chunk stats, and parallel-pass fraction. Publish a scaling
table (1/2/4/8/16 cores × the phase's measurement set) in
[design/README.md § Measured](../design/README.md#measured) — **through
[`utils/bench_env.sh`](../../../utils/bench_env.sh) `--cores`**, which shipped
at S1a.7.1 and is what makes the table's "8 cores" mean one machine rather
than three (`P:8`, `PT:8` and `E:8` are all "8 cores" and none of them is the
others). "both puzzles" was the pre-S1a.7.0 target set and is superseded.

### Task T1a.7.5.6 — Failure modes

Decide and document: what happens on a worker panic (abort the solve
with a clear message — do not silently drop a candidate); how
`max_time` interacts with in-flight speculation (budget is checked at the
commit point, so a cut is deterministic); and how cancellation
propagates (a cooperative flag checked at the same commit point, so an
embedder can cancel a solve without a thread kill).

## Notes

- Resist adding a `--jobs auto` *default* later "because it is faster".
  The default is a correctness-of-measurement choice, and the
  conformance harness, the benchmarks and the acceptance gate all depend
  on it.
- An embedder that drives many solves can reasonably default its own
  jobs to `auto`; that is a caller-side decision and does not change the
  CLI default.
