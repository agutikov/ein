# Static Analysis & Compiler Technology

Analysing program behaviour without running on all inputs; compiler
internals that share machinery with solvers and proof systems.

---

## 1. Abstract interpretation

### Abstract interpretation
Framework for sound *approximation* of program semantics: replace
concrete values with elements of an abstract domain, run the program's
abstract semantics to a fixpoint, derive properties.
- Wikipedia: <https://en.wikipedia.org/wiki/Abstract_interpretation>
- Cousot & Cousot foundational paper (1977): "Abstract interpretation:
  a unified lattice model for static analysis of programs by
  construction or approximation of fixpoints".

### Cousot & Cousot
Patrick & Radhia Cousot — inventors of the framework.

### Lattice (in static analysis)
Abstract domains are usually complete (or join-semi)lattices; analysis
joins states, climbs towards a fixed point.
- Wikipedia: <https://en.wikipedia.org/wiki/Lattice_(order)>

### Galois connection
Formal relation between concrete and abstract semantics
(`α : C → A`, `γ : A → C` with `α ⊣ γ`); the mechanism that proves
abstraction *soundness*.
- Wikipedia: <https://en.wikipedia.org/wiki/Galois_connection>

### Fixed point
Stable state of the abstract analysis; iteratively computed.
- Wikipedia (Knaster–Tarski): <https://en.wikipedia.org/wiki/Knaster%E2%80%93Tarski_theorem>

### Widening / narrowing
Acceleration operators that force termination on infinite-height
domains (interval analysis without widening never stabilises on a
`while(x<N) x++` loop).
- Wikipedia: <https://en.wikipedia.org/wiki/Widening_(computer_science)>

---

## 2. Abstract domains

### Sign domain
`{neg, zero, pos, ⊤, ⊥}`. Simplest illustrative domain.

### Interval domain
Each variable abstracted by `[a, b]`. Very common in production
analysers.
- Wikipedia: <https://en.wikipedia.org/wiki/Interval_arithmetic>

### Octagon domain
Constraints of the form `±x ± y ≤ c`. Mid-power, sub-cubic.
- "The Octagon Abstract Domain", Miné:
  <https://www-apr.lip6.fr/~mine/publi/article-mine-HOSC06.pdf>

### Polyhedra domain
Arbitrary linear constraints `Ax ≤ b`. Most expressive of the classic
numerical domains; expensive.
- Wikipedia: <https://en.wikipedia.org/wiki/Convex_polytope>

---

## 3. Production analysers

### Astrée
Industrial abstract interpreter; famous for proving absence of
runtime errors in Airbus flight software.
- <https://www.absint.com/astree/>

### Infer
Meta/Facebook static analyser based on separation logic.
- <https://fbinfer.com/>

### Frama-C
Modular framework for C analysis with many plugins (value analysis,
WP for deductive verification, etc.).
- <https://frama-c.com/>

### Airbus (consumer of Astrée)
Reference point for safety-critical avionics use of abstract
interpretation.

---

## 4. Symbolic execution & model checking

### Symbolic execution
Execute the program with symbolic inputs, branching on path conditions,
discharging satisfiability with SMT.
- Wikipedia: <https://en.wikipedia.org/wiki/Symbolic_execution>

### KLEE
LLVM-based symbolic-execution engine.
- <https://klee-se.org/>

### angr
Python-based binary symbolic-execution framework.
- <https://angr.io/>

### Symbolic execution graph
Per-path tree → merged DAG of symbolic program states + SSA.

### Model checking
Exhaustive verification over a state graph against temporal-logic
properties; see [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md).

### Taint analysis
Tracks flow of "tainted" (e.g. user-controlled) values through the
program; a particular abstract / dataflow analysis used for security.
- Wikipedia: <https://en.wikipedia.org/wiki/Taint_checking>

---

## 5. Compiler intermediate representations

### MLIR — Multi-Level Intermediate Representation
LLVM-project framework for layered IRs (dialects); active research host
for equality saturation on tensor IRs.
- <https://mlir.llvm.org/>

### LLVM IR
Reference low-level SSA-based compiler IR.
- <https://llvm.org/docs/LangRef.html>

### SSA — Static Single Assignment
Mid-level IR form; explicit data-dependence DAG.
- Wikipedia: <https://en.wikipedia.org/wiki/Static_single_assignment_form>

### Control / dataflow / program / data dependence graphs
See [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md).

---

## 6. Equality saturation in compilers

### Tensat
Tensor-graph superoptimisation with e-graphs; applied to ML
compilers.
- <https://github.com/uwplse/tensat>

### Diospyros
Vector-instruction synthesis via equality saturation.
- <https://github.com/cucapra/diospyros>

### Herbie
Equality-saturation-based floating-point accuracy improver.
- <https://herbie.uwplse.org/>

### SPORES
Equality saturation for sparse-tensor algebra.

### "MLIR + equality saturation"
Active research direction merging the two ecosystems.

---

## 7. Program synthesis & verified optimisation

### Program synthesis
Generating programs from constraints, examples, or equivalences;
overlaps with equality saturation, SMT-based synthesis, and
type-directed synthesis.
- Wikipedia: <https://en.wikipedia.org/wiki/Program_synthesis>

### Superoptimisation
Search for provably optimal code (under some cost) over an
instruction set.
- Wikipedia: <https://en.wikipedia.org/wiki/Superoptimization>

### Verified optimisers
Compiler passes that come with a proof of semantic preservation; the
extreme end is CompCert (a fully verified C compiler).
- CompCert: <https://compcert.org/>

---

## 8. Connection map

```
Abstract interpretation  — reasons about *possible executions*
E-graphs                 — reasons about *equivalent meanings*
SMT / SAT                — reasons about *satisfiability*
Symbolic execution       — reasons about *path conditions*
Model checking           — reasons about *temporal behaviour*
```

Conversation-4 frames these as "two fundamental axes of symbolic
computation": *over-approximation of states* (AI) and *equivalence of
representations* (e-graphs). Modern reasoning engines combine them
freely.

## Cross-references

- E-graphs themselves: [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- SMT solvers used by symbolic execution:
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Theorem proving / verified compilation:
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
- Type theory / Hindley–Milner (a kind of abstract interpretation):
  [03-theorem-proving-formal-methods.md](03-theorem-proving-formal-methods.md)
