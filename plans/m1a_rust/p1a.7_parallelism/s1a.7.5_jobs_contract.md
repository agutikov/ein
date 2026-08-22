# S1a.7.5 — The `--jobs` contract

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Depends on:** [S1a.7.2](s1a.7.2_parallel_enterings.md),
[S1a.7.3](s1a.7.3_parallel_boundary.md),
[S1a.7.4](s1a.7.4_parallel_enqueue.md)
**Implements:** [design/08](../design/08_parallelism.md) §1
**Decides:** Q-M1a.7

## Context

Three parallel levels exist by now; this stage turns them into a *user
contract* — one flag, three documented modes, and the conformance matrix
that keeps the promise honest.

The default matters more than the speedup. `--jobs 1` stays the default
because a benchmark or a golden run must never silently become a
different computation, and because the whole parity apparatus assumes it.

## Acceptance

- `--jobs N` (deterministic) is **T3-identical** to `--jobs 1` for
  N ∈ {1, 2, 4, 8, 16} across the whole corpus, with only wall-clock
  fields normalised.
- `--unordered` is T0-identical (same verdict, same model set) and
  documented as *not* counter-identical, with a fixture demonstrating a
  counter difference so the distinction is visible rather than theoretical.
- The CLI surface and the `SolverConfig` interaction agree on the same
  semantics, and the embedding API ([P1a.9](../p1a.9_release/README.md))
  exposes the same knob under the same name.
- `docs/api/inference.md` gains the flag with its guarantees; the
  guarantee table from [design/08](../design/08_parallelism.md) §1 lands
  in user-facing documentation, not only in the plan.
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

### Task T1a.7.5.3 — The cross-jobs conformance matrix

Extend the harness: every corpus entry × every `jobs` value, T3-diffed
against `--jobs 1`. Nightly tier. This is the mechanism that keeps the
promise; without it, "deterministic parallel" decays within a month.

### Task T1a.7.5.4 — `--unordered`

Relax the ordered commit (level 1) to commit-on-completion, keeping only
solution-node recording and no-good emission (both order-insensitive in
their *effect*, if not in their counters). Verify T0 holds — and be
explicit in the docs that entering counts, nogood counts and traversal
order may differ, so nobody benchmarks with it by accident.

### Task T1a.7.5.5 — Diagnostics and scaling report

A `--stats` addition (or a sibling flag) reporting: workers used,
speculative vs committed enterings, case-2/case-3 validation counts,
boundary chunk stats, and parallel-pass fraction. Publish a scaling
table (1/2/4/8/16 cores × both puzzles) in
[design/README.md § Measured](../design/README.md#measured).

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
