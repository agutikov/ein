# Q-M1f.523 — The facts about relations are every fact a knowledge base holds with a relation of the program under an argument arrow

A **slice** of a knowledge base is the facts it holds that a selection keeps, drawn with what they point at
([Q-M1f.527](Q-M1f.527-a-slice-is-drawn-with-what-its-facts-point-at.md)). **The facts about relations** are the slice
of every fact the knowledge base holds with a relation of the program **directly** under one of its argument
arrows, `2`, `3`, …: `(symmetric next-to)`, `(bijective color-loc)`, a declaration `(relation lives Person House)` and
the fact `(relation lives)` the loader stores beside it, `(__closed__ asked)`, and a fact a rule derives of them,
`(functional-negative color-of)`, as well as one the program states.

- **A relation of the program** is the one notion a goal and a claim are held to
  ([Q-M1f.119](../language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)): an atom the program writes
  first in a tuple ([Q-M1f.522](../language/Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)), the
  kernel's words it writes among them, and `eq` and `neq`. It is the program's, so the slice reads every state of the program by the same relations, and a relation
  with no fact in a state is one there too.
- **An atom that is a relation is one wherever it stands**: `(n z 4)` is kept, `z` being the relation of no argument
  `std.math.arith` declares and a numeral's zero, and so is an order's `(cl eq 0 0 eq)`. The graph does not tell an
  atom used as a relation from the same atom used as a value, and `eq` and `neq` are not left out.
- **Directly**: a fact whose relation of the program stands only under an arrow of a fact under its own arrows,
  `(claims Bob (symmetric lives))`, is not kept, and neither is a fact under an arrow that the knowledge base does not
  hold: it is drawn through a kept fact that points at it.

The slice is defined in the engine and read off the graph alone: no fact's `:source`, its rule or the file it is
written in, and no name read for what it means. It is no goal, and the language is unchanged: a goal names its
relations, a pattern sees one count and one place, and nothing a goal can write says *this atom is a relation of the
program*.

**Why.** A fact with a relation's atom under a later arrow speaks of the relation as one object
([Q-M1f.381](../language/Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)), so this is the definition
of a fact about a relation, read on the graph and needing no annotation. One notion of a relation of the program
serves a goal, a claim and the slice, so the slice is the same selection in every state, as a property of a relation
is the same property in every state.

**Held by.** `ein_infer::slice::about_relations` and its unit test
`the_facts_about_relations_are_those_with_a_relation_under_an_arrow`; `ein_core::Program::relations_of`, which the
loader fills; `ein-render/tests/every_object_once.rs`: `the_constraint_view_draws_a_property_as_a_fact`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#constraints--ein-render-constraints).
