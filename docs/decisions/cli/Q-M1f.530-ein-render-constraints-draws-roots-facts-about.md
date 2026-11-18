# Q-M1f.530 — `ein render constraints` draws root's facts about relations

`ein render constraints FILE` draws **the facts about relations**
([Q-M1f.523](Q-M1f.523-the-facts-about-relations-are-every-fact.md)) of the program's root at its fixpoint, by the
KB view's code ([Q-M1f.527](Q-M1f.527-a-slice-is-drawn-with-what-its-facts-point-at.md)): the declarations and the facts
the loader stores beside them, every fact of a property, given or derived, an imported module's among them, and every
other fact root holds with a relation of the program under an argument arrow, each drawn as a fact
([Q-M1f.426](Q-M1f.426-the-constraint-view-draws-each-fact-as-a-fact.md)). It reads the knowledge base and not the
file: neither a fact's `:source` or rule nor the file it is written in decides what it keeps, and the language is
unchanged. A saved file's root is its program's, its choices not committed. A program the loader refuses, or whose
root does not saturate, is refused with the line every command prints for it, exit 1.

**Why.** What the view means to draw, *co-located is symmetric*, *color-loc is a bijection*, is a fact about a
relation, and the graph defines one; a selection by how a fact is written misses what is derived, imported or given a
source, and takes in tables of objects and stored negations. Defined in the engine, the slice needs no form of the
language to say *an atom is a relation of the program*, which no goal can.

**Held by.** `ein_render::constraints::render_constraints` and `ein-cli`'s `render::cmd_constraints`;
`ein-cli/tests/render_doors.rs`: `render_constraints_draws_roots_facts_about_relations`; the shape digests
`dot[constraints]` and `dot[constraints-fold]` of every corpus program, `ein-render/tests/golden/corpus_shapes.md5`.
The page is [rendering](../../ein/user/04-reference/12-rendering.md#constraints--ein-render-constraints).
