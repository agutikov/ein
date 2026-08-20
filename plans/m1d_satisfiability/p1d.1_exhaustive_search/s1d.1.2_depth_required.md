# S1d.1.2 — What depth is required, and for what

**Phase:** P1d.1 (Exhaustive search over many models)
**Estimate:** 2 days
**Depends on:** [S1d.1.1](s1d.1.1_why_it_does_not_finish.md)

## Context

The measurement that started the phase separates two numbers that the engine
does not distinguish:

- the depth at which **every model has been found** — 3, on zebra2-minus-15;
- the depth at which the search **can stop** — 5, the `max_set_size` cap, and
  only because the cap says so rather than because the lattice is exhausted.

Between them sit layers 4 and 5: tens of millions of enterings that find
nothing and exist only to fail to find anything. If that gap is a general
property of the under-determined regime, it is where the entire cost is.

## Acceptance

- Per corpus entry, both depths, measured: **d_found** (last depth that
  yielded a model not already known) and **d_stop** (the depth at which the
  search terminates, and *why* — lattice exhausted, `alive` empty, or the cap).
- The gap `d_stop − d_found`, and what it costs in enterings and wall clock.
- **Whether `d_found` is knowable in advance.** Probably not in general; the
  useful form of the question is whether anything *observable at layer d*
  predicts that layer d+1 will yield nothing. If something does, it is
  [S1d.1.3](s1d.1.3_stopping_criterion.md)'s input; if nothing does, that is
  a result and S1d.1.3 has to look elsewhere.
- **What `max_set_size = 5` is doing.** It is a default nobody has re-examined
  in this regime: on zebra2-minus-15 it is the only reason the search
  terminates at all, and a run that stops because of the cap reports
  `truncated`, not `exhausted`. Whether the default is right, and whether a
  puzzle can *need* depth 6, are both open.

## Tasks

### Task T1d.1.2.1 — Measure both depths across the corpus
### Task T1d.1.2.2 — The gap's cost
### Task T1d.1.2.3 — Predictors at layer d

Candidates to test against the census: models found this layer, new clauses
this layer, the ratio of alive to entered, whether `alive` shrank. Report
which correlate and which do not — a negative result here is worth as much as
a positive one, and cheaper to trust.

### Task T1d.1.2.4 — Is depth 5 ever needed?

Across the corpus: does any entry find a model at depth 4 or 5? If none does,
the default is doing nothing except making the under-determined regime
expensive, and that is a finding about the default rather than about the
search.

## Notes

- Keep "found all models" and "proved there are no more" in separate columns
  everywhere in this phase. Conflating them is what makes the current
  behaviour look like a performance problem when it is a *termination-argument*
  problem.
