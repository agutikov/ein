# Zebra-puzzle graph reasoner (5-year-old Ein design)

The canonical record is the repository [README](../../README.md) and
the `.dot` files in `files/`. This file restates the user's own
pre-existing design in their own terms — the prior art that everything
else here builds on.

## The architecture, in one paragraph

Model a Zebra-style puzzle as a typed graph (`House`, `Color`,
`Nationality`, `Pet`, `Cigarette`, `Drink`) with relations from the
puzzle text. Apply structural constraints (each object has at most
one link per type). Apply inference rules (triangle = transitive
composition, square = spatial propagation). When propagation runs
out, branch on hypotheses, test them by propagating until a
constraint fails. Multilevel hypothesis branching handles ambiguity;
exhaustion of branches signals genuine multiple solutions.

## The three implicit layers

These are worth naming explicitly because they're useful primitives
going forward:

| layer | content |
|---|---|
| Ontology graph | types, instances, value domains, a-priori inter-type relations |
| Fact graph | known relations from the puzzle text |
| Reasoning graph | inferred relations, rejected hypotheses, hypothesis branches |

Today these are tangled together in the README's single graph. A
clean rewrite would separate them.

## Constraints (as the README puts it)

- **Single-type constraint**: each object node has at most one link
  to a type node.
- **Number-of-attributes constraint**: for any object instance of
  `Type_1`, for any `Type ≠ Type_1`, there is at most one path
  from the object to a node of `Type`.

In Zebra these collapse to "every node has at most one link of every
possible type" — which is essentially a per-type `allDifferent`.

## Inference rules

Two were defined; their structure suggests they are instances of a
larger family (see [06-inference-rules-completeness.md](06-inference-rules-completeness.md)).

### Triangle rule
```
A → B
B → C
⇒
A → C
```
Transitive composition; categorical composition; the basic
substitution / `Eq + Eq` step. See
[07-categorical-formulation.md](07-categorical-formulation.md).

### Square rule
```
A → B
A next P
P → Q
B next Q  (so Q is the neighbour of B sharing the property)
```
Used for spatial relations (`next_to`, `right_of`) — Allen-style
interval reasoning on a 1-D house line.

## Hypothesis mechanism

A *hypothesis* is a copy of the graph with one added link.

### Generation (`hgen`)
1. Pick an object node of type `Type_1`.
2. Pick another type `Type_2`.
3. Generate the set of links from the object to every instance of
   `Type_2`.
4. Drop any candidate that immediately violates a constraint.

### Testing
1. Apply inference: derive everything derivable.
2. After each derived link, recheck constraints.
3. If any constraint fails, the hypothesis is false.

### Verification
At most one hypothesis from the generated set survives. If >1
survive → *ambiguity*.

### Multilevel hypothesis
On ambiguity, branch again from one of the surviving hypotheses.
- All next-level branches fail → original branch was false.
- Exactly one next-level branch survives → original branch was true.
- Multiple next-level branches survive → recurse.

Nodes that gained links during the previous level are good seeds for
the next level.

### Ambiguity
If no further branching reduces the survivors, the puzzle genuinely
has multiple solutions — and that is the correct answer.

## The algorithm, as printed in the README

```
S - relations state.
Constraint verification:    verify : S -> bool;
Inference:                  infer  : S -> S;
Hypothesis generation:      hgen   : S -> [S];

s = infer(s)
while not solved(s):
    h = hgen(s)
    while len(h) > 1:
        for s in h:
            if verify(s):
                s = infer(s)
            else:
                del s

        for s in h:
            # replace surviving hypothesis with next-level hypotheses
            h += hgen(s)
            del s
    s = h[0]   # only one hypothesis alive
```

## Known weaknesses (the user named these explicitly)

1. **Graphs are drawn by hand** — there is no automatic NL → graph
   converter; see [04-nlp-to-graph-to-solver-pipeline.md](04-nlp-to-graph-to-solver-pipeline.md).
2. **Are two inference rules enough?** — for Zebra, possibly. For
   other puzzles, almost certainly not. See
   [06-inference-rules-completeness.md](06-inference-rules-completeness.md).

## Open question recorded in the README

> *How to define constraint of existence of Ivory house to the left
> of Green one?*

i.e. the "neighbour with a direction" constraint isn't yet expressed
purely structurally in the graph — it currently sneaks in as a
specialised inference rule. A clean formalisation is missing.

## What today's index relates this to

- Truth-maintenance / ATMS — the hypothesis branching with rollback
  is structurally what ATMS does; CDCL is the same loop with clause
  learning bolted on. See
  [09-cognitive-architectures-neurosymbolic.md](../../docs/lib/09-cognitive-architectures-neurosymbolic.md).
- Graph rewriting (DPO/SPO) — the inference rules are graph rewrites
  in disguise. See
  [05-category-theory.md](../../docs/lib/05-category-theory.md).
- Constraint propagation (arc / path consistency,
  `allDifferent`) — the constraints map onto well-studied CSP
  notions. See
  [02-solvers-csp-sat-smt.md](../../docs/lib/02-solvers-csp-sat-smt.md).

## Connections to the other ideas

- [02-graph-as-formal-substrate.md](02-graph-as-formal-substrate.md) —
  this design is the most concrete realisation of the
  "compute-on-the-graph" intuition.
- [03-three-task-classes.md](03-three-task-classes.md) —
  *solve* is here today; *gaps* is implicit in the ambiguity
  handling; *contradictions* needs provenance edges to be useful.
- [08-human-style-deductive-trace.md](08-human-style-deductive-trace.md) —
  the inference + hypothesis trace is *almost* a human-readable proof
  already; what's missing is the rule-naming + elimination-of-cases
  presentation.
