# S1d.10.2 — What depth is required, and for what

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 2 days
**Depends on:** [S1d.10.1](s1d.10.1_why_it_does_not_finish.md)

## Context

The measurement that started the phase separates two numbers that the engine
does not distinguish:

- the depth at which **every model has been found** — 3, on zebra2-minus-15;
- the depth at which the search **can stop** — 5, the `max_set_size` cap, and
  only because the cap says so rather than because the lattice is exhausted.

Between them sit layers 4 and 5: tens of millions of enterings that find
nothing and exist only to fail to find anything. If that gap is a general
property of the under-determined regime, it is where the entire cost is.

**The gap has since been measured at a second cap, and it does not close.** A
`-m 10` run of the obligations twin, 2026-08-25:
**10 587 736 enterings, 15 minutes, the same 32 models**
([layer census §4.1](layer_census.md#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing)).
So `d_found` is still 3, `d_stop` is still whatever `-m` says, and the fraction
of the run spent past the last new model moves **92.1 % → 99.54 %**. Two
consequences for this stage:

- **T1d.10.2.4 has both halves now.** The corpus question — *does any entry
  need depth 4 or 5?* — is **yes**, `saturation/type-exclusivity/*`
  ([census §7](layer_census.md#7-d_found-against-d_stop--a-down-payment-on-s1d102)),
  so the default is not dead. For the entry the phase is named after the answer
  is **no, through depth 10**. Neither is the general rule, which is precisely
  why a *criterion* is needed rather than a better default.
- **T1d.10.2.2's cost column is unbounded, not large.** It scales with the cap,
  so "what the gap costs" has no single number and should be reported as a
  function of `-m` — two points exist already.

One question the probe raised and could not answer: an entering at depth 10
costs **0.085 ms** against depth 5's **0.674 ms**, 7.9× cheaper. If the reason
is that deep enterings die at their first firing where §4's layers were 97 %
alive, then the barren regime's cost is concentrated shallow and
[S1d.10.4](s1d.10.4_conflict_mining.md) should be judged there. Settling it
needs the `layer` event at `-m 10`, which is one flag on
[S1d.10.1](s1d.10.1_why_it_does_not_finish.md)'s instrument.

## Acceptance

- Per corpus entry, both depths, measured: **d_found** (last depth that
  yielded a model not already known) and **d_stop** (the depth at which the
  search terminates, and *why* — lattice exhausted, `alive` empty, or the cap).
- The gap `d_stop − d_found`, and what it costs in enterings and wall clock.
- **Whether `d_found` is knowable in advance.** Probably not in general; the
  useful form of the question is whether anything *observable at layer d*
  predicts that layer d+1 will yield nothing. If something does, it is
  [S1d.10.3](s1d.10.3_stopping_criterion.md)'s input; if nothing does, that is
  a result and S1d.10.3 has to look elsewhere.
- **What `max_set_size = 5` is doing.** It is a default nobody has re-examined
  in this regime: on zebra2-minus-15 it is the only reason the search
  terminates at all, and a run that stops because of the cap reports
  `truncated`, not `exhausted`. Whether the default is right, and whether a
  puzzle can *need* depth 6, are both open.

## Tasks

### Task T1d.10.2.1 — Measure both depths across the corpus
### Task T1d.10.2.2 — The gap's cost
### Task T1d.10.2.3 — Predictors at layer d

Candidates to test against the census: models found this layer, new clauses
this layer, the ratio of alive to entered, whether `alive` shrank. Report
which correlate and which do not — a negative result here is worth as much as
a positive one, and cheaper to trust.

### Task T1d.10.2.4 — Is depth 5 ever needed?

Across the corpus: does any entry find a model at depth 4 or 5? If none does,
the default is doing nothing except making the under-determined regime
expensive, and that is a finding about the default rather than about the
search.

## Notes

- Keep "found all models" and "proved there are no more" in separate columns
  everywhere in this phase. Conflating them is what makes the current
  behaviour look like a performance problem when it is a *termination-argument*
  problem.
