# Q-M1f.200 — An integer division's result is named after its convention

`(int-div-euclid X Y)` is answered by `(int-quot-euclid-of X Y Q R)`, `(int-div-trunc X Y)` by
`(int-quot-trunc-of X Y Q R)` and `(int-div-floor X Y)` by `(int-quot-floor-of X Y Q R)`, the operands repeated as
written, after the decimal arithmetic's `quot-of`.

**Why.** Each convention's remainder has a shape of its own, the Euclidean a natural number and the other two
integers, so each result is a relation of its own.

**Held by.** `stdlib/math/int.ein`; `tests/stdlib/math/int/09_div_euclid.ein`, `10_div_trunc.ein` and
`11_div_floor.ein`.
