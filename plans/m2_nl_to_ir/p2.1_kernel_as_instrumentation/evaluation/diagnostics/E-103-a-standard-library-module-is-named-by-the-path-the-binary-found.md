# E-103 — A standard library module is named by the path the binary found it under

**Kind:** weakness
**Status:** open
**Filed:** by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), 2026-11-25, on `df75643c`
**Owner:** none

## What

**A standard library module is named by the path the binary found it under**: since [E-101](E-101-a-modules-path-in-a-message-is-absolute-the-programs-as.md) a file-relative module is named as the program names it, and a `std.*` module under the library's root as the binary found it: `$EIN_STDLIB` as set, a checkout found by walking up from the executable, which is absolute, or `<embedded>`. No program gives that path, and it differs between a checkout and an installed binary

## Evidence

`ein solve examples/broken/load/import_module_not_found.ein`, from any directory: `kb load error: (import std.nope) — module not found at /home/…/ein/stdlib/nope.ein (None)`; the same program under an installed binary names `<embedded>/nope.ein`

## What it costs the application

a model that imports a module of the library that is not there is told the host's directory, and the line moves with where the binary runs
