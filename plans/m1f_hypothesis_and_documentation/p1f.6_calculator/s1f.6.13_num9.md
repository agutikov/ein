# S1f.6.13 — Fixed-width numbers as a second module, `std.num9`

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's ruling of that day.
**Depends on:** [S1f.6.2](s1f.6.2_digits_and_tables.md) for the tables; [S1f.6.6](s1f.6.6_division.md) for the
conversions to and from `std.arith`'s numbers.

## The ruling

The user, 2026-11-11: *"yes, ship num9 in stdlib, why not"*.

## What it builds

`stdlib/num9.ein`, imported as `std.num9`: the user's own number, `(num9 0 0 0 1 2 3 0 7 8)`, most significant
digit first, nine digits. It is round 1's theory
([design.md § How the prototype is built](design.md#how-the-prototype-is-built)): one rule with a premise per
column for a sum, a difference and a comparison, shift registers over step names for a product and a quotient.
Where it wins is a single request on numbers that fit: one firing for a sum.

**Decisions taken here, each the user's to overrule:**

- **One width ships: nine.** The module is written by a generator that takes the width, as the prototype's
  `gen.py` does, and `--check` in the gate holds the shipped file to it. Another width is the generator run
  with another number, not a shipped module; a second shipped width would double the rules the coverage gate
  asks tests for.
- **Its relations are its own:** `add9`, `sum9-of` and so on, so that a program may import both modules flat.
  The names are the stage's to propose with examples before the module is written.
- **Overflow is a fact,** `(overflow <request>)`, and no number (the design's recommendation; the user ruled
  division by zero and not this). **Division by zero** is as in `std.arith`: the flag, then `(false)` from it.
- **Conversions are requests,** one firing each way, as round 3's mix built them: `(to9 X)` answered by
  `(as9 X W)` or by `(big X)` for a number of ten digits or more, and `(from9 W)` answered in normal form. They
  live in `std.num9`, which imports `std.arith` for the number's shape. No routing between the two theories is
  shipped: round 3's was the slowest design measured.

**Its known cost.** The division's step rule joins its state with the divisor's multiples on the divisor, a
nested fact, so a batch is quadratic: 250 divisions in 193 ms and 4 000 in 8.1 s on the prototype
([design.md § 4](design.md#4-the-table)). Task 3 decides whether the multiples travel in the state, as
`std.arith`'s do.

## Tasks

1. **The generator and the module,** with `--check` in the gate, importing `std.digits` and adding its own step
   counter.
2. **The names,** shown on the design's target programs before the module is written.
3. **The division's batch cost,** measured on the shipped module; the multiples moved into the state if that
   removes the join, and the measure re-taken.
4. **The conversions,** and a program that computes one sum both ways and compares the answers with `cmp`.
5. **The tests:** one program per rule under `tests/stdlib/num9/`; the harness test, exhaustive at width 2 on
   the generator's output and sampled at width 9; overflow on every operation that can overflow, never a
   truncated number; the search-free test.
6. **The page:** when to use which module, with round 3's table as the evidence.

## Acceptance

- The eight target programs of `design.md`'s round 1 answer as that page showed, under the shipped names.
- Overflow and division by zero as stated; the stdlib's gates hold for the module.
- H empty, k_s 1, R {∅} on every program.
