# Q-M1f.228 — The interface of the tagged calls is what their callers write

The decimal arithmetic's interface is six calls and their answers:

- `(add-walk TAG X Y C z)`: X + Y + C, C a carry in of `0` or `1`; answered by `(done TAG S)`;
- `(sub-walk TAG X Y 0 z)`: X − Y; answered by `(done TAG D)`, or by `(short TAG)` where X is the smaller;
- `(cmp-walk TAG X Y eq)`: X against Y; answered by `(order-is TAG O)`, O one of `lt`, `eq` and `gt`;
- `(mul-call TAG X Y)`: answered by `(done TAG P)`;
- `(div-call TAG X Y)`: answered by `(div-done TAG Q R)`;
- `(operand TAG X)`: what X stands for, a number or a name a `let` states; answered by `(operand-is TAG N)`.

In the first five X and Y are numbers, high zeros allowed, and never a name: a call looks nothing up. Every answer
is in normal form. A module over this one may answer `operand` for its own kind of number
([Q-M1f.177](Q-M1f.177-a-module-over-the-decimal-arithmetic-tags-each.md)).

**Why.** It promises what the callers depend on: `std.math.int` subtracts by the walk, `std.math.int` and
`std.math.rat` look every operand up and extend the look-up, and `std.math.convert` doubles a number and adds a bit
in one walk, the bit as the carry in.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`.
