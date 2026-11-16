# Q-M1f.189 — The decimal arithmetic's tagged calls are an interface

Beside its public requests, `std.math.decimal-arith` offers tagged calls: a rule asks under a tag of its own, a
nested fact that names the caller, and the answer is handed back under the same tag, so the rule that reads it has
one premise. These calls are an interface, as the requests are: what a theory over the decimal arithmetic calls,
and what a rule of a program's own may call. Which calls they are is
[Q-M1f.228](Q-M1f.228-the-interface-of-the-tagged-calls-is-what-their.md).

**Why.** A result repeats its request's operands, so a rule that wants an answer for a purpose of its own would
have to find it by a number, a join the engine answers by a scan
([Q-M1f.362](Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)); under a tag it finds it by one premise.
`std.math.int`, `std.math.rat`, `std.math.gcd` and `std.math.convert` are built on them.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`, which holds the declarations
and every caller under `stdlib/`, `examples/` and `tests/` to their shapes;
`examples/beyond_arith/casting_out_nines.ein`, a program whose own rules call them.
