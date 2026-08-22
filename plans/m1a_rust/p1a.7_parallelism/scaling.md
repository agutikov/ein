# P1a.7 — the scaling measurements

What the phase is chosen by, in the shape
[baseline.md](../p1a.6_performance/baseline.md) established for P1a.6: the
numbers first, the argument after, and every section reproducible from one
command.

**Machine.** Intel i9-14900HX — **8 P-cores** (`cpu0–15`, SMT, 5.6–5.8 GHz)
and **16 E-cores** (`cpu16–31`, 4.1 GHz, no SMT); `powersave`, turbo on;
Linux 7.1.8 / Manjaro. Single-core cells are pinned to `cpu4` through
[`utils/bench_env.sh`](../../../utils/bench_env.sh), best-of-N, as P1a.6's
were.

> **The measurement policy this phase needs — and now has.**
> `bench_env.sh` pinned to *one* P-core hyperthread, which is exactly right for
> P1a.6 and exactly wrong here. On a hybrid CPU "8 cores" is three different
> machines — 8 P-cores, 8 P-core *threads* on 4 physical cores, or 8 E-cores —
> and a scaling table that does not say which is not a measurement. Every
> `--jobs N` number in this file must name its core set.
>
> **`bench_env.sh --cores` shipped at [S1a.7.1](s1a.7.1_sync_shared_state.md)**
> (2026-08-22). `P:N` takes one thread per physical P-core, `PT:N` takes P-core
> threads in cpu order, `E:N` / `ET:N` the same for the E-cores, and a literal
> list is passed through with each core classified. The fingerprint prints the
> resolved list *and* the number of distinct physical cores it covers — the two
> lines below are the same "8 cores" and are not the same machine —
>
> ```text
>   pinned to       cpu0,2,4,6,8,10,12,14 — 8 cpu(s), 8 physical core(s), all P
>   pinned to       cpu0,1,2,3,4,5,6,7    — 8 cpu(s), 4 physical core(s), all P
> ```
>
> and a spec the machine cannot fill is refused rather than quietly reduced.
> [S1a.7.5](s1a.7.5_jobs_contract.md) T1a.7.5.5's scaling table is unblocked.

---

## 1. Amdahl — what a solve spends its time on

`ein solve <file> -e -t`, release + snmalloc, `taskset -c 4`, 2026-08-20,
`master` @ `2cbcef1`. "hypothesis search" is Phase 2 — everything levels 1–3
could touch; the rest is the serial residue no `--jobs` can move.

| workload | parse | kb load | root sat | **hyp search** | end-to-end | search share |
|---|---:|---:|---:|---:|---:|---:|
| `zebra2.ein -e` | 0.61 ms | 1.06 ms | 2.57 ms | **26.82 ms** | 31.06 ms | 86.3 % |
| `zebra.ein -e` | 0.11 | 0.72 | 1.46 | **44.56** | 46.85 | 95.1 % |
| `branching/06_lookahead_on -e` | — | — | — | **211.25** | 211.65 | 99.8 % |
| `saturation/square-bwd/houses -e` | — | — | — | **271.17** | 271.27 | 100.0 % |
| `branching/07_lookahead_off -e` | — | — | — | **906.39** | 906.58 | 100.0 % |
| `features/01_not_and_absent -e` | — | — | — | **1 856.06** | 1 856.19 | 100.0 % |

Two readings:

- **The serial residue is small everywhere and negligible on the big
  workloads.** Amdahl is not what bounds this phase.
