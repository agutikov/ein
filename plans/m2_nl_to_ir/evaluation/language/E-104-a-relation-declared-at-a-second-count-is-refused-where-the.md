# E-104 — A relation declared at a second count is refused where the program imports a module

**Kind:** bug
**Status:** closed by [S2.2e.1](../../p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md) (`8d1f89ba`)
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none; a fix leaves Ein's syntax and semantics as they are, so a stage of M2 that needs it may make it
([Q-M2.16](../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md))
**Since P2.2's close (2026-11-27):** [S2.2e.1](../../p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md); [Q-M2.42](../../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md)

## What

**A relation declared at a second count is refused where the program imports a module**: `(relation likes Person
Drink …)` beside `(relation likes Person …)` loads, as
[Q-M1f.447](../../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md) says it
does (*"A relation may be declared more than once, at one count or at several, and no declaration is refused as a
duplicate of another"*), and the same two declarations in a program with one `(import …)` line are refused:
`kb load error: conflicting definitions of 'likes': same name, different body`. The refusal is the imports pass's
de-duplication of declarations (`ein-ir/src/imports.rs`, `dedup_declarations`, keyed on kind and name and not on
the count), which runs only where a program imports; [`07-diagnostics.md`](../../../../docs/ein/user/04-reference/07-diagnostics.md)'s
row for the line says *a module that declares one name twice, with two bodies*, and a main program is refused by it
too.

## Evidence

[`../zebra_formulations/probes/language/`](../../p2.2_formalizer/zebra_formulations/probes/language/): `ein solve
declared-at-two-counts.ein` exits 0 with the one class `(likes Ann Tea)`; `ein solve
declared-at-two-counts-with-an-import.ein`, the same file and `(import std.algebra)`, exits 1 with the line above.
`ein-cli/tests/declared_counts.rs`, which holds the decision, has no import in its programs.

Confirmed by S2.2.4, 2026-11-26, at one count: the model's `unused`, step 1, declares `(relation has Person Number
Thing …)` and `(relation has Person Number Box …)`, two label lists at one count; step 1's program, with no import,
loads; step 2's, which imports the calculator, is refused, *conflicting definitions of 'has': same name, different
body* ([S2.2.4's record](../../p2.2_formalizer/steps/README.md), `runs/steps/unused/`). The de-duplication is keyed on the name alone,
whatever the count.

## What it costs the application

`examples/zebra3.2.ein` writes `the` at two counts, `(the Red House)` and `(the Coffee (is drunk in) House)`, and
imports `std.slots`: it declares `the` at three arguments only, and its two-argument descriptions are of a relation
no declaration names, which no `:why` reads and EinAf's check would report as undeclared. A formalizer that
declares what it writes, as the contract asks, is refused on a valid program.

## Closed

By S2.2e.1, `8d1f89ba`, 2026-11-27: `dedup_declarations` passes a `relation` form through as it is written, and keeps
its fold and its refusal for a rule and a macro. The fix restores Ein's semantics as Q-M1f.447 states them, two
declarations being two facts of `relation` which different rules may read; no decision is written. The probes answer
alike (`declared-at-two-counts.ein` and `-with-an-import.ein`, exit 0, one class `(likes Ann Tea)`); the model's
`unused` program of both conditions loads (`ein solve --no-search`, exit 0); `examples/zebra3.2.ein` declares `the`
at both counts, its answer unchanged. Held by `ein-cli/tests/declared_counts.rs` (every case with an import and
without, and what reads a declaration reading each) and `ein-ir/tests/imports_semantics.rs`. What the pass still
does to a rule or a macro is [E-129](../../p2.2e_kernel_fixes/evaluation/language/E-129-a-rule-or-a-macro-written-twice-the-same-is-read-once-where-the-program-imports.md);
the user's statement on a program's declaration of a module's relation, which Q-M1f.234 answers otherwise for a
listed name, is [Q-M2.43](../../p2.2g_names_across_an_import/open_questions/kernel/Q-M2.43-is-a-programs-declaration-of-a-modules-relation-a-second-declaration-of-it.md).
