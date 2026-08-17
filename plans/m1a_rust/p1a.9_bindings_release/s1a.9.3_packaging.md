# S1a.9.3 — Packaging and release

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 2 days
**Depends on:** [S1a.9.2](s1a.9.2_api_parity_tests.md)
**Implements:** [design/12](../design/12_toolchain_and_layout.md) §4
(release tier)

## Context

Distribution was reason #1 for the port — "M1b GUI + M2 NL frontend ship
to end-users; PyPy adds a second interpreter for the user to install,
ein.rs ships as a single binary". This stage cashes that in: static
binaries for three platforms, wheels for the PyO3 module, and a release
process that cannot ship an engine whose parity is unverified.

## Acceptance

- `ein` binaries for Linux (x86_64 + aarch64, statically linked against
  musl where practical), macOS (universal2), and Windows (x86_64), each
  running the corpus at T3 on its own platform.
- `ein_rs` wheels for CPython 3.10–3.13 on the same platforms, built by
  `maturin`, installable into a clean venv, passing
  [S1a.9.2](s1a.9.2_api_parity_tests.md)'s suite.
- `ein --version` reports engine version, protocol version, feature
  flags, and the stdlib manifest hash.
- Release artefacts carry checksums; the release job refuses to publish
  if conformance-full or the acceptance gate is red.
- A `--no-default-features` build still compiles and passes the unit
  suite (proving the feature gating is real).

## Tasks

### Task T1a.9.3.1 — Build matrix

Cross-compile targets, the musl static build, macOS universal2, and the
Windows toolchain. Keep the embedded stdlib
([S1a.0.3](../p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md)
T1a.0.3.4) working on every target — `include_dir!` paths are the usual
casualty of a cross build.

### Task T1a.9.3.2 — Wheels

`maturin` for the `python` feature. Decide the wheel name (`ein-rs`
distribution, `ein_rs` module) and record that it is intentionally
installable alongside `ein`. abi3 if it holds; per-version wheels if not.

### Task T1a.9.3.3 — Platform conformance

Run the corpus at T3 on each platform, not just Linux. The likely
offenders are line endings on Windows, filesystem case-sensitivity on
macOS, and locale-dependent float or string formatting — all of which
would be silent on a single-platform CI.

### Task T1a.9.3.4 — Version reporting

`ein --version` and the server's `server.hello` report the same tuple.
Include the stdlib manifest hash: it is the one input that can differ
between a binary and a checkout, and a mismatch explains a whole class of
confusing behaviour instantly.

### Task T1a.9.3.5 — Release workflow

Tag → build matrix → conformance-full → acceptance gate → sign and
publish. The gate ordering matters: artefacts are built first (so a
failure is diagnosable) but published only after the parity run is
green.

### Task T1a.9.3.6 — Install docs

A short install section for each channel (binary download, `cargo
install`, `pip install ein-rs`), including how to point at a different
stdlib (`EIN_STDLIB`) and how to verify a binary against the manifest.

## Notes

- Do not install the `ein` binary onto `$PATH` in developer setups by
  default; the conformance harness invokes both engines by explicit path
  and an ambiguous `ein` on `$PATH` has burned every project that allowed
  it.
- The Python package `ein` (ein.py) keeps its own release cadence and
  its own name. Any future meta-package that re-exports one of the two
  is a separate decision, not part of this stage.
