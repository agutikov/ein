# S1a.7.1 — Making the shared state `Sync`

**Phase:** P1a.7 (Parallelism)
**Estimate:** 3 days → **4.5 d.** 2 d spent; five of the eight tasks are done
and two more are decided-not-built. The estimate moves because T1a.7.1.7 did
not exist when it was written: the provenance arena is a shared structure
design/08 §6 has no row for, and building the per-worker arena it now has a
decision for is **~1.5 d** that the original three did not budget.
**Depends on:** [P1a.6](../p1a.6_performance/README.md)
**Implements:** [design/08](../design/08_parallelism.md) §6
**Measures:** what a worker actually shares —
[shared_state.md](shared_state.md)

> **Re-shaped 2026-08-22 by its own T1a.7.1.0**, the same reflex that added
> [S1a.7.0](s1a.7.0_speculation_audit.md) to the phase: the stage's premise is
> a *write rate*, and a write rate is measurable before anything is built.
> Read [shared_state.md](shared_state.md) first; the short version is
> § What the measurement changed.

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
content-based. That invariant already has an instrument —
`ein-render/tests/id_order_invariance.rs`, which runs the whole corpus under a
deliberately hostile permutation of the id space and holds every search
counter, every verdict and every model exactly. What this stage may not do is
give it something new to find.

## What the measurement changed

T1a.7.1.0 ran the six workloads of the phase's measurement set, marked the end
of root saturation, and counted what the **search** does to each shared table.

- **The fact store's write rate is not a write rate.** `features/01 -e` is
  384 167 enterings, 26.1 M borrow-returning reads, 3.4 M interning calls —
  and **41** new fact ids. `branching/06 -e`, the densest of the six, is 417 in
  2.3 M. design/08 §6's lock-free segmented vec is built for an append that
  fires tens of times per second; it fires tens of times per *solve*.
- **And counted per entering — which is what a worker runs — it does not
  happen at all.** Four of the six workloads have **zero** enterings that
  append a fact id; the other two have 7 of 111 and 1 of 101. Every one of the
  417 ids `branching/06 -e` assigns is assigned by the committing thread, and
  every appending entering anywhere in the corpus is in the **head** of its
  layer (largest within-layer index: 6, 21, 83). So the store needs no
  concurrent-append design — it needs workers to be *unable* to append, which
  `&FactStore` already is. T1a.7.1.2 is decided rather than benched.
- **The hard part is the read path, and design/08 does not mention it.**
  `FactStore::args` returns a `&[Value]` into the argument arena and `row`
  returns a `&Row`. No lock returns a borrow outliving its guard, and a `Vec`
  reallocates on push — so an `RwLock<FactStore>` is not a change to the write
  sites, it is a change to every read site in the port, on the path carrying
  26 M calls.
