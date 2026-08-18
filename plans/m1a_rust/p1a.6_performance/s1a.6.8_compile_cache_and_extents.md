# S1a.6.8 — The compile cache and the extent counts

**Phase:** P1a.6 (Performance)
**Status:** **shipped 2026-08-18** — `391a506` (T1a.6.8.1) and `d944c4a`
(T1a.6.8.2). Results in
[baseline.md §10](baseline.md#10-after-s1a68--the-same-instruments-re-run);
every acceptance item below is met.
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

## What it did

| | at S1a.6.1 | at S1a.6.8 |
|---|---:|---:|
| `solve zebra2 -e` | 198.8 ms | **138.1 ms** (−30.5 %) ✅ target |
| `solve zebra -e` | 585.8 ms | **539.9 ms** (−7.8 %) ❌ 1.35× short, was 1.46× |
| the acceptance gate | 1.27 s | **1.02 s** (−19.7 %) |
| `plan_compile` (`zebra2 -e`) | 17 430 | **305** |
| `ein_infer::compile` cumulative | 21.1 % | **2.4 %** |
| `Kb::n_facts_of` self | 9.5 % | **1.2 %** |
| allocations (`zebra2 -e`) | 2 536 702 | **1 344 404** |
| `fork/zebra2` | 257 ns | **268.9 ns** (+4.6 %, and it should have) |
| T3 | 472/473, D2 only | **472/473, D2 only** |

Three things the stage found that its plan did not contain:

1. **The two halves move different puzzles.** The memo is worth 18.3 % on
   `zebra2 -e` and **0.1 %** on `zebra -e`; the extent count is worth 13.9 %
   and **7.3 %**. Built together, either would have read as a wash on one of
   the two — which is why they were built separately and measured as a
   series.
2. **`boundary` barely moved** (−2.9 % / −0.9 %) while the extent fix was
   worth 7.3 % of `zebra -e` end-to-end. The bench saturates a *root*, where
   depth is 1 and the fold was already O(1). A bench set that only measures
   roots cannot price a fix to the search.
3. **Half of `zebra2 -e`'s allocations were the compiler's** — 2.54 M → 1.34 M,
   with no allocation work done. [§7](baseline.md#7-the-top-five-costs) item 4
   predicted it in its caller list and item 1 collected it.

## Tasks

### Task T1a.6.8.1 — Hoist `PlanMemo`

Move the memo out of `Engine` and behind a handle (`Rc<RefCell<PlanMemo>>`
or a `&mut` threaded through `Session`) owned by whatever outlives the
forks — the solve loop for a search, the caller for a bare saturation.
`Engine` keeps `plans` / `keys` / `by_key` / `activators` / `fired`
unchanged, so its order and its `_fired` semantics are untouched.

**Done, with two departures from the sketch.** The handle is
`SharedMemo = Arc<Mutex<PlanMemo>>` and **not** `Rc<RefCell<_>>`: `terms.rs`
asserts `Send + Sync` on the intern tables from the start, with a test whose
comment says the point is to rule out "an `Rc` or a `RefCell` creeping in
later", and the plans are exactly what P1a.7 shares across threads. The lock
costs nothing because it is taken only on an *engine* cache miss — `Engine`
now keeps its own `Arc<Plan>` per cached pair, so the read path never reaches
the memo. And it lives on the `Session` rather than being threaded
separately, which is what gives `lookahead` and `closed` the same memo for
free; `try_commitment_set` takes it as one added parameter and forwards it.

Watch: `PlanMemo::intern` takes `&mut Terms`, and a plan compiled during
one fork's saturation interns symbols that outlive it. That is already true
today and is why the memo can be shared at all — `Terms` is per-run, not
per-fork. It is also why the memo is per-**run** and not per-process: a
`PlanKey` holds `Symbol`s, and symbols are only meaningful inside the `Terms`
that interned them, so a genuinely process-global memo would be unsound the
moment a second `Terms` existed — which is every test binary.

While here: `compile_for` stops recomputing `plan_key` inside `intern`
(`intern_keyed`). It already had the key, and building one renders and
re-interns every activator argument.

### Task T1a.6.8.2 — Per-relation extent counts

Maintain `n_facts_of` in O(1): the natural place is alongside `by_rel`,
since every write already touches it. Keep `n_facts_of`'s signature; only
its cost changes. Two invariants to assert rather than assume: a layer's
count matches `by_rel[rel].len()` after every write, and the KB's sum
matches a full walk (a debug assertion, so the parity build checks it on
every corpus entry).

**Done as one count per `Kb` rather than one per `Layer`** — a `Layer` is
compared field-by-field by `Layer::diff` and sized by `Layer::footprint`, and
a count is neither shape nor delta. The invariant is checked by
`Kb::check_extent_counts`, called from `check_layering`, which is where every
KB-shape fixture already asks whether the layer stack adds up. The cost lands
on `fork`, which clones the map: +4.6 % on that bench, recorded in
[§10](baseline.md#10-after-s1a68--the-same-instruments-re-run).

### Task T1a.6.8.3 — Re-measure and re-choose ✅

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
  `Compiler::premise`'s 2.1 % self time says otherwise. — **It moved it
  18.3 %**, and moved `zebra -e` by 0.1 %. The threshold was the right
  question asked of the wrong puzzle.
- **What this leaves for the next stage.** `zebra -e` is now **72.6 %
  match/bind** self time and `zebra2 -e` is 42.2 %; `ein_infer::compile` is
  2.4 % and 0.3 %. The phase's remaining work is one subsystem. Re-read
  [§8](baseline.md#8-what-this-chooses-for-the-rest-of-the-phase) against
  [§10](baseline.md#10-after-s1a68--the-same-instruments-re-run) before
  starting the next one, per rule 6 — the allocator share is down to 17.9 %
  from 21 %, which slightly weakens S1a.6.2's own headline while
  [§9](baseline.md#9-the-fork-entry-re-derivation) and S1a.6.3 are untouched
  by this stage.
