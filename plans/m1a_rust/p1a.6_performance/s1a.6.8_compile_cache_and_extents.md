# S1a.6.8 — The compile cache and the extent counts

**Phase:** P1a.6 (Performance)
**Estimate:** 2 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** what [design/06](../design/06_saturation.md) § Win A already
says, and a correction to [design/03](../design/03_data_model.md) §5
**Added:** 2026-08-18 — this stage did not exist when the phase was planned.
The phase README's rule is that *"everything after S1a.6.1 is chosen by the
table S1a.6.1 produces"*, and the table produced two costs the phase had no
stage for — 21.1 % and 9.5 % of an exhaustive `zebra2`
([baseline.md §7](baseline.md#7-the-top-five-costs) items 1 and 2). One of
them `design/06` § Win A specifies in full and nobody built; the other no
design doc anticipated, because it is a bill `design/03` §5's own trade sent
to a different address. It runs **first** in the phase because both fixes are
small and both are parity-preserving by construction rather than by
measurement.

## Context

### The plan memo is per-saturation, not per-process

`Saturator::new` builds `Engine::new()`, and `Engine` owns its `PlanMemo`.
There is one saturator per fork saturation — plus one per `lookahead` probe
and one per `closed` marking — so a plan is re-compiled for every one of
them. On `zebra2 -e` that is **17 430 compiles**, and `perf` puts 21.1 % of
the run inside `ein_infer::compile` with 19.7 % of it under
`PlanMemo::intern`. `Interner::intern`'s own 1.8 % belongs here too: its
callers are `Compiler::slot` (42 %) and `Compiler::premise` (33 %).

**ein.py compiles 17 430 times as well** — the two agree exactly, in all four
corpus cells — so nothing here is a parity defect and nothing here is
recovered by copying the oracle more closely. What is being claimed is
[design/06](../design/06_saturation.md) § Win A, which specified this memo,
predicted this count, and was never built.

`engine.rs`'s own module comment describes the design this stage
implements, in the course of explaining why the *order* must stay
per-engine:

> …which is why the **process-wide memo** holds the plans and each engine
> keeps its own ordered list ([design/06](../design/06_saturation.md)
> § Win A).

**The parity argument is already written into the code.** Two properties
make the hoist invisible:

1. The `compile` event fires on an **engine** cache miss, not a memo miss
   (`engine.rs`, `compile_for`) — deliberately, and with a comment saying
   so. A shared memo changes memo hits, not engine misses, so the T2 event
   stream is bit-identical.
2. Cache *iteration order* is the per-engine `plans: Vec<PlanId>`, which is
   built by the same `rules × activators` walk in the same order whether or
   not the plan behind each id had to be compiled.

So this is a cache hit rate change and nothing else. It is still measured
against T3, because "nothing else" is a claim.

### `n_facts_of` is O(layers), and the search reaches depth 35

`watch_stamp_into` asks for a relation's extent size once per watched
relation per parked boundary entry: **644 166 times** on `zebra2 -e`, a
number the two implementations agree on exactly. In ein.py that is
`len(self.kb._facts_by_relation.get(rel, ()))` — one dict, O(1). In ein.rs
`Kb::n_facts_of` folds over `self.layers()`, and an exhaustive `zebra2`
reaches **35 layers** ([baseline.md §5](baseline.md#5-memory)), so the same
question costs up to 35 hash lookups. It is 9.5 % of the run.

This is the first cost in the phase that the port **created**: design/03 §5
bought an O(1) fork by making the KB a layer stack, and never asked what an
extent *count* costs on the other side of that trade. The two candidate
fixes are independent and can both land:

- a per-relation count maintained on insert (`FxHashMap<Symbol, u32>` per
  layer, summed once per *layer*, or a single running total per KB), or
- the delta-flatten threshold ([S1a.6.2](s1a.6.2_memory_layout.md)
  T1a.6.2.5), which bounds the depth rather than the per-layer cost.

The count is the direct fix and is invisible to every observable; the
threshold is a tuning knob that helps every other layered read as well.

## Acceptance

- `plan_compile` on `zebra2 -e` drops from 17 430 to **one per distinct plan
  key** — design/06 § Win A estimates ~170 for that run, and the exact number
  is whatever `PlanMemo::by_key` ends with, which the stage reports rather
  than predicts. ein.py's count stays 17 430; **this is the first counter the
  two implementations are expected to disagree on**, which is why the
  `compile` event count (17 250, identical) is the acceptance item next to it
  rather than the compile count.
- `Kb::n_facts_of` is O(1) in layer depth, shown by a test that builds a KB
  of *n* forks and asserts the call count of the underlying map lookup does
  not grow with *n* (the `fork_cost.rs` counting-allocator pattern, applied
  to lookups instead of bytes).
- Before/after on `solve zebra2 -e`, `solve zebra -e`, `boundary` (both
  puzzles now) and `saturate_root`, in
  [design/README § Measured](../design/README.md#measured).
- **T3 green on the whole corpus**, and T2 identical on the `compile` event
  stream specifically — the one place where a cache change could leak.
- `counter_cost` re-run and the table in [baseline.md §4](baseline.md#4-what-the-engine-did--the-work-counters)
  updated, so the next stage starts from counts that describe the build it
  is looking at.

## Tasks

### Task T1a.6.8.1 — Hoist `PlanMemo`

Move the memo out of `Engine` and behind a handle (`Rc<RefCell<PlanMemo>>`
or a `&mut` threaded through `Session`) owned by whatever outlives the
forks — the solve loop for a search, the caller for a bare saturation.
`Engine` keeps `plans` / `keys` / `by_key` / `activators` / `fired`
unchanged, so its order and its `_fired` semantics are untouched.

Watch: `PlanMemo::intern` takes `&mut Terms`, and a plan compiled during
one fork's saturation interns symbols that outlive it. That is already true
today and is why the memo can be shared at all — `Terms` is per-run, not
per-fork.

### Task T1a.6.8.2 — Per-relation extent counts

Maintain `n_facts_of` in O(1): the natural place is alongside `by_rel`,
since every write already touches it. Keep `n_facts_of`'s signature; only
its cost changes. Two invariants to assert rather than assume: a layer's
count matches `by_rel[rel].len()` after every write, and the KB's sum
matches a full walk (a debug assertion, so the parity build checks it on
every corpus entry).

### Task T1a.6.8.3 — Re-measure and re-choose

Re-run the S1a.6.1 instruments (`profile_ein_rs.py`, `counter_cost`,
`alloc_cost`, `cargo bench`) and update [baseline.md](baseline.md). Both
fixes move the *saturate* bucket, which is 59.7 % of `zebra2 -e`; the stage
after this one should be chosen from the profile that results, not from the
one that chose this stage.

## Notes

- Neither fix is allowed to touch a *decision*: not which plan is compiled,
  not the order plans enter a cache, not which candidate a guard drops.
  Both are pure memoisation of an answer that was already deterministic.
- If the memo hoist does not move `solve zebra2 -e` by ≥ 10 %, that is a
  finding worth recording rather than a reason to keep it: it would mean
  compile time was dominated by something other than the compiler, and
  `Compiler::premise`'s 2.1 % self time says otherwise.
