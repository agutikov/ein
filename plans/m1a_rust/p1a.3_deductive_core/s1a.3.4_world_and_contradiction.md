# S1a.3.4 — The NAF boundary

**Phase:** P1a.3 (Deductive core)
**Estimate:** 4 days
**Depends on:** [S1a.3.3](s1a.3.3_saturator.md)
**Implements:** `ein/inference/world.py` + `Saturator._admit_from_boundary`,
[design/06](../design/06_saturation.md) §4

## Context

`(absent P)` is a query against a **world** — a saturated KB plus the
commitment that produced it — not a ground atom. S1.21.8 made that
literal: guards are lifted out of the closure at compile time and judged
at quiescence, one admission per round.

It is also, measurably, where an exhaustive solve spends **72 %** of its
time. This stage ports the semantics exactly and then makes the boundary
semi-naive — the port's single largest algorithmic win, and one that
changes no verdict.

## Acceptance

- T2 parity for `park` / `admit` / `retire` / `quiesce` events on the
  whole corpus, including `examples/features/03_forall.ein` and
  `04_open.ein`, and including the classic unstratifiable fixture
  (`p ← absent q; q ← absent p` — add one if the corpus lacks it).
- `naf_rounds` / `naf_admitted` / `naf_retired` identical.
- `absent_premises` recorded on every admitted firing's provenance,
  identical to ein.py's `NafRef` tuples (relation + grounded arg pattern
  with `None` where the query ranged free), deduped preserving first-seen
  order.
- Guard sub-plan **evaluations** down ≥ 80 % on exhaustive zebra2 with
  the event sequence unchanged.

## Tasks

### Task T1a.3.4.1 — `World`

`holds` / `absent` / `admits` / `first_failing` / `negative_premises`,
with `project(bindings, scope)` restricting to the guard's scope.
`commitment` is carried and deliberately **inert** — ein.py pins that
with a test, and the reason it stays inert (a fork's facts already
include its hypotheses, so every query is branch-relative by
construction) is worth keeping in a comment.

### Task T1a.3.4.2 — `negative_premises`

Ground each guard sub-plan's relation patterns against the projected
bindings, recursing into nested guards, dedup preserving order. `_ground`
returns the bound value for a `Reg`, the name for a `Const` atom, the
value for an int, a nested tuple for a `Nested`, and `None` when free.

### Task T1a.3.4.3 — The boundary round

Pop parked candidates in priority/FIFO order; skip those already fired;
skip those whose watch state is unchanged; evaluate `first_failing`;
admit the first that passes and **stop**; retire a candidate whose
failing guard is monotone; re-park the rest. One admission per round —
the batch alternative is unsound and the reason is a fixture, not a
comment.

### Task T1a.3.4.4 — Version counters

Replace the `_watch_stamp` size tuple with per-relation version counters
(sizes are monotone, so equal versions ⇔ equal extents). Store the
last-seen version vector per parked candidate.

### Task T1a.3.4.5 — Semi-naive guard re-evaluation

For a **monotone** guard that previously found nothing: re-evaluate by
seeding the sub-plan at the delta facts in its watched relations, rather
than re-running the whole query. Exact, because a purely positive query's
match set only grows, so a new match must use a new fact. For a
**non-monotone** guard (nested absent), full re-evaluation.

Ship this behind the T2 diff and with the argument written down in the
code, next to the `monotone` flag it depends on.

### Task T1a.3.4.6 — Per-round memo

Memoise `(guard, projected_env) → verdict` for the duration of one
round. Sound because the KB cannot change mid-round.

### Task T1a.3.4.7 — Dirty set

Index parked candidates by watched relation; evaluate only the ones whose
watched relations moved, walking the parked structure in the same
priority/FIFO order so the admitted candidate is the same one.

## Notes

- Tasks 4–7 are four independent exact optimisations. Land them **one at
  a time**, each with a full T2 re-diff, so an ordering regression is
  attributable. Landing them together is the fastest way to spend a week
  bisecting.
- `naf_dropped` must stay structurally 0. If it can be non-zero in
  ein.rs, the boundary was rebuilt wrong — there is no enqueue/fire race
  to lose any more.
