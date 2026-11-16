# Q-M1f.116 — An activator's argument at a parameter that names no relation is not checked

An `:hrules` activator's argument at a parameter that names a relation is checked against the program's relations
([Q-M1f.114](Q-M1f.114-a-relation-a-solve-list-names-is-one-the-program.md)). At any other parameter it is a value,
a type or an object, and the loader does not check it: `(guess (at Colour House))`, `Colour` misspelt, loads, the
hrule matches nothing for that entry and proposes nothing, and the read-out's `universe` row says there is no
candidate.

**Why.** A value is any name, and an activator may be the first place a program names one: an activator such as
`(guess (Ann S1))` gives the hrule objects nothing else writes. A rule that refused a value named nowhere else
would refuse those programs with the misspellings.

**Held by.** `ein-cli/tests/solve_lists.rs`: `a_list_naming_what_the_program_has_loads`, whose
`a-value-that-is-no-relation` program, `(solve :hrules (guess (at Colour House)))`, loads on every command.
