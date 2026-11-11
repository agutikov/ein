# P1g.5 — Natural numbers: Peano's axioms, recursion, induction, and Gödel made concrete

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s rulings; `std.arith`, the decimal calculator
of [P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md), which S1g.5.2 holds the unary
numbers to; for S1g.5.3, [S1g.4.4](../p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md)'s proof checker.
**Pulls in:** [F26](../../followups/f26_consistency_and_completeness.md)'s § *Does Gödel's theorem say
anything about an Ein program?* and its promotion 5;
[S1f.6.10](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md)'s item 4,
bounded theorem checking, which this phase continues from where that stage says *what it cannot do: quantify
over unbounded numbers, or state an induction schema*.

## The instruction's part

> Natural numbers, Peano, induction.

## What it is

The calculator computes with numbers as people write them, digit by digit, through tables. Peano's
arithmetic is the other end: one constant, one constructor, and everything else defined by recursion and
proved by induction. The phase writes it in Ein and asks, at each step, which of three things Ein is doing:

- **computing:** a ground equation, `2 + 3 = 5`, derived by saturation from the recursion equations. Expected
  possible today, search-free, on the calculator's request protocol; probe 6 computed one
  ([M1g § First probes](../README.md#first-probes-2026-11-11)).
- **checking up to a bound:** a universal statement, verified on every instance below a bound. What
  S1f.6.10 does for the decimal theory.
- **proving:** a universal statement for *all* numbers. This is induction, and induction is not a fact a
  state can hold: it is a rule of proof over **formulas with a variable**, so it lives at the object level,
  in [P1g.3](../p1g.3_proof_systems/README.md)'s and [P1g.4](../p1g.4_first_order_logic/README.md)'s proof
  checker. The phase's central stage, [S1g.5.3](s1g.5.3_induction.md), finds out how far that goes.

The axioms themselves split the same way, which is the phase's first observation to test: zero is a number
and a successor is a number are *rules that build terms* (and the second, run forward, does not stop: probe
3); zero is no successor and the successor is injective are **true of nested facts already**, because a
nested fact is a free constructor and `eq` is identity of values; the recursion equations are rules; and
induction is a schema.

And it makes F26's Gödel section concrete as far as a program can: what the theory decides, what it leaves
open, where a search does not end, and what self-reference would need.

## The stages

| ID | title | depends on | status |
|---|---|---|---|
| [S1g.5.1](s1g.5.1_unary_numbers_and_the_axioms.md) | Unary numbers and the axioms as rules: `z`, `(s N)`, addition, multiplication and order by recursion, on request | P1g.1 | planned |
| [S1g.5.2](s1g.5.2_agreement_with_the_calculator.md) | Agreement with the calculator: conversion between unary and decimal, and the two theories held equal | S1g.5.1; `std.arith` | planned |
| [S1g.5.3](s1g.5.3_induction.md) | Induction: a schema over formulas against checking up to a bound; one theorem proved for all numbers, checked by Ein | S1g.5.1; S1g.4.4 | planned; **stops for the user** |
| [S1g.5.4](s1g.5.4_goedel_made_concrete.md) | Gödel made concrete: what the theory decides, a search that does not end, the liar, and what self-reference needs | S1g.5.1 to .3; S1g.4.5 | planned; a report |

## Acceptance

- **Peano's recursion computes,** search-free, held by the search-free test: addition, multiplication and
  order on unary numbers, each answer exactly once.
- **Right, against Rust's integers,** on every pair up to a bound, and **against `std.arith`,** on the same
  pairs through the conversion: the two theories written in Ein agree with each other, not only each with
  Rust.
- **Each axiom has a program** that shows what it is in Ein: a rule, a property of nested facts, or a schema.
- **One universal theorem** (the plan's candidate: `0 + n = n`, then commutativity of addition) **has an
  induction proof written as data and checked by Ein,** or the page names the step that could not be
  checked and why; and the same theorem's bounded check beside it, with what each licenses one to say.
- **The Gödel page:** every claim either shown by a program or marked as the textbook's, not Ein's.
- No program in the phase runs without a bound that fails loudly, until S1g.6.1's word exists.

## Risks

- **Unary is exponentially longer than decimal.** A number n is n nested constructors, and the parser nests
  to 1 000 (E-4 of the calculator's
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)): a program writes
  numbers below a thousand, and multiplication's cost is in the product. The bounds are small and stated.
- **Induction needs most of P1g.4.** If substitution or the quantifier rules slip, S1g.5.3 falls back to an
  induction rule for quantifier-free equations only (primitive recursive arithmetic's shape), which needs
  substitution of a term for one variable and no binder. The fallback is planned, not improvised.
- **Overclaiming.** *Ein proved commutativity* would mean: Ein checked a proof a person or the harness wrote.
  [S1g.1.2](../p1g.1_survey_and_design/s1g.1.2_two_levels.md)'s vocabulary applies, and the page keeps
  to it.
