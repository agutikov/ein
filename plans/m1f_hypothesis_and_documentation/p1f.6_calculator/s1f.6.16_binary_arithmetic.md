# S1f.6.16 — Arithmetic with binary numbers

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. **T1 done 2026-11-11, as a prototype and a
page shown to the user: [`binary_arithmetic.md`](binary_arithmetic.md)**, the three ways each tried and measured,
with a recommendation of (b). Nothing shipped; T2 to T4 wait for the user's ruling on that page's questions.
**Depends on:** [S1f.6.6](s1f.6.6_division.md), the four operations in decimal, which it is compared with.

## The instruction

The user, 2026-11-11: *"new more stages: arithmetic with binary numbers, I suspect simpler rules"*.

(On 2026-11-11 the user also said *"skip different bases"*, of hexadecimal and base 64 as a way to make decimal
arithmetic faster. This stage is not that: binary is asked for as the simplest arithmetic, and as the foot of
[S1f.6.17](s1f.6.17_binary_logic.md)'s logic.)

## The question

Whether the rules get simpler, and what exactly does. The coordinator's expectation, to be measured and not
assumed:

- **The tables shrink, a lot.** A full adder is 8 rows where decimal's is 200; a digit product with a carry is
  4 where decimal's is 900; *not zero* is one row; the comparison table is 12.
- **The wiring may not change at all.** The circuits of [`std.arith`](../../../stdlib/arith.ein) walk a
  number a column at a time and look each column up; nothing in a walk says *ten*. If so, binary arithmetic is
  the same rules over other tables, and the stage's finding is that the theory is **generic in its base**,
  which says where decimal lives: in the tables and nowhere else.
- **Division does get simpler.** A quotient digit is 0 or 1, so one comparison decides it, where decimal
  bisects ten multiples: the bisection tables and their rules are not needed.
- **Numbers get longer**, about 3.3 times, so the firings go up as the tables go down.

## Tasks

1. **T1, on a prototype under `build/s1f.6.16/`, and shown to the user before anything ships:** the same
   targets as decimal's, in binary, each with its answer computed; the rule and row counts side by side; the
   firings and the times side by side; and the three ways to have binary numbers, each tried:
   - **(a)** the same shape and the same relations, `(n PREFIX DIGIT)` over the digits `0` and `1`, with
     binary tables in a module of their own: then a program picks its base by which tables it imports, and a
     number does not say which base it is in;
   - **(b)** a shape of its own, e.g. `(b2 PREFIX BIT)`, and rules of its own, so the two can be imported
     together and never confused;
   - **(c)** one generic theory whose every request carries its base.
   The stage recommends one. The user's rule for new syntax is real examples first.
2. **The module, its tables and its programs,** as ruled: generated tables with `--check`, a program per
   rule, the search-free test, the rule of the theory held (no rule joins two facts on a number), every
   answer held to Rust's integers.
3. **Conversion between decimal and binary,** as requests, each a chain of the theory's own operations
   (repeated division by two one way, multiply and add the other). It is the first use of the calculator
   by a theory, before the word problems.
4. **The comparison page:** what got simpler, what did not, and the cost.

## Acceptance

- The prototype's page, with the three options and a recommendation, and the user's ruling.
- Binary numbers add, subtract, multiply, divide and compare, search-free and right against the integers.
- The page says, with numbers, which rules binary made simpler and which it left as they were.
