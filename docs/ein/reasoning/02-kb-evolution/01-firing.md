# Firing a rule — what Ein solves, and the operations it performs

This page places Ein's reasoning: what it answers, the three classical paradigms it is built from, the steps every
answer goes through, the objects those steps act on, and the nine operations an implementation performs, each with
its name in the fields it comes from. The pages after it state each step exactly.

## What Ein answers

Ein answers **finite-domain constraint-satisfaction** questions over a typed hypergraph of facts and the rules over
it, the Zebra puzzle and its kin. The answer to a program is one object, the region R of the commitments whose
state is alive and discharges what it owes ([solution semantics](06-solution-semantics.md)), and every question a user
asks is read off it, never chosen in advance:

- **solve**: is there exactly one state? R is one state;
- **gaps**: which cells are forced and which are open? R holds several states, and what they share is forced;
- **contradictions**: why is there none? R is empty, and a dead root or node carries a core, the facts its
  contradiction rests on ([cores](09-cores.md)).

These are three readings of one answer, not three problems: the same program gives the same R whichever is asked.

## Three paradigms

Almost every part of Ein is a piece of one of three classical paradigms:

| paradigm | what Ein takes from it | classic systems |
|---|---|---|
| **deductive database** (Datalog) | forward chaining to a fixpoint, bottom up, never retracting; negation judged one closure at a time, at a boundary | Soufflé, LogicBlox, DDlog |
| **constraint satisfaction and SAT** | propose undecided choices, propagate, detect a clash; a refuted choice is not entered again | DPLL and CDCL solvers, CSP solvers, answer-set programming |
| **truth maintenance** (ATMS) | hypotheses as assumptions, every fact with its justifications, a contradiction explained by the assumptions and givens it rests on | de Kleer's ATMS, JTMS |

The single most useful reframing: **the deductive part of Ein is a Datalog engine, and the search is an
environment search over sets of assumptions.** Ein reasons over **sets of commitments**, read as a lattice, rather
than over one decision variable at a time, and it keeps an explicit world per assumption set with full provenance
rather than one trail. That is what makes *forced*, *open* and *why not* fall out of one answer, at the cost of the
machinery a per-variable solver has.

## Two layers

The reasoning splits along the monotone/non-monotone seam:

- **The deductive layer**, monotone and append-only: saturate a knowledge base by its rules to a fixpoint, never
  retracting a fact. It is itself two-phase: a purely positive closure that consults no negation, and a boundary at
  each quiescence of the closure where the `(absent …)` premises are judged against that fixpoint
  ([saturation](02-saturation.md), [`absent`: negation as failure](03-absent-and-naf.md)).
- **The search over commitments**, where the non-monotonicity lives: a candidate set H is read off root's fixpoint,
  and each commitment `C ⊆ H` names a node, root's fixpoint with `C` written in and saturated again. Nothing a node
  derives is written back into root; a commitment that leads to a contradiction is not retracted, its node is dead,
  and a search learns from it by not entering what it decides ([finding R](07-finding-r.md)).

## The main steps

```text
  a program (.ein)
     │  parse, expand macros, resolve imports, load
     ▼
  the knowledge base K₀: relations, rules, hrules, facts, each fact with its provenance
     │  each rule instantiated once per activator fact of its parameters
     ▼
  root = sat(K₀)                         closure to quiescence, then the boundary, repeated
     │  contradiction?  → root is dead, R is empty, root's core explains it
     │  what root owes                    the obligations it holds no witness for
     ▼
  H, the candidate set                    the hypothesis ladder, read off root's fixpoint
     │
     ▼
  for a commitment C ⊆ H:  st(C) = sat(root + C)
     │  dead(C): st(C) holds a contradiction
     │  owes(C): the obligations st(C) holds no witness for
     ▼
  R = { C ⊆ H : st(C) is alive and owes nothing }     every question is read off R
```

Every step but the last is a function of the program alone. The last is a definition that several routes compute,
each held to the definition by brute force on every program small enough
([finding R](07-finding-r.md)).

## The objects

| object | what it is | elsewhere |
|---|---|---|
| **fact** | a relation applied to arguments, each a name or a fact; its identity is the relation and the arguments, whatever derived it | a ground atom, a tuple, a labelled hyperedge |
| **stored negation** | `(not F)`: the statement that the fact `F` does not hold, stored as a fact is | a negative literal |
| **relation** | a name and a signature of types | a database schema, a predicate symbol |
| **rule** | parameters, a `:match` premise, an `:assert` conclusion and a `:why`; applied once per activator fact of its parameters | a Datalog or production rule, a Horn-like clause |
| **world** | one knowledge base under saturation, asked at a quiescence: root, or a node | a possible world, an ATMS environment |
| **derivation record** | how a fact came to be: given, a rule's firing (rule, activator, premises, bindings, the absences it rested on), or a commitment ([cores](09-cores.md)) | a TMS justification, why-provenance, a proof term |
| **commitment** | a set of candidates assumed together | a partial assignment, an ATMS environment |
| **state** | a commitment's world at its fixpoint, identified by its fact set ([values and their order](../01-static-ir/03-values-and-order.md)) | a model, an answer set |
| **core** | of a dead node, the facts its contradiction rests on, read off its derivation records | an unsat core, a conflict |

## The core operations

Strip away the puzzle vocabulary and the reasoning performs nine operations.

- **O1, conjunctive matching.** Bind a rule's premises, `(R ?a ?b) ∧ (S ?b ?c) ∧ …`, against the knowledge base. A
  match is structural: two names are two objects, and nothing makes them equal.
- **O2, saturation.** Fire rules until no firing adds a fact ([saturation](02-saturation.md)).
- **O3, negation as failure.** Judge `(absent P)` and the `forall` built from it, once, at the closure's fixpoint
  ([`absent`: negation as failure](03-absent-and-naf.md)).
