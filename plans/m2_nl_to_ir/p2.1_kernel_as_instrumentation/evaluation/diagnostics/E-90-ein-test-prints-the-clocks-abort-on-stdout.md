# E-90 — `ein test` prints the clock's abort on stdout

**Kind:** weakness
**Status:** closed by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), `df75643c`: `ein test` prints the abort on stderr in the words `ein solve` prints it, its `ERROR` row on stdout saying only `aborted before an answer`, and its summary lists the abort where stderr printed it
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** [S2.1.4](../../s2.1.4_bugs_the_phase_found.md)

## What

**`ein test` prints the clock's abort on stdout**, as its `ERROR` row's line *aborted before an answer: --max-time*, where `ein solve` prints it on stderr; its summary and its report's rows carry the deadline's diagnostic all the same

## Evidence

`ein test examples/lattice/18_two_candidates.ein -T 0 --json-summary F`: stdout `ERROR`, stderr empty, `F`'s `diagnostics` one `deadline`

## What it costs the application

a caller of the binary that reads streams finds the abort in two places by command; one that reads the summary does not
