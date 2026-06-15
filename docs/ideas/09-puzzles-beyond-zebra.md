# Puzzles beyond Zebra

Zebra is M1's acceptance gate, but it's only one *kind* of puzzle. The
engine's design choices (typed hypergraph, named-rule trace,
hypothesis-loop) make sense only when measured against a wider class
of human-style logic puzzles. This note catalogues the classes worth
keeping in view as Ein grows, and what each one stresses that
Zebra does not.

The companion file [`docs/index/12-llm-and-reasoning-benchmarks.md`](../index/12-llm-and-reasoning-benchmarks.md)
catalogues LLM/neuro-symbolic *benchmarks*. This file is about
*puzzles people enjoy solving* — ones with an insight, a recognisable
named-move repertoire, or a beautiful piece of reasoning. The
overlap with research benchmarks is partial; the orientation is
different.

## Classical deductive puzzles

### Einstein's riddle / Zebra puzzle
The canonical logic-grid puzzle. Many entities, gradual elimination,
tabular bookkeeping, almost no "insight" — just careful deduction.
**Stresses:** elimination + composition + global cardinality. The
Ein baseline.

### Knights and Knaves
Smullyan's island. Knights always tell the truth; knaves always lie.
"A says: we are both knaves." Compact, recursive, self-referential
sentences. **Stresses:** truth-functional reasoning over self-
reference; small state space but tricky semantics.

### Blue-eyed islanders
Common knowledge, recursive knowledge, public announcements. Famously
counter-intuitive. **Stresses:** epistemic logic — "knowing that the
others know".

### Monty Hall
Probability paradox. Most people guess wrong; the explanation seems
"impossible". **Stresses:** conditional probability; not deductive
reasoning at all. Boundary case for the engine's scope.

### Two-envelopes problem
Expectation paradox; conditional expectation done carefully. Same
scope-boundary concern as Monty Hall.

## Hat puzzles

A huge sub-family. People stand in a line / circle, see some hats but
not their own, must guess their colour. Strategy is fixed in advance.
Variants escalate fast:

- binary codes;
- parity arguments;
- information-theoretic bounds;
- common-knowledge analysis.

**Stresses:** strategy synthesis (not just inference) + epistemic
reasoning.

## Self-reference and paradoxes

### Unexpected hanging paradox
Paradox of prediction and knowledge.

### Liar paradox
"This statement is false." Substrate for truth-theory, Gödel, fixed
points.

### Barber paradox
The barber who shaves all those who do not shave themselves. Connected
to Russell's paradox in set theory.

**Stresses:** none of these is *solvable* in the Ein sense; they
sit at the boundary of what a reasoning engine should *recognise* as
self-reference / contradiction rather than try to mechanically prove.

## Knowledge-about-knowledge

### Muddy children
The classical epistemic-logic puzzle: who knows their forehead is
dirty after k announcements. **Stresses:** common knowledge,
synchronous public announcements.

### 100 prisoners problem
Looks impossible; has a beautiful solution via permutation cycles.
**Stresses:** strategy synthesis; combinatorial structure that is
*not* search-tree-shaped.

## Planning and state-transition

### Bridge and torch
Cross a bridge with a torch, time-bounded, n people with different
speeds. **Stresses:** optimal-strategy planning.

### Wolf, goat, cabbage
The ancient state-graph puzzle. **Stresses:** state-transition
planning over a discrete state graph with safety constraints.

### Tower of Hanoi
Recursion and induction. **Stresses:** a recursive structure that
*should* be expressible as a rule schema in a rule-based engine.

### Josephus problem
Cyclic elimination. **Stresses:** arithmetic / number-theoretic
structure on top of discrete elimination.

### Eight queens
Combinatorial search; backtracking. **Stresses:** the search side of
"reasoning" — far less deduction-rich than Zebra; closer to pure CSP.

## Spatial / visual

Tangram, Soma cube, Rubik's cube. **Stresses:** geometry / 3-D
spatial transformations. M1's declarative graph-only spatial
formulation (`right-of` / `next-to` + `square-fwd` / `square-bwd` /
`square-unique` rules) is 1-D only and adjacency-flavoured; it
handles none of these geometry-heavy puzzles. They're explicitly out
of scope for the current engine and return in
[followups F4 Q32 (2-D / N-D spatial)](../../plans/followups/f4_cross_cutting.md).

## Lateral thinking

Edward de Bono's situation puzzles: not formal deduction at all — the
solver asks yes/no questions to discover the hidden model. **Stresses:**
abductive reasoning over open-world models. Out of scope for Ein
unless / until an LLM front-end handles abduction.

## Books and cultural references

- *Gödel, Escher, Bach* (Hofstadter) — self-reference, formal systems,
  paradoxes, consciousness, recursion. Not a puzzle book but the
  cultural background that motivates this project.
- *What Is the Name of This Book?* (Smullyan) — classical logic
  puzzles, mostly Knights & Knaves.
- *The Lady or the Tiger?* (Smullyan) — another classical Smullyan
  collection.

---

## Classification by mode of thought

| Mode                          | Examples            |
|-------------------------------|---------------------|
| Tabular deduction             | Zebra               |
| Self-reference                | Liar paradox        |
| Knowledge about knowledge     | Muddy children      |
| Probabilistic counter-intuit. | Monty Hall          |
| Strategy synthesis            | 100 prisoners       |
| Optimisation / planning       | Bridge and torch    |
| State-transition              | Wolf-goat-cabbage   |
| Spatial / visual              | Tangram             |
| Lateral thinking              | Situation puzzles   |
| Recursive logic               | Knights and Knaves  |

The current engine targets row 1 ("tabular deduction") via M1, and
implicitly rows 6–7 ("strategy" / "state-transition") via the
hypothesis loop. Rows 2–3 ("self-reference" / "knowledge about
knowledge") would need the engine to *represent the engine itself*
as an entity — see [idea 01 self-modifying language](01-self-modifying-constraint-language.md).
Rows 4 (probability) and 9 (lateral) are out of scope.

---

## Open questions

1. Which of these classes is the next *deliberate* expansion target?
   - Knights & Knaves looks closest: same engine shape, different
     ontology (truth-values + meta-statements). A natural M1-finalisation
     test that does *not* require multi-dimensional spatial.
2. Should the engine grow an *epistemic layer* (muddy children, hat
   puzzles), or is that a separate sub-project?
3. Where does Tower of Hanoi belong — as a recursion test for the rule
   schema, or as a planning test for the hypothesis loop?
4. Is there value in the spatial / visual cluster *before* a 2-D
   position lattice exists? Probably not — defer with
   [F4 Q32](../../plans/followups/f4_cross_cutting.md).

## Connections

- [`docs/ideas/05-zebra-puzzle-graph-reasoner.md`](05-zebra-puzzle-graph-reasoner.md) —
  the current target class.
- [`docs/ideas/06-inference-rules-completeness.md`](06-inference-rules-completeness.md) —
  rule families that recur across these puzzles.
- [`docs/index/12-llm-and-reasoning-benchmarks.md`](../index/12-llm-and-reasoning-benchmarks.md) —
  the *machine-evaluation* counterpart to this human-facing catalogue.
- [`plans/followups/f4_cross_cutting.md`](../../plans/followups/f4_cross_cutting.md)
  Q32 — 2-D / N-D spatial puzzles parked there.
