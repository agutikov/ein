# Q-M1f.116 — An activator's argument is not checked against the program's relations

An activator's argument at a parameter that names a relation names one by being written there: the atom is a relation
of the program ([Q-M1f.522](Q-M1f.522-a-relation-of-the-program-is-any-atom-it-writes.md)), and a rule that concludes a
hypothesis of it proposes its facts, `(guess att Color House)` the facts of `att`. At any other parameter an argument
is an object, any atom or a fact, and is not checked either: `(guess at Colour House)`, `Colour` misspelt, loads, the
rule matches nothing for it and proposes nothing, and the read-out's `universe` row says what H holds.

**Why.** An argument may be any atom, and an activator may be the first place a program writes one: `(guess Ann S1)`
gives a rule atoms nothing else writes. A rule that refused an atom written nowhere else would refuse those programs
with the misspellings; a relation is one the program writes, wherever it writes it.

**Held by.** `ein_ir::from_ir`'s `relations_written`, which takes what a fact passes where a relation's name is held;
`ein_infer::hypothesis_rules`.
