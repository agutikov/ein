# S1a.6.2 — Memory layout

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to [design/03](../design/03_data_model.md)

## Context

The data model landed in P1a.2 chose *correct and simple* over *optimal*
in four places, each flagged at the time as "measure in P1a.6". This
stage revisits them with the profile in hand — and reverts any that do
not pay, because a layout change with no measured win is pure risk to a
T3-green build.

## Acceptance

- Each change lands as its own commit with a before/after on the
  benchmark it targets.
- T3 green after every commit.
- Peak RSS not worse; ideally better.
- Any change that measures within noise is reverted in the same session,
  with the number recorded in
  [design/README.md § Measured](../design/README.md#measured).

## Tasks

### Task T1a.6.2.1 — The participation index key

`(Symbol, u8, Value)` is 9 bytes padded to 12, hashed per `Scan` step.
Alternatives: hash the triple to a `u64` and keep the exact key in the
bucket for collision checks; or split into a per-relation table indexed
by slot, so the lookup is `by_rel[rel].slots[i].get(value)` and the
relation lookup is hoisted out of the inner loop entirely (the relation
is compile-time constant per step, so this is a pointer cached on the
plan).

The second is more promising and is also a step toward
[S1a.6.3](s1a.6.3_beta_memories.md)'s memories.

### Task T1a.6.2.2 — Bucket-major candidate storage

Today a bucket is a `Vec<FactId>` and the matcher then loads each fact's
row and args. A bucket-major layout — storing the *args* of the bucket's
facts contiguously — turns the inner loop into a linear scan over the
values it actually compares. Costs duplication and index maintenance;
worth trying only if the profile shows candidate iteration dominating.

This is also the precondition for any future SIMD work
([design/08](../design/08_parallelism.md) §7 rejected it *for now*
precisely because the layout is pointer-chasing).

### Task T1a.6.2.3 — `SmallVec` sizing

Instrument the actual distributions — premises per firing, args per
fact, registers per plan, elements per commitment, alternatives per fact
— and size every inline capacity from data instead of from the guesses
in [design/03](../design/03_data_model.md) / [05](../design/05_matcher.md).
Oversized inline capacity is a memory and memcpy cost; undersized is a
heap allocation in a hot loop.

### Task T1a.6.2.4 — Arena reuse across forks

A fork allocates a fresh `Delta`; a search enters hundreds. Pool the
delta arenas (and the matcher's cursor/trail buffers) per worker so a
fork reuses memory rather than asking the allocator. Must not change
iteration order or contents — assert with the `flatten()` comparison.

This is *pooling* — the weaker half of T1a.6.2.8's region. Do them in
that order: if the region lands, most of what this task pools is inside
it.

### Task T1a.6.2.5 — Delta-flatten threshold

P1a.2 shipped "flatten when delta > 25 % of base" as a placeholder. Sweep
it; the tradeoff is lookup indirection (deep deltas) against copy cost
(eager flattening). Observable behaviour is unchanged for any value, so
this is free to tune — which also means it must be measured, not
argued.

### Task T1a.6.2.6 — Row packing

`Row { rel: Symbol, args_at: u32, arity: u16, _pad: u16 }` is 12 bytes
with 2 wasted. Options: pack arity into the top bits of `args_at`
(arity is tiny), giving 8-byte rows and 1.5× the rows per cache line; or
leave it, if rows are not hot. The profile decides.

### Task T1a.6.2.7 — A system allocator with per-thread caches

Rust's default global allocator on Linux is glibc `malloc`, which is
exactly the **15.2 % `[libc.so.6]` + 3.0 % `malloc` + 2.9 % `cfree`** the
profile reports ([baseline.md §7](baseline.md#7-the-top-five-costs) item
4) over 2.5–3.1 M allocations averaging ~53 bytes. That allocation profile —
enormous counts, tiny sizes, extreme lifetime locality — is the one
`jemalloc`, `mimalloc` and `snmalloc` are built for, and swapping it is
four lines:

```rust
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

Measure all three (`tikv-jemallocator`, `mimalloc`, `snmalloc-rs`) against
the default rather than picking one by reputation, on both puzzles, and
report:

- wall clock through `utils/e2e_baseline.py` and `cargo bench`;
- **peak RSS**, which is the number that can regress — these allocators
  buy speed with retained arenas. [§5](baseline.md#5-memory) already
  records that the ~19 % RSS spread across runs of the *same* binary is
  the allocator's high-water mark rather than the program's, so the noise
  floor for this claim is known before the task starts;
- allocation counts, which must not move at all — `examples/alloc_cost.rs`
  counts through `GlobalAlloc`, so it measures the program, not the
  allocator, and a change there means something else moved.

**Parity is free here by inspection, not just by the harness:** nothing
outside `#[cfg(test)]` orders, hashes or compares by address — no
`ptr::eq`, no `as *const`, no `by_address`, and `matched_plans` is a
bitset over plan indices where ein.py uses `id(plan)`. An allocator cannot
reach an observable. T3 still runs, as the standard.

Ship it as a default-on feature with an escape hatch, since a distro build
may want the system allocator, and note the binary-size delta —
[P1a.9](../p1a.9_bindings_release/README.md) ships one binary.

### Task T1a.6.2.8 — A per-entering region

The stronger form of T1a.6.2.4, and
[§9](baseline.md#9-the-fork-entry-re-derivation) is its justification: a
`zebra -e` entering allocates ≈ 28 000 times for ≈ 2.7 MB of churn, and
what survives it is a **3.9 KB** delta — on the order of **0.15 %**. The
other 99.85 % (registers, trails, `BindingKey` boxes, `Entry`s,
`GuardSetId` tables, `plan_key`'s `Vec<String>`s, compile scratch) dies at
one instant, and **64 % of enterings die entirely**, taking their KB delta
with them.

That is a textbook region: bump-allocate everything an entering owns into
an arena, and release the arena when the node is dropped instead of
freeing 28 000 objects one at a time. Design notes:

- **The survivor is the constraint.** `CommitmentSetResult.kb` is handed
  back for an *alive* fork — "which is what lets the caller keep an alive
  fork without a second saturation" (`commitment.rs`). So either the KB
  delta and its provenance live outside the region, or an alive fork pays
  one copy-out of ~3.9 KB at the end. Measure both; the copy-out is
  probably cheaper than splitting the allocation domain, and it is
  certainly simpler.
- **Scope it to the entering, not to the lattice node.** A node is a
  search-layer concept; `try_commitment_set` is the unit that is pure with
  respect to root, and it is the unit
  [P1a.7](../p1a.7_parallelism/README.md) parallelises — so a region per
  entering is also a region per worker task, with no sharing to reason
  about.
- **Ordering.** A bump arena hands out addresses in allocation order,
  which is *more* deterministic than the general allocator, and nothing
  reads an address anyway (see T1a.6.2.7). The `flatten()` comparison and
  T3 are still the gate.
- **Where it stops.** Interned symbols, the fact-text arena and the
  provenance arena are process-global and append-only; they are not in
  scope and must not be moved into a per-entering region.

Report peak RSS with the region as well: trading 28 000 frees for one
release can raise the high-water mark if a long-lived entering's arena
grows to hold its whole churn. If it does, the region is chunked and the
chunks are recycled through T1a.6.2.4's pool.

## Notes

- These are the *only* layout changes in scope. Anything that changes
  what a `Value` or a `FactId` *means* belongs back in
  [P1a.2](../p1a.2_kb_core/README.md) with a full KB-shape re-diff.
- Run the allocation-counting test after each task; a "layout
  optimisation" that adds an allocation to the inner loop is a
  regression that a wall-clock benchmark on a warm machine can easily
  hide.
