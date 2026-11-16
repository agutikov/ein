# Q-M1e2.22 — A bare `_` is reserved and refused

Ein has no wildcard. A bare `_`, and `?_`, are refused at the parse wherever either is written, a fact, a pattern,
a guard, a goal and every other place, with the note *`_` is reserved and names nothing: where any value will do,
write a variable used once*. Inside a name or a variable `_` is an ordinary character: `_a_in_b_`, `_______01`,
`A________B` and `?_x` are names and a variable.

**Why.** A variable used once answers every question a wildcard would, and binds nothing another clause reads. A
bare `_` read as an ordinary name would make `(holds _)` a pattern that never fires, in silence; reserved, it can
mean nothing by accident.

**Held by.** The lexer, `ein-ir/src/ein.l` (`UNDERSCORE`, `VARUNDERSCORE`, tokens no production takes) and the
note, `ein-ir/src/grammar.rs`. `ein-cli/tests/cli_semantics.rs`: `a_bare_underscore_names_nothing_in_every_place`,
on every command that reads a program; the grammar's table, `ein-ir/tests/grammar_pages.rs`, for every other place.
