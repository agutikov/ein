# P3.5 — Hybrid orchestration

**Estimate:** 3-4 days.
**Depends on:** P3.1-P3.4.
**Blocks:** M3 done.

## Goal

The default mode. Graph engine drives; SMT handles slices the
engine declares hard. Per
[`docs/ideas/02 §Pragmatic note`](../../../docs/ideas/02-graph-as-formal-substrate.md#pragmatic-note),
this is the most honest realisation of the design: the IR owns the
proof; solvers contribute, they don't replace.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S3.5.1  | Hybrid driver + handoff policy     | 3-4 days |

## Acceptance

- `ein solve --backend=hybrid` is the M3 default.
- Trace shows graph-engine steps interleaved with SMT slice
  invocations.
- For Zebra, the graph engine handles everything; the trace shows
  zero SMT invocations.
- For a deliberately arithmetic-heavy puzzle (a P3.5-introduced
  micro-puzzle that adds a "sum-of-positions" constraint), the
  engine flags a slice and SMT solves it.

## Connections

- [Idea 02 §When does the graph stop being enough](../../../docs/ideas/02-graph-as-formal-substrate.md#open-questions) —
  the threshold criterion.
- [Idea 02 §Where it lands](../../../docs/ideas/02-graph-as-formal-substrate.md#where-it-lands-compared-to-graph--ir-for-solver) — the philosophical position.
