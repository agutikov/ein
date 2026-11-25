# Q-M1f.546 — A page's drawing of a derivation is drawn at the three levels a view draws

A drawing of a derivation that a page makes by hand is drawn by the rule `ein` draws a derivation by
([Q-M1f.428](../cli/Q-M1f.428-a-derivation-is-drawn-with-each-fact-its-node.md)): each step in a hue of the view's
palette, the fact it adds in the hue's bright shade at `penwidth=3`, the step, its label and its arrows in the deeper
shade at `penwidth=2`, and everything else Graphviz's default black. Where the page's derivation is one the engine
draws, its steps take the hues the engine gives their rules. A given fact that a derivation also concludes is no fact
the derivation added, and is drawn as a given one, in no hue. The page's own marks stand beside the three levels: a
terminal's double border and a core's red ([Q-M1f.469](Q-M1f.469-a-derivations-drawing-on-a-page-marks-its.md)).

**Why.** A reader who has seen a trace's drawing reads a page's by the same levels; a second convention for the
pages would make one picture mean two things.

**Held by.** The drawings of a derivation on [saturation](../../ein/reasoning/02-kb-evolution/02-saturation.md) and
[cores](../../ein/reasoning/02-kb-evolution/09-cores.md), the sources `02-derivation.dot`,
`09-core-in-derivation.dot`, `09-cyclic.dot` and `09-proof-graph.dot` in that folder's `diagrams/`, the cyclic
drawing's given `r(a, b)`, which its alternative derivation concludes, in no hue; the palette they take,
`ein_render::slice::DERIVATION_PALETTE`.
