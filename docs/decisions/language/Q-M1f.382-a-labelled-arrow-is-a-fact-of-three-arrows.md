# Q-M1f.382 — A labelled arrow is a fact of three arrows folded, and never the representation

A fact with three arrows, a relation and two arguments, may be **folded** into one arrow from its first argument to
its second, labelled with the relation's name. That is a convention of drawing, for a reader, asked for and never
the default: `ein render` draws every fact as its node unless `--fold` is given
([Q-M1f.419](../cli/Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)). In the graph a fact is a node, a
directed hyperedge over the objects under its arrows, and the facts of one relation are as many nodes whose first
arrows meet at one atom ([Q-M1f.380](Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)).

Folded, the same facts are as many arrows carrying one word, and the relation is no node. So the drawing has no
place for a fact about the relation, a declaration or a property, which points at that node, nor for a fact that
points at a fact, which keeps its node: a fact that a drawn fact points at is never folded. A fact of two arrows, a
relation and one argument, is folded the same way into a loop on its argument, labelled with the relation's name
([Q-M1f.425](../cli/Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md)); a fact of one arrow, or of four
or more, stays its node. No
page defines a knowledge base, a fact or a relation by the fold: a drawing with labelled arrows says that it is
one, or is a view drawn with `--fold`, whose legend states the convention.

**Why.** A labelled arrow is the natural picture of a fact of two arguments and the one a reader takes in fastest,
which is why the views offer it. It loses the relation's identity: there are many arrows with one label, where the
graph has many facts and one atom their first arrows point at. A definition read off the drawing would make a
relation a label, which nothing can point at, and a fact of three arguments a different sort of thing from a fact of
two.

**Held by.** The pages: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § Drawn with
labelled arrows, with one pair of facts in both forms and the table of what each element of the graph is drawn as;
[rendering](../../ein/user/04-reference/12-rendering.md), the legend of the views.
`ein-render/tests/every_object_once.rs`, every view of the corpus in both forms, and
`ein-cli/tests/render_doors.rs`: `every_door_draws_a_fact_in_both_forms`.
