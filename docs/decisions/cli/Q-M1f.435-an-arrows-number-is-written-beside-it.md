# Q-M1f.435 — An arrow's number is written beside it

Every number on an arrow of a fact is the arrow's `xlabel`, which Graphviz places beside the arrow once the graph is
laid out, in every drawing of facts; so is a guard's sign on its link. In a drawing of a knowledge base, the KB view,
the program view's `kb` and the constraint view, a folded arrow's relation is its `xlabel` too. A rule's panel and a
trace's slice, small, write a folded arrow's relation as its `label`.

**Why.** A `label` is a node of `dot`'s layered layout, and one in a digraph makes `dot` lay out every arrow over two
ranks: a knowledge base of some hundreds of facts in long chains, which `dot` lays out in seconds with `xlabel`s, takes
it minutes with `label`s. A `label` on a link that ranks nothing is also one the spline router can find no room for.

**Held by.** `ein_render::fact_graph`, `ein_render::rules` and `ein_render::slice`; `ein-render/tests/dot_wellformed.rs`,
which lays out every view of the corpus programs. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#node-shape-legend).
