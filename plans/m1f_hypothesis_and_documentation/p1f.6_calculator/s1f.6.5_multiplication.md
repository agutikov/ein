# S1f.6.5 — Multiplication: the array multiplier, with overflow

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

`(mul X Y)` answered by the product, school-style:
- **One row per digit of Y.** X times that digit, column by column: each column one digit-product lookup, its
  high digit carried into the next column by the full adder. The row is X × the digit, one column wider.
- **Shifted.** Row i is moved i columns to the left.
- **Summed.** The rows are added by S1f.6.3's adder. The multiplier states each addition as a request the adder
  answers, so the two circuits are wired by saturation.

This is an array multiplier: the rows are its partial products and the adder its summing tree. Whether the
rows are summed in a chain or in a tree is the stage's to measure, as rounds, the circuit's depth.

**The width:** a product of two w-digit numbers has up to 2w digits. Under S1f.6.1's question 4 (a) a product
that does not fit is an overflow. Under (b) it is a high and a low half. S1f.6.9 takes up width extension.

## Tasks

1. **The row's rules, the shift and the request to the adder.**
2. **Overflow,** as ruled.
3. **The tests:** as for addition, plus every pair at the exhausting width and a product of zero. Also a
   product whose rows are all zero but one.
4. **The circuit's measure:** firings and rounds of one multiplication, and how they grow with the width.
5. **The example:** 123 × 456 = 56088, and an overflowing product.

## Acceptance

- S1f.6.1's multiplication programs answer as the page showed.
- The harness test passes, and so does the search-free test.
- No product is ever a wrong number: it is right, or an overflow as ruled.
