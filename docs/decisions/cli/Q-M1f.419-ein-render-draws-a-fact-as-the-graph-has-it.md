# Q-M1f.419 — `ein render` draws a fact as the graph has it, and folds it on request

Every view of `ein render`, and every drawing `ein solve --trace` writes, draws a fact in the **canonical form**:
an octagon with no name in it, its first arrow, numbered `1`, at its relation's atom, and an arrow numbered `2`,
`3`, … at each argument in turn. The relation is a node labelled with its name, one node for every fact of it in the
drawing, and every atom is a node, whatever relation points at it
([Q-M1f.421](Q-M1f.421-no-view-draws-a-relation-otherwise-than-another.md)). It is the graph as the knowledge base
is defined ([Q-M1f.379](../language/Q-M1f.379-an-object-is-a-node-of-the-knowledge-base-graph.md),
[Q-M1f.381](../language/Q-M1f.381-a-relation-is-the-atom-under-a-facts-first-arrow.md)).

**On request**, `--fold` on each view that draws facts (`ein render program`, `kb`, `rules`, `rule`,
`constraints`) and on `ein solve`, for the trace's drawings, a fact of three arrows, a relation and two arguments,
that no drawn fact points at is **folded** into one arrow from its first argument to its second, and a fact of two
arrows into a loop on its argument, each labelled with its relation's name
([Q-M1f.425](Q-M1f.425-fold-folds-a-fact-of-three-arrows-into-an-arrow.md),
[Q-M1f.382](../language/Q-M1f.382-a-labelled-arrow-is-a-fact-of-three-arrows.md)). Nothing else is folded: a fact a
drawn fact points at is a node, and in a drawing of a derivation so is a fact the derivation's arrows touch, since no
arrow goes to or from an arrow ([Q-M1f.429](Q-M1f.429-no-arrow-goes-to-or-from-an-arrow.md)).

**No view draws an octagon with a name in it, and no node labelled with a fact's text**: the drawings of a
derivation (the trace's slices, the derivation graph, a written trace's `dag` view) draw each fact as its node and
each step as a node of its own, from its premises to what it concludes. What a fact rests on, a source's short
text or `by <rule>`, stands beside its octagon. The KB and the program views write a relation's name as it is,
`owns`, and number arrows with digits alone. No page under `docs/` draws either form.

**Why.** A drawing that writes the relation's name in a fact's node, or the fact's text in a node, draws a relation
as a word and a fact as a string, where the graph has one atom under every first arrow and one node for every fact.
The canonical form is the graph itself, so a reader of a view reads the knowledge base the pages define; the fold
is the compact picture a reader of many facts of two arguments wants, asked for and never the default.

**Held by.** `ein-render/tests/every_object_once.rs`: every view of every corpus program keeps the rules in the form
it is drawn in, and each writer on its fixtures in both forms; `ein-cli/tests/render_doors.rs`:
`every_door_draws_a_fact_in_both_forms`, through every door and the trace; `ein-render/src/fact_graph.rs`, the one
drawing of facts the KB, program and constraint views share. The page is
[rendering](../../ein/user/04-reference/12-rendering.md).
