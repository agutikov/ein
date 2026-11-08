# S1f.6.7 — Extension: negative numbers as a positive and a negative part, simplified until one is zero

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.6](s1f.6.6_division.md).

## What it builds

The user's proposal: *"numbers containing 2 parts - positive and negative, that can be simplified until one of
them is zero"*. An integer is a pair of natural numbers `(int P N)` whose value is P − N. Simplifying a pair is
S1f.6.4's subtraction of N from P, whose two numbers under S1f.6.1's question 3 (a) are already the simplified
pair. So the normal form has one part zero, zero itself is `(int 0 0)`, and every integer has exactly one normal
form.

Each operation works on the parts with the natural-number circuits, then simplifies:
- **Addition:** (P1 + P2, N1 + N2).
- **Negation:** (N, P), the parts swapped.
- **Subtraction:** add the negation.
- **Multiplication:** (P1·P2 + N1·N2, P1·N2 + N1·P2). On simplified operands, only the sign rules remain.
- **Division:** on simplified operands, the magnitudes are divided by S1f.6.6 and the signs set by a rule.
  Which remainder convention it uses is the stage's question to the user:
  - **truncated:** the remainder has the dividend's sign;
  - **floored:** the remainder has the divisor's sign;
  - **Euclidean:** 0 ≤ remainder < |divisor|.

  The recommendation is Euclidean, the only one whose remainder is always a natural number.

It is a module over the natural-number theory, which it does not edit, and it is search-free as the theory is.

## Tasks

1. **The pair, its simplification and the four operations,** with negation and comparison.
2. **The remainder convention,** put to the user with the recommendation, before division is written.
3. **The tests:**
   - as for the natural numbers, against the harness's signed integers;
   - every sign combination at the exhausting width;
   - an unsimplified operand, such as `(int 5 3)`, which must give the same answers as `(int 2 0)`.
4. **The example:** S1f.6.1's programs with negative operands.

## Acceptance

- Every answer is a simplified pair and agrees with the signed integers.
- Search-free, held by the same test.
- The natural-number theory is unchanged.
