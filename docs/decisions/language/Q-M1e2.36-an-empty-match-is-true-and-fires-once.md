# Q-M1e2.36 — An empty `:match` is true and fires once

`()` as a `:match` is the empty conjunction, which is true. A rule whose `:match` is `()` fires once at root, or once
per activator fact, with only its parameters bound; an hrule whose `:match` is `()` proposes its conclusion likewise.
`()` as a conclusion concludes nothing. `(and)` with no conjunct is refused at the parse, since `and` takes
conjuncts and the empty conjunction is written `()`; `(or)` with no disjunct is refused the same way. The refusal's
line: [Q-M1f.45](../diagnostics/Q-M1f.45-the-parse-error-at-and-or-or-carries-a-note.md).

**Why.** A conjunction is true when each of its conjuncts is, so the conjunction of none is true: a rule with no
premise holds unconditionally, the logical reading of what is written. One spelling for it, `()`, keeps the forms a
reader meets to one, and a connective written with nothing to connect is more likely a slip than a meaning.

**Held by.** `ein-infer/tests/rule_semantics.rs`: `an_empty_match_fires_once_or_once_per_activator`;
`ein-cli/tests/cli_semantics.rs`: `an_empty_match_fires_and_an_empty_conclusion_asserts_nothing`, on every
command; `ein-ir/tests/grammar_pages.rs`: `every_row_of_the_grammars_table_is_refused_at_the_parse`, `(and)` and
`(or)` among the rows.
