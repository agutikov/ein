# Q-M1e.25 — Both Zebra encodings are held to the published grid

`examples/zebra.ein`, one generic `co-located` relation, and `examples/zebra2.ein`, five projections, a relation for
each kind of attribute, are each held to the Zebra puzzle's published answer, all 25 cells read through each
encoding's own vocabulary, and so to each other. `zebra.ein`'s answer is held as *unique*: `--single` decides that R
is that one state. `zebra2.ein`'s model fills the same 25 cells, and its uniqueness is held by a decided `--single`
too.

**Why.** A digest of a program's own output can pin a wrong model and stay green. Asserting the published answer
is what makes these two more than self-goldens, and holding both encodings to one grid is what shows they differ in
their ontology and not in their answer.

**Held by.** `ein-infer/tests/acceptance.rs`: `the_generic_link_encoding_is_the_unique_solution`,
`both_ontologies_reach_the_same_model`, `the_typed_encoding_is_unique_and_the_search_that_says_so_was_exhausted`.
