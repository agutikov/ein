# E-11 — A theory selected under a name the file does not list is inert, with no message

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A theory selected under a name the file does not list is inert, with no message.** `(transitive left-of)` selects the library's rule only where the file lists `transitive` in `:symbols` from the module that declares it (or writes its full name, or an alias); otherwise the fact is the file's own. `(bijective R)` also needs its two hierarchy activators

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk), [audit of P2.2](../../analysis/audit_p2.2.md)

## What it costs the application

the commonest mistake a model can make in selecting a theory produces a program that loads, runs and answers something else