- **The interner needs no lock at all**, because it can be made not to grow.
  Between the end of root saturation and the end of the solve, **four**
  distinct names arrived, on 24 of the 90 corpus files that solve. Three were
  the engine's own — `<lookahead-dies-immediately>` (19 files),
  `<forced-positive>` and `<monotonic-unconditional>` (4 each) — and are now
  interned by `Terms::new` as
  [`ein_core::terms::ENGINE`](../../../ein.rs/crates/ein-core/src/terms.rs),
  which holds eight, the other five being names that arrive during root
  saturation and so were never a hazard (`__symmetric__` on 94 files, and the
  mirror's `a` / `b`, re-interned *per firing*). The fourth, `Ann`, was a
  *program* constant that appears only as a rule argument, so the compiler was
  the first to see it, mid-search; `intern_program_names` closes that at load.
  The integer pool never grew at all. **`&Interner` is `Sync` already** —
  T1a.7.1.1 is deleted rather than built, and not one read site changes.
- **The plan memo was already done.** `Arc<Mutex<PlanMemo>>` since
  [S1a.6.8](../p1a.6_performance/s1a.6.8_compile_cache_and_extents.md), for the
  unrelated reason that a memo shared across forks needed one owner.
- **The structure that does have a write rate has no row in design/08 §6.**
  The **provenance arena** is written by 100 % of enterings — 39 records per
  entering on `branching/06 -e`, **2 135 093 records and 205 MB** on
  `features/01 -e` — and has the same borrow-returning read. It is also where
  the phase's memory risk lives: that file peaks at 724 MB at `--jobs 1` and
  ~28 % of it is an arena nothing reclaims until the run ends. New task,
  T1a.7.1.7.

What is left of the stage is therefore one question, and it is not a
concurrency question: **`&mut Terms` is threaded through 99 signatures**, and a
worker needs a `&Terms`. The measurement says it needs nothing else — except
for provenance, which is T1a.7.1.7's.

## Acceptance

> **Restated 2026-08-22.** The original criteria named `--features parallel`,
> TSan and `loom` — which stand — and one instrument that no longer exists:
> "the full corpus runs identically" was a T3 claim, and
> [P1a.10](../p1a.10_single_implementation/README.md) retired the tier
> vocabulary with the harness. The successor is named per item. See the phase
> [README § The acceptance, restated](README.md#the-acceptance-restated).

- **`cargo test --workspace` is green with the engine compiled `--features
  parallel` and executed at `--jobs 1`** — the `Sync` refactor alone changes
  nothing. The corpus-wide half of that claim is
  `ein-render/tests/corpus_shapes.rs`: **5 178 renderings** of the manifest's
  128 files against `tests/golden/corpus_shapes.md5`, which is the byte
  comparison the tier vocabulary used to name. **No `EIN_BLESS=1` may be needed** — a re-bless
  here would be the refactor announcing that it changed an observable.
- **`ein-infer/tests/interning.rs` holds the invariant that removes the
  interner's lock** — the symbol table and the integer pool do not grow
  between the end of root saturation and the end of the search, over every
  corpus file that solves. ✅ **shipped**, 0 of the 90 that reach a solve —
  where 24 of the 90 grew before it.
- **`id_order_invariance` is unchanged in what it finds.** The refactor may
  not add a rendering to the moved set, and may not remove one either: the
  test asserts both directions, and its `EIN_PARITY_STRICT=1` tally is the
  before-column.
- A synthetic multi-threaded stress (N threads interning and forking
  concurrently) shows: interning is idempotent across threads, a
  `FactId` means the same proposition in every thread, and no thread sees
  another's presence bits.
- TSan clean; `loom` model checks pass for whatever protocol the shared
  structures end up with. **The fact store ends up with none** (T1a.7.1.2:
  workers hold `&FactStore` and `intern` is `&mut`), so this criterion now
  points at whatever T1a.7.1.7 gives the provenance arena — and if that is
  also a per-worker structure with no cross-thread protocol, the criterion is
  met by there being nothing to model and the stage says so rather than
  shipping a `loom` test of a `Mutex`.
- The determinism lint (no hash-map iteration at an observable site) is
  green with an explicitly reviewed allow-list.
- **The read path is not slower.** `cargo bench` on the eight-bench M1a
  measurement set, `--features parallel` against the default build, within
  noise. The 26 M reads are the reason this is an acceptance item and not a
  note — and after T1a.7.1.2 the expected answer is *identical*, not *within
  noise*, because no read site is changed at all.

## Tasks

### Task T1a.7.1.0 — What the shared state costs ✅

**Shipped 2026-08-22.** `ein-infer/examples/shared_state_probe.rs` +
four counters on `FactStore` (`fact_read`, `fact_probe`, `fact_intern`,
`fact_new`, behind the existing `counters` feature, so a shipped build has
none). Numbers and argument: [shared_state.md](shared_state.md).

### Task T1a.7.1.1 — Interner ✅ — *and it is not a lock*

**Shipped 2026-08-22, by deletion.** The sharded `RwLock` this task specified
is not needed: the table does not grow while it is shared. What shipped
instead —

- `ein_core::terms::ENGINE`, the eight names the engine writes rather than
  reads, interned by `Terms::new` with the kernel vocabulary, and reachable as
  `Kernel` fields so no call site interns them again;
- `ein_ir::from_ir::intern_program_names`, a load pass over the registered
  rules and the query, so a constant that appears only as a rule argument is
  known before the search;
- `ein-infer/tests/interning.rs` — three tests, the load-bearing one being the
  corpus sweep.

The **rank table** stays derived state and is already `OnceLock`, which is the
right primitive: built on demand, and — now that the table is frozen after
load — built exactly once.

### Task T1a.7.1.2 — Fact store ✅

**Decided 2026-08-22, and the answer is that it needs nothing** — by
measurement, not by the bench this task asked for,
because the bench would have compared three ways of making an append cheap,
and [shared_state.md §2a](shared_state.md#2a-and-a-total-is-the-wrong-shape-of-number-for-it)
found the append does not happen.

A **total** is the wrong shape of number here: 417 ids spread one per entering
is a design and 417 inside one entering is another. Counted per entering —
which is what a worker runs —

| workload | enterings | that append a fact id | largest within-layer index of one |
|---|---:|---:|---:|
| `zebra -e` | 111 | 7 | 21 |
| `zebra2 -e` | 101 | 1 | 6 |
| `branching/06 -e` | 5 173 | **0** | — |
| `branching/07 -e` | 11 501 | 1 | 83 |
| `sq-bwd/houses -e` | 21 699 | **0** | — |
| `features/01 -e` | 384 167 | **0** | — |

`branching/06 -e` assigns 417 fact ids and **not one of them from inside an
entering**: they are the hypothesis generator interning a layer's candidates,
the singleton writeback and the forced-positive cascade — all on the
committing thread. A fork derives propositions that already have numbers.

**So the decision is that workers do not append.** `FactStore::intern` takes
`&mut self`; a worker holds `&FactStore` and therefore cannot call it, which
makes the type system the enforcement and leaves no protocol to model. An
entering that would have appended hands itself back and is re-run on the
committing thread. What that buys over the three routes below is not only
simplicity: **fact-id assignment stays deterministic**, because every
assignment happens on one thread in candidate order.

The three routes are rejected in
[shared_state.md §5](shared_state.md#5-what-this-rejects-and-what-it-would-have-cost)
with what each would have cost — a branch on 26 M reads, a dependency
[design/12 §2](../design/12_toolchain_and_layout.md#2-dependency-policy) does
not list, or a renumbering whose determinism benefit route 2 gets for free.

**The bound this hands to [S1a.7.2](s1a.7.2_parallel_enterings.md).** The rate
above is *sequential*: entering 1 appends and 2…n then find the id. A batch of
`jobs` workers forks one snapshot, so wasted work is bounded by
`appending enterings × jobs` — ≤ 56 of 111 on `zebra -e` at `--jobs 8`, and
≤ 8 of 11 501 on `branching/07 -e`. The `max i` column is why that bound does
not bite: every appending entering is in the **head** of its layer, so running
the first ~100 sequentially leaves the corpus with none in the fanned-out
tail. Measuring it with threads is S1a.7.2's; deciding whether it needed a
mechanism was this task's, and it did not.

### Task T1a.7.1.3 — Plan memo ✅

**Already true.** `Arc<Mutex<PlanMemo>>` with a double-checked insert since
S1a.6.8; `Engine` holds resolved `Arc<Plan>`s so the read path never touches
the memo. Per-engine plan *lists* stay thread-local and keep their ordering
([design/06](../design/06_saturation.md) § Win A's order caveat).

What is left is the assertion, not the structure: a counter in debug builds
saying how often the memo is entered under contention, so "compiles are rare"
stays measured rather than remembered.

### Task T1a.7.1.4 — `KbCore` / `Program` audit ✅

**Shipped 2026-08-22** as `ein-infer/tests/shareable.rs`, because the compiler
already knew the answer and what was missing was somebody asking it in a file
that fails when it changes.

Nine types a fan-out would hand a worker are `Send + Sync` today — `Terms`,
`Kb`, `Program`, `Ast`, `Engine`, `SharedMemo`, `SolveOptions`,
`SolverConfig`, `CommitmentSetResult` — so no lazily-computed cache, `Cell` or
`Rc` is hiding in any of them.

**One thing is not, and it is the audit's real finding**: `events::Buffer` is
`Rc<RefCell<Vec<u8>>>`, so a worker cannot hold an event sink.
[design/08](../design/08_parallelism.md) §3's "no shared queue" hid it, because
a sink is not a queue. The fix is not a lock — it is the shape the counters
already have, a per-worker buffer merged at the **ordered commit**
(T1a.7.2.2), and it belongs to that stage. Until then the test pins the state
of affairs in the negative direction too, so the day `Buffer` becomes `Send`
is a day somebody notices.

The other interior mutability in the engine crates is `counters.rs`'s
thread-local `RefCell`, which is compiled out unless `--features counters` and
is per-thread by construction — and which is *why* a per-worker counter merge
is the established pattern rather than a new idea.

### Task T1a.7.1.5 — Ordering audit under concurrency

Grep every `sort`, `sort_by_key`, `BTreeMap` and `min`/`max` in the
engine and classify each as identity-order (fine) or content-order
(must use the semantic comparator / rank table). Add the classification
as a comment at each site — this is the audit that
[design/02](../design/02_determinism_and_order.md) §3 started and that
concurrency makes load-bearing.

The audit has an oracle now that it did not have when it was written:
`id_order_invariance` finds a site that *does* leak, where the grep finds one
that *could*. Use the grep to explain, and the sweep to decide.

### Task T1a.7.1.6 — Verification harness

Two halves, and one of them exists.

- **The permuted-interner mode is built** —
  `ein-render/tests/id_order_invariance.rs`, corpus-wide, with `EIN_ID_SEEDS`
  and an `EIN_ID_FILES` seam the fuzzer already drives. Nothing to add.
- **The multi-threaded stress is not.** N threads interning and forking
  concurrently against one `Terms`, asserting idempotence, `FactId` agreement
  and delta isolation. It is the only test in this stage that needs a thread,
  and it is the one that decides whether T1a.7.1.2's route (a) is right.

### Task T1a.7.1.7 — The provenance arena ◑ — *decided, not yet built*

**Added 2026-08-22 by T1a.7.1.0** — the shared structure
[design/08 §6](../design/08_parallelism.md#6-what-must-be-sync-and-how) has no
row for — and **decided the same day** by the measurement it asked for.

| workload | records pushed | records read | still referenced when the solve ends |
|---|---:|---:|---:|
| `zebra -e` | 6 335 | 347 | **33** |
| `branching/06 -e` | 201 902 | 125 238 | **162** |
| `sq-bwd/houses -e` | 271 909 | 291 032 | **0** |
| `features/01 -e` | **2 135 093** | **15** | **0** |

Three things fall out, and each rules something out
([shared_state.md §2b](shared_state.md#2b-the-structure-with-a-real-write-rate-is-the-one-design08-6-left-out)):

- ≥ 33.7 % of the pushes — and ≥ 99.5 % on four of the six — happen inside a
  fork, so this is a **worker** write;
- the arena is read 15 times against 2.1 M pushes on the largest workload, so
  unlike the fact store there is **no hot read path to protect**;
- and **almost nothing survives**: 2 135 093 records created and none
  referenced at the end. An alive entering's fork is dropped after the
  `complete()` probe and the dumper hook, a dying one keeps only `FactId`s,
  and the sole retainer is `record_node`'s snapshot of a solution.

**The decision is that the arena is per-worker.** A fork's records die with
the fork, so a worker holds its own and the ordered commit promotes only what
a solution node keeps — **zero** on four of the six workloads. Nothing is
shared, so nothing needs a lock and there is no protocol for `loom`.

**And the claim is asserted, in both directions**, because it is too
load-bearing to rest on a reading of the search loop:

- *nothing reads a retired record* — `Run::entering` marks the range
  `try_commitment_set` created and hands it to `ProvArena::retire`;
  `ProvArena::get` panics on a retired id in **any debug build**, so the whole
  gate is the experiment. Arming it found exactly one reader, and it turned
  out to be a *scan* rather than a reference: `ein-einb`'s writer walks the
  arena end to end. Scans now go through `ProvArena::scan`, which is the seam
  between the two kinds of read;
- *nothing **holds** one* — the stronger claim a reclamation needs, since an
  id that is stored and never read trips nothing and would still be corrupted
  by reuse. `ein-infer/tests/provenance.rs`: **5 328 live justifications over
  90 corpus files, none retired.**

#### What is left, and why it is not a one-line truncate

`ProvArena::retire` **frees nothing** today — it arms an assertion. Making it
reclaim is the change the claim licenses, and it is *not* a `Vec::truncate`,
for a reason worth writing down: on the dead path `handle_dead` pushes root's
own records (the no-good, the singleton writeback) **after** the fork's, so
the retired range is not the tail. Retiring earlier is not available either —
`handle_dead` still reads the fork through `state_key` and the dumper hook,
and under `--dump-states` that hook renders its justifications, which is
precisely what the assertion would catch.

So the reclamation *is* the per-worker arena rather than a shortcut to it:
`Terms` gains a second arena for the fork in hand, `ProvId` distinguishes the
two (the read is 15 to 308 288 calls, so the branch is free), and
`record_node` promotes on the one path that retains. **~1.5 d**, and it pays
twice: it removes the last shared mutable structure from a worker's path, and
it reclaims **205 MB** on `features/01 -e` at `--jobs 1` — which is where the
phase's "memory scales with jobs" risk turned out to live. It also stops
`.einb` writing 2 135 093 records for the twelve that are live.

## Notes

- Resist making the KB itself `Sync`-mutable. Each fork owns its delta;
  that ownership is the whole safety argument, and sharing a mutable KB
  would reintroduce every hazard this design avoids.
- If contention appears anywhere, the first answer is per-worker staging
  with a merge at join, not a finer lock.
- **The `&mut Terms` count is 99, not the 78 the phase README quoted.** It grew
  with P1a.8 and P1a.10; a third of them are in `tests/`, where a `&Terms`
  signature costs nothing to change and buys nothing either.
