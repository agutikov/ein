# Q-M1f.379 — An object is a node of the knowledge-base graph: an atom or a fact

A knowledge base is a graph, and an **object** is a node of it. Nothing else is called an object. The graph has two
sorts of node, so there are two sorts of object:

- an **atom** is a labelled node with no outgoing arrow, and its label is its **name**. `Norwegian`, `House-1`,
  `left-of` and `7` are atoms. *Name* is the word, and *atom* the node that carries it;
- a **fact** is a node with ordered outgoing arrows. Its first arrow points at its relation, an atom
  ([Q-M1f.381](Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)), and its other arrows at its
  **arguments**, each an object: an atom, or a fact
  ([Q40](Q40-a-fact-may-hold-a-fact-as-an-argument.md)). `(owns Norwegian Dog)` is a fact with three arrows.

So what stands in an argument's place, what a variable is bound to and what a guard compares is an object, and
which facts may stand there is [Q-M1f.113](Q-M1f.113-a-value-is-a-name-or-a-fact-a-state-could-hold.md)'s. *Atom*
never means an atomic formula: `(owns Norwegian Dog)` is a fact, and the atoms are the three nodes it points at. A
string, a variable, a keyword and a connective are no object, and no object has a type
([Q-M1f.372](Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)).

The graph starts from the Levi graph of a hypergraph, which has one node for each vertex, one for each hyperedge and
an arrow from a hyperedge's node to each vertex it joins, and it is not strictly that graph: a fact may be an
argument of a fact, so an arrow may point at a node that has arrows of its own, and the graph is not bipartite.
What it keeps is the two sorts of node, those without outgoing arrows and those with them.

**Why.** One word for whatever a fact can be about. An argument is an atom or a fact alike, a rule binds a variable
to either, and a statement about a fact or about a relation is a fact pointing at a node, with no second sort of
thing beside the graph. Naming the node, and keeping *name* for its label, is what lets a page say that a word
written twelve times is one atom.

**Held by.** The page: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The graph and
§ What is where, with the definitions drawn as one diagram. In the engine an object is an `ein_core::Value`, of
exactly two shapes, a name's symbol and a fact's id: `ein-core/src/value.rs`'s unit test
`the_two_shapes_round_trip_through_the_tag`; `ein-infer/tests/rule_semantics.rs`:
`a_nested_pattern_binds_inside_the_nested_fact`, a variable bound to an atom under a fact that is an argument.
