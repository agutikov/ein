# S1f.6.6 — Division with remainder: long division, the quotient digit chosen by the comparator

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.4](s1f.6.4_subtraction.md) and [S1f.6.5](s1f.6.5_multiplication.md).

## What it builds

`(div X Y)` answered by two numbers, the quotient and the remainder, school-style long division:
1. **The ten multiples.** Y × 0 to Y × 9, from S1f.6.5's single-digit row, computed once per division.
2. **One step per digit of X, from the left.** The partial remainder is shifted one column, and X's next digit
   is brought down.
3. **The quotient digit.** It is the one k with Y × k ≤ partial remainder < Y × (k + 1), decided by S1f.6.4's
   comparator against the multiples. Exactly one k satisfies it, so this is a selection the tables make, not a
   guess. Since the comparator derives `lt`, `eq` and `gt` outright, it needs no `absent`.
4. **The next remainder.** The partial remainder minus Y × k, from S1f.6.4's subtractor.
5. **The answer.** The quotient digits in order, and the last remainder.

Each step is a request the previous step's result makes, so the division is a chain of saturation rounds, as
long as X is wide.

**Short division.** S1f.6.2's short-division table divides by a one-digit divisor in one lookup per step. The
stage measures whether it is worth a separate path for a one-digit Y, or whether long division covers it at a
cost the page states.

**Division by zero** is reported as S1f.6.1 ruled. With Y = 0, every multiple is 0 and no k satisfies step 3,
so a separate rule must detect it; the stage adds that rule, so the division never goes silent.

## Tasks

1. **The multiples, the step, the selection and the answer.**
2. **Division by zero,** as ruled.
3. **The tests:**
   - as for addition: one program per rule, the harness test against the integers' quotient and remainder, and
     the search-free test;
   - every pair at the exhausting width, divisor 0 included;
   - X < Y (quotient 0), and X = Y.
4. **The circuit's measure:** firings and rounds of one division.
5. **The example:** 127645 ÷ 25 = 5105 remainder 20, a division by zero, and S1f.6.1's chained program, a
   division whose dividend is a product the same run computes.

## Acceptance

- S1f.6.1's division programs answer as the page showed, with two numbers each.
- The harness test passes, and so does the search-free test.
- A division by zero is reported, and never answered with a number.
