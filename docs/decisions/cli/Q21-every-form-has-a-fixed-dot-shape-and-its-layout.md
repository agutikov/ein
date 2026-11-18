# Q21 — Every form has a fixed DOT shape, and its layout is free

Every form of the program and of the KB has a fixed DOT shape: a view's DOT fixes the graph's structure, and its
layout (positions, ranks, the style choices the shape leaves open) is Graphviz's to choose. A fact is a node of the
graph and DOT has no hyperedge, so a fact of any count is drawn as its node, an octagon with an edge numbered `1` to
its relation's atom and one to each argument; under `--fold`, a fact of three arrows that nothing drawn points at is
one labelled arrow ([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)). `ein` writes DOT and
never rasterises it.

**Why.** A program is one structure with two surfaces, the text to write and the graph to look at, and the meaning is
in the structure. Fixing the structure and not the layout makes a view comparable across runs and versions of
Graphviz while leaving the drawing to the tool that draws.

**Held by.** `ein_render::ir_dot` and `ein_render::kb_dot`; `ein-render/tests/dot_wellformed.rs`, the views of the
corpus programs accepted by `dot`; `ein-render/tests/id_order_invariance.rs`:
`every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, every DOT view's bytes held to a digest.
The shapes: [12-rendering.md](../../ein/user/04-reference/12-rendering.md).
