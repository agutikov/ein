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

## Notes

- These are the *only* layout changes in scope. Anything that changes
  what a `Value` or a `FactId` *means* belongs back in
  [P1a.2](../p1a.2_kb_core/README.md) with a full KB-shape re-diff.
- Run the allocation-counting test after each task; a "layout
  optimisation" that adds an allocation to the inner loop is a
  regression that a wall-clock benchmark on a warm machine can easily
  hide.
