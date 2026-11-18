# Q-M1f.428 — A derivation is drawn with each fact its node and each step a node, its arrows standing out

The drawings of a derivation, a trace's slices, the derivation graph and a written trace's `dag` view, draw each
fact as its node in the canonical form ([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)) and
each step as a node of its own, from its premises to what it concludes: in a slice a firing's node labelled with its
rule and its `:why`, in the derivation graph a small diamond for every derivation, in a written trace's view a
step's node. A commitment's facts are red, a stored negation grey, a fact derived bold; no node is filled. A
refutation's `⊥` is a mark that the core's facts point at, and the learned no-good a note that points at each of its
facts. **The derivation stands out**: each of its arrows, from a premise to a step, from a step to what it
concludes, from the core to `⊥`, and each step's border is twice Graphviz's default width, `penwidth=2`, in a colour
of a fixed palette picked by the rule's name, dark enough to read against the facts' arrows, and never red, the
commitment's and the core's, nor grey, a negation's.

**Why.** A derivation's arrows cross the facts' own arrows, which point at relations and arguments; drawn thin and in
the facts' pale colours they could not be told apart. Wide and dark, the derivation reads as one path through the
graph of its facts.

**Held by.** `ein_render::slice` (`DERIVATION_PALETTE`, `DERIVATION_WIDTH`), `ein_render::derivation` and
`ein_render::ir_dot`; the goldens `ein-render/tests/golden/dot/slice_render_slice.dot`, `kb_provenance_dag.dot` and
`ir_render_trace_dag.dot`, and the trace goldens; `ein-render/tests/presentation_semantics.rs`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#the-trace--ein-solve---trace).
