# Q-M1f.267 — Binary's names carry the prefix `bin-`, and its numeral is `b2`

Binary's numeral is `(b2 PREFIX BIT)`. Its other names are decimal's with `bin-` before them: the requests and
results, `bin-add` answered by `bin-sum-of` and so on, `bin-let`, the circuit's relations and tags, its own rules,
the flags `bin-malformed` and `bin-undeclared`, and the tables of `std.math.bits`, `bin-fa`, `bin-fs`, `bin-cl` and
`bin-nz`. The entries are `binary` and `binary-arith`, the numeral's flag `not-binary`, and the conversions
`to-bin`, `bin-of`, `to-dec` and `dec-of` ([Q-M1f.323](../Q-M1f.323-conversion-between-the-bases-is-a-module.md)).

**Why.** A word prefix reads as what it is, as `std.math.int`'s `int-` does, and the two bases' names stand in one
program.

**Held by.** `stdlib/math/binary-arith.ein` and `stdlib/math/bits.ein`; `ein-infer/tests/arith_theory.rs`:
`the_tagged_calls_keep_their_shapes`.
