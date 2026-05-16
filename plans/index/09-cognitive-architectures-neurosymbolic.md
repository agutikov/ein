# Cognitive Architectures & Neuro-symbolic AI

Systems that try to unify multiple reasoning modes — symbolic + neural,
logical + probabilistic, memory + execution — inside one substrate.
Most lines of the discussion converge here, since the `ein-bot` Zebra
solver itself is a *typed constraint graph + rule engine + backtracking
solver* with explicit hypothesis branching.

---

## 1. OpenCog stack

### OpenCog
Symbolic cognitive-architecture framework; combines knowledge graphs,
logic, probabilistic reasoning, pattern matching, program rewriting,
attention/economy systems, evolutionary search, and neural components.
A radical "unify symbolic cognition inside one graph substrate"
attempt.
- <https://opencog.org/>
- Wikipedia: <https://en.wikipedia.org/wiki/OpenCog>

### AtomSpace
OpenCog's typed hypergraph; simultaneously memory, executable graph,
and probabilistic graph. *Not* a generic graph DB — types and
executable semantics are first-class.
- <https://wiki.opencog.org/w/AtomSpace>

### PLN — Probabilistic Logic Networks
Probabilistic logic with `(strength, confidence)` truth values on
links; mixes theorem proving, Bayesian reasoning, fuzzy logic, and
graph inference.
- <https://wiki.opencog.org/w/Probabilistic_Logic_Networks>

### MOSES
Evolutionary program-learning component of OpenCog; generates
programs / expressions via evolutionary + probabilistic search.
- <https://wiki.opencog.org/w/Meta-Optimizing_Semantic_Evolutionary_Search>

### Conceptual blending
Analogy-engine / semantic-recombination component, drawn from
Fauconnier–Turner cognitive science.
- Wikipedia: <https://en.wikipedia.org/wiki/Conceptual_blending>

### Attention allocation (ECAN)
Atoms carry *importance / attention values*; modelled almost as a
miniature economic system inside the graph — a way to bound finite
cognitive resources.
- <https://wiki.opencog.org/w/ECAN>

### Pattern matcher
Subgraph pattern matching engine over AtomSpace — Prolog-like,
graph-rewrite-like, symbolic-execution-like all at once.

---

## 2. Truth maintenance systems (TMS / ATMS)

### Truth Maintenance System (TMS)
Old-AI mechanism for tracking beliefs, their dependencies, contradictions,
and justifications. Retracting an assumption automatically retracts its
consequences. Directly applicable to "Zebra-puzzle reasoning with
hypothesis branching".
- Wikipedia: <https://en.wikipedia.org/wiki/Reason_maintenance>

### Assumption-based TMS (ATMS) — de Kleer
Maintains multiple consistent sets of assumptions in parallel.
Particularly close to:
- *Hypothesis-then-check-then-backtrack* reasoning;
- CDCL's *assume / propagate / conflict / backjump / learn* loop.
- Original paper (J. de Kleer, 1986): "An Assumption-Based TMS".

---

## 3. Neuro-symbolic landscape

### Neuro-symbolic AI
Cross-listed from [01-llm-constrained-generation.md](01-llm-constrained-generation.md).
General pattern: neural generator + symbolic verifier / search.
- Wikipedia: <https://en.wikipedia.org/wiki/Neuro-symbolic_AI>

### LLM as policy over a proof / reasoning graph
LLM proposes the next hypothesis / rewrite / inference; an external
symbolic engine verifies and propagates. Structurally analogous to
AlphaZero (network proposes moves, search engine validates them).

### Graph-Constrained Reasoning (GCR)
Already detailed in
[01-llm-constrained-generation.md](01-llm-constrained-generation.md).
Same pattern, with the *graph* being the symbolic substrate.

### Knowledge-graph AI / Retrieval
Use of triple stores / property graphs as the long-term memory of an
LLM-based agent; foundational substrate for current "agentic" stacks.

### Differentiable reasoning
Soft / differentiable relaxations of logic (e.g. Logic Tensor Networks,
Neural Theorem Provers) that allow gradient-based learning of inference.

---

## 4. Adjacent "graph + cognition" systems

### LangGraph / agent-workflow graphs
Practical engineering analogue: model the *workflow* of an LLM-based
agent as a graph with explicit nodes & edges.
- <https://www.langchain.com/langgraph>

### LangChain / LlamaIndex (background context)
Common framework substrates for the LLM + tool-using-agent pattern.
- <https://www.langchain.com/>
- <https://www.llamaindex.ai/>

### Production / rule systems
Forward-chaining rule engines (Rete algorithm, CLIPS, Drools).
Foundational background for OpenCog's rule engines and for many
classical "expert systems".
- Wikipedia (Rete algorithm): <https://en.wikipedia.org/wiki/Rete_algorithm>
- CLIPS: <https://www.clipsrules.net/>

### ein-bot itself
The `ein-bot` repository in this working tree fits squarely in this
chapter: a typed relational hypergraph (Ontology + Fact + Reasoning
layers) + inference rules (triangle, square, exclusivity, hypothesis)
+ backtracking. README:
[../../README.md](../../README.md).

---

## 5. Cognitive-architecture buzzwords from the source

- **AGI architecture** (OpenCog framing).
- **Hypergraph-based cognitive operating system** (paraphrase).
- **Symbolic probabilistic graph-rewriting AGI architecture** (paraphrase).
- **Reflective interpreters / meta-circular evaluators** (from the
  self-modifying-language thread).
- **Compositional cognition** (categorical framing).
- **Active knowledge graph / inferential graph** (vs static KG).
- **Proof state graph** (vs proof tree / proof DAG).

---

## 6. Why these ideas keep coming back

Conversation-4's framing: pure next-token prediction looks insufficient
for proofs, planning, long-horizon consistency, scientific reasoning,
and reliable agents — so structured symbolic scaffolding is returning,
typically in graph form. OpenCog's bet (one substrate for knowledge,
memory, logic, attention, execution, learning) is being re-explored in
modern neuro-symbolic and agentic systems, often under different names.

## Cross-references

- LLM-side constrained reasoning frameworks:
  [01-llm-constrained-generation.md](01-llm-constrained-generation.md)
- Hypergraphs / AtomSpace structure:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- Categorical view of composable cognition:
  [05-category-theory.md](05-category-theory.md)
- Underlying logic / theorem-proving substrate:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- Search strategies (MCTS, evolutionary, etc.):
  [11-search-optimization-algorithms.md](11-search-optimization-algorithms.md)
