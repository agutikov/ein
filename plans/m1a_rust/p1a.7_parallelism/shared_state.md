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

---

## 2a. …and a total is the wrong shape of number for it

417 ids spread one per entering is a design; 417 inside one entering is
another. An **entering** is what a worker runs, so the question that decides
the design is *how many enterings append at all* — and, if few do, *where in
their layer they sit*.

| workload | enterings | enterings that append a fact id | …a provenance record | largest within-layer index of an appending entering |
|---|---:|---:|---:|---:|
| `zebra -e` | 111 | **7** (6.31 %) | 111 (100 %) | 21 |
| `zebra2 -e` | 101 | **1** (0.99 %) | 101 (100 %) | 6 |
| `branching/06 -e` | 5 173 | **0** | 5 173 (100 %) | — |
| `branching/07 -e` | 11 501 | **1** (0.01 %) | 11 501 (100 %) | 83 |
| `sq-bwd/houses -e` | 21 699 | **0** | 21 699 (100 %) | — |
| `features/01 -e` | 384 167 | **0** | 384 167 (100 %) | — |

**Four of the six workloads never append a fact id from inside an entering at
all** — `branching/06 -e` assigns 417 of them, and every one is assigned by
the committing thread: the hypothesis generator interning a layer's
candidates, the singleton writeback, the forced-positive cascade. What a
*fork* does is derive propositions that already have numbers.

And the appends that do happen from inside an entering are in the **head** of
their layer: index ≤ 6 on `zebra2 -e`, ≤ 21 on `zebra -e`, ≤ 83 on
`branching/07 -e` — where a layer is ~2 300 enterings. Nothing appends in the
tail, which is where the parallelism is.

