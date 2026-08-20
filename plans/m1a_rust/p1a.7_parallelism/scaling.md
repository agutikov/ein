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

> **The measurement policy this phase needs, and does not yet have.**
> `bench_env.sh` pins to *one* P-core hyperthread, which is exactly right for
> P1a.6 and exactly wrong here. On a hybrid CPU "8 cores" is three different
> machines — 8 P-cores, 8 P-core *threads* on 4 physical cores, or 8 E-cores —
> and a scaling table that does not say which is not a measurement. Every
> `--jobs N` number in this file must name its core set;
> [S1a.7.5](s1a.7.5_jobs_contract.md) T1a.7.5.5's scaling table needs a
> `bench_env.sh --cores` mode before it can be published.

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
- **The two zebras are 29 and 47 ms.** After P1a.6 took them 165× off PyPy,
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
| `zebra2 -e` | 101 | 35 | 37 ms | 111 | 5 | 40 ms | **617** | 3 | 163 ms |
| `zebra -e` | 111 | 34 | 62 ms | 192 | 5 | 101 ms | **617** | 3 | 273 ms |
| `zebra2-hints -e` | 36 | 25 | 13 ms | 46 | 4 | 16 ms | **72** | 3 | 21 ms |
| `branching/04 -e` | 36 | 2 | 1 ms | 36 | 2 | 1 ms | 36 | 2 | 1 ms |
| `branching/06 -e` | 5 173 | 2 | 263 ms | 5 173 | 2 | 262 ms | **5 173** | 2 | 259 ms |
| `sq-bwd/houses -e` | 21 699 | 2 | 349 ms | 21 699 | 2 | 349 ms | **21 699** | 2 | 348 ms |
| `branching/07 -e` | 11 501 | **164** | **1 135 ms** | 11 501 | 13 | 447 ms | **11 501** | **3** | **406 ms** |

Three readings, and the third is a finding rather than a confirmation.

**1. What deferral costs is exactly the prune it defers.** On the zebras the
singleton writeback is doing enormous work in layer 1 — 6.1× and 5.6× the
enterings without it — and batching at 20 recovers almost all of it (1.1× and
1.7×). So batch size is the knob that trades pruning for parallelism, and it is
a per-workload knob, not a constant.

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
3. **Layer 1 needs a decision**, and it is now between measured options —
   continue-and-validate (recovers the verdict, moves the narration, needs the
   fail-fast ruling above), sequential (exact, and costs 42–53 % of a zebra's
   firings), or `--unordered` only.
4. **The scaling target moves.** `zebra2 -e`'s Phase 2 is 26.8 ms and 42.3 %
   of its firings are in the exactly-parallel part — ~11 ms, if firings price
   time, which is the proxy §2 has and a per-layer clock would replace. No
   engine change shows 6× on that. The
   entries with a search — `branching/06`, `branching/07`, `sq-bwd/houses`,
   `features/01`, 0.2–1.9 s and ≥ 98 % of enterings past layer 1 — are the
   phase's measurement set, and the two zebras stay as the *parity* cells they
   have always been.

## 6. Reproducing

```sh
# §1
utils/bench_env.sh ein.rs/target/release/ein solve examples/zebra2.ein -e -t

# §2
ein.rs/target/release/ein solve examples/zebra2.ein -e --events /tmp/ev.jsonl

# §3
cd ein.rs && cargo build --release --features spec-audit --target-dir target-sa
python3 utils/spec_audit.py --timeout 90 --json /tmp/sweep.json
python3 utils/spec_audit.py -k zebra --no-fail-fast --timeout 300

# §4
cd ein.rs
cargo run --release -p ein-infer --example defer_probe
cargo test --release -p ein-infer --test search_invariants
```
