# 02 — Determinism and iteration order

**Settles:** every place where iteration order is observable, and the
structure that fixes it.
**Phase:** audited in [P1a.0](../README.md#p1a0--corpus-and-shared-assets),
consumed by [P1a.1](../README.md#p1a1--ir-frontend)–[P1a.5](../README.md#p1a5--presentation-and-cli).

---

## 1. Why order is the hard part

Nothing in the engine's *semantics* depends on iteration order — S1.5a.1a
made the search order content-determined, S1.21.8 demoted the priority
bands from load-bearing to advisory, and shuffle-invariance is a pinned
property. But the engine's *observables* depend on it everywhere: which
of two equally-valid matches fires first decides the firing sequence,
which decides the trace, which decides `--trace out.md`'s bytes; which
candidate hypgen yields first decides `stats.filtered` attribution; and
so on down to `--dump-states`.

So a reproducible output is, almost entirely, the problem of **fixing
every observable order**. This document is the audit. It is meant to be
exhaustive; anything found later that is not here is a bug in the audit
and gets added.

Three ordering facts the engine leans on:

- **An ordered container and a hash container differ.** A `Vec` or an
  `IndexMap`-shaped registry keeps insertion order; a hash map or set
  does not, and its iteration order depends on hash values and insertion
  history. Every hash container is therefore either (a) only ever
  membership-tested, or (b) sorted before use. The audit confirms this
  holds — see §4.
- **Names sort by Unicode code point**, which is what `Ord for str`
  gives: UTF-8 byte order is code-point order.
- **Every observable sort is stable**, so a sort on a partial key leaves
  ties in their input order: `sort_by` / `sort_by_key`, and
  `sort_unstable_*` **never at an observable site**.

---

## 2. Ordered containers

| container | order semantics | structure |
|---|---|---|
| the registries — relations, rules, hrules, macros | insertion order; iterated by hypgen's `raw_candidates`, `Engine::compile_all`, hrule matching, `rebuild_indexes` | `IndexMap`-shaped: `Vec<T>` + `FxHashMap<Symbol, u32>` |
| the fact list | append order; drives `rebuild_indexes`, `all_facts()`, `state_key` input | `Vec<FactId>` |
| a relation's extent | append order; **drives match order** via candidate selection | `Vec<FactId>` per relation |
| a participation bucket `(R, i, v)` | append order; the narrowed candidate list | `Vec<FactId>` |
| `names` | unordered; every consumer sorts or looks up by key | `FxHashMap<Symbol, NameRef>`; consumers sort |
| an engine's plan list | insertion order; iterated by the enqueue pass, the pending check, `closed`'s producible relations, `naf_deps` | `Vec<JoinPlan>` + `FxHashMap<CacheKey, PlanId>` |
| `bindings` (match result) | **binding order** = order vars were first bound; lands in `Provenance.bindings` and thus in the trace | registers + a `SmallVec<[VarSlot; 8]>` bind trail ([05](05_matcher.md)) |
| `Provenance.premises` | plan step order (a seeded match puts the seeded fact at its own step) | `SmallVec<[FactId; 4]>` in step order |
| the saturator's queue / parked set | min-heap on `(priority, tiebreaker, …)` | `BinaryHeap<Reverse<(u32, u64)>>` + side table |

### The heap is exactly reproducible

An agenda entry is keyed `(priority, tiebreaker)`, and the tiebreaker is
a per-saturator counter incremented on every enqueue, so **no two entries
ever compare equal on the key** and comparison never reaches the payload
(the plan, which is not orderable). Pop order is therefore a total
function of the key sequence, and any correct priority queue gives it —
here a `BinaryHeap` with a reversed `(priority, tiebreaker)` key.

The same argument covers the parked set, including the re-push of the
rejected candidates at the end of a boundary round: whatever the heap's
internal array holds, its pop order is fixed.

---

## 3. Order-sensitive sites — the audit

Each row: the site, what it orders, and the obligation. Sites are
grouped by which output they can perturb.

### 3a. Sites that change the firing sequence (→ trace, event stream)

| site | orders | obligation |
|---|---|---|
| candidate selection (`match_`) | the fact list scanned for a `Scan`/`Join` | preserve per-relation and per-`(rel,slot,val)` append order; **and pick the bucket of the *first* slot with a known value**, scanning the arg slots left to right |
| step execution (`match_`) | left-deep, steps in plan order | the register machine must not reorder joins ([05](05_matcher.md) § Join order) |
| the seeded match (`run_seeded`) | seeds at *each* matching step index `i`, in step order, with the seeded fact at its own premise position | the same loop |
| `run` / `run_guarded` | `steps` first, then each `extra_match_plans` disjunct in order | the same |
| `Engine::compile_all` | rules × activators (the rule's applications, in append order) | ordered registries |
| `enqueue_pass` | full pass iterates the plan list; delta pass iterates the delta facts then `pos_index[rel]` | `pos_index` values are `Vec<PlanId>` built by iterating the plan list in order |
| `closure_step` | mirror firings before rule firings; the mirror queue is a **LIFO** | `Vec` used as a stack — not a queue, despite the name |
| `next_mirror_firing` | cold seed iterates the symmetric relations, then each relation's extent | **sorted by name** — a hash-ordered set here is the one genuine hazard found. See §5 |
| `admit_from_boundary` | parked pop order; first passing candidate wins | the heap argument above |
| `first_failing` | guards in tuple order | the same |

### 3b. Sites that change search-layer counters

| site | orders | obligation |
|---|---|---|
| `hypgen::candidate_objects` | name order | sort by interned *strings*, not `Symbol` ids |
| `hypgen::generate` | objects sorted by `(-participation, name)`, **stable** | stable sort with that key |
| `hypgen::raw_candidates` | relations in declaration order, then slot index ascending | ordered registry |
| `hypgen::fill_slot` | `candidate_objects` again (name order) | same |
| `apriori::layer_1` | sorts `alive` | see §5 — mixed-type args |
| `apriori::apriori_prefix_join` | the sorted previous layer, then the `break` on prefix mismatch | the same comparator |
| `apriori::order_candidates` | sorted (lex) or by `(-score, c)` | stable |
| `nogoods::emit_nogood` | subsumption scan over a set — **order-free** (removal + insert) | any container |
| `solve`'s `record_node` | the smaller of two sorted commitments for one state | the same comparator |

### 3c. Sites that change output bytes

| site | orders / formats | obligation |
|---|---|---|
| `canon::state_key` | the sorted fact list | **identity only** — any total order is equivalent (see §6). *But* the lattice dump sorts nodes by the `repr` of the state key, so under `--dump-states` the repr order **is** observable → needs `repr` (§7) |
| `explain::build_graph` / `recorded_fallback` | sorted by `repr` over facts; the fallback keys on the core's `repr`s, sorted and space-joined | the `repr` of a `Fact` (§7) |
| the trace's answer and `linearize` | several sorts over sets of strings | string sort |
| `kb_dot`'s `schema_nodes` | the sorted type set, the sorted instances that are not types | string sort |
| `dot_util::hashed_id` | content-hash-derived node ids | **verify the hash** — see §8 |
| `solve`'s `print_final` | the core sorted by `(relation, its args' s-exprs)` | the same key |
| `naf_deps` | derived and declared, each sorted | string sort |
| `rebuild_indexes` | rules by relation = sorted names | string sort |
| `SolverConfig` field order (`--dump-config`) | declaration order | a fixed array in that order |
| `HypGenStats`' report lines | the filter and pre-candidate counter names, sorted | string sort over the counter names |

---

## 4. `set` audit — where non-determinism could have leaked and doesn't

Every hash set in the engine, and its use:

| container | use | verdict |
|---|---|---|
| the saturator's `seen` | membership | safe |
| the engine's `fired` | membership | safe |
| the negated set | membership | safe |
| the no-good store | membership + subsumption (order-free) | safe |
| `NafGuard.watched` / `scope` | sorted before use in the watch stamp; `scope` used as a filter | safe |
| a plan's shared vars | informational only | safe |
| `explain` env sets | sorted by `repr` before emission | safe |
| `unsat_core` (a set of facts) | sorted at *most* display sites | **hazard — §5 H4.** Two sites did not sort: the trace slice's `⊥` edges and the lattice dump's `unsat_core.jsonl`. Both do |
| `alive` (a set of `FactId`s) | sorted in `layer_1`; membership elsewhere | safe *modulo* §5 |
| the symmetric relations | **iterated** in `next_mirror_firing`'s cold seed and the pending-mirror check | **hazard — §5** |

---

## 5. The four real hazards

> H1–H3 were predicted by this audit and confirmed at
> [S1a.0.1](../README.md#s1a01--corpus-manifest-and-determinism-audit).
> **H4 was not** — the determinism harness found it on the second corpus
> entry it looked at. That is the argument for building the instrument
> before the thing it measures, restated as evidence.

### H1 — set iteration in the symmetric mirror

`next_mirror_firing` cold-seeds the mirror queue by iterating the
relations marked `(__symmetric__ R)`. With two or more markers, a
hash-ordered set there makes the seed order — and therefore the mirror
firing order — depend on hash values, so one puzzle could produce two
firing sequences across runs.

The seed iterates them **sorted by name**, pinned by
`saturator.rs`'s `the_mirror_seeds_its_relations_in_name_order`. The
current corpus may not trigger it (zebra2 marks few relations); the
fuzzer will.

### H2 — sorting `alive` with mixed-type args

`layer_1` sorts `alive`, and two facts of the *same* relation may hold a
symbol in slot *i* in one and an integer in the other — or, from an
`hrule` whose `:assert` head is a `(not …)`, a nested `Fact` in each.

**Confirmed at S1a.0.1, with a narrower scope than expected.** Blind
hypgen cannot reach it: it builds candidates out of the KB's names, and
only symbols enter those. Only an `hrule` can carry a non-symbol through,
because its `:assert` args come from bindings. Reproducers:
[`examples/ein-bugs/mixed-type-hypothesis.ein`](../../../../examples/ein-bugs/mixed-type-hypothesis.ein)
and
[`nested-fact-hypothesis.ein`](../../../../examples/ein-bugs/nested-fact-hypothesis.ein).

`Value` has a total order by construction, so the sort always answers:
`Int < Sym < Fact` by tag, then within a tag
([defined_behaviour.md §2.1](../../../kernel/defined_behaviour.md#21-fact-arguments-are-totally-ordered-int--sym--fact)).

### H3 — `--shuffle` is one seeded generator across layers

`--shuffle` builds one generator from `lattice_order_seed` and shuffles
each layer's candidates with it, carrying its state across layers, so
the traversal is reproducible only if the generator is reproduced bit for
bit. It is MT19937 with the standard `init_by_array` seeding — over the
seed's absolute value split into 32-bit words, least significant first —
and the shuffle is a downward Fisher–Yates that, for `i` from `len − 1`
down to 1, swaps `x[i]` with `x[randbelow(i+1)]`, `randbelow` drawing by
rejection from `getrandbits` (`ein-infer/src/mt19937.rs`,
[defined_behaviour.md §3](../../../kernel/defined_behaviour.md#3-search)).
≈ 60 lines, deterministic, pinned by a table of draws.

**Confirmed at S1a.0.1**: two `--shuffle` runs at the same seed are
byte-identical, and across seeds the verdict, every counter and the root
shape agree — only the *order* the k models are found in moves. That is
why `--json-summary` sorts its `solutions` array by model: leaving the
engine's order there would report a difference on exactly the runs whose
point is that there is none.

### H4 — `unsat_core` iterated raw at two display sites

The §4 audit recorded `unsat_core` as "sorted at every display site".
Two sites were not:

- the trace slice's refuted-branch block iterates the core to emit
  `<fact> -> "⊥"` edges. That DOT lands verbatim in `solve --trace`
  output, so in hash order **the same puzzle produces two different
  trace files across runs** — one transposed edge on
  `examples/branching/04_two_levels.ein`.
- the lattice dump writes `unsat_core.jsonl` one line per core fact in
  iteration order, so the `--dump-states` tree carried the same
  instability.

Both sort by `repr` — rather than by a bare comparator because it is the
explanation's existing convention and is total over the mixed arg types
H2 describes.

The audit was wrong in a specific and instructive way: it classified the
container by what *most* of its consumers do. The rule that replaces it —
for the remaining §4 rows — is that a set is safe only when **every**
reader is checked, and "safe" is a claim about call sites, not about
containers.

---

## 6. Where order provably does *not* matter

Worth stating explicitly so the engine does not over-constrain itself:

- **`state_key` identity.** The key is a sorted permutation of a
  multiset. For any two KBs, sorted-by-X keys are equal iff the
  multisets are equal iff sorted-by-Y keys are equal. So the engine sorts
  by `FactId` (a `u32` sort) instead of `repr` **for identity purposes**
  — 1-2 orders of magnitude cheaper — and needs `repr` ordering only
  where the key is *displayed* (§7). P1.21 R1's rule ("identity
  is the representation, never a hash of it") is preserved: the
  representation is still the full sorted vector; only the sort key
  changes.
- **Binding keys** (`binding_key`, `fired`): hashed and compared, never
  iterated.
- **No-good subsumption**: emitting clause *c* removes every stored
  superset and inserts *c*; the resulting minimal set is order-free.
- **Index rebuild**: `rebuild_indexes` is a pure function of the fact
  list's order, which is preserved.

---

## 6a. The no-good store under `--jobs N` — the timing half of §6

> **Added 2026-08-29 by
> [M1e S1e.1.2](../../m1e_review_processing/README.md#s1e12--determinism-under---jobs-q1),
> answering the
> [review](../../m1e_review_processing/review/open-questions.md)'s
> Q1.** Everything below describes the engine as shipped; the section is new,
> the behaviour is not.

§6's third bullet says the store's **content** is order-free: emitting *c*
removes every stored superset and inserts *c*, so the minimal set is a function
of the clauses and not of their arrival order. That is a claim about a *set*.
What it does not say — and what `--jobs N` needs — is that the store's **state
at every read** is a function of the candidate order.

The gap is real and not hypothetical. `Kb::branch` hands every worker of a
fanned-out layer the same `Arc<RwLock<Nogoods>>`
(`ein-core/src/kb.rs:664`), and [08](08_parallelism.md) §2's fan-out predicate
is about facts — *a layer is fanned out iff it cannot write a **fact** to
root* — while a clause is not a fact and every dead entering of every fanned-out
layer produces one. So a clause learned mid-layer is *reachable* by a worker
mid-entering, and until this section nothing said why that cannot decide
whether the worker's candidate lives.

### Every access to the store, and this is all of them

| | site | lock | thread, and when |
|---|---|---|---|
| **W1** | `nogoods::emit_nogood` — `ein-infer/src/nogoods.rs:74` | write | from `Run::handle_dead` (`solve.rs:2341`) and `Run::integrate` (`solve.rs:2398`) |
| **W2** | `ein_einb::sections::read_nogoods` — `ein-einb/src/sections.rs:689` | write | loading a `.einb`, before any search |
| **R1** | `nogoods::subsumed` — `nogoods.rs:127` | read | the deferred-integration half of that same `handle_dead` (`solve.rs:2337`) |
| **R2** | `Run::generate_layer` — `solve.rs:1206` | read | **one** guard, held across the whole of layer *L+1*'s candidate generation |
| **R3** | `Run::proof` — `solve.rs:2534` | read | once, after the search |
| **R4** | `Kb::snapshot` — `kb.rs` | read (clone) | the archival copy `Run::record_node` keeps for a model |

Plus four that are not the engine: `ein-infer/src/shape.rs:445` and `:685`
(the `lattice` and `hyp` shape instruments, single-threaded by construction),
`ein_einb::sections::write_nogoods`, and `Kb::diff`'s round-trip comparison.

W1's two callers are the whole write side. `handle_dead` is called only from
`Run::commit_entering` (`solve.rs:1984`), which is *"called in candidate order,
always, whether the entering was computed here or on a worker"*; `integrate` is
called only at the batch barrier and the layer barrier. Neither is reachable
from `Run::speculate` (`solve.rs:1854`), which is what a worker runs:
`commitment::try_commitment_set` and `hypgen::complete` reach no row of the
table. R2's `apriori::filter_reason` (`apriori.rs:140`) *does* run on worker
threads — `generate_layer` fans its predicate out with `par_iter` — but it is
handed the `&Nogoods` behind the single read guard the committing thread holds
for that whole pass, and takes no lock of its own.

### The argument

1. **Every write is on the committing thread**, at a point the candidate order
   fixes.
2. **`Run::fan_out` (`solve.rs:1776`) is a barrier.** The committing thread is
   inside `pool.install` for exactly as long as a worker of that batch exists,
   and it is the only writer. **So a worker cannot observe a clause arriving:
   while it lives there is no writer.**
3. Every read is therefore of a store whose contents are a function of the
   candidate order — at the layer's open, at a commit, or at the end of the
   run.

Note what this does **not** rest on. The review's guess was that commits
replaying in candidate order would mask a mid-flight difference; that is true
of the *result* but would not have been enough, because a candidate pruned by
`filter_candidate` is never entered and so has no result to replay. The
mechanism is one step earlier: the store is not read during an entering at all,
and it is not written during a fan-out at all.

### The premise, and what enforces it

The argument's whole weight is on *no worker-reachable path touches the store*,
and until 2026-08-29 that was one sentence in `Run::fan_out_this_layer`'s doc
comment — *"a store no fork reads while it saturates"* — enforced by nothing.
`emit_nogood` takes `&Kb` and locks internally, so the type system does not
stop a worker from writing the way `Interner::intern`'s `&mut self` stops one
from interning ([08](08_parallelism.md) §6). By
[`standard_of_proof.md`](../../../kernel/standard_of_proof.md) Rule 2 that is
not yet enough: *an argument suffices when its premise is itself enforced.*

So it is now checked. `Kb::freeze_nogoods` (`kb.rs:1066`) returns a `Drop`
guard that marks the store **frozen**, `Run::fan_out` takes one across the
window in which workers exist (`solve.rs:1827`), and `Nogoods::insert` /
`Nogoods::remove` panic on a write while it is set (`kb.rs:489`). It is
`assert!` rather than `debug_assert!` for `Kb::branch`'s reason — a predictable
branch on a path taken hundreds of times per run, against a failure that is
silent everywhere else. Three tests hold the mechanism itself: the store
refuses a frozen write and lifts the freeze on the way out
(`ein-core/src/kb.rs`, two unit tests), and a fanned-out run is asserted to
have *taken* the guard while `--jobs 1` is asserted never to
(`ein-infer/tests/search_invariants.rs::a_fanned_out_layer_freezes_the_clause_store`)
— because a guard that stopped being taken would break nothing and fail
nothing.

### And the counters, not only the answer

`LatticeStats` is compared in full corpus-wide, and its `elapsed_seconds` is
the one field legitimately allowed to differ: `Op::Dump("lattice")` writes
`proof_summary.json` into the tree `render_tree` inlines byte for byte, and
the normalisation blanks `elapsed_seconds` and `ts_ms` to `<ts>` because they
are wall clock.

The store itself has a write side and a read side in the counters, and only one
of them was compared. `BaseStats::nogoods_emitted` / `nogoods_subsumed` are in
`MonotonicStats`, which `jobs_does_not_move_the_answer_or_a_counter` compares
exactly, and in `solve_shape`'s `STATS` line, which `jobs_invariance` compares
byte-for-byte over the whole corpus along with the store's own clause list.
**`LayerCensus::dropped_nogood` — what the clauses took off the next layer's
join, which is the read side — was in the unit comparison not at all, and in
the corpus one only nominally**: the census is per *layer*, so it is
deliberately not in `MonotonicStats`, and its transport is the `layer` event
and the progress dumper. S1e.1.2 put it in the unit comparison
(`Ran::census`, collected through a `Dumper` that answers `reads_forks` false),
with a non-vacuity assertion that some candidate somewhere in the file set is
actually dropped by a clause.

The corpus sweep *does* reach the column, through `Op::Dump("progress")` — and
reaches it empty. `dump_shape` runs that op at `max_enterings = 60`, and at
that budget **0 of the 202 corpus entries** have a nonzero `dropped_nogood`
while **16** have a nonzero per-layer `nogoods_emitted` (measured 2026-08-29).
So the write side is compared for real corpus-wide and the read side is a
column of zeroes agreeing with itself; the unit sweep above is the only place
it is compared at all. Raising the budget re-blesses every `dump[progress]`
cell of `corpus_shapes.md5`, which is why S1e.1.2 recorded it as
[Q-M1e.14](../../m1e_review_processing/open_questions.md#q-m1e14--the-corpus---jobs-sweeps-per-layer-census-coverage-is-vacuous)
rather than taking an unpredicted golden move.

**What would re-open this**: any path reachable from `Run::speculate` that
takes `Kb::nogoods()`, and any fan-out that does not freeze. The first panics;
the second fails the freeze-count test.

---

## 7. `repr` — the value renderer

A handful of output sites sort or print the `repr` of a value. The
renderer covers exactly the shapes a fact argument can take:

| value | repr | note |
|---|---|---|
| symbol | `'text'`; `"it's"` when the body contains `'` and no `"` | the chosen quote and `\` backslash-escaped; `\t` `\n` `\r`; any other C0 control and DEL as `\xNN`; any other non-printable code point as `\xNN` / `\uNNNN` / `\UNNNNNNNN`, lower-case hex |
| `int` | canonical decimal, `-` prefix | |
| tuple | `(a, b)`; **1-tuples are `(a,)`**; `()` for empty | |
| `Fact` | `Fact(relation_name='co-located', args=('a', 'b'))` | the relation name as a symbol and the args as a tuple, a nested fact recursively |

Scope: ~120 lines in `ein-core::repr`, pinned row by row by
`ein-core/tests/format_tables.rs`. Used only at display/sort sites —
never on a hot path.

**Alternative considered and rejected:** replace the `repr` sorts with
explicit comparators. It would delete this module, but it re-baselines
every golden that shows one of those orders. Revisit only if `repr`
turns out to be a bug farm.

---

## 8. Hashes that reach the output

**Three** hash functions surface in artefacts. Two are stable digests and
the third is display-only:

- **`dot_util::hashed_id(prefix, seed)` is a stable digest.** It is
  `prefix` + the first 10 hex digits of the MD5 of `seed`'s UTF-8 bytes;
  the checked-in DOT goldens (`ein-render/tests/golden/dot/`) are the
  fixture. There is exactly one definition. Callers own `seed`
  construction (`fact_key`'s flat `rel|arg,arg` form, and the slice's
  recursive key), so **those** string builders are part of the id, not
  just the digest.
- **`palette::hash_color(name)` is a stable digest**:
  `PALETTE[sha1(name) mod 10]`, never a salted hash, so nothing about it
  varies run to run. It reaches every DOT artefact as a colour attribute
  but never as a node *id*, so a mismatch would be visible rather than
  structural. The 20-byte digest is folded under the modulus rather than
  built into a 160-bit integer — same residue, no bignum.
- **`canon::state_digest` is display-only.** It is a 64-bit hash of the
  state key, written into sanity-error messages, and stable run to run —
  but it is never identity: distinct states may share one, and nothing
  reads it back. This confirms the P1.21 R1 rule from the outside: a
  digest is never identity.

---

## 9. The determinism test

Added in [P1a.0](../README.md#p1a0--corpus-and-shared-assets):
the engine run twice must be byte-identical. It catches `HashMap`
iteration leaking in, since Rust's default `RandomState` is per-process
randomised — hence [12](12_toolchain_and_layout.md)'s rule: **`FxHashMap`
everywhere, and no iteration over any hash map at an observable site**.

A lint enforces it structurally: a `dylint` rule — or, pragmatically, a
grep in CI — forbidding `.iter()` / `.values()` / `.keys()` on a hash map
outside an explicitly annotated allow-list.

---

## Cross-links

- [05 — Matcher](05_matcher.md) — the bind trail that preserves binding
  order; the candidate-bucket rule.
- [07 — Search layer](07_search_layer.md) — hypgen and apriori ordering.
- [08 — Parallelism](08_parallelism.md) — parallel execution must
  *restore* these orders, not preserve them incidentally.
