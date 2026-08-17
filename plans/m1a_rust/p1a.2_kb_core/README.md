# P1a.2 — KB core

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 2.5 weeks (12 days of stages)
**Depends on:** [P1a.1](../p1a.1_ir_frontend/README.md)
**Blocks:** [P1a.3](../p1a.3_deductive_core/README.md)

## Goal

The data model: interning, `Value`/`FactId` as integers, the fact row
store, the seven indexes, the layered copy-on-write KB, provenance, and
the loader that turns resolved forms into all of it.

This is where invariant I2 is cashed in — it is the phase that decides
what every later loop costs. It is also where the port's most
load-bearing *unobservable* change lives: `fork()` going from an
O(|facts|) index copy to an `Arc` clone, without which
[P1a.6](../p1a.6_performance/README.md)'s beta-memories and
[P1a.7](../p1a.7_parallelism/README.md)'s parallel search are both
unaffordable.

Design: [design/03](../design/03_data_model.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.2.1](s1a.2.1_interner_and_values.md) | Interner, `Value`, `FactId`, the fact store | 3 d |
| [S1a.2.2](s1a.2.2_store_and_indexes.md) | KB, the seven indexes, the layered fork | 4 d |
| [S1a.2.3](s1a.2.3_loader.md) | `from_ir` — validation, registries, load errors | 3 d |
| [S1a.2.4](s1a.2.4_provenance.md) | Provenance, alternatives, derivation walks | 2 d |

## Acceptance for the phase

- **T3 on load errors**: every fixture in `examples/broken/load/`
  produces a byte-identical `KBLoadError` message (accumulated,
  `; `-joined, in the same order).
- **KB-shape diff** after load and after `rebuild_indexes`, for every
  corpus file: relation/rule/hrule/macro registries with their
  **insertion order**, fact list order, per-relation extents in order,
  the participation index, `names` with categories, the negated set,
  rule-application indexes.
- `flatten(fork) == materialised copy` assertion holds in debug builds.
- `fork()` is O(1): allocation count independent of `|facts|`, asserted
  with a counting allocator.
- Peak RSS on `load(zebra2.ein)` ≤ 1/5 of ein.py's.

## Cross-links

- [design/03 — Data model](../design/03_data_model.md)
- [design/02 §2 — ordered containers](../design/02_determinism_and_order.md)
- [`docs/kernel/ir/02-data-model/`](../../../docs/kernel/ir/02-data-model/README.md)
