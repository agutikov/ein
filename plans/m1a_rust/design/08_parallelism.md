# 08 — Parallelism: four levels, all deterministic

**Settles:** where ein.rs uses multiple cores, and how each site keeps
the sequential engine's observable behaviour.
**Phase:** [P1a.7](../p1a.7_parallelism/README.md) (after parity, after
the single-threaded optimisation programme).

---

## 1. The contract first

Parallelism is the one part of the port that can break
[01](01_parity_contract.md) by accident, because "same answer" and "same
counters" are different promises. So the contract is explicit and
user-visible:

| mode | flag | guarantees |
|---|---|---|
| **sequential** | `--jobs 1` (**default**) | T3 — byte-identical to ein.py. This is what the conformance harness runs. |
| **deterministic parallel** | `--jobs N` | T3 as well: same verdict, same models, same dead set, **same counters**, same stdout. Only wall-clock differs. |
| **unordered parallel** | `--jobs N --unordered` | T0 only: same verdict and same model set. Counters and traversal order may differ. Opt-in, for throughput on large searches. |

The default being `--jobs 1` is deliberate: a benchmark or a golden run
must never silently become a different computation. `--unordered` exists
because there are workloads where the last 20 % of determinism costs 2×,
and the user should be able to buy it back knowingly.

### The invariant that makes determinism affordable

> **No observable ordering may depend on interner assignment order.**

`Symbol` and `FactId` ids are assigned in first-seen order, and under
parallelism that order is nondeterministic. Every observable sort must
therefore be **content-based** — by name rank, by `Value` semantics, by
`python_repr` — never by raw id. Identity uses (hash keys, set
membership, `state_key` equality) may use ids freely, because they never
leak an order. This is the same rule [02](02_determinism_and_order.md)
§3 and [03](03_data_model.md) §3 already state; parallelism is what makes
violating it *visible*, and the lint that forbids raw-id sorts at
observable sites is what keeps it true.

---

## 2. Level 1 — commitment enterings (the big one)

Phase 2 evaluates 101 independent enterings on exhaustive zebra2 and
3 336+ with `enable_singleton_writeback` off. Each is
`try_commitment_set(root, C)`: fork, write hypotheses, saturate, detect.
Root is never mutated by the entering itself (P1.21 R2), so the work is
embarrassingly parallel.

Except for one thing. The sequential loop **does** write to root
mid-layer: `_handle_dead` calls `_emit_negated_fact_writeback`, which
adds `(not h)` to root when a dead commitment's learned clause is a
singleton. A later entering in the same layer forks a root that now
contains that negative — and can therefore die *pre*-saturation instead
of *post*, or die at all.

So a naive parallel layer changes `enterings_dead_pre` /
`enterings_dead_post` — a T1 failure.

### Speculate, then validate by continuation

```
R0 = Arc::clone(root_core)              // snapshot: free (03 §5)
results = candidates.par_iter().map(|c| try_commitment_set(R0, c)).collect()  // index-ordered
W = {}                                  // root writes committed so far
for (i, c) in candidates.enumerate() {
    r = validate(results[i], c, W)      // see below
    commit(r)                           // stats, nogood emit, writeback -> W
    if stop_after reached { break }     // identical early stop
}
```

`validate(result_i, c, W)` — three cases, and each is decided without a
re-run in the common one:

1. **`W = ∅`** (nothing was written back yet this layer). `result_i` was
   computed against exactly the root the sequential engine would have
   used. Accept. *This is the whole of layer 1*, where every learned
   clause is the candidate itself, so a writeback can only concern the
   candidate that just died.
2. **`c ∩ {h : (not h) ∈ W} ≠ ∅`.** The sequential engine would have
   found the pre-saturation contradiction immediately. Emit `dead-pre`
   with the frontier computed from the clash, no saturation needed —
   which is precisely what `try_commitment_set`'s pre-check does.
3. **Otherwise** — `W` is non-empty but disjoint from `c`. The only way
   `W` changes the outcome is if fork *i*'s saturation could have
   *consumed* one of those `(not h)` facts. Rather than re-running, take
   the fork's saturator (still alive, with its queues) and **continue it
   with `W` as the delta** — the same semi-naive seeding the closure
   already uses ([06](06_saturation.md)). If nothing new is derived and
   no contradiction appears, `result_i` stands and the continuation cost
   is one delta pass; if something is derived, the continuation *is* the
   corrected result.

Case 3 is exact because the KB is append-only and saturation is a least
fixpoint: `sat(base ∪ W ∪ c) = sat(sat(base ∪ c) ∪ W)`. The engine
already relies on that identity — it is the same argument behind
`is_stalled()` re-enqueueing after external writes, and behind
fail-fast's "inconsistent at firing *n* ⇒ inconsistent at the fixpoint".

**Cost.** Case 1 is free and covers layer 1. Case 2 is a bitset test.
Case 3 costs a delta pass over a handful of facts and only fires when a
singleton writeback happened earlier *in the same layer*, which on
zebra2 means layer 2 only. Measured re-validation rate is a
[P1a.7](../p1a.7_parallelism/README.md) acceptance number.

**Memory.** N concurrent forks hold N deltas over one shared `Arc<KbCore>`.
Per-fork delta on zebra2 is tens of facts, so `--jobs 16` costs kilobytes
— the whole reason [03](03_data_model.md) §5 exists.

