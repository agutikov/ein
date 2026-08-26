# S1d.10.2 — What depth is required, and for what

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 2 days → **1 day** after the 2026-08-26 reconnaissance
(§ What is already taken), of which the census half is already banked
**Depends on:** [S1d.10.1](s1d.10.1_why_it_does_not_finish.md)
**Runs 2nd of six.**

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

> **Settled 2026-08-26 without the flag, and the answer is *no***
> ([README §5b](README.md#5-two-things-the-phase-believed-measured-false)).
> The 0.674 ms is the census's `--jobs 1` run and the 0.085 ms is not, so the
> two rows were never comparable. Re-taken at a constant `-j16` on the
> obligations twin — `-m 5`: 618 076 enterings in 50 181 ms, **0.0812 ms**
> each; `-m 6`: 1 483 833 in 123 361 ms, so layer 6 alone is 865 757 enterings
> in 73 179 ms, **0.0845 ms** each; and against the banked `-m 38` run, layers
> 7–22 are 15 720 759 enterings in 1 372 639 ms, **0.0873 ms** each. The
> per-entering cost **rises 7.5 % over seventeen layers**. It does not fall by
> 87 %.
>
> The proposed mechanism is refuted by the same pair of runs from the other
> side: `dead_post` is **19 129** at `-m 6` and **19 129** at `-m 38`, so
> layers 7–22 contain **no deaths at all** — deep enterings are not dying at
> their first firing, because they are not dying. What that leaves is the
> flattest possible reading of the regime: `Σₖ C(alive, k)` enterings at a
> uniform price, with nowhere for a cost to be concentrated and nothing for
> [S1d.10.4](s1d.10.4_conflict_mining.md) to attack.

## What is already taken — 2026-08-26

The stage is **mostly answered by its own predecessor**, and what is left is
smaller than the estimate. Against the acceptance below:

| acceptance bullet | where it stands |
|---|---|
| `d_found` and `d_stop` per entry, and *why* the search stopped | **taken** — [layer census §7](layer_census.md#7-d_found-against-d_stop--a-down-payment-on-s1d102), plus the `-m 38` run where `d_stop` is the lattice ending rather than the cap |
| the gap's cost in enterings and wall | **taken, and it is a function rather than a number**: 92.1 % of the run past the last new model at `-m 5`, 99.54 % at `-m 10`, 99.72 % at `-m 38` |
| whether `d_found` is knowable in advance | **open**, and it is T1d.10.2.3 — the one live task |
| what `max_set_size = 5` is doing | **taken** — `saturation/type-exclusivity/*` finds one model at depth 4 and four more at depth 5, so the default is load-bearing; the phase's own entry finds none past 3 through depth 22 |

So the stage costs **1 day, not 2**, and it runs second because the baseline it
produces is what [S1d.10.6](s1d.10.6_the_traversal.md) is measured against.
Two things it should do that the original text did not ask for:

- **Re-state the gap as a ratio against the terminating run**, not against a
  cap. `d_stop − d_found` was written when `d_stop` was whatever `-m` said;
  it is now 22 − 3 on the entry the phase is named after, and *that* is the
  number a cheaper argument has to beat.
- **The census re-take is already banked** — [`layer_census.md` §10](layer_census.md#10-the-re-take--2026-08-26-and-what-p1d2-and-p1d3-moved),
  taken by the reconnaissance because it was one command and the stage would
  have rested on it anyway. Read it before T1d.10.2.3: the predictor columns
  that task is about are the ones in that table.

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

**Answered in both directions and neither is the general rule** — `yes` for
`saturation/type-exclusivity/*`, `no` for the phase's own entry through depth
22. What is left of the task is the *conclusion*: the default is load-bearing,
so lowering it changes answers, and raising it changes nothing but the bill.

### Task T1d.10.2.5 — The census re-take — **taken 2026-08-26**

`utils/layer_census.py --layers --json` on the engine P1d.2 and P1d.3 left,
banked as [`layer_census.md` §10](layer_census.md#10-the-re-take--2026-08-26-and-what-p1d2-and-p1d3-moved)
by the reconnaissance rather than by this stage. Every moved row is the two
fixtures P1d.2 added; the phase entry's three layers reproduce to the digit; and
the two crosses the original census could not take are §10.1 and §10.2. What is
left for the stage is only what §10 explicitly declined: its `ms` and `MiB`
columns, which were taken on a shared machine and are not comparable with §4's.
Re-take them on a quiet one if any conclusion is going to rest on a wall clock.

## Notes

- Keep "found all models" and "proved there are no more" in separate columns
  everywhere in this phase. Conflating them is what makes the current
  behaviour look like a performance problem when it is a *termination-argument*
  problem.
