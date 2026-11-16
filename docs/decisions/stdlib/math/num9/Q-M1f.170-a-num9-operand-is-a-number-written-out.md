# Q-M1f.170 — A `num9` operand is a number written out

An operand of `std.math.num9` is a number written out, and the module has no named constants. A conversion reads a
number as written: `to9` counts high zeros, and a numeral of more than nine digits, high zeros among them, is
answered `(big X)`; `from9` answers in normal form. Nothing routes a request of one module to the other: a program
that wants both writes the rule that asks.

**Why.** The module's point is one firing a sum; a program that wants a name writes a rule of its own from the name
to the request.

**Held by.** `ein-infer/tests/num9_theory.rs`: `conversions_are_right_at_every_length`;
`tests/stdlib/math/num9/10_to9.ein` and `11_from9.ein`; `examples/arith/num9_both_ways.ein`.
