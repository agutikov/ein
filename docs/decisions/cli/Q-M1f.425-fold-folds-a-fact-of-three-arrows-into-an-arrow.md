# Q-M1f.425 — `--fold` folds a fact of three arrows into an arrow, and a fact of two into a loop

`--fold` is one option on every command that draws facts: `ein render program`, `kb`, `rules`, `rule` and
`constraints`, and `ein solve` for the trace's drawings. Under it a fact that no drawn fact points at is **folded**
where it has three arrows or two: of three, a relation and two arguments, it is one arrow from its first argument to
its second; of two, a relation and one argument, a loop on its argument; either labelled with its relation's name,
and its relation is then no node for that fact. A fact of one arrow, or of four or more, stays its octagon, and so
does a fact a drawn fact points at ([Q-M1f.395](Q-M1f.395-ein-render-draws-every-object-once.md)) and, in a drawing
of a derivation, a fact the derivation's arrows touch
([Q-M1f.429](Q-M1f.429-no-arrow-goes-to-or-from-an-arrow.md)). Without the option every fact is its octagon
([Q-M1f.419](Q-M1f.419-ein-render-draws-a-fact-as-the-graph-has-it.md)).

**Why.** A labelled arrow is the compact picture of a fact of two arguments, and a labelled loop the compact
picture of a fact of one, a property of an object or of a relation, `(symmetric co-located)` or the `(relation r)`
the loader adds beside a declaration. Both take the relation's node out of the drawing the same way, and a drawing
that folded the one and not the other would leave the facts of one argument as the octagons that crowd a folded
picture most. One option for every view keeps the form the same wherever facts are drawn.

**Held by.** `ein_render::fact_graph::folds`, which the KB, program and constraint views, a rule's panels and a
trace's slices fold by; `ein-render/tests/every_object_once.rs`, which fails on a fact of two or three arrows that
nothing points at and is not folded, in every folded view of every corpus program;
`ein-cli/tests/render_doors.rs`: `every_door_draws_a_fact_in_both_forms`. The page is
[rendering](../../ein/user/04-reference/12-rendering.md#a-fact-as-the-graph-has-it--the-canonical-form-and-the-fold).
