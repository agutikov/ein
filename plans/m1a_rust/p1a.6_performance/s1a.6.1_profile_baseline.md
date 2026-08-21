# S1a.6.1 — Fresh profile and bench baseline

**Phase:** P1a.6 (Performance)
**Status:** **shipped** 2026-08-18 — every acceptance item met; the numbers
and the artefact list are in **[baseline.md](baseline.md)**, and the
re-planning it forced is in [§8](baseline.md#8-what-this-chooses-for-the-rest-of-the-phase)
and the [phase README](README.md#stages).
**Estimate:** 2 days
**Depends on:** [P1a.5](../p1a.5_presentation/README.md)
**Implements:** the phase's method, before any of its content

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `bench_baseline.py`, `count_work.py` and `profile_solve.py`. They are gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

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

All met.

| item | result |
|---|---|
| attribution table for the parity build, `zebra2 -e` / `zebra -e` / the acceptance gate | [baseline.md §3](baseline.md#3-attribution--where-the-time-goes) and [§6](baseline.md#6-cargo-bench--variance-and-the-acceptance-gate). `zebra2 -e`: saturate 59.7 %, match/bind 29.0 %, hypgen 7.3 %, **0.3 % unattributed**. `zebra -e`: match/bind **66.9 %**. The gate: 1.27 s against 36.0 s |
| `criterion` stable under 3 % | **11 cases, worst relative sd 2.40 %** (`parse/corpus`), gated by `utils/criterion_table.py`'s exit code |
| design/README § Measured, P1a.5 row | [filled](../design/README.md#measured), with the acceptance-gate footnote it needed |
| top five costs, each with the design section that predicted it | [baseline.md §7](baseline.md#7-the-top-five-costs) — two of the five were predicted by no design section, and one of those is 21 % of the run |

Beyond the list:

- **The targets split.** `solve zebra2 -e` is **met at 198.8 ms** (≤ 200 ms,
  24.8× PyPy); `solve zebra -e` is **not**, at 585.8 ms against ≤ 400 ms;
  parse + load and the acceptance gate are met with an order of magnitude to
  spare. Which is the point of measuring before optimising: one of the four
  targets needs the phase, and the profile says the matcher is where it lives.
- **The Python denominators moved and one is unreproducible.** PyPy's
  `zebra2 -e` is 4.94 s today against the recorded 4.07 s, so the ≥ 20×
  target is ≤ 247 ms, not ≤ 204 ms; the recorded 0.78 s for parse + load
  cannot be derived from its own components on either interpreter
  ([baseline.md §1](baseline.md#where-the-milestones-denominators-moved)).
- **A stage was added.** [S1a.6.8](s1a.6.8_compile_cache_and_extents.md) —
  the plan memo is a field of `Engine` and there is one engine per saturation,
  so an exhaustive `zebra2` compiles 17 430 plans and **21.1 % of the run is
  inside the compiler**. ein.py compiles exactly as many, so this is not a
  parity defect: it is [design/06](../design/06_saturation.md) § Win A, planned
  in full and never built, worth nearly twice the 12 % it estimated.
- **One number was wrong before it was published.** The first work-counter run
  put ein.py's compile count at 180 against that 17 430 — a 97× gap that was
  the *instrument*, not the engine (`utils/count_work.py` wrapped a module
  attribute that `engine.py` had already bound into its own namespace). A
  second instrument caught it: both implementations emit **17 250** `compile`
  events on that run. Recorded in
  [baseline.md §4](baseline.md#4-what-the-engine-did--the-work-counters),
  because a measurement tool that can be wrong quietly is the one thing this
  stage cannot afford.
- **The gate held throughout.** T3 over the corpus after the
  instrumentation: **472 same, 1 differ**, the differing cell being
  [D2](../divergences.md) and nothing else — the same result as
  [P1a.5](../p1a.5_presentation/README.md)'s.

## Tasks

### Task T1a.6.1.1 — Bench stabilisation

Pin the bench environment: CPU governor, turbo, core affinity,
`--no-default-features --features einb` (so the event emitter is compiled
out entirely). Report variance, not just means.

> **Done as `utils/bench_env.sh`, with two of the four asks answered
> differently.** Core affinity is pinned to cpu4 and the machine state —
> governor, turbo, current and max MHz, loadavg, `perf_event_paranoid`, git
> revision — is printed to stderr ahead of every run, so no artefact in this
> phase can be read without it. The **governor cannot be pinned**:
> `scaling_governor` is root-owned here, and a bench script that asks for
> `sudo` is worse than one that reports `powersave` and lets the variance
> column carry it — which it does, at ≤ 2.40 % on every criterion case.
>
> **The feature flags were a stale premise.** The workspace had no `[features]`
> at all, and `einb` is [P1a.8](../p1a.8_binary_container/README.md)'s
> container rather than an emitter switch. Nor is one needed: every `emit` call
> takes a **closure** that builds the payload, and `Events::emit` reads its
> `Option` sink before calling it, so an emitter that is off costs one branch
> and builds nothing — the design said so
> ([`events.rs`](../../../ein.rs/crates/ein-infer/src/events.rs) § "Off is
> free") and the profile agrees: **zero of 8 169 samples** in events code on an
> exhaustive `zebra2`. The one feature this stage did add is `counters`, which
> is off by default for the same reason inverted — see T1a.6.1.3.

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

> **Done, and it found that the *shape* of the old measurement was wrong too.**
> `bench_baseline.py` times engine calls inside one warm interpreter, which is
> the right shape for a `parse` against a `parse` and the wrong one for the
> milestone's headline claim: end-to-end is a **process**, including a cold
> JIT. Warm, PyPy solves `zebra2 -e` in 2.30 s; as a process it takes 4.94 s,
> and the recorded baseline is 4.07 s. `utils/e2e_baseline.py` was added for
> the process shape and **both** are reported
> ([baseline.md §1](baseline.md#1-end-to-end-process-against-process),
> [§2](baseline.md#2-the-in-process-bench-set)) — which is also how the finding
> that **PyPy is slower than CPython on two of the six workloads** surfaced.

## Notes

- Expect the profile to look nothing like the Python one. If it *does* —
  if the boundary is still 72 % — that is itself the finding, and
  [S1a.6.3](s1a.6.3_beta_memories.md) is probably not the next stage.
- Record the profile artefacts (not just the summary) somewhere
  durable; the next re-measure wants a diff, not a fresh start.

> **On the first note: the boundary is not 72 %, and it is not one profile.**
> `zebra2 -e` and `zebra -e` disagree about what the engine is — 59.7 % / 29.0 %
> saturate-vs-match on one, 25.6 % / 66.9 % on the other — so the phase has to
> serve both, and the target that is *missed* is the one the matcher dominates.
> The boundary is ≥ 41.4 % cumulatively and 3.6 % of self time; the number that
> replaced its 72 % is **21.1 % inside the compiler**, which no design section
> had a stage for. [S1a.6.3](s1a.6.3_beta_memories.md) is still not the next
> stage — [S1a.6.8](s1a.6.8_compile_cache_and_extents.md) is — but its gate
> opens rather than closes: 66.9 % of `zebra -e` is the join, and a fork's
> delta is 3.6 KB, which is the fact F11 D1 was parked on.
>
> **On the second: artefacts land in `ein.rs/bench-out/`** (git-ignored,
> machine-specific) — `e2e.json`, `py-cpython.json`, `py-pypy.json`,
> `prof-zebra2-e.json`, `prof-zebra-e.json`, `work-py.json`, `criterion.json` —
> and the tables derived from them are committed in [baseline.md](baseline.md),
> the same split `utils/feature_matrix_results.json` uses. A re-measure diffs
> the JSON; a reader reads the tables.
