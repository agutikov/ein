# P1a.0 — Conformance harness + shared assets

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **shipped** 2026-08-17 — all four stages, acceptance below.
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

All met, 2026-08-17. Each was run, not read:

| item | result |
|---|---|
| `ein-conformance run --impl-a python --impl-b python` green at T3 over the corpus × run matrix | **the whole corpus: 556 cells over 95 entries, 0 differences** (34 min of engine time). The per-commit subset is 438 of those. T2 over the per-commit set: 215 compared, 0 differences, 223 correctly skipped as emitting no log |
| the `PYTHONHASHSEED` sweep is green, or every failure is fixed in ein.py and pinned | green at T3 (0 vs 42, 438 cells) — **after** fixing H1 and H4, both pinned by regressions that fail on every seed without the fix |
| `stdlib-check` and the corpus-completeness check are in CI and fail on a deliberate drift | both in the per-commit tier; the stdlib check was verified by corrupting `typing.ein` and watching `cargo test -p ein-ir` name the file |
| `cd ein.rs && cargo test` runs | 6 crates, 37 tests |
| `divergences.md` exists and is empty | empty — nothing has needed accepting yet |
| `AGENTS.md` documents `ein.rs/`, `stdlib/`, `conformance/` | plus how to run the harness |

The milestone README also asked this phase to re-measure the acceptance gate
"before trusting the number", and it moved: **43.7 s under PyPy 3.11, not the
~91 s recorded at S1.21.8**. So the "under 5 s" target is ~9×, not the ~18×
the stale figure implied. The target stands; the claim about it did not.

## What the phase found

The premise was "building the harness first finds ein.py bugs". It found
**five**, three of them predicted:

| | |
|---|---|
| **H1** | the `__symmetric__` mirror seed iterated a `frozenset`, so with ≥ 2 markers the firing order depended on `PYTHONHASHSEED` |
| **H2** | mixed `str`/`int` hypothesis args crash `apriori.layer_1` — confirmed, and narrowed to hrule-generated candidates only |
| **H3** | `--shuffle` — confirmed benign: same seed byte-identical, across seeds only the order the k models are found in moves |
| **H4** | *not predicted.* `unsat_core` iterated raw at two display sites, so **the same puzzle produced two different `--trace` files across runs** |
| — | `ein saturate` raised `KBLoadError` through to a traceback where `solve` prints one line, and parsed without `filename=` |

Two more that were decisions rather than defects: Q-M1a.14's proposed
crash-parity rule ("exit code + first stderr line") turned out to compare a
line that is not stable under `PYTHONHASHSEED`, and Q-M1a.16 opened because
only four of the ten `SolverConfig` levers are reachable from a CLI.

And one bug in the harness itself, found by its own first whole-corpus run:
`execute` polled a child whose stdout was a pipe, so `render lattice` — which
writes more DOT than a pipe holds — blocked on `write` while the harness waited
for it to exit. Two 0.3 s cells sat for two minutes. It is the failure mode a
harness is least able to report on itself: it does not crash and it does not
diff, it simply never finishes.

## Cross-links

- [design/01 — Parity contract](../design/01_parity_contract.md)
- [design/02 — Determinism & order](../design/02_determinism_and_order.md)
- [design/11 — Shared assets](../design/11_shared_assets.md)
- [design/12 — Toolchain & layout](../design/12_toolchain_and_layout.md)
