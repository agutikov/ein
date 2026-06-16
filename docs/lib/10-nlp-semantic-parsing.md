# NLP & Semantic Representation

Translating natural language into a formal IR (constraint graph,
SMT/CP code, proof state). Bridge between human-written puzzles /
problem statements and the solver / proof chapters of this index.

---

## 1. Why "AST → SMT" alone is not enough

Syntactic parse trees show *form* ("The Norwegian lives in the first
house"), not the *semantic roles* a solver actually needs
(`entity: Norwegian`, `property: nationality`, `relation: lives-in`,
`target: house-1`, `constraint: equality`).

What is needed is a **semantic representation**, not just an AST.

---

## 2. Layered NLP → solver pipeline

The recommended architecture from the source:

```
Natural language
 → syntactic / semantic parse
 → entity extraction
 → typed ontology
 → constraint hypergraph IR  (with provenance)
 → normalisation + typechecking
 → backend compiler
        ↙        ↓         ↘
   MiniZinc     Z3         clingo (ASP)
        ↘        ↓         ↙
   models / unsat-core / ambiguity analysis
 → explanation graph
```

Key reframing: *the graph is not a competitor of the solver — it is
the central memory / IR structure from which different reasoning modes
(solving, gap-finding, contradiction-explaining) can be launched.*

---

## 3. Semantic-representation devices

### Semantic role labelling / semantic frames
Tag the *agent / patient / instrument / location / …* roles in a
sentence; useful as an intermediate before binding to domain ontology.
- Wikipedia (semantic role labelling):
  <https://en.wikipedia.org/wiki/Semantic_role_labeling>
- FrameNet: <https://framenet.icsi.berkeley.edu/>

### Abstract Meaning Representation (AMR)
Single rooted DAG capturing the meaning of a sentence; commonly cited
"semantic IR" in modern NLP.
- <https://amr.isi.edu/>

### Entity extraction / Named-entity recognition
Find domain entities in text.
- Wikipedia: <https://en.wikipedia.org/wiki/Named-entity_recognition>

### Coreference resolution
Identify mentions referring to the same entity ("the Norwegian", "he",
"his house"); essential for Zebra-shape texts.
- Wikipedia: <https://en.wikipedia.org/wiki/Coreference>

### Semantic parsing
Map sentences directly to executable logical forms (DSL / SQL /
constraint terms).
- Wikipedia: <https://en.wikipedia.org/wiki/Semantic_parsing>

### Typed ontology
Catalogue of domain types (`House`, `Color`, `Nationality`, `Pet`,
`Drink`, `Cigarette`) with their value sets and allowed relations.
- See [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
  for hypergraph-shaped representations.

### Constraint hypergraph IR
Variables = nodes; constraints = hyperedges spanning all involved
variables. The right shape for Zebra-style problems, because relations
like `allDifferent(...)` or `abs(pos(fox) − pos(Chesterfields)) = 1`
are not binary.

---

## 4. LLM-driven extraction

The pragmatic recommendation:

> Don't try "universal language understanding". For a bounded domain
> (Zebra-like) build a controlled pipeline:
> LLM extracts structured facts → validator normalises → typechecker
> checks against ontology → compiler emits constraints.

Example structured output from an LLM step:

```json
{
  "relation": "same_position",
  "left":  {"type": "nationality", "value": "Brit"},
  "right": {"type": "color",       "value": "red"},
  "source": "The Brit lives in the red house."
}
```

A small deterministic program then compiles this to the chosen
backend (Z3 / MiniZinc / clingo / Prolog CLP(FD)).

### Where GBNF fits
Force the LLM's extraction step to emit grammar-valid JSON via GBNF or
equivalent, so the deterministic compiler can rely on the shape. See
[01-llm-constrained-generation.md](01-llm-constrained-generation.md).

### Schema-guided extraction (Pydantic / SGR)
Same idea at the schema level: a typed schema both *forces structure
during decoding* and *gives the validator something to check against*.

---

## 5. Three classes of task on the IR

(Direct from the source.)

| class | what to compute | typical tooling |
|---|---|---|
| A. find a solution | satisfiability / model | SMT, CP-SAT, MiniZinc, ASP, Prolog CLP(FD) |
| B. find ambiguity / gaps | enumerate models / backbone | re-solve with `≠ S_prev`, backbone analysis |
| C. find contradictions | unsat core + provenance graph | SMT unsat-cores, MUS, dependency tracing |

For (C), per-constraint *provenance* (back to the originating sentence)
is what enables a human-readable "the contradiction is between
conditions 3, 7 and 12" report — and pairs naturally with an
explanation graph.

---

## 6. Where category theory enters NLP→IR

The pipeline `NL → IR → SMT` is naturally a functor between categories
of representations; alternative back-ends are alternative functors; an
*explanation* of why one back-end suffices for another is a natural
transformation between them. Conversation-4 explicitly proposes this
view, while cautioning that for *implementing* a Zebra solver, plain
"typed hypergraph + constraint compiler" is more practical.

## Cross-references

- LLM-side decoding constraints and structured output:
  [01-llm-constrained-generation.md](01-llm-constrained-generation.md)
- Solver back-ends:
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Explanation / proof-state graph:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md),
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
- Hypergraph IR substrate:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- Categorical view of translations:
  [05-category-theory.md](05-category-theory.md)
