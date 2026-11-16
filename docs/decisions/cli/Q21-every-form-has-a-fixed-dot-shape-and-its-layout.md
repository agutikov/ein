# Q21 — Every form has a fixed DOT shape, and its layout is free

Every form of the program and of the KB has a fixed DOT shape: a view's DOT fixes the graph's structure, and its
layout (positions, ranks, the style choices the shape leaves open) is Graphviz's to choose. DOT has no hyperedge, so
a fact of more than two arguments is drawn Levi-bipartite, the fact a node with an edge to each argument; a unary or
binary fact is a labelled arrow, and `ein render program --levi` draws every fact the Levi way. `ein` writes DOT
and never rasterises it.

**Why.** A program is one structure with two surfaces, the text to write and the graph to look at, and the meaning is
in the structure. Fixing the structure and not the layout makes a view comparable across runs and versions of
Graphviz while leaving the drawing to the tool that draws.

**Held by.** `ein_render::ir_dot` and `ein_render::kb_dot`; `ein-render/tests/dot_wellformed.rs`, the views of the
corpus programs accepted by `dot`; `ein-render/tests/id_order_invariance.rs`:
`every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, every DOT view's bytes held to a digest.
The shapes: [04_dot_rendering.md](../../kernel/ir/03-ein-lang/04_dot_rendering.md).
