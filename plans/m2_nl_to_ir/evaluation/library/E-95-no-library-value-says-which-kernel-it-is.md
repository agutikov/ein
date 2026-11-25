# E-95 — No library value says which kernel it is

**Kind:** gap
**Status:** open
**Filed:** by S2.1.3, 2026-11-25, on `0ab1aef0`
**Owner:** none

## What is missing

**No library value says which kernel it is**: `ein --version`'s version and stdlib digest are `ein-cli`'s (`version.rs`), and `ein_einb::meta::engine_version` is `ein-einb`'s, a crate a caller of the four library crates does not link

## Evidence

the feedback object's `kernel` block: EinAf reads the version from the engine's workspace manifest when it is built and hashes `stdlib/MANIFEST.sha256` as resolved, as `ein-cli` does ([`einaf/src/kernel/mod.rs`](../../../../einaf/src/kernel/mod.rs), `kernel_id`)

## What it costs the application

a record's kernel is rebuilt by the application, and can drift from what `ein --version` prints