- **The two zebras are 29 and 47 ms.** After P1a.6 took them 165× off PyPy
  — a ratio whose denominator is a [frozen
  constant](../p1a.6_performance/baseline.md), the Python engine being gone —
  the absolute room left in them is a few tens of milliseconds — and §2 shows
  that less than half of it is in the part that parallelises exactly. The
  phase's stated scaling target (`≥ 6× on 8 cores for exhaustive zebra2's
  Phase 2`) was written when that run was 4.5 s.

## 2. The layer profile — where the enterings and the firings are

`--events`, `e == "enter"` counted by `layer`, and `e == "writeback"` likewise.

| workload | enterings | layers | layer 1 | layer ≥ 2, enterings | layer ≥ 2, firings | writebacks |
|---|---:|---:|---:|---:|---:|---:|
| `zebra2 -e` | 101 | 2 | 56 (5 672 firings) | **44.6 %** | **42.3 %** | 32, all layer 1 |
| `zebra -e` | 111 | 2 | 56 (12 471) | **49.5 %** | **53.2 %** | 31, all layer 1 |
| `branching/06 -e` | 5 173 | 5 | 42 (340) | **99.2 %** | **99.6 %** | 0 |
| `branching/07 -e` | 11 501 | 5 | 204 (542) | **98.2 %** | **99.8 %** | 162, all layer 1 |
| `sq-bwd/houses -e` | 21 699 | 5 | 20 (5) | **99.9 %** | **100.0 %** | 0 |

This table is the phase's most important one, and it says two things at once.

**Every mid-layer root *fact* write is in layer 1**, and the reason is the
shape of a learned clause rather than anything about layers. The search is a
cardinality BFS — layer *L* enters commitment **sets of size L** — so a dead
commitment `{h_1 … h_L}` licenses `¬(h_1 ∧ … ∧ h_L)`, a clause of width *L*.
A clause is not a fact; it goes to the no-good store, where it prunes later
candidate *generation*. At *L = 1*, and only there, that clause is a **unit**:
`¬h_1` *is* a fact, and root gains `(not h_1)`. Both engines guard the
writeback on the **commitment's length** for exactly that reason —
`solve.rs:908` `c.len() == 1`, `_helpers.py:454` `len(c) == 1` — and neither
minimises a learned clause below the commitment (`learned_clause =
frozenset(c)`), so "the learned clause is a singleton" and "the commitment is
a singleton" are one condition.

So layers ≥ 2 gain no fact between the layer opening and closing, which makes
every entering there **case 1** of
[design/08](../design/08_parallelism.md) §2: parallel, exact, and needing no
validator at all. The `writeback` column is the empirical half of the same
claim, and §3's case column is its consequence.

**What *is* mutated mid-layer at every level is the no-good store** — shared
across forks by `Arc`, not copied. It is harmless because **no fork reads it
while saturating**: its only readers are `generate_layer`, at layer start, and
`emit_nogood`'s subsumption check, at commit time — and `emit_nogood` takes
`&Kb`, so it structurally cannot add a fact. Everything else in the loop is
fork-local (`complete`, `record_node`, `check_commutativity`) or between
layers (`compute_alive`, `promote_forced_positives`).

**And layers ≥ 2 are where the work is** — on everything except the two
zebras, which are also the only two workloads fast enough not to need cores.
`branching/07 -e` puts 99.8 % of its firings past layer 1; `zebra2 -e` puts
42.3 %.

## 3. The audit

[S1a.7.0](s1a.7.0_speculation_audit.md)'s instrument: the sequential engine,
and beside every entering the same entering re-run against `R0` — root as it
stood when the layer opened. `utils/spec_audit.py`, `spec-audit` build.

### Corpus-wide

73 runs over 69 `positive` / `stdlib` entries, `solve` and `solve -e`,
fail-fast on, 90 s per cell:

| | count | rate |
|---|---:|---:|
| enterings speculated and compared | 1 078 704 | |
| case 1 — `W` empty (**the control**) | 1 078 154 | 99.9 % |
| case 2 — `c` meets `¬W` | **0** | 0 % |
| case 3 — `W` disjoint from `c` | 550 | **0.1 %** |
| `kind` moved | 35 | 0.003 % |
| `core` moved | 115 | 0.011 % |
| alive fork's state moved | 107 | 0.010 % |
| **control failures** | **0** | |

**The control is the strongest line in the table.** 1 078 154 speculations that
fork the same root as the sequential arm, and not one of them differed — which
is the corpus-scale form of the property level 1's whole safety argument rests
on: `try_commitment_set` is pure with respect to root. `commitment.rs` asserted
it on one fixture; this asserts it on every entering the corpus has.

**Case 2 never happened.** Layer 1's candidates are distinct singletons, so a
`(not h_j)` written back by an earlier death can never name a later candidate;
and layers ≥ 2 have no `W` at all. The design's cheapest case is dead code on
this corpus, which is worth knowing before it is written.

### Per run — and why the corpus average is the wrong number

| run | case 3 | rate | `kind` moved |
|---|---:|---:|---:|
| `solve -e examples/zebra2-hints.ein` | 35 / 36 | **97.2 %** | 7 |
| `solve -e examples/zebra2.ein` | 50 / 101 | **49.5 %** | 14 |
| `solve -e examples/zebra.ein` | 49 / 111 | **44.1 %** | 13 |
| `solve examples/zebra.ein` | 6 / 13 | 46.2 % | 1 |
| `solve examples/zebra2.ein` | 5 / 11 | 45.5 % | 0 |
| `solve -e examples/branching/07_lookahead_off.ein` | 202 / 11 501 | 1.8 % | 0 |
| the other 65 runs | 0 | 0 % | 0 |

The phase's acceptance says the re-validation rate must be "≤ a few percent".
Corpus-wide it is **0.1 %** and the criterion passes; on every workload a
reader of the milestone would recognise it is **36–50 %**. A criterion that an
average can satisfy while the puzzles it was written for fail it is not a
criterion, and the phase README restates it per workload.

### What moves, and how much of it is fail-fast

The zebra family (7 entries, 9 runs, 399 enterings), both arms, 300 s cells:

| | fail-fast **on** | fail-fast **off** |
|---|---:|---:|
| case 3 | 146 (36.6 %) | 146 (36.6 %) |
| `kind` moved | **35 (8.8 %)** | **35 (8.8 %)** |
| `core` moved | 75 (18.8 %) | **35 (8.8 %)** |
| alive fork's state moved | 25 (11.2 % of 223) | 25 (11.2 %) |
| state moved past `W` | 106 (26.6 %) | 60 (15.0 %) |
| firing count moved | 109 (27.3 %) | 60 (15.0 %) |

With fail-fast off, **`core` moved collapses exactly onto `kind` moved**. That
decomposition is the finding: 40 of the 75 core divergences are not
disagreements about the answer at all, they are the two forks stopping at
different firings of the same one — `enable_fail_fast_fork` halts a dying fork
at the firing that killed it, and a fork that already holds `W` dies sooner.

The consequence for [S1a.7.2](s1a.7.2_parallel_enterings.md) is sharper than
design/08 §2 anticipated:

- the case-3 **continuation recovers `kind`** for exactly the reason fail-fast
  is sound — a fork inconsistent at firing *n* is inconsistent at the fixpoint,
  and `W` only adds facts;
- it recovers **`core` only where the fork runs to its fixpoint**, because a
  continued fork's firing order is not the sequential one and fail-fast reads
  the order;
- so **`enable_fail_fast_fork` × speculation is the phase's real correctness
  question**, and it appears nowhere in design/08. S1a.7.2 has to answer it
  before `--jobs N` can claim T1.

### The speculation is wrong, not merely stale

`solve -e examples/zebra.ein`, layer 1, entering 11, `|W| = 2`:

```
commitment    (co-located Englishman House-5)
sequential    dead-post, 603 facts, 382 firings
speculative   alive,     590 facts, 370 firings
derived only by the sequential fork
    (co-located Dog House-4)  (co-located House-3 Japanese)
    (co-located House-3 Parliament)  …and 34 more
