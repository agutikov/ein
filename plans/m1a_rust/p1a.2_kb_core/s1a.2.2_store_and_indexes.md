# S1a.2.2 — The KB, its seven indexes, and the layered fork

**Phase:** P1a.2 (KB core)
**Estimate:** 4 days
**Depends on:** [S1a.2.1](s1a.2.1_interner_and_values.md)
**Implements design:** [design/03](../design/03_data_model.md) §§5–6, 8

## Context

`KnowledgeBase` in ein.py is registries + a fact list + seven indexes,
with `fork()`/`snapshot()` shallow-copying six of them and sharing the
seventh. ein.rs replaces the copy with a layer: `Arc<KbCore>` plus a
`Delta`, read base-then-delta so iteration order is identical to
"copy the list, then append".

The whole design rests on one property the engine already relies on
everywhere: **the KB is append-only within a run**.

## Acceptance

- KB-shape diff (see the [phase README](README.md)) identical to ein.py
  after load and after any saturation, for every corpus file.
- `flatten(kb)` — materialising base+delta — is byte-identical to a
  freshly-rebuilt KB with the same fact sequence, including list orders.
  Asserted in debug builds after every saturation in conformance runs.
- `fork()` allocates a constant number of times regardless of `|facts|`.
- `snapshot()` copies `_nogoods` while `fork()` shares it — the
  distinction ein.py documents, preserved and tested.
- Delta flattening (when a delta exceeds a threshold) produces a KB
  indistinguishable from the unflattened one.

## Tasks

### Task T1a.2.2.1 — `KbCore` and `Delta`

The layered structures, with base-then-delta iteration for every ordered
list and delta-then-base lookup for every membership test. Ordered
registries (`Vec` + `FxHashMap`) for relations/rules/hrules/macros so
insertion order survives — it is observable through
`hypgen._raw_candidates` and `Engine.compile_all`
([design/02](../design/02_determinism_and_order.md) §2).

### Task T1a.2.2.2 — The seven indexes

`by_rel`, `by_rel_slot_val` (the participation index), `negated`
(a bitset), `rule_apps_by_rule`, `rule_apps_on_rel`, `names`, and the
alternative-justification table. Incremental `index_fact` matching
`_index_fact`'s exact behaviour, including which arg types are keyed
(`str`/`int` only — nested facts are not) and the `NameRef` head/arg
bumping.

`rebuild_indexes` is also needed: the loader calls it once after batch
ingest, and it must produce the same result as the incremental path
(a property test: build both ways, compare).

### Task T1a.2.2.3 — Fork, snapshot, flatten

`fork()` = `Arc` clone + empty delta. `snapshot()` = the same plus a
`_nogoods` copy. `flatten()` = materialise, used by the debug assertion
and when a fork is promoted to a root. `EqClasses` union-find copies its
parent map on both, as ein.py does, and stays inert.

### Task T1a.2.2.4 — `add_fact` vs `add_and_index_fact`

Two distinct paths in ein.py with different dedup strategies: the
loader's `add_fact` scans `self.facts` linearly (indexes not built yet
and first-occurrence wins), while the saturation-time
`add_and_index_fact` dedups against the live index and records an
alternative justification on a hit. Port both, with their different
return semantics — the difference is observable through which
provenance survives.

### Task T1a.2.2.5 — Views

`all_facts()` / `FactView` (`relation`, `about`, `by_source`,
`by_rule`), used by tests and the API surface. Iterator-based, no
materialisation.

## Notes

- The one place ein.py's fork semantics are surprising — a shared entity
  keeps a `_kb` back-pointer to the *original* KB, so `Relation.facts`
  on a fork returns root's facts — has no analogue in Rust (accessors
  take `&Kb`). Confirm no engine path depended on it; the docstring says
  it is intentional and that fork-scoped queries use the indexes
  directly.
- Delta flattening thresholds change no observable behaviour, so they
  are free to tune. Start with "flatten when delta > 25 % of base".
