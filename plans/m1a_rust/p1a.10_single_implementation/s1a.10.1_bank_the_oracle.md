# S1a.10.1 — Bank what only the oracle proves

**Phase:** P1a.10 (One implementation)
**Estimate:** 4 days
**Depends on:** [P1a.9](../p1a.9_bindings_release/README.md)
**Gate:** nothing in this phase is deleted until this stage's ledger has an
owner for every row.

## Context

The harness is not one check, it is four tiers over ~505 cells plus a fuzzer
plus a determinism sweep, and they do not all prove the same *kind* of thing.
Some of it is already duplicated by ein.rs's own tests. Some of it is a claim
about ein.py that stops mattering when ein.py does. And some of it is a claim
about **the semantics** that ein.rs currently gets right only because
something else was checking.

Sorting those three apart is the whole stage, and it has to happen before the
delete because afterwards there is no way to find out which row was which.

## Acceptance

- A **ledger**, one row per behaviour the harness asserts, each with exactly
  one disposition:
  - **covered** — an ein.rs test already asserts it; the test is named;
  - **banked** — a new ein.rs test asserts it; the test is named and lands in
    this stage;
  - **retired** — it was a claim *about ein.py* (its exception classes, its
    `argparse` text, its `sorted()` raising) and dies with it; the reason is
    written down;
  - **accepted loss** — nothing will assert it again. Every row here needs a
    sentence saying what regression could now pass unnoticed. **A short list
    is a result; an empty list is a claim to be suspicious of.**
- The four tiers are each accounted for, not the corpus as a whole:
  **T0** verdict, **T1** every counter, **T2** the event log, **T3** the bytes.
- The **determinism sweep** (`--env-a PYTHONHASHSEED=0 --env-b
  PYTHONHASHSEED=42`, which found hazards H1 and H4) has a successor that does
  not need two engines — ein.rs against itself under a shuffled interner is
  the same question, and
  [S1a.7.1](../p1a.7_parallelism/s1a.7.1_sync_shared_state.md) T1a.7.1.6
  already wants that instrument.
- The **fuzzer** ([S1a.6.6](../p1a.6_performance/s1a.6.6_differential_fuzzer.md))
  keeps every property that is self-checkable — no panic, dump→parse→dump
  round-trip, hash-seed determinism, `--jobs` invariance — and the acceptance
  states plainly that the differential arm, which found all four of its bugs,
  is gone.
- The **divergence ledger** ([D1–D3](../divergences.md)) is re-read: each entry
  either becomes an ein.rs-side fixture asserting *ein.rs's* behaviour, or is
  marked historical.

## Tasks

### Task T1a.10.1.1 — Inventory the tiers

Walk `conformance/` and classify. The mechanical part is cheap — the corpus
manifest lists entries and runs — and the judgement is per *tier*: T3 on a DOT
file is a golden ein.rs can own outright, T1 on a counter is a property, and
T0 on a verdict is the thing P1a.11's stdlib corpus is about to assert from
the outside.

### Task T1a.10.1.2 — Bank the T3 bytes as ein.rs goldens

The pattern exists: [S1a.6.11](../p1a.6_performance/s1a.6.11_fixture_goldens.md)
already did this for what the contract stopped comparing, and
`ein.rs/crates/*/tests/golden*` is where they live. Extend it to the cells
that only the harness covers. Byte goldens are cheap to generate and their
weakness is well known — they pin *behaviour*, not *intent* — so record which
ones are pinning something nobody can otherwise state.

### Task T1a.10.1.3 — Bank the T1 counters as properties

A counter golden rots into "whatever it was last time". Prefer a property
where one exists: `enterings_total = alive + dead_pre + dead_post`,
`nogoods_emitted + nogoods_subsumed = deaths under path-nogoods`, the
`--jobs` invariants from
[S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md). Where no
property exists, a golden with a comment saying *why* the number is that
number.

### Task T1a.10.1.4 — The determinism successor

`utils/check_hashmap_iteration.py` and the two-seed sweep, re-aimed: one
engine, interner pre-seeded in a random order, whole corpus, output must not
move. This is the invariant [design/08](../design/08_parallelism.md) §1 calls
the one that makes determinism affordable, and it needs no oracle.

### Task T1a.10.1.5 — The accepted-loss list

Written last, from what is left over, and reviewed rather than filed. This is
the honest part of the stage: the harness caught four parity bugs on a surface
five phases had signed off, and whatever class those came from is exactly the
class that will not be caught again.

## Notes

- The temptation is to bank everything as byte goldens because it is
  mechanical. Resist it in proportion to how much the golden would be
  *explaining*: a DOT file has no argument to make, a counter does.
- Anything that cannot be banked is a candidate for
  [P1a.11](../p1a.11_stdlib_conformance/README.md), which checks rules against
  stated expectations rather than against a second engine — the one kind of
  check that gets *stronger* when the oracle leaves.