**Early stop.** `stop_after` must cut at the same candidate the
sequential engine would. Committing in candidate order and breaking there
does that; speculative work past the cut is discarded (a bounded waste,
capped by the job count).

---

## 3. Level 2 — the enqueue pass

`_enqueue_pass` is **read-only over the KB**: it runs matchers and pushes
onto the queue. Two parallel shapes:

- **full pass** (cold start, `is_stalled`) — one task per plan;
- **delta pass** — one task per `(delta fact, plan)` pair from
  `pos_index`.

Determinism comes from the merge, not the execution: each task collects
its matches into its own buffer, buffers are concatenated in the
canonical order (`cache` order for a full pass; delta-fact order then
plan order for a delta pass), and **tiebreakers are assigned during the
merge**. Since `_tiebreaker` is just a monotone counter, assigning it
after the fact reproduces the sequential sequence exactly.

Worth doing only above a work threshold (root saturation of a large KB);
a threshold changes nothing observable, so it can be tuned freely.

## 4. Level 3 — the boundary round

`_admit_from_boundary` evaluates parked candidates' guards against a
quiesced world — read-only, and 72 % of an exhaustive solve's time
([06](06_saturation.md) §2). Parallel shape: evaluate the guards of all
*dirty* parked candidates concurrently, then scan the results in
priority/FIFO order and admit the first whose guards pass.

Identical to sequential because:

- the world does not change during a round (at most one admission, and
  it ends the round);
- `first_failing` is per-candidate and per-guard-order, so each result is
  independent;
- the *choice* is made by the ordered scan afterwards, not by which task
  finished first.

The only waste is evaluating candidates after the eventual winner; bound
it by evaluating in chunks of `jobs` in priority order and stopping at
the first chunk that contains a pass.

## 5. Level 4 — process level

Independent work with no shared mutable state: the conformance corpus
runner, the fuzzer, `feature_matrix`, and any embedder that drives
several engines at once — [M1b](../../m1b_gui/README.md)'s GUI holding
one session per open puzzle is the concrete one. Level 4 needs nothing
from the engine beyond `Send + Sync` on the shared `Arc<KbCore>` /
`Arc<Program>` / `Arc<PlanMemo>`, which levels 1–3 already require.

---

## 6. What must be `Sync`, and how

| shared state | strategy |
|---|---|
| `KbCore`, `Program` (relations/rules/macros/query/config) | immutable after publication; `Arc`, no lock |
| `Interner` | sharded `RwLock` (read-mostly; writes are rare after load) — or per-thread staging with a merge at join, if contention shows |
| `FactStore` | append-only; a lock-free segmented vec (`boxcar`-style) plus a sharded `RwLock` on the lookup map |
| `PlanMemo` | append-only, keyed by `(rule, activator)`; `RwLock` with double-checked insert (compiles are rare — see [06](06_saturation.md) § Win A) |
| `_nogoods` on root | written only at commit time, on the committing thread — no sharing needed |
| stats | accumulated at commit time — no atomics, no contention |

Note what is *not* on this list: no shared mutable KB, no shared queue,
no shared `_seen`/`_fired`. Each fork owns its saturator state
outright. That is what makes the design safe rather than merely fast.

---

## 7. Rejected designs

| idea | why not |
|---|---|
| parallel **depth-first** search with work stealing across layers | the engine's search is a cardinality-BFS by construction; going depth-first changes which no-goods exist when, i.e. the pruning, i.e. the counters |
| batching boundary admissions across threads | unsound, and for the same reason it is unsound sequentially (`p ← absent q; q ← absent p`) — see [06](06_saturation.md) §1 |
| parallelising the hypgen filter pipeline | `enable_lookahead_kill_cache` makes filtering feed forward: a killed candidate writes `(not h)` that later candidates in the *same call* observe ([07](07_search_layer.md) §2). Parallelising it changes `HypGenStats` attribution. Could be revisited with the cache disabled, which is a different computation. |
| SIMD unification | the inner loop is pointer-chasing over candidate buckets, not a dense array scan. Revisit only if a bucket-major layout lands in [P1a.6](../p1a.6_performance/README.md). |
| GPU offload | no. |

---

## 8. Acceptance for this design

- `--jobs {1,2,4,8,16}` on the whole corpus: T3-identical output for
  every N (with `--timing`/wall-clock normalised).
- A stress test: 10 000 randomised `--jobs 8` runs of the corpus, diffed
  against the `--jobs 1` run, with no divergence.
- Scaling: ≥ 6× on 8 cores for exhaustive zebra2's Phase 2, measured as
  layer-2 wall-clock (layer 1 is 34 alive + 67 dead — enough to scale).
- Re-validation rate (case 3 in §2) reported per run under
  `--stats`-adjacent diagnostics; if it exceeds a few percent, the
  read-set tracking gets refined before the mode ships.
- Thread-sanitizer and `loom`-style model checks on the interner /
  fact-store shards.

## Cross-links

- [03 — Data model](03_data_model.md) §5 — `Arc<KbCore> + Delta`, the
  precondition for all of this.
- [06 — Saturation](06_saturation.md) — levels 2 and 3 live here.
- [07 — Search layer](07_search_layer.md) — level 1 lives here.
- [12 — Toolchain & layout](12_toolchain_and_layout.md) §2 — the crate
  boundaries level 4's consumers link against.
