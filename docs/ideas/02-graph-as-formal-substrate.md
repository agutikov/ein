# Graph IR as formal computation substrate

## The idea

When translating from one representation to another (say, NLP AST →
SMT), the intuitive thing is to introduce an intermediate graph
representation — knowledge graph, categorical structure, etc. — and
then translate from the graph to SMT.

But: a graph is *already* a formal system. So why translate at all?
The computation can be performed *directly on the graph*.

## User's own words

> When translating from a syntax tree to another representation, the
> instinct is to build an intermediate graph-like representation —
> knowledge graph, categorical something or similar — and then
> translate from it to SMT. But why do this if the graph is already a
> formal system, and the computation can probably be done directly
> there?

## Why this matters

Most modern stacks treat the IR as a *waypoint* on the way to a
solver. This idea inverts the relationship: the graph is the
*primary* computational object; solvers are optional acceleration.

That gives different affordances:

- **Memory** — the graph *is* the working memory of the reasoner,
  not a throwaway intermediate.
- **Explanation** — derivation lives on the graph, so a proof trace
  is just a sub-graph traversal.
- **Multiple reasoning modes** — solve / find-gaps /
  find-contradictions (see [03-three-task-classes.md](03-three-task-classes.md))
  all read from the same substrate.
- **Mutability and learning** — adding facts or revising hypotheses
  is a local graph edit, not a re-compilation.

## What "compute directly on the graph" can mean

This is what the user proposes — not what the assistant catalogued.
Some plausible operationalisations:

- **Constraint propagation in place** on a typed hypergraph
  (≈ arc / path consistency).
- **Graph rewriting** as the inference step (DPO/SPO categorical
  formalisation in the limit).
- **Equality saturation** as the equational-reasoning step
  (an e-graph laid over the IR).
- **Pattern-match + rule-fire** as the AtomSpace / production-system
  flavour.

The 5-year-old `Ein` design already does this in a small way
(triangle + square inference rules over a typed graph) — see
[05-zebra-puzzle-graph-reasoner.md](05-zebra-puzzle-graph-reasoner.md).

## Where it lands compared to "graph = IR for solver"

| view | what is primary | role of solver | trace artefact |
|---|---|---|---|
| graph as IR (mainstream) | the solver | central | solver's internal log (often unreadable) |
| graph as substrate (user's idea) | the graph | optional accelerator for sub-problems | sub-graph of derived edges |

Both views can coexist: a graph-native engine can still hand a
fragment to Z3 / clingo / OR-Tools when that genuinely helps. The
philosophical difference is who *owns* the proof.

## Open questions

1. **When does the graph stop being enough?** Plausible threshold:
   tasks dominated by arithmetic constraints, large combinatorial
   search, or global cardinality where a real solver dominates.
2. **What is the right kind of graph?** Plain directed graph?
   Typed multigraph? Typed hypergraph? Adhesive category? e-graph
   on top?
3. **What primitives does the graph engine need?** Pattern matching,
   rewriting, propagation, hypothesis branching, transitive closure,
   equality, negation-as-failure, …
4. **How does this scale?** And — is the right scaling strategy
   *better local engines* or *hybrid handoff to solvers*?

## Connections (context, not answers)

- Graph / hypergraph / e-graph machinery:
  [06-graphs-rewrite-systems.md](../lib/06-graphs-rewrite-systems.md).
- Solver back-ends, if/when you do hand off:
  [02-solvers-csp-sat-smt.md](../lib/02-solvers-csp-sat-smt.md).
- Categorical formalisation of graph rewriting (DPO/SPO):
  [05-category-theory.md](../lib/05-category-theory.md).
- Existing graph-native system (closest large prior art):
  OpenCog / AtomSpace —
  [09-cognitive-architectures-neurosymbolic.md](../lib/09-cognitive-architectures-neurosymbolic.md).
