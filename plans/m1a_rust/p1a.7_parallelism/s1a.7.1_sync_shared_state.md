# S1a.7.1 — Making the shared state `Sync`

**Phase:** P1a.7 (Parallelism)
**Estimate:** 3 days
**Depends on:** [P1a.6](../p1a.6_performance/README.md)
**Implements:** [design/08](../design/08_parallelism.md) §6

## Context

Four structures are shared across every fork and therefore across every
worker: the interner, the fact store, the plan memo, and the immutable
`KbCore` / `Program`. Two of them are append-only and one is fully
immutable, which is why this is a three-day stage and not a rewrite —
[design/03](../design/03_data_model.md) §5 was built for it, and
[S1a.2.1](../p1a.2_kb_core/s1a.2.1_interner_and_values.md) put the seams
in from the start.

The one thing this stage must protect is the invariant that makes
determinism affordable: **no observable ordering may depend on interner
assignment order**. Under concurrency, `Symbol` and `FactId` ids get
assigned nondeterministically, so any sort that reaches output must be
content-based.

## Acceptance

- The full corpus runs identically with the engine compiled `--features
  parallel` but executed at `--jobs 1` (i.e. the `Sync` refactor alone
  changes nothing).
- A synthetic multi-threaded stress (N threads interning and forking
  concurrently) shows: interning is idempotent across threads, a
  `FactId` means the same proposition in every thread, and no thread sees
  another's presence bits.
- TSan clean; `loom` model checks pass for the interner and fact-store
  shard protocols.
- The determinism lint (no hash-map iteration at an observable site) is
  green with an explicitly reviewed allow-list.

## Tasks

### Task T1a.7.1.1 — Interner

Sharded by hash prefix, each shard an `RwLock` over its span table and
lookup map, with the text arena append-only behind its own lock (or
per-shard arenas — simpler, at the cost of a shard index in the
`Symbol`). Reads (the overwhelming majority after load) take a read lock
or, better, a lock-free snapshot pointer.

The **rank table** is derived state: build it once, invalidate on
growth, and rebuild under a lock. Since symbols are effectively frozen
after load, contention here should be nil — assert that with a counter
in debug builds rather than assuming it.

### Task T1a.7.1.2 — Fact store

Rows and args are append-only, so use a segmented vector
(`boxcar`-style: fixed-size blocks, atomically published) which gives
lock-free reads and never invalidates an existing `&[Value]`. The lookup
map is sharded like the interner, with a double-checked insert so two
threads interning the same fact agree on one `FactId`.

### Task T1a.7.1.3 — Plan memo

`RwLock<PlanMemo>` with a double-checked insert. Compiles are rare after
[design/06](../design/06_saturation.md) § Win A, so a coarse lock is
fine — measure and only refine if it shows.

Per-engine plan *lists* stay thread-local and keep Python's ordering
([design/06](../design/06_saturation.md) § Win A's order caveat).

### Task T1a.7.1.4 — `KbCore` / `Program` audit

Confirm nothing mutates a published `KbCore` — including lazily-computed
caches, which must be either absent or behind `OnceLock`. Any `Cell` /
`RefCell` in the engine crates is a bug at this point; a lint catches
them.

### Task T1a.7.1.5 — Ordering audit under concurrency

Grep every `sort`, `sort_by_key`, `BTreeMap` and `min`/`max` in the
engine and classify each as identity-order (fine) or content-order
(must use the semantic comparator / rank table). Add the classification
as a comment at each site — this is the audit that
[design/02](../design/02_determinism_and_order.md) §3 started and that
concurrency makes load-bearing.

### Task T1a.7.1.6 — Verification harness

The stress test, plus a mode that runs a corpus entry with the interner
deliberately **pre-seeded in a random order**, proving that id assignment
order does not reach the output. This is a cheap, permanent regression
net for the invariant.

## Notes

- Resist making the KB itself `Sync`-mutable. Each fork owns its delta;
  that ownership is the whole safety argument, and sharing a mutable KB
  would reintroduce every hazard this design avoids.
- If contention appears anywhere, the first answer is per-worker staging
  with a merge at join, not a finer lock.
