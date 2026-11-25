# L-M2.1 — The host builds with rustc 1.100, not the pinned 1.97.1

**Status:** open; the three lints it raised in the engine's code are closed by `acd04b13`
**Found:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md#t2125b--a-saturation-is-bounded), T2.1.2.5b, 2026-11-25
**Owner:** none

## What

The engine pins its toolchain, `ein.rs/rust-toolchain.toml`'s `channel = "1.97.1"`, which CI installs. The host has no
rustup, so nothing reads the pin there: every build, test and lint of the phase ran under the distribution's rustc,
1.100. S2.1.2's record, verbatim: *"`cargo clippy -D warnings` finds nothing in the changed code; under this host's
rustc 1.100 (the distro's; the pinned 1.97.1 is not installed, there being no rustup) it finds three things in files
the task did not touch, `ein-einb/src/sections.rs` (`chunks_exact_to_as_chunks`, twice),
`ein-infer/examples/object_census.rs` (`needless_range_loop`) and `ein-cli/tests/answer_round_trip.rs`
(`manual_unwrap_or`), lints the pinned toolchain does not have."*

## Evidence

- `rustc --version` on the host: `rustc 1.100.0 (0ef1bc386 2026-11-09) (Arch Linux rust 1:1.100.0-1)`; `which rustup`
  finds none (re-taken 2026-11-25).
- `acd04b13`, *"Clippy is clean under the host's newer rustc"*: `as_chunks` for `chunks_exact` with a constant size
  (stable since 1.88, so the pinned toolchain reads it too), an iteration for a range loop, `unwrap_or` for a
  `map_or` of the identity; no behaviour moved. The phase report's § 1 lists it as *"clippy for rustc 1.100"*.
- S2.1.2's later runs and S2.1.3's say *"(the host's rustc 1.100)"* beside each clippy run.

## What it costs

A green gate here is the host's toolchain's, and CI's is the pin's: a lint or a compile error that one of the two
has and the other has not is met by one of them only (the three above were met here and not in CI). Nothing in the
gate says which toolchain it ran under.
