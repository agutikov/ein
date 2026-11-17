# Q-M1f.368 — A diagram is a file, and a page shows its picture

A diagram on a page of `docs/` is Graphviz or PlantUML, and never drawn in characters: no boxes of `+--` and `|`,
no box-drawing characters, no arrows of dashes, no chain of names joined by arrows.

**Its source is a file**, `NN-name.dot` for Graphviz or `NN-name.puml` for PlantUML, in the folder `diagrams/`
beside the page. `NN` is the page's number, `00` for a `README.md`, and the name says what the diagram shows.
**The picture drawn from it**, `NN-name.svg`, stands beside the source, and the page shows the picture: an image
whose target is `diagrams/NN-name.svg` and whose text is a short true description of what it shows. No page holds
a fenced `dot` or `plantuml` block.

**A picture is drawn from its source and never edited.** Its last line is a comment that carries the SHA-256 of
the source it was drawn from, `<!-- source sha256: … -->`, so a source that changed and was not drawn again is
found by reading the two files, with nothing drawn. Every source has its picture and every picture its source,
each is shown by the page it is numbered for, and a `diagrams/` folder holds nothing else. Every `.dot` is one
Graphviz lays out, and every `.puml` one PlantUML draws.

What a program prints is not a diagram. A read-out's table, a grid, a trace's lines are output and are shown as
a block of text, exactly as printed. Where such a block could be read as a drawing, the page says which it is: the
block stands in a region the tests compare with the binary's output, or the line before it is the comment
`<!-- output -->`, which a page writes only before what `ein` printed.

**Why.** A fenced block is drawn by whatever shows the page, and not everything that shows Markdown draws one, or
draws both kinds, or draws a large one: where it is not drawn the reader is left with source text in place of the
diagram. An image is shown wherever the page is. The source is still a file, which a reader opens, edits and draws
again, so nothing a block gave is lost. A drawing in characters cannot be laid out again, breaks when a label
changes length, and reads differently in every font. Ein's own views are DOT, so a page's diagram and the engine's
drawing of the same thing are in one notation, and a drawing of the engine's is kept as the file the command wrote.
A picture is not compared with a second drawing of its source, because Graphviz lays a graph out differently from
one version to the next; the hash says the same thing and depends on nothing installed.

**Held by.** The folders themselves: under `docs/ein/`, every page's diagrams are the sources and the stamped
pictures of the `diagrams/` folder beside it. `ein-ir/tests/grammar_pages.rs` writes the sources of the grammar's
diagrams and fails where one differs from the grammar files
([Q-M1f.378](Q-M1f.378-the-grammar-page-is-the-two-grammar-files-as-ebnf.md)), and
`ein-render/tests/dot_wellformed.rs` holds the views the engine's drawings are written by to Graphviz.
