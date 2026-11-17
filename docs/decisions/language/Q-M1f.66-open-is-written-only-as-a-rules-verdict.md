# Q-M1f.66 — `open` is written only as a rule's verdict

`open` is a reserved word, never a name. It is written in one place: as the whole `:assert` of a rule, `(open)` or
`(open ?R)`, the verdict an obligation states. Everywhere else it is a parse error at the `open`, on every command:
as a fact's, a nested fact's or a pattern's relation, and as an argument, `(p open)`, a declaration's label, an
activator's argument, a name in a `(solve …)` list, an import, a trace step or the verdict's own argument. The error
carries a note saying where the word is written
([Q-M1f.88](../diagnostics/Q-M1f.88-a-misplaced-open-is-a-parse-error-with-a-note.md)).

**Why.** A word the engine reads as a verdict in one position and as a name in every other lets a program state,
unawares, a fact or an argument that an obligation's reader could mistake for the verdict. Reserved in the plain
sense, it means one thing wherever it is read.

**Held by.** `ein-ir/src/ein.y`'s `Assert`, the one production that takes `open`, and
`ein_ir::grammar::RESERVED_WORDS`; `ein-cli/tests/open_reserved.rs`:
`open_is_refused_wherever_it_is_not_the_verdict`, every place on every command, and
`the_verdict_loads_where_a_rule_concludes_it`; `ein-ir/tests/grammar_pages.rs`, a row of the grammar's table per
place.
