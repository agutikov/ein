# Q-M1f.382 — A labelled arrow draws a fact of two arguments and is never the representation

A fact with three arrows, a relation and two arguments, may be drawn as one arrow from its first argument to its
second, labelled with the relation's name. That is a convention of drawing, for a reader. In the graph a fact is a
node, a directed hyperedge over the objects under its arrows, and the facts of one relation are as many nodes whose
first arrows meet at one atom ([Q-M1f.380](Q-M1f.380-an-objects-identity-is-its-name-or-its-tuple.md)).

Drawn with labelled arrows, the same facts are as many arrows carrying one word, and the relation is no node. So
the drawing has no place for a fact about the relation, a declaration or a property, which points at that node, nor
for a fact that points at a fact, and it draws a fact of any other count of arguments as a node all the same. No
page defines a knowledge base, a fact or a relation by it: a drawing with labelled arrows says that it is one, or
is a view `ein render` writes, whose legend states the convention. `ein render program --levi` draws every fact as
a node.

**Why.** A labelled arrow is the natural picture of a fact of two arguments and the one a reader takes in fastest,
which is why the views draw it. It loses the relation's identity: there are many arrows with one label, where the
graph has many facts and one atom their first arrows point at. A definition read off the drawing would make a
relation a label, which nothing can point at, and a fact of three arguments a different sort of thing from a fact of
two.

**Held by.** The pages: [the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § Drawn with
labelled arrows, with one pair of facts in both forms and the table of what each element of the graph is drawn as;
[rendering](../../ein/user/04-reference/12-rendering.md), the legend of the views.
`ein-cli/tests/render_doors.rs`: `every_door_prints_dot_that_dot_draws`, one program drawn in both forms.
