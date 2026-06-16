# Programming Languages

Languages in scope, organised by paradigm.

---

## 1. S-expression / homoiconic / Lisp family

Recommended as the surface syntax for LLM-emitted IRs, because the AST
≈ text, the grammar is tiny, and self-modification ("code as data") is
natural. Specifically argued: prefer this to Haskell-like, Prolog-like,
or C-style for recursive self-modifying constrained systems.

### Lisp / Scheme / Common Lisp
The original `(operator arg arg arg)` family. Common Lisp is the host
of ACL2.
- Wikipedia: <https://en.wikipedia.org/wiki/Lisp_(programming_language)>
- Common Lisp HyperSpec: <https://www.lispworks.com/documentation/HyperSpec/Front/>

### Racket
Scheme-derived "language-oriented" Lisp; host of PLT Redex.
- <https://racket-lang.org/>

### Clojure (not directly named but in the family)
Modern JVM Lisp — listed for completeness given the S-expression theme.
- <https://clojure.org/>

### SMT-LIB (as a Lisp-shaped DSL)
S-expression standard for SMT solvers — best concrete example of a
small, formally specified Lisp-like IR.
- <https://smt-lib.org/>

---

## 2. Functional / typed-functional

### Haskell
Pure lazy functional language; "implicitly encodes lots of category
theory" (functors, monads, natural transformations, arrows,
applicatives). GHC's STG machine + G-machine use graph reduction.
- <https://www.haskell.org/>
- GHC: <https://www.haskell.org/ghc/>

### Glasgow Haskell Compiler (GHC) — STG / G-machine
The reduction machines behind GHC; canonical example of graph reduction
as runtime semantics.
- Wikipedia (STG): <https://en.wikipedia.org/wiki/Spineless_tagless_G-machine>

### Scala
JVM functional/OO hybrid; another common host for "category-theory in a
programming language" (cats, scalaz).
- <https://www.scala-lang.org/>

### OCaml (referenced by adjacency)
ML-family functional language; the type-inference family lives here.

---

## 3. Dependently typed

(Cross-listed; see [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md).)

- **Lean** — modern; mathlib is huge.
- **Coq** — long-established.
- **Agda** — strongest dependent-types ergonomics among the typed
  functional ones.
- **Idris** — closest to "programming with proofs".

---

## 4. Logic / declarative

(Cross-listed; see [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md).)

- **Prolog / SWI-Prolog** — classical logic programming.
- **Datalog** — decidable subset; popular in static analysis, graph
  databases, compiler analysis.
- **Mercury** — typed, mode-checked logic/functional.
- **miniKanren** — embeddable relational mini-language.
- **MiniZinc** — modelling language for combinatorial problems.
- **Essence / Conjure** — academic combinatorial-modelling DSLs.
- **Clingo / ASP** — declarative search over possible worlds.

---

## 5. Numerical / scientific host languages

### Julia
General-purpose technical-computing language; host of Catlab.jl, which
makes it the most "category-theory native" mainstream language.
- <https://julialang.org/>

### Catlab.jl
Julia framework for applied category theory — categorical structures
expressed programmatically.
- <https://algebraicjulia.github.io/Catlab.jl/dev/>

### Python (with Pydantic etc.)
Lingua franca for ML; SGR schema work is typically in Pydantic. DisCoPy
is a Python library for compositional diagrammatic reasoning.
- <https://www.python.org/>
- Pydantic: <https://docs.pydantic.dev/>
- DisCoPy: <https://discopy.org/>

---

## 6. Systems / native

### C++
Host of llama.cpp, OR-Tools internals, ImGui, ImNodes, imgui-node-editor,
OGDF, mxGraph (historically), and most production solvers.
- <https://isocpp.org/>

### Rust
Host of `egg` (e-graph library), many modern compiler / solver
projects.
- <https://www.rust-lang.org/>

### C (background)
Underlying low-level layer for solvers, GHC's RTS, llama.cpp, etc.

---

## 7. Web / JS ecosystem

JS/TS show up almost exclusively as the host platform for diagramming
libraries (see [08-diagramming-visualization-libraries.md](08-diagramming-visualization-libraries.md)).

- **JavaScript / TypeScript** — host for Cytoscape.js, React Flow,
  Sigma.js, JointJS, AntV X6, Konva, Fabric.js, etc.
- **React** — UI substrate for React Flow.

---

## 8. Niche / "categorical" / experimental

- **Globular** — proof assistant for higher categories.
  - <https://globular.science/>
- **Homotopy.io** — interactive higher-category diagram editor.
  - <https://homotopy.io/>
- **Quantomatic** — graphical-calculi-for-quantum proof assistant.
  - <https://quantomatic.github.io/>
- **DisCoPy** — Python toolkit for monoidal-category string diagrams.
  - <https://discopy.org/>

These straddle the line between "language" and "interactive proof tool"
and are detailed under category theory.

---

## 9. Why the surface-syntax recommendation matters

Conversation-1 strongly recommends, for any *LLM-emitted IR*:

```
minimal homoiconic symbolic IR
not:
 - Haskell-like
 - Prolog-like (as surface syntax)
 - C-style
 - natural language
```

Reasons cited:
- syntax entropy kills recursive self-modifying systems;
- ambiguity (precedence, infix, implicitness) accumulates;
- parser complexity explodes;
- tokenizer alignment with grammar worsens;
- the simpler and more uniform the syntax, the more cognitive capacity
  remains for *semantics* and *search*.

SMT-LIB and miniKanren are cited as the cleanest existing references.

## Cross-references

- LLM-emitted IR design considerations:
  [01-llm-constrained-generation.md](01-llm-constrained-generation.md)
- Solver-side declarative languages:
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Proof-assistant languages:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- Category-theory-flavored programming:
  [05-category-theory.md](05-category-theory.md)
- Compiler intermediate representations:
  [07-static-analysis-compilers.md](07-static-analysis-compilers.md)
