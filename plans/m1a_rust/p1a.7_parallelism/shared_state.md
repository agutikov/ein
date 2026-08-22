# P1a.7 — what the shared state costs

[S1a.7.1](s1a.7.1_sync_shared_state.md)'s measurements, in
[scaling.md](scaling.md)'s shape: the numbers first, the argument after, and
every section reproducible from one command.

**Taken** 2026-08-22 on `master`, Intel i9-14900HX,
[`utils/bench_env.sh`](../../../utils/bench_env.sh)'s fingerprint. Only counts
are published: the instrument puts a counter on `FactStore::rel`, so the build
that produces these is not the build that ships and its wall clock would be a
measurement of the counter.

---

## 1. What is shared, and what the design assumed

[design/08 §6](../design/08_parallelism.md#6-what-must-be-sync-and-how) names
six rows, of which four are structures a worker touches — the other two,
root's no-good store and the stats, are written on the committing thread and
need no sharing. Three of the four strategies are about **writing**, and the
write rate had never been measured:

| shared state | design/08 §6's strategy | what it assumes |
|---|---|---|
| `KbCore`, `Program` | `Arc`, immutable | nothing — it is true by construction |
| `Interner` | sharded `RwLock`, "writes are rare after load" | that there *are* writes after load |
| `FactStore` | lock-free segmented vec + sharded lookup map | that the write rate justifies a segmented vec |
| `PlanMemo` | `RwLock`, double-checked insert | already true — it has been `Arc<Mutex<PlanMemo>>` since [S1a.6.8](../p1a.6_performance/s1a.6.8_compile_cache_and_extents.md) |

The two middle rows are what this stage measured, and both moved.

---

## 2. The fact store is a read structure with a rare append

One process per file, root saturated, then the search — so every column below
is the **search's** work, which is the only region P1a.7 shares anything
across.

| workload | enterings | fact-store **reads** | `intern` calls | `probe` | **ids assigned** | intern : new |
|---|---:|---:|---:|---:|---:|---:|
| `zebra -e` | 111 | 1 170 570 | 74 626 | 212 015 | **230** | 324 : 1 |
| `zebra2 -e` | 101 | 884 005 | 28 536 | 135 604 | **118** | 242 : 1 |
| `branching/06 -e` | 5 173 | 10 186 284 | 2 318 815 | 133 882 | **417** | 5 561 : 1 |
| `branching/07 -e` | 11 501 | 9 102 221 | 640 806 | 162 | **367** | 1 746 : 1 |
| `sq-bwd/houses -e` | 21 699 | 5 800 172 | 635 453 | 668 000 | **24** | 26 477 : 1 |
| `features/01 -e` | 384 167 | 26 114 237 | 3 444 921 | 881 220 | **41** | **84 022 : 1** |

*reads* is the borrow-returning path — `rel`, `args`, `row`, `get`; `intern`
is every interning call, hits included; *ids assigned* is how many of them
created a row.

**The whole search of the phase's largest workload assigns forty-one fact
ids.** 384 167 enterings, 26 M reads, and the append that a lock-free
segmented vec exists to make cheap fires 41 times. `branching/06 -e` is the
densest of the six and still only 417 in 2.3 M interning calls.

That is because interning is not deriving. A fork derives the *same*
propositions its siblings derive — the puzzle's vocabulary is fixed by load,
and layer 2 re-derives what layer 1 already numbered — so after root
saturation an interning call is a **lookup** 99.98 % of the time
(`branching/06`) and 99.999 % of the time (`features/01`).

So the sharded-map-plus-segmented-vec design is aimed at a write rate that
does not exist. What the fact store needs is the opposite shape: a **read path
that stays lock-free**, because that is the one carrying 26 M calls, and an
append that may cost whatever it likes.

The read path is also where the difficulty actually is, and design/08 does not
mention it: `FactStore::args` returns a `&[Value]` into the argument arena and
`row` returns a `&Row`. **No lock returns a borrow that outlives its guard**,
and a `Vec` reallocates on push, so "put the store behind an `RwLock`" is not a
change to the write sites — it is a change to every read site in the port.
[S1a.7.1](s1a.7.1_sync_shared_state.md) § What the measurement changes has the
three ways out and which one these numbers choose.

---

## 3. The interner does not grow during a search — now

Same question of the two tables that hand out `&str`: the symbol interner and
the integer pool. Growth is measured between the **end of root saturation**
and the end of the solve, because the search is the only region a worker
shares anything across, and root saturation is single-threaded by
construction. Over every corpus file that reaches a solve — 90 of the
manifest's 128 entries; the other 38 are the negative groups and never load.

**Before** (2026-08-22, first measurement): **24 of 90** files grew the symbol
table. **Four distinct names in the whole corpus**, and the integer pool never
grew at all:

| name | files | who interns it |
|---|---:|---|
| `<lookahead-dies-immediately>` | 19 | `hypgen::write_negated`, the kill-cache's provenance |
| `<forced-positive>` | 4 | the forced-positive cascade's promotion |
| `<monotonic-unconditional>` | 4 | the singleton `(not h)` writeback |
| `Ann` | 1 | **not the engine's** — see below |

Three of the four are the engine's own vocabulary. They are now interned by
`Terms::new` with the kernel names, as
[`ein_core::terms::ENGINE`](../../../ein.rs/crates/ein-core/src/terms.rs).

`ENGINE` holds **eight**, not three, and the extra five are worth naming
because they say where the window's edge is. Marking *after load* instead —
the first cut of this measurement — finds three more, on **95** files:
`__symmetric__` (94 of them: `Saturator::new` interns the mirror marker on
every construction, whether or not the program marks a relation) and `a` / `b`,
the two names the native mirror reports a firing's bindings under, re-interned
**per mirror firing**. None of those is a hazard, because they land during
root saturation — but they are a hash lookup on a warm path for a pair of
constants, so they moved to `Kernel` too. `__closed__` and `<query>` complete
the list: same kind of site, interned earlier only by where their writers run.

The fourth name is the interesting one. `Ann` appears in
`examples/ein-bugs/mixed-type-hypothesis.ein` only as an *argument* — in an
`hrule`'s `:assert (seat Ann ?v)` and in the query goal — and no fact mentions
it. The loader interns a pattern's **relation names** (`Pattern::relation_names`)
and its facts' arguments, but not a rule's argument constants, so the first
thing to see `Ann` was the compiler, building a plan for `guess` *inside the
hypothesis loop*. `ein_ir::from_ir::intern_program_names` now walks the
registered rules and the query at load and interns their leaves — a superset
of what the compiler asks for, which is cheaper than a second opinion about
which leaves it reads.

**After**: 0 of 90, and the sweep is `ein-infer/tests/interning.rs`, which
runs in the gate.

**What the load pass costs**, since it is work moved into a phase the
milestone has a target for: `zebra2`'s `kb load` is **0.58 ms with it and
0.55 without** — best of 7, `taskset -c 4`, release + snmalloc — so **+0.03 ms**,
which is 0.09 % of that file's 32 ms end-to-end. It is also not new work: the
compiler was going to intern those leaves, and now does not have to.

### What that buys, and the one shape it does not cover

An interner that does not grow needs **no lock and no sharding**: `&Interner`
is `Sync` already, `text` keeps returning a borrow, and not one read site
changes. design/08 §6's interner row is answered by deleting it.

One shape is outside the measurement and is named rather than hidden: a
pattern head `(?rel ?a ?b)` whose `?rel` binds to an **integer**, whose decimal
text the compiler interns as a symbol. Nothing in the corpus has one. What
makes it survivable rather than a race is that a worker which cannot intern
can only *fail to find a name* — so the fallback is to re-run that entering on
the committing thread, which
[S1a.7.2](s1a.7.2_parallel_enterings.md) owes anyway for its own reasons.

---

## 4. What this chooses

1. **The interner and the integer pool are shared by `&`.** No lock, no shard,
   no read-site change. The invariant that licenses it is a test, not a
   comment.
2. **The fact store is not a write-scalability problem.** Any scheme whose
   read path stays lock-free will do, and the append may take a mutex: at 41
   to 417 appends per search, contention is not a thing that can happen.
3. **The plan memo is already done** — `Arc<Mutex<PlanMemo>>` since S1a.6.8,
   for the unrelated reason that a memo shared across forks needed one owner.
4. **`&mut Terms` is still threaded through 99 signatures**, and that is what
   is left of this stage. The refactor is now a question about *those*
   signatures rather than about concurrency: what a worker needs is a `&Terms`
   plus a way to append, and the measurement says the second half is small.

---

## 5. Reproducing

```sh
# §2, §3 — the counters need the feature; the interner columns do not
cd ein.rs
cargo run --release -p ein-infer --features counters --example shared_state_probe

# §3's assertion, over the whole corpus
cargo test -p ein-infer --test interning

# the machine state every number above was taken under
utils/bench_env.sh --report
```
