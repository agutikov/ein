# Solvers: CSP / SAT / SMT / CP / LP / Logic Programming

Declarative search engines: describe the constraints, let the machine
find satisfying or optimal assignments. This is the broadest tools-side
cluster in scope.

---

## 1. Problem classes

### Constraint Satisfaction Problem (CSP)
Variables with domains plus constraints; find a satisfying assignment.
Sudoku, scheduling, type inference, FPGA placement & routing,
configuration, dependency resolution all fall here.
- Wikipedia: <https://en.wikipedia.org/wiki/Constraint_satisfaction_problem>

### Boolean SAT
"Does an assignment of true/false to variables satisfy this propositional
formula?" Surprisingly fundamental — EDA, verification, cryptanalysis,
theorem proving, planning, synthesis all rely on SAT solvers.
- Wikipedia: <https://en.wikipedia.org/wiki/Boolean_satisfiability_problem>

### SMT — Satisfiability Modulo Theories
SAT extended with first-order theories: integers, arrays, bitvectors,
floats, algebra, strings. Closer to "mathematical thinking".
- Wikipedia: <https://en.wikipedia.org/wiki/Satisfiability_modulo_theories>
- SMT-LIB: <https://smt-lib.org/>

### Constraint Programming (CP)
Declarative approach: model variables / constraints / objective; solver
propagates, prunes, and backtracks. Hybrid `CP-SAT` combines CP with
SAT and integer optimisation.
- Wikipedia: <https://en.wikipedia.org/wiki/Constraint_programming>

### Linear Programming (LP)
`max c^T x s.t. Ax ≤ b`. Foundational continuous optimisation.
- Wikipedia: <https://en.wikipedia.org/wiki/Linear_programming>

### Mixed-Integer Linear Programming (MILP / MIP)
LP with integer variables. Huge industrial usage.
- Wikipedia: <https://en.wikipedia.org/wiki/Integer_programming>

### Answer Set Programming (ASP)
Declarative "search over possible worlds"; particularly natural for
combinatorial puzzles and rich rule sets.
- Wikipedia: <https://en.wikipedia.org/wiki/Answer_set_programming>

### Logic programming
Program = set of logical assertions; computation = theorem proving via
unification + resolution + backtracking on Horn clauses.
- Wikipedia: <https://en.wikipedia.org/wiki/Logic_programming>

---

## 2. SAT solvers

### MiniSAT
Reference small/clean CDCL SAT solver — many subsequent solvers
descend from it.
- <http://minisat.se/>

### Glucose
CDCL solver with clause-quality (LBD) heuristics.
- <https://www.labri.fr/perso/lsimon/research/glucose/>

### CaDiCaL
Modern, fast, readable CDCL solver by Armin Biere.
- <https://github.com/arminbiere/cadical>

### Kissat
Even faster successor to CaDiCaL; current competition-grade workhorse.
- <https://github.com/arminbiere/kissat>

### CDCL — Conflict-Driven Clause Learning
Algorithmic core of modern SAT solvers: assume → propagate (BCP) →
conflict → analyse implication graph → learn clause → backjump.
- Wikipedia: <https://en.wikipedia.org/wiki/Conflict-driven_clause_learning>

### DPLL
The pre-CDCL backtracking algorithm; conceptual ancestor.
- Wikipedia: <https://en.wikipedia.org/wiki/DPLL_algorithm>

### Implication graph
Per-conflict causal DAG that CDCL analyses to learn a new clause —
literally a graph data structure inside the solver.

### Clause–variable bipartite graph
Standard structural view of a SAT formula; variables and clauses are
nodes, edges encode occurrence.

---

## 3. SMT solvers

### Z3
Microsoft Research SMT solver; S-expression input via SMT-LIB.
- <https://github.com/Z3Prover/z3>
- Z3 Guide: <https://microsoft.github.io/z3guide/>

### CVC5
Open-source SMT solver, successor to CVC4.
- <https://cvc5.github.io/>

### Yices
SMT solver from SRI.
- <https://yices.csl.sri.com/>

### SMT-LIB
S-expression standard for SMT input. Concrete example shape:
```lisp
(declare-const x Int)
(assert (> x 10))
(check-sat)
(get-model)
```
Often cited as the model of "Lisp-like surface syntax + strict formal
semantics" — a recurring recommendation for LLM-emitted IRs.
- <https://smt-lib.org/>

### Congruence closure
Equality-reasoning data structure used inside SMT solvers; close cousin
of e-graphs.
- Wikipedia: <https://en.wikipedia.org/wiki/Congruence_closure>

---

## 4. CP / MIP / OR

### MiniZinc
High-level modelling language for combinatorial problems; compiles to
many backends. Often called "SQL for combinatorial optimisation".
- <https://www.minizinc.org/>

### Essence / Conjure
Academic modelling DSL for combinatorial problems.
- Conjure: <https://conjure.readthedocs.io/>

