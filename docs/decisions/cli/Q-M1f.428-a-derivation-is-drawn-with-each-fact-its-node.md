# Q-M1f.428 — A derivation is drawn with each fact its node and each step a node, at three levels of emphasis

The drawings of a derivation, a trace's slices, the derivation graph and a written trace's `dag` view, draw each
fact as its node in the canonical form ([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)) and
each step as a node of its own, from its premises to what it concludes: in a slice a firing's node labelled with its
rule and its `:why`, in the derivation graph a small diamond for every derivation, in a written trace's view a
step's node; no node is filled. A refutation's `⊥` is a mark that the core's facts point at, and the learned
no-good a note that points at each of its facts. **The derivation stands out, at three levels**, each step in a hue
of a fixed palette picked by its rule's name, in two shades:

1. **the fact a step adds**, its octagon and its arrows, is the brightest: the hue's bright shade, at three times
   Graphviz's default width, `penwidth=3`;
2. **the derivation's explanation**, each step's border and label and its arrows, from a premise to the step and from
   the step to what it concludes, is the same hue's deeper shade, at twice the default width, `penwidth=2`;
3. **everything else** is Graphviz's default black at the default width.

No shade is near black, and each reads on white. None is red, which a commitment's facts and the refutation's arrows
are, from the core to `⊥` and the no-good's, nor grey, which a stored negation no step added is. A redundant firing
adds nothing, so what it concludes is drawn as the rest; the derivation graph, which records no rule, draws every
derivation in the palette's first hue, and a fact as added where a derivation concludes it and its first record is a
rule's.

**Why.** A derivation's arrows cross the facts' own arrows, which point at relations and arguments; drawn thin and in
the facts' colours they could not be told apart. Wide and in its rule's hue, the derivation reads as one path through
the graph of its facts, and the fact it adds, brightest, as where the path ends. Black is the colour of what is not
emphasised, so nothing emphasised is black or near it.

**Held by.** `ein_render::slice` (`DERIVATION_PALETTE`, `DERIVATION_WIDTH`, `DERIVED_WIDTH`),
`ein_render::derivation` and `ein_render::ir_dot`; the goldens `ein-render/tests/golden/dot/slice_render_slice.dot`,
`kb_provenance_dag.dot` and `ir_render_trace_dag.dot`, and the trace goldens; `ein-render/tests/presentation_semantics.rs`,
whose `a_derivation_is_drawn_at_three_levels` holds the levels and `no_shade_of_the_derivation_palette_is_near_black`
the palette's contrast against black and white. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#the-trace--ein-solve---trace).
