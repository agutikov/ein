# S1a.12.5 — What `exhausted` means

**Phase:** P1a.12 (Exhaustive search over many models)
**Estimate:** 2 days
**Depends on:** [S1a.12.3](s1a.12.3_stopping_criterion.md),
[S1a.12.4](s1a.12.4_conflict_mining.md)

## Context

Whatever the two preceding stages land, the user-visible contract needs
restating. Today a run reports `exhausted true|false` and a verdict, and the
under-determined regime exposes a gap between them: a run that stops at the
depth cap reports `exhausted false` and `truncated`, which is honest but says
nothing about *how far* from exhaustion it got — 32 of 32 models, or 32 of
unknown-many?

## Acceptance

- The vocabulary is settled and documented: **`exhausted`** means the lattice
  was exhausted and the model set is proven complete; anything less says so in
  a different word. A heuristic stop never sets it.
- `Ambiguity` reports what is known about completeness. "k = 32, exhausted"
  and "k = 32, cap reached at depth 5" are different answers to the user's
  question and should not print the same.
- If [S1a.12.3](s1a.12.3_stopping_criterion.md) found a sound criterion, the
  verdict says *which* argument closed the search — exhaustion, or the
  criterion — because they are different guarantees and a reader deserves to
  know which one they have.
- `docs/api/inference.md` and the CLI help carry it; a contract that lives only
  in a plan is not a contract.
- Whatever the corpus entry for `zebra2-minus-15.ein` says about `solve -e` is
  updated to match reality — today its note reads "the exhaustive search is
  large rather than pathological", written before anyone measured it.

## Tasks

### Task T1a.12.5.1 — The vocabulary
### Task T1a.12.5.2 — The verdict surface
### Task T1a.12.5.3 — Docs and help
### Task T1a.12.5.4 — The corpus note, corrected

The note excludes `solve -e` on the grounds that it "outlives a 150s budget
under CPython, and a run nobody can finish is not coverage" — both still true,
and the phase either makes the run finishable or documents that it is not,
with the measured reason.

## Notes

- This stage is small and worth its own slot because it is the one a
  performance phase forgets. The engine's verdict is the product; a faster
  search that reports the same word for two different guarantees has made the
  product worse.
