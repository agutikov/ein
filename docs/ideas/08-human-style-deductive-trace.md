# Solver as reproducer of human deductive trace

## The idea

The solver's job is *not* to print the answer to the puzzle. The
solver's job is to *reproduce a deductive trace of the kind a human
would write*, with named reasoning moves at every step.

This is the **acceptance criterion** that distinguishes the project
from a wrapper around Z3 or MiniZinc.

## User's own words

> Ideally I wanted to use the solver to reproduce a deductive
> solution, for example like this: …

Followed by a several-paragraph step-by-step worked solution to the
classical Zebra puzzle — *the* concrete target output, not a sketch.

## The target trace (paraphrased)

The example walkthrough the user provided uses these reasoning moves:

- **Direct fact assertion** — "By condition (10), the Norwegian
  lives in the first house."
- **Composition** — "From (10) and (15) it follows that the second
  house is blue."
- **Elimination by exclusion** — "What colour is the first house?
  Not green and not white (because they must be neighbours and the
  2nd is blue). Not red (because the Englishman lives there).
  Therefore the first house is yellow."
- **Forward chaining** — "It follows that Kools are smoked in the
  first house (8), and the horse is kept in the second house (12)."
- **Case analysis** — "Suppose Lucky Strike is smoked in the second
  house — then orange juice is drunk there (13). Who could live
  there? Not the Norwegian (10). Not the Englishman (2). … This
  situation is impossible, so Lucky Strike is not smoked in the
  second house."
- **Reductio ad absurdum** — "Suppose the fox is in the third
  house. Then … this situation is impossible. Therefore the fox is
  kept in the first house, not the third."
- **Symmetry / "doesn't matter which"** — "It doesn't matter which
  way the houses are numbered, only the order matters."

A passable solver, by this criterion, must:

1. Match every step to a *named rule firing* on the constraint graph.
2. Present the elimination cases the human enumerated, in the same
   order or a recognisably equivalent one.
3. Quote the originating numbered constraint after each step.
4. Recognise the "doesn't matter which" moves and either suppress
   them or label them as symmetry breaks.

## Why SMT alone fails this criterion

SMT solvers are aggressive CDCL machines with clause learning and
backjumping. Their proof output is *correct but not readable*. The
kind of trace the user showed is closer to:

- a **tableau prover**'s case-split tree,
- an **ATMS / TMS**'s assumption-with-justification log,
- a **forward-chaining production system**'s rule-firing log,
- a **proof assistant**'s tactic script.

The solver should expose that structure, not hide it under a SAT-style
trace.

## What the implementation needs (implied by the trace)

| trace element | implementation requirement |
|---|---|
| "By condition (n)" | per-edge provenance back to the source sentence |
| "It follows that X" | named rule firings with `(premises) ⊢_rule X` records |
| "Not X, not Y, … therefore Z" | explicit *elimination by exhaustion* rule, with all alternatives enumerated for the trace |
| "Suppose X. Then ⊥." | hypothesis branch + contradiction with the chain that led to ⊥ |
| "Doesn't matter which way numbered" | a symmetry-detection / symmetry-breaking layer |

The 5-year-old design has 1, 4 partially. It does not have explicit
elimination-by-exhaustion as a rule, the rule provenance, or symmetry
handling. See [06-inference-rules-completeness.md](06-inference-rules-completeness.md).

## Why this is a hard problem

Producing a *human-readable* trace requires more than logging:

- **Ordering** — the rule-firing order chosen by the engine may not
  match the order a human would present. The trace probably needs
  *reordering / clustering* to be readable.
- **Granularity** — many trivial constraint-propagation steps must
  be collapsed into one ("then by exclusion …") to match human pace.
- **Naming** — variables / entities need their puzzle-language names
  ("the Norwegian", not `pos_var_3`).
- **Discourse coherence** — pronouns, "therefore", "however",
  "suppose" sequencing.
- **Self-referential summary** — "we have now filled all but one;
  the zebra is kept by the Japanese."

Probably the right architecture is:

```
graph engine derives raw rule-firing DAG
      ↓
trace planner reorders / clusters / picks rules to show
      ↓
linguistic surface generator (small LLM or template engine)
```

The first stage is essentially what [05-zebra-puzzle-graph-reasoner.md](05-zebra-puzzle-graph-reasoner.md)
already does, modulo provenance. The other two stages are missing.

## Open questions

1. **What evaluation harness?** Compare generated traces against a
   small corpus of human walkthroughs — automated similarity, or
   only human review?
2. **What's the right rule vocabulary for the trace?** A small,
   stable, named set is essential for readable text.
3. **Should symmetry breaking be done before or after trace
   generation?** Before = simpler proofs; after = traces that match
   the human's own "doesn't matter which" remark.
4. **Is the surface generator allowed to be an LLM?** Or must it be
   a templated, deterministic renderer for verifiability?

## Connections (context, not answers)

- ATMS / TMS as the architectural sweet spot:
  [09-cognitive-architectures-neurosymbolic.md](../index/09-cognitive-architectures-neurosymbolic.md).
- Why SMT proof traces aren't useful as-is:
  [02-solvers-csp-sat-smt.md](../index/02-solvers-csp-sat-smt.md) §2 +
  [11-search-optimization-algorithms.md](../index/11-search-optimization-algorithms.md) §1.
- Tableau / natural deduction style of proofs:
  [03-theorem-proving-formal-methods.md](../index/03-theorem-proving-formal-methods.md) §4.
- The provenance graph required to make this possible:
  [10-nlp-semantic-parsing.md](../index/10-nlp-semantic-parsing.md) §5,
  [03-three-task-classes.md](03-three-task-classes.md).
