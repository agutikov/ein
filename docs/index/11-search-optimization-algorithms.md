# Search & Optimization Algorithms

The algorithmic engines underneath solvers, theorem provers, LLM
agents, and optimisation systems. Cross-listed pieces appear in their
"home" files; this chapter collects them as algorithms.

Primary sources: scattered across all four raw conversations
(SAT/SMT internals, MCTS in Const-o-T, hypothesis branching in
Zebra, evolutionary search in MOSES, equality saturation).

---

## 1. Tree / state-space search

### Backtracking
Depth-first exploration of partial assignments; revert on failure.
Foundational for CSP, Prolog, classical SAT.
- Wikipedia: <https://en.wikipedia.org/wiki/Backtracking>

### AND / OR trees
Backtracking-tree shape arising in Prolog and logic-programming search.
- Wikipedia: <https://en.wikipedia.org/wiki/And%E2%80%93or_tree>

### Branch-and-bound
Backtracking augmented with a bound function to prune subtrees;
classical for MILP, combinatorial optimisation.
- Wikipedia: <https://en.wikipedia.org/wiki/Branch_and_bound>

### Iterative deepening
Bounded DFS with growing depth; combines BFS-optimality with DFS-space
behaviour.

### DPLL
The original backtracking SAT algorithm.
- Wikipedia: <https://en.wikipedia.org/wiki/DPLL_algorithm>

### CDCL — Conflict-Driven Clause Learning
Modern SAT-solver loop: assume → propagate (BCP) → conflict → analyse
implication graph → learn clause → backjump non-chronologically.
- Wikipedia: <https://en.wikipedia.org/wiki/Conflict-driven_clause_learning>

### Hypothesis branching with rollback
The "human style" version used in the `Ein` README — pick a
candidate fact, propagate, look for contradiction, retract on failure.
Structurally identical to CDCL + ATMS.

---

## 2. Monte-Carlo / heuristic-guided search

### Monte Carlo Tree Search (MCTS)
Iteratively grow a search tree via *selection (UCB) → expansion →
rollout → back-propagation*. Used in Const-o-T as the
reasoning-step generator over `(intent, constraint)` pairs.
- Wikipedia: <https://en.wikipedia.org/wiki/Monte_Carlo_tree_search>

### UCB1 / UCT
Multi-armed bandit selection rule used inside MCTS.
- Wikipedia: <https://en.wikipedia.org/wiki/Multi-armed_bandit#Upper_Confidence_Bound>

### AlphaZero-style guided proof search
Neural network proposes "moves" (lemmas / hypotheses / tactics);
MCTS or another search engine explores. Conversation-4 explicitly
positions this as where neuro-symbolic proof search is heading
(LLM as policy / value over a proof-state graph).
- AlphaZero paper:
  <https://www.deepmind.com/publications/mastering-the-game-of-go-without-human-knowledge>

---

## 3. Constraint-propagation algorithms

### AC-3 / AC-4 (arc consistency)
Worklist-based propagation algorithms ensuring arc-consistency over
CSP variables.
- Wikipedia: <https://en.wikipedia.org/wiki/AC-3_algorithm>

### Path consistency
Propagation over triples of variables.

### Unit propagation / BCP (Boolean Constraint Propagation)
Inside SAT, deduce implied literals whenever a clause becomes unit.
The fastest, most-frequently-run inner loop of every SAT solver.
- Wikipedia: <https://en.wikipedia.org/wiki/Unit_propagation>

### Backbone / model enumeration
Identify forced-value variables across all models (Section 8 of
[02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)).

### Unsat core / Minimal Unsat Subset (MUS)
Find a minimal contradicting subset; basis for explainable
infeasibility reporting.
- Wikipedia: <https://en.wikipedia.org/wiki/Unsatisfiable_core>

---

## 4. Equality / rewrite-based search

### Equality saturation
Run all rewrite rules into an e-graph until fixed-point; extract the
best representative under a cost function. See
[06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md).

### Superoptimisation
Search for a provably optimal sequence of instructions implementing a
given function. See [07-static-analysis-compilers.md](07-static-analysis-compilers.md).

---

## 5. Numerical / continuous optimisation

### Linear programming (LP), MILP
See [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md).

### Convex optimisation (background)
Continuous-optimisation backbone.
- Boyd & Vandenberghe textbook:
  <https://web.stanford.edu/~boyd/cvxbook/>

---

## 6. Evolutionary / stochastic search

### Evolutionary algorithms / genetic programming
Population + mutation + selection; the substrate of MOSES in OpenCog.
- Wikipedia (GP):
  <https://en.wikipedia.org/wiki/Genetic_programming>

### MOSES (OpenCog)
Probabilistic + evolutionary program-learning engine.
See [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md).

### Probabilistic inference (background)
Belief propagation on factor graphs, variational inference, sampling
methods (MCMC) — adjacent but tangential to the source conversations.

---

## 7. Program-synthesis-shaped search

### Component-based synthesis
Compose programs from a library; search over typed compositions.

### Sketch-based synthesis
Fill holes in a partial program by solving a synthesis condition with
SAT / SMT.

### Refinement-type / type-directed synthesis
Drive synthesis by enriched type constraints.

(All tied back to [07-static-analysis-compilers.md](07-static-analysis-compilers.md).)

---

## 8. Cross-cutting themes

Three recurring themes:

1. **Constrained search > free generation.** Pure next-token sampling
   is too unstructured for long-horizon reasoning; wrap it with a
   verifier / propagator / search algorithm.
2. **Backtracking + learning beats pure backtracking.** CDCL's
   *learn-from-failure* loop is structurally the same as ATMS;
   `Ein`'s "hypothesis → contradiction → retract" loop is
   surprisingly close to CDCL.
3. **Substructure matters.** E-graphs, congruence closure, union-find,
   implication graphs, proof DAGs — the same data-structural primitives
   recur in solver, prover, and analyser internals; pick / build them
   intentionally.

## Cross-references

- SAT / SMT solver internals (CDCL, BCP, congruence closure):
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Proof DAG / tableau / natural deduction:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- E-graphs and equality saturation as search:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- MCTS / constrained-reasoning frameworks:
  [01-llm-constrained-generation.md](01-llm-constrained-generation.md)
- TMS / ATMS as logical search:
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
