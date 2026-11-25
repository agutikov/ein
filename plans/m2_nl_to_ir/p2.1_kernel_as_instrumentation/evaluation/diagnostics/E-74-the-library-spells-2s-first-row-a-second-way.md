# E-74 — The library spells § 2's first row a second way

**Kind:** bug
**Status:** closed by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), `df75643c`: `ein_ir::load_file` and `ein test` read a path through `ein_ir::read`, so a missing file is one line, the page's, from the library and every command (`program_read.rs`' `every_command_and_the_library_refuse_a_missing_file_with_one_line`); which words that line has is [Q-M2.21](../../open_questions/kernel/Q-M2.21-a-missing-files-line-the-pages-python-spelling-or-eins-own.md), the user's
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** [S2.1.4](../../s2.1.4_bugs_the_phase_found.md); the words, [F23](../../../../followups/f23_diagnostics_in_eins_words.md)'s D-1

## What

**The library spells § 2's first row a second way**: `ein_ir::load_file` reads the path itself and refuses a missing file with `<path>: No such file or directory (os error 2)`, where every command prints `FileNotFoundError: [Errno 2] …` through `ein_ir::read`; both are `unreadable`

## Evidence

`ein_ir::load_file_with` (`from_ir.rs`); `ein solve nope.ein`

## What it costs the application

a caller of `load_file` and one of the binary see two lines for one refusal; the kind is one
