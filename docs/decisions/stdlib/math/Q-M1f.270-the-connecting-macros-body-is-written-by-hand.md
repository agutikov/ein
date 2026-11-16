# Q-M1f.270 — The connecting macro's body is written by hand

The body of `std.math.positional`'s macro, one activator a shared rule with that rule's parameters in their order,
is written by hand beside the rules it activates, and a test holds that it names each rule of the module once, with
that rule's parameters.

**Why.** The module stays one file a reader can follow, and the test fails on the drift a generated body would
prevent.

**Held by.** `ein-infer/tests/arith_theory.rs`: `one_call_of_the_macro_connects_each_base`.
