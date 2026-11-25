# E-96 — The clock's abort is the binary's line, not a library value

**Kind:** gap
**Status:** open
**Filed:** by S2.1.3, 2026-11-25, on `0ab1aef0`
**Owner:** none

## What is missing

**The clock's abort is the binary's line, not a library value**: `SolveError::Deadline` displays *--max-time reached inside the cube*, the oracle's wording, and `ein solve`'s line, *--max-time S s reached before the question was decided*, is made by `ein-cli`'s `modes::aborted`

## Evidence

EinAf builds the line itself, `Kind::Deadline`, to agree with the summary ([`einaf/src/kernel/feedback.rs`](../../../../einaf/src/kernel/feedback.rs), `build`); with E-57, nothing else of the run comes back

## What it costs the application

a diagnostic the object carries is the application's copy of the binary's
