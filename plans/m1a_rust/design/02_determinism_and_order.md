# 02 — Determinism and iteration order

**Settles:** every place in ein.py where iteration order is observable,
and the Rust structure that reproduces it.
**Phase:** audited in [P1a.0](../p1a.0_conformance_harness/README.md),
consumed by [P1a.1](../p1a.1_ir_frontend/README.md)–[P1a.5](../p1a.5_presentation/README.md).

---

## 1. Why order is the hard part of this port

Nothing in the engine's *semantics* depends on iteration order — S1.5a.1a
made the search order content-determined, S1.21.8 demoted the priority
bands from load-bearing to advisory, and shuffle-invariance is a pinned
property. But the engine's *observables* depend on it everywhere: which
of two equally-valid matches fires first decides the firing sequence,
which decides the trace, which decides `--trace out.md`'s bytes; which
candidate hypgen yields first decides `stats.filtered` attribution; and
so on down to `--dump-states`.

So T2/T3 parity ([01](01_parity_contract.md)) is, almost entirely, the
problem of **reproducing CPython's ordering guarantees in Rust**. This
document is the audit. It is meant to be exhaustive; anything found
later that is not here is a bug in the audit and gets added.

Three ordering facts about CPython that the port leans on:

- **`dict` and `set` differ.** `dict` preserves insertion order (a
  language guarantee since 3.7); `set` does not, and its iteration order
  depends on hash values and insertion history. Every `set` in ein.py is
  therefore either (a) only ever membership-tested, or (b) `sorted()`
  before use. The audit confirms this holds — see §4.
- **`sorted()` on `str` compares Unicode code points**, and Rust's
  `Ord for str` compares UTF-8 bytes. **These give the same order** —
  UTF-8 is order-preserving on code points. So `sorted(names)` in Python
  and `names.sort()` in Rust agree for all inputs.
- **`sorted()` is stable**, so a sort on a partial key leaves ties in
  their input order. Rust's `sort_by`/`sort_by_key` are also stable
  (`sort_unstable_*` are not — **never use them at an observable site**).

---

## 2. Ordered containers ein.rs must provide

