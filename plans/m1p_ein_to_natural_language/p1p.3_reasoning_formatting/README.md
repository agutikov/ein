# P1p.3 — reasoning formatting

**Milestone:** [M1p](../README.md)
**Status:** created 2026-11-20 on the user's instruction; **not planned into stages, not started**.
**After:** [P1p.1](../p1p.1_custom_formatting/README.md) and [P1p.2](../p1p.2_facts_formatting/README.md).

## The instruction, its part

> (1) all ein facts, inference, conclusions translated to NL

## What it is

A step of reasoning is a rule that fired on some facts and concluded others. Today the trace quotes the rule's
`:why` filled with that firing's bindings and prints what it derived as written. This phase makes the whole line
language: what was known, what rule was used, what follows, and, at the end of a run, what the answer is and why
a commitment that was refuted is refuted.

## Open, for its planning

Nothing here is ruled; each is a question the phase's first stage puts, on real programs.

1. **What a step says**: its premises, its rule and its conclusions each in words, and how much of each; the
   guide's annotated walkthrough is the text to come near.
2. **A hypothesis and a commitment**: *suppose*, *then*, and the return from a refuted supposition.
3. **A refutation**: a dead state's core and its `refuted by`, as a sentence. A core is an explanation and may
   differ between two runs of one answer.
4. **An obligation**: what a state still owes, and what would pay it.
5. **The answer**: the region, its states, a query's classes, in words, beside the read-out, which stays what it is.
6. **A program nothing is claimed of**: what is said of the reasoning of a program the static check flags.
7. **How long**: a run of the zebra has hundreds of steps. What is told and what is folded.