**That kills all three of the routes the stage was going to bench.** A worker
does not need to append to the fact store; it needs to be *told* when it
would have to, and to hand that entering back. `FactStore::intern` takes
`&mut self`, so a worker holding `&FactStore` cannot call it — the type system
is the enforcement, and there is no protocol to model in `loom` because there
is no protocol. See [§5](#5-what-this-chooses).

---

## 2b. The structure with a real write rate is the one design/08 §6 left out

Every column of §2a's `provenance record` half is 100 %.

| workload | provenance records pushed | per entering | `Vec<Prov>` bytes |
|---|---:|---:|---:|
| `zebra -e` | 6 335 | 57 | 0.6 MB |
| `zebra2 -e` | 7 280 | 72 | 0.7 MB |
| `branching/06 -e` | 201 902 | 39 | 19 MB |
| `branching/07 -e` | 170 140 | 15 | 16 MB |
| `sq-bwd/houses -e` | 271 909 | 13 | 26 MB |
| `features/01 -e` | **2 135 093** | 5.6 | **205 MB** |

`Prov` is 96 bytes plus three boxed slices (premises, bindings, `absent`), so
the byte column is a floor. [design/08
§6](../design/08_parallelism.md#6-what-must-be-sync-and-how) does not list the
arena at all, and it has exactly the fact store's borrow-returning read
(`ProvArena::get` → `&Prov`) with a write rate four orders of magnitude
higher.

It is also the phase's memory risk, already written down and now attributable:
`features/01 -e` peaks at **724 MB** at `--jobs 1`
([S1a.6.4](../p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)), and ~28 % of
that is an arena whose own module doc says the quiet part — *"a dead fork's
records are not reclaimed until the run ends"*. A job count multiplies live
forks, and every one of them writes here.

What looked tractable is that a fork's records are its own: it writes them and
reads them back for its unsat core, and no *sibling* ever names one — so a
per-worker arena dropped at join would need no lock. What stops that is that
`CommitmentSetResult` hands the fork's **`Kb` back**, so an entering that
*lives* keeps a justification table pointing into the arena — and the alive
rate is **92.3 % on `branching/06 -e` and 100 % on both `sq-bwd/houses -e` and
`features/01 -e`**, so "promote only the survivors" promotes everything. That
is [T1a.7.1.7](s1a.7.1_sync_shared_state.md#task-t1a717--the-provenance-arena)'s
to settle, with a sharper question than it started with, and it did not exist
before this measurement.

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
2. **The fact store is shared by `&` too, and workers do not append to it.**
   `intern` takes `&mut self` and therefore stays on the committing thread; an
   entering that would have appended hands itself back and is re-run there.
   Sequentially that is 0 to 7 enterings out of 111 to 384 167, and all of
   them in the head of a layer, so "run the head, fan out the tail" removes
   them. The read path — 5.8 to 26 M borrow-returning calls — is **not
   touched**, there is no dependency, no `unsafe`, and fact-id assignment
   stays **deterministic**, because every assignment happens on one thread in
   candidate order. §5 has what this rejects and why.
3. **The provenance arena is the one that needs a design**, and it did not
   have a row in design/08 §6. 100 % of enterings write it, `features/01 -e`
   writes 2.1 M records and 205 MB of them, and none of it is reclaimed until
   the run ends.
4. **The plan memo is already done** — `Arc<Mutex<PlanMemo>>` since S1a.6.8,
   for the unrelated reason that a memo shared across forks needed one owner.
5. **The event sink is not `Send`.** `events::Buffer` is
   `Rc<RefCell<Vec<u8>>>`, so a worker cannot hold one — which design/08 §3's
   "no shared queue" hid, because a sink is not a queue. It wants the shape the
   counters already have: a per-worker buffer merged at the ordered commit, so
   the stream a reader sees is the sequential one. `ein-infer/tests/shareable.rs`
   pins it, in both directions.
6. **`&mut Terms` is still threaded through 99 signatures**, and that is what
   is left of this stage. The refactor is now a question about *those*
   signatures rather than about concurrency: what a worker needs is a `&Terms`,
   and the measurement says it needs nothing else.

---

## 5. What this rejects, and what it would have cost

The stage was going to bench three ways of making a concurrent append cheap.
All three are answered by §2a rather than by a bench, because the append they
optimise happens **at most seven times** in a search and never in the part of a
layer that is parallel.

| route | what it does | why not |
|---|---|---|
| `Arc<Core>` snapshot + overlay | immutable core read by `&`, a mutex'd overlay for new rows, folded at the layer barrier | a branch on every one of 26 M reads, to serve ≤ 7 appends. Cheap, and still more than nothing for nothing |
| a lock-free segmented vec (`boxcar`) | solves the borrow problem properly | two dependent loads per read instead of one, **and** a dependency [design/12 §2](../design/12_toolchain_and_layout.md#2-dependency-policy) does not list — for a write rate of 41 per solve |
| per-worker id space, promoted at commit | keeps id assignment deterministic | the deterministic-id benefit is what route 2 above gets *for free* by never assigning off-thread. Without that motive, what is left is renumbering a fork's delta, provenance, no-good clause and state keys at the boundary — and the KB's presence bitsets are dense over `FactId`, so a reserved high band would allocate 128 MB a set |

**The bound to carry into [S1a.7.2](s1a.7.2_parallel_enterings.md).** The
per-entering rate above is *sequential*: entering 1 appends and enterings 2…n
then find the id already there. A batch of `jobs` workers all fork the same
snapshot, so siblings of an appending entering may append too, and the wasted
work is bounded by `appending enterings × jobs` — ≤ 56 of 111 on `zebra -e`
at `--jobs 8`, which is bad, and ≤ 8 of 11 501 on `branching/07 -e`, which is
not. The `max i` column is why that bound is loose enough to ignore: run the
first ~100 enterings of each layer on the committing thread and the corpus has
no appending entering left in the fanned-out tail. On `branching/07 -e` that
is 83 of a ~2 300-entering layer, and on four of the six it is nothing at all.
**S1a.7.2 measures it with threads; this stage's job was to find out whether
it needed a mechanism, and it does not.**

## 6. Reproducing

```sh
# §2, §3 — the counters need the feature; the interner columns do not
cd ein.rs
cargo run --release -p ein-infer --features counters --example shared_state_probe

# §3's assertion, over the whole corpus, and §4's audit
cargo test -p ein-infer --test interning --test shareable

# the machine state every number above was taken under
utils/bench_env.sh --report
```
