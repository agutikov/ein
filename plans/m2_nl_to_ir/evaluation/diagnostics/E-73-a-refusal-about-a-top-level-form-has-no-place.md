# E-73 — A refusal about a top-level form has no place

**Kind:** gap
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** [F23](../../../followups/f23_diagnostics_in_eins_words.md)'s D-2 and D-22

## What is missing

**A refusal about a top-level form has no place**: every import refusal, every duplicate and every missing clause ends `at None`, and its finding's `place` is `null`

## Evidence

`ein solve examples/broken/load/rule_duplicate_name.ein --json-summary F`: one `declaration` finding, `place` `null`

Confirmed by S2.2.2, 2026-11-26: the refusals a model's first programs met most at the load, `unbound-conclusion` (the probe's `library.off.gap`, `.bystander`, `.boxes12`), name the rule and the variable and have no place, where `full-name` and a parse error have their line and column ([the probe's record](../../p2.2_formalizer/model_runtime/README.md)).

## What it costs the application

the model is told which declaration, not where
