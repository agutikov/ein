# Q-M1f.147 — The DOT sweep skips a view of more than 1 000 edges

The sweep that puts every view of every corpus program, and the `ein render h-space` graphs of every program that
searches, to `dot -Tsvg` puts none of more than 1 000 edges to Graphviz at all: such a view is counted and skipped.
The bound is under the smallest digit table, so a view that holds a table is past it and every view that holds none
is laid out. A skip is never silent: the sweep prints how many views it skipped and each with its edges, the
directories that hold a skipped view of a program are exactly the ones its list names, and the skipped graphs of
the hypothesis space exactly the (program, view) pairs its second list names, so a view that starts to be skipped
somewhere new fails the sweep, and so does a name nothing past the bound answers to any more. Nothing checks the
DOT text of a skipped view.

**Why.** The test holds the engine's DOT, not Graphviz: Graphviz is the one authority on whether the grammar and the
attribute values are real, and how it lays out a large graph is its own affair
([Q21](../cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md)). A library of facts is where a layered
layout grows past any budget, and no layout finishes a digit table's view in minutes. The writers of a skipped view
are the functions that write the small ones, which the small views check. A list of directories, unlike an exact
count of skips, moves only where the bound starts to skip a view in a new place, which is where a bound that skips
too much is noticed.

**Held by.** `ein-render/tests/dot_wellformed.rs`: `DOT_LAYOUT_BOUND`, `SKIPPING` and `STRUCTURE_SKIPPING`, through
`graphviz_accepts_every_view_of_every_corpus_file` and
`graphviz_accepts_the_structure_graphs_of_every_program_that_searches`.
