# Q-M1f.180 — The names of `std.math.rat` carry the prefix `rat-`

`std.math.rat`'s requests and results are named with `rat-`: `rat-add` answered by `rat-sum-of`, `rat-sub` by
`rat-diff-of`, `rat-mul` by `rat-product-of`, `rat-div` by `rat-quot-of`, `rat-cmp` by `rat-cmp-of`, `rat-neg` by
`rat-neg-of`, `rat-simplify` by `rat-simple-of`. `rat-quot-of` holds one rational after its operands, and no
remainder. The rational is `(rat Z D)`, the entry `rat-arith`, and the signs a tag carries are `pos` and `neg`.

**Why.** `std.math.int`'s scheme, so that a reader who knows one module reads the other. A quotient of rationals is
exact, so it is a relation of another arity under another prefix.

**Held by.** `stdlib/math/rat.ein`; `ein-infer/tests/rat_theory.rs`: `every_pair_of_small_fractions_is_right`.
