# S1a.8.5 — Solve jobs

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [S1a.8.4](s1a.8.4_query_and_inspect.md)
**Implements:** [design/09](../design/09_server_mode.md) §§4, 7

## Context

A solve can run for seconds (or, on a wide search with
`enable_singleton_writeback` off, indefinitely). A request/response call
is the wrong shape for that, so `solve.start` returns a handle
immediately and the work runs in the background, streaming progress and
answering to `await` / `cancel` / `result`.

The engine needs almost nothing new: budgets and cancellation land at the
same checkpoint (`_check_budget`), and the parallel search from
[P1a.7](../p1a.7_parallelism/README.md) already isolates a solve's state
in its own forks.

## Acceptance

- `solve.start` returns in under a millisecond regardless of the puzzle.
- `solve.result` for a completed job matches the CLI's verdict, `k`,
  stats, models and unsat core exactly (T1) for every corpus entry with
  `--no-cache`.
- `solve.cancel` releases threads and memory within 100 ms, and the
  handle then reports a cancelled status distinct from `Aborted`.
- A budget cut returns `Aborted(reason, stats)` — a *result*, not an
  error — matching `on_budget="verdict"`.
- Concurrent solves in one session respect `max_jobs` in aggregate, not
  per solve.

## Tasks

### Task T1a.8.5.1 — Job lifecycle

`solve.start {kb, config, stop_after, max_set_size, budgets, jobs}` →
handle. States: running / done / cancelled / failed. `solve.await`
(long-poll or notification-driven), `solve.result`, `solve.close`.

The `config` param takes the same `SolverConfig` field names as
`(config …)` and `--dump-config`, so there is one vocabulary for the
knobs across file, CLI and wire.

### Task T1a.8.5.2 — Cancellation

Cooperative, checked at the existing budget checkpoints: the per-candidate
`_check_budget` in the layer loop, and (for a long root saturation) the
saturator's step loop. Cancelling mid-fork drops the fork wholesale —
which is free, because a fork owns its delta and root was never mutated.

Under [S1a.7.2](../p1a.7_parallelism/s1a.7.2_parallel_enterings.md)'s
speculation, cancellation must also abandon in-flight speculative work
without committing it.

### Task T1a.8.5.3 — Budgets

Per-request `max_time` / `max_enterings`, clamped by the session's
budget. Map a cut to `Aborted` (never to a protocol error), and keep
`Aborted` distinguishable from cancellation and from
`Contradiction` — `solution_nodes == 0` under `Aborted` means
*unexplored*, not *proven unsatisfiable*, and a client that confuses
those will report a wrong answer.

### Task T1a.8.5.4 — Results and model handles

`solve.result` returns the verdict, `k`, `exhausted`, all counters, the
unsat core, and a `model` handle per solution node (so a client can
`kb.query` a model without transferring it). `store_lattice` is opt-in
per request, as in the API.

### Task T1a.8.5.5 — Scheduling

A bounded worker pool shared across sessions with per-session
concurrency limits. A long solve must not starve short queries: give
`kb.query` / `kb.facts` a separate small pool or a priority lane, and
test it (100 queries issued while an exhaustive solve runs should all
answer promptly).

### Task T1a.8.5.6 — Failure isolation

A panic inside a solve fails that job with a diagnostic, not the
process. Catch at the job boundary, log, and mark the handle failed —
with a test that deliberately panics a worker.

## Notes

- The `dumper` hooks are how progress escapes; wiring them to the event
  stream is [S1a.8.6](s1a.8.6_streaming.md), and this stage should not
  invent a second progress path.
- Do not let a solve mutate a `kb` handle. It forks; the handle's KB is
  unchanged, which is what lets a client run several solves with
  different configs against the same saturated KB — a genuinely useful
  workflow and a good acceptance test.
