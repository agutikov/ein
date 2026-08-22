# S1a.6.2 — Memory layout

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to [design/03](../design/03_data_model.md)

**Status: shipped 2026-08-19.** `solve zebra2.ein -e` **99.1 → 75.8 ms**
(−23.5 %) and `solve zebra.ein -e` **397.2 → 349.1 ms** (−12.1 %), on two
changes out of eight tasks:

| task | outcome |
|---|---|
| [T1a.6.2.7](#task-t1a627--a-system-allocator-with-per-thread-caches) the global allocator | **shipped** — `snmalloc`, −15.9 % / −7.5 %, allocator self time 20.0 → 9.0 % and 9.4 → 3.0 % |
| [T1a.6.2.6](#task-t1a626--row-packing) row packing | **shipped, inverted** — the row got *bigger* (20 bytes, two arguments inline), −8.5 % / −4.7 % |
| [T1a.6.2.2](#task-t1a622--bucket-major-candidate-storage) bucket-major storage | **not built** — the buckets carry 0.9 % of the candidates |
| [T1a.6.2.1](#task-t1a621--the-participation-index-key) the index key | **not built** — same 0.9 % |
| [T1a.6.2.3](#task-t1a623--smallvec-sizing) `SmallVec` sizing | **instrumented, nothing to size** — the hot path allocates nothing |
| [T1a.6.2.5](#task-t1a625--delta-flatten-threshold) the flatten threshold | **built and reverted** — **+7.6 %** on the search, and the reason is that forks *share* the layered index |
| [T1a.6.2.4](#task-t1a624--arena-reuse-across-forks) pooling / [T1a.6.2.8](#task-t1a628--a-per-entering-region) the region | **parked with a number** — the allocator is 3.2 % / 7.8 % and what is left of it is copied live state |

Every number, and the controls behind them, are in
[baseline.md § 13](baseline.md#13-s1a62--the-layout-stage-and-the-profile-it-starts-from).
**Five of the eight tasks were closed by measurement rather than by code**,
which is the stage working as written: "reverts any that do not pay, because a
layout change with no measured win is pure risk to a T3-green build."

## Context

The data model landed in P1a.2 chose *correct and simple* over *optimal*
in four places, each flagged at the time as "measure in P1a.6". This
stage revisits them with the profile in hand — and reverts any that do
not pay, because a layout change with no measured win is pure risk to a
T3-green build.

**The re-measure changed the stage before it started** ([rule
6](README.md#rules-for-this-phase)). Three tasks below were written against a
profile in which the allocator was 21 % of self time over 2.5 M allocations
averaging ~53 bytes. After S1a.6.8 and S1a.6.9 it is **20.0 % on `zebra2 -e`
and 9.4 % on `zebra -e`**, over 0.88 M / 1.67 M allocations averaging **71–76**
bytes — half the allocations gone, and the ones that went were the small ones.
Where a task's premise moved, the task says so.

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

> **Not built.** The index is asked for **0.9 %** of an exhaustive `zebra`'s
> candidates and 2.3 % of `zebra2`'s; the other 99 % come from a full extent
> scan, because `index_fact` does not key a nested-fact argument and
> `(not (R …))` is what the corpus scans. No key layout reaches a run through
> 1 % of it. The *contents* question this uncovered — key the inside of a
> nested argument — is real, is worth much more, and is
> [S1a.6.3](s1a.6.3_beta_memories.md)'s.

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

> **Not built, and its stated precondition is false.** "Worth trying only if
> the profile shows candidate iteration dominating" — it does (75.4 % of
> `zebra -e` is `unify` + `try_candidate` + `walk`), but the buckets carry
> 0.9 % of the candidates, so a bucket-major layout would serve 0.9 % of it.
> The SIMD argument goes with it: the whole fact store is **22 KB** and has
> never left L1, so the layout was not pointer-chasing through memory, it was
> a two-load dependency chain — which is what T1a.6.2.6 removed instead.

Today a bucket is a `Vec<FactId>` and the matcher then loads each fact's
row and args. A bucket-major layout — storing the *args* of the bucket's
facts contiguously — turns the inner loop into a linear scan over the
values it actually compares. Costs duplication and index maintenance;
worth trying only if the profile shows candidate iteration dominating.

This is also the precondition for any future SIMD work
([design/08](../design/08_parallelism.md) §7 rejected it *for now*
precisely because the layout is pointer-chasing).

### Task T1a.6.2.3 — `SmallVec` sizing

> **Instrumented; nothing on the hot path to size.** `examples/layout_shape.rs`
> prints all five distributions (registers per plan ≤ 5, premises per disjunct
> ≤ 3, arity ≤ 5 with 96.6 % at ≤ 2, extents, fork depth). The matcher already
> allocates **nothing** per candidate — `tests/match_alloc.rs` holds it to
> that — and `--callers` puts the remaining allocator traffic in the
> per-entering snapshot's *copies of live state*, which no inline capacity can
> remove. The one inline capacity the data did choose is
> [`INLINE_ARGS`](#task-t1a626--row-packing), and it is checked by
> `inline_share()`.

Instrument the actual distributions — premises per firing, args per
fact, registers per plan, elements per commitment, alternatives per fact
— and size every inline capacity from data instead of from the guesses
in [design/03](../design/03_data_model.md) / [05](../design/05_matcher.md).
Oversized inline capacity is a memory and memcpy cost; undersized is a
heap allocation in a hot loop.

### Task T1a.6.2.4 — Arena reuse across forks

> **Parked with a number, together with T1a.6.2.8.** After T1a.6.2.7 the
> allocator is **3.2 %** of `zebra -e` and **7.8 %** of `zebra2 -e`, and
> `--callers` says what is left is not scratch: `Entry` drop glue is 44 % of
> the deallocations and `Vec::clone<Entry>` (from `Saturator::resume`) 1.0–1.6 %
> self, i.e. the per-entering snapshot copying live state. Pooling cannot take
> a copy; a region can, and that is T1a.6.2.8 — for a ceiling of a few per
> cent. Re-price after [S1a.6.3](s1a.6.3_beta_memories.md).

A fork allocates a fresh `Delta`; a search enters hundreds. Pool the
delta arenas (and the matcher's cursor/trail buffers) per worker so a
fork reuses memory rather than asking the allocator. Must not change
iteration order or contents — assert with the `flatten()` comparison.

This is *pooling* — the weaker half of T1a.6.2.8's region. Do them in
that order: if the region lands, most of what this task pools is inside
it.

### Task T1a.6.2.5 — Delta-flatten threshold

> **There was no threshold: P1a.2 never built one**, and `Kb::flatten` has one
> caller, a test. So the task became the question behind it — does flattening
> pay? — and the answer is **no, by 7.6 %**. A KB-level flat extent per
> relation makes `facts_of` one hash lookup instead of a chain over 24 layers,
> and it is 8 % *faster* on `match_hot`, 5–7 % faster on `boundary`, and
> **+7.6 % on `solve zebra -e`** at identical work and identical output. The
> benches that improved are the ones that never fork. A fork shares its
> parent's index vectors behind an `Arc`; flattening gives every one of the 24
> live KBs its own copy of the extent the matcher scans. Reverted, and
> [design/03 §5](../design/03_data_model.md) now says so.

P1a.2 shipped "flatten when delta > 25 % of base" as a placeholder. Sweep
it; the tradeoff is lookup indirection (deep deltas) against copy cost
(eager flattening). Observable behaviour is unchanged for any value, so
this is free to tune — which also means it must be measured, not
argued.

### Task T1a.6.2.6 — Row packing

> **Shipped inverted: the row got bigger.** "The profile decides", and it
> decided against the premise — rows are hot, but the store is 22 KB and lives
> in L1, so 8-byte rows would have bought cache density nobody was short of.
> What a candidate actually pays is a *dependency chain*: `rows[id]`, then
> `args[row.args_at]`, twice over (the premise's arguments and the nested
> fact's). `Row` is now **20 bytes with two arguments inline** — 96.6 % of
> `zebra`'s facts, 83.5 % of `zebra2`'s — for **−8.5 %** and **−4.7 %**
> end-to-end. The half that makes it pay is on the caller's side:
> `FactStore::row` + `args_of` read the row once and take the arguments from
> it, where `rel`-then-`args` loads it twice and `get` resolves arguments 79 %
> of `zebra2`'s candidates never read.

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

**Shipped 2026-08-19: `snmalloc`, default-on, `--no-default-features` to opt
out.** All three were measured; the losers are recorded in
[§13](baseline.md#13-s1a62--the-layout-stage-and-the-profile-it-starts-from)
rather than kept as features, because a dependency nobody selects still has to
be audited. `mimalloc` matched the speed and cost **+42 % peak RSS** on
`zebra -e`; `tikv-jemallocator` kept the RSS, returned two thirds of the win,
and gates itself on `cfg(not(target_env = "msvc"))`. Two things the task did
not ask for and got: **`fork` +11.7 %** (a fresh arena's slow path — 3.3 µs a
run, and it read 274 ns again at the end of the stage with nothing in that
path changed, so a 30 ns bench is measuring alignment too), and a *profiling*
binary that ran **+49.6 %**
slower than release until `[profile.profiling.package.snmalloc-sys] debug =
false` stopped `cmake`-rs building the vendored allocator as `RelWithDebInfo`.

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
[P1a.9](../p1a.9_release/README.md) ships one binary.

### Task T1a.6.2.8 — A per-entering region

> **Its premise moved twice.** The 28 000 allocations per entering below were
> measured before S1a.6.9; the same run now allocates 1 674 387 times over 111
> enterings — ≈ **15 000** — and T1a.6.2.7 has already taken the per-allocation
> price down by roughly two thirds (`zebra -e`'s allocator self time is
> **3.0 %**, not 9.4 %). A region can still remove allocations that a fast
> allocator merely makes cheap, but the ceiling on this task is now ~3 % of
> `zebra -e` and ~9 % of `zebra2 -e`, and it is measured against a bump
> allocator's own cost, not against glibc's. Re-price it before building it.

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
