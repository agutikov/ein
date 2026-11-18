# Q-M1f.421 — No view draws a relation otherwise than another

`is-a` is an ordinary relation, and so is `relation`. No drawing gives either, or any relation, a look of its own:
no dashed arrow with an empty head for a fact of `is-a`, no box for an atom an `is-a` fact points at and no oval for
one it starts from, every atom one shape. A declaration is drawn as the fact it is, an octagon whose first arrow is
at the atom `relation`, or folded where it has three arrows
([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)), in the program view and the KB view
alike. `ein render kb` takes no option that leaves out what an `is-a` fact points at or starts from.

**Why.** Ein has no types ([Q-M1f.372](../language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)):
`is-a` is a relation a program declares or a theory brings, and a drawing that knows its name draws one theory's
reading of it into every program's picture.

**Held by.** `ein-render/tests/every_object_once.rs`: `is_a_and_relation_are_ordinary_relations`, and the sweep
`every_view_of_every_corpus_program_keeps_the_rules`, which fails on an oval, an empty arrowhead or a class node.
The page is [rendering](../../ein/user/04-reference/12-rendering.md#node-shape-legend).
