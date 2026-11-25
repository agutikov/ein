# E-38 — `(transitive R)` with `(irreflexive R)` does not refute a cycle

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**`(transitive R)` with `(irreflexive R)` does not refute a cycle.** The library's `transitive` derives no self-loop, so a three-cycle is alive with all six pairs; `(asymmetric R)` refutes it

## Evidence

[`cycle_only.ein`](../../analysis/probes/p2.3/cycle_only.ein) (`Solution`), [`rank_unsat_strict.ein`](../../analysis/probes/p2.3/rank_unsat_strict.ein) (`Open`), [`rank_unsat_asym.ein`](../../analysis/probes/p2.3/rank_unsat_asym.ein) (`Contradiction`)

## What it costs the application

the textbook spelling of a strict order is weaker than it reads, in silence, and a model will write it
