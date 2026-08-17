# P1a.9 — Bindings and release

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 1.5 weeks (8 days of stages)
**Depends on:** [P1a.8](../p1a.8_server_mode/README.md) (or
[P1a.5](../p1a.5_presentation/README.md), if M2 needs the binding
earlier — the PyO3 surface only needs a parity engine, not a server)

## Goal

Ship it: a PyO3 module so [M2](../../m2_nl_to_ir/README.md)'s CPython NL
frontend can drive ein.rs in-process, binaries and wheels for the three
platforms, and the documentation updates that make ein.rs the engine of
record without orphaning ein.py.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.9.1](s1a.9.1_pyo3_surface.md) | The PyO3 surface | 3 d |
| [S1a.9.2](s1a.9.2_api_parity_tests.md) | API parity tests | 2 d |
| [S1a.9.3](s1a.9.3_packaging.md) | Packaging and release | 2 d |
| [S1a.9.4](s1a.9.4_documentation.md) | Documentation | 1 d |

## Acceptance for the phase

- `pip install ein-rs && python -c "import ein_rs; …"` reproduces the
  `docs/api/ein.md` worked example, output-identical to `ein`.
- The API-parity test suite is green (a shared test body parameterised
  over both modules).
- Wheels build in CI for the platform matrix; `ein --version` reports the
  engine and the protocol version.
- Docs no longer describe ein.py as the only implementation, and
  `plans/README.md`'s status table records M1a as shipped with its date.
- ein.py's own suite (`./run_tests.sh`) is still green — it stays the
  oracle.

## Notes

- **The binding is a surface, not a boundary.** Q-M1a.1 settled that the
  port is full (Boundary A); `ein_rs` exists so M2 can call the engine
  cheaply, not so a Python harness can own the loop. Keep the module
  thin: handles in, results out, no callbacks into Python on a hot path.
- Naming: the module is `ein_rs`, not `ein`, so both can be installed
  side by side — which is exactly what the parity tests need. A future
  `ein` meta-package that re-exports one of them is a separate decision.
- The M2 boundary question (PyO3 vs stdin/JSON) is now answered by
  *both* being available: PyO3 from this phase, the socket from
  [P1a.8](../p1a.8_server_mode/README.md). M2 picks per use case rather
  than committing once.

## Cross-links

- [`docs/api/ein.md`](../../../docs/api/ein.md) — the contract to keep
- [design/12 — Toolchain](../design/12_toolchain_and_layout.md)
- [M2 — NL → IR](../../m2_nl_to_ir/README.md)
