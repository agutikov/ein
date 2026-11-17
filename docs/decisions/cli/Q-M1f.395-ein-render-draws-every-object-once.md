# Q-M1f.395 — `ein render` draws every object once

In every drawing of facts `ein render` writes, a fact that a drawn fact points at is drawn **once, as its node**:
an octagon with the relation's name in it and a numbered arrow to each of its arguments, whatever its count and
whether or not the knowledge base holds it. A fact of two arguments is drawn as a labelled arrow **only where no
drawn fact points at it**; one that something points at is its octagon and no arrow. No node is labelled with a
fact's text.

The octagon carries the two marks of the drawings of the graph
([Q-M1f.398](../docs/Q-M1f.398-a-fact-in-a-context-is-drawn-in-a-frame-named.md)): it is **filled** where the
knowledge base holds the fact, and it stands in a **dashed frame** labelled with the relations of the facts that
point at it and the word `ctx`, one frame naming every such relation, in order, frames side by side. A fact that
points is every fact the view draws: in the KB view the loader's `written` facts among them, so a frame there may
read `claims, written ctx`. What is drawn outside every frame is held.

It holds of the four writers that draw facts, each in its own drawing: the KB view, a state among them
(`ein render kb`), the program view with its Levi form (`ein render program`), a rule's two modes
(`ein render rule`, `ein render rules`), where a pattern under an arrow of a pattern is the octagon, filled where
it is also a premise or a literal of the conclusion, and the constraint view (`ein render constraints`). A negation
that is drawn as the fact it denies, dashed or with `¬`, is an octagon `not` that points at that fact's node where
the fact is one. A drawing with no fact under an arrow is the drawing it was. The views of a derivation, the slice
of a trace and the derivation graph, draw each fact of the derivation as one node that carries its sentence, and
are not drawings of facts with their arrows.

**Why.** A fact is one node of the knowledge base wherever it is written, so a drawing that writes a fact under an
arrow as a node labelled with its text draws a fact that is also held twice, as an arrow and as that node, and
shows neither that the two are one nor what the fact under the arrow is made of.

**Held by.** `ein-render/tests/every_object_once.rs`, each writer on one knowledge base and one rule: a fact in
the contexts of two relations is one octagon in one frame that names both; a fact held and pointed at is filled
and no arrow; a held fact that nothing points at is an arrow; the variables of a nested pattern are drawn.
`ein-render/tests/id_order_invariance.rs` holds every corpus program's drawings to their digests, and
`ein-render/tests/dot_wellformed.rs` puts each to Graphviz. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#a-fact-under-an-arrow--every-object-once).
