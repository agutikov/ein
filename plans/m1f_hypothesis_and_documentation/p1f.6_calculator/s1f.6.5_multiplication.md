# S1f.6.5 — Multiplication

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.3](s1f.6.3_addition.md).

## What it builds

`(mul X Y)` answered by `(product-of X Y P)`, the school method from the multiplier's top digit:

1. **The multiplier is turned round once,** so its most significant digit comes first.
2. **A row per multiplier digit:** X times one digit, a walk over the `md` table.
3. **The sum so far times ten, plus the row.** Times ten is one constructor in the public shape, `(n ACC 0)`,
   and the addition is S1f.6.3's walk, called with a tag that holds the multiplier's remaining digits.
4. **The product** when no digit is left. A product has no width to overflow.

Every call carries its tag, so the rule of the theory holds: the rule that goes on after a row reads its whole
state out of `(done (TAG …) ROW)`.

## Tasks

1. **The row, the step and the answer.**
2. **The tests,** as for addition: one program per rule, the harness test (zero on either side, a multiplier
   with inner zeros, operands with high zeros, long operands), the search-free test and the test of the rule
   of the theory.
3. **The circuit's measure:** firings and depth of one multiplication, and how they grow with the two lengths.
4. **The example:** 123 × 456, and 123078 × 123078, which round 1's `num9` could not hold.

## Acceptance

- 123 × 456 is 56088 and 123078 × 123078 is 15148194084.
- The three tests pass.
