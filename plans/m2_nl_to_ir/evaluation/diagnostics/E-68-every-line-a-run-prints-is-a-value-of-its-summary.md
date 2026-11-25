# E-68 — Every line a run prints is a value of its summary

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**Every line a run prints is a value of its summary.** `--json-summary` ends with `diagnostics`, each line of stderr as `{kind, severity, message, place}` in its order, and is written for a program refused at the parse, the load or the judgement and for a run the clock ended, its answer keys `null`; `ein test --json-report` carries an `error` row's. On every corpus program, root alone, and on every `solve` run the corpus declares, stderr is the summary's diagnostics and nothing else

## Evidence

`ein solve examples/broken/load/derivation_cycle.ein --json-summary F`: exit 1, `diagnostics` one `derivation-cycle` error, `place` `null`; `cargo test -p ein-cli --test summary_diagnostics`, `--test corpus_cli`

## What it gives the application

the loop reads why a program was refused, and what a run noted, from a file and by kind; it reads no stderr
