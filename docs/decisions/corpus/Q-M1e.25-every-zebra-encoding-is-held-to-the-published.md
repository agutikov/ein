# Q-M1e.25 — Every Zebra encoding is held to the published grid

`examples/zebra.ein`, one generic `co-located` relation, `examples/zebra2.ein`, five projections, a relation for
each kind of attribute, and the four encodings whose statements are the puzzle's sentences in the text's own words,
`examples/zebra3.1.ein` (one relation a verb), `zebra3.2.ein` (descriptions, and what is the same), `zebra3.3.ein`
(an adjective of a house a relation of one argument) and `zebra3.4.ein` (the verb as an argument), are each held
to the Zebra puzzle's published answer, all 25 cells read through each encoding's own vocabulary, and so to each
other. Each answer is held as *unique*: a decided `--single` says that R is that one state. Where an encoding asks
the questions in its statements' own relations, the answer is held in them too: the Norwegian drinks water and the
Japanese owns the zebra, as `(drinks Norwegian Water)` and `(owns Japanese Zebra)`.

**Why.** A digest of a program's own output can pin a wrong model and stay green. Asserting the published answer
is what makes these encodings more than self-goldens, and holding all of them to one grid is what shows they differ
in their ontology and their theory and not in their answer.

**Held by.** `ein-infer/tests/acceptance.rs`: `the_generic_link_encoding_is_the_unique_solution`,
`both_ontologies_reach_the_same_model`, `the_typed_encoding_is_unique_and_the_search_that_says_so_was_exhausted`,
`one_relation_a_verb_is_the_unique_solution`, `descriptions_and_what_is_the_same_are_the_unique_solution`,
`an_adjective_a_relation_of_one_argument_is_the_unique_solution`, `the_verb_as_an_argument_is_the_unique_solution`.
