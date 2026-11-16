# Q-M1f.364 — A subtraction answers the difference and the shortfall

`(sub X Y)` is answered by `(diff-of X Y D R)`: D is X − Y where X is not the smaller, and `z` otherwise; R is what
X is short by, Y − X, where X is the smaller, and `z` otherwise. One of the two is always `z`, and both are where
the operands are equal. Binary's `bin-diff-of` and `std.math.num9`'s `diff9-of` answer the same pair.

**Why.** The natural numbers have no negative number, and the shortfall is how a subtraction says *less than
nothing, by this much*: every subtraction is answered, and by one fact.

**Held by.** `ein-infer/tests/arith_theory.rs`: `every_small_request_is_right`;
`tests/stdlib/math/decimal-arith/08_sub.ein` and `09_sub_borrow_and_zeros.ein`.
