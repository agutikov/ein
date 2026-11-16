# Q-M1f.320 — `std.math.num9` is a second representation, and flags no malformed digit

`std.math.num9` ships beside the decimal arithmetic as a second representation of the natural numbers, nine digits
in one fact, computed by wide rules, a premise a column, with programs of its own under `tests/stdlib/`. It flags
no malformed digit: a place of `(num9 …)` that holds no digit is answered by silence. A wrong count of digits is
refused when the program is loaded. The module is generated, and not edited by hand.

**Why.** It is kept as the record of another way to represent numbers; the decimal arithmetic is the one the rest
of the calculator is built on.

**Held by.** `stdlib/math/num9.ein`; `ein-infer/tests/num9_theory.rs`: `width_two_is_right_on_every_pair`,
`the_theory_writes_no_negation`; `tests/stdlib/math/num9/`.