```

The mid-layer `(not h)` is a **premise** of `std.elim`, not bookkeeping:
`domain-elimination` matches `(forall ?v_other … (not (?R ?a ?v_other)))` and
*asserts a positive* when every other value is excluded (priority 400), and
`no-room-left` asserts `(false)` when every value is. Accumulated writebacks
are what let either fire. A validator that decided this fork was unaffected would move
`enterings_alive`, `enterings_dead_post`, the no-goods emitted, the writeback
set and the next layer's candidate list.

## 3a. Where the writebacks are *inside* layer 1 — and the split that is not there

§3 says case 3 lives only in layer 1. It does not say where in layer 1, and
[S1a.7.2](s1a.7.2_parallel_enterings.md)'s decision turns on that: if `W`
stopped growing early, a layer could run its head sequentially and fan out its
tail with no validator at all — exact, for free. The instrument is the event
stream, so this needed nothing built:

| workload | enterings | layer 1 | writebacks | first | **last** | sequential span | of enterings | of Phase-2 firings |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `zebra -e` | 111 | 56 | 31 | 7 | **55** | 49 | 44.1 % | 40.4 % |
| `zebra2 -e` | 101 | 56 | 32 | 6 | **55** | 50 | 49.5 % | 49.2 % |
| `zebra2-hints -e` | 36 | 36 | 23 | 1 | **35** | 35 | 97.2 % | 93.7 % |
| `branching/07 -e` | 11 501 | 204 | 162 | 2 | **204** | 203 | 1.8 % | **0.24 %** |
| `branching/06 -e` | 5 173 | 42 | **0** | — | — | 0 | 0 % | 0 % |
| `sq-bwd/houses -e` | 21 699 | 20 | **0** | — | — | 0 | 0 % | 0 % |
| `features/01 -e` | 384 167 | 35 | **0** | — | — | 0 | 0 % | 0 % |

*first* / *last* are the layer-1 candidate indices at which `W` first and last
**grew** — a re-issued `(not h)` emits the event without adding a fact and is
not counted. The *sequential span* is `last − first + 1`: the candidates that
can neither be accepted as computed (because `W` is non-empty) nor skipped.

**There is no head/tail split.** `W` grows until candidate 55 of 56, 55 of 56,
35 of 36 and **204 of 204**. The tail a fan-out could take exactly is one
candidate, one, one and zero. Whatever else layer 1 is, it is not
front-loaded — the deaths that write back are spread over the whole of it, and
on `branching/07 -e` the very last candidate of the layer still writes one.

This is worth stating because the opposite was the expectation.
[T1a.7.1.2](s1a.7.1_sync_shared_state.md#task-t1a712--fact-store) found the
*fact-id* appends of a layer clustered in its head — largest within-layer index
6, 21, 83 — and "run the head, fan out the tail" is exactly the mechanism that
finding licensed for the fact store. It does not transfer. The two quantities
are not the same one seen twice: an appending entering interns a proposition
*inside* `try_commitment_set`, and the singleton writeback is a **commit-time
root write** that happens after the entering returns. They have opposite
distributions, and only the measurement says so.

### And the layer that has to be sequential is 0.016 % of the corpus

Every `.ein` under `examples/` and `stdlib/` that produces events, `solve -e`,
20 s per file (5 hit the cap and are counted up to the cut):

| layer | enterings | writebacks |
|---:|---:|---:|
| **1** | **1 343** | **248** |
| 2 | 38 009 | 0 |
| 3 | 1 213 248 | 0 |
| 4 | 5 351 172 | 0 |
| 5 | 1 554 433 | 0 |
| | **8 158 205** | **248** |

design/08 §2 argues from the clause width that only layer 1 can add a fact to
root mid-layer, and S1a.7.0 counted `writeback` events by layer on three files.
This is the same claim over the whole corpus and five layers deep: **248 of 248
writebacks are in layer 1**, and layer 1 holds **0.016 %** of the enterings.

So the question "what happens to layer 1" has a cost attached to every answer
now, and the cheapest answer is affordable: making layer 1 sequential costs
**0.24 % of `branching/07 -e`'s Phase-2 firings and nothing at all on the other
three workloads of the measurement set**, because those three never write back.
By Amdahl a 0.24 % sequential fraction admits 417×, against a phase target of
6×. What it costs the zebra family is 40–94 % — and the zebra family is the
*parity* cell set, not the measurement set (§5.4).

---

## 4. Deferred integration — the shape a parallel layer actually has

§3 audits [design/08](../design/08_parallelism.md) §2's *speculate and repair*.
There is a second shape, and it is the one a parallel layer has whether or not
anybody designs it: **test a batch of candidates against one KB, then integrate
what the whole batch learned.** `SolveOptions::integrate_every` is that mode —
`None` is the sequential engine, `Some(n)` puts a barrier every *n* enterings,
`Some(usize::MAX)` puts one at each layer end.

The argument, with the commutation identity and the one case that needs a
re-check, is [design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer).
The numbers are here. Every cell below produced the **same verdict and the same
model set** as the sequential run — that is
`ein-infer/tests/search_invariants.rs`: **16 files under 4 candidate orders and
3 integration policies**, plus two five-layer searches under a whole-layer
barrier, plus the composition of the two.

`ein-infer/examples/defer_probe.rs` is the instrument and this is its output.
"depth" is `Kb::depth()` at exit — root's layer stack, one sealed layer per
write burst.

| workload | sequential | | | barrier every 20 | | | one barrier per layer | | |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| | ent. | depth | wall | ent. | depth | wall | ent. | depth | wall |
| `zebra2 -e` | 101 | 35 | 37 ms | 111 | 5 | 40 ms | **521** | 3 | 163 ms |
| `zebra -e` | 111 | 34 | 62 ms | 192 | 5 | 101 ms | **617** | 3 | 273 ms |
| `zebra2-hints -e` | 36 | 25 | 13 ms | 42 | 4 | 16 ms | **57** | 3 | 21 ms |
| `branching/04 -e` | 36 | 2 | 1 ms | 36 | 2 | 1 ms | 36 | 2 | 1 ms |
| `branching/06 -e` | 5 173 | 2 | 263 ms | 5 173 | 2 | 262 ms | **5 173** | 2 | 259 ms |
| `sq-bwd/houses -e` | 21 699 | 2 | 349 ms | 21 699 | 2 | 349 ms | **21 699** | 2 | 348 ms |
| `branching/07 -e` | 11 501 | **164** | **1 135 ms** | 11 501 | 13 | 447 ms | **11 501** | **3** | **406 ms** |

Three readings, and the third is a finding rather than a confirmation.

**1. What deferral costs is exactly the prune it defers.** On the zebras the
singleton writeback is doing enormous work in layer 1 — 5.2× and 5.6× the
enterings without it — and batching at 20 recovers almost all of it (1.1× and
1.7×). So batch size is the knob that trades pruning for parallelism, and it is
a per-workload knob, not a constant.

> **Three of the entering cells above were wrong until 2026-08-22**, and this
> is what a measurement document owes: `zebra2 -e` read 617 whole-layer where
> the instrument says **521** — the row below's number, copied one line up —
> and `zebra2-hints -e` read 46/72 where it says **42/57**. Found by re-running
> `defer_probe` before writing §6 next to it. Nothing drifted: the same probe
> on the pre-[S1a.7.1](s1a.7.1_sync_shared_state.md) build prints today's
> numbers, so these were transcription slips on the day, not an engine change.
> The wall-clock and depth columns reproduce, the reading above is 5.2× rather
> than 6.1×, and the conclusion — deferral is rejected because it moves a
> search counter at all — never depended on which multiple it was.

**2. On the workloads that want cores it costs nothing.** `branching/06 -e` has
no singleton writeback at all; `branching/07 -e` has 162 and they prune
nothing. Both are entering-identical to the unit under a whole-layer barrier.
That is the same split §2 found from the other side: the deep searches put
98–100 % of their enterings in layers that never write to root, so there is
nothing there for a barrier to delay.

**3. Deferral is 2.8× *faster* on `branching/07 -e`, single-threaded — and the
depth column is why.** Every root write seals another layer: `Kb::fork` seals
the top so the parent's later appends land in a new one, and **every fork
inherits the whole stack**. 162 mid-layer writebacks put root at **depth 164**,
and all 11 501 forks walk it. A whole-layer barrier coalesces them into one
write burst — **depth 164 → 3** — for the same 11 501 enterings and the same
answer, and the run goes 1 135 → 406 ms. Batching at 20 lands between:
depth 13, 447 ms. The zebras collapse the same way (35 → 3, 34 → 3) from a
starting depth an eighth the size, which is why nothing was visible there.

That last one is a P1a.6-shaped result found by a P1a.7 correctness experiment,
and it is worth stating as its own claim, because it does not need parallelism
to be useful:

> **A root write costs every later fork a layer.** Coalescing the writes of a
> layer is worth 2.8× on the corpus's deepest writeback-heavy search, at
> `--jobs 1`.

Whether the *sequential* engine should coalesce is not this phase's call — it
changes the traversal, so it is a `--jobs`-scoped or `--unordered`-scoped
decision, not a free optimisation. It is recorded here so
[S1a.7.2](s1a.7.2_parallel_enterings.md) chooses with it in hand.

> **What it chose** (2026-08-22): not deferral, and not nothing. Deferral is
> rejected as the *parallelism* mechanism because it moves `enterings_total`
> — 101 → 521 whole-layer, 111 at batch 20 on `zebra2 -e` — and a search
> counter that differs between `--jobs 1` and `--jobs N` is the one thing the
> restated acceptance does not admit. But the 2.8× above is **not** a
> deferral result: the entering count is identical on `branching/07 -e`, so
> all of it is the depth column. The route that takes the depth win without
> deferring a single prune is to **flatten root at the layer barrier**, which
> is answer-neutral for a reason that already has a test — `Kb::depth()`
> reaches instruments only, never output, and `check_layering` is the
> standing invariant that a flattened KB and a layered one agree. It recovers
> the win for every layer above the first (11 297 of `branching/07 -e`'s
> 11 501 forks) and leaves layer 1's own 204 paying a growing stack.
> [S1a.7.2](s1a.7.2_parallel_enterings.md) T1a.7.2.0 measures it first,
> because it is two lines and it is worth 2.8× at `--jobs 1`. **It is 3.2×**,
> and § 6 is where it went.

### The `stop_after` caveat

Every number above is exhaustive. Under `stop_after`, a deferred layer records
a solution node from a *provisional* alive verdict, which is the one case
design/08 §2a says needs a barrier re-check — so a `-n 1` run under deferral is
not covered by the tests above and is not claimed here.

## 5. What this chooses

1. **Levels 2 and 3 are unaffected** — they live inside one fork's saturation
   and have no cross-entering dependency. [S1a.7.3](s1a.7.3_parallel_boundary.md)
   and [S1a.7.4](s1a.7.4_parallel_enqueue.md) stand as written.
2. **Level 1 splits at the layer boundary.** Layers ≥ 2: parallel, no
   validator, exact — and 98–100 % of the work on every workload that needs
   cores. Layer 1: a real dependency chain, a writeback every ~1.8 enterings on
   the zebras, and a speculation that is wrong 1 time in 8.
3. **Layer 1 runs sequentially**, decided 2026-08-22 at
   [S1a.7.2](s1a.7.2_parallel_enterings.md) § The decision. §3a is why: `W`
   grows to the second-to-last candidate of the layer, so there is no exact
   head/tail split to take, and the layer that has to be serial is 0.016 % of
   the corpus's enterings and 0.24 % of `branching/07 -e`'s firings — nothing
   at all on the other three workloads of the measurement set, which never
   write back. Continue-and-validate is not built: with no fanned-out layer
   ever seeing a `W`, design/08 §2's three cases reduce to case 1 and the
   fail-fast interaction above **does not arise**.
4. **The scaling target moves.** `zebra2 -e`'s Phase 2 is 26.8 ms and 42.3 %
   of its firings are in the exactly-parallel part — ~11 ms, if firings price
   time, which is the proxy §2 has and a per-layer clock would replace. No
   engine change shows 6× on that. The
   entries with a search — `branching/06`, `branching/07`, `sq-bwd/houses`,
   `features/01`, 0.2–1.9 s and ≥ 98 % of enterings past layer 1 — are the
   phase's measurement set, and the two zebras stay as the *parity* cells they
   have always been.

> **A sibling file.** This one is about the *search* — where the enterings are
> and what a speculation costs. [shared_state.md](shared_state.md) is
> [S1a.7.1](s1a.7.1_sync_shared_state.md)'s, in the same shape, and is about
> the four structures a worker shares: how hard each is read, how rarely each
> is written, and which of design/08 §6's strategies survived being measured.

## 6. T1a.7.2.0 — the layer stack, coalesced at the barrier

§4's third reading found a 2.8× that had nothing to do with the mode it was
found in: `branching/07 -e` runs 1 135 ms at root depth 164 and 406 ms at depth
3, **for the same 11 501 enterings**. Deferring is one way to get the depth. It
is not the cheap one, because it also postpones every prune. Flattening root at
the layer barrier is: integration stays immediate, no writeback moves, and only
root's *representation* is rebuilt.

That is `Kb::flatten()` at the end of each layer, gated on
`SolveOptions::coalesce_root_at` — the depth at which a barrier is worth an
O(facts) rebuild. `ein-infer/examples/flatten_probe.rs` is the instrument.

### What it is worth

`solve -e` through the CLI, best of five, the two binaries interleaved so a
thermal drift cannot land on one column. `--cores P:1` on cpu0 of an i9-14900HX,
governor `powersave`, turbo on.

| workload | before | after | | peak RSS before → after |
|---|---:|---:|---:|---|
| `branching/07 -e` | 882 ms | **278 ms** | **3.17×** | 16.4 → 16.7 MB |
| `zebra -e` | 48.8 ms | 45.2 ms | 1.08× | 16.6 → 16.7 MB |
| `zebra2 -e` | 31.2 ms | 29.8 ms | 1.05× | 16.6 → 16.7 MB |
| `zebra2-hints -e` | 13.0 ms | 13.1 ms | 0.99× | 14.6 → 14.6 MB |
| `branching/06 -e` | 196 ms | 199 ms | 0.98× | 26.6 → 26.7 MB |
| `sq-bwd/houses -e` | 252 ms | 250 ms | 1.01× | 16.4 → 16.7 MB |
| `features/01 -e` | 1 680 ms | 1 636 ms | 1.03× | 94.4 → 94.7 MB |

**3.17×, above the 2.8× §4 predicted**, and the last three rows are the reason
to trust it rather than a reason to doubt it: those three workloads **flatten
zero times** — their barriers leave root at depth 2, below the threshold — so
their columns are the measurement's own noise floor, and it is ±2 %. The
0.98× on `branching/06 -e` reproduces at nine repetitions and is not work: the
only difference on that file's path is five `depth()` comparisons.

### What it costs

`materialise()` is O(facts) per layer, so the setting is a threshold and not a
`bool`. The probe's cost columns, over all 49 non-slow corpus files that reach
a `solve -e` verdict:

| threshold | files that flatten | flattens | facts copied, worst file |
|---|---:|---:|---:|
| `None` — off | 0 | 0 | — |
| `Some(2)` — every barrier | 33 | 1–5 each | 1 160 |
| **`Some(3)` — shipping** | **4** | **1 each** | **533** |
| `Some(20)` | 4 | 1 each | 533 |

Three is "a mid-layer write happened": a fork seals root's top, so a layer with
no writeback leaves depth 2, and 3 is the first depth a writeback can produce.
The four files that reach it are exactly the four that write back — the two
zebras, the hints fixture and `branching/07 -e` — and each flattens **once**,
after layer 1, which is where §3a found all 248 of the corpus's writebacks.
`Some(20)` behaves identically because no corpus layer stack lands between 3
and 20; `Some(2)` costs 5× the copying for no measurable time, on files that
were already at the depth it flattens.

The cost case the threshold exists for — a large root, cheap layers, a
writeback every layer — **is not in this corpus**: the worst `flatten_facts` in
the sweep is 1 160, and 533 in the shipping configuration. If one arrives, the
counter pair (`flatten`, `flatten_facts`, behind `--features counters`) is what
prices it, and the threshold is where the answer goes.

### What makes it safe

Not an argument — three things that fail loudly.

- **The entering count is identical in every column, on every one of the 49
  files.** That is what separates this from deferral, which moves it by 5.2× on
  `zebra2 -e`, and it is the property `--jobs N` will need later in the phase:
  a knob that rebuilt a representation *and* moved a counter would be a
  traversal change wearing a performance change's clothes.
- **`cargo test --workspace` is green with no `EIN_BLESS`.** `corpus_shapes`'s
  5 178 renderings of 128 files, the four golden sets, `summary_properties`'s
  thirteen identities over every `solve` cell — a re-bless here would have been
  the flatten announcing that it changed an observable, and there was none.
- **Two tests hold the reason rather than the result.**
  `search_invariants.rs`'s `coalescing_at_the_barrier_collapses_roots_layer_stack`
  asserts that with the barrier *off* root still ends deeper than 100 on
  `branching/07 -e` — so the day the writebacks go, the test says so rather than
  passing vacuously — and that with it on the depth collapses **and the
  enterings do not move**. `coalescing_costs_no_prune_where_deferring_costs_many`
  is the same claim on the two zebras, where the deferral's price is visible and
  the flatten's is zero. `Kb::depth()` reaches four probes and no renderer,
  which is why this needs a test and not a golden.

### Where the win is *not*

Layer 1 keeps its growing stack: the barrier is a layer boundary, and the
writebacks are all inside layer 1 (§3a, 248 of 248). On `branching/07 -e` that
leaves 204 of 11 501 forks walking a stack that is still growing under them and
puts 11 297 on a stack of one — which is the same 98/2 split every other number
in this file has, arriving for the third time and from a third direction.

## 7. T1a.7.2.1 — the seam, and what it costs

Before a thread there is a question of *types*: what does a worker hold? Four
things had to change, and the interesting number is that together they cost
nothing.

| what a worker holds | before | after |
|---|---|---|
| root | `&mut Kb` — `fork()` seals the parent's top layer | `&Kb`. `Kb::fork` splits into `seal_top` (mutates, once per fanned-out layer) and `branch` (shared, once per worker) |
| the intern tables | `Interner` / `IntPool` / `FactStore` owned by `Terms` | `Table<T>`: `Own(T)`, or `Shared(Arc<T>)` between `Terms::share` and `Terms::reclaim`. A lent table answers a lookup and refuses an assignment |
| the record arena | one fork region on the shared `ProvArena` | `records` shared, the **region per worker** and carried back on the result (`ProvArena::share` / `take_fork` / `swap_fork`) |
| the event sink | `Rc<RefCell<Vec<u8>>>`, not `Send` | `Events::worker()` buffers whole lines with a hole where the ordinal goes; `Events::replay` numbers them at the commit |

### What it costs

`solve -e` through the CLI, best of 13, the two binaries **alternated** run by
run. `taskset -c 0` on an i9-14900HX, governor `powersave`.

| workload | before | after | |
|---|---:|---:|---:|
| `branching/06 -e` | 197 835 µs | 199 089 µs | +0.63 % |
| `branching/07 -e` | 276 220 µs | 273 996 µs | **−0.81 %** |
| `features/01 -e` | 1 671 766 µs | 1 643 684 µs | **−1.68 %** |
| `sq-bwd/houses -e` | 253 692 µs | 251 343 µs | −0.93 % |
| `zebra2 -e` | 30 739 µs | 30 801 µs | +0.20 % |
| `zebra -e` | 46 146 µs | 46 050 µs | −0.21 % |

Inside the ±2 % floor §6 established, and signed both ways — which is what
"free" looks like when it is measured rather than asserted. Two effects cancel:
`Table`'s branch on the fact store's 5.8–26 M reads costs, and
`try_commitment_set` losing its `&mut Kb` pays it back.

### The route that was not free, and why it is worth writing down

The obvious spelling is `Arc<T>` in **both** states — read through the `Arc`,
grow through `Arc::get_mut`. It was built first, and it is **4 % slower on
`branching/06 -e`**:

| workload | `Arc` in both states | `Table` (shipping) |
|---|---:|---:|
| `branching/06 -e` | −4.0 % | +0.6 % |
| `features/01 -e` | −1.9 % | −1.7 % |
| `branching/07 -e` | −1.7 % | −0.8 % |

`Arc::get_mut` has to *prove* uniqueness, and its proof is a locked
read-modify-write on the weak count. §2's table says why that is the wrong
place to spend it: `branching/06 -e` makes **2 318 815** interning calls to
assign **417** ids, so the atomic is paid 5 561 times per assignment it
enables. The two-state enum pays a branch instead — on more calls, but a branch
whose outcome is constant for the whole of a layer.

The general form is worth keeping: *when a structure is read far more often
than it is grown, put the sharing in the type and not in the pointer.*

## 8. T1a.7.2.1 — the fan-out, and the three things it costs

**The first threads in the repo**, and the first numbers. `--jobs N` on 8
physical P-cores (`cpu0,2,4,6,8,10,12,14`), best of five, `solve -e` through
the CLI.

| workload | `-j 1` | `-j 2` | `-j 4` | `-j 8` | |
|---|---:|---:|---:|---:|---:|
| `sq-bwd/houses -e` | 254.5 ms | 161.8 | 97.9 | **59.9** | **4.25×** |
| `features/01 -e` | 1 669.9 ms | 1 173.6 | 776.7 | **552.5** | **3.02×** |
| `branching/06 -e` | 205.2 ms | 152.7 | 101.3 | **71.1** | **2.89×** |
| `branching/07 -e` | 284.1 ms | 218.3 | 144.4 | **109.3** | **2.60×** |
| `zebra -e` | 46.3 ms | 41.3 | 36.1 | 34.7 | 1.34× |
| `zebra2 -e` | 31.2 ms | 29.2 | 27.4 | 27.5 | 1.13× |

**2.60–4.25× on the measurement set against a ≥ 6× target**, and the zebras at
1.1–1.3× exactly as §5.4 said they would be — their layer 1 holds half their
firings and layer 1 is the one that cannot be fanned out. The gap is § Where
the other 2× is, below, and it is not one thing.

> These are the numbers **after** § The commit's real cost, which took the
> measurement set from 2.19–2.89× to the row above by moving one `drop` from
> the committing thread to the worker. The first table this section had is kept
> there, because the before-column is the finding.

### It is the same computation, and that is checked three ways

- **The whole corpus.** All 47 non-slow entries that reach a `solve -e`
  verdict, at `--jobs 1` and `--jobs 8`: same exit code, byte-identical stdout,
  and **every field of `--json-summary`** — which is every engine counter —
  equal. 0 divergences.
- **The event stream, byte for byte.** `--events --events-level verbose` at
  both job counts on four files, `run` line excluded because it echoes argv:
  identical, including `branching/06 -e`'s **2 200 561** lines. That is the
  ordered commit and the narration replay checked together, and it is the
  strongest form the promise has — an event's ordinal is assigned at the
  commit, not by the thread that emitted it.
- **The counters, as a unit test.** `search_invariants.rs`'s
  `jobs_does_not_move_the_answer_or_a_counter` compares the whole
  `MonotonicStats` at `--jobs {2,4,8}` over 16 files, and
  `a_deep_search_is_counter_identical_under_jobs` does it on the two five-layer
  searches.

### …and one entering in the corpus cannot be done on a worker

`lattice/02_genuine_3set_death.ein` hands **three** enterings back per run, and
that is a correction to
[shared_state.md §2a](shared_state.md#2a-and-a-total-is-the-wrong-shape-of-number-for-it)
rather than a surprise about it. That table measured `try_commitment_set` only,
and a fork stays alive through the `complete()` probe — whose blind enumerator
*numbers the candidates it walks*. Those are the ones that come back.

What happens then is the mechanism working: `Terms::refused` is set at the
single point where a lent table declines, everything the worker produced after
it is discarded — its narration included — and the committing thread re-runs
the entering where the tables can grow. Every counter still matches, which is
the point: the fallback is not an approximation, it is the same entering
computed where it can be. `JobStats::handed_back` is the running count.

### The commit's real cost, and it is not the commit

The first fan-out was **2.19–2.89×**, and CPU utilisation said why: 382 % of
800 % at `--jobs 8`, with total CPU time up only 29 % — the threads were idle,
not doing extra work. Timing the two halves of each batch on `branching/06 -e`
put 20 ms of a 79 ms run in the ordered commit, which is a 26 % serial fraction
and an Amdahl ceiling of 3.8× before the fan-out's own efficiency is counted.

Timing the commit's *parts* found what it was, and it was not the learned
clause, the subsumption or the `state_key`:

| `features/01 -e`, `--jobs 8` | ms |
|---|---:|
| the commit loop | 269 |
| …of which `drop(result)` on the alive path | **156.7** |
| …of which `discard_fork` — the region's records | **35.3** |
| …of which `handle_dead` | 0.0 |

**192 of 269 ms is freeing memory another thread allocated.** A fork's KB, its
firings and its provenance region are built on a worker and returned on the
committing thread, and every modern allocator makes a cross-thread free its
slow path — `snmalloc` posts it to the owning thread's message queue, which is
why `sn_rust_dealloc` and `handle_message_queue_slow` were 6 % of the profile.

The fix is to free it where it was allocated, and the only question is *when
that is allowed*. It is allowed whenever nothing at the commit will read the
fork, which is: not a solution node (`record_node` snapshots one and promotes
what it cites), no `store_lattice` (the proof reads dead forks' state keys),
and a dumper that says it does not look (`Dumper::reads_forks`, `true` by
default and `false` for `NoDumper`). The worker then drops the KB, the firings
and the region, and does it **in parallel**.

| workload | before | after | |
|---|---:|---:|---:|
| `sq-bwd/houses -e` | 2.89× | **4.25×** | +47 % |
| `features/01 -e` | 2.40× | **3.02×** | +26 % |
| `branching/07 -e` | 2.19× | **2.60×** | +19 % |
| `branching/06 -e` | 2.66× | **2.89×** | +9 % |

The general form: **in a fan-out, freeing is work too, and it belongs to
whoever allocated.** A result that crosses a thread boundary should carry only
what the far side reads.

The hypothesis this replaced is worth recording as rejected. The dead
commitment's `state_key` is a sort of the whole fork's fact list, computed on
every death for a `LatticeProof` that a solve without `--trace` never builds —
so it looked like the answer. Making it conditional measures **±2 %**, which is
the noise floor: the corpus's forks are small enough that sorting their fact
lists costs nothing. It is now conditional anyway, because `Entered::kb` is
`None` where the worker dropped the fork, but that is a consequence and not a
win.

### Where the other 2× is

With the commit's frees moved, `--jobs 8` breaks down like this — Phase 2 only,
by timing each region:

| | `branching/06` | `branching/07` | `features/01` | `houses` |
|---|---:|---:|---:|---:|
| the fan-out | 38.2 ms | 55.6 | 441.5 | 49.3 |
| **candidate generation + ordering** | **10.8** | **39.5** | **55.8** | **3.1** |
| the ordered commit | 13.8 | 3.0 | 54.6 | 3.2 |
| layer 1, sequential | 0.9 | 7.4 | 0.2 | 0.1 |
| the layer barrier | 0.3 | 0.3 | 8.9 | 0.4 |

Two things are left, and neither is what design/08 has a level for:

1. **`generate_layer` + `order_candidates` is the serial term now** — 39.5 ms
   of `branching/07 -e`'s 109 ms, 36 % of the run. Building layer *L+1*'s
   commitment sets from layer *L*'s survivors and filtering each against the
   no-good store is per-candidate work with no shared state, so it is
   parallelisable the same way the enterings are; the ordering is a sort whose
   *keys* are independent. That is T1a.7.2.7.
2. **The fan-out is ~5× on 8 cores**, not 8×. On `sq-bwd/houses -e` the serial
   terms are down to 4 ms of 60 and the run is still 4.25×, so what is left is
   the fan's own efficiency. The profile has no lock in it — the engine's one
   `Arc<Mutex<PlanMemo>>` does not appear — and 11 % is allocator and libc, so
   this reads as memory rather than contention: a fork allocates and frees a KB
   delta and a saturator, and eight of them at once is a bandwidth question.
   Reducing what a fork allocates is a
   [P1a.6](../p1a.6_performance/README.md)-shaped answer, not a P1a.7 one.

Neither is S1a.7.3's or S1a.7.4's — those parallelise root saturation and the
boundary round, which are Phase *1* and are not in this denominator at all.

### The batch is a memory decision before it is a scheduling one

The first fan-out ran a whole layer at once, and on `features/01 -e` — 384 167
enterings in one layer — that meant **1.9 GB** of peak RSS against 84 MB
sequential, because every speculated result holds a fork's KB and its record
region until the commit reaches it. It was also *slower* than `--jobs 1`:

| | `-j 1` | `-j 2` | `-j 8` | peak RSS |
|---|---:|---:|---:|---:|
| whole layer in flight | 1 704 ms | **1 833 ms** | 1 173 ms | **1.9 GB** |
| batch = `jobs` × 32 | 1 746 ms | 1 413 ms | **727 ms** | **89 MB** |

So the batch is bounded, and `BATCH_PER_WORKER` is a measured constant rather
than a chosen one. The sweep, on `features/01 -e` at `--jobs 8` with the pool:

| enterings in flight per worker | 8 | 32 | 128 | 512 |
|---|---:|---:|---:|---:|
| wall | 831 ms | **740 ms** | 730 ms | 856 ms |
| peak RSS | 86 MB | **89 MB** | 93 MB | 122 MB |

32 is the knee. A **cut** narrows it further — `stop_after` and
`max_enterings` stop mid-layer and everything past the cut is waste — so the
batch is one round of workers then (T1a.7.2.4).

Peak RSS with the bound in place, which is [README § Risks](README.md#risks)'s
"memory scales with jobs" answered:

| workload | `-j 1` | `-j 8` | `-j 16` |
|---|---:|---:|---:|
| `features/01 -e` | 79.8 MB | 82.8 MB | **90.3 MB** |
| `branching/06 -e` | 20.0 MB | 28.6 MB | 36.3 MB |
| `sq-bwd/houses -e` | 11.6 MB | 19.0 MB | 24.8 MB |

### And the pool is a measurement, not a dependency preference

A bounded batch means many barriers per layer, which makes *the cost of a
barrier* the thing to watch. The first implementation used
`std::thread::scope`, spawning `jobs` threads per batch: on `features/01 -e`
that is ~96 000 spawns, and at `--jobs 2` it was a **3× slowdown** —

| batch/worker | `-j 2` with `std::thread::scope` | `-j 2` with a pool |
|---|---:|---:|
| 4 | 5 426 ms | — |
| 16 | 3 543 ms | — |
| 32 | 3 794 ms | **1 413 ms** |

— so the threads have to stay alive between batches, which is what `rayon`'s
pool is for. It is built **once per solve** and only when `jobs > 1`, so a
default `--jobs 1` run creates no thread at all; it lives behind `ein-infer`'s
`parallel` feature, on by default, per
[design/12 §2](../design/12_toolchain_and_layout.md#2-dependency-policy).
`collect_into_vec` over an *indexed* parallel iterator is what keeps the
results in candidate order whatever order the workers finished in.

## 9. Reproducing

```sh
# §1
utils/bench_env.sh ein.rs/target/release/ein solve examples/zebra2.ein -e -t

