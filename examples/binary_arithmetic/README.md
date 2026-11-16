# Binary arithmetic

Arithmetic on binary numbers, computed by saturation alone, and conversion between decimal and binary both
ways: programs over the stdlib's [`std.math.binary-arith`](../../stdlib/math/binary-arith.ein) and
[`std.math.convert`](../../stdlib/math/convert.ein). M1f P1f.6, stage
[S1f.6.16](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md).

The theory was banked here as an example on 2026-11-12, the plan page's way (b) generated from decimal's
module, and became the stdlib's at the stage's build (2026-11-15): the rules both bases share are written
once, in [`std.math.positional`](../../stdlib/math/positional.ein), and each base connects them by one call.
The plan page, [`stdlib.md`](../../docs/ein/user/reference/stdlib.md#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith),
has the prototype, the counts, the costs, the comparison with decimal and what each of the user's rulings
made of it; [`stdlib/README.md`](../../stdlib/README.md#binary-stdmathbits-stdmathbinary-and-stdmathbinary-arith-m1f-s1f616)
has the modules.

## What is here

| file | what it is | answer |
|---|---|---|
| [`first.ein`](first.ein) | 101 + 111, and a subtraction of two named numbers, `bin-let` | 1100; 110 − 11 = 11 |
| [`five_operations.ein`](five_operations.ein) | the five operations at decimal's example operands, in binary | 127645, 118511 each way, 56088, 5105 remainder 20, `gt` |
| [`div_by_zero.ein`](div_by_zero.ein) | a division by zero | `std.math.arith`'s flag, then `(false)`: R is empty, the claim `(solve :expect (false))` |
| [`both_bases.ein`](both_bases.ein) | `std.math.decimal-arith` and `std.math.binary-arith` in one program | 123078 + 4567 = 127645 in decimal, 101 + 111 = 1100 in binary |
| [`conversions.ein`](conversions.ein) | five numbers to binary and seven to decimal, by `std.math.convert` | 0, 1, 2, 10, 255 and back; 256 and 1000 |

Every program is search-free: no candidate, one state, one saturation. Every claim was computed in Python
and is checked by `ein test`; `ein-cli/tests/arith_search_free.rs` holds each program search-free, and
`ein-infer/tests/binary_theory.rs` holds the theory itself to Rust's integers.

## Running one

```sh
ein solve examples/binary_arithmetic/first.ein
ein test  examples/binary_arithmetic
ein solve examples/binary_arithmetic/five_operations.ein --trace   # the steps, each rule's :why
```

A step of a shared rule prints by `std.math.positional`'s name, `std.math.positional.add-column`, whichever
base it walked.

## The names

The requests are decimal's with `bin-` before each, `bin-add` answered by `bin-sum-of`, and so on, and a
number is `(b2 PREFIX BIT)` of `std.math.binary`, its shape saying its base (P1f.6's replies 13 and 14;
Q-M1f.267, defaulted). Until the build the example theory named them `add2`, `sum-of2`, `let2`: the claims
here are the ones banked then, every request renamed and no answer moved.

## The sizes

Every corpus program is solved, its answer dumped and reloaded on each commit, and `ein solve` writes the
state as text, which for a circuit grows with its steps. So the operands here are short. The batches of
requests on thirty-bit operands are `ein-infer/tests/arith_batch.rs`'s, run by hand, and the conversion of
2^40 + 1 is `binary_theory.rs`'.
