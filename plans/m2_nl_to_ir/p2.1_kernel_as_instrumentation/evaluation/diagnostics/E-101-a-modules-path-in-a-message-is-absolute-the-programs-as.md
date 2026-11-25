# E-101 — A module's path in a message is absolute, the program's as given

**Kind:** weakness
**Status:** closed by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), `df75643c` (EinAf's part `3e801779`): a module is named by the program's directory as given joined with its name, in a message, a place and the summary, and EinAf's comparison no longer strips the repository's root (`ein-cli/tests/module_paths.rs`; [Q-M1f.93](../../../../../docs/decisions/diagnostics/Q-M1f.93-a-modules-read-line-names-the-path-its-import.md)); a `std.*` module's path is [E-103](E-103-a-standard-library-module-is-named-by-the-path-the-binary-found.md)
**Filed:** by [S2.1.3](../../s2.1.3_feedback_object.md), 2026-11-25, on `0ab1aef0`
**Owner:** [S2.1.4](../../s2.1.4_bugs_the_phase_found.md)

## What

**A module's path in a message is absolute, the program's as given**: an import cycle, a module not found and a module's parse error name the module by the path the resolver made, `/home/…/examples/broken/load/import_cycle_b.ein`, beside the program's own relative name

## Evidence

`ein solve examples/broken/load/import_cycle.ein --json-summary F` from the repository's root: the finding names both files absolute; `ein solve examples/broken/load/reserved_open_symbols.ein`: the parse error's `place.file` absolute

## What it costs the application

a message rendered to a model carries the host's directory; a comparison of two runs from two directories differs
