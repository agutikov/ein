# Q21 — Every form has a fixed DOT shape, and its layout is free

Every form of the program and of the KB has a fixed DOT shape: a view's DOT fixes the graph's structure, and its
layout (positions, ranks, the style choices the shape leaves open) is Graphviz's to choose. A fact is a node of the
graph and DOT has no hyperedge, so a fact of any count is drawn as its node, an octagon with an edge numbered `1` to
its relation's atom and one to each argument; under `--fold`, a fact of three arrows that nothing drawn points at is
one labelled arrow, and a fact of two arrows a loop
([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)). `ein` writes DOT and never rasterises it.

**A drawing of facts asks for layers by default.** A fact's first arrow is written from its relation's atom to the
fact and drawn back, `"rel" -> f [xlabel="1", dir=back]`, so that it still points at the relation and `dot` ranks
the relation above the fact, and the fact's next arrows go down to its arguments; and a drawing of a knowledge base
writes its layers as `rank=same` groups, objects and facts in turn, a sandwich. A knowledge base of facts about
names stands in five layers so: the atom `relation`, with what states a property of a relation; the declarations;
the relations and their labels; the relations' facts; the plain objects. A band of facts is as deep as its facts
nest: the facts nothing points at share its top rank, a fact under a fact's arrow stands a rank below that fact, and
the names under the band's arrows stand in the layer below the whole band. A folded arrow is no node, and ranks as any
arrow does. That is a preference, and not part of the shape: a test reads a view's nodes and arrows, an arrow drawn
back read as it is drawn, and never its positions or its groups.

**Why.** A program is one structure with two surfaces, the text to write and the graph to look at, and the meaning is
in the structure. Fixing the structure and not the layout makes a view comparable across runs and versions of
Graphviz while leaving the drawing to the tool that draws. A layout is a reader's preference and depends on the
names a program uses; layers, each relation above its facts and each fact above its arguments, read as the graph is
defined, objects and facts in turn, where a strip laid out from the arguments up crowds every fact onto one rank.

**Held by.** `ein_render::ir_dot`, `ein_render::kb_dot` and `ein_render::fact_graph`: `first_arrow`, the one way a
first arrow is written, and the layers a drawing of a knowledge base writes; `ein-render/tests/every_object_once.rs`, which reads an arrow drawn back as it is drawn;
`ein-render/tests/dot_wellformed.rs`, the views of the
corpus programs accepted by `dot`; `ein-render/tests/id_order_invariance.rs`:
`every_rendering_reproduces_its_digest_and_none_depends_on_the_id_order`, every DOT view's bytes held to a digest.
The shapes: [12-rendering.md](../../ein/user/04-reference/12-rendering.md).
