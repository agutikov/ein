# S1a.6.1 — Fresh profile and bench baseline

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days
**Depends on:** [P1a.5](../p1a.5_presentation/README.md)
**Implements:** the phase's method, before any of its content

## Context

Every stage after this one is *chosen* by the table this stage produces.
The repo has a standing rule about it —
[F11](../../followups/f11_deductive_layer_perf.md): *"Re-run the baseline
before starting either entry — this has moved twice"* — and it has moved
twice more since, once when S1.9.E23's fail-fast removed over half the
exhaustive wall-clock and once when S1.21.8's boundary rewrite made
negation the dominant cost.

The parity build is a different engine again: integer facts, a register
matcher, O(1) forks, compile-once and a semi-naive boundary have all
landed in P1a.2–3. Assuming the Python profile still describes it would
be the single most expensive mistake in this phase.

## Acceptance

- A published attribution table for the parity build in the same shape as
  the [milestone README](../README.md#baseline--what-einrs-has-to-beat)'s
  Python one, covering `zebra2 -e`, `zebra -e`, and the acceptance gate.
- `criterion` benches green and stable (< 3 % run-to-run variance on the
  bench machine) for all eight benchmarks.
- [design/README.md § Measured](../design/README.md#measured) filled in
  for the P1a.5 row.
- A written list of the top five costs, each with the design doc section
  that predicted (or failed to predict) it.

## Tasks

### Task T1a.6.1.1 — Bench stabilisation

Pin the bench environment: CPU governor, turbo, core affinity,
`--no-default-features --features einb` (so the event emitter is compiled
out entirely). Report variance, not just means.

### Task T1a.6.1.2 — Profile the parity build

`perf record` / `samply` on `zebra2 -e` and `zebra -e`; produce a
self-time table at function granularity plus a subsystem rollup matching
`utils/profile_solve.py`'s categories (match/bind, saturate, hypgen,
apriori, canon, contradiction, fork, alive) so the Python and Rust
tables are directly comparable.

### Task T1a.6.1.3 — Counter-based comparison

Wall-clock is machine-dependent; *work* is not. Emit and compare the
counts the Python profile gives for free — unification calls, candidate
iterations, guard sub-plan evaluations, plan compiles, fork count,
provenance-walk nodes — so a "3× faster" claim can be split into "did
less" vs "did the same faster".

### Task T1a.6.1.4 — Memory baseline

Peak RSS and allocation counts for both puzzles, and the per-fork delta
size distribution ([design/03](../design/03_data_model.md) §5) — the
number [P1a.7](../p1a.7_parallelism/README.md) needs to size `--jobs`.

### Task T1a.6.1.5 — Refresh the Python baseline

Re-run `utils/bench_baseline.py` under both CPython and PyPy on the same
machine on the same day. A 20× claim measured against a six-month-old
number is not a measurement.

## Notes

- Expect the profile to look nothing like the Python one. If it *does* —
  if the boundary is still 72 % — that is itself the finding, and
  [S1a.6.3](s1a.6.3_beta_memories.md) is probably not the next stage.
- Record the profile artefacts (not just the summary) somewhere
  durable; the next re-measure wants a diff, not a fresh start.
