# Q-M1f.483 — A word on an arrow that ranks nothing is the arrow's `xlabel`

An arrow that sets no rank, `constraint=false`, carries its word as its `xlabel` and never as its `label`, in every
view: the region view's `sat`, from a commitment to its closure, and `✗ alive`, an unsound exclusion; a guard's sign
on its link in a rule's panel; and a folded arrow in a drawing of a knowledge base, which runs within its arguments'
layer ([Q21](Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md)). An arrow whose word is a `label` ranks as any
arrow does: a folded arrow in a rule's panel and in a trace's slice keeps its `label` and its ranking, and so do the
region view's repair edge, `+fact`, and the `lines` view's `+k`.

**Why.** Graphviz's spline router can find no room for a `label` on a link that ranks nothing, and refuses the
drawing, as it refused a rule's `≠` link. An `xlabel` is placed once the graph is laid out and takes no part in the
layout ([Q-M1f.435](Q-M1f.435-an-arrows-number-is-written-beside-it.md)). A rule's panel and a slice are small, and
there a folded arrow ranked with its `label` routes.

**Held by.** `ein_render::region_dot`, `ein_render::rules`, `ein_render::fact_graph` and `ein_render::slice`;
`ein-cli/tests/region_render.rs`: `every_view_is_drawn_and_auto_picks_the_measured_one`, every region view put to
`dot`; `ein-render/tests/dot_wellformed.rs`, every view of the corpus programs and every slice of two calculator
programs' traces put to `dot`. The page: [rendering](../../ein/user/04-reference/12-rendering.md#edges), the region
view's edges.
