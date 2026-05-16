# Index of Topics

Covers GBNF/llama.cpp, category-theory diagram languages, interactive
diagramming libraries, and CSP / SMT / theorem proving / NLP→solver pipelines.

The goal is an "awesome-list"-style knowledge map: every technology, tool,
language, system, scientific area, and buzzword in scope, grouped, briefly
described, and linked.

Many items belong to several groups (e.g. e-graphs are graph theory,
rewrite-system, compiler tech, and SMT internals at once). Each item lives in
its most-natural primary file; cross-references point to the others.

## Files

1. [LLM & Constrained Generation](01-llm-constrained-generation.md) —
   GBNF, llama.cpp, tokenization, logits/softmax, grammar-constrained decoding,
   constrained reasoning frameworks (GCR, CRANE, Const-o-T, SGR).
2. [Solvers: CSP / SAT / SMT / CP / LP / Logic Programming](02-solvers-csp-sat-smt.md) —
   Z3, MiniZinc, Prolog, Datalog, Clingo/ASP, OR-Tools, Gurobi, CPLEX,
   constraint propagation, arc/path consistency, CDCL.
3. [Theorem Proving & Formal Methods](03-theorem-proving-formal-methods.md) —
   Lean, Coq, Isabelle, Agda, Idris, dependent types, Curry–Howard,
   automated provers, proof DAGs, tableau, natural deduction.
4. [Programming Languages](04-programming-languages.md) —
   languages mentioned, organized by paradigm (functional, dependently typed,
   logic, S-expression, scripting/host, low-level).
5. [Category Theory & Compositional Formalisms](05-category-theory.md) —
   CT, monoidal/string-diagram categories, higher categories, adhesive
   categories, double/single-pushout rewriting, diagram languages
   (TikZ-cd, Quiver, Catlab.jl, DisCoPy, Globular, Homotopy.io).
6. [Graphs & Rewrite Systems](06-graphs-rewrite-systems.md) —
   DAGs, hypergraphs, e-graphs, equality saturation, congruence closure,
   union-find, term/graph rewriting, automata theory (DFA/NFA/Büchi/Kripke),
   knowledge graphs.
7. [Static Analysis & Compiler Tech](07-static-analysis-compilers.md) —
   abstract interpretation, lattices, Galois connections, fixpoints,
   widening, abstract domains (intervals, octagons, polyhedra),
   symbolic execution, model checking, MLIR, SSA, verified optimizers.
8. [Diagramming & Visualization Libraries](08-diagramming-visualization-libraries.md) —
   Cytoscape.js, React Flow, JointJS, AntV X6, Sigma.js, Konva, Fabric.js,
   ImGui node editors, GoJS, mxGraph, Graphviz; plus workflow editors
   (n8n, Node-RED, LangFlow, draw.io).
9. [Cognitive Architectures & Neuro-symbolic AI](09-cognitive-architectures-neurosymbolic.md) —
   OpenCog, AtomSpace, PLN, MOSES, Truth Maintenance Systems (TMS/ATMS),
   conceptual blending, attention allocation, neuro-symbolic hybrids,
   AGI-flavored architectures.
10. [NLP & Semantic Representation](10-nlp-semantic-parsing.md) —
    syntactic vs semantic parsing, semantic frames, entity extraction,
    constraint-graph IR, LLM-driven NLP→IR pipelines.
11. [Search & Optimization Algorithms](11-search-optimization-algorithms.md) —
    MCTS, CDCL, DPLL, backtracking, AND/OR trees, branch-and-bound,
    evolutionary search, AlphaZero-style guided proof search,
    program synthesis, superoptimization.

## Conventions

- Each entry has a short description (1–3 lines).
- External links go to upstream project page, Wikipedia, official docs, or
  representative publication, in roughly that order of preference.
- Cross-references between files use relative links and the same anchor
  conventions GitHub-flavoured Markdown derives from headings.
- Where there is a recommendation, judgement, or important caveat
  about an item, it is preserved as a short note.

## Next step (per TODO)

The TODO calls for a knowledge graph: many of these items participate in
multiple groups simultaneously. After this index stabilises, the natural
next move is to attach explicit cross-links (or a dedicated edges file) so
the same items can be navigated along several axes — paradigm, structural
substrate (graph / lattice / category / term), application area (NLP,
verification, optimization), and tool category.

