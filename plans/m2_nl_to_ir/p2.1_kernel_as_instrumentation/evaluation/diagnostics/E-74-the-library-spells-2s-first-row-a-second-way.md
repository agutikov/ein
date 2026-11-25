# E-74 — The library spells § 2's first row a second way

**Kind:** bug
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** [F23](../../../../followups/f23_diagnostics_in_eins_words.md)'s D-1; [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), planned

## What

**The library spells § 2's first row a second way**: `ein_ir::load_file` reads the path itself and refuses a missing file with `<path>: No such file or directory (os error 2)`, where every command prints `FileNotFoundError: [Errno 2] …` through `ein_ir::read`; both are `unreadable`

## Evidence

`ein_ir::load_file_with` (`from_ir.rs`); `ein solve nope.ein`

## What it costs the application

a caller of `load_file` and one of the binary see two lines for one refusal; the kind is one
