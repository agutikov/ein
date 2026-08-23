# S1d.10.1 — Why it does not finish

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 3 days

## Context

The instrument stage, and it comes first for the reason
[F9](../../followups/f9_e_catalog.md) gives: a search proposal judged against
the wrong regime is judged wrong. What is known is one puzzle's numbers; what
is needed is the shape of the regime.

The one mechanism already identified: **a layer that kills nothing learns
nothing**. Pruning in this engine comes from deaths — the learned clause and,
for a singleton commitment, the `(not h)` writeback. Layer 1 of
`zebra2-minus-15` has 96 candidates and 0 deaths, so the next layer is the
full `C(96,2)` with no clause to filter it. On zebra2, layer 1 kills 67 of 101
and that is what makes it tractable.

So the question is not "why is it slow" but **"what does a layer have to
produce for the next one to be affordable, and what happens when it produces
none of it?"**

## Acceptance

- A **clause-yield census**, per layer, per corpus entry: candidates entered,
  deaths, clauses emitted, clauses subsumed, writebacks, and **how many
  candidates the next layer's generation was filtered by them**. That last
  column is the one nothing currently reports, and it is the phase's core
  measurement.
- The corpus split into regimes by that census — not by hand. The expectation
  is at least two (determinate, under-determined) and possibly a third
  (deep-but-pruning: `branching/07 -e` is 11 501 enterings over 5 layers and
  finishes in a second).
- **Growth rate per layer** and how much of it the clause store removes. On
  zebra2-minus-15 the answer at layer 2 appears to be "nothing"; that should be
  a number.
- The memory profile alongside the time, because
  [baseline.md §15](../../../docs/history/m1a_rust/measurements/baseline.md) says the wall is likely
  to be RAM.

## Tasks

### Task T1d.10.1.1 — The census instrument

Per-layer counters exist in part (`nogoods_emitted`, `nogoods_subsumed`,
`enterings_*`); what is missing is the *effect*: how many candidates
`generate_layer` produced, and how many `filter_candidate` rejected against
the store. Add those, behind the same discipline
[`ein_core::counters`](../../../ein.rs/crates/ein-core/src/counters.rs) uses —
compiled out unless asked, because this is the hottest loop in the engine.

### Task T1d.10.1.2 — The corpus census

Run it over every entry, depth-capped where the entry does not finish. Report
the table.

### Task T1d.10.1.3 — Classify the regimes

From the census, not from intuition. The classifier should be something a
reader can apply to a new puzzle — "deaths per entering below x at layer 1" —
because [S1d.10.5](s1d.10.5_contract.md) may want to *report* it.

### Task T1d.10.1.4 — Where the time goes in this regime

A profile of `zebra2-minus-15 -m 3`, bucketed like
[`profile_ein_rs.py`](../../../utils/profile_ein_rs.py) does, against the
determinate profile in [baseline.md §3](../../../docs/history/m1a_rust/measurements/baseline.md). The
question is whether the under-determined regime is the *same* engine costs at
a larger count or a different mix — if `generate_layer` and `filter_candidate`
dominate where the determinate profile has the matcher and the boundary, the
optimisation targets are different ones.

## Notes

- Resist proposing anything in this stage. Its output is a table and a
  classification; the proposals are [S1d.10.3](s1d.10.3_stopping_criterion.md)
  and [S1d.10.4](s1d.10.4_conflict_mining.md), and both are better arguments
  for having this first.
