# S1a.7.1 — Making the shared state `Sync`

**Phase:** P1a.7 (Parallelism)
**Estimate:** 3 days → **4.5 d. Closed 2026-08-22**; all eight tasks are done,
**three of them by deletion** — the interner's lock, the fact store's
concurrent append, and the multi-threaded stress, each removed by a measurement
rather than by a judgement call. The estimate moved because T1a.7.1.7 did not
exist when it was written: the provenance arena is a shared structure
design/08 §6 has no row for, and building the per-worker arena it had a
decision for was **~1.5 d** the original three did not budget. It is built.
**What the stage hands on** is one refactor and no design: `&mut Terms` is
threaded through 99 signatures and a worker needs a `&Terms`, which is
[S1a.7.2](s1a.7.2_parallel_enterings.md) T1a.7.2.1's first move because that
is where the first thread is.
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
content-based.

> **The second sentence turned out to be false, and it is the reason
> [T1a.7.1.5](#task-t1a715--ordering-audit)
> is not the task it was written as.** Nothing this phase builds assigns an id:
> the symbol table does not grow during a search (T1a.7.1.1) and a worker
> cannot append to the fact store (T1a.7.1.2), so a parallel run's ids are the
> sequential run's, assigned at load on one thread. The *invariant* stands and
> its instrument stands; what is gone is the concurrency that was supposed to
> make it newly load-bearing.

That invariant already has an instrument —
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
  the phase's memory risk turned out to live: that file peaked at 684–708 MB
  at `--jobs 1`, and nearly all of it was an arena nothing reclaimed until the
  run ended. New task, T1a.7.1.7 — **built**, and the file now peaks at
  85–91 MB.

What is left of the stage is therefore one question, and it is not a
concurrency question: **`&mut Terms` is threaded through 99 signatures**, and a
worker needs a `&Terms`. The measurement says it needs nothing else, and
provenance — the one structure that *did* need something — has it.

> **And it is not this stage's question either**, decided when .5 and .6 closed
> it: a signature refactor with no consumer is a refactor nobody can check.
> S1a.7.2 T1a.7.2.1 is the first code that needs a `&Terms`, so it is where the
> 99 signatures change and where a compile error means something.

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
- ~~A synthetic multi-threaded stress (N threads interning and forking
  concurrently) shows: interning is idempotent across threads, a
  `FactId` means the same proposition in every thread, and no thread sees
  another's presence bits.~~ **Not built, because two of the three claims
  stopped being claims.** No thread can intern — `intern` is `&mut` and a
  worker holds `&` (T1a.7.1.2) — so "idempotent across threads" describes an
  event the type system forbids; `FactId` agreement is a property of one store
  shared by `&`, asserted by `shareable.rs`; and delta isolation is about fork
  ownership rather than threads. The stress that *is* worth a thread is
  `--jobs 8` against `--jobs 1`, and it is
  [S1a.7.2](s1a.7.2_parallel_enterings.md) T1a.7.2.6. See T1a.7.1.6.
- TSan clean; `loom` model checks pass for whatever protocol the shared
  structures end up with. ✅ **There is no protocol to model, and that is the
  finding rather than an evasion.** The fact store ends up with none
  (T1a.7.1.2: workers hold `&FactStore` and `intern` is `&mut`); the interner
  ends up with none (T1a.7.1.1: it does not grow); and the provenance arena
  ends up with none either (T1a.7.1.7: the region a worker writes is its own).
  Every structure design/08 §6 named is now `&`-shared or per-worker, so a
  `loom` test here would be a model of scaffolding this stage invented for it.
  design/08 §8's bullet is struck through to say so. TSan still applies, and
  applies to the fan-out — which is S1a.7.2's, because there is no thread
  until then.
- The determinism lint (no hash-map iteration at an observable site) is
  green with an explicitly reviewed allow-list. ✅ **and it was red.** Six
  findings, all of them T1a.7.1.7's own — none a leak, none of them saying so.
  `python3 utils/check_hashmap_iteration.py` exits 0 over 170 files with 39
  reviewed annotations, twelve of which are T1a.7.1.5's classification of the
  identity-order *sorts*, which live in the same list because they answer the
  same question.
- **The read path is not slower.** `cargo bench` on the eight-bench M1a
  measurement set, `--features parallel` against the default build, within
  noise. The 26 M reads are the reason this is an acceptance item and not a
  note — and after T1a.7.1.2 the expected answer is *identical*, not *within
  noise*, because no read site is changed at all.
  ✅ **for the one read site that did change.** T1a.7.1.7 put a branch in
  `ProvArena::get`, so this item came due early and was answered in the
  shipping build rather than in a bench: best-of-five `solve -e` on one
  P-core, before/after one commit, on the two read-heaviest workloads —
  `branching/07 -e` (308 288 arena reads) 0.92 → **0.90 s**,
  `sq-bwd/houses -e` (291 032) 0.28 → **0.25 s**, and `features/01 -e`
  1.97 → **1.68 s**. Not slower anywhere; 15 % faster where the arena was
  largest ([shared_state.md §2c](shared_state.md#2c-what-the-region-did--the-after-column)).
  The `--features parallel` half stays owed, because the flag does not exist.

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

### Task T1a.7.1.5 — Ordering audit ✅

**Done 2026-08-22 — and it is not about concurrency.** The restatement comes first because it is what the
stage's own measurements did to the task. It was written as an audit *under
concurrency*: the premise, stated in § Context, is that "under concurrency,
`Symbol` and `FactId` ids get assigned nondeterministically, so any sort that
reaches output must be content-based". **That premise is gone.**
[T1a.7.1.1](#task-t1a711--interner---and-it-is-not-a-lock) found the symbol
table does not grow during the search and
[T1a.7.1.2](#task-t1a712--fact-store) decided a worker cannot append to the
fact store, so every id a parallel search will ever see is assigned **at load,
on one thread, in file order** — exactly as today. Nothing this phase builds
can perturb an id.

The audit is still worth having, because `id_order_invariance` exists and a
permuted id space is reachable other ways (a different load order, a `.einb`,
a generated file the fuzzer points the sweep at). What it is not is a
concurrency obligation, and a task that keeps claiming to be one would have the
next reader looking for a hazard that was removed two tasks ago.

**The compiler already forbids the dangerous half.** `Symbol` and `IntId` have
**no `Ord`** — deliberately
([`intern.rs`](../../../ein.rs/crates/ein-core/src/intern.rs)) — so a numeric
sort on a name or an integer literal does not compile, and the whole class of
"a sort that fell back on `Symbol`'s id" is a build error rather than a review
item. `FactId` and `ProvId` do derive `Ord`, and that is the audit's surface.

**138 ordering sites over the six shipping crates, in four classes:**

| class | what it orders | sites | verdict |
|---|---|---:|---|
| 1 — content, through a comparator or the rank table | `cmp_fact_semantic`, `syms.rank(...)`, `sym(a).cmp(sym(b))`, and every sort of rendered `String`s | 18 + 34 | safe by construction |
| 2 — string-keyed `BTreeMap` / `BTreeSet` | all of `ein-ir`'s loader: macro tables, the import graph, keyword maps | 28 | safe by construction, and they are `BTree` *because* a hash map's order would leak |
| 3 — identity order as a canonical key or a set normalisation | `state_key`, `emit_nogood` / `subsumed`, `filter_candidate`, `union_dead_cores`, `Snapshot::new_facts_of`, `distinct_models`, the snapshot's three | 12 | safe **iff ids are a function of the input** — which is what T1a.7.1.1/.2 now guarantee for a parallel run too |
| 4 — identity as a *tiebreak* inside a `(String, FactId)` sort | six render and audit sites, all sorting by the rendered text first | 6 | the residual — and it never fires, because interning is by content, so two distinct `FactId`s cannot render identically |
| — | ordered collections (`BinaryHeap<Ranked>`, the boundary's `BTreeSet<(i64,u64,u32)>`) and `.max()` over lengths | 22 | not identity-ordered at all |

Class 3 is the one a reader stops at, because a `.sort()` on a `Vec<FactId>`
*looks* like it orders an output. It never does: every one of the twelve is a
dedup precondition, a subsumption key or a set union, and every consumer
re-sorts by text before printing — `clause_repr` for a clause,
`canon_key_repr` for a state key, `sexpr` for a core. Each of the twelve now
says so at the site, in the `determinism-ok:` form the lint already reads, so
the classification is in the file a reader has open rather than in this one.

**And the audit found something, which is why it was worth running rather than
declaring.** The determinism lint — the phase acceptance's "no hash-map
iteration at an observable site, green with an explicitly reviewed allow-list"
— **was red**, with six findings, and all six were
[T1a.7.1.7](#task-t1a717--the-provenance-arena)'s: `promote_provenance`'s walk
for cited fork records, `rewrite_provenance`'s two remaps,
`cites_fork_provenance`'s second operand, and the corresponding line in
`tests/provenance.rs`. None is a leak — they are `any`, `extend` into a set,
and an in-place remap — but none said so, and the lint is only worth having
if it is green. `python3 utils/check_hashmap_iteration.py` now exits 0 with
**39 annotations**.

### Task T1a.7.1.6 — Verification harness ✅

**One half built, one half gone.**

- **The permuted-interner mode is built** —
  `ein-render/tests/id_order_invariance.rs`, corpus-wide, with `EIN_ID_SEEDS`
  and an `EIN_ID_FILES` seam the fuzzer already drives. Nothing to add.
- **The multi-threaded stress is not built, and it is not deferred — its
  subject was removed.** It was specified as *N threads interning and forking
  concurrently against one `Terms`*, and the task said outright that it "is the
  one that decides whether T1a.7.1.2's route (a) is right". Route (a) was not
  taken. Of its three assertions:
  - *interning is idempotent across threads* — **cannot be tested, because it
    cannot happen.** `FactStore::intern` and `Interner::intern` take
    `&mut self`; a worker holds `&`. A test would have to construct the
    situation the type system forbids, and the type system is the enforcement
    ([T1a.7.1.2](#task-t1a712--fact-store)).
  - *a `FactId` means the same proposition in every thread* — true because
    there is one store, shared by `&`, that nobody writes. `shareable.rs`
    ([T1a.7.1.4](#task-t1a714--kbcore--program-audit)) is where that is
    asserted, in the only form it has: the nine types are `Send + Sync`.
  - *no thread sees another's presence bits* — a claim about **fork
    ownership**, not about threads: each fork owns its delta, which
    `ein-core/tests/fork_cost.rs` and `fork_audit` already hold, and which a
    thread cannot make more or less true.

  What a thread will genuinely be able to check is *`--jobs 8` answers as
  `--jobs 1` does*, and that is [S1a.7.2](s1a.7.2_parallel_enterings.md)
  T1a.7.2.6 — a sixth property of `utils/fuzz_ein.py` — with T1a.7.2.1 as the
  first line of code in the repo that spawns one. Building a synthetic stress
  here would be scaffolding for a design that was measured away, and it would
  be the second time this stage was asked to build one: `loom` went the same
  way, for the same reason, in the acceptance list above.

### Task T1a.7.1.7 — The provenance arena ✅

**Added, decided and shipped 2026-08-22** — the shared structure
[design/08 §6](../design/08_parallelism.md#6-what-must-be-sync-and-how) had no
row for. T1a.7.1.0 added it, the measurement it asked for decided it, and the
assertion that decision rested on is what licensed building it.

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

**And the claim was asserted before it was built on**, in both directions,
because it is too load-bearing to rest on a reading of the search loop. The
first pass shipped `ProvArena::retire`, which freed nothing and armed an
assertion instead:

- *nothing reads a retired record* — `Run::entering` marked the range
  `try_commitment_set` created; `ProvArena::get` panicked on a retired id in
  any debug build, so the whole gate was the experiment. Arming it found
  exactly one reader, and it turned out to be a *scan* rather than a
  reference: `ein-einb`'s writer walks the arena end to end. Scans go through
  `ProvArena::scan`, which is the seam between the two kinds of read;
- *nothing **holds** one* — the stronger claim a reclamation needs, since an
  id that is stored and never read trips nothing and would still be corrupted
  by reuse. `ein-infer/tests/provenance.rs`: **5 328 live justifications over
  90 corpus files, none retired.**

Both survive the build that followed, in stronger form — the region's monotone
base replaces the debug-only bitset, and the test now asks its question of the
recorded solutions too. `retire` is gone; the assertion it armed is what the
mechanism now enforces.

#### What shipped, and why it was not a one-line truncate

`ProvArena::retire` used to **free nothing** — it armed an assertion. Making it
reclaim was not a `Vec::truncate`, for a reason worth keeping: on the dead path
`handle_dead` pushes root's own records (the no-good, the singleton writeback)
*after* the fork's, so the retired range is not the tail. Retiring earlier was
not available either — `handle_dead` still reads the fork through `state_key`
and the dumper hook, and under `--dump-states` that hook renders its
justifications, which is precisely what the assertion would catch.

So the reclamation **is** the per-worker arena rather than a shortcut to it.

- **`ProvId` grew a tag.** Bit 31 says *fork region*; the remaining 31 bits
  index the arena proper or position in the region's sequence. The tag
  preserves the order the untagged ids had, because a fork's records are
  pushed after everything that existed when it opened.
- **`ProvArena` grew the region**, and `push` routes into it. Every existing
  `terms.provs.push` / `get` call site — some thirty of them, down call stacks
  the search does not own — is untouched, which is the whole reason the region
  lives on the arena rather than being threaded through as a second parameter.
- **Three verbs, not two.** `open_fork` starts routing; `close_fork` *stops
  routing while the records stay readable*; `discard_fork` frees. The middle
  one exists because `handle_dead` writes root's no-good and `(not h)` after
  the fork is over but before the dumper has rendered the fork's own
  justifications — routing has to stop one step earlier than reclamation does.
- **Reuse is caught, in release too.** The region's base is monotone:
  `discard_fork` advances it past every id that region issued, so a stale id
  falls *below* the live base and `get` panics instead of silently addressing
  the wrong record. That is strictly stronger than the debug-only bitset it
  replaces, and it is the property a reclamation actually needs — an id that
  is stored and never read trips no read-side check in any build.
- **`Kb::promote_provenance` is the one retaining path.** `record_node`
  snapshots a solution's KB, so before the snapshot the KB's citations are
  copied into the arena proper and rewritten. It walks layers, and it
  `Arc::make_mut`s only a layer that actually cites a fork record — so root's
  shared sealed layers, which by construction cite none, are never cloned; in
  practice one layer is touched, the fork's own top.
- **Promotion order is the fork's push order**, not the caller's iteration
  order, because the citations are collected from `FxHashMap`s and which ids a
  promotion assigns may not depend on where a `FactId` hashed
  ([design/02](../design/02_determinism_and_order.md) §3). `id_order_invariance`
  is the instrument that would have found the other choice: 25 280 permutations,
  478 moved, **0 answers differ**.
- **`.einb` cannot save one.** The writer stores a `ProvId` as the record's
  position in its own scan, which holds for the arena proper and for nothing
  else. Saving happens between enterings so there are none — asserted rather
  than assumed, because the failure mode is a saved KB whose derivations
  silently point at the wrong records.

**And it pays twice.** It removes the last shared mutable structure from a
worker's path — nothing is shared, so there is no protocol for `loom` and the
acceptance says so — and it reclaims the memory *sequentially*, which is where
the phase's "memory scales with jobs" risk turned out to live:

| | before | after |
|---|---:|---:|
| `features/01 -e` peak RSS at `--jobs 1` | 684–708 MB | **85–91 MB** |
| …wall clock, best of five | 1.97 s | **1.68 s** |
| `sq-bwd/houses -e` peak RSS | 93 MB | **17 MB** |
| `branching/07 -e` peak RSS | 55 MB | **16 MB** |
| arena bytes, five of the six workloads | 0.6–205 MB | **< 0.5 MB** |

`branching/06 -e` is the sixth, and it is the one that proves the mechanism
rather than breaking it: 22 solution nodes, so 6 MB of its 19 is what
promotion copied out. Full table:
[shared_state.md §2c](shared_state.md#2c-what-the-region-did--the-after-column).

**What checks it.** `cargo test --workspace` is green in **both** profiles at
587 tests with no `EIN_BLESS`, which is the acceptance item's real content — a
re-bless would have been the refactor announcing that it changed an
observable. `ein-infer/tests/provenance.rs` is re-pointed from "is this id
retired" to "is this id a fork's", which is checkable in every build, and
extended to the recorded solutions, because promotion is the step that could be
incomplete: **7 037 live justifications over 90 files and 65 solution nodes,
none of them a fork's, 6 773 records promoted.** Four unit tests in `prov.rs`
hold the region's own contract, including that a stale id panics rather than
aliasing.

## Notes

- Resist making the KB itself `Sync`-mutable. Each fork owns its delta;
  that ownership is the whole safety argument, and sharing a mutable KB
  would reintroduce every hazard this design avoids.
- If contention appears anywhere, the first answer is per-worker staging
  with a merge at join, not a finer lock.
- **The `&mut Terms` count is 99, not the 78 the phase README quoted.** It grew
  with P1a.8 and P1a.10; a third of them are in `tests/`, where a `&Terms`
  signature costs nothing to change and buys nothing either.
