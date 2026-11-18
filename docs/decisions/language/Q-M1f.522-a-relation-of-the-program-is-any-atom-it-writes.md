# Q-M1f.522 — A relation of the program is any atom it writes first in a tuple

An atom is a relation of the program wherever the program writes it first in a tuple, at any depth: the relation of
a fact it states, of a fact inside another, `=` and `+` in `(think Bob (= 7 (+ 3 4)))`, of any form in a rule's, an
hrule's or an obligation's `:match` or `:assert`, and the relation an activator gives a parameter that heads a form
of its rule, `happy` in `(tag happy)` beside `(rule tag (?R) :match (person ?x) :assert (?R ?x))`. Writing the atom
first in a tuple is enough. A **declaration**, `(relation R …)`, is an optional fact, a convention some modules of the
standard library use to reason about a relation's arguments; no relation of the program depends on one, and a
relation the program declares is one of its relations as any other is.

**A relation is no object the blind enumerator guesses about**, whichever way the program writes it, and neither is
the relation of a fact root's fixpoint holds: `(likes Ann happy)` is no candidate beside `(likes Ann Bob)`.

**The kernel's own words are not made relations of the program by heading a tuple**, since the kernel reads each as
a form of its language: the connectives `and` and `or`, `absent`, the computed relations `eq` and `neq`, the negation
`not`, whose fact is of its argument's relation, `false`, the verdict `open`, the declaration `relation` and the
closure marker `__closed__`; and neither are the names the loader makes up, which start with `#`. `=` is none of
them: the grammar keeps it a name, and the kernel reads no form of it. A goal and a claim may name the kernel's words
a program writes, and `eq` and `neq`, which every program has
([Q-M1f.119](Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)).

**Why.** Being the first item of a tuple is enough to be a relation, a nested fact's included, and a declaration is
a convention: a relation is the atom under a fact's first arrow
([Q-M1f.381](Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)), and a program writes its relations
however it writes its tuples. One notion serves every reader of the program's relations: what is an object and what
the blind enumerator guesses about, the alive-set invariant's *no new relations*, and the list a goal, a claim and a
`(solve …)` list are held to ([Q-M1f.114](Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)). A relation
taken for an object would be guessed about as a value, a state holding `(likes Ann happy)` about a name that is no
thing the program speaks of.

**Held by.** `ein_ir::from_ir`'s `relations_written`, `register_relations` and `program_relations`, and
`ein_core::terms::heads_a_relation`, the kernel's words; `ein_infer::hypgen::candidate_objects`; the unit tests
`from_ir`'s `the_registry_holds_every_atom_first_in_a_tuple` and `hypgen`'s `candidate_objects_hold_no_relation`;
`ein-infer/tests/alive_invariant.rs`: `no_run_reaches_a_name_the_program_does_not_have`, whose relations are the
registry's and the kernel's words, over every state a bounded describe meets on the corpus;
`ein-cli/tests/goal_premises.rs`: `a_goal_over_a_relation_of_the_program_loads`, whose `a-facts-argument` and
`passed-to-an-activator` ask about a relation written only inside a fact and one only an activator names. The pages:
[the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md),
[saturation § The alive-set invariant](../../ein/reasoning/02-kb-evolution/02-saturation.md#the-alive-set-invariant),
[hypothesis generation § 5](../../ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#5-the-blind-enumerator).
