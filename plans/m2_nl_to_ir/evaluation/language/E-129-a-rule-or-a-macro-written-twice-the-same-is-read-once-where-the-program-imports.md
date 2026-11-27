# E-129 — A rule or a macro written twice the same is read once where the program imports a module, and refused where it imports none

**Kind:** bug
**Status:** open
**Filed:** by [S2.2e.1](../../p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md), 2026-11-27, on `f281d35a` and again on
`8d1f89ba`
**Owner:** none; a fix leaves Ein's syntax and semantics as they are, so a stage of M2 that needs it may make it
([Q-M2.16](../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md))

## What

**What the imports pass does to a program's own forms where the program imports, and nothing does where it imports
none.** The pass's de-duplication (`ein-ir/src/imports.rs`, `dedup_declarations`) reads the resolved list of a
program that imports, its main file's forms among them; a program that imports nothing (but the module every
program gets) does not pass through it, and its forms reach the loader as written. Since S2.2e.1 the pass leaves a
relation's declarations alone (E-104); of a rule and a macro it still does two things the loader does not:

- **the same rule or macro written twice is read once**, where the loader refuses the second, `duplicate rule name
  'r' at None` or `duplicate macro 'pp' at None`;
- **one written twice with two bodies is refused by another line**, `conflicting definitions of 'r': same name,
  different body (at f.ein:6:7)`, placed at the second one's name, where the loader's is placed `at None`.

So one line, `(import std.algebra)`, which brings nothing the program writes, turns a refused program into one that
loads. A module read as a program and the same module imported differ the same way. No other effect: the pass
drops a fact only where the same node arrives twice, which a file's own forms never are, and a program cannot
declare a name under a module's full name (Q-M1f.239), so two files never meet in it; a module reached by two paths
is one copy before the pass (`emit`).

`07-diagnostics.md` § 2.32 says it as it is since S2.2e.1: the two rows of the loader *in a program that imports
nothing*, and the pass's row *in a module or in a program that imports one; the same rule or macro written twice
there is read once*.

## Evidence

[`../../probes/`](../../p2.2e_kernel_fixes/probes/), each program and the same with `(import std.algebra)` first, `ein solve`:

| the program | no import | with the import |
|---|---|---|
| `rule-twice-the-same.ein` | exit 1, `duplicate rule name 'r' at None` | exit 0 |
| `rule-twice-two-bodies.ein` | exit 1, `duplicate rule name 'r' at None` | exit 1, `conflicting definitions of 'r': same name, different body (at …:6:7)` |
| `macro-twice-the-same.ein` | exit 1, `duplicate macro 'pp' at None` | exit 0 |
| `macro-twice-two-bodies.ein` | exit 1, `duplicate macro 'pp' at None` | exit 1, `conflicting definitions of 'pp': same name, different body (at …:6:8)` |

No corpus program meets either: a build that printed each form the pass dropped or refused, run under `ein render
program` over every entry of `corpus/corpus.toml` on `f281d35a`, printed nothing. The pass was written when a
module came once per import; since a module is one copy per program, what it folds is only a file that writes one
form twice.

## What it cost the application

Nothing yet: no model's program has written a rule twice. A model that does is judged by whether it also imports.

## A fix

Not made here; which one is the fixer's to say. The pass could leave a file's own forms to the loader, as it now
leaves a relation's declarations, so that a program reads alike with an import or without; or it could fold
nowhere, the loader's two refusals then holding of a module too, placed `at None` (a top-level form has no place,
§ 1.5) where the pass's line names the module's file. Whether a module may write one form twice is the question
either way.
