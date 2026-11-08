# S1f.6.3 — Addition: the ripple-carry adder, with overflow

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08.
**Depends on:** [S1f.6.2](s1f.6.2_digits_and_tables.md).

## What it builds

`(add X Y)` answered by the sum, column by column from the right. Each column is one full-adder lookup, and its
carry-out is the next column's carry-in, exactly as a ripple-carry adder wires its full adders. A carry out of
the top column is an overflow, reported as S1f.6.1 ruled.

This stage writes the circuit in the form S1f.6.1 measured and recommended: one rule with a premise per column,
or one rule per column over position names. The adder is the circuit the others call: the multiplier sums its
partial products here, and a division step's subtraction is its sibling.

## Tasks

1. **The adder's rules.** A carry chain with no `absent` and no `not`.
2. **Overflow:** the top column's carry-out as the ruled fact.
3. **The tests.**
   - One program per rule under `tests/stdlib/arith/`, each with its `:expect`.
   - A harness test against the integers: a sample of operand pairs at the full width, every pair at a width
     small enough to exhaust, and every pair that overflows reported as overflowing.
   - **The search-free test**, which every later operation joins: H empty, k_s 1, R {∅}, no fork, and the same
     answer under `ein solve`, `ein saturate` and `ein test`.
4. **The circuit's measure:** the firings and the saturation rounds of one addition, with the command that
   re-takes them. A cost is recorded, never estimated.
5. **The example:** `examples/arith/` gains S1f.6.1's addition program, with its corpus entry.

## Acceptance

- `123078 + 4567 = 127645` and the rest of S1f.6.1's addition programs, with their answers as the page showed.
- The harness test passes, and so does the search-free test.
- Overflow is reported as ruled, and never as a wrong sum.
