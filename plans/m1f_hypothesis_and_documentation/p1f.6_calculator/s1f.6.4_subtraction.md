# S1f.6.4 — Subtraction with its two numbers, and comparison

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

**Subtraction** (the user's ruling on question 3, (a)). `(sub X Y)` is answered by `(diff-of X Y D R)`: D is
X − Y when X is not the smaller and `z` otherwise, and R is Y − X when Y is the larger and `z` otherwise. It is
the normal form S1f.6.7's negative numbers want. The circuit subtracts both ways with S1f.6.3's walk over the
`fs` table; the walk that ends without a borrow hands its answer over, and the one that ends with a borrow
derives `(short TAG)` and no number.

**Comparison** (the user: *"what we forgot from school arithmetic is at least numbers comparison"*).
`(cmp X Y)` is answered by `(cmp-of X Y lt)`, `eq` or `gt`: one walk from the units over the `cl` table, with no
answer to build. A number that has run out counts as zeros, so the comparison is by value and holds across
spellings with high zeros.

`le`, `ge`, `max` and `min` are not built unless a later stage needs one; each is a rule over `cmp-of`.

## Tasks

1. **The two walks of subtraction, and the walk of comparison,** each under S1f.6.3's rule of the theory.
2. **The tests,** as for addition: one program per rule, the harness test against the integers (X > Y, X < Y,
   X = Y, operands with high zeros, zero on either side), the search-free test, and S1f.6.3's test of the rule
   of the theory extended to these rules.
3. **The circuit's measure:** firings and depth of one subtraction and one comparison.
4. **The example:** 123078 − 4567, 4567 − 123078, and a comparison.

## Acceptance

- 123078 − 4567 is (118511, z) and 4567 − 123078 is (z, 118511).
- Every comparison agrees with the integers, and `(cmp 5 5)` written with and without high zeros is `eq`.
- The three tests pass.
