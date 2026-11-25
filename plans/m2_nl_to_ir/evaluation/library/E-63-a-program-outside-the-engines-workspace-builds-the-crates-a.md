# E-63 — A program outside the engine's workspace builds the crates a second time

**Kind:** limit
**Status:** open
**Filed:** by S2.1.1, 2026-11-25, on `0effa7b0`
**Owner:** none

## The limit

**A program outside the engine's workspace builds the crates a second time**, and three things of the engine's manifest do not reach it: the `lrpar` patch (E-62), the `gate` profile, and the lock file's versions. A shared target directory saves the registry crates and not the engine's: a path dependency's artefacts are keyed by its path relative to the workspace root, so `ein-core` to `ein-render` and the patched `lrpar` compile again either way

## Evidence

`einaf/` (its own `target/`, 398 MB): a cold `cargo test --profile gate --no-run` 26.8 s, the tests 4.7 s; warm 4.5 s; after a touch of `ein-infer/src/lib.rs` 20.7 s; into the engine's target directory, after the engine's crates, 17.5 s, every engine crate recompiled (`cargo build -v`: `Compiling ein-core … ein-render`, every registry crate `Fresh`). i9-14900HX, 32 threads, AC

## What it bounds

the gate's EinAf step costs 5 s warm and about 30 s cold; CI keeps `einaf/target/` in its cache. Inherited `workspace = true` fields, `include_dir!` of the stdlib and the generated parser's build script needed nothing
