# Category Theory & Compositional Formalisms

Mathematical-semantics layer; notation and software for category theory
plus the compositional rewriting calculi that connect it to other parts
of this index (e-graphs, proof systems, NLP→IR translation).

---

## 1. Core concepts

### Category, object, morphism
A category is a directed graph (objects + morphisms / arrows) with two
extra structures: associative composition of morphisms, and an identity
morphism on every object.
- Wikipedia: <https://en.wikipedia.org/wiki/Category_(mathematics)>
- nLab: <https://ncatlab.org/nlab/show/category>

### Functor
Structure-preserving map between categories.
- Wikipedia: <https://en.wikipedia.org/wiki/Functor>

### Natural transformation
Structure-preserving map between functors.
- Wikipedia: <https://en.wikipedia.org/wiki/Natural_transformation>

### Commutative diagram
Diagram in which every directed path between two objects gives the
same composite morphism. The fundamental notational device for
category theory.
- Wikipedia: <https://en.wikipedia.org/wiki/Commutative_diagram>

### Monoidal category
Category with a tensor product, identity, and coherence isomorphisms;
substrate for string diagrams and quantum / process theories.
- Wikipedia: <https://en.wikipedia.org/wiki/Monoidal_category>

### Higher categories (2-categories, n-categories)
Categories whose morphisms have their own morphisms (2-morphisms), and
so on; substrate for higher-dimensional rewriting.
- nLab: <https://ncatlab.org/nlab/show/n-category>

### Adhesive categories
Abstract setting in which double-pushout graph rewriting behaves well.
- nLab: <https://ncatlab.org/nlab/show/adhesive+category>

### Limits, pullbacks, monomorphisms (mentioned)
Standard categorical constructions cited as the right home for
`allDifferent`-style global constraints once one models a puzzle
categorically.

---

## 2. Notation / diagram languages

### TikZ-cd (LaTeX)
De facto standard for typesetting commutative diagrams in papers.
- CTAN: <https://ctan.org/pkg/tikz-cd>

### Quiver
Interactive web editor for category-theory diagrams; exports TikZ-cd,
SVG, LaTeX.
- <https://q.uiver.app/>

### xymatrix
Older LaTeX diagram package.
- <https://ctan.org/pkg/xypic>

### Mermaid
Generic, lightweight diagram DSL — usable but primitive for CT.
- <https://mermaid.js.org/>

### PlantUML
UML-oriented diagram DSL — not categorical, but a common reference
point for "what a textual diagram language looks like".
- <https://plantuml.com/>

### Graphviz / DOT
Structural graph layout language; can encode CT diagrams as plain
graphs but has no built-in commutativity / functor semantics.
- <https://graphviz.org/>

---

## 3. String diagrams / process theories

Wires-as-morphisms notation widely used in monoidal categories, quantum
computing, tensor networks, and process theories.

### DisCoPy
Python toolkit for compositional diagrammatic reasoning over monoidal
categories.
- <https://discopy.org/>

### Quantomatic
Older Edinburgh proof assistant for graphical calculi (notably the
ZX-calculus).
- <https://quantomatic.github.io/>

### Globular
Web-based proof assistant for higher categories — 2-categories,
n-categories, visual rewriting of pasting diagrams.
- <https://globular.science/>

### Homotopy.io
Successor to Globular for interactive higher-category diagrams.
- <https://homotopy.io/>

---

## 4. Categorical software & libraries

### Catlab.jl
Julia framework for applied category theory; categorical structures
programmatically.
- <https://algebraicjulia.github.io/Catlab.jl/dev/>

### AlgebraicJulia ecosystem
Constellation of Julia packages around Catlab.jl (decapodes, semagrams,
etc.).
- <https://www.algebraicjulia.org/>

### Haskell as a "category-theory language"
Functor / Monad / NaturalTransformation type classes line up with
categorical constructions.

### Lean / mathlib `category_theory`
Most thorough formalisation of category theory in a proof assistant.
- <https://leanprover-community.github.io/mathlib4_docs/Mathlib/CategoryTheory/Category/Basic.html>

---

## 5. Graph rewriting as categorical operation

### Double-pushout (DPO) rewriting
Categorical formalisation of graph rewriting; rule = span L ← K → R,
rewrite = two pushouts in an adhesive category.
- Wikipedia: <https://en.wikipedia.org/wiki/Graph_rewriting>

### Single-pushout (SPO) rewriting
Variant using a single pushout; tolerates dangling edges.

### Term rewriting / categorical perspective
Term rewriting viewed through equivalence classes and functorial
semantics; conceptual ancestor of e-graphs and equality saturation.

### Operadic DSLs
DSLs built on operads (multi-input categorical generalisation); cited
under "experimental categorical languages".

---

## 6. "Three meanings of *category theory language*"

The phrase is overloaded:

| meaning | examples |
|---|---|
| diagram notation | TikZ-cd, Quiver |
| formal proof language | Lean, Coq, Agda, Idris |
| computational categorical programming | Haskell, Scala, Catlab.jl |

These are different problems and pick different tools.

---

## 7. Why categorical structure appears in this index

Patterns from the Zebra puzzle / proof-graph discussion:

- *Triangle inference* in the toy reasoner = composition of morphisms.
- *Constraints as commuting diagrams* — two different inferred chains
  must agree.
- *Functors* as translations between IR layers: NL → constraint graph →
  SMT → proof.
- *Natural transformations* as alternative reasoning strategies between
  two such translations.
- Graph rewriting as categorical operation (DPO/SPO) links the
  Zebra-style reasoner to a well-studied formal theory.

CT is best treated as a *meta-language / semantics*, not as a runtime
solver — i.e. it explains *why* the pieces compose, not what executes
them.

## Cross-references

- Diagram libraries used in practice:
  [08-diagramming-visualization-libraries.md](08-diagramming-visualization-libraries.md)
- Graph rewriting machinery and e-graphs:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- Proof systems where these ideas land in code:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- Categorical considerations in compiler IRs (MLIR):
  [07-static-analysis-compilers.md](07-static-analysis-compilers.md)
- Programming-language hosts (Julia / Haskell / Lean):
  [04-programming-languages.md](04-programming-languages.md)
