# Graphs & Rewrite Systems

The structural substrate behind almost everything else in this index:
DAGs, hypergraphs, e-graphs, automata, knowledge graphs, term/graph
rewriting. The recurring observation: "computation, logic and proofs
naturally turn into graphs of dependencies, states and equivalences."

---

## 1. Basic graph kinds

### Directed graph
Nodes plus directed edges; underlying shape of virtually every IR
discussed elsewhere in this index.
- Wikipedia: <https://en.wikipedia.org/wiki/Directed_graph>

### DAG — Directed Acyclic Graph
Used as: AST after sharing, SSA program form, proof DAGs, e-graph
backbone, scheduling (PERT/CPM), tensor compute graphs.
- Wikipedia: <https://en.wikipedia.org/wiki/Directed_acyclic_graph>

### Hypergraph
Edge can connect any number of nodes; appropriate for *relations*
rather than binary edges. Natural fit for `allDifferent`, `next_to`,
`abs(pos(a) − pos(b)) = 1`, AtomSpace links.
- Wikipedia: <https://en.wikipedia.org/wiki/Hypergraph>

### Bipartite graph
Two disjoint node sets; e.g. clause–variable bipartite graphs in SAT,
factor graphs in probabilistic inference.
- Wikipedia: <https://en.wikipedia.org/wiki/Bipartite_graph>

### Term graph
Shared-subterm DAG representation of expressions; core of e-graphs
and graph reduction in functional language runtimes.
- Wikipedia: <https://en.wikipedia.org/wiki/Term_graph>

### Knowledge graph
Subject–predicate–object triples; substrate for RDF/OWL, semantic
web, GCR-style LLM grounding.
- Wikipedia: <https://en.wikipedia.org/wiki/Knowledge_graph>

### Factor graph
Bipartite graph for probabilistic inference connecting variables and
factors.
- Wikipedia: <https://en.wikipedia.org/wiki/Factor_graph>

### Bayesian network
Directed acyclic graphical model of conditional dependencies.
- Wikipedia: <https://en.wikipedia.org/wiki/Bayesian_network>

### Flow network
DAG with capacities; classical optimisation backbone.
- Wikipedia: <https://en.wikipedia.org/wiki/Flow_network>

---

## 2. Equivalence machinery

### Union-find / disjoint-set
Data structure for tracking equivalence classes. Inside e-graphs, SMT
congruence closure, Hindley–Milner unification.
- Wikipedia: <https://en.wikipedia.org/wiki/Disjoint-set_data_structure>

### Congruence closure
Smallest equivalence relation that is closed under "equal arguments →
equal applications" (`a = b → f(a) = f(b)`). Powers SMT equality
reasoning; close conceptual sibling of e-graphs.
- Wikipedia: <https://en.wikipedia.org/wiki/Congruence_closure>

### Equivalence relation / quotient structure
Categorical / set-theoretic backdrop for everything above.

---

## 3. E-graphs & equality saturation

### E-graph
Data structure that stores *many equivalent forms* of an expression
simultaneously. Backbone of equality saturation, modern compiler
rewrites, and a clean substrate for equational reasoning.
- Wikipedia: <https://en.wikipedia.org/wiki/E-graph>

### E-class
Set of e-nodes representing equivalent expressions.

### E-node
A function symbol applied to e-classes.

### Equality saturation
Apply *all* rewrite rules, saturate the e-graph with equivalences, then
extract the best expression under a chosen cost function (smallest,
fastest, vectorisable, …). Replaces greedy local rewriting with
global equivalence search.
- "egg: Fast and Extensible E-Graphs" paper:
  <https://dl.acm.org/doi/10.1145/3434304>

### egg
Rust library for e-graphs and equality saturation.
- <https://egraphs-good.github.io/egg/>

### Notable e-graph applications

- **Herbie** — improves floating-point accuracy.
  - <https://herbie.uwplse.org/>
- **Tensat** — tensor-graph superoptimisation for ML compilers.
  - <https://github.com/uwplse/tensat>
- **Diospyros** — vector-instruction synthesis via equality saturation.
  - <https://github.com/cucapra/diospyros>
- **SPORES** — equality-saturation for sparse tensor algebra.

### Conversation framing
> Many equivalent worlds coexist simultaneously inside one e-graph —
> a "quantum-like" representation, in contrast to abstract
> interpretation's "many concrete states → one abstract state".

---

## 4. Term & graph rewriting

### Term rewriting system
Rules `lhs → rhs` rewriting terms / ASTs; foundation of much of
symbolic computation, compiler optimisation, proof search.
- Wikipedia: <https://en.wikipedia.org/wiki/Rewriting>

