# Theorem Proving & Formal Methods

Systems where the central act is *constructing or checking a proof*, not
just finding a satisfying assignment.

---

## 1. Interactive theorem provers (ITP)

### Lean
Modern dependently typed prover and language; `mathlib` is the canonical
formalised mathematics library. Often singled out as the "closest thing
to a real executable language for category theory" today.
- <https://lean-lang.org/>
- Lean 4 docs: <https://leanprover.github.io/lean4/doc/>
- mathlib: <https://leanprover-community.github.io/mathlib4_docs/>

### Coq
Calculus-of-inductive-constructions ITP. Long-standing standard in
type theory and verified software.
- <https://coq.inria.fr/>

### Isabelle / Isabelle/HOL
Higher-order-logic proof assistant with Isar declarative proofs.
- <https://isabelle.in.tum.de/>

### Agda
Dependently typed functional language doubling as proof assistant.
- <https://wiki.portal.chalmers.se/agda/>

### Idris
Practical dependently typed language; emphasises programming with proofs.
- <https://www.idris-lang.org/>

### ACL2
Lisp-based first-order ITP, descendant of Boyer–Moore.
- <https://www.cs.utexas.edu/users/moore/acl2/>

---

## 2. Automated theorem provers (ATP)

### Vampire
First-order resolution / superposition ATP; consistently top of CASC.
- <https://vprover.github.io/>

### E
First-order equational ATP, also superposition-based.
- <https://wwwlehre.dhbw-stuttgart.de/~sschulz/E/E.html>

### Prover9
Older first-order ATP from McCune (with Mace4 for finite models).
- <https://www.cs.unm.edu/~mccune/prover9/>

---

## 3. Foundations

### Curry–Howard correspondence
*Proofs ↔ programs, propositions ↔ types.* Constructive proof of a
proposition = construction of a term of the corresponding type. Basis
for Coq/Agda/Idris/Lean.
- Wikipedia: <https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence>

### Dependent types
Types that depend on values (e.g. `Vec A n`). Power the strongest
proof-assistant type systems.
- Wikipedia: <https://en.wikipedia.org/wiki/Dependent_type>

### Type theory (Martin-Löf, CIC, …)
The mathematical foundations underlying ITPs.
- Wikipedia: <https://en.wikipedia.org/wiki/Type_theory>

### Homotopy type theory (HoTT)
Type theory with a homotopy-theoretic interpretation; univalence.
- <https://homotopytypetheory.org/>
- HoTT Book: <https://homotopytypetheory.org/book/>

### Hindley–Milner type inference
Polynomial-time inference for ML-style let-polymorphism — internally a
unification / constraint graph algorithm.
- Wikipedia: <https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system>

---

## 4. Proof structures

### Proof tree
Classical presentation of a derivation in a natural-deduction or
sequent-calculus system.

### Proof DAG
Modern provers aggressively share subproofs → proof object becomes a DAG
rather than a tree.

### Dependency graph (theorems → lemmas)
A library like mathlib is, structurally, a giant DAG of theorems
depending on lemmas — itself a graph artefact.

### Tableau method
Refutation-style proof by attempting to extend a model and detecting
contradiction; close relative of the "branch then propagate then look
for contradiction" pattern in puzzle-solving.
- Wikipedia: <https://en.wikipedia.org/wiki/Method_of_analytic_tableaux>

### Natural deduction
Proof system that matches ordinary "if / then / contradiction" reasoning;
the conceptual ancestor of the *human-style deductive trace* one wants
from a Zebra-puzzle explainer.
- Wikipedia: <https://en.wikipedia.org/wiki/Natural_deduction>

### Resolution
Refutation-complete inference rule for first-order logic; classical core
of Prolog and most ATPs.
- Wikipedia: <https://en.wikipedia.org/wiki/Resolution_(logic)>

### Reductio ad absurdum
Assume the negation, derive a contradiction, conclude the original. The
human reasoning move that backtracking solvers and CDCL automate.
- Wikipedia: <https://en.wikipedia.org/wiki/Reductio_ad_absurdum>

---

## 5. Verified software & formal methods

### K Framework
Executable semantics framework — describe a language, its states, and
its rewrite rules; then run, analyse, and prove properties.
- <https://kframework.org/>

### PLT Redex
DSL inside Racket for specifying grammars + operational semantics +
reduction rules; "describe a language and its meaning".
- <https://docs.racket-lang.org/redex/>

### Astrée
Industrial-grade abstract interpreter, famous for proving absence of
runtime errors in Airbus flight software. Cross-listed under static
analysis but conceptually a verification system.
- <https://www.absint.com/astree/>

### Infer
Facebook/Meta static analyser, separation-logic based.
- <https://fbinfer.com/>

### Frama-C
Modular static analysis platform for C, integrates many analyses.
- <https://frama-c.com/>

---

## 6. Project-specific observations

- The Zebra-puzzle "human deductive trace" goal sits closer to ITP /
  tableau / TMS than to SMT: SMT proves *satisfiability* quickly but
  its CDCL trace is not human-readable.
- Triangle inference rule = composition (categorical composition);
  square inference rule ≈ path consistency. Both are proof-shaped
  graph rewrites.
- Hypothesis branching with rollback on contradiction = assumption-based
  natural deduction, also = CDCL-style assumption/propagation/backjump.

## Cross-references

- Underlying solvers / SAT / SMT used as kernels:
  [02-solvers-csp-sat-smt.md](02-solvers-csp-sat-smt.md)
- Category-theoretic view of proofs:
  [05-category-theory.md](05-category-theory.md)
- Graph rewriting as proof step:
  [06-graphs-rewrite-systems.md](06-graphs-rewrite-systems.md)
- Truth-maintenance / proof-state systems:
  [09-cognitive-architectures-neurosymbolic.md](09-cognitive-architectures-neurosymbolic.md)
- LLM-guided proof search (AlphaZero-style):
  [11-search-optimization-algorithms.md](11-search-optimization-algorithms.md)
