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
   used. Accept. *This is the whole of every layer above the first* —
   see § Which layers have a `W` at all.
2. **`c ∩ {h : (not h) ∈ W} ≠ ∅`.** The sequential engine would have
   found the pre-saturation contradiction immediately. Emit `dead-pre`
   with the frontier computed from the clash, no saturation needed —
   which is precisely what `try_commitment_set`'s pre-check does.
   [S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md) measured
   this case at **0 occurrences in 1 078 704 enterings**: layer 1's
   candidates are distinct singletons, so a `(not h_j)` written by an
   earlier death cannot name a later candidate, and no layer above has a
   `W`. It stays in the design because a future `W` writer need not have
   that shape; it is not a case to optimise for.
3. **Otherwise** — `W` is non-empty but disjoint from `c`. The only way
   `W` changes the outcome is if fork *i*'s saturation could have
   *consumed* one of those `(not h)` facts. Rather than re-running, take
   the fork's saturator (still alive, with its queues) and **continue it
   with `W` as the delta** — the same semi-naive seeding the closure
   already uses ([06](06_saturation.md)). If nothing new is derived and
   no contradiction appears, `result_i` stands and the continuation cost
   is one delta pass; if something is derived, the continuation *is* the
   corrected result.

### Which layers have a `W` at all

The search is a cardinality BFS: layer *L* enters commitment **sets of
size L**. A dead commitment `{h_1 … h_L}` licenses `¬(h_1 ∧ … ∧ h_L)` — a
clause of width *L*, and a clause is not a fact. It goes into the no-good
store, where it prunes later candidate *generation*.

At *L = 1*, and only there, that clause is a **unit**: `¬h_1`, which *is*
a fact, and root gains `(not h_1)`. That is the writeback, and it is why
both engines guard it on the **commitment's length** rather than on
anything about the clause — `c.len() == 1` in `solve.rs`'s `handle_dead`,
`len(c) == 1` in `_helpers.py`'s `_handle_dead`. Neither minimises a
learned clause below the commitment (`learned_clause = frozenset(c)`), so
"the learned clause is a singleton" and "the commitment is a singleton"
are the same condition.

Nothing else adds a **fact** to root inside a layer: `try_commitment_set`
is pure with respect to root (P1.21 R2) and only forks it; `complete`,
`record_node` and `check_commutativity` write into the *fork*;
`order_candidates` and `emit_nogood` take `&Kb`; and `compute_alive` /
`promote_forced_positives` run **between** layers. So:

> **Layer 1 is the only layer that adds a fact to root mid-layer. Every
> layer above it is case 1 by construction, and its validator is dead
> code.**

The one thing that *is* mutated mid-layer at every level is the **no-good
store**, which forks share by `Arc` rather than copy. It is harmless here
for a reason worth stating rather than assuming: **no fork reads it while
saturating.** Its only readers are `generate_layer`, at layer start, and
`emit_nogood`'s own subsumption check, at commit time on the committing
thread. A no-good emitted mid-layer therefore cannot change what any fork
of that layer derives — it changes the *next* layer's candidate list, and
the ordered commit already serialises that.

