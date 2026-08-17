# P1a.0 — Conformance harness + shared assets

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 2 weeks (10 days of stages)
**Depends on:** nothing (M1 shipped)
**Blocks:** every other phase — no engine code lands before the oracle
can measure it.

## Goal

Build the machinery that makes "100 % surface match" a measurable claim,
*before* there is anything to measure. At the end of this phase the
repo can prove **ein.py ≡ ein.py** across the whole corpus at every
parity tier — which sounds circular and is exactly the point: a harness
that cannot detect a difference between an implementation and itself
cannot detect one between two implementations either.

Also: the shared-asset move ([design/11](../design/11_shared_assets.md))
and the Rust workspace skeleton, so P1a.1 starts with `cargo test`
already wired to the corpus.

## Why first

Three reasons, in order:

1. **Invariant I2 depends on it.** Every optimisation in
   [P1a.6](../p1a.6_performance/README.md)–[P1a.7](../p1a.7_parallelism/README.md)
   is justified by "the harness says nothing changed". Building the
   harness late means back-filling that justification for work already
   done.
2. **It finds ein.py bugs.** The determinism sweep
   ([design/02](../design/02_determinism_and_order.md) §9) is expected to
   surface at least one — the `frozenset` iteration in the symmetric
   mirror (H1). Those must be fixed in the oracle *before* the port
   copies them.
3. **It is the server's event protocol too.**
   ([design/09](../design/09_server_mode.md) §5.) Designing it once, for
   two consumers, is cheaper than twice.

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.0.1](s1a.0.1_parity_contract_and_corpus.md) | Parity contract, corpus manifest, divergence ledger | 3 d |
| [S1a.0.2](s1a.0.2_oracle_event_protocol.md) | The `--events` protocol in ein.py + the differ | 3 d |
| [S1a.0.3](s1a.0.3_shared_stdlib_and_examples.md) | Repo-root `stdlib/`, resolution chain, drift checks | 2 d |
| [S1a.0.4](s1a.0.4_workspace_skeleton_and_ci.md) | `ein.rs/` workspace, crates, CI tiers, benches | 2 d |

## Acceptance for the phase

- `ein-conformance run --impl-a python --impl-b python` is green at T3
  over the whole corpus × run matrix.
- The determinism sweep (`PYTHONHASHSEED` ∈ {0, 1, 42, random}) is green,
  or every failure is fixed in ein.py and pinned by a test.
- `stdlib-check` and the corpus-completeness check are in CI and fail on
  a deliberately introduced drift.
- `cd ein.rs && cargo test` runs (an empty workspace with the
  conformance crate compiling and the corpus manifest parsed).
- `plans/m1a_rust/divergences.md` exists and is empty.
- `AGENTS.md` documents `ein.rs/`, `stdlib/` and `conformance/`.

## Cross-links

- [design/01 — Parity contract](../design/01_parity_contract.md)
- [design/02 — Determinism & order](../design/02_determinism_and_order.md)
- [design/11 — Shared assets](../design/11_shared_assets.md)
- [design/12 — Toolchain & layout](../design/12_toolchain_and_layout.md)
