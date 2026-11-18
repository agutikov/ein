# Q-M1f.395 — `ein render` draws every object once

In every drawing `ein render` and `ein solve --trace` write, every object is drawn **once, as its node**. A fact is
an octagon with no name in it, its first arrow at its relation's atom and the next at its arguments
([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)), whatever its count and whether or not the
knowledge base holds it. Folded on request, a fact of three arrows is one labelled arrow **only where no drawn fact
points at it**; one that something points at is its octagon and no arrow, so a fact stated and also claimed is drawn
once, not as an arrow and again as a node. No node is labelled with a fact's text.

The octagon carries the mark of the drawings of the graph
([Q-M1f.398](../docs/Q-M1f.398-a-fact-in-a-context-only-is-drawn-in-a-frame.md)): where the knowledge base does
not hold the fact, in a context only, it stands in a **dashed frame** labelled with the relations of the facts
that point at it and the word `ctx`, one frame naming every such relation, in order, frames side by side; where
the knowledge base holds it, it stands in no frame. No octagon is filled. A fact that points is every fact the
view draws. What is drawn outside every frame is held.

It holds of every view and of the drawings of a derivation alike: the KB view, a state among them (`ein render kb`),
the program view (`ein render program`), a rule's two modes (`ein render rule`, `ein render rules`), where a pattern
under an arrow of a pattern is its octagon, in no frame where it is also a premise or a literal of the conclusion,
the constraint view (`ein render constraints`), and a trace's slices, the derivation graph and a written trace's
`dag` view, where a fact a step reads or concludes is its node and the step a node of its own. A stored negation
is a fact of `not` whose second arrow points at the node of the fact it denies, in the frame `not ctx` where the
fact is not held.

**Why.** A fact is one node of the knowledge base wherever it is written, so a drawing that writes a fact under an
arrow as a node labelled with its text draws a fact that is also held twice, as an arrow and as that node, and
shows neither that the two are one nor what the fact under the arrow is made of.

**Held by.** `ein-render/tests/every_object_once.rs`, each writer on one knowledge base and one rule, in both forms:
a fact in the contexts of two relations and not held is one octagon in one frame that names both; a fact held and
pointed at is one octagon in no frame, and no arrow; no node is filled; the variables of a nested pattern are
drawn; and every view of every corpus program, in both forms. `ein-render/tests/id_order_invariance.rs` holds every
corpus program's drawings to their digests, and `ein-render/tests/dot_wellformed.rs` puts each to Graphviz. The
page is [rendering](../../ein/user/04-reference/12-rendering.md#a-fact-under-an-arrow--every-object-once).
