# P3.3 — Three task classes on SMT

**Estimate:** ~1 week.
**Depends on:** P3.1 + P3.2.
**Blocks:** P3.5.

## Goal

Implement the three task classes from
[`docs/ideas/03-three-task-classes.md`](../../../docs/ideas/03-three-task-classes.md)
on the SMT backend, mirroring the M1 graph-native implementations
so the user can switch backends transparently:

| class | SMT pattern                                          |
|-------|------------------------------------------------------|
| Solve | `(check-sat) + (get-value …)`                        |
| Gaps  | model enumeration + agreement check                  |
| Contradictions | `(get-unsat-core)` + MUS refinement         |

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S3.3.1  | Solve / gaps / contradictions impl | 5-7 days |

## Acceptance

- `ein solve --backend=smt:z3 --mode=solve` returns the canonical
  Zebra answer.
- `--mode=gaps` on a Zebra-minus-1 returns at least the colour of
  house 1 as free.
- `--mode=contradictions` on a `zebra-bad.ein` returns a 2-3 named
  SMT-assertion unsat core; the lift-back (P3.4) names the IR
  facts.

## Connections

- [Idea 03 §Why distinguishing these matters](../../../docs/ideas/03-three-task-classes.md#why-distinguishing-these-matters) — the table is the source of truth.
