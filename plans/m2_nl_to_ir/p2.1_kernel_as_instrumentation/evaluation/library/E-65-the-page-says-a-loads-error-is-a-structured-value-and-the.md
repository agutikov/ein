# E-65 — The page says a load's error is a structured value, and the load of a file hands even the parse's back as text

**Kind:** bug
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`62a8ee45`; EinAf's part `d13dc4d9`)
**Filed:** by [S2.1.1](../../s2.1.1_tree_and_outcomes.md), 2026-11-25, on `0effa7b0`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md): `TextError::Parse` carries the `ParseError` and `KbLoadError` keeps it beside its findings (`62a8ee45`); EinAf reads the value and parses once (`d13dc4d9`)

## What

**The page says a load's error is a structured value, and the load of a file hands even the parse's back as text.** [`04-api.md` § 2.2](../../../../../docs/ein/ein.rs/04-api.md#22-load): *both errors are structured values, not strings to parse again*; `KbLoadError` is `pub struct KbLoadError(pub String)`, its problems joined by `; `, and `ein_ir::load_file` and `ein_ir::saved::load_text`, the load `ein solve` makes, return a parse error as its text (`TextError::Parse(String)`, folded into `KbLoadError` by `load_file`), dropping the `ParseError` that `ein_ir::parse` returns

## Evidence

`ein_ir::saved::TextError`; EinAf parses a refused text a second time to have the value ([`einaf/src/kernel/mod.rs`](../../../../../einaf/src/kernel/mod.rs), `refusal_of_load`)

## What it costs the application

a caller has the parse error's places only by parsing twice, and the load's refusals only as text
