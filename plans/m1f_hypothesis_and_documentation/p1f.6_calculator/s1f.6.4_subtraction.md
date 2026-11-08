# S1f.6.4 — Subtraction with remainder: two numbers, and the comparator

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

`(sub X Y)` answered by two numbers, as the user asked (*"sub with remainder … produces 2 numbers"*), in the
form S1f.6.1's question 3 rules.
- **Under (a)** the pair is `(d r)`: d = X − Y when X ≥ Y, otherwise 0, and r = Y − X when Y > X, otherwise 0.
  One subtractor chain runs on X − Y. If its top borrow-out is 0, the difference is d and r is zero. If it is
  1, a second chain on Y − X gives r, and d is zero. A comparator over the two numbers could pick the
  direction instead; the stage measures both.
- **Under (b)** it gives the difference modulo 10^width and the borrow flag, from one chain.

**The number comparator:** `(compare X Y)` answered by `lt`, `eq` or `gt`. It is computed from the
most significant column down by the digit comparator: the first column that differs decides, and if none does
the answer is `eq`. It needs no `absent`, since each column's comparison is a table fact. Long division
(S1f.6.6) uses the comparator to choose each quotient digit.

## Tasks

1. **The subtractor's rules and the second number,** as ruled.
2. **The comparator's rules.**
3. **The tests:** as for addition (one program per rule, the harness test, the search-free test), plus every
   pair at the exhausting width where X < Y, X = Y and X > Y.
4. **The circuit's measure:** firings and rounds of one subtraction and one comparison.
5. **The example:** 123078 − 4567, and 4567 − 123078, the second number non-zero.

## Acceptance

- Both of S1f.6.1's subtraction programs answer as the page showed, with two numbers each.
- The comparator agrees with the integers on every pair at the exhausting width.
- Search-free, held by the same test.
