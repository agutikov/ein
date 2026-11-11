# S1f.6.6 — Division with remainder, and division by zero

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08; re-planned 2026-11-11 to [the rulings](design.md#the-rulings-2026-11-11).
**Depends on:** [S1f.6.4](s1f.6.4_subtraction.md), [S1f.6.5](s1f.6.5_multiplication.md).

## What it builds

`(div X Y)` answered by `(quot-of X Y Q R)`, school long division:

1. **The divisor in normal form, and its multiples by 2 to 9,** each a row of S1f.6.5, computed once per
   division in a chain whose tags hold the multiples already known. They then travel in the state as one fact.
2. **The dividend turned round once,** so its top digit comes first.
3. **One step per dividend digit.** The digit is brought down under the remainder, which is one constructor.
4. **The quotient digit by bisection.** It lies between 0 and 10; the middle multiple is subtracted by
   S1f.6.4's walk; *it fits* or *it borrows* halves the range, through the `mid` and `adj` tables. At most four
   subtractions a digit. Nothing is guessed: each trial is a subtraction whose outcome the tables decide.
5. **The answer** when no digit is left.

**Division by zero, in two steps** (the user: *"I expected the flag derived from division by zero and then flag
cause (false), the cost is ok, program contains division by zero, why we would want other queries results?"*).
- A divisor whose normal form is `z` derives `(division-by-zero (div X Y))`, by one rule.
- A second rule derives `(false)` from any `(division-by-zero ?request)`.

So the trace shows the flag as a step of its own and the contradiction as the next, the core is the request,
and `refuted by` names the second rule. The costs are accepted as ruled: R is empty, every other query of the
program loses its answer, and only `(false)` can be claimed
([design.md § E](design.md#e-division-by-zero-as-false-question-5)). The stage shows every surface again on the
shipped theory.

## Tasks

1. **The multiples, the turn, the step, the bisection and the answer,** under the rule of the theory.
2. **Division by zero,** the two rules, and its surfaces: `ein solve`, `--trace`, `--dump-answer`,
   `ein saturate --dump`, `ein test` on `(solve :expect (false))`.
3. **The tests,** as for addition, with: X < Y, X = Y, a divisor of one digit and of many, a zero divisor
   written `z` and written with high zeros, and long operands.
4. **The circuit's measure:** firings and depth of one division, and the batch of S1f.6.3's test.
5. **The example:** 127645 ÷ 25, a division by zero, and the chained program of `design.md`: a rule of the
   program's own from `product-of` to a `div` request.

## Acceptance

- 127645 ÷ 25 is 5105 remainder 20.
- A division by zero derives the flag and then `(false)`, each by its own rule, and never a number.
- The three tests pass.
