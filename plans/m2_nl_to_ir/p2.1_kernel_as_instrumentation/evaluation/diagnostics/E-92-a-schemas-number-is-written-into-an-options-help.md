# E-92 — A schema's number is written into an option's help

**Kind:** weakness
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** the user's to rule; the default is [S2.1.2](../../s2.1.2_diagnostics_surface.md)'s record; the question is [Q-M2.19](../../open_questions/kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md)

## What

**A schema's number is written into an option's help**: `--json-summary`'s and `--json-report`'s help name `ein-summary/2` and `ein-test-report/6`, so a schema that moves moves `help_shape.txt`. `diagnostics` was added to the report's rows under `/6`, where the report's earlier keys each moved its number (`/4`, `/5`, `/6`); the summary's page defines its number by its one shape, which a key added on every run keeps

## Evidence

`grep -n 'ein-summary/2|ein-test-report/6' ein.rs/crates/ein-cli/tests/golden/help_shape.txt`; `git log -S'ein-test-report/5'`

## What it costs the application

a reader that keys on the number cannot tell the report before the key from after it
