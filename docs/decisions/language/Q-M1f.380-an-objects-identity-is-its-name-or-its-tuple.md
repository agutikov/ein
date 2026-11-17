# Q-M1f.380 — An object's identity is its name or its tuple, and the graph is the representation

An atom is identified by its name, and a fact by the list of the objects its arrows point at, its relation first.
The graph has exactly one node with a given name and exactly one node with a given list of arrows:

- two atoms are one when their names are one word, character for character, so `1` and `01` are two atoms
  ([Q-M1e2.24](Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md));
- two facts are one when they have one relation, one count of arguments and one object in each place, at every
  depth, so the count and the order of the arguments are part of what a fact is;
- no atom is a fact.

Where an object is written, how often, a fact's `:source` and whether a knowledge base holds the fact are no part of
its identity. A fact that is an argument of another is the same object whether or not anything holds it, and one
fact held by two knowledge bases is one fact.

**The graph is the representation.** It is the knowledge base, and not a picture of one. A written list of tuples,
the facts of a program or the listing of a state, is a notation for it. The notation repeats a name wherever a fact
uses it, `left-of` once for each fact of the relation, and the graph has that atom once; reading the notation sends
each written name to the one node with that name and each written tuple to the one node with that list of arrows.

**Why.** In a list of tuples a name can be written many times, and in the graph there is only one node with that
name and only one with that list of arrows. So what makes two things one is a property of the graph and needs no
comparison of spellings: *the same relation* is two first arrows meeting at one atom, a fact speaks of a relation
or of another fact by pointing at its node, and nothing ever merges two nodes
([Q1](Q1-the-knowledge-base-is-a-hypergraph-of-atoms.md)).

**Held by.** The page: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The graph, and
how it is written, and § Identity. `ein-core/src/intern.rs`'s unit test `interning_is_injective_and_stable`, one
name one symbol; `ein-core/src/facts.rs`'s unit tests `interning_is_injective_and_stable`,
`arity_is_part_of_identity`, `nested_identity_cascades_through_the_inner_fact` and
`interning_a_nested_fact_says_nothing_about_belief`, one list of arrows one fact, at every depth, held or not;
`ein-infer/tests/search_semantics.rs`: `a_nested_positive_pairs_through_structural_identity`.
