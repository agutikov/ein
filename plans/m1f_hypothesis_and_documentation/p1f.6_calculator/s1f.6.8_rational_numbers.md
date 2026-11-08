# S1f.6.8 — Extension: rational numbers, an integer numerator over a natural denominator

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.7](s1f.6.7_negative_numbers.md).

## What it builds

The user's proposal: rational numbers *"containing integer numerator and natural denominator"*. A rational is
`(rat Z D)`: Z is an integer in S1f.6.7's form and D a non-zero natural number. Its normal form has the
numerator and denominator coprime, and zero written as 0 over 1.

**Simplifying needs the gcd, and Euclid's algorithm is a chain of divisions.** gcd(a, b) is gcd(b, a mod b)
until the remainder is zero. Each step is a request that the previous step's remainder makes, answered by
S1f.6.6's division, so the algorithm runs as saturation rounds. It terminates because the remainders strictly
decrease. Dividing both parts by the gcd is two more divisions.

**The operations, each followed by simplification:**
- **Addition:** (Z1·D2 + Z2·D1) / (D1·D2).
- **Subtraction:** add the negation.
- **Multiplication:** (Z1·Z2) / (D1·D2).
- **Division,** by a non-zero rational: (Z1·D2) / (D1·|Z2|), with Z2's sign moved to the numerator.
- **Comparison:** compare Z1·D2 with Z2·D1. Each denominator is positive, so the order is the order of the
  cross products.

A zero denominator, written by a program or reached by dividing by a zero rational, is reported, as
S1f.6.1 ruled for division by zero.

**The width matters most here.** Cross products need twice the operands' width, and a sum of them can need one
more digit. Under S1f.6.1's question 4 (a) they overflow early. The stage states where, and S1f.6.9's
width extension is what lifts the limit.

## Tasks

1. **The gcd chain and the normal form.**
2. **The operations and comparison.**
3. **The tests:**
   - against the harness's exact fractions;
   - every operand pair at the exhausting width;
   - an unsimplified operand, such as 2/4, which must answer as 1/2.
4. **The example:** a short computation over rationals, such as 1/2 + 1/3 = 5/6.

## Acceptance

- Every answer is in normal form and agrees with exact fractions.
- Search-free, held by the same test.
- The natural-number and integer theories are unchanged.
- Where overflow limits the extension, it says so with the command that shows it.
