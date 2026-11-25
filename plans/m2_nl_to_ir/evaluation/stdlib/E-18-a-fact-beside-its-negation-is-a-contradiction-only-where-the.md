# E-18 — A fact beside its negation is a contradiction only where the program imports `std.contradiction`

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** the design: [Q-M1f.557](../../../../docs/decisions/stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)

## What

**A fact beside its negation is a contradiction only where the program imports `std.contradiction`**

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk)

## What it costs the application

a generated program that forgets the import is alive where its author meant it dead
