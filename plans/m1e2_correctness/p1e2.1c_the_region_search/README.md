# P1e2.1c — The region search: R past the oracle's reach

**Estimate:** ~2.5 weeks: 4 stages, **13 days**, since 2026-11-02. It was 5
stages and 17.5 days until P1e2.1b's re-plan that day moved
[S1e2.1c.3](s1e2.1c.3_the_non_monotone_search.md) (4 days) to
[F21](../../followups/f21_non_monotone_programs_decided.md)'s D7 and scoped
[S1e2.1c.4](s1e2.1c.4_the_flags_at_scale.md) to monotone programs (4 → 3.5
days). The stages waited for
[S1e2.1a.9](../p1e2.1a_the_solution_region/s1e2.1a.9_solve_describes_the_region.md)
to measure how far its routes reach, which it did on 2026-10-16
([`route1_reach.md`](../p1e2.1a_the_solution_region/route1_reach.md)), and for
[S1e2.1b.6](../p1e2.1b_the_non_monotone_lattice/s1e2.1b.6_the_two_lattices_compared.md)
to write down the strategies, which it did on 2026-11-02
([`monotonicity.md` § 6](../../../docs/kernel/inference/monotonicity.md#6-how-to-solve)). They
were specified on 2026-11-02 with what is known now. P1e2.1b rules nothing
since its re-plan the same day, so no stage here waits on a ruling: every
route this phase builds is for **monotone** programs.
**Status:** created 2026-10-16, on the user's *"Q-M1e2.8 - yes"*. That ruled
the plan's recommendation: the oracle becomes `solve`'s answer now
(S1e2.1a.9), and the region search is a phase of its own, next.
**Depends on:** S1e2.1a.9 (the default, its routes and its flags) and
[P1e2.1b](../p1e2.1b_the_non_monotone_lattice/README.md): its detector
(S1e2.1b.5), whose warnings every new route carries and keeps free on an
unflagged program, and S1e2.1b.6's strategies for the monotone case.
**Scope, since 2026-11-02.** The user deferred every decision about a
non-monotone program to F21. So this phase's routes are for programs the
static check calls monotone. A flagged program keeps what it has today:
route 2, exact at n ≤ `--region-limit`, partial past it, with P1e2.1b's
warnings. A route that built a node from a parent's fixpoint on a flagged
program could depend on the path and disagree with the oracle
([`monotonicity.md` § 7.2](../../../docs/kernel/inference/monotonicity.md#72-paths-and-the-general-form)),
which is F21's D3.
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

## Stages

**The first step is the state route** (the user, 2026-10-16: *"(A) now, with
(B) as the first step of P1e2.1c"*). It visits each alive state once and
counts k_c over the state lattice. It is
[S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md)'s first half, and it is what
`--maximal`, `--single` and describe need to stop costing |alive|.

Every stage has a file since 2026-11-02. S1e2.1c.5 also takes the old walk's
fate, which the milestone's Phases table gives this phase and no row named:
the walk's library, and the unread levers `configuration.md` says this phase
gives a meaning or retires.

| ID | title | the question |
|---|---|---|
| [S1e2.1c.1](s1e2.1c.1_how_large_the_alive_set_gets.md) | How large the alive set gets | route 1's reach on the zebra family and the slow entries, and what a description costs as the alive set grows. First answer: [`route1_reach.md`](../p1e2.1a_the_solution_region/route1_reach.md) (zebra counted; `zebra2-minus-15` and the seven everything-alive cubes not) |
| [S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md) | The lines by duality | for a monotone program only, enumerate N and E and dualise (MARCO-style, incremental hypergraph transversal), so that the lines, and with them k_s and k_c by sub-cubes, cost output size rather than \|alive\|. [`walk_and_region.md` § 6](../p1e2.1a_the_solution_region/walk_and_region.md#6-what-this-means-for-question-3) measured the state lattice beside the commitment one: zebra has 27 alive states against 270 454 alive commitments, and a route over states costs at most *states × n* |
| [S1e2.1c.3](s1e2.1c.3_the_non_monotone_search.md) | The non-monotone search | **moved to [F21](../../followups/f21_non_monotone_programs_decided.md)'s D7, 2026-11-02.** Its file stays, with a banner, as the design notes the decision will start from |
| [S1e2.1c.4](s1e2.1c.4_the_flags_at_scale.md) | The flags at scale | on a monotone program, *any*, *single*, *minimal* and *maximal* by the cheapest sound route for each, with the certificate each one is entitled to. The target is [`solve_interface.md` § 3.2](../p1e2.1a_the_solution_region/solve_interface.md#32-a-monotone-program-each-modes-route)'s right-hand column. It includes licence L5, the obligation tree, which decides `zebra2-obligations`' uniqueness in 9 enterings, and whether a `--count` is worth a flag |
| [S1e2.1c.5](s1e2.1c.5_wrong_answers_re_scoped.md) | P1e2.2 re-scoped | which register rows the new default closes by construction, and which survive into it. **The walk is not kept**: the user withdrew `ein walk` on 2026-10-16, and its rows flipped at S1e2.1a.5's switch ([`solve_interface.md` § 6](../p1e2.1a_the_solution_region/solve_interface.md#6-what-moves)): 24 to `fixed` on 2026-10-18. *Forbid* re-opened W3's four on 2026-11-02, which leaves the register at 21 fixed and 14 open. Since the same day it also carries W8's tension with the milestone's acceptance, parked for the revisit |

## Where it stops

It makes `ein solve`'s default describe R for every **monotone** program the
corpus holds, or say precisely why not. A flagged program is described as
today, route 2 within the limit and partial past it, with P1e2.1b's warnings.
What R means is P1e2.1a's (monotone); on the rest it is open, and F21's. The
search past the limit on a flagged program is F21's D7. The individual wrong
answers that survive are P1e2.2's.