Measured rather than argued —
[S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md) counts
`writeback` events by layer (`{1: 32}` on `zebra2 -e`, `{1: 31}` on
`zebra -e`, `{1: 162}` on `branching/07 -e`) and finds every case 3 in
layer 1. This is the inverse of what an earlier draft of case 1 above
said, and the difference matters twice over: the parallel path needs a
*debug assertion* that no root write reaches a layer above the first, and
the workloads with a search big enough to want cores put **98.2–99.9 % of
their enterings past layer 1**
([scaling.md §2](../p1a.7_parallelism/scaling.md#2-the-layer-profile--where-the-enterings-and-the-firings-are)).

### What case 3 costs — measured before it was built

The rate is **0.1 % corpus-wide and 36–50 % on the zebra family**, and on
**35** enterings the speculation returns `alive` where the sequential
engine returns `dead-post`: a mid-layer `(not h)` is a *premise* of
`std.elim`, whose `domain-elimination` asserts a **positive** once every
other value is excluded and whose `no-room-left` asserts `(false)` once
every value is. A fork without the accumulated writebacks fires neither. Case 3's continuation is therefore load-bearing, not
a formality.

And the identity below is about **fixpoints**, which
`enable_fail_fast_fork` means a dying fork never reaches. With fail-fast
off, the speculation's `core` errors collapse exactly onto its `kind`
errors (35 = 35); with it on, 40 further cores differ purely because the
two forks stopped at different firings of the same death. So a
continuation recovers `kind` — a fork inconsistent at firing *n* is
inconsistent at the fixpoint, and `W` only adds facts — but recovers
`core` only where the fork ran to quiescence.
[S1a.7.2](../p1a.7_parallelism/s1a.7.2_parallel_enterings.md) has to
settle that interaction before `--jobs N` can claim T1.

Case 3 is exact because the KB is append-only and saturation is a least
fixpoint: `sat(base ∪ W ∪ c) = sat(sat(base ∪ c) ∪ W)`. The engine
already relies on that identity — it is the same argument behind
`is_stalled()` re-enqueueing after external writes, and behind
fail-fast's "inconsistent at firing *n* ⇒ inconsistent at the fixpoint".

**Cost.** Case 1 is free and covers every layer above the first. Case 2
is a bitset test. Case 3 costs a delta pass over a handful of facts
(`|W| ≤ 32` on the zebras, ≤ 161 corpus-wide) and fires only where a
singleton writeback happened earlier *in the same layer*, which means
layer 1 and only layer 1. Measured re-validation rate is a
[P1a.7](../p1a.7_parallelism/README.md) acceptance number, and
[S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md) took it
before the mechanism was built.

**Memory.** N concurrent forks hold N deltas over one shared `Arc<KbCore>`.
Per-fork delta on zebra2 is tens of facts, so `--jobs 16` costs kilobytes
— the whole reason [03](03_data_model.md) §5 exists.

**Early stop.** `stop_after` must cut at the same candidate the
sequential engine would. Committing in candidate order and breaking there
does that; speculative work past the cut is discarded (a bounded waste,
capped by the job count).

---

## 2a. Deferred integration — the batch-synchronous layer

§2 speculates and then *repairs*. There is a second shape, and it is the one a
parallel layer has whether or not anybody designs it: **test a batch of
candidates against one KB, then integrate what the whole batch learned.**
S1a.7.0 built it (`SolveOptions::integrate_every`) and measured it, because it
is cheaper to answer "does the answer survive this?" with a test than with an
argument.

### The objects

| symbol | what |
|---|---|
| `B` | root's fact set when the layer opens; already a fixpoint, `sat(B) = B` |
| `C = (c_1 … c_m)` | the layer's candidates, in canonical order; `c_i` a set of hypothesis facts |
| `sat(X)` | the engine's rule fixpoint over `X` — **monotone**, **inflationary** (`X ⊆ sat(X)`), **idempotent** |
| `dead(X)` | `X` holds a contradiction. **Monotone**: `X ⊆ Y ∧ dead(X) ⇒ dead(Y)`, because the KB is append-only and nothing retracts |
| `W_i` | `{ ¬h : c_j = {h} for some j < i that died }` — the writebacks a candidate can see |

Under **immediate** integration candidate *i* is entered against `B ∪ W_i`.
Under **deferred** integration with barriers at `β`, it is entered against
`B ∪ W_{β(i)}`, and `W_{β(i)} ⊆ W_i`.

### Four claims

**(1) A writeback prunes; it does not decide.** If `sat(B ∪ {h})` is dead then
for every `c ⊇ {h}`, `sat(B ∪ c) ⊇ sat(B ∪ {h})` by monotonicity, so `c` dies
whether or not `¬h` is at root. The writeback makes that death *cheaper* and
*earlier*, never *possible*.

**(2) …but it also derives.** `¬h` is a fact, and rules read it: `std.elim`'s
`domain-elimination` matches `(forall ?v_other … (not (?R ?a ?v_other)))` and
**asserts a positive**; `no-room-left` asserts `(false)`. So

> `sat(B ∪ c) ⊆ sat(B ∪ W ∪ c)`, and the inclusion is **strict** in general.

Which gives the asymmetry the whole section turns on:

> - `dead(sat(B ∪ c))` **⇒** `dead(sat(B ∪ W ∪ c))` — a death under a
>   *smaller* root is a real death;
> - `dead(sat(B ∪ W ∪ c))` **⇏** `dead(sat(B ∪ c))` — an **alive** verdict
>   under a smaller root is *provisional*.

S1a.7.0 measured the second line on the corpus: **35 enterings** come back
`alive` from `B ∪ c` where `B ∪ W ∪ c` says `dead-post`.

**(3) The commutation identity.** For `W` a set of facts (never a retraction):

> **`sat(B ∪ W ∪ c) = sat( sat(B ∪ c) ∪ W )`**

*⊇*: `B ∪ c ⊆ B ∪ W ∪ c` gives `sat(B ∪ c) ⊆ sat(B ∪ W ∪ c)`, and
`W ⊆ sat(B ∪ W ∪ c)`; so `sat(B ∪ c) ∪ W ⊆ sat(B ∪ W ∪ c)`, and applying
`sat` with idempotence gives `sat(sat(B ∪ c) ∪ W) ⊆ sat(B ∪ W ∪ c)`.
*⊆*: `B ∪ W ∪ c ⊆ sat(B ∪ c) ∪ W` because `B ∪ c ⊆ sat(B ∪ c)`; apply `sat`. ∎

This is what licenses §2's case 3 — feed the quiesced fork `W` as a delta and
it lands on exactly the fork the sequential engine had. It is the same
identity behind `is_stalled()`'s re-enqueue after an external write, and the
mechanism is the one [S1a.6.9](../p1a.6_performance/s1a.6.9_fork_entry_delta.md)
already built (`Saturator::resume`).

**It is an identity about *fixpoints*, and `enable_fail_fast_fork` means a
dying fork never reaches one.** So the continuation recovers `kind` (claim 1)
and recovers the *fixpoint* (claim 3), but the **unsat core** it computes is
read off wherever fail-fast stopped, and that is a different firing in the two
runs. Measured: with fail-fast off, the speculation's `core` errors collapse
exactly onto its `kind` errors (35 = 35); with it on, 40 further cores differ.

**(4) What deferral does to the answer.** By claim 2, the deferred alive set is
a **superset** of the sequential one. Three things happen to the extra members,
and only one of them is not automatic:

- **alive ∧ incomplete** → expanded to the next layer. By then the barrier has
  run, `compute_alive` has dropped every refuted element and the no-good store
  filters their supersets, so the branch dies out. **Cost: enterings. Effect on
  the answer: none.**
- **dead** → a real death, by the first line of claim 2's asymmetry: what
  died under the smaller root dies under the bigger one. **Sound as
  recorded.**
- **alive ∧ complete** → recorded as a **solution node**, and that is the one
  provisional verdict that reaches the answer — through `k`, through the
  printed models, and through `stop_after`'s cut.

So the rule is one line:

> **A death found under deferral needs no re-check. A *solution* does.**

`integrate_every` as it stands does **not** re-check, and the model set is
therefore *measured* equal rather than equal by construction:
`ein-infer/tests/search_invariants.rs` compares verdict + model set over **16
files under 4 candidate orders and 3 integration policies**, plus two
five-layer searches (5 173 and 11 501 enterings) under a whole-layer barrier,
plus the composition of a shuffled order with a whole-layer barrier. Making it a theorem costs
one re-entry per recorded solution node at the barrier — solutions are rare,
so this is cheap — and it is [S1a.7.2](../p1a.7_parallelism/s1a.7.2_parallel_enterings.md)'s
to build.

### What it costs, measured

Whole-layer deferral, exhaustive, against the sequential engine — same answer
in every cell:

| workload | enterings | root depth at exit | wall |
|---|---:|---:|---:|
| `zebra2 -e` sequential | 101 | 35 | 37 ms |
| `zebra2 -e` batch 20 | 111 | 5 | 40 ms |
| `zebra2 -e` whole layer | **617** | 3 | 163 ms |
| `zebra -e` sequential → whole layer | 111 → **617** | 34 → 3 | 62 → 273 ms |
| `branching/06 -e` (0 writebacks) | 5 173 → **5 173** | 2 → 2 | 263 → 259 ms |
| `branching/07 -e` (162 writebacks) | 11 501 → **11 501** | **164 → 3** | **1 135 → 406 ms** |

Three readings, and the third was not expected:

1. **The cost of deferring is exactly the prune it defers.** On the zebras the
   singleton writeback is doing enormous work in layer 1 — 6.1× the enterings
   without it — and batching at 20 recovers almost all of it.
2. **On the workloads that want cores it costs nothing.** `branching/06` has no
   writebacks at all and `branching/07`'s prune nothing: same entering count to
   the unit.
3. **On `branching/07` deferral is 2.8× *faster*, single-threaded.** Every root
   write seals another layer (`Kb::fork` seals the top so the parent's later
   appends land in a new one) and **every fork inherits the whole stack**: 162
   mid-layer writebacks put root at **depth 164**, and all 11 501 forks walk
   it. A barrier coalesces them — depth 164 → 3 — for the same enterings and
   the same answer. That is a P1a.6-shaped finding that fell out of a P1a.7
   correctness experiment, and it is pinned by
   `deferring_collapses_roots_layer_stack`.

### Order

The other half of what a parallel layer needs is that the *order* of the
candidates does not reach the answer. That is not new — it is what `--shuffle`
has always claimed (Q-M1a.5) — but the claim was only ever exercised through
the traversal-parity sweep, which compares two runs of the *same* engine
against ein.py. `the_answer_does_not_depend_on_the_entering_order` asserts the
invariant directly: `lex`, `score-sum` and two seeded shuffles, same verdict
and same model set.

Order-invariance and integration-invariance **compose**, which is the property
a parallel layer actually uses: it enters a batch in whatever order the workers
finish and integrates at the barrier.

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

> **Corrected 2026-08-22 by measurement**
> ([S1a.7.1](../p1a.7_parallelism/s1a.7.1_sync_shared_state.md) T1a.7.1.0,
> [shared_state.md](../p1a.7_parallelism/shared_state.md)). Three of the six
> rows below were *write* strategies, and the write rate had never been taken.
> Two of the three are wrong, and the table now carries what was measured
> beside what was assumed. The corrections are struck through rather than
> deleted, because the reason a design was wrong is worth more than the design.

| shared state | strategy | measured |
|---|---|---|
| `KbCore`, `Program` (relations/rules/macros/query/config) | immutable after publication; `Arc`, no lock | ✅ true by construction |
| `Interner` | ~~sharded `RwLock` (read-mostly; writes are rare after load)~~ | **no lock.** Writes during a *search* are not rare, they are removable: four distinct names arrived after root saturation across the whole corpus, three of them the engine's own (now `ein_core::terms::ENGINE`) and one a rule's argument constant (now interned at load). The integer pool never grew at all. `&Interner` is `Sync`, `text` keeps returning a borrow, and no read site changes. `ein-infer/tests/interning.rs`, 0 of the 90 corpus files that solve |
| `FactStore` | ~~append-only; a lock-free segmented vec (`boxcar`-style) plus a sharded `RwLock` on the lookup map~~ | **the append is not the problem; the read is.** A search assigns **41 to 417** fact ids — `features/01 -e` is 41 across 384 167 enterings — against 5.8–26 M borrow-returning reads. A mutex on the append is free. What no lock can do is return the `&[Value]` that `args` returns and the `&Row` that `row` returns, on the path carrying the 26 M |
| `PlanMemo` | append-only, keyed by `(rule, activator)`; `RwLock` with double-checked insert (compiles are rare — see [06](06_saturation.md) § Win A) | ✅ shipped as `Arc<Mutex<PlanMemo>>` at [S1a.6.8](../p1a.6_performance/s1a.6.8_compile_cache_and_extents.md), for an unrelated reason |
| `_nogoods` on root | written only at commit time, on the committing thread — no sharing needed | — |
| stats | accumulated at commit time — no atomics, no contention | — |

Note what is *not* on this list: no shared mutable KB, no shared queue,
no shared `_seen`/`_fired`. Each fork owns its saturator state
outright. That is what makes the design safe rather than merely fast.

**And one thing that was on it and should not have been.** The event sink is
`Rc<RefCell<Vec<u8>>>` (`ein-infer/src/events.rs`), which is not `Send` — a
worker cannot hold one. It needs the same treatment as the stats: a per-worker
buffer merged in commit order, so the stream a reader sees is the sequential
one. §3's "no shared queue" hid it, because a sink is not a queue.

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