| ein.py | order semantics | ein.rs |
|---|---|---|
| `kb.relations`, `kb.rules`, `kb.hrules`, `kb.macros` | insertion order; iterated by `hypgen._raw_candidates`, `Engine.compile_all`, `hrule.Hrules`, `store.rebuild_indexes` | `IndexMap`-shaped: `Vec<T>` + `FxHashMap<Symbol, u32>` |
| `kb.facts` | append order; drives `rebuild_indexes`, `all_facts()`, `state_key` input | `Vec<FactId>` |
| `kb._facts_by_relation[R]` | append order; **drives match order** via `match._candidates` | `Vec<FactId>` per relation |
| `kb._facts_by_rel_slot_val[(R,i,v)]` | append order; the narrowed candidate list | `Vec<FactId>` |
| `kb.names` | rebuilt as a dict comprehension over a `set` union → **order is set-order**, but every consumer sorts (`sorted(kb.names)`) or looks up by key | `FxHashMap<Symbol, NameRef>`; consumers sort |
| `Engine._cache` | insertion order; iterated by `_enqueue_pass`, `_iter_pending`, `closed.producible_relations`, `naf_deps` | `Vec<JoinPlan>` + `FxHashMap<CacheKey, PlanId>` |
| `bindings` (match result) | **binding order** = order vars were first bound; lands in `Provenance.bindings` and thus in the trace | registers + a `SmallVec<[VarSlot; 8]>` bind trail ([05](05_matcher.md)) |
| `Provenance.premises_raw` | plan step order (rebuilt by `_seed_steps` to match `run`'s order) | `SmallVec<[FactId; 4]>` in step order |
| `Saturator._queue` / `_parked` | `heapq` min-heap on `(priority, tiebreaker, …)` | `BinaryHeap<Reverse<(u32, u64)>>` + side table |

### The heap is exactly reproducible

`heapq` entries are 6-tuples whose first two elements are
`(priority, tiebreaker)`; `_tiebreaker` is a per-Saturator counter
incremented on every enqueue, so **no two entries ever compare equal on
the first two elements** and tuple comparison never reaches element 2
(the `JoinPlan`, which is not orderable). Pop order is therefore a total
function of the key sequence, and any correct priority queue reproduces
it. ein.rs uses `BinaryHeap` with a reversed `(priority, tiebreaker)`
key.

The same argument covers `_parked`, including the re-push of the
`rejected` list at the end of `_admit_from_boundary`: the heap's internal
array differs between implementations, its pop order does not.

---

## 3. Order-sensitive sites — the audit

Each row: the site, what it orders, and the Rust obligation. Sites are
grouped by which output they can perturb.

### 3a. Sites that change the firing sequence (→ trace, T2)

| site | orders | obligation |
|---|---|---|
| `match._candidates` | the fact list scanned for a `Scan`/`Join` | preserve per-relation and per-`(rel,slot,val)` append order; **and pick the same bucket** — Python takes the *first* slot with a known value, scanning `arg_slots` left to right |
| `match._run_steps` | recursion = left-deep, steps in plan order | same step order; the register machine must not reorder joins ([05](05_matcher.md) § Join order) |
| `match._seed_steps` | seeds at *each* matching step index `i`, in step order, rebuilding `premises` at `prem_pos` | identical loop |
| `match.run` / `run_guarded` | `steps` first, then each `extra_match_plans` disjunct in order | identical |
| `Engine.compile_all` | `kb.rules.values()` × `_activators_for` (which reads `_rule_apps_by_rule[name]` append order) | ordered registries |
| `Saturator._enqueue_pass` | full pass iterates `cache.values()`; delta pass iterates `delta_facts` then `pos_index[rel]` | `pos_index` values are `Vec<PlanId>` built by iterating the cache in order |
| `Saturator._closure_step` | mirror firings before rule firings; `_mirror_queue` is a **LIFO** (`.pop()`) | `Vec` used as a stack — not a queue, despite the name |
| `Saturator._next_mirror_firing` | cold seed iterates `_symmetric_rels()` (a `frozenset`!) then each relation's extent | **`frozenset` iteration order leaks here.** See §5 — this is the one genuine hazard found. |
| `Saturator._admit_from_boundary` | parked pop order; first passing candidate wins | heap parity as above |
| `World.first_failing` | guards in tuple order | identical |

### 3b. Sites that change search-layer counters (T1)

| site | orders | obligation |
|---|---|---|
| `hypgen._candidate_objects` | `sorted(kb.names)` → name order | sort interned *strings*, not `Symbol` ids |
| `hypgen._generate` | `sorted(objects, key=(-participation, name))`, **stable** | stable sort with the same key |
| `hypgen._raw_candidates` | `kb.relations.values()` insertion order, then `slot_idx` ascending | ordered registry |
| `hypgen._fill_slot` | `_candidate_objects(kb)` again (name order) | same |
| `apriori.layer_1` | `sorted(alive)` over `FactId` tuples | see §5 — mixed-type args |
| `apriori.apriori_prefix_join` | `sorted(a_prev)`, then the `break` on prefix mismatch | identical comparator |
| `apriori.order_candidates` | `sorted(candidates)` (lex) or `sorted(key=(-score, c))` | identical, stable |
| `nogoods.emit_nogood` | subsumption scan over a `set` — **order-free** (removal + insert) | any container |
| `_helpers._record_setnode` | `tuple(sorted(commitment)) < tuple(sorted(cur.commitment))` | same comparator |

### 3c. Sites that change output bytes (T3)

| site | orders / formats | obligation |
|---|---|---|
| `canon.state_key` | `sorted(facts, key=repr)` | **identity only** — any total order is equivalent (see §6). *But* `_lattice_dump` sorts nodes by `repr(state_key)` for the dump tree, so under `--dump-states` the repr order **is** observable → needs `python_repr` (§7) |
| `explain._build_graph` / `_recorded_fallback` | `sorted(…, key=repr)` over `Fact`s; `" ".join(sorted(repr(f) for f in core))` | `python_repr` of a `Fact` dataclass |
| `trace.answer` / `trace.linearize` | several `sorted({…})` over sets of strings | string sort |
| `kb.render._schema_nodes` | `sorted(type_set)`, `sorted(insts - types)` | string sort |
| `render.dot_util.hashed_id` | content-hash-derived node ids | **verify the hash** — see §8 |
| `cli.solve._print_final` | `sorted(core, key=(relation_name, tuple(fact_sexpr(a))))` | same key |
| `naf_deps` | `tuple(sorted(derived))`, `tuple(sorted(declared))` | string sort |
| `store.rebuild_indexes` | `_rules_by_relation` = `sorted(names)` of a `set` | string sort |
| `SolverConfig` field order (`--dump-config`) | dataclass declaration order | fixed array in the same order |
| `HypGenStats.as_report_lines` | `sorted(self.filtered)`, `sorted(self.pre_candidate)` | string sort over the counter names |

---

## 4. `set` audit — where non-determinism could have leaked and doesn't

Every `set`/`frozenset` in the engine, and its use:

| container | use | verdict |
|---|---|---|
| `Saturator._seen` | membership | safe |
| `Engine._fired` | membership | safe |
| `kb._negated_facts` | membership | safe |
| `kb._nogoods` | membership + subsumption (order-free) | safe |
| `NafGuard.watched` / `scope` | `sorted()` before use in `_watch_stamp`; `scope` used as a filter | safe |
| `JoinPlan…shared_vars` | informational only | safe |
| `explain` env sets | sorted with `key=repr` before emission | safe |
| `unsat_core` (`set[Fact]`) | sorted at every display site | safe |
| `alive` (`frozenset[FactId]`) | `sorted(alive)` in `layer_1`; membership elsewhere | safe *modulo* §5 |
| `Saturator._symmetric_rels()` (`frozenset[str]`) | **iterated directly** in `_next_mirror_firing`'s cold seed and `_has_pending_mirror` | **hazard — §5** |

---

## 5. The three real hazards

### H1 — `frozenset` iteration in the symmetric mirror

`Saturator._next_mirror_firing` cold-seeds `_mirror_queue` by iterating
`self._symmetric_rels()`, a `frozenset[str]`. With two or more
`(__symmetric__ R)` markers, the seed order — and therefore the mirror
firing order — depends on CPython's set iteration, which depends on
string hashes, which depend on `PYTHONHASHSEED`.

**This means ein.py is not self-parity-stable today** on such a puzzle.
Verify with `PYTHONHASHSEED=0` vs `=1` in
[S1a.0.1](../p1a.0_conformance_harness/s1a.0.1_parity_contract_and_corpus.md);
if it reproduces, the fix is a one-line `sorted(...)` in ein.py — a
genuine ordering bug the port surfaced — and ein.rs then sorts too. The
current corpus may not trigger it (zebra2 marks few relations); the
fuzzer will.

### H2 — `sorted()` over `FactId`s with mixed-type args

`apriori.layer_1` does `sorted(alive)` where an element is
`(relation_name, args)` and `args` may mix `str` and `int`. Two facts of
the *same* relation whose slot *i* is `str` in one and `int` in the other
make Python raise `TypeError: '<' not supported between instances of
'str' and 'int'`. `canon.state_key` avoids this deliberately
(`key=repr`); `apriori` does not.

ein.rs's `Value` has a total order by construction (tag, then payload),
so it cannot raise. The port must therefore decide what "parity" means
for an input where Python crashes:

- **Recommended:** ein.rs orders `Int < Symbol < Fact` (tag order) and
  the harness records this as an accepted divergence (D-class:
  *ein.rs is total where ein.py raises*), with a fixture pinning that
  ein.py raises `TypeError` and ein.rs solves.
- The alternative — fixing ein.py to sort by `repr` here too — changes
  ein.py's candidate order and would re-baseline every T2 golden. Not
  worth it unless a real puzzle needs it. Tracked as Q-M1a.4.

### H3 — `--shuffle` needs CPython's Mersenne Twister

`solver._explore_layers` builds `random.Random(cfg.lattice_order_seed)`
and calls `.shuffle(candidates)` once per layer, carrying RNG state
across layers. Reproducing that in Rust means porting MT19937 seeding
(`init_by_array` on the seed) plus CPython's `random.shuffle` loop
(`for i in reversed(range(1, len(x))): j = _randbelow(i+1); x[i], x[j] = x[j], x[i]`)
and `_randbelow_with_getrandbits`. ≈ 60 lines, deterministic, testable
against a table of `Random(seed).shuffle(list(range(n)))` outputs
generated from Python. Recommended; see Q-M1a.5.

---

## 6. Where order provably does *not* matter

Worth stating explicitly so the port does not over-constrain itself:

- **`state_key` identity.** The key is a sorted permutation of a
  multiset. For any two KBs, sorted-by-X keys are equal iff the
  multisets are equal iff sorted-by-Y keys are equal. So ein.rs may sort
  by `FactId` (a `u32` sort) instead of `repr` **for identity purposes**
  — 1-2 orders of magnitude cheaper — and only needs `python_repr`
  ordering where the key is *displayed* (§7). P1.21 R1's rule ("identity
  is the representation, never a hash of it") is preserved: the
  representation is still the full sorted vector; only the sort key
  changes.
- **`frozenset` binding keys** (`_binding_key`, `Engine._fired`): hashed
  and compared, never iterated.
- **`_nogoods` subsumption**: emitting clause *c* removes every stored
  superset and inserts *c*; the resulting minimal set is order-free.
- **Index rebuild**: `rebuild_indexes` is a pure function of
  `kb.facts` order, which is preserved.

---

## 7. `python_repr` — a small compatibility module

A handful of T3 sites sort or print `repr()` of Python values. ein.rs
needs a faithful renderer for exactly three shapes:

| Python value | repr | note |
|---|---|---|
| `str` | `'text'`, `"it's"` when it contains `'` and no `"` | plus `\\`, `\n`, `\r`, `\t`, `\xNN`, `\uNNNN` escapes |
| `int` | decimal, `-` prefix | |
| `tuple` | `(a, b)`; **1-tuples are `(a,)`**; `()` for empty | |
| `Fact` | `Fact(relation_name='co-located', args=('a', 'b'))` | dataclass repr; `provenance`/`raw`/`loc`/`_kb` are `repr=False` |

Scope: ~120 lines in `ein-core::pyrepr`, with a differential test that
feeds every corpus value through both `repr()` and the Rust renderer.
Used only at display/sort sites — never on a hot path.

**Alternative considered and rejected:** change ein.py's `key=repr`
sorts to explicit comparators. It would delete this module, but it
re-baselines existing goldens and edits M1 code for the port's
convenience — exactly the direction the milestone's non-goals forbid.
Revisit only if `python_repr` turns out to be a bug farm.

---

## 8. Hashes that reach the output

Two hash functions surface in artefacts, and they are in opposite
states — verified 2026-08-17:

- **`render.dot_util.hashed_id(prefix, seed)` is portable.** It is
  `prefix + md5(seed.encode("utf-8")).hexdigest()[:10]`. ein.rs
  reproduces it with any MD5 implementation; the checked-in goldens in
  `ein.py/tests/golden/dot/` are the fixture. The four historical
  hand-rolled copies were collapsed onto this one definition in S1.7c.25,
  so there is exactly one to match. Callers own `seed` construction
  (`fact_key`'s flat `rel|arg,arg` form, and `render/slice`'s recursive
  key) — ein.rs must match **those** string builders too, not just the
  digest.
- **`canon.state_digest` is *not* portable, and not stable across
  CPython runs either.** It is `hash(state_key)` — a tuple of `str`s
  through SipHash, salted by `PYTHONHASHSEED`. Its docstring says
  display-only, and it is: `_lattice_dump` writes it to
  `state_hash.txt` and into `SanityError` messages. So **`--dump-states`
  output is not byte-stable under ein.py itself**.

  Consequence for T3: `state_hash.txt` and any `state_digest` in a
  message are on the normalisation list ([01](01_parity_contract.md) §5)
  — compared for *presence and shape* (10-16 hex/decimal chars), not
  value. ein.rs uses a stable 64-bit digest of the same canonical
  representation so that its own output is reproducible run-to-run,
  which is strictly better and costs nothing.

  This also confirms the P1.21 R1 rule from the outside: a digest is
  never identity, and nothing reads `state_hash.txt` back.

---

## 9. The determinism test, on both sides

Added in [P1a.0](../p1a.0_conformance_harness/README.md) and run by both
suites:

1. **Self-parity under hash seeds** — run the corpus under
   `PYTHONHASHSEED ∈ {0, 1, 42, random}`; T3-diff the outputs. Any
   difference is a ein.py ordering bug (H1 is the expected hit).
2. **Self-parity across runs** — ein.rs run twice must be byte-identical
   (catches `HashMap` iteration leaking in, since Rust's default
   `RandomState` is per-process randomised — hence [12](12_toolchain_and_layout.md)'s
   rule: **`FxHashMap` everywhere, and no iteration over any hash map at
   an observable site**).
3. **Cross-parity** — the T0–T3 diff from [01](01_parity_contract.md).

A lint enforces (2) structurally: a `dylint` rule — or, pragmatically, a
grep in CI — forbidding `.iter()` / `.values()` / `.keys()` on a hash map
outside an explicitly annotated allow-list.

---

## Cross-links

- [01 — Parity contract](01_parity_contract.md) — the tiers this audit
  serves.
- [05 — Matcher](05_matcher.md) — the bind trail that preserves binding
  order; the candidate-bucket rule.
- [07 — Search layer](07_search_layer.md) — hypgen and apriori ordering.
- [08 — Parallelism](08_parallelism.md) — parallel execution must
  *restore* these orders, not preserve them incidentally.
