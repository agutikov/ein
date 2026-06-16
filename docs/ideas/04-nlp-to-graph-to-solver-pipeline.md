# NLP → graph IR → solver pipeline

## The idea

To automatically solve a puzzle stated in natural language (Zebra
being the canonical case), the pipeline should *not* go directly
from an NLP syntax tree to SMT. There should be an intermediate
graph-shaped semantic representation; the solver is the *third* stage,
not the second.

## User's own words

> Suppose I want to build a solver for puzzles like the Zebra Puzzle.
> Rewriting [the puzzle] in SMT or a similar language is one option,
> but how do you automate that? How do you get SMT code from an NLP
> syntax tree? On top of that — there can probably be more than one
> valid variant. When translating from a syntax tree to another
> representation, the instinct is to build an intermediate graph-like
> representation — knowledge graph, categorical something or similar —
> and then translate from it to SMT.

## Why direct AST → SMT is the wrong cut

A syntax tree shows *form*, not the *semantic roles* a constraint
solver needs:

```
"The Norwegian lives in the first house."
        ↓ AST shows ↓
subject  = "The Norwegian"
verb     = "lives in"
object   = "the first house"
        ↓ but the solver needs ↓
entity   = Norwegian
property = nationality
relation = lives-in
target   = House-1
constraint = equality (pos(Norwegian) = 1)
```

The AST → solver path either:
- bakes in language-specific rules per template (brittle), or
- requires the LLM to do the whole semantic step in one shot
  (high error rate, no validator).

A graph-shaped IR makes the *semantic-binding* step explicit and
checkable in isolation.

## What "graph IR" means here, concretely

Not RDF triples. Not a generic knowledge graph. Something narrower:

- **typed** — every entity carries a type from the puzzle's ontology
  (`Nationality`, `Color`, `House`, `Pet`, …);
- **relational hypergraph** — constraints span >2 variables
  (`allDifferent`, `next_to`, `abs(pos(a)−pos(b))=1`);
- **provenance-bearing** — every edge remembers which sentence /
  rule produced it (for the contradiction-and-explanation task class
  in [03-three-task-classes.md](03-three-task-classes.md));
- **mutable** — propagation, hypothesis branching, and rollback all
  happen by graph edit.

## Multiple-variant complication

The user explicitly raised: *"there can probably be more than one
valid variant"*. So the pipeline cannot assume a unique answer per
puzzle. This is the gap-finding / ambiguity-detection use-case from
[03-three-task-classes.md](03-three-task-classes.md), surfaced again
at the NLP layer:

- A sentence may map to multiple semantically distinct frames.
- A puzzle text may be under-determined.
- The pipeline must keep alternatives alive, not collapse early.

## Sketch of the pipeline

```
puzzle text
   │
   ▼
NLP front-end (LLM or symbolic parser)
   │
   │  per-sentence structured fact (JSON), e.g.
   │      { relation: same_position,
   │        left: {type: nationality, value: Brit},
   │        right: {type: color, value: red},
   │        source: "The Brit lives in the red house." }
   ▼
typed-ontology validator   ──→  reject / disambiguate
   │
   ▼
constraint hypergraph IR (with provenance edges)
   │
   ├──→ graph-native engine  ──→  solve / gaps / contradictions
   └──→ solver back-end (SMT / CP-SAT / clingo / Prolog) for the
        slices where it earns its keep
   ▼
explanation graph  ──→  human-readable trace
```

## Open questions

1. **Who does the NLP step?** A small bounded LLM with GBNF-forced
   JSON output, or a hand-written parser tuned to the puzzle genre,
   or both (LLM first-pass + parser validator)?
2. **How is ontology coverage handled?** Per-puzzle declared
   ontology? Inferred from the text?
3. **Where do ambiguous parses go?** Branched as hypotheses on the
   IR, exactly like the puzzle-level hypothesis mechanism — see
   [05-zebra-puzzle-graph-reasoner.md](05-zebra-puzzle-graph-reasoner.md).
4. **When (if ever) is direct LLM → constraint emission acceptable?**
   For toy problems only? Never?

## Ontology deduction by common sense

User direction (2026-05-19): when a puzzle is stated in NL, the
**ontology is implicit** — it has to be deduced "by common sense"
from the facts. The IR explicitly carries an ontology block, but the
NL frontend must build it from the puzzle text without an explicit
declaration.

> *"Norwegian as a noun could be a Human, a Nationality, a Language.
> 'Norwegian lives in yellow house' — languages and nationalities
> don't live in houses, but humans do. So Norwegian here is a
> Human."*

This is the **constraint-driven disambiguation** pattern: the
predicate (`lives-in`) has a type signature (`Human × House`); when
the subject is polysemous (`Norwegian = Human ∨ Nationality ∨
Language`), the predicate's signature selects which sense.

In ein-lang terms, this maps onto:

```lisp
(ontology
  (relation lives-in Human House)
  (instance Norwegian Human))     ; deduced — `Norwegian` constrained by lives-in
(facts
  (lives-in Norwegian YellowHouse :source "(N)"))
```

The deduction is *forward-chaining* over the predicate registry: each
fact's relation signature narrows the candidate types for its
arguments. If the narrowing collapses to a unique type — assign it;
if multiple types remain — branch (the ambiguity-detection
sub-machinery from idea 03 / idea 05).

**Implementation hint (M2 P2.4)**: the NL frontend emits *typed*
JSON per sentence with the type filled by the predicate signature:

```json
{ "relation": "lives-in",
  "left":  { "value": "Norwegian", "type": "Human"     },
  "right": { "value": "YellowHouse", "type": "House"   },
  "source": "(N)" }
```

…and the loader treats the `type` field as an `(instance Norwegian
Human)` ontology fact. This sidesteps a separate "ontology induction"
stage by piggy-backing on the relation signatures that already exist
in any well-formed predicate registry.

**Why this matters as a kernel observation**: the engine's *fact
layer* is what the puzzle text *says*; the *ontology layer* is what
the puzzle reader *brings*. The NL→IR pipeline must produce both —
the engine on its own can't reason without the ontology. Idea 09's
benchmarks all-but-confirm this: every reasoning benchmark that
involves NL puts ontology induction as the implicit first step.

Connection: [`docs/lib/10-nlp-semantic-parsing.md`](../lib/10-nlp-semantic-parsing.md)
for semantic-frame parsing (which produces the same type
disambiguation as a by-product).

TODO: ontology induction
Use rule that is inverse of inheritance - generalization or something.
Brief example: French drinks coffee, French is a human, coffee is a drink, so humans can drink drinks.
What's the difference between deduction and induction of the ontology?


## Connections (context, not answers)

- NLP layer recipes (semantic frames, AMR, NER, coreference):
  [10-nlp-semantic-parsing.md](../lib/10-nlp-semantic-parsing.md).
- Constraint hypergraph as IR substrate:
  [06-graphs-rewrite-systems.md](../lib/06-graphs-rewrite-systems.md).
- LLM constrained to emit valid JSON via GBNF:
  [01-llm-constrained-generation.md](../lib/01-llm-constrained-generation.md).
- The graph-native engine half:
  [02-graph-as-formal-substrate.md](02-graph-as-formal-substrate.md).
