# L-M2.2 — EinAf has no toolchain pin: CI builds it with the runner's stable

**Status:** open
**Found:** by S2.1.1, its default 4, 2026-11-25
**Owner:** none; it bears on [Q-M2.18](../../open_questions/einaf/Q-M2.18-are-einafs-fmt-and-clippy-steps-of-the-gate.md)

## What

S2.1.1's default 4, verbatim: *"**No toolchain pin of EinAf's**: the step runs from the root, where rustup reads no
`rust-toolchain.toml`, so CI builds EinAf with the runner's stable and the engine's steps with the engine's pin;
locally both with the host's."*

So one commit is built under three toolchains: the engine's pin in CI's engine steps, the runner's stable in CI's
EinAf step, which rebuilds the engine's crates it links
([E-63](../../evaluation/library/E-63-a-program-outside-the-engines-workspace-builds-the-crates-a.md)), and the
host's rustc for everything here ([L-M2.1](L-M2.1-the-host-builds-with-rustc-1-100-not-the-pinned-1-97-1.md)).

## Evidence

`ein.rs/rust-toolchain.toml` is the one toolchain file of the repository; `einaf/` has none, and the gate's EinAf
step is `cargo test --manifest-path einaf/Cargo.toml --profile gate`, run from the repository's root.

## What it costs

The engine's crates are compiled in CI by two toolchains, and EinAf's step can fail, or pass, on what the engine's
steps do not see: a newer stable's deprecation or lint, or a feature the pin lacks. A clippy step of EinAf's
(Q-M2.18) would read the runner's stable's lints.
