# Q-M1f.283 — Zero is a relation of no argument, and each closing rule names the entries below it

`std.math.arith` declares `z` as a relation of no argument that no fact is of: a declaration gives `z` one full
name, `std.math.arith.z`, and, its signature empty, no search guesses it. The module's entry is `arith`, and
`std.math.decimal`'s is `decimal`. A closing rule names the entries of the calculator's modules its module imports:
`decimal-arith` names `arith` and `decimal`, `int-arith` names `decimal-arith`, `decimal` and `arith`, `convert`
names `decimal-arith` and `binary-arith`. `std.math.num9`, which imports `std.math.decimal-arith` whole and lists
`n`, `z` and the division by zero's flag from the modules below it, names `std.math.arith`'s rule
`division-by-zero-is-false`.

**Why.** A name has one full name only by a declaration, and a relation of no argument adds no fact and no
candidate.

**Held by.** `ein-infer/tests/arith_theory.rs`, `int_theory.rs`, `rat_theory.rs`, `gcd_theory.rs` and
`num9_theory.rs`: `the_closing_rule_names_every_rule_and_closes_every_relation`; `stdlib/math/arith.ein`.
