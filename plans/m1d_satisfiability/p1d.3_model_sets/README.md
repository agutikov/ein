# P1d.3 — Model sets without enumeration

**Milestone:** [M1d — From saturation to satisfiability](../README.md)
**Estimate:** 1.5 weeks (7 days of stages)
**Depends on:** [P1d.2](../p1d.2_obligations/README.md) — the question only
becomes tractable once a state can say what it still owes.

**Depth: phase README only**, and of the three phases this is the one most
likely to change shape before it starts. See
[§ How deep this plan is](../README.md#how-deep-this-plan-is).

## Goal

**Decide whether 32 models should be printed or described.** The note's second
conclusion, after the one about obligations:

> all solutions compactly = saturation over symbolic constraints

— and its motivating complaint, that with several solutions there is no way to
find them without enumerating every hypothesis. A partial state with three
independent open choices *is* eight models; writing them out is a presentation
decision, and an expensive one.

## Why it might already be free, and why it might not

If [P1d.2](../p1d.2_obligations/README.md) lands, a saturated state carries
its open obligations and their candidate sets. When those choices are
**independent**, that state is exactly the compact answer: the model count is
the product of the candidate-set sizes, and no search is needed to report it.

When they are **not** independent — when choosing a witness for one obligation
narrows another — the product overcounts, and something has to represent the
dependency: a decision graph with shared subtrees, a disjunctive constraint
store, a BDD/ZDD, or projected model counting. Which of those, and whether any
is worth it, is what this phase decides. **The honest possible outcome is
"enumerate, and say so"**: 32 models is 32 lines, and a compact form that
nobody can read is worse than a list.

## Stages

| stage | title | est. |
|---|---|---|
| S1d.3.1 | What the 32 models actually differ in | 3 d |
| S1d.3.2 | Candidate representations, and what each costs to produce and to read | 2 d |
| S1d.3.3 | What the verdict says, and whether this ships | 2 d |

**S1d.3.1** is the measurement that decides the rest, and it is cheap: take
the 32 models of `zebra2-minus-15` (established independently in
[P1c.2](../../m1c_external_validation/p1c.2_external_benchmarks/README.md)),
and ask whether they factor. Independent choices mean the compact form is a
by-product of P1d.2; coupled ones mean this phase has real work.

## Acceptance for the phase

- **A written answer to "print or describe"**, with the factorisation of a
  real model set behind it rather than an intuition.
- If something ships, it is **additional output, not a replacement**: the
  models remain enumerable, because every consumer — the trace, the GUI,
  `:expect`, the benchmark adapters — reads models.
- Whatever is reported carries **the same guarantee vocabulary** the rest of
  the milestone settles: a compact description of a model set claims
  completeness only when the search proved it.

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
