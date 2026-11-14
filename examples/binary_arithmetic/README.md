# Binary arithmetic

Arithmetic on binary numbers as an Ein theory, computed by saturation alone, and conversion between decimal
and binary both ways. M1f P1f.6, stage
[S1f.6.16](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md).

These are **examples, not a stdlib module**. The theory lives in this directory and the programs import it
file-relative. The plan page,
[`binary_arithmetic.md`](../../plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_arithmetic.md),
has the prototype these were banked from, the counts, the costs and the comparison with decimal.

## What is here

| file | what it is | answer |
|---|---|---|
| [`bits.ein`](bits.ein) | the tables: the full adder `fa2`, the full subtractor `fs2`, comparison `cl2`; 28 rows where decimal has 1 664. Generated | facts, no rule |
| [`bin.ein`](bin.ein) | the theory: a number is `(b2 PREFIX BIT)` and `z` is zero; requests `add2`, `sub2`, `cmp2`, `mul2`, `div2`, answered by `sum-of2`, `diff-of2`, `cmp-of2`, `product-of2`, `quot-of2`; 68 rules and a closing rule, `bin`. Generated from `stdlib/math/decimal-arith.ein` | a module |
| [`conv.ein`](conv.ein) | conversion as requests: `(to-bin X)` answered by `(bin-of X B)`, `(to-dec B)` by `(dec-of B X)`; eleven rules over `std.math.decimal-arith` and `bin`. Written by hand | a module |
| [`first.ein`](first.ein) | 101 + 111, and a subtraction of two named numbers | 1100; 110 − 11 = 11 |
| [`five_operations.ein`](five_operations.ein) | the five operations at decimal's example operands, in binary | 127645, 118511 each way, 56088, 5105 remainder 20, `gt` |
| [`div_by_zero.ein`](div_by_zero.ein) | a division by zero | the flag, then `(false)`: R is empty, the claim `(solve :expect (false))` |
| [`both_bases.ein`](both_bases.ein) | `std.math.decimal-arith` and `bin` imported flat into one program | 123078 + 4567 = 127645 in decimal, 101 + 111 = 1100 in binary |
| [`conversions.ein`](conversions.ein) | five numbers to binary and seven to decimal | 0, 1, 2, 10, 255 and back; 256 and 1000 |
| [`gen_binary.py`](gen_binary.py) | writes `bits.ein` and `bin.ein`, and the copy of `bits.ein` in [`../binary_logic/`](../binary_logic/) | `--check` exits 1 if one differs |

Every program is search-free: no candidate, one state, one saturation. Every claim was computed in Python
and is checked by `ein test`.

## Running one

```sh
ein solve examples/binary_arithmetic/first.ein
ein test  examples/binary_arithmetic
ein solve examples/binary_arithmetic/five_operations.ein --trace   # the steps, each rule's :why
```

## Which variant, and why

The plan page built three ways to have binary numbers and recommends **(b), a theory of its own**: its own
number shape and every name its own. That one is banked. It is the smallest of the three, it stands beside
decimal in one program with a plain import, which conversion needs, and a number says which base it is in.
The two it sets aside, (a) the shipped names over binary tables and (c) one theory whose every request
carries its base, are described on the page with their measurements and are not here.

The page also recommends that the binary rules be generated from `std.math.decimal-arith`, since 52 of the 68 differ from
it only in their names. So `bin.ein` is generated, by [`gen_binary.py`](gen_binary.py), which reads
`stdlib/math/decimal-arith.ein`. **It leaves `std.math.decimal-arith`'s check of malformed numbers and undeclared names out** (M1f
S1f.6.12): those rules name the ten digits and the table `nz`, so they are decimal's, and `bin.ein` is
what it was before them. A malformed binary request, `(add2 (b2 z 7) X)` or a name no `let2` states, is
answered by silence; whether the binary theory gets a check of its own is the page's to ask.
**The generator is run by hand and is not in the gate.** After an edit to
`stdlib/math/decimal-arith.ein`:

```sh
python3 examples/binary_arithmetic/gen_binary.py --check   # exit 1 if a generated file is stale
python3 examples/binary_arithmetic/gen_binary.py           # write them again
```

The conversions are in a module of their own, `conv.ein`, as on the prototype.

**The page's questions stay open.** Banking the recommended variant is the user's instruction for the
examples (2026-11-12, *"bank binary arithmetic and logic prototypes into examples too"*) and not a ruling:
which of the three ways ships, the names (these are the shipped ones with a `2`, placeholders), whether zero
is the same `z` in both bases, whether `std.math.decimal-arith` may be generated in part, and whether the tagged calls
`conv.ein` uses become an interface, are each the user's to decide.

## The sizes

Every corpus program is solved, its answer dumped and reloaded on each commit, and `ein solve` writes the
state as text, which for a circuit grows with its steps. So the operands here are short. The page has the
batches of 250 requests on thirty-bit operands and the conversion of 2^40 + 1; those stay with the prototype
under `build/s1f.6.16/`.
