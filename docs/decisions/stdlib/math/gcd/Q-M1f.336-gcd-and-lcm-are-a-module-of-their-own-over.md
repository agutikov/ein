# Q-M1f.336 — Gcd and lcm are a module of their own, over natural numbers

The greatest common divisor and the least common multiple are `std.math.gcd`: `(gcd X Y)` answered by
`(gcd-of X Y G)`, by Euclid's chain over the decimal arithmetic's tagged division, and `(lcm X Y)` by
`(lcm-of X Y L)`, the product divided by it. Its operands are natural numbers, written out or named by a `let`; an
integer or a rational is no operand, and such a request is answered by silence. The module's entry is `gcd`, which
is also the request's name.

**Why.** The decimal arithmetic stays the five operations of school arithmetic, and a theory that needs a common
divisor, `std.math.rat`'s lowest terms, imports the module that computes it.

**Held by.** `ein-infer/tests/gcd_theory.rs`: `every_pair_to_a_bound_is_right`, `what_is_not_an_operand`;
`tests/stdlib/math/gcd/01_gcd.ein` and `05_lcm.ein`.
