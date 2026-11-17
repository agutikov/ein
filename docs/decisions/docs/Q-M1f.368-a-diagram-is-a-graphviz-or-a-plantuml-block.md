# Q-M1f.368 — A diagram is a Graphviz or a PlantUML block, never drawn in characters

A diagram on a page of `docs/ein/` is a fenced `dot` block, which Graphviz draws, or a fenced `plantuml` block,
in the page beside the text it illustrates. None is drawn in characters: no boxes of `+--` and `|`, no box-drawing
characters, no arrows of dashes, no chain of names joined by arrows. A graph the engine draws that is too large to
read as source stays a `.dot` file with its `.svg`, in the `diagrams/` folder beside the page, and the page shows
the picture. Every `dot` block is one Graphviz lays out, and every `plantuml` block one PlantUML accepts.

What a program prints is not a diagram. A read-out's table, a grid, a trace's lines are output and are shown as
a block of text, exactly as printed. Where such a block could be read as a drawing, the page says which it is: the
block stands in a region the tests compare with the binary's output, or the line before it is the comment
`<!-- output -->`, which a page writes only before what `ein` printed.

**Why.** A drawing in characters cannot be laid out again, breaks when a label changes length, and reads
differently in every font. A `dot` or a `plantuml` block is source: a reader renders it, edits it and checks it.
Ein's own views are DOT, so a page's diagram and the engine's drawing of the same thing are in one notation.

**Held by.** The pages: the *As a graph* sections of `docs/ein/user/03-syntax-and-semantics/` and the guide's
drawings are `dot` blocks, and the region drawings of `docs/ein/reasoning/02-kb-evolution/` are files under its
`diagrams/`. `ein-render/tests/dot_wellformed.rs` holds the views those blocks quote to Graphviz.
