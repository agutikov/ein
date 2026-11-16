# Q-M1f.174 — The names of `std.math.int` carry the prefix `int-`

`std.math.int`'s requests and results are named with `int-`: `int-add` answered by `int-sum-of`, `int-sub` by
`int-diff-of`, `int-mul` by `int-product-of`, `int-cmp` by `int-cmp-of`, `int-neg` by `int-neg-of`, `int-simplify`
by `int-simple-of`, and the divisions of [Q-M1f.179](Q-M1f.179-integer-division-names-its-convention.md). The
integer is `(int P N)`, and the module's entry `int-arith`.

**Why.** The names read as what they are, and none is a name of the decimal arithmetic or of `std.math.num9`, whose
requests a program may write beside these.

**Held by.** `stdlib/math/int.ein`; `ein-infer/tests/int_theory.rs`: `every_pair_of_small_parts_is_right`.
