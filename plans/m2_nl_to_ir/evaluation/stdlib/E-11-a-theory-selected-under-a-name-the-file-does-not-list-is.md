# E-11 — A theory selected under a name the file does not list is inert, with no message

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A theory selected under a name the file does not list is inert, with no message.** `(transitive left-of)` selects the library's rule only where the file lists `transitive` in `:symbols` from the module that declares it (or writes its full name, or an alias); otherwise the fact is the file's own. `(bijective R)` also needs its two hierarchy activators

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk), [audit of P2.2](../../analysis/audit_p2.2.md)

Confirmed by S2.2.4, 2026-11-26, on a relation where it was a rule: the model's word problems write the calculator's
`sum-of` beside `(import std.math.decimal-arith :symbols (add product-of))` (`gap`, `gap_fact`, `kinds`, `kinds_full`
under the steps' grammars; `gap_fact`, `boxes12`, `boxes13`, `pencils` in one call): the program's `sum-of` is its
own, the calculator's answer is `std.math.decimal-arith.sum-of`, and the answer query has no row. The contract's check
finds it (`name-not-listed`, [S2.2.4's record](../../p2.2_formalizer/steps/README.md)); the kernel loads it in silence.

## What it costs the application

the commonest mistake a model can make in selecting a theory produces a program that loads, runs and answers something else
