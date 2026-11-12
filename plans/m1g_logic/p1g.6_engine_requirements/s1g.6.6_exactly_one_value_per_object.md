# S1g.6.6 — A cheaper search where each object takes exactly one value

**Phase:** [P1g.6](README.md) — candidate C-12
**Status:** planned 2026-11-12, on the user's reply of that day to the calculator's questions. Not started.
Provisional until P1g.1's stop, as the phase's other stages are.
**Depends on:** [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md) and
[S1g.2.4](../p1g.2_propositional_logic/s1g.2.4_applications_and_scale.md), which are the stages that need it
and that bring its witnesses; of M1f,
[P1f.10](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md), the structure of
the hypothesis set, whose groups (*pick at most one from each*) are this shape seen from the other side.
It starts from what P1f.10 built and does not repeat it.

## Where it comes from

Question 25 of the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q25): *Is a cheaper
search for "exactly one value per object" a requirement?* With each input a hypothesis that takes 0 or 1,
describing R costs 3ⁿ saturations for n inputs, and finding one state 2ⁿ; the same truth table from written
input vectors is one saturation. Its options were (a) *the search form is a demonstration; truth tables are
computed from written vectors; no engine work*, and (b) *it is a requirement for M1g's valuations, and a
cheaper route for this shape of hypothesis is planned there*. The user, 2026-11-12:

> Is this about SAT, binary circuits or what? I don't see how (a) and (b) conflict, are they?

[Round 2, item 25](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q25):

> It is about SAT, in Ein's form. […] Nothing in the engine knows that *exactly one value per object* is a
> special, very common shape, which a SAT solver exploits with unit propagation and learned clauses.
>
> You are right that they do not conflict. (a) said *this phase needs no engine work*, (b) said *M1g will*.
> Both hold.

and, meanwhile: *P1f.6 treats the search form as a demonstration, and M1g's engine-requirements phase gets
a stage for a cheaper route on this shape, with the table of `questions.md` 25 as its witness.* This is
that stage. The user's reply in round 2, committed as `dc88520b`: *"yes, both"*.

## The witness

The table of `questions.md` 25 (`python3 scale.py`, `scale2.py` under `build/s1f.6.17/`, 2026-11-11, budget
raised to 2³⁰):

| inputs | by search: describe R | by search: one state | written vectors: one saturation |
|---|---|---|---|
| 12 | 531 453 saturations, 11.7 s | 4 096 saturations | 49 152 firings, 64 ms |
| 16 | no answer in 150 s, 8 GiB | 65 536 saturations, 2.9 s | 1 048 576 firings, 2.3 s |

The shape: n objects, each owing exactly one of k values (`total-owed` and `functional`, or
`std.bool.inputs`' obligation). The alive nodes are the partial assignments, (k+1)ⁿ of them; the complete
ones are kⁿ. M1g's valuations ([S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md))
are this shape with k = 2, and so are the calculator's unknown digits with k = 10 and every zebra
bijection.

## Why the logic needs it

A truth table of n letters is R of 2ⁿ states, and the milestone's README says so *without apology: it is
the semantics made visible, not a solver*. What is not the semantics is the 3ⁿ: the partial valuations are
alive nodes the engine visits and describes on the way, and they are no row of any truth table. So the
question is narrow: **for this shape, can describing R, counting it, and finding one state each cost what
the complete assignments cost, or less, with R unmoved?**

## What it tries

Each on the witnesses first, measured in saturations and through `utils/bench_env.sh`; none assumed:

1. **What already exists.** `--single` and `-n` walk the obligation tree, which branches on one owed
   instance's alternatives and never visits a partial assignment as a state to describe: 4 096 and 65 536
   in the table are that. `--count` counts R by describe's route. The first task is the table of every mode
   on the shape at several n, so that the gap is located: it may be describe alone.
2. **Describe by the obligation tree** where the program's every hypothesis is an alternative of an owed
   instance: the states of R are then the tree's leaves, and the partial assignments are owing nodes that
   need not be entered to be counted. What describe prints of them (the owing frontier, N) is what
   [P1g.7](../p1g.7_unsatisfiable_against_owing/README.md) is ruling on, so the two are read together.
3. **Propagation.** A value forced because every other alternative of its instance is dead (unit
   propagation in a solver's words) and a dead set learned from one branch used in another. The engine has
   minimal dead sets; what it does not do is *conclude* from them inside a state. Whether that is this
   stage, [S1g.6.5](s1g.6.5_rules_as_data_and_refutations.md)'s *a refutation a rule can read*, or P1g.7's
   propagation is settled at P1g.1's stop; here it is measured as an experiment on a prototype and priced.
4. **E-31's transversal:** describing R spends its time on minimal transversals and not on saturations (94 %
   of 5.1 s on a five-digit equation, `questions.md` 72). M1f's
   [S1f.6.27](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.27_ein_test_by_the_cheapest_route.md)
   takes the part that is free; what is left of it on this shape is here.

## Tasks

1. **T1g.6.6.1 — the modes on the shape,** k = 2 and k = 10, n to where each mode stops: saturations, time,
   memory, and where the time goes.
2. **T1g.6.6.2 — the design,** written, for whichever of 2 to 4 the measurements point at, with what it
   changes in the order states are met (and so in which explanations a run prints; a core is an explanation,
   but every surface is held by a golden).
3. **T1g.6.6.3 — the prototype,** held to `ein oracle` on every program of the shape the oracle reaches,
   and to describe as it is today on every corpus program.
4. **T1g.6.6.4 — the build,** only if the prototype flattens the cost and the sweep is clean; the route is
   chosen by the program's shape, never by a flag.
5. **T1g.6.6.5 — the page:** what a truth table of n letters costs after, and where it still ends.

## Acceptance

- For the shape, describing R, counting it and finding one state are each measured before and after, and
  the page says which improved.
- **R, N, H, every claim and every exit code are identical before and after on every corpus program,** and
  every mode agrees with the oracle where the oracle reaches.
- No option selects the route (*config never changes an answer*; a route a flag could switch is a lever,
  and is then held by `lever_invariance`).
- If nothing is built, the page says why, and S1g.2.4 states the limit with the witness's numbers.

## What it must not do

- **Not turn Ein into a SAT solver.** No clause database, no restarts, no decision heuristics beyond what
  P1f.10 derives from the program's structure.
- **Not change what a partial assignment is:** an alive node that owes. What it is *called* when it cannot
  pay is P1g.7's.
- **Not special-case the Boolean values.** The shape is *exactly one of k*, read off the program's
  obligations and restrictions, whatever the values are named.

## The discipline

[P1g.6 § The discipline](README.md#the-discipline): a wrong answer is a register row before it is fixed;
config never changes the answer; one tool, consistent answers; minimum work per question; every sweep names
the goldens it expects to move before blessing.
