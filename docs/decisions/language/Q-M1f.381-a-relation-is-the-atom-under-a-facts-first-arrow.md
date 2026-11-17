# Q-M1f.381 — A relation is the atom under a fact's first arrow, and its extent is every fact of it

A **relation** is the object under the first arrow of a fact, and it is an atom. *Relation* is a part an atom plays
and not a sort of atom: the same atom may be an argument of another fact, and *the same relation* means that the
first arrows of two facts point at one atom. Three things go by a relation's name, and they are kept apart:

- **the relation**, the single atom;
- its **extent**, every fact whose first arrow points at that atom, whether a knowledge base holds it or not: the
  facts written and derived, a fact that is only an argument of another, and a fact no knowledge base holds yet. The
  part a knowledge base holds is the **held extent**;
- **a fact of** the relation, one member of the extent.

Which of them a statement is about is read off where the atom stands. A fact with the atom under its first arrow is
one fact of the relation and speaks of its own arguments. A fact with the atom under a later arrow speaks of the
relation as one object, of all its facts at once: `(transitive left-of)` does, `(__closed__ lives-in)` does, and so
does a **declaration**, `(relation left-of House House)`, which is itself a fact, its first arrow at the atom
`relation`, its second at the atom it declares and the others at that relation's labels. A **declared relation** is
the atom a declaration names. Neither definition includes the other: a declared relation of which there is no fact
is under no first arrow, and a relation nothing declares is a relation all the same.

So what is closed, complete, read by an `absent` or still to be extended is an extent or a held extent, and what is
declared, listed by name or passed to a rule's parameter is the relation.

**Why.** The relation being one node is what lets one fact speak of all the facts of it, and a rule take a
relation as a parameter. Keeping the atom apart from its facts keeps a statement about what a state holds, *the
held extent of `lives-in` is complete*, from reading as a statement about the atom, and the extent being every fact
of the relation, held or not, makes it a property of the relation alone, the same in every state.

**Held by.** The page: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The relation,
its extent, a fact of it, whose drawing has a fact of the extent that is not held. `ein_ir::from_ir`'s unit test
`a_declaration_stores_itself_as_facts_so_rules_can_read_signatures`, a declaration as facts about the relation;
`ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, the count a declaration
fixes for every fact of the relation; `tests/stdlib/algebra/21_transitive.ein`, one fact about a relation read by a
rule over its facts; `ein-core/src/kb.rs`'s unit test `a_rebuild_reconstructs_the_extent_counts`, the size of a held
extent.
