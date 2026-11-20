# Q-M1f.87 — The loader's refusals of what an obligation owes start where the `(open …)` is written

A rule that concludes `(open R)` or `(open ?R)`, `R` a relation's name or `?R` a parameter, is an obligation: what
it owes is read off its own `:match` ([Q-M1f.573](../hypotheses/Q-M1f.573-under-open-a-relation-is-owed-and-a-fact.md)).
The grammar takes the form, and the loader refuses five shapes of it, each with a line that starts with where the
`(open …)` is written, `<file>:<line>:<col>: rule '<n>': …`:

- `(open R)` beside another conclusion
  ([Q-M1f.575](../language/Q-M1f.575-open-r-is-a-rules-whole-assert-and-open-f-stands.md));
- no `(absent …)` of the `:match` holds a positive `R` premise;
- two or more do;
- the `R` premise binds no variable of its own;
- two positive `R` premises each bind one.

**Why.** Each of the five is read off the rule's own `:match` and `:assert`, and the place of the `(open …)` in the
file says where to look. What the grammar refuses never reaches the loader
([Q-M1f.88](Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)).

**Held by.** `validate_open` in `ein_ir::from_ir`; `ein-ir`'s
`from_ir::tests::the_verdict_atom_refuses_every_shape_it_cannot_resolve`; `ein-ir/tests/grammar_pages.rs`:
`the_lines_the_grammar_does_not_cover_are_still_reached`, `every_deleted_line_is_a_row_of_the_table`;
`examples/broken/load/open_compound_witness.expected` and `open_mixed_assert.expected`.
