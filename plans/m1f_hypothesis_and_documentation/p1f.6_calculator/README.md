# P1f.6 — A calculator in Ein: school arithmetic as a theory over digit atoms, computed by saturation alone

**Milestone:** [M1f](../README.md)
**Status:** planned 2026-11-08, on the user's instruction of that day ([§ The instruction](#the-instruction)).
No stage has started. The design proposal (S1f.6.1) runs first and stops to show the user target programs
before any theory is written.
**Depends on:** [P1f.5](../p1f.5_the_no_owner_items/README.md), for the language the theory is written in.
[S1f.5.4](../p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md) makes `0` to `9` plain names, so
`(num9 0 0 0 1 2 3 0 7 8)` is nine names and not nine ints. It lands with S1f.5.5's swap, and T1f.5.4.3 retires
the int kind. [S1f.5.7](../p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md),
[S1f.5.11](../p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md) and
[S1f.5.14](../p1f.5_the_no_owner_items/s1f.5.14_nl_required.md) settle the guards, the keywords and a
required `:why`. S1f.6.1 can run once the swap has landed, since it writes programs in the final grammar. The
theory stages, S1f.6.2 onward, run after P1f.5 closes, so the theory is written once.
**Blocks:** nothing.

## The instruction

The user, 2026-11-08:

> plan a phase p1f.6 - calculator in ein; define a theory that allows making fixed length digital numbers like
> this (num9 0 0 0 1 2 3 0 7 8) == 123078, where single digits are separate atoms used in all theory and define
> operations - sum, mul, sub with remainder and division with reminder, so sub and div produces 2 numbers,
> implement just simple school style summation, multiplication and division, so the program in ein can import
> this and calculate some simple arithmetic, first stage would be design proposal that will show user a couple of
> target example programs; also propose extension with negative numbers (e.g. numbers containing 2 parts -
> positive and negative, that can be simplified until one of them is zero; also propose extension with rational
> numbers in this case containing integer numerator and natural denominator; with overflow detection or width
> extension if it is possible, I'm not sure now but maybe relation could have variadic arguments then can write
> numbers in reverse order e.g. (num 8 7 0 3 2 1) == 123078; additional stage in the end of the phase -
> investigation of what can be done with this theory except simple arithmetic, can it be used for any kind of
> theorems proving, or solving equations, or numbers theory exploration, or anything else.

and, the same day, on how it computes:

> I expect ideally solving simple arithmetic will utilize only saturation with hardcoded summation, subtraction,
> multiplication and divisiono tables like it is done in electronics, so search will not be involved at all

## The principle: a circuit, not a search

The theory computes arithmetic the way hardware does. The digit tables are the gates and the rules are the
wiring:
- **Addition** is a ripple-carry adder: one full-adder table lookup per column, with the carry passed to the next.
- **Subtraction** is a ripple-borrow subtractor.
- **Multiplication** is an array multiplier: digit products from the multiplication table, shifted, then summed
  by the adder.
- **Division** is long division. At each step, a comparator picks the one quotient digit k with
  k × divisor ≤ remainder < (k + 1) × divisor, out of the ten multiples the multiplier computed. A subtractor
  then gives the next remainder.

Every step is a table lookup or a comparison whose outcome the tables decide, so **nothing is guessed**. In
Ein's terms:
- **No search.** The answer is root's fixpoint. On every arithmetic program H is empty, R is {∅} and k_s is 1,
  with no fork and no hypothesis. `ein solve`, `ein saturate` and `ein test` read the same facts.
- **Monotone.** No `absent` and no `not` premise anywhere in the theory: a comparator derives `lt`, `eq` or `gt`
  outright, so selecting a quotient digit needs no negation. The strata diagnostic reports one stratum.
- **Demand-driven.** A program states a request, `(add X Y)`. Rules answer only that request, and requests the
  circuit makes itself (the multiplier asking the adder for its partial sums, a division step asking for the
  next). Nothing enumerates the numbers.
- **Measured as a circuit.** Each operation reports its firings (the circuit's size) and its saturation rounds
  (its depth), so a design is compared the way circuits are.

What breaks the principle is a finding, not a cost to accept. If one of the four operations cannot be computed
without a hypothesis, or H is not empty on an arithmetic program, the stage stops and shows the user.

**Where search does belong** is the phase's last stage. Once arithmetic is a circuit, an *inverse* question,
such as which x makes x + 4567 = 127645, is a puzzle. Ein's search answers puzzles, and S1f.6.10 asks what it
makes of these.

## What this is not

- **Not native numbers.** The user's ruling of 2026-11-06 stands
  ([Q-M1e2.24](../../m1e2_correctness/open_questions.md)): Ein has no numbers, no ranges, and no arithmetic
  predicate or token. This phase writes a theory *in* Ein, over ten digit names, and adds nothing to the engine.
- **Not an engine change, unless a ruling makes one.** The one candidate is variadic relations (S1f.6.9). It
  is a grammar change, so it would need the user's ruling and its own stage, after P1f.5's grammar is final.
- **Not the harness's arithmetic.** A test may hold the theory's answers to Rust's integers; that is the
  oracle for the theory, and it never enters a program.

## The stages

| ID | title | est. | depends on | status |
|---|---|---|---|---|
| [S1f.6.1](s1f.6.1_design_proposal.md) | The design proposal: representations, the request protocol, and target programs computed on a prototype | — | S1f.5.5's swap | run 2026-11-11, [`design.md`](design.md); **stopped for the user** |
| [S1f.6.2](s1f.6.2_digits_and_tables.md) | The digits and the hardcoded tables: full adder, full subtractor, digit product, comparator, short division | — | S1f.6.1 ruled, P1f.5 closed | planned |
| [S1f.6.3](s1f.6.3_addition.md) | Addition: the ripple-carry adder, with overflow | — | S1f.6.2 | planned |
| [S1f.6.4](s1f.6.4_subtraction.md) | Subtraction with remainder: two numbers, and the comparator | — | S1f.6.3 | planned |
| [S1f.6.5](s1f.6.5_multiplication.md) | Multiplication: the array multiplier, with overflow | — | S1f.6.3 | planned |
| [S1f.6.6](s1f.6.6_division.md) | Division with remainder: long division, the quotient digit chosen by the comparator | — | S1f.6.4, S1f.6.5 | planned |
| [S1f.6.7](s1f.6.7_negative_numbers.md) | Extension: negative numbers as a positive and a negative part, simplified until one is zero | — | S1f.6.6 | planned |
| [S1f.6.8](s1f.6.8_rational_numbers.md) | Extension: rational numbers, an integer numerator over a natural denominator | — | S1f.6.7 | planned |
| [S1f.6.9](s1f.6.9_width.md) | Width: overflow detection, width extension, and numbers written in reverse order (variadic or nested) | — | S1f.6.6 | planned; **stops for the user** on the representation |
| [S1f.6.10](s1f.6.10_beyond_arithmetic.md) | Investigation: what else the theory can do, from equations and puzzles to bounded theorem checking and number theory | — | S1f.6.8, S1f.6.9 | planned; a report |
| [S1f.6.20](s1f.6.20_word_problems.md) | An application: elementary-school word problems, the calculator imported among other relations; the words' meaning as rules, and a missing quantity detected and named | — | S1f.6.6, the protocol ruled | planned 2026-11-11; **stops for the user** with examples |

No stage carries an estimate (the user's ruling of 2026-11-08).

## The order

1. **S1f.6.1, the design, first.** The user's rule for any new syntax is real examples first. So it shows
   target programs with their answers computed on a prototype, and asks the questions only the user can rule.
   Nothing under `stdlib/` is written until the user has read it.
2. **S1f.6.2 to S1f.6.6, the four operations, in order.** Each later circuit is built from the earlier ones: the
   multiplier asks the adder; long division asks the multiplier, the comparator and the subtractor.
3. **S1f.6.7 and S1f.6.8, the two extensions.** Each is a module over the natural-number theory and does not
   edit it. Negative numbers come first, because a rational's numerator is one.
4. **S1f.6.9, the width.** It may run beside S1f.6.7 and S1f.6.8 if it changes no file they touch. If the user
   rules a new representation, it is applied to all four operations and both extensions.
5. **S1f.6.10, the investigation, last.**
6. **S1f.6.20, the word problems**, added by the user on 2026-11-11, any time after S1f.6.6: it needs the four
   operations and not the extensions. Its number leaves 11 to 19 unassigned, as the user gave it.

## Findings stay in the phase

What a stage finds in the engine or the language outside its scope is a row of [`findings.md`](findings.md),
not of F25 (the user's ruling of 2026-11-11: F25 was for P1f.7 alone). Each row is shown to the user and is
fixed, ruled or given an owner before the phase closes.

## Acceptance

- **Importable.** A program imports the theory and computes the user's four operations on fixed-width numbers
  written as `(num9 0 0 0 1 2 3 0 7 8)`. Subtraction and division each give two numbers.
- **Search-free, held by a test.** On every arithmetic program in the corpus:
  - H has no candidate, k_s is 1 and R is {∅}, and no fork happens;
  - the answer is root's fixpoint, and it is the same under `ein solve`, `ein saturate` and `ein test`;
  - the theory writes no `absent` and no `not` premise.
- **Right, held by a test.** Every table row, and every answer on a sample of operands, agrees with the
  harness's integers. At a width small enough to exhaust, the test covers every operand pair.
- **Overflow and division by zero are reported as the design rules,** never as a wrong number.
- **The stdlib's own gates hold.** Every rule of the theory is fired by a program under `tests/stdlib/`
  (`stdlib_coverage.rs`), every program there states an `:expect`, and `utils/stdlib_mutants.py` covers the
  new module.
- **The extensions.** Negative and rational numbers each have a module with the same four operations, the same
  two tests, and a normal form.
- **The width.** The user rules the representation question, and the ruled form is built.
- **The investigation** is a page whose every claim carries the command that re-takes it. Any wrong answer it
  finds in the engine is a register row before anything fixes it.
- **Every relation and rule carries a `:why`.** S1f.5.14 makes it required.

## Risks, each with what checks it

- **H is not empty by default.** A relation that a rule derives is a hypothesis candidate unless something
  closes it. Before any theory existed, a prototype of a nested-fact request answered by two rules
  (2026-11-08, `ein saturate --dump` and `ein solve`) stored the right facts. Its `ein solve` met a cube of 21
  candidates and could not count it. S1f.6.1 rules how the theory keeps H empty, and the search-free test holds
  it.
- **Rule width.** A ripple-carry adder over `num9` is one rule with a premise per column, or one rule per column
  over positions. S1f.6.1 measures both on a prototype.
- **The nesting limit.** The generated parser nests to 1 000 (S1f.5.5's T3). A nested little-endian number
  (S1f.6.9) is bounded by it.
- **The 2^128 commitments limit** stops a describe run whose state route cannot count. A search-free program
  never meets it, and the test that holds H empty holds this too.
