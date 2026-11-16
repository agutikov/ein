





Proof System, Theory, Models
Proof Theory
Models Theory

If Metatheory M in Proof System Q defines Theory T of Proof System P,
can Q === P ?


4 points of interest:
- Proof-Theoretic Semantics
- Structural Proof Theory
- Proof Search
- Cyclic Proofs



[Constraint Learning for Non-confluent Proof Search](https://arxiv.org/abs/2603.05258)


Ein is confluent by design, right? Or multi-model R is not? Or non-monotone program is not?


--------------------------------------------------------------------------------


# **Proof Theory Research Landscape for Ein**

The four areas form a substantial research program at the intersection of **mathematical logic, automated deduction, algebraic logic, term rewriting, formal verification, and neuro-symbolic reasoning**.

For Ein, I would organize them into four research tracks, each with its own foundational questions, technical subfields, and literature.

## **1. Proof-Theoretic Semantics (PTS)**

Meaning through inference rules rather than predefined interpretations

**Central question:** Can the meaning of a logical connective, relation, or operator be characterized by the rules governing its use?

### **Research topics and keywords**

| Subfield                | Concepts and keywords                                                  |
| ----------------------- | ---------------------------------------------------------------------- |
| Inferentialism          | Meaning-as-use, inferential roles, proof-theoretic validity            |
| Harmony of Rules        | Introduction–elimination harmony, local soundness, local completeness  |
| Inversion Principles    | Prawitz inversion, Gentzen inversion, admissibility                    |
| Bilateralism            | Assertion and denial, proof and refutation, bilateral calculi          |
| Substructural Logics    | Linear, relevant, affine, ordered and resource-sensitive logics        |
| Algebraic Proof Theory  | Algebraization, residuation, lattice-ordered algebras                  |
| Display Logic           | Display property, structural connectives, Belnap calculi               |
| Deep Inference          | Calculus of Structures, atomic flows, context-sensitive rewriting      |
| Categorical Semantics   | Cartesian closed categories, adjunctions, internal languages           |
| Fixed-Point Logics      | Inductive definitions, least/greatest fixed points, \\(\mu\\)-calculus |
| Generalized Connectives | Nonstandard operators, definability, compositionality                  |
| Logical Pluralism       | Multiple consequence relations, nonclassical semantics                 |

### **Key papers and books**

- Gerhard Gentzen (1935), *Investigations into Logical Deduction* — introduction and elimination rules, structural proof theory.
- Dag Prawitz (1965), *Natural Deduction: A Proof-Theoretical Study* — normalization and proof-theoretic foundations.
- Nuel Belnap (1982), *Display Logic* — general proof systems for substructural and modal logics.
- Michael Dummett (1991), *The Logical Basis of Metaphysics* — inferential meaning and logical justification.
- Peter Schroeder-Heister (1984), *A Natural Extension of Natural Deduction* — hypothetical rules and generalized inference.
- Sara Negri and Jan von Plato (2001), *Structural Proof Theory* — rule-based systems and proof transformations.
- Kosta Došen (1989), *Logical Constants as Punctuation Marks* — structural and inferential accounts of logical operators.
- Anupam Das and Lutz Straßburger (2016), *On Linear Rewriting Systems for Boolean Logic and Some Applications to Proof Theory* — rewriting and deep inference.

### **Potential Ein research questions**

**Rule-based relational semantics.** Can Ein define relations entirely through inference rules, without a primitive model-theoretic interpretation?

**Higher-order relational operators.** Can composition, projection, converse, transitive closure, and inclusion be characterized as proof-theoretic operators?

**Algebra extraction from rules.** Given a collection of deduction rules, can the system synthesize algebraic laws such as associativity, distributivity, monotonicity, and idempotence?

**Theory synthesis.** Can a new rule be justified by admissibility, conservativity, or proof-theoretic harmony rather than merely by empirical success on examples?

A useful formal target is:

\\[ \mathcal R\vdash\operatorname{Law}(O_1,\ldots,O_n) \\]

where \\(\mathcal R\\) is a rule set and \\(\operatorname{Law}\\) is a proposed algebraic property.

The important distinction is whether this law is **derivable, admissible, semantically valid, or merely consistent** with the rule set.

## **2. Structural Proof Theory**

The mathematical structure, transformation, normalization, and equivalence of proofs

**Central question:** What properties of a proof depend on its structure rather than merely on its conclusion?

### **Research topics and keywords**

| Subfield              | Concepts and keywords                                               |
| --------------------- | ------------------------------------------------------------------- |
| Cut Elimination       | Hauptsatz, cut admissibility, cut rank                              |
| Normalization         | Strong/weak normalization, normalization by evaluation              |
| Proof Identity        | Proof equivalence, commuting conversions, bureaucracy               |
| Proof Nets            | Graphical proofs, correctness criteria, sequentialization           |
| Deep Inference        | Locality, atomic flows, structural rewriting                        |
| Focusing              | Andreoli focusing, polarity, synchronous/asynchronous phases        |
| Proof Compression     | DAG proofs, lemma sharing, proof certificates                       |
| Proof Complexity      | Resolution complexity, proof length, width, lower bounds            |
| Proof Transformations | Permutation of inference rules, proof normalization                 |
| Interpolation         | Craig interpolation, Maehara's lemma                                |
| Subformula Property   | Analytic proofs, proof locality                                     |
| Ordinal Analysis      | Proof-theoretic ordinals, \\(\varepsilon_0\\), collapsing functions |
| Reflection Principles | Uniform reflection, local reflection, consistency strength          |
| Abstract Proof Theory | Generalized consequence relations, abstract proof systems           |

### **Key papers and books**

- Gerhard Gentzen (1935), *Investigations into Logical Deduction* — cut elimination.
- Gerhard Gentzen (1936), *The Consistency of Arithmetic* — transfinite induction and consistency.
- Jean-Yves Girard (1987), *Linear Logic* — resource-sensitive logic and proof nets.
- Jean-Marc Andreoli (1992), *Logic Programming with Focusing Proofs in Linear Logic* — focusing.
- Alwen Tiu (2006), *A System of Interaction and Structure II: The Need for Deep Inference* — deep inference.
- Lutz Straßburger (2006), *Proof Nets and the Identity of Proofs* — proof identity and graph representations.
- Stephen Cook and Robert Reckhow (1979), *The Relative Efficiency of Propositional Proof Systems* — foundational proof complexity.
- Samuel Buss (1998), *An Introduction to Proof Theory* — broad technical reference.
- Wolfram Pohlers (2009), *Proof Theory: The First Step into Impredicativity* — ordinal analysis.
- A. S. Troelstra and H. Schwichtenberg (2000), *Basic Proof Theory* — standard reference.

### **Potential Ein research questions**

**Proof DAGs versus derivation trees.** Can Ein represent shared subproofs canonically, reducing repeated derivations?

**Proof provenance.** Can every derived relation carry a machine-checkable justification, including the exact assumptions and rule instances used?

**Proof minimization.** Can Ein find minimal proofs, minimal premise sets, or minimal unsatisfiable hypothesis cores?

**Proof equivalence.** When two saturation schedules derive the same fact, are the resulting proofs equivalent? Under which equivalence relation?

**Normalization.** Can redundant intermediate relational facts be eliminated while preserving the conclusion?

**Proof certificates.** Can Ein emit a compact proof object that an independent kernel checks without trusting the search algorithm?

A useful distinction is:

\\[ \text{Same conclusion} \\;\not\Rightarrow\\; \text{Same proof} \\]

Even if hypothesis-set indexing makes the search state canonical, it does not necessarily make the **derivation history** canonical.

## **3. Proof Search**

Algorithms for constructing proofs efficiently, completely, and with controlled search complexity

**Central question:** How can we navigate an enormous space of possible derivations without losing proofs?

### **Research topics and keywords**

| Subfield                     | Concepts and keywords                                      |
| ---------------------------- | ---------------------------------------------------------- |
| Saturation-Based Reasoning   | Given-clause algorithms, redundancy elimination, fairness  |
| Connection Calculi           | Connection tableaux, model elimination, leanCoP            |
| Resolution and Superposition | Ordered resolution, paramodulation, term orderings         |
| Constraint Learning          | Conflict learning, nogoods, explanation-based learning     |
| CDCL / CDCL(T)               | Clause learning, implication graphs, theory propagation    |
| DPLL(T) / SMT                | Theory solvers, Nelson–Oppen, lazy SMT                     |
| Hypothesis Search            | Assumption-based reasoning, abduction, ATMS                |
| Lattice Search               | Boolean lattices, antichains, upward/downward closure      |
| Abstract Interpretation      | Monotone frameworks, fixed-point iteration, widening       |
| Datalog Evaluation           | Semi-naive evaluation, magic sets, incremental maintenance |
| Search Strategies            | Best-first, depth-first, iterative deepening, A\*          |
| Heuristic Guidance           | Premise selection, clause ranking, neural guidance         |
| Proof-Producing Search       | Proof logging, certified solvers, proof reconstruction     |
| Model-Based Reasoning        | Model evolution, counterexample-guided refinement          |
| Portfolio ATP                | Strategy scheduling, solver portfolios, parallel search    |
| Confluence                   | Church–Rosser, critical pairs, completion procedures       |

### **Key papers and books**

- J. Alan Robinson (1965), *A Machine-Oriented Logic Based on the Resolution Principle* — resolution and unification.
- Robert Kowalski and Donald Kuehner (1971), *Linear Resolution with Selection Function* — goal-directed resolution.
- Wolfgang Bibel (1982), *Automated Theorem Proving* — connection methods.
- Martin Davis, George Logemann, Donald Loveland (1962), *A Machine Program for Theorem-Proving* — DPLL.
- João Marques-Silva and Karem Sakallah (1999), *GRASP: A Search Algorithm for Propositional Satisfiability* — conflict-driven learning.
- Robert Nieuwenhuis, Albert Oliveras, Cesare Tinelli (2006), *Solving SAT and SAT Modulo Theories: From an Abstract Davis–Putnam–Logemann–Loveland Procedure to DPLL(T)*.
- Leonardo de Moura and Nikolaj Bjørner (2008), *Z3: An Efficient SMT Solver*.
- Stephan Schulz (2002), *E — A Brainiac Theorem Prover* — saturation-based ATP.
- Jens Otten and Wolfgang Bibel (2003), *leanCoP: Lean Connection-Based Theorem Proving*.
- Adam Chlipala et al. — research on proof-producing automation and reflective tactics.
- Michael Rawson, Clemens Eisenhofer, Laura Kovács (2025), *Constraint Learning for Non-confluent Proof Search* — connection calculus and learned constraints.

### **Potential Ein research questions**

**Confluent hypothesis-set exploration.** What completeness and termination guarantees follow from representing hypothesis states as subsets in a set-indexed DAG?

**Conflict antichains.** Can minimal inconsistent hypothesis sets be represented as an antichain, enabling efficient pruning of upward-closed regions?

**Incremental saturation.** Can Ein reuse closure results between adjacent hypothesis states rather than recomputing deductions?

**Proof-aware constraint learning.** Can contradiction proofs be used to extract minimal nogoods automatically?

**Search-space quotienting.** Can states equivalent under symmetry, renaming, or logical equivalence be merged beyond ordinary set-indexing?

**Theory-guided search.** Can newly synthesized rules be evaluated by their impact on proof search, redundancy, and completeness?

A natural mathematical foundation is a monotone consequence operator:

\\[ F:\mathcal P(X)\rightarrow\mathcal P(X) \\]

with:

\\[ A\subseteq B\Rightarrow F(A)\subseteq F(B) \\]

Then saturation is characterized by a least fixed point:

\\[ \operatorname{Closure}(A)=\operatorname{lfp}(X\mapsto A\cup F(X)) \\]

This connects Ein to Datalog, deductive databases, abstract interpretation, and fixed-point logics.

## **4. Cyclic Proofs**

Finite proof graphs representing potentially infinite reasoning, recursion, and induction

**Central question:** Under which conditions can a finite cyclic derivation represent a valid proof?

### **Research topics and keywords**

| Subfield                       | Concepts and keywords                                         |
| ------------------------------ | ------------------------------------------------------------- |
| Cyclic Proof Systems           | Regular infinite proofs, finite cyclic representations        |
| Global Trace Conditions        | Infinite-path conditions, progress measures                   |
| Infinite Descent               | Well-foundedness, descending measures                         |
| Inductive Proofs               | Mathematical induction, structural induction                  |
| Coinductive Proofs             | Greatest fixed points, bisimulation, productivity             |
| Inductive Definitions          | Least fixed points, inductively generated predicates          |
| Fixed-Point Logics             | \\(\mu\\)-calculus, \\(\mu MALL\\), \\(\mu LK\\)              |
| Cyclic Arithmetic              | Cyclic PA, cyclic HA, arithmetic strength                     |
| Cyclic Separation Logic        | Recursive predicates, heap reasoning                          |
| Parity Conditions              | Büchi/parity automata, infinite-trace acceptance              |
| Cyclic Proof Checking          | Decidability of trace conditions, automata-theoretic checking |
| Cyclic Cut Elimination         | Normalization, admissibility, infinitary transformations      |
| Proof Graph Compression        | Back-edges, shared subderivations, regularity                 |
| Circular Reasoning             | Sound circularity, guarded recursion                          |
| Termination Analysis           | Ranking functions, well-founded relations                     |
| Cyclic versus Inductive Proofs | Expressive equivalence, proof translations, proof complexity  |

### **Key papers and books**

- James Brotherston (2005), *Cyclic Proofs for First-Order Logic with Inductive Definitions* — foundational development of cyclic proof systems.
- James Brotherston and Alex Simpson (2011), *Sequent Calculi for Induction and Infinite Descent* — cyclic reasoning and inductive definitions.
- James Brotherston, Dino Distefano, Rasmus Petersen (2011), *Automated Cyclic Entailment Proofs in Separation Logic* — automated cyclic reasoning.
- Anupam Das (2018), *On the Logical Complexity of Cyclic Arithmetic* — proof-theoretic strength and cyclic arithmetic.
- David Baelde (2012), *Least and Greatest Fixed Points in Linear Logic* — fixed-point proof systems.
- Dexter Kozen (1983), *Results on the Propositional μ-Calculus* — foundational fixed-point logic.
- Graham E. Leigh and Dominik Wehr (2026), *Unravelling Cyclic First-Order Arithmetic* — relationships between cyclic and ordinary arithmetic proofs.

### **Example: induction as a cyclic structure**

Suppose we want to prove:

\\[ \forall n\\,P(n) \\]

An ordinary inductive proof establishes:

\\[ P(0) \\]

and

\\[ \forall n\\,(P(n)\rightarrow P(n+1)) \\]

A cyclic proof system can instead represent certain recursive proof obligations using back-edges.

Conceptual cyclic proof graph. The back-edge is sound only if an appropriate global progress condition is satisfied.

A cycle alone proves nothing. For example, accepting \\(P\\) because \\(P\\) is assumed would make every proposition provable.

Cyclic proof systems therefore impose conditions ensuring that infinite unfolding cannot produce an invalid infinite descent through a well-founded measure.

### **Potential Ein research questions**

**Cyclic relational proofs.** Can Ein represent recursive relational properties using finite proof graphs with back-edges?

**Induction synthesis.** Can a recurring deduction pattern suggest an induction hypothesis or a well-founded ranking function?

**Proof graph regularization.** Can repeated structures in a potentially infinite derivation be identified and compressed into a regular cyclic proof?

**Trace-condition verification.** Can an independent checker verify the soundness of cyclic proofs generated by Ein?

**Induction and coinduction.** Can Ein support both least-fixed-point reasoning and greatest-fixed-point reasoning, with clearly separated proof principles?

**Cyclic theory synthesis.** Can automatically discovered recursive rules be accepted only after proving a termination, productivity, or trace condition?

A useful target is:

\\[ \operatorname{CyclicProof}(G,\varphi) \land \operatorname{GlobalTraceValid}(G) \implies \operatorname{Valid}(\varphi) \\]

This is a schematic soundness statement, not a theorem for arbitrary cyclic graphs. Its validity depends on the precise calculus and trace condition.

## **5. Cross-cutting research areas**

Several additional disciplines connect all four tracks and are particularly relevant to EinAf.

| Area                                | Keywords                                                              | Relevance                                        |
| ----------------------------------- | --------------------------------------------------------------------- | ------------------------------------------------ |
| **Algebraic Logic**                 | Relation algebras, cylindric algebras, residuated lattices, quantales | Algebraic foundations of relational operators    |
| **Universal Algebra**               | Equational theories, varieties, term algebras, congruences            | Discovering algebraic laws                       |
| **Term Rewriting**                  | Critical pairs, Knuth–Bendix completion, rewriting modulo theories    | Rule normalization and confluence                |
| **Automated Theory Formation**      | Concept formation, conjecture generation, theory exploration          | Automatic synthesis of mathematical knowledge    |
| **Abductive Reasoning**             | Abduction, explanations, minimal hypotheses, ATMS                     | Hypothesis lattice and explanatory search        |
| **Abstract Interpretation**         | Galois connections, abstract domains, widening/narrowing              | Sound approximation of deduction                 |
| **Logic Programming**               | Datalog, CHC, SLG resolution, tabling                                 | Relational deduction and fixed-point computation |
| **Formal Concept Analysis**         | Concept lattices, closure operators, implications                     | Extracting structure from relational knowledge   |
| **Proof Complexity**                | Lower bounds, resolution width, proof-size complexity                 | Complexity of proof search                       |
| **Neuro-Symbolic Reasoning**        | Autoformalization, premise selection, neural-guided ATP               | EinAf integration with LLMs                      |
| **Proof-Carrying Computation**      | Certificates, independent checkers, LF, Dedukti                       | Trustworthy reasoning kernel                     |
| **Counterexample-Guided Synthesis** | CEGIS, countermodel generation, refinement                            | Theory synthesis and verification                |

## **6. Additional papers worth studying**

These are particularly relevant to connecting Ein's existing relational reasoning architecture with established theory.

| Paper / work                                                                                                                | Why it matters                                        |
| --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| Alfred Tarski (1941), *On the Calculus of Relations*                                                                        | Algebraic foundations of binary relations             |
| Alfred Tarski and Steven Givant (1987), *A Formalization of Set Theory Without Variables*                                   | Expressiveness of relation algebra                    |
| Patrick Cousot and Radhia Cousot (1977), *Abstract Interpretation: A Unified Lattice Model for Static Analysis of Programs* | Lattice-based fixed-point reasoning                   |
| Robert E. Bryant (1986), *Graph-Based Algorithms for Boolean Function Manipulation*                                         | Canonical graph representations and decision diagrams |
| Johan de Kleer (1986), *An Assumption-based TMS*                                                                            | Assumption sets, environments, nogoods                |
| Donald Knuth and Peter Bendix (1970), *Simple Word Problems in Universal Algebras*                                          | Completion and rewriting                              |
| Franz Baader and Tobias Nipkow (1998), *Term Rewriting and All That*                                                        | Confluence, termination, critical pairs               |
| Michael Gelfond and Vladimir Lifschitz (1988), *The Stable Model Semantics for Logic Programming*                           | Nonmonotonic reasoning and alternative models         |
| Thomas Eiter and Georg Gottlob (1995), *The Complexity of Logic-Based Abduction*                                            | Complexity of hypothesis generation                   |
| Konstantinos Sagonas, Terrance Swift, David S. Warren (1994), *XSB as an Efficient Deductive Database Engine*               | Tabling and reusable deductions                       |
| Armin Biere et al. (2009), *Handbook of Satisfiability*                                                                     | SAT search, propagation, conflict learning            |
| Leonardo de Moura and Nikolaj Bjørner (2008), *Z3: An Efficient SMT Solver*                                                 | Combining theories and automated reasoning            |

## **7. Research priorities for Ein**

I would organize the program into the following research clusters.

**Relational Proof Calculus**

Relational algebra, generalized composition, higher-order relations, inference-rule semantics, algebraic law synthesis, proof-theoretic harmony.

**Confluent Hypothesis-Lattice Reasoning**

Set-indexed DAGs, Boolean lattices, antichain algorithms, nogood learning, minimal inconsistent subsets, incremental saturation, closure operators.

**Proof Objects and Certified Deduction**

Proof DAGs, provenance, independent checking, proof normalization, proof compression, admissibility, soundness and completeness.

**Inductive and Cyclic Relational Reasoning**

Recursive rules, fixed-point semantics, induction synthesis, cyclic proof graphs, global trace conditions, coinduction.

**Neural-Guided Theory Synthesis**

Autoformalization, conjecture generation, premise selection, rule synthesis, counterexample-guided refinement, symbolic verification.

The first four clusters correspond closely to the four proof-theoretic areas. The fifth connects them to EinAf's neuro-symbolic objectives.

A particularly promising unifying question is:

> **Can a relational reasoning system automatically synthesize inference rules, derive their algebraic properties, construct and normalize proofs, and explore alternative hypotheses through a confluent lattice-based search architecture, while preserving formally stated soundness and completeness guarantees?**

That is a coherent research agenda rather than merely a collection of ATP features. It also makes clear which guarantees would need to be established mathematically rather than assumed from the implementation.

For publication-oriented work, the strongest starting point would be a precise formal definition of Ein's relational calculus, its consequence relation, and its hypothesis-state transition system. From there, soundness, closure, confluence, completeness, and admissibility become concrete theorem statements that can be proved, refuted, or qualified.