- **O4, equality.** Ein has none of its own: equality between objects is whatever relation a program states, read
  as any other relation, and `eq` and `neq` compare two values as written.
- **O5, contradiction detection.** Find `(false)`, or a fact beside its stored negation ([cores](09-cores.md)).
- **O6, provenance and the core.** Record every derivation of each fact; search those derivations for the smallest
  set of givens and commitments a contradiction rests on ([cores](09-cores.md)).
- **O7, the candidate set and the lattice.** Read the undecided candidate facts off root's fixpoint, and the
  commitments as subsets of them ([hypothesis generation](04-hypothesis-generation.md),
  [the hypothesis space](05-hypothesis-space.md)).
- **O8, learning from a refutation.** On a monotone program a superset of a dead commitment is dead, so one dead
  node decides every node above it; what a route may conclude without saturating a node is stated as its licences
  ([finding R](07-finding-r.md), [monotonicity](08-monotonicity.md)).
- **O9, state identity.** Two commitments whose worlds hold the same facts are one state; the identity is the sorted
  fact set itself, never a hash of it ([values and their order](../01-static-ir/03-values-and-order.md)).

## Analogs in other fields

| op | this is, elsewhere | canonical names |
|---|---|---|
| **O1** matching | relational join; conjunctive-query evaluation; production-system matching; subgraph homomorphism | RETE, TREAT, LEAPS; hash, sort-merge and index joins; worst-case-optimal joins |
| **O2** saturation | Datalog bottom-up evaluation; transitive closure; chaotic-iteration fixpoints; forward chaining | naive and semi-naive evaluation; magic sets; DRed; differential dataflow |
| **O3** negation | stratified negation; the closed-world assumption; default logic | stratified Datalog; well-founded and stable-model semantics |
| **O4** equality | disjoint sets; congruence closure; term rewriting | union-find; Nelson–Oppen, Downey–Sethi–Tarjan; e-graphs and equality saturation |
| **O5** clash | clause violation; integrity-constraint check; tableau clash | unit-propagation conflict; watched literals |
| **O6** provenance | truth maintenance; database provenance; proof certificates | ATMS and JTMS justifications; provenance semirings; DRUP and DRAT; MUS |
| **O7** the lattice | CSP value enumeration; SAT decisions; frequent-itemset mining; ATMS environments | DPLL decisions; Apriori candidate generation; minimal hitting sets |
| **O8** learning | conflict-clause learning; constraint propagation; consistency | CDCL; conflict-directed backjumping; arc consistency |
| **O9** identity | state canonicalisation; symmetry breaking | order-insensitive keys; SBDS and SBDD; graph canonisation |

## The design principles

1. **The graph is the knowledge, and reasoning only adds to it.** A world's facts only grow; a rule never removes a
   fact, and a fact never changes. What a search learns from a dead node, it learns by not entering it.
2. **Rules may be higher-order.** A rule's parameters can stand for relations, bound by its activator facts, so one
   rule states a property for every relation that has it (`(functional color-loc)` activates the functional rules
   for `color-loc`). The matcher ranges over relations as it ranges over objects.
3. **Every firing leaves provenance.** Each fact records how it came to be, every recorded derivation of it and not
   only the first, so a fact is an OR over its derivations and a derivation an AND over its premises; a trace, a
   `:why` text and a core are all read off that one structure
   ([Q18](../../../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)).
4. **Saturate before supposing.** Propagation decides everything it can before anything is hypothesised: H is
   proposed at root's fixpoint, and a node is judged as its rules leave it
   ([Q19](../../../decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md)).
5. **Encoding-agnostic.** The reasoning works over any ontology a program writes: the Zebra puzzle encoded with one
   generic equivalence relation (`examples/zebra.ein`) and with five typed relations (`examples/zebra2.ein`) reaches
   one model. Membership is facts like any other; no type has a meaning the engine reads.
6. **Negation sits on the closure/world boundary.** The closure is purely positive and consults no negation;
   `(absent P)` is asked once, of a saturated world, never of a half-built one
   ([`absent`: negation as failure](03-absent-and-naf.md)). So on a stratified program the order rules fire in decides
   nothing derivable, and on a monotone program it decides no answer.
7. **Ein decides every program itself.** No part of a program is handed to an external solver: an answer the
   engine gives is one it can explain from the program's own rules
   ([Q2](../../../decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md)).

## What holds it

| claim | test |
|---|---|
| one engine, three readings, chosen by the input and never by which question was asked; a contradiction's core names the culprit | `ein-infer/tests/acceptance.rs`: `the_three_readings_agree_with_the_input`, `the_contradictions_view_is_the_verdict_explained` |
| a satisfiable puzzle is never reported unsatisfiable, and an unsatisfiable one never solved | `ein-infer/tests/acceptance.rs`: `a_satisfiable_puzzle_is_never_unsat_and_an_unsat_one_is_never_solved` |
| the two Zebra encodings reach the same model | `ein-infer/tests/acceptance.rs`: `both_ontologies_reach_the_same_model`; `ein-cli/tests/acceptance_cli.rs`: `solve_zebra_prints_the_same_answer_through_the_generic_encoding` |
| every route computes the R the definition gives | `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle` |
| matching is structural, and two names are never resolved as one | `ein-infer/tests/naf_semantics.rs`: `matching_does_not_resolve_equality_classes` |
| nothing is retracted | `ein-infer/tests/naf_semantics.rs`: `an_unstratifiable_loop_converges_to_a_supported_model` |
| a rule written once with a relation parameter fires for each relation its activators name | `ein-infer/tests/stdlib_semantics.rs`: `a_bijective_declaration_fans_out_into_the_operational_activators` |
