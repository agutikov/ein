# S1g.5.2 — Agreement with the calculator: conversion between unary and decimal, and the two theories held equal

**Phase:** [P1g.5](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.5.1](s1g.5.1_unary_numbers_and_the_axioms.md);
[`std.arith`](../../../stdlib/arith.ein), the decimal calculator
([P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)); and, if it shipped, the binary
module of [S1f.6.16](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md).

## What it asks

Ein now has two theories of the natural numbers written in itself: the calculator's, positional, computed
through tables of digit facts, and Peano's, unary, computed from two equations an operation. They were
written independently and each was held to Rust's integers. The stage holds them **to each other, inside
Ein**: a conversion, and a program that says the two agree. It is the smallest real instance of something
the milestone cares about, one theory used as the specification of another, and the first place where a
program imports both and has to keep their names apart.

## The conversion

Two requests, each a chain of the theories' own operations:

- **decimal to unary:** walk the decimal number from its top digit; at each digit, multiply the unary
  accumulator by ten and add the digit's unary value (ten facts, `(unary-digit 3 (s (s (s z))))`). Horner's
  rule, in unary arithmetic. Or the direct way: count down the decimal number by `std.arith`'s subtraction of
  one, adding a successor each step. Both to be tried; the first uses only S1g.5.1's theory, the second uses
  the calculator as the counter.
- **unary to decimal:** walk the unary number down, adding one to a decimal accumulator with `std.arith`'s
  addition at each successor. n calculator additions for the number n.

Each is tagged or carries a continuation as `std.arith`'s own circuits do, since a conversion calls an
operation and needs its answer: `std.arith`'s tagged entries (`add-walk`, `mul-call` and `div-call`, answered by `done` and `div-done` under
the caller's tag) are what S1f.6.3 to S1f.6.6 left for a later theory to use, and this is such a theory. S1f.6.16's
decimal-to-binary conversion is the model, if it was built.

## The agreement

For an operation ∘ and decimal numbers a and b, one program computes four things and a claim relates them:

```
        a , b  ──── std.arith ────▶  a ∘ b          (decimal)
          │                             │
       to unary                      to unary
          ▼                             ▼
        A , B  ──── Peano ────────▶  A ∘ B   =?   unary(a ∘ b)
```

The square commutes when the two unary results are the same value. In Ein: a rule that derives
`(agree OP A B)` when both routes' answers are present and `eq`, and one that derives `(false)` when they
are present and `neq`, so a disagreement kills root with a core that names the request. A sketch of the two
rules; with one request's two answers written as facts it derived `agree` (one run, release binary at
`8860d616`, 2026-11-11). The routes that produce those facts are the stage, and the relation names are its
to settle:

```lisp
;; sketch: the two routes' unary answers for one request must be one value
(relation by-peano Request Nat      :why "Peano's theory answers {?1} with {?2}")
(relation by-calculator Request Nat :why "the calculator answers {?1} with {?2}, converted")
(relation agree Request             :why "the two theories agree on {?1}")
(rule routes-agree ()
  :match  (and (by-peano ?q ?u) (by-calculator ?q ?u))
  :assert (agree ?q)
  :why    "both routes give {?u} for {?q}")
(rule routes-differ ()
  :match  (and (by-peano ?q ?u) (by-calculator ?q ?v) (neq ?u ?v))
  :assert (false)
  :why    "the two theories disagree on {?q}: {?u} against {?v}")
```

Then, over every pair up to a bound, written as requests by a generator: one program per operation whose
claim is that `agree` holds of every request and root is alive. Addition, multiplication, order against
`cmp`; subtraction with its two numbers against truncated subtraction both ways; division with remainder
against repeated subtraction in unary, if S1g.5.1 built it.

## What is measured

- **The ratio.** The firings of a decimal operation against the unary one, as the numbers grow: the
  calculator's are in the number of digits, Peano's in the number itself. One table, and the size at which
  the unary side stops being runnable. It is the cost of the simplest theory, stated once.
- **The names.** `z` is zero in both theories; `n`, `s` and the request relations are each one theory's.
  What a program that imports both has to write, flat or qualified, and whether anything clashes
  (S1f.6.12's question for the calculator alone, here with a second theory beside it).
- **The join.** `routes-agree` joins two facts on a request, a nested value: E-3's case. The batch cost is
  counted, and if it is quadratic the stage says what it did about it (a tag; one request a program).

## Tasks

1. **T1g.5.2.1 — the two conversions,** each a request, search-free, each held to Rust on every number up
   to a bound.
2. **T1g.5.2.2 — the agreement programs,** generated with a `--check`, one per operation, with claims.
3. **T1g.5.2.3 — a planted disagreement:** one wrong row in a copy of a digit table, or one wrong Peano
   rule; the agreement program must die at root and name the request. Held by a test, since a check that
   cannot fail checks nothing.
4. **T1g.5.2.4 — the ratio table and the names note.**

## Acceptance

- Both conversions are search-free and right against Rust up to the bound.
- For each operation, every request up to the bound agrees, in one saturation, R {∅}, held by `ein test` on
  the generated programs.
- The planted disagreement is caught, with the request in the core.
- The ratio table's numbers each have their command.

## What it may find

- **A disagreement.** Then one of two shipped theories is wrong on a case its own Rust check missed: a
  register row if an engine answer is wrong, a stdlib fix with its test otherwise.
- **That importing two theories flat cannot be done** without a clash. Then the qualified import is what the
  page shows, with what it costs to read, and the finding goes to the list the calculator's S1f.6.12 opened.
