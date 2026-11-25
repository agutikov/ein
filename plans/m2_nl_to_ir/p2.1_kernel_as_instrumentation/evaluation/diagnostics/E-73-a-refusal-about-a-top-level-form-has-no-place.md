# E-73 — A refusal about a top-level form has no place

**Kind:** gap
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** [F23](../../../../followups/f23_diagnostics_in_eins_words.md)'s D-2 and D-22

## What is missing

**A refusal about a top-level form has no place**: every import refusal, every duplicate and every missing clause ends `at None`, and its finding's `place` is `null`

## Evidence

`ein solve examples/broken/load/rule_duplicate_name.ein --json-summary F`: one `declaration` finding, `place` `null`

## What it costs the application

the model is told which declaration, not where