# §2
ein.rs/target/release/ein solve examples/zebra2.ein -e --events /tmp/ev.jsonl

# §3
cd ein.rs && cargo build --release --features spec-audit --target-dir target-sa
python3 utils/spec_audit.py --timeout 90 --json /tmp/sweep.json
python3 utils/spec_audit.py -k zebra --no-fail-fast --timeout 300

# §3a — no instrument but the event stream: walk it, index the enterings
# within their layer, and note where a *new* (not h) lands.
ein.rs/target/release/ein solve -e examples/zebra.ein --events /tmp/ev.jsonl
python3 - <<'EOF'
import json, collections
layer, idx, seen, at = None, collections.Counter(), set(), []
for line in open('/tmp/ev.jsonl'):
    e = json.loads(line)
    if e['e'] == 'enter':
        layer = e['layer']; idx[layer] += 1
    elif e['e'] == 'writeback' and e['reason'] == 'singleton-dead-clause':
        if e['fact'] not in seen:
            seen.add(e['fact']); at.append((layer, idx[layer]))
print('layer 1 candidates:', idx[1], ' W grew at:', at[:3], '…', at[-1])
EOF

# §4
cd ein.rs
cargo run --release -p ein-infer --example defer_probe
cargo test --release -p ein-infer --test search_invariants

