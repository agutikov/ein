# P1d.3 — Model sets without enumeration

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 1.5 weeks (7 days of stages)
**Depends on:** [P1d.2](../p1d.2_obligations/README.md) — the question only
becomes tractable once a state can say what it still owes.

**Depth: stage files, written 2026-08-25** — three of them, and the phase did
change shape before it started, exactly as
[§ How deep this plan is](../README.md#how-deep-this-plan-is) predicted it
might. What changed it is a reconnaissance measurement, below: the phase's
central hope is false on the phase's own case.

## Goal

**Decide whether 32 models should be printed or described.** The note's second
conclusion, after the one about obligations:

> all solutions compactly = saturation over symbolic constraints

— and its motivating complaint, that with several solutions there is no way to
find them without enumerating every hypothesis. A partial state with three
independent open choices *is* eight models; writing them out is a presentation
decision, and an expensive one.

**That "independent" is the load-bearing word, and the next section is where it
fails.**

## Why it is not free — measured 2026-08-25

The paragraph this section used to open with said: *if the open choices are
**independent**, the state is already the compact answer — the model count is
the product of the candidate-set sizes, and no search is needed to report it.*

**They are not independent.** A reconnaissance over the 32 models of
`examples/zebra2-minus-15.ein`, at three granularities. The model set comes
from one run —

```sh
ein solve -e -m 3 examples/zebra2-minus-15.ein --json-summary m15.json   # 25.5 s
```

— and everything below is `verdict.solutions` read as 32 fact sets;
[S1d.3.1](s1d.3.1_what_the_models_differ_in.md) turns the reading into
`utils/model_set_census.py` and re-takes it over every multi-model entry:

| granularity | test | result |
|---|---|---:|
| by relation | is `color-loc`'s projection independent of `pet-loc`'s? | **all 10 pairs coupled** |
| by attribute | 23 varying decision variables `(relation, value) → House` | product of domains **9.95 × 10¹³** against 32 models |
| by any partition | connected components of the coupling graph | **one**, containing all 23 |

So the free-by-product path is closed, and P1d.2's candidate sets are not the
compact answer. What is left is the second half of the old paragraph — a
decision graph, a disjunctive store, a BDD/ZDD, projected model counting —
priced in [S1d.3.2](s1d.3.2_representations.md), and **the honest possible
outcome is still "enumerate, and say so"**: a compact form that nobody can read
is worse than a list.

Three findings survive the reconnaissance as leads, each resting on n = 1 and
each re-taken properly in [S1d.3.1](s1d.3.1_what_the_models_differ_in.md):

- **78 % of every model is shared** — 340 of 435 facts hold in all 32. A
  "certain core plus a varying frontier" costs nothing and is *lossy*: it is
  the smallest box round the model set, and the box has 10¹³ cells where the
  set has 32.
- **The minimum determining set is four variables** (22 of 8 855 quadruples;
  no triple works), and it does not compress the way independence would — the
  quadruple ranges over **32 of the 320** combinations its domains allow, so
  the description is a 32-row table four columns wide instead of twenty-five.
- **Two of the 25 decision variables are fixed**, and they are the puzzle's two
  stated arrows — `Milk@House-3` and `Norwegian@House-1`. The same asymmetry
  [S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md) found
  from the other end, where `nation-loc` and `drink-loc` owe 8 at root and the
  other three owe 10.

## The corpus offers one case

Nine entries report `k > 1` under `solve -e -m 2`. Seven are two- or
three-model toys (`branching/02`, `04`, `06`, `08`, `12`,
`features/11_expect_ambiguity`, `lattice/02`); the other two are
`zebra2-minus-15.ein` and its obligations twin, at 28 by depth 2 and 32 by
depth 3.

**So this phase decides presentation on n = 1**, and that is a fact about the
corpus rather than about the plan. It sets the burden: shipping a
representation needs an argument that survives having been tested on one
puzzle, and *enumerate, and say so* needs only the measurement.

## Stages

| stage | title | est. |
|---|---|---|
| [S1d.3.1](s1d.3.1_what_the_models_differ_in.md) | What the 32 models actually differ in | 3 d |
| [S1d.3.2](s1d.3.2_representations.md) | Candidate representations, and what each costs to produce and to read | 2 d |
| [S1d.3.3](s1d.3.3_the_verdict.md) | What the verdict says, and whether this ships | 2 d |

**S1d.3.1** is the measurement that decides the rest, and the reconnaissance
above is its first hour: take the 32 models of `zebra2-minus-15` (established
independently in [M10](../../m10_external_benchmarks/README.md)) and ask
whether they factor. They do not — so the stage's job is no longer *does the
compact form fall out of P1d.2* but **what the coupling is made of**, which is
what tells S1d.3.2 whether any representation can exploit anything. It also
takes the number [P1d.2 handed
forward](../p1d.2_obligations/hypotheses_from_obligations.md): the per-state
leftover-open count, whose probe P1d.2 declined because it would have measured
a different engine.

**S1d.3.3 inherits two questions rather than one.** Besides
[Q-M1d.5](../open_questions.md#q-m1d5--print-or-describe) it owns the
**closed-world completion** question — `ideas.md`'s *обязательно ли назначать
значение каждому возможному факту?* — which both
[`domain_contract.md` §3](../p1d.2_obligations/domain_contract.md) and
[the openness census §6](../p1d.2_obligations/openness_census.md) deferred here
by name. It does not have to adopt it; it has to say which semantics a reported
model set is under, because a compact form is a claim about a family of graphs
and the family's size depends on the answer.

## Acceptance for the phase

- **A written answer to "print or describe"**, with the factorisation of a
  real model set behind it rather than an intuition.
- If something ships, it is **additional output, not a replacement**: the
  models remain enumerable, because every consumer — the trace, the GUI,
  `:expect`, the benchmark adapters — reads models.
- Whatever is reported carries **the same guarantee vocabulary** the rest of
  the milestone settles: a compact description of a model set claims
  completeness only when the search proved it. **On this phase's own case it
  has not**: `solve -e zebra2-minus-15` is `Ambiguity k=32, exhausted=false`
  ([layer census §4](../p1d.10_exhaustive_search/layer_census.md#4-zebra2-minus-15-all-five-layers)),
  so a "certain core" computed by intersecting 32 models that might not be all
  of them is certain of nothing — a 33rd could contradict any of its 312 facts.
  Intersecting a subset gives a superset of the truth, which makes this the
  easy mistake rather than a remote one, and
  [S1d.3.3](s1d.3.3_the_verdict.md) owns the fixture that catches it.

## Risks

- **This is the research end of the milestone** and it can absorb arbitrary
  time: model counting and knowledge compilation are entire literatures
  ([`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md),
  [`11`](../../../docs/lib/11-search-optimization-algorithms.md)). Three
  stages is a decision budget, not an implementation one, and the phase is
  scoped to end in a written decision.
- **A compact answer is a new thing to explain.** Ein's differentiator is the
  human-readable trace ([idea 08](../../ideas/08-human-style-deductive-trace.md));
  a BDD is the opposite of that. Anything shipped here has to be readable by
  the same person who reads the trace, or it belongs in a followup.
