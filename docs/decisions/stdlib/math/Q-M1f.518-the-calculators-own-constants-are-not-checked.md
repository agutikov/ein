# Q-M1f.518 — The calculator's own constants are not checked

A numeral the calculator's own rules write into its own walks is not checked: `(n z 1)`, the one that
`std.math.int`'s Euclidean and floored divisions add to a quotient's size where a remainder is left, through
`add-run`, and `(n z 2)`, by which `std.math.convert` halves a number through `div-run`. A walk checks nothing
([Q-M1f.404](Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)), and no call is written for them, so
neither is walked or labelled.

**Why.** A constant the calculator writes is its own text, a number by construction as a number an operation makes
is, and not a program's; walking it on every division would check the calculator's spelling again at each request.

**Held by.** `stdlib/math/int.ein`: `int-div-euclid-np-more`, `int-div-euclid-nn-more`, `int-div-floor-pn-more` and
`int-div-floor-np-more`; `stdlib/math/convert.ein`: `to-bin-halve`; `ein-infer/tests/arith_theory.rs`:
`the_tagged_calls_keep_their_shapes`, each walk and every caller that starts one.