# §7 — the seam is types, so the gate is the check; the cost is a bench
cargo test -p ein-infer --test worker_view --test shareable
cargo build --release -p ein-cli   # …and time it against the parent commit

# §8 — the scaling table, one row
for j in 1 2 4 8; do
  utils/bench_env.sh --cores P:8 ein.rs/target/release/ein \
      solve -e examples/branching/06_lookahead_on.ein -j $j
done
# …the batch sweep
EIN_BATCH_PER_WORKER=128 ein.rs/target/release/ein \
    solve -e examples/features/01_not_and_absent.ein -j 8
# …and the invariance, which is what the numbers are only worth having with
cargo test -p ein-infer --test search_invariants
ein.rs/target/release/ein solve -e examples/branching/06_lookahead_on.ein \
    -j 1 --events /tmp/a.jsonl --events-level verbose
ein.rs/target/release/ein solve -e examples/branching/06_lookahead_on.ein \
    -j 8 --events /tmp/b.jsonl --events-level verbose
diff <(tail -n +2 /tmp/a.jsonl) <(tail -n +2 /tmp/b.jsonl) && echo identical

# §6 — the cost/benefit columns, and the corpus sweep behind the threshold
cargo run --release --features counters -p ein-infer --example flatten_probe
cargo run --release --features counters -p ein-infer --example flatten_probe -- \
    $(python3 - <<'EOF'
import re
s = open('../corpus/corpus.toml').read()
for b in s.split('[[entry]]')[1:]:
    m, r = re.search(r'path\s*=\s*"([^"]+)"', b), re.search(r'runs\s*=\s*\[(.*?)\]', b, re.S)
    if m and r and '"solve -e"' in r.group(1) and 'slow' not in b \
       and m.group(1).startswith('examples/'):
        print(m.group(1))
EOF
)
```
