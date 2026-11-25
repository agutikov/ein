# E-64 — Part of the load's refusal is the binary's, not the library's

**Kind:** weakness
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`62a8ee45`; EinAf's part `d13dc4d9`)
**Filed:** by [S2.1.1](../../s2.1.1_tree_and_outcomes.md), 2026-11-25, on `0effa7b0`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md): the judgement is the library's, `ein_infer::mechanisms::judge_loaded`, a step after the load that `04-api.md` names and every command takes (`62a8ee45`); EinAf calls it (`d13dc4d9`)

## What

**Part of the load's refusal is the binary's, not the library's.** `ein solve` judges the hypothesis mechanisms on root's fixpoint after the load (`ein-cli`'s `common::judge_mechanisms`) and refuses a conflict at the load's exit; `ein_ir::load_file`, the load [`04-api.md`](../../../../../docs/ein/ein.rs/04-api.md) names, does not, so a library caller answers two corpus programs the binary refuses

## Evidence

`examples/broken/load/no_hypothesis_beside_open.ein` (a `load-negative` entry) and `examples/ein-bugs/closed-and-owing.ein`: `ein_ir::load_file` returns a KB, `ein solve` exits 1 with `kb load error: … its :no-hypothesis names likes …`; EinAf calls `ein_infer::mechanisms::{may_conflict, check}` itself ([`einaf/src/kernel/mod.rs`](../../../../../einaf/src/kernel/mod.rs), `judge_mechanisms`)

## What it costs the application

a caller that follows the page reads *invalid* programs as answered; it has to know the binary's extra step and copy it
