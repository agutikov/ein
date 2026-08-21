# P1a.9 — Bindings and release

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 1.5 weeks (8 days of stages)
**Depends on:** [P1a.10](../p1a.10_single_implementation/README.md) —
**this phase releases ein.rs and only ein.rs**, after the Python engine has
left the tree.

> **Amended 2026-08-21.** The dependency was P1a.5 ("the PyO3 surface needs
> a parity engine and nothing else"), and P1a.10's README named *this* phase
> as **its** hard dependency, on the grounds that `docs/api/` documents the
> Python embedding surface and the PyO3 module is its successor — "if P1a.9
> has not landed, this phase deletes the only implementation of a documented
> contract".
>
> The order is reversed, deliberately, and the argument is simpler than the
> one it replaces: **there is nothing to release two of.** A binding phase
> that ships while a second Python engine is still in the tree has to answer
> "which one does `import ein` get", keep two exception hierarchies in step,
> and publish two packages on the same cadence — all of it work that exists
> only because the other engine does. Removing it first deletes the
> question. What P1a.10 gives up in exchange is one stage of
> `docs/api/` documenting a module that does not exist yet;
> [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md) states that
> gap and [S1a.9.4](s1a.9.4_documentation.md) closes it.

## Goal

Ship it: a PyO3 module so [M2](../../m2_nl_to_ir/README.md)'s CPython NL
frontend can drive ein.rs in-process, binaries and wheels for the three
platforms, and the documentation that makes ein.rs the engine of record —
by then the only one.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.9.1](s1a.9.1_pyo3_surface.md) | The PyO3 surface | 3 d |
| [S1a.9.2](s1a.9.2_api_parity_tests.md) | API parity tests | 2 d |
| [S1a.9.3](s1a.9.3_packaging.md) | Packaging and release | 2 d |
| [S1a.9.4](s1a.9.4_documentation.md) | Documentation | 1 d |

## Acceptance for the phase

- `pip install ein-rs && python -c "import ein_rs; …"` reproduces the
  `docs/api/ein.md` worked example, output-identical to the `ein` **CLI**.
- The API contract suite is green: every symbol `docs/api/` documents is
  exercised, and the module and the CLI give the same answers on the same
  input ([S1a.9.2](s1a.9.2_api_parity_tests.md)).
- Wheels build in CI for the platform matrix; `ein --version` reports the
  engine and the protocol version.
- `docs/api/` documents the PyO3 surface rather than a Python package that
  no longer exists, and `plans/README.md`'s status table records M1a as
  shipped with its date.

## Notes

- **The binding is a surface, not a boundary.** Q-M1a.1 settled that the
  port is full (Boundary A); `ein_rs` exists so M2 can call the engine
  cheaply, not so a Python harness can own the loop. Keep the module
  thin: handles in, results out, no callbacks into Python on a hot path.
- **Naming is now an open choice, not a constraint.** The module is
  `ein_rs` because `ein` was taken by the Python package and the parity
  tests needed both installed at once. Neither reason survives P1a.10.
  Keeping `ein_rs` costs nothing and keeps every plan document correct;
  claiming `ein` is a decision about a published name, and belongs to
  [S1a.9.3](s1a.9.3_packaging.md) with the rest of the release surface.
  Recorded as open rather than settled here.
- **PyO3 is the only boundary M2 gets.** The socket alternative went
  with the server (dropped 2026-08-18), so this phase is not one of two
  options — it is *the* way CPython drives the engine, and the M2
  boundary question is closed by construction. A subprocess call to the
  CLI remains available for anything batch-shaped.

## Cross-links

- [`docs/api/ein.md`](../../../docs/api/ein.md) — the contract to keep
- [design/12 — Toolchain](../design/12_toolchain_and_layout.md)
- [M2 — NL → IR](../../m2_nl_to_ir/README.md)
