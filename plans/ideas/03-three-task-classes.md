# Three task classes on a constraint graph

## The idea

Once you have a constraint graph for a problem, there are three
distinct things you can ask of it — and they should be first-class
operations on the same substrate:

1. **Compute a solution to the complete problem.**
2. **Find the gaps in an incomplete problem** (which questions are
   under-determined, where ambiguity lives).
3. **Find contradictions** (which subset of constraints is
   inconsistent, and where it came from).

## User's own words

> Here I see different classes of tasks — computing a solution to the
> complete problem, computing gaps in an incomplete one, and searching
> for contradictions.

## Why distinguishing these matters

It is tempting to treat solvers as monolithic "give me an answer"
oracles. But the three classes have different shapes:

| class | input | output | tooling kind |
|---|---|---|---|
| A. Solve | constraint set | one (or all) models | SAT/SMT/CP `(check-sat)` + model extraction |
| B. Gaps / ambiguity | constraint set | set of free variables / multiple-model variance | model enumeration, backbone analysis |
| C. Contradictions | constraint set | minimal infeasible subset + provenance | unsat core, MUS, dependency graph |

A correct architecture distinguishes them rather than collapsing them
into "ask the solver and see what it returns".

## What "gaps" specifically means here

For a Zebra-style puzzle, a gap is:
- a *question* the puzzle states that the current constraint set
  cannot uniquely answer;
- equivalently: at least two models agree on the given facts but
  disagree on the answer to that question;
- equivalently: not in the *backbone* of the model set.

This is a more useful notion than "is it SAT/UNSAT".

## What "contradictions" specifically means here

Not just "UNSAT" — *which constraints contradict, and why?*

- Per-constraint **provenance** back to the source (a sentence in
  the puzzle text, a derived fact, an assumed hypothesis) is what
  makes the answer human-readable.
- A minimal unsatisfiable subset is the right granularity, not the
  whole formula.

## The implicit fourth class

Falls out by composition: **explain a derived fact** = find the
sub-DAG of edges from the source facts through to a target fact.
The user did not enumerate it but the conversation around hypothesis
generation / testing in the `Ein` README implies it.

## Open questions

1. **Are these three the right cut?** Or are they really five (add
   explanation + decision-under-uncertainty)?
2. **On a graph-native substrate ([idea 02](02-graph-as-formal-substrate.md))
   what does each class look like?** Solve = saturation + selection;
   gaps = enumeration of models / branching depth; contradictions =
   detection of conflict + minimal-subset trace.
3. **How are they wired into the same API?** Single
   `query(graph, mode={solve|gaps|contradictions})` or three
   distinct operations?
4. **What does the LLM frontend look like?** The user wants a system
   that can answer "what's wrong with my puzzle?" as easily as "what's
   the answer?".

## Connections (context, not answers)

- Solver-level support for each class — `(check-sat)` / model
  enumeration / unsat-core / MUS:
  [02-solvers-csp-sat-smt.md](../../docs/lib/02-solvers-csp-sat-smt.md), §8.
- Provenance + explanation graph:
  [10-nlp-semantic-parsing.md](../../docs/lib/10-nlp-semantic-parsing.md), §5.
- Why the explanation graph belongs on the IR rather than inside the
  solver:
  [02-graph-as-formal-substrate.md](02-graph-as-formal-substrate.md).
