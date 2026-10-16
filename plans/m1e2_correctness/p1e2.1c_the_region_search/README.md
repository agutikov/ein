# P1e2.1c — The region search: R past the oracle's reach

**Estimate:** not yet. Five stages are named and none is specified. They are
specified when [S1e2.1a.9](../p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
has measured how far its routes reach, and when
[S1e2.1b.6](../p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md)
has written down the strategies.
**Status:** created 2026-10-16, on the user's *"Q-M1e2.8 - yes"*. That ruled
the plan's recommendation: the oracle becomes `solve`'s answer now
(S1e2.1a.9), and the region search is a phase of its own, next.
**Depends on:** S1e2.1a.9 (the default, its routes and its flags) and
[P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md) (what R is for a
non-monotone program, and S1e2.1b.6's strategies).
**Blocks:** [P1e2.2](../p1e2.2_wrong_answers/README.md). Its stages are
re-scoped against this phase before any of them runs.

---

## Why a phase

The user, 2026-10-16: *"our default behaviour that searches for maximal
solutions is not completely wrong but it is relevant only in special case with
no obligations. So it has to be reworked"*, and *"the default simple `solve`
without flags we get exhausive certifying full region description"*.

S1e2.1a.9 gives the default an honest answer wherever it is cheap: the alive
down-set of a monotone program, and the 2ⁿ oracle at n ≤ the limit. Everywhere
else it says *partial*. The zebra family is past the oracle at n = 56, and
past route 1's default budget. Its alive set was counted on 2026-10-16
([`route1_reach.md` § 6](../p1e2.1a_the_solution_region/route1_reach.md#6-where-route-1-declines-and-zebras-region-counted)):

- `zebra.ein` has **270 454** alive commitments, and its R is **one state**
  of 261 982 commitments with 63 minimal generators. That took 25 s on 12
  jobs, which a reader can ask for and a default should not pay;
- `zebra2-minus-15` already holds 704 443 commitments of R at sizes ≤ 5, and
  each layer is 2.5–4× the last.

A default that says *partial* on the flagship puzzle is honest, but it is not
finished. This phase finishes it.

## Stages (named, not specified)

| ID | title | the question |
|---|---|---|
| S1e2.1c.1 | How large the alive set gets | route 1's reach on the zebra family and the slow entries, and what a description costs as the alive set grows. First answer: [`route1_reach.md`](../p1e2.1a_the_solution_region/route1_reach.md) (zebra counted; `zebra2-minus-15` and the seven everything-alive cubes not) |
| S1e2.1c.2 | The lines by duality | for a monotone program, enumerate N and E and dualise (MARCO-style, incremental hypergraph transversal), so that the lines, and with them k_s and k_c by sub-cubes, cost output size rather than \|alive\|. [`walk_and_region.md` § 6](../p1e2.1a_the_solution_region/walk_and_region.md#6-what-this-means-for-question-3) measured the state lattice beside the commitment one: zebra has 27 alive states against 270 454 alive commitments, and a route over states costs at most *states × n* |
| S1e2.1c.3 | The non-monotone search | S1e2.1b.6's guard split, or whatever P1e2.1b rules, past the oracle's limit |
| S1e2.1c.4 | The flags at scale | *any*, *single*, *minimal* and *maximal* by the cheapest sound route for each, with the certificate each one is entitled to |
| S1e2.1c.5 | The old walk, and P1e2.2 re-scoped | which register rows the new default closes by construction (W1, W2 and W5 are symptoms of walking for the maximal line), which survive into it, and whether the lattice walk is kept, as a lever, or retired |

## Where it stops

It makes `ein solve`'s default describe R for every program the corpus holds,
or say precisely why not. What R means is P1e2.1a's (monotone) and P1e2.1b's
(the rest). The individual wrong answers that survive are P1e2.2's.
