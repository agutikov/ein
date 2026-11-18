# Q-M1f.430 — The canonical form draws a knowledge base, and the views of states draw states

The canonical form, and its fold, is the drawing of a knowledge base's objects and facts: the KB, program and
constraint views, a rule's patterns and a derivation's facts
([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)). The lattice view, the region view and the
h-space graphs draw states, nodes and candidates, each a node labelled with what it holds or proposes, in the way
that reads best for an order between sets: their label is a list of facts, and no drawing of a fact.

**Why.** A state is a set of facts and the lattice an order between such sets. Drawn with each state's facts as a
graph, a lattice is a lattice of graphs, several times as wide, and its point, which set is above which, is lost
among the facts' arrows; the canonical form is for reading what a knowledge base holds.

**Held by.** `ein_render::region_dot`, `ein_render::dump::region_lattice` and `ein_render::structure`, which draw no
fact's octagon; the page [rendering](../../ein/user/04-reference/12-rendering.md), § The lattice, § The region and
§ The hypothesis space.
