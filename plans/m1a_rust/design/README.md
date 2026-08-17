# M1a design docs — ein.rs

The *how* of the [M1a Rust port](../README.md). The milestone README
carries scope, phases and status; these twelve documents carry the
decisions.

## Reading order

Three groups, and they are worth reading in this order because each
constrains the next:

1. **The contract** — what may not change.
   [01 Parity contract](01_parity_contract.md) →
   [02 Determinism & order](02_determinism_and_order.md) →
   [11 Shared assets](11_shared_assets.md)

2. **The machine** — what changes underneath.
   [03 Data model](03_data_model.md) →
   [04 IR frontend](04_ir_frontend.md) →
   [05 Matcher](05_matcher.md) →
   [06 Saturation](06_saturation.md) →
   [07 Search layer](07_search_layer.md)

3. **The scale-out** — what the port unlocks.
   [08 Parallelism](08_parallelism.md) →
   [09 Server mode](09_server_mode.md) →
   [10 Binary format](10_binary_format.md) →
   [12 Toolchain & layout](12_toolchain_and_layout.md)

Group 1 is not optional preamble. Every optimisation in group 2 is
justified *only* because group 1 can prove it changed nothing; a reader
who skips to [05](05_matcher.md) will find claims like "this preserves
match order" whose enforcement lives in [02](02_determinism_and_order.md).

## What each doc settles

| doc | settles | phase |
|---|---|---|
| [01 Parity contract](01_parity_contract.md) | the four parity tiers, the JSONL oracle event protocol, the corpus, the divergence ledger | [P1a.0](../p1a.0_conformance_harness/README.md) |
| [02 Determinism & order](02_determinism_and_order.md) | the audited list of order-sensitive sites in ein.py and the Rust structure that reproduces each | [P1a.0](../p1a.0_conformance_harness/README.md)–[P1a.5](../p1a.5_presentation/README.md) |
| [03 Data model](03_data_model.md) | `Symbol`/`Value`/`FactId` as `u32`, the fact row store, the seven indexes, the layered COW KB, arenas | [P1a.2](../p1a.2_kb_core/README.md) |
| [04 IR frontend](04_ir_frontend.md) | hand-written lexer + recursive-descent parser, AST arena, dumper, macro expansion, import resolution | [P1a.1](../p1a.1_ir_frontend/README.md) |
| [05 Matcher](05_matcher.md) | plan bytecode, slot registers + backtrack trail, candidate selection, beta-memories, WCOJ trigger | [P1a.3](../p1a.3_deductive_core/README.md), [P1a.6](../p1a.6_performance/README.md) |
| [06 Saturation](06_saturation.md) | the two-phase closure/boundary loop, semi-naive delta, the two heaps, incremental NAF invalidation, the native mirror | [P1a.3](../p1a.3_deductive_core/README.md) |
| [07 Search layer](07_search_layer.md) | hypgen enumeration, lookahead, apriori generation, no-good store, the layer loop, verdict synthesis | [P1a.4](../p1a.4_search_layer/README.md) |
| [08 Parallelism](08_parallelism.md) | four parallel levels; speculate-and-validate with read-set tracking; the `--jobs` contract | [P1a.7](../p1a.7_parallelism/README.md) |
| [09 Server mode](09_server_mode.md) | daemon lifecycle, session/KB/solve handles, JSON-RPC surface, streaming, caches | [P1a.8](../p1a.8_server_mode/README.md) |
| [10 Binary format](10_binary_format.md) | `.einb` container layout, mmap-ability, versioning, content addressing, the solution store | [P1a.8](../p1a.8_server_mode/README.md) |
| [11 Shared assets](11_shared_assets.md) | repo-root `stdlib/`, resolution order in both impls, drift detection, the shared corpus | [P1a.0](../p1a.0_conformance_harness/README.md) |
| [12 Toolchain & layout](12_toolchain_and_layout.md) | the `ein.rs/` workspace, crate split, dependency policy, MSRV, CI, benches | [P1a.0](../p1a.0_conformance_harness/README.md) |

## Measured

Kept here so the milestone's claims stay falsifiable. Filled in per
phase; the baseline row is the promotion-time measurement from the
[milestone README](../README.md#baseline--what-einrs-has-to-beat).

| date | build | `zebra2 -e` e2e | `zebra -e` e2e | acceptance gate | note |
|---|---|---|---|---|---|
| 2026-08-17 | ein.py, CPython 3.14 | 5.69 s | — | — | baseline |
| 2026-08-17 | ein.py, PyPy 3.11 | 4.07 s | 8.15 s | ~91 s † | baseline |
| — | ein.rs P1a.5 (parity, unoptimised) | — | — | — | *expected slower than PyPy; that is fine* |
| — | ein.rs P1a.6 (optimised, `--jobs 1`) | — | — | — | target ≤ 0.2 s / ≤ 0.4 s / ≤ 5 s |
| — | ein.rs P1a.7 (`--jobs 8`) | — | — | — | — |

† The `zebra2 -e` / `zebra -e` figures were measured 2026-08-17 on the
dev machine; the acceptance-gate figure is the one recorded at S1.21.8
and is re-measured in [P1a.0](../p1a.0_conformance_harness/README.md).

## Conventions used in these docs

- **Rust snippets are sketches, not committed API.** They exist to pin
  a *shape* (how many bytes, how many indirections, what is copied on a
  fork). Names will drift; the byte counts must not.
- **Every "faster" claim names the ein.py site it replaces**, so the
  parity harness knows what to watch.
- **`§Ox`** refers to the operation numbering in
  [`docs/kernel/inference/architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md)
  §4–§6 (O1 join, O2 saturation, O3 NAF, O4 equality, O5 clash,
  O6 provenance, O7 lattice, O8 pruning, O9 canonicalisation). That
  document is the shared vocabulary between ein.py and ein.rs; these
  docs extend it rather than restate it.
