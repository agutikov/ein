# E-62 — The page's dependency block does not build

**Kind:** bug
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`62a8ee45`)
**Filed:** by [S2.1.1](../../s2.1.1_tree_and_outcomes.md), 2026-11-25, on `0effa7b0`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), `62a8ee45`: § 1's block names the `[patch.crates-io]` line, and a package of the page's lines builds outside the workspace, with and without the engine's lock file

## What

**The page's dependency block does not build.** [`04-api.md` § 1](../../../../../docs/ein/ein.rs/04-api.md#1-the-crates) says a program depends on the crates by path and shows four lines; `ein-ir` compiles only against the patched `lrpar` under `ein.rs/vendor/lrpar/` (`RECOVERY_STEP_BUDGET`, `recovery_cut`, `recovery_steps`), which the engine's workspace names in a `[patch.crates-io]` that reaches only its own builds. A downstream manifest has to write the same patch, and the page does not say so

## Evidence

[`page_dependency_block/Cargo.toml`](../../../analysis/probes/p2.1/page_dependency_block/Cargo.toml), the page's lines alone, `ein-ir` by path: `cargo build` resolves the published `lrpar` 0.15.0 and fails, `E0432 unresolved import lrpar::parser::RECOVERY_STEP_BUDGET` and two `E0599`; [`einaf/Cargo.toml`](../../../../../einaf/Cargo.toml) writes the patch

## What it costs the application

a first build that fails with an error inside `ein-ir`; loud, so no wrong parser is linked
