# P1a.2 — KB core

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **shipped** 2026-08-18 — all four stages, acceptance below.
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

**Landed in the order 2.1 → 2.2 → 2.4 → 2.3**, because the loader builds
a `Provenance` for every fact and calls `detect_provenance_cycles`
itself: S1a.2.4 is its dependency, not its sequel.

## Acceptance for the phase

All met, 2026-08-18. 167 tests in `cargo test --workspace`, of which 10
are differential against `ein.py`.

| item | result |
|---|---|
| **T3 on load errors**: every `examples/broken/load/` fixture byte-identical, accumulated and `; `-joined in the same order | all **18** loader fixtures (the 11 import ones landed at S1a.1.3), plus five accumulation programs pinning the cross-pass order — macro → relation → rule → fact → config |
| **KB-shape diff** after load: registries with insertion order, fact order, per-relation extents, the participation index, `names` with categories, the negated set, the rule-application indexes | **95 corpus files**, 0 differences: 62 load, 33 are rejected (29 `KBLoadError`, 4 that never parse), and a rejection is compared on its **message**, which extends the accumulated-error check from the fixtures to the whole corpus |
| `flatten(fork) == materialised copy` in debug builds | `Kb::check_layering` — asserted after every load in the parity test and by unit tests over three-deep branch stacks |
| `fork()` is O(1): allocation count independent of `|facts|`, asserted with a counting allocator | identical `(allocations, bytes)` at 10 facts and at 10 000; ten nested forks cost the same at 100 facts as at 10 000 |
| Peak RSS on `load(zebra2.ein)` ≤ 1/5 of ein.py's | **3.1 MB vs 46.6 MB — 15×**. The load *itself* adds 0.73 MB against 16.2 MB (22×) |

Two numbers the phase also owes the bench set — the `load` and `fork`
entries were the last two `pending(…, "P1a.2")` rows:

| bench | ein.py | ein.rs | |
|---|---:|---:|---|
| `load` zebra2 | 625.6 ms | **1.03 ms** | 607× |
| `fork` + first delta write | 17.3 µs | **248 ns** | 70× |

Both sides independently report **84 facts, 17 relations, 30 rules** for
`zebra2` — the loader's parity restated by a program that is not the
parity test (`crates/ein-ir/examples/load_rss.rs`).

The per-commit conformance tier, re-run at close: **438 cells, 0
differences, T3, 317.9 s of engine time**. `./run_tests.sh --fast`:
1 490 passed.

### The instrument this phase needed

A KB has no CLI surface — the registries, the seven indexes and the
participation counts are exactly what `ein-conformance` cannot see. So
`ein_core::shape` renders all of it as one deterministic text and
`utils/ir_oracle.py` grew a `kb-shape` op that renders the same text from
ein.py. Two rules make the two comparable:

- **facts are named by position** in the fact list, so the first thing a
  diff can report is a fact-order difference;
- **values are rendered with `repr`**, so the integer `7` and the atom
  `7` cannot collide — and `names` is compared *sorted*, because ein.py
  builds that dict over a **set** union and its order is not reproducible
  even run to run ([design/02](../design/02_determinism_and_order.md) §2).

### Where the phase's scope moved

- **The `Prov` arena is global**, beside the fact store, where
  [design/03](../design/03_data_model.md) §5 sketches it inside `KbCore`.
  It is the same trade interning makes: building a record says nothing
  about whether any KB recorded it, so a fork builds freely and a
  `ProvId` means one thing everywhere. What ein.py copies per fork — and
  must, because a justification recorded in a branch can name premises
  root never assumed — is the *table*, and that stayed per-KB. Pinned by
  a test that a fork's alternatives do not reach its parent. The cost is
  that a dead fork's records are not reclaimed until the run ends;
  `accepts_justification` is what keeps that bounded, and
  [P1a.6](../p1a.6_performance/README.md) can revisit it with a number.
- **`record_justification` landed with S1a.2.2**, not S1a.2.4:
  `add_and_index_fact` is defined in terms of it, and a stage that ships
  that half-wired is not shippable.
- **The registries hold syntax by handle.** A rule's `:match` stays as
  parsed until the compiler lowers it, but `ein-core` depends on nothing
  and the AST lives in `ein-ir`. `ExprRef(u32)` is the seam — the loader
  converts from `NodeId`, the compiler will convert back, and the data
  model never learns what a node is.
- **The loader lives in `ein-ir`**, because it needs the frontend and the
  data model at once. `ein.py` keeps `imports.py` in `kb/` for the
  mirror-image reason, and the port already moved that one.
- **`ein-render` opened early**, with `dot_util` and the derivation DAG:
  a provenance walk whose output nobody can read is a walk nobody can
  check. The DAG reproduces
  `ein.py/tests/golden/dot/kb_provenance_dag.dot` byte-for-byte, reading
  the *committed* golden rather than a copy of it.

### What is not yet checked, and why

- **The walks against a saturated KB.** The corpus contains no
  load-time rule provenance at all — `:using` appears once, in a
  comment, and once in the cycle fixture — so `walk_premises`,
  `derivation_dag` and `unsat_core` are exercised here only by unit
  tests and by the golden. Their corpus-wide comparison needs firings,
  and lands with [P1a.3](../p1a.3_deductive_core/README.md).
- **The `alt` events.** `record_justification` returns whether it
  recorded, and `Added::Existing { alt }` carries it to the caller, but
  ein.rs has no events sink yet — that arrives with the CLI at
  [P1a.5](../p1a.5_presentation/README.md).
- **An integer literal wider than `i64`** in a `:priority` or a config
  seed. Priorities are pooled and compared at any width; the two config
  seeds are `i64`, where ein.py is unbounded. No puzzle has one, and the
  place a huge seed would matter is CPython's Mersenne seeding, which is
  already Q-M1a.5.

## Cross-links

- [design/03 — Data model](../design/03_data_model.md)
- [design/02 §2 — ordered containers](../design/02_determinism_and_order.md)
- [`docs/kernel/ir/02-data-model/`](../../../docs/kernel/ir/02-data-model/README.md)