### OR-Tools
Google's optimisation toolkit; includes the strong CP-SAT solver.
- <https://developers.google.com/optimization>

### Gurobi
Commercial LP/MILP/QP solver, state of the art on many benchmarks.
- <https://www.gurobi.com/>

### CPLEX
IBM commercial LP/MILP solver.
- <https://www.ibm.com/products/ilog-cplex-optimization-studio>

### SCIP
Open-source MILP and constraint-integer programming framework.
- <https://www.scipopt.org/>

### CP-SAT
OR-Tools' hybrid that mixes CP-style modelling with SAT + integer search.
A pragmatic top pick for puzzle-style problems (incl. Zebra).
- <https://developers.google.com/optimization/cp/cp_solver>

---

## 5. ASP systems

### Clingo
Reference ASP grounder + solver; from Potassco.
- <https://potassco.org/clingo/>

### DLV
Older ASP solver from Calabria.
- <https://www.dlvsystem.com/>

---

## 6. Logic-programming systems

### SWI-Prolog
The most widely used open Prolog implementation.
- <https://www.swi-prolog.org/>

### Datalog
Decidable Horn-clause subset of Prolog; backbone of modern graph queries,
static analysis frameworks, knowledge-graph engines.
- Wikipedia: <https://en.wikipedia.org/wiki/Datalog>

### Mercury
Strongly typed, mode-checked logic/functional language.
- <https://mercurylang.org/>

### miniKanren
Tiny embeddable relational-programming DSL (typically in Scheme/Racket);
clean substrate for goals, unification, and constraint solving.
- <http://minikanren.org/>

### CLP(FD) — Constraint Logic Programming over Finite Domains
Prolog extension with built-in propagation over finite-domain
constraints — the classical "Prolog way" to solve Zebra-style puzzles.
- SWI docs: <https://www.swi-prolog.org/man/clpfd.html>

---

## 7. Constraint propagation & consistency

### All-different / global constraints
Constraints that span many variables; specialised propagators are far
stronger than encoding them as a quadratic blow-up of pairwise
inequalities.
- Wikipedia: <https://en.wikipedia.org/wiki/Global_constraint>

### Arc consistency
For every value of a variable there must be a compatible value of each
neighbour; achieved by classical propagation algorithms (AC-3, AC-4…).
- Wikipedia: <https://en.wikipedia.org/wiki/Local_consistency#Arc_consistency>

### Path consistency
Strengthens arc consistency by considering paths of three variables.
Conversation-4's "square inference" rule is essentially this.
- Wikipedia: <https://en.wikipedia.org/wiki/Local_consistency#Path_consistency>

### Backtracking with branching / hypothesis testing
Classic depth-first search through partial assignments, with revert on
conflict. Generalised in CDCL with non-chronological backjumping and
clause learning.
- Wikipedia: <https://en.wikipedia.org/wiki/Backtracking>

### Allen interval algebra
Calculus of 13 base relations between time intervals — close relative of
spatial / ordering constraints in puzzles.
- Wikipedia: <https://en.wikipedia.org/wiki/Allen%27s_interval_algebra>

### RCC — Region Connection Calculus
Qualitative spatial logic; analogous to Allen's algebra but for regions.
- Wikipedia: <https://en.wikipedia.org/wiki/Region_connection_calculus>

---

## 8. Analysis on top of a model

### Model enumeration / backbone analysis
"Are there multiple models? Which variable values are forced?"
- Re-solve adding `solution ≠ S_prev` to detect ambiguity.
- Variables with the same value in every model form the *backbone*.

### Unsat core / minimal unsat subset (MUS)
When a model is infeasible, return a minimal contradicting subset of
constraints — basis for human-readable "why is this unsatisfiable?"
explanations. Pairs naturally with constraint *provenance* (each
constraint annotated with its source sentence/rule).
- Wikipedia: <https://en.wikipedia.org/wiki/Unsatisfiable_core>

### Constraint hypergraph
Variables = nodes, constraints = hyperedges spanning all their
variables. Natural IR for "Zebra-puzzle"-shaped problems, since
`allDifferent`, `next-to`, etc. are not binary.
- See [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md).

---

## 9. Conceptual structure

```
Logic
 → Constraints
 → Search
 → Proof
 → Optimization
 → Program synthesis
 → Verification
```
All of these end up sharing internal machinery: implication graphs,
congruence closure, propagation queues, DAG-shared subterms. Picking a
"solver category" is mostly about picking which slice of that machinery
is exposed at the user API.

## Cross-references

- SMT internals (congruence closure) → e-graphs:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- Theorem proving / proof assistants:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- Static analysis using constraint solving:
  [07-static-analysis-compilers.md](07-static-analysis-compilers.md)
- NLP → constraint-IR → solver pipeline:
  [10-nlp-semantic-parsing.md](10-nlp-semantic-parsing.md)
- MCTS / search strategies that wrap solvers:
  [11-search-optimization-algorithms.md](11-search-optimization-algorithms.md)
