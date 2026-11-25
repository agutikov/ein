# E-37 — `Open` over an empty R does not tell a problem with no solution from a formalization that is unfinished

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** [F26](../../../followups/f26_consistency_and_completeness.md)

## What

**`Open` over an empty R does not tell a problem with no solution from a formalization that is unfinished.** Two corpus programs whose own text says *no solution* answer `Open`, and so does a cyclic order over a rank bijection

## Evidence

`ein solve examples/binary_logic/unsatisfiable.ein --single`, `examples/beyond_arith/equation_no_solution.ein`; [`rank_unsat.ein`](../../analysis/probes/p2.3/rank_unsat.ein)

## What it costs the application

the kernel's word cannot be the benchmark's class *unsatisfiable*: the class is the canonical program's to state
