# P1a.3 — Deductive core

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3.5 weeks (18 days of stages)
**Depends on:** [P1a.2](../p1a.2_kb_core/README.md)
**Blocks:** [P1a.4](../p1a.4_search_layer/README.md)

## Goal

The engine proper: the pattern compiler, the matcher, the two-phase
closure/boundary saturator, the `World` NAF boundary, and contradiction
detection. At the end of this phase, `ein.rs saturate <file>` reaches
**T2 event-trace parity** — the same firings, in the same order, with the
same provenance — on every saturation-only fixture.

This is the phase where the port stops being a transcription and starts
being an engine: [design/05](../design/05_matcher.md)'s register machine
and [design/06](../design/06_saturation.md)'s two exact wins land here.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.3.1](s1a.3.1_compiler.md) | Pattern compiler → plan bytecode | 4 d |
| [S1a.3.2](s1a.3.2_matcher.md) | Register matcher, candidate probes, entry points | 5 d |
| [S1a.3.3](s1a.3.3_saturator.md) | Closure loop, semi-naive delta, queues, mirror | 5 d |
| [S1a.3.4](s1a.3.4_world_and_contradiction.md) | NAF boundary, negative provenance, clash detection | 4 d |

## Acceptance for the phase

- **T2** on `examples/saturation/**`, `examples/features/**`,
  `examples/domain_elim/**`, plus root saturation of `zebra.ein` and
  `zebra2.ein` (378 / 502 facts): identical firing sequence, identical
  per-fact provenance, identical alternative-justification lists.
- Counters identical: `naf_rounds`, `naf_admitted`, `naf_retired`,
  `naf_dropped == 0`, redundant-firing count, `len(engine.cache)`.
- `ein.rs saturate` output byte-identical (this is a T3 surface and it is
  small enough to close early).
- Compile-call count on exhaustive zebra2 down from **17 430** to one per
  distinct `(rule, activator)` pair (~170), with the cache *order*
  unchanged — [design/06](../design/06_saturation.md) § Win A.
- Guard sub-plan evaluations down ≥ 80 % with an identical
  park/admit/retire event sequence — § Win B.
- No heap allocation in the matcher's inner loop (counting-allocator
  test).

## Risks

- **Order drift is invisible until it is expensive.** Run the T2 diff
  from the first working saturation, not at the end of the phase.
- **The two "exact wins" are only exact if argued.** Both change *when*
  work happens, not *what* is derived; each ships with the T2 diff green
  on the whole corpus, and Win B additionally with the monotone/nested
  split written down and tested on a `forall` fixture
  (`examples/features/03_forall.ein`).

## Cross-links

- [design/05 — Matcher](../design/05_matcher.md)
- [design/06 — Saturation](../design/06_saturation.md)
- [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
- [`architecture_and_algorithms.md` §O1–O3, §O5](../../../docs/kernel/inference/architecture_and_algorithms.md)
