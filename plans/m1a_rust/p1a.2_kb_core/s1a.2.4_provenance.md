# S1a.2.4 — Provenance and derivation walks

**Phase:** P1a.2 (KB core)
**Estimate:** 2 days
**Depends on:** [S1a.2.2](s1a.2.2_store_and_indexes.md)
**Implements:** `ein/kb/provenance.py`,
[design/03](../design/03_data_model.md) §7

## Context

Provenance is what makes Ein's answers explainable, and the AND/OR
structure S1.21.7 added (a fact is an OR-node over its recorded
derivations) is what makes the explanation search able to find a
*minimal* one rather than an order-dependent one. It is also the part of
the store with the most policy in it — the alternative-justification
rules are semantics, not bookkeeping, and porting them loosely would
change which explanation comes out.

## Acceptance

- `record_justification` returns the same boolean and produces the same
  stored list, in the same order, for every corpus run — verified via
  the `alt` events ([S1a.0.2](../p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md))
  and by a post-run dump of per-fact justification counts.
- `MAX_ALT_JUSTIFICATIONS = 32`, sorted by premise count, with the same
  eviction (an arriving shorter derivation evicts the longest) and the
  same O(1) rejection fast path.
- `derivation_dag`, `walk_premises`, `unsat_core`,
  `detect_provenance_cycles`, `justifications` all agree with ein.py on
  every corpus fact.
- `DerivationDAG.to_dot` byte-identical (`tests/golden/dot/kb_provenance_dag.dot`).

## Tasks

### Task T1a.2.4.1 — `Prov` arena

`kind` / `source` / `rule` / `premises` / `bindings` / `absent_premises`
/ `branch` / `loc`, with `Symbol`-based storage and rendering deferred to
display. Primary map as a dense `Vec<Option<ProvId>>` over `FactId`.

### Task T1a.2.4.2 — Alternative justifications

Port `record_justification`'s rules verbatim:

- rule-kind with non-empty premises only (source/hypothesis are
  assumptions; an empty-premise rule record is a synthetic engine
  writeback whose contract is that walks ground out on it);
- a fact whose *primary* is a terminal takes no alternatives at all;
- identity of a justification is `(rule, premises)` — `bindings` is
  display metadata and is excluded;
- the cap, the sort, the eviction, the fast path.

Plus `accepts_justification`'s cheap pre-check, which exists so the
saturator's highest-volume path never builds a `Prov`.

### Task T1a.2.4.3 — Walks

`walk_premises` as a BFS over `FactId` with a `BitSet` visited set;
`build_derivation_dag` (primary-only by default, all-justifications
opt-in, cycles broken at re-visit); `unsat_core` (frontier = source /
hypothesis / un-provenanced); `justifications` (primary first, then
alternatives in stored order).

### Task T1a.2.4.4 — DOT

`DerivationDAG.to_dot` with `_fact_dot_id` / `_fact_dot_label` / `_esc`
and the shared `hashed_id` (`md5(seed)[:10]`) — the seed builders matter
as much as the digest.

## Notes

- `absent_premises` (S1.21.8's negative provenance) is *recorded* but not
  yet interpreted by any walk. Port it faithfully and resist the urge to
  make walks honour it — that is a semantics change, and
  `absent_semantics.md` explicitly leaves the question open.
- The symmetric mirror creates genuinely cyclic justification graphs
  (`(R a b)` and `(R b a)` justifying each other). The explanation search
  handles this by taking a least fixpoint from the sources up; the walks
  here handle it by breaking at re-visit. Keep both behaviours.
