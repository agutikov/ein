# E-104 — A relation declared at a second count is refused where the program imports a module

**Kind:** bug
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; a fix leaves Ein's syntax and semantics as they are, so a stage of M2 that needs it may make it
([Q-M2.16](../../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md))

## What

**A relation declared at a second count is refused where the program imports a module**: `(relation likes Person
Drink …)` beside `(relation likes Person …)` loads, as
[Q-M1f.447](../../../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md) says it
does (*"A relation may be declared more than once, at one count or at several, and no declaration is refused as a
duplicate of another"*), and the same two declarations in a program with one `(import …)` line are refused:
`kb load error: conflicting definitions of 'likes': same name, different body`. The refusal is the imports pass's
de-duplication of declarations (`ein-ir/src/imports.rs`, `dedup_declarations`, keyed on kind and name and not on
the count), which runs only where a program imports; [`07-diagnostics.md`](../../../../../docs/ein/user/04-reference/07-diagnostics.md)'s
row for the line says *a module that declares one name twice, with two bodies*, and a main program is refused by it
too.

## Evidence

[`../zebra_formulations/probes/language/`](../../zebra_formulations/probes/language/): `ein solve
declared-at-two-counts.ein` exits 0 with the one class `(likes Ann Tea)`; `ein solve
declared-at-two-counts-with-an-import.ein`, the same file and `(import std.algebra)`, exits 1 with the line above.
`ein-cli/tests/declared_counts.rs`, which holds the decision, has no import in its programs.

Confirmed by S2.2.4, 2026-11-26, at one count: the model's `unused`, step 1, declares `(relation has Person Number
Thing …)` and `(relation has Person Number Box …)`, two label lists at one count; step 1's program, with no import,
loads; step 2's, which imports the calculator, is refused, *conflicting definitions of 'has': same name, different
body* ([S2.2.4's record](../../steps/README.md), `runs/steps/unused/`). The de-duplication is keyed on the name alone,
whatever the count.

## What it costs the application

`examples/zebra3.2.ein` writes `the` at two counts, `(the Red House)` and `(the Coffee (is drunk in) House)`, and
imports `std.slots`: it declares `the` at three arguments only, and its two-argument descriptions are of a relation
no declaration names, which no `:why` reads and EinAf's check would report as undeclared. A formalizer that
declares what it writes, as the contract asks, is refused on a valid program.
