# Q-M1f.429 — No arrow goes to or from an arrow

In every drawing an arrow goes from a node to a node: an object's box, a fact's octagon, a variable's diamond, a
step, a mark. None ends at a folded arrow or starts from one. So a fact that another arrow of the drawing touches is
never folded ([Q-M1f.425](Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md)): a fact a drawn fact points
at ([Q-M1f.395](Q-M1f.395-ein-render-draws-every-object-once.md)), and in a drawing of a derivation a fact the
derivation's arrows touch, a premise, a fact concluded, a fact of the core or of the no-good, is its octagon under
`--fold` too. Every other fact of two or three arrows folds. A trace's slice draws no point for an arrow to meet an
arrow at: a step's slice, every fact of which is the step's premise or conclusion, is drawn under `--fold` as it is
without it.

**Why.** An arrow is an argument of a fact, or a step's reading of a fact, and a folded fact is an arrow. An arrow at
an arrow would make one fact both an arrow and a node, which the graph has no place for: a fact is folded only where
nothing else touches it.

**Held by.** `ein_render::slice`; `ein-cli/tests/render_doors.rs`: `every_door_draws_a_fact_in_both_forms`, a step's
drawings under `--fold` the same as without it, and no point; `ein-render/tests/every_object_once.rs`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#the-trace--ein-solve---trace).
