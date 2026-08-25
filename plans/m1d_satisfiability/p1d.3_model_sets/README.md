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
**[S1d.3.1](s1d.3.1_what_the_models_differ_in.md) is done, 2026-08-25** — the
reconnaissance is superseded by
[`model_set_census.md`](model_set_census.md), over every multi-model entry and
not one, and the leftover-open probe P1d.2 handed forward is **taken** rather
than declined again.
**[S1d.3.2](s1d.3.2_representations.md) is done, 2026-08-25** — the five
candidates priced on four columns and the two the stage insisted be *printed*
are printed ([`representations.md`](representations.md)). It reverses the
stage's own prediction: **(a), "the candidate to beat", loses the readability
veto and (b) wins**, with its exactness verified rather than asserted.

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

## Why it is not free — measured 2026-08-25, over the whole corpus

The paragraph this section used to open with said: *if the open choices are
**independent**, the state is already the compact answer — the model count is
the product of the candidate-set sizes, and no search is needed to report it.*

**The product form exists, it is worth nothing, and the reason is a number.**
[`model_set_census.md`](model_set_census.md) is the measurement — every corpus
entry, each at the depth that finds every model it has — and it answers four
questions rather than the reconnaissance's three:

| granularity | test | result |
|---|---|---:|
| by relation | is `color-loc`'s projection independent of `pet-loc`'s? | **0 of 20 relation pairs independent** |
| by variable pair | is `proj(u,v)` the whole `dom(u) × dom(v)`? | `zebra2-minus-15`: **248 of 253 coupled** |
| by partition | components, with `Π \|proj(cᵢ)\| == k` | **2 of 13 entries** — and both have **two objects** |
| by basis | is the set a free grid over a determining key? | **5 of 13** — every one of them `k ≤ 4` |

The two that partition are `saturation/type-exclusivity/{colors,nationalities}`
— two-fact demos whose 9 models are 3 × 3 over two independent blocks. **The
same program with three instances (`pets.ein`) has one component and 35
models.** So the free-by-product path is real and it closes between two objects
and three; on `zebra2-minus-15` the 23 varying decision variables are **one**
component whose graph is K₂₃ minus five edges, with a minimum vertex separator
of **17**.

What is left is the second half of the old paragraph — a decision graph, a
disjunctive store, a BDD/ZDD, projected model counting — priced in
[S1d.3.2](s1d.3.2_representations.md), and **the honest possible outcome is
still "enumerate, and say so"**: a compact form that nobody can read is worse
than a list. The separator number is what prices the diagram family: a BDD is
small when some variable order has small separators, and here none does.

Three findings that were leads at n = 1 are now measured, and one changed:

- **78 % of every model is shared** — 340 of 435 facts hold in all 32 (312
  positive, 28 negative). A "certain core plus a varying frontier" costs
  nothing and is *lossy*: it is the smallest box round the model set, and the
  box has 9.95 × 10¹³ cells where the set has 32.
- **The minimum determining set is four variables** (22 of 8 855 quadruples; no
  triple works), it ranges over **32 of the 320** combinations its domains
  allow — and the *"why these four"* objection now has an answer: **two of them
  are in all 22 keys**, `pet-loc:Horse` and `pet-loc:Zebra`. Ten of the 23
  variables are in no minimum key at all.
- **Two of the 25 decision variables are fixed**, and they are the puzzle's two
  stated arrows — `Milk@House-3` and `Norwegian@House-1`. The same asymmetry
  [S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md) found
  from the other end, where `nation-loc` and `drink-loc` owe 8 at root and the
  other three owe 10.

And one the reconnaissance could not have found, because it needed a probe
rather than a reading: **`zebra2`'s unique model leaves 3 678 facts that the
blind enumerator would still propose** — none of them an attribute arrow, all
of them on the four relations the puzzle never closes, and most of them
ill-typed. Under a literal open-world reading that "one model" is 2³⁶⁷⁸ of
them. That is [S1d.3.3](s1d.3.3_the_verdict.md)'s closed-world question with a
number attached ([census §6](model_set_census.md)).

## The corpus offers one case

**Thirteen entries have a model set** at the depth that finds every model they
have — four more than the `-m 2` count this section used to quote. Eleven are
two- to thirty-five-model toys; the other two are `zebra2-minus-15.ein` and its
obligations twin, at 28 by depth 2 and 32 by depth 3.

**So this phase decides presentation on n = 1**, and that is a fact about the
corpus rather than about the plan. It sets the burden: shipping a
representation needs an argument that survives having been tested on one
puzzle, and *enumerate, and say so* needs only the measurement.

## Stages

| stage | title | est. |
|---|---|---|
| [S1d.3.1](s1d.3.1_what_the_models_differ_in.md) | What the 32 models actually differ in | 3 d — **done 2026-08-25** |
| [S1d.3.2](s1d.3.2_representations.md) | Candidate representations, and what each costs to produce and to read | 2 d — **done 2026-08-25** |
| [S1d.3.3](s1d.3.3_the_verdict.md) | What the verdict says, and whether this ships | 2 d |

**S1d.3.1 is done**, and it is the measurement that decides the rest:
[`utils/model_set_census.py`](../../../utils/model_set_census.py) and
[`model_set_census.md`](model_set_census.md). It settled the factorisation on
13 entries rather than 1, reported what the coupling is *made of* — which is
what tells S1d.3.2 whether any representation can exploit anything — and
**took** the number [P1d.2 handed
forward](../p1d.2_obligations/hypotheses_from_obligations.md). P1d.2 declined
the per-state leftover-open probe because a blind pass over the live node
writes `(not h)` and would move the model dedup; running it on a **discarded
fork** is the difference, and with the lever on and off every field of every
summary outside the new `leftover` block is identical on all 121 entries that
reach a fixpoint.

**S1d.3.2 is done**, and what it hands S1d.3.3 is a recommendation rather than
a list: **(b), the determining key, if anything ships at all.** It is 2 506
bytes on the phase's own case — 15 % smaller than (a) and the only form that
fits 72 columns — it is **exact and verified** (all 32 key rows reconstruct
their model to the fact, 30 of them without entering a single commitment), and
it is the only form that answers *what else would determine the Zebra*. (a) is
the one that loses: it cannot say how many models there are and the arithmetic
it invites over-states by 3.11 × 10¹². (c) is priced out by `k = 32` rather
than by the coupling — a reduced MDD is bounded in [24, 737] nodes under any
order — and (d) is deferred with a three-clause trip-wire no entry trips.

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