### Graph rewriting
Generalisation to graphs; rule = subgraph pattern + replacement.
- Wikipedia: <https://en.wikipedia.org/wiki/Graph_rewriting>

### Double/Single-pushout rewriting
Categorical formalisations of graph rewriting; see
[05-category-theory.md](05-category-theory.md).

### Categorical rewriting
Rewriting modulo equivalence as a quotient operation in an appropriate
category.

---

## 5. Automata & state-transition graphs

### DFA / NFA
Deterministic / non-deterministic finite automata; states + transitions
on input symbols.
- Wikipedia (DFA): <https://en.wikipedia.org/wiki/Deterministic_finite_automaton>
- Wikipedia (NFA): <https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton>

### Pushdown automaton
DFA/NFA + stack — recognises context-free languages.
- Wikipedia: <https://en.wikipedia.org/wiki/Pushdown_automaton>

### Büchi automaton
ω-automaton over infinite words; standard for ω-regular model
checking.
- Wikipedia: <https://en.wikipedia.org/wiki/B%C3%BCchi_automaton>

### Kripke structure
Directed graph (S, R) used as the state model in model checking.
- Wikipedia: <https://en.wikipedia.org/wiki/Kripke_structure_(model_checking)>

### Model checking
Verification by exhaustive (often symbolic) exploration of a state
graph against a temporal-logic specification.
- Wikipedia: <https://en.wikipedia.org/wiki/Model_checking>

---

## 6. Compiler-side graphs

### Control flow graph (CFG)
Basic blocks + control-transfer edges; per-function structure built
by every compiler / static analyser.
- Wikipedia: <https://en.wikipedia.org/wiki/Control-flow_graph>

### Dataflow graph
Edges represent data dependencies, used for type / interval / taint
propagation.

### SSA — Static Single Assignment
Compiler IR where every variable is assigned exactly once; explicit
data-dependence graph.
- Wikipedia: <https://en.wikipedia.org/wiki/Static_single_assignment_form>

### Program / data dependence graphs (PDG / DDG)
Combined control + data dependencies; used in slicing, parallelisation,
optimisation.

### Tensor compute graph
Tensor ops + dependency edges; substrate of every ML compiler.
See [07-static-analysis-compilers.md](07-static-analysis-compilers.md).

### Graph reduction
Functional-language runtime model where evaluation rewrites a graph of
shared thunks; STG / G-machine in GHC.

---

## 7. Knowledge-graph & symbolic-AI structures

### RDF / OWL / semantic web
Standardised triple-store knowledge representation and ontology
languages.
- RDF: <https://www.w3.org/RDF/>
- OWL: <https://www.w3.org/OWL/>

### AtomSpace (OpenCog)
Typed hypergraph + symbolic memory + executable graph. See
[09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md).

### Graph neural networks
Neural architectures that operate on graphs; mainstream "neural" side
of neuro-symbolic graph reasoning.
- Wikipedia: <https://en.wikipedia.org/wiki/Graph_neural_network>

### Neo4j
Property-graph database; canonical reference for "knowledge graph +
reasoning engine".
- <https://neo4j.com/>

---

## 8. SAT/SMT internals as graphs

Cross-listed from solver chapter:
- *Implication graph* (CDCL conflict analysis).
- *Clause–variable bipartite graph*.
- *Congruence closure graph* (inside SMT).
- *Search tree / proof DAG* of resolution refutations.

---

## 9. "How graph-native is X?" summary

| area | how graph-native |
|---|---|
| e-graphs | extremely |
| category theory | extremely |
| Datalog | extremely |
| model checking | extremely |
| symbolic execution | very |
| SAT / CDCL | very |
| abstract interpretation | very |
| tensor compilers | very |
| type inference | medium-high |
| classical theorem proving | medium-high |
| LP / MILP | medium |

Conclusion: graph reasoning is plausibly *the most universal form of
symbolic computation*, because dependencies, causality, equivalence,
flow and composition all naturally land on edges.

---

## 10. Three representational tiers

```
Tree   — hierarchy (AST)
 → DAG  — sharing
  → E-graph — sharing + equivalence
   → Category — graph + composition laws
```

Many modern systems migrate up this ladder over their lifetime.

## Cross-references

- E-graph applications in compilers / superoptimisation:
  [07-static-analysis-compilers.md](07-static-analysis-compilers.md)
- Hypergraph-based AI (AtomSpace, TMS):
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
- Category-theoretic formalisations of graph rewriting:
  [05-category-theory.md](05-category-theory.md)
- Constraint hypergraph as NLP→IR target:
  [10-nlp-semantic-parsing.md](10-nlp-semantic-parsing.md)
- Visualising graphs interactively:
  [08-diagramming-visualization-libraries.md](08-diagramming-visualization-libraries.md)
