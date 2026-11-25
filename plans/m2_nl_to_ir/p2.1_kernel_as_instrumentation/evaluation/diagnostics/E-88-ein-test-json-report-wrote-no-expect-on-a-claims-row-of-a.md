# E-88 — `ein test --json-report` wrote `no-expect` on a claim's row of a file the clock ended

**Kind:** bug
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`)
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`): every row of such a file that states a claim is `error`, with the deadline's diagnostic

## What

**`ein test --json-report` wrote `no-expect` on a claim's row of a file the clock ended**: the row carried its claim and said it had none

## Evidence

`ein test ein.rs/crates/ein-cli/tests/bounded/grow_node.ein -T 0.5 --json-report R` on `95f5dc49`: the row's `outcome` `no-expect`, its `expect` the claim

## What it costs the application

a report that says a claim was absent where it was not checked
