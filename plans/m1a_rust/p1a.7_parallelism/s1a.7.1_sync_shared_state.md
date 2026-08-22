# S1a.7.1 — Making the shared state `Sync`

**Phase:** P1a.7 (Parallelism)
**Estimate:** 3 days — **1 d spent; four of the seven tasks are done and the
remaining three are smaller than they were**
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
  fires tens of times per second; it fires tens of times per *solve*. The
  append may take a mutex.
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

What is left of the stage is therefore one question, and it is not a
concurrency question: **`&mut Terms` is threaded through 99 signatures**, and a
worker needs a `&Terms` plus somewhere to append. The measurement says the
second half is small.

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
- TSan clean; `loom` model checks pass for whatever protocol the fact store
  ends up with. **If it ends up with none** — the snapshot route below — the
  criterion is met by there being nothing to model, and the stage says so
  rather than shipping a `loom` test of a `Mutex`.
- The determinism lint (no hash-map iteration at an observable site) is
  green with an explicitly reviewed allow-list.
- **The read path is not slower.** `cargo bench` on the eight-bench M1a
  measurement set, `--features parallel` against the default build, within
  noise. The 26 M reads are the reason this is an acceptance item and not a
  note.

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

### Task T1a.7.1.2 — Fact store

**Re-aimed by T1a.7.1.0.** The task was a segmented vec plus a sharded lookup
map; what the numbers ask for is a lock-free *read* and an append that may
cost anything. Three routes, and the stage picks one **with a bench**, not by
argument:

- **(a) `Arc<Core>` snapshot + overlay.** Take the store as an immutable
  `Arc` at layer start; workers read it by `&`, lock-free and with the borrow
  signatures unchanged. A worker that assigns an id takes a mutex on a small
  overlay; a worker that *reads* an overlay id copies the row into a
  worker-local vec on first sight, which is sound because a row is immutable
  once written. The overlay folds into the core at the layer barrier — tens to
  hundreds of rows. No new dependency, no `unsafe`, and no read-site change.
- **(b) a lock-free segmented vec** (`boxcar`-style), as design/08 §6 has it.
  It solves the borrow problem properly, and it costs a dependency the policy
  table does not list — [design/12 §2](../design/12_toolchain_and_layout.md#2-dependency-policy)
  names `rayon` for this phase and nothing else — for a write rate of 41 per
  solve.
- **(c) per-worker id space with promotion at commit.** Assigns global ids in
  candidate order, so `FactId` assignment stays *deterministic* under
  `--jobs N`, which (b) and (a) do not. That is worth something real — but it
  renumbers a fork's delta, provenance, no-good clause and state keys at the
  boundary, and the KB's presence bitsets are indexed by `FactId`, so a
  reserved high band is not free either. **Do not take this route for
  determinism's sake without checking whether determinism needs it**: the
  observables that could move under a permuted id space are already measured
  at 51 of 3 160 permuted pairs, all of them narration, and every search
  counter is held exactly ([`ein-parity`](../../../ein.rs/crates/ein-parity/src/lib.rs)).

(a) is the one to beat, and the reason is the read column: it is the only one
of the three that leaves 26 M borrow-returning reads exactly as they are.

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

## Notes

- Resist making the KB itself `Sync`-mutable. Each fork owns its delta;
  that ownership is the whole safety argument, and sharing a mutable KB
  would reintroduce every hazard this design avoids.
- If contention appears anywhere, the first answer is per-worker staging
  with a merge at join, not a finer lock.
- **The `&mut Terms` count is 99, not the 78 the phase README quoted.** It grew
  with P1a.8 and P1a.10; a third of them are in `tests/`, where a `&Terms`
  signature costs nothing to change and buys nothing either.
