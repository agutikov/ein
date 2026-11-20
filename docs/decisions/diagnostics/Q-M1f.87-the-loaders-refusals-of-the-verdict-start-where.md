# Q-M1f.87 — The loader's refusals of the verdict start where the `(open …)` is written

`open` stands only as a rule's whole `:assert`, `(open)` or `(open ?R)`; anywhere else it is a parse error
([Q-M1f.88](Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)). Of a verdict the grammar takes, the loader
refuses five shapes, each with a line that starts with where the `(open …)` is written,
`<file>:<line>:<col>: rule '<n>': …`:

- `(open ?R)` whose `?R` is not a parameter;
- no `(absent …)` of the `:match` holds a positive `?R` premise;
- two or more do;
- the `?R` premise binds no variable of its own;
- two positive `?R` premises each bind one.

**Why.** Each of the five is read off the rule's own `:match`, and the verdict's place in the file says where to
look. What the grammar refuses never reaches the loader, which holds as an invariant that no `(open …)` stands but
a rule's whole `:assert` (`validate_open` in `ein_ir::from_ir`).

**Held by.** `ein-ir`'s `from_ir::tests::the_verdict_atom_refuses_every_shape_it_cannot_resolve`;
`ein-ir/tests/grammar_pages.rs`: `the_lines_the_grammar_does_not_cover_are_still_reached`,
`every_deleted_line_is_a_row_of_the_table`; `examples/broken/load/open_compound_witness.expected`.
