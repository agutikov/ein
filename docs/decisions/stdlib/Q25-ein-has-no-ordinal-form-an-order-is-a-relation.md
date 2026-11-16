# Q25 — Ein has no ordinal form: an order is a relation a program declares

No form of the language says *less than*, *position of* or *between*. An order is a relation a program declares and
states facts of, and what follows from it is derived by rules: its transitive closure by `std.algebra`'s
`transitive`, activated by `(transitive R)`, and any rule of the program's own. A position is a name like any other
([Q-M1e2.24](../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)); where numbers are ordered, the calculator
compares numerals by saturation, `(cmp X Y)` answered by `(cmp-of X Y O)`.

**Why.** Ordinality reduces to the machinery every program already has: a relation, its properties, and the rules
they activate. A form of its own would tie the language to a lattice of positions, which a declared relation serves
as well.

**Held by.** `stdlib/algebra.ein`'s `transitive`, and `tests/stdlib/algebra/21_transitive.ein`, which closes a
chain under it and keeps a cycle's closure irreflexive; `examples/zebra2.ein`, whose houses are names and whose
`right-of` and `next-to` are declared relations.
