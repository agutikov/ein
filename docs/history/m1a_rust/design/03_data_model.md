# 03 — Data model: integers all the way down

**Settles:** how atoms, values, facts, indexes and forks are represented.
**Phase:** [P1a.2](../README.md#p1a2--kb-core).
**Replaces:** `ein/kb/entities.py`, `ein/kb/store.py`, `ein/kb/views.py`,
`ein/kb/provenance.py` (storage half).

---

## 1. What ein.py stores, and what it costs

A `Fact` is `(relation_name: str, args: tuple[str | int | Fact, ...])`
with identity on exactly those two fields
(`kb/entities.py`). Everything
else — `provenance`, `raw`, `loc`, `_kb` — is `compare=False`.

Per fact, CPython holds: a `Fact` instance (frozen dataclass, ~56 B +
`__dict__`-free slots), a `tuple` for `args` (~56 B + 8 B/elem), and a
pointer per arg into a `str` object (~50 B each, shared only when
CPython happens to have interned it — which for `"House-1"` read from a
file it has **not**). Every identity comparison is a tuple compare that
recurses into `str.__eq__`; every hash is a tuple hash over string
hashes.

The profile in the [milestone README](../README.md#what-shipped)
is the bill: 46 % of an exhaustive solve is `_bind_arg`/`_bind_args`, and
31.9 M `isinstance` calls are the type dispatch that unification needs
*because* a slot may be `Var | Atom | Int | NestedPattern` and an arg may
be `str | int | Fact`.

The port's answer is uniform: **every one of those becomes a `u32`.**

---

## 2. Symbols

One global interner per process — equivalently, one per `Engine`, since
a process holds a single engine now that the resident server is dropped.
`.einb` is the one place that crosses interner boundaries, and it remaps
([10](10_binary_format.md) §3).

```rust
pub struct Symbol(u32);                 // 4 bytes, Copy, Eq, Hash

pub struct Interner {
    arena:  String,                     // all text, one allocation family
    spans:  Vec<(u32, u32)>,            // Symbol -> (start, len) in arena
    lookup: FxHashMap<&'arena str, Symbol>,
    rank:   RefCell<Option<Vec<u32>>>,  // Symbol -> lexicographic rank
}
```

Interned: relation names, object names, rule names, variable names,
keyword names, and every `String`/`Range`/`Var` literal that reaches a
fact argument. That last group matters — ein.py's `_atomic_value`
already flattens `Atom("foo")`, `String("foo")`, `Var("x") → "?x"` and
`Range(1,5) → "1..5"` **into plain `str`** before they become fact args,
so a `(likes A "foo")` and `(likes A foo)` are *the same fact* today.
The port must preserve that collapse, and interning does so naturally.

### The rank table

`Symbol` ids are assigned in first-seen order, so `Symbol` ordering is
**not** lexicographic — but several observable sites sort by name
([02](02_determinism_and_order.md) §3b). `rank` maps `Symbol → position
in the lexicographically sorted symbol list`, so those sorts become
`u32` sorts.

It is cheap to maintain because the symbol table is effectively frozen
after load: rules cannot fabricate atoms (an `:assert` template's slots
are `Atom`/`Int` literals or bound variables), so saturation and search
add facts but no symbols. Build `rank` lazily on first use, invalidate on
interner growth, rebuild on next use. On zebra2 that is one sort of
~150 strings, once.

---

## 3. `Value` — 4 bytes, three shapes

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Value(u32);                  // [tag:2][payload:30]

const TAG_SYM: u32 = 0;   // payload = Symbol
const TAG_INT: u32 = 1;   // payload = index into the int pool
const TAG_FACT: u32 = 2;  // payload = FactId (a nested relational node)
```

- **Symbols** cover every textual arg (§2).
- **Ints** go through a small pool rather than being inlined, for one
  reason: Python integers are unbounded and `INT: /-?[0-9]+/` accepts any
  width. The pool stores the *canonical decimal form* (parse, then
  re-render — so `007` and `7` and `-0`/`0` collapse exactly as
  `Int(value=int(tok))` does) plus an `Option<i64>` fast field for the
  overwhelmingly common case. Two ints are equal iff their pool ids are.
- **Nested facts** are just `FactId`s. This is the design's quiet win:
  `(not (color-loc Red House-1))` is a fact whose single arg is a
  `Value` tagged `FACT`, so the negation index becomes a **bitset over
  `FactId`** and `contradiction.contradicts()` becomes one bit test
  instead of a tuple lookup in a `set`.

**Ordering trap.** `Value`'s derived `Ord` (tag, then payload) is
identity order, not semantic order. Anywhere ein.py's behaviour depends
on sorting by name, ein.rs must sort by `(tag, rank[sym] | int_value)`.
Two comparators, two names, and a lint that the raw one never appears in
a `sort` at an observable site. See [02](02_determinism_and_order.md) §3.

---

## 4. Facts — interned rows

```rust
pub struct FactId(u32);

struct Row { rel: Symbol, args_at: u32, arity: u16, _pad: u16 }   // 12 B
// …20 B since T1a.6.2.6, with `inline: [Value; 2]` — see the note below

pub struct FactStore {
    rows:   Vec<Row>,
    args:   Vec<Value>,                        // flat arena, 4 B/arg
    lookup: FxHashMap<RowKey, FactId>,         // hash+eq over (rel, args)
}
```

**Interning a fact is global and side-effect free.** `intern(rel, args)
-> FactId` says "this proposition has this number"; it does **not** say
the proposition is believed. Belief is per-KB (§5). That separation is
what makes forking cheap and, more importantly, correct: a fork may
intern freely without the parent ever seeing the proposition.

Consequences:

| ein.py | ein.rs |
|---|---|
| `Fact.__eq__` → tuple compare → per-arg `str.__eq__` | `FactId == FactId` → `u32` compare |
| `Fact.__hash__` → tuple hash | `FactId` is the hash |
| `kb._fact_by_id(rel, args)` — O(deg(rel)) linear scan over `_facts_by_relation` | intern lookup + presence bit — O(1) |
| `(rel, args)` tuples as dict keys (`_alt_justifications`, `_negated_facts`, `FactId` in nogoods/apriori) | `FactId` — dense `u32`, usable as a `Vec` index |
| a nested `Fact` arg is an unregistered object | a nested arg is a `FactId` like any other |

Memory, zebra2 post-saturation (381 facts, mean arity ≈ 2.2):
~381 × 12 B rows + ~840 × 4 B args ≈ **8 KB**, contiguous. The
equivalent Python object graph is ~60–80 KB scattered across the heap.

> **The row is 20 bytes since T1a.6.2.6 (2026-08-19), and bigger on purpose.**
> A whole exhaustive `zebra` interns 1 104 facts — the store is **22 KB** and
> has never left L1 — so the size of a row was never a cache-footprint
> question. What a candidate pays is a *dependency chain*: `rows[id]`, then
> `args[row.args_at]`, whose address the first load produces, and twice over
> because most premises are nested (`(not (R …))`). `Row` now carries
> `inline: [Value; 2]`, which holds the arguments of **96.6 %** of `zebra`'s
> facts and 83.5 % of `zebra2`'s outright — the arity histogram is bimodal at
> 1 and 2 — and `FactStore::row` + `args_of` let the matcher read the row once
> and take the relation and the arguments from it. Worth **−8.5 %** on
> `solve zebra2 -e` and −4.7 % on `solve zebra -e`;
> [baseline.md § 13](../measurements/baseline.md#t1a622-and-t1a626--the-candidate-loop-and-the-two-tasks-that-swapped-places)
> has the three-step measurement, including the two intermediate forms that
> each lost one of the two puzzles.

`CanonicalSetId` (a commitment) becomes `SmallVec<[FactId; 4]>`;
no-good clauses become sorted `Box<[FactId]>` (or a `u64` bitmask when
the alive set is ≤ 64 — a fast path worth measuring in
[P1a.6](../README.md#p1a6--performance)); `state_key` becomes a sorted
`Box<[FactId]>` compared by `memcmp`.

---

## 5. The KB — a base and a stack of deltas

ein.py's `fork()` shallow-copies the fact list and six index dicts;
`snapshot()` does the same plus a `_nogoods` copy. Measured cost today is
negligible (0.003 s / 206 calls on exhaustive zebra2) *because there are
only 101 enterings*. The port needs forks to be free for two reasons
that do not exist yet: [P1a.7](../README.md#p1a7--parallelism) wants
hundreds live at once, and [05](05_matcher.md)'s beta-memories are only
affordable if a fork does not copy them — the exact objection
[F11](../../../../plans/followups/f11_deductive_layer_perf.md) parks D1 on.

```rust
pub struct KbCore {           // immutable once published; shared by Arc
    facts:      Vec<FactId>,              // insertion order
    present:    BitSet,                   // FactId -> believed here
    by_rel:     FxHashMap<Symbol, Vec<FactId>>,
    by_rel_slot_val: FxHashMap<(Symbol, u8, Value), Vec<FactId>>,
    negated:    BitSet,                   // inner FactId of every (not X)
    rule_apps_by_rule: FxHashMap<Symbol, Vec<FactId>>,
    rule_apps_on_rel:  FxHashMap<Symbol, Vec<FactId>>,
    names:      FxHashMap<Symbol, NameRef>,
    prov:       ProvStore,
}

pub struct Kb {
    base:  Arc<KbCore>,
    delta: Delta,             // same shape, but only what this branch added
}
```

- **Read** = check `delta`, then `base`. For the ordered lists that means
  *concatenated iteration*, base first — which is exactly the order
  ein.py produces (copy the list, then append).
- **Write** = append to `delta` only. `base` is never mutated, so it can
  be shared by `Arc` across threads with no lock.
- **`fork()`** = clone the `Arc` + an empty `Delta`. O(1).
- **`snapshot()`** = the same, plus copying `_nogoods` (which ein.py
  copies for snapshots and shares for forks — preserve the distinction;
  the reason is documented in `store.py` and is about archival isolation).
- **Flatten** = when a delta grows past a threshold (or when a fork
  becomes a new root, e.g. a forced-positive promotion), materialise
  `base + delta` into a fresh `KbCore`. Bounded work, amortised.

> **The threshold was never built, and S1a.6.2 measured why it should not be
> (2026-08-19).** P1a.2 shipped the layered KB without one and `Kb::flatten`
> has a single caller, a test. T1a.6.2.5 built the strongest form of the idea
> — a KB-level flat extent per relation, so `facts_of` is one hash lookup
> instead of a chain over 24 layers — and it is **+7.6 % on `solve zebra -e`**
> at identical work, identical output and identical allocation counts, while
> `match_hot` got **8 % faster** and `boundary` 5–7 %. The benches that
> improved are the ones that never fork: a fork shares its parent's index
> vectors behind an `Arc`, so the ~450-fact extent the matcher scans is *one*
> copy read by all 24 live KBs on the search stack, and flattening hands each
> fork a private copy to fill a cache with. Reverted;
> [baseline.md § 13](../measurements/baseline.md#t1a625--the-flatten-threshold-was-never-built-and-building-it-costs-76)
> has the isolation control. The consequence is a *positive* one for
> [P1a.7](../README.md#p1a7--parallelism): sharing the base index across
> workers is worth more than shortening the chain.

> **What this section did not ask, and S1a.6.1 measured (2026-08-18).** A
> read that *aggregates* over layers is O(depth), not O(1), and one of them is
> hot: `Kb::n_facts_of` — a relation's extent **size**, which the NAF boundary
> asks 644 166 times on an exhaustive `zebra2` and ein.py answers with one
> `len()` on a flat dict. An exhaustive search reaches **35 layers**, and the
> count is **9.5 % of the run**
> ([baseline.md §7](../measurements/baseline.md#7-the-top-five-costs)
> item 2). The fork is O(1) as designed — a delta is 3.6 KB mean over 101
> enterings, which is the number P1a.7 wanted — and the bill landed on the
> other side of the trade.
> [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts) keeps
> per-relation counts so the answer is O(1) at any depth; the flatten
> threshold above gains a second reason to be tuned rather than guessed.

The registries (`relations`, `rules`, `hrules`, `macros`, `query`,
`config`) are immutable after load and live in an `Arc<Program>` shared
by every KB — matching ein.py's share-by-reference exactly, including its
documented caveat that a shared entity's back-pointer sees the *root*'s
facts. ein.rs has no back-pointers (accessors take `&Kb` explicitly), so
that caveat evaporates without changing any behaviour that depended on
it: no engine path uses `Relation.facts` on a fork.

### Why this is trivially correct

The KB is **append-only within a run** — the property S1.9.E23's
fail-fast and S1.21.8's monotone-growth argument already lean on. A
layer that only adds, over a base that never changes, cannot disagree
with a copy that was mutated in place. The invariant is checked in
debug builds by a `flatten()`-and-compare assertion in the conformance
runs.

---

## 6. The seven indexes, one by one

| ein.py index | purpose | ein.rs |
|---|---|---|
| `_facts_by_relation` | relation extent; match candidates; `_watch_stamp` sizes | `Vec<FactId>` per `Symbol`; size = `base.len + delta.len` |
| `_facts_by_rel_slot_val` | the participation index (RETE alpha-memory, S1.8.B-idx) | `Vec<FactId>` per `SlotKey` — and since T1a.6.3.0 the key reaches one level *inside* a nested argument, which ein.py's does not (see note) |
| `_negated_facts` | `(not X)` membership → hypgen Tier-A filter, `contradicts` | **`BitSet` over `FactId`** |
| `_rule_apps_by_rule` | activators for a rule | `Vec<FactId>` per `Symbol` |
| `_rule_apps_on_relation` | property facts targeting a relation | `Vec<FactId>` per `Symbol` |
| `names` (`NameRef`) | participation (`as_head` / `as_arg`) → hypgen candidate objects + popularity scoring | `FxHashMap<Symbol, NameRef>` with `Vec<FactId>` lists |
| `_alt_justifications` | alternative derivations, capped at 32, sorted by premise count | `FxHashMap<FactId, SmallVec<[ProvId; 4]>>` |

Note on the participation key: `(Symbol, u8, Value)` is 4+1+4 = 9 bytes
padded to 12. Packing to a `u64` needs the `Value`'s 32 bits and the
symbol's 32, which overflows — so either keep the 12-byte key with
`FxHash` (fine; it is one hash per Scan step) or hash the triple to a
`u64` and keep the exact key in the bucket for collision checking. Start
with the former; measure in [P1a.6](../README.md#p1a6--performance).

> **Measured, and the key grew a level instead of shrinking
> ([T1a.6.3.0](../README.md#s1a63--beta-memories-f11-d1), 2026-08-19).**
> The 12-byte key's *hashing* was never the cost; what it did not index was.
> ein.py keys the join types only — a `Fact`-valued argument is not keyed — so
> a `(not (R ?b ?i))` premise, which is `stdlib/slots.ein`'s and **99.1 %** of
> an exhaustive `zebra`'s candidates, walked the whole `not` extent. The key is
> now `(Symbol, slot: u16, inner: u16, Value)` — still 12 bytes, `inner` living
> in the padding — where `inner` names a position *inside* the nested fact or
> is `DIRECT`. Candidates 25.16 M → **1.17 M**, `solve zebra -e` 349 → 78 ms,
> and the whole firing sequence unchanged (T2 239/240) because a narrowing
> changes which facts are *offered*, not which ones match.
>
> Two consequences worth carrying: each layer also holds a **2048-bit Bloom
> filter** over its keys, because with the lookup now the common case a fork 24
> layers deep was spending 15.6 % of the run hashing the same key per layer;
> and `Kb::index_sizes` reports the `DIRECT` postings only, so `saturate`'s
> snapshot keeps describing the *knowledge base* rather than ein.rs's indexing
> of it.

`NameRef.category` (`object` / `relation` / `rule`) is computed exactly
as `_categorise_name` does — including the S1.7.6 rule that `type` /
`instance` are ordinary names unless declared.

---

## 7. Provenance

```rust
pub struct ProvId(u32);

pub enum ProvKind { Source, Rule, Hypothesis, Rejected }

pub struct Prov {
    kind:      ProvKind,
    source:    Option<Symbol>,        // source-kind sentence id
    rule:      Option<Symbol>,
    premises:  Box<[FactId]>,         // positive premises, in step order
    bindings:  Box<[(Symbol, Symbol)]>, // in BIND order — see 02 §3a
    absent:    Box<[NafRef]>,         // S1.21.8 negative premises
    branch:    Option<u32>,
    loc:       Option<Loc>,
}
```

- Primary provenance: a dense `Vec<Option<ProvId>>` indexed by `FactId`
  (per KB, in the delta/base split).
- `bindings` is stringified in ein.py (`(k, str(v))`) and lands in the
  trace. ein.rs keeps `Symbol` pairs and renders at display time — the
  rendering must match `str(v)` for each of `str`/`int`/`Fact` (a
  nested `Fact`'s `str()` is its dataclass `repr`; see
  [02](02_determinism_and_order.md) §7).
- The `MAX_ALT_JUSTIFICATIONS = 32` cap, the sorted-by-premise-count
  invariant, and the O(1) rejection fast path in
  `store.accepts_justification` port verbatim — they are semantics
  (which explanation the minimality search can find), not tuning.
- `record_justification`'s rules — rule-kind only, non-empty premises,
  terminal primaries take no alternatives — port verbatim.

---

## 8. `EqClasses`

A placeholder union-find over names in ein.py (no propagation fires;
reserved for [F4](../../../../plans/followups/f4_cross_cutting.md)'s e-graph). ein.rs
ports it as a `Vec<u32>` union-find over `Symbol` with the *same*
path-compression and union-by-first-argument behaviour, because
`fork()`/`snapshot()` copy its parent map and the copy is observable
through `classes()`. It stays inert.

---

## 9. What this does *not* change

Stated because the temptation is real:

- **No structural sharing of args across facts** beyond the arena. A
  fact's args are contiguous; two facts with equal args do not share
  storage. Deduplicating them would save little and complicate the
  interner.
- **No hash-consing of provenance.** Two firings with the same premises
  produce two `Prov` records; ein.py's dedup is by
  `(rule, premises_raw)` inside `record_justification` and is preserved
  there, not moved into the store.
- **No fact deletion, ever.** The append-only model is load-bearing for
  §5, for fail-fast (S1.9.E23), and for the monotone-growth argument
  behind `_watch_stamp`. Retraction stays modelled as "fork a fresh KB".
- **No change to fact identity.** `(relation, args)`, provenance
  excluded — the same rule as `Fact.__eq__`, `add_fact`'s dedup,
  `state_key`, and `_is_new_relative_to`.

---

## 10. Acceptance for this design

- A `flatten()`-and-compare assertion holds after every saturation in
  debug builds (layered view ≡ materialised copy, list orders included).
- `intern` is bijective on the corpus: no two distinct
  `(rel, args)` share a `FactId`, and re-interning is stable across a
  fork.
- Memory: exhaustive zebra2 peak RSS ≤ 1/5 of ein.py's.
- T1 parity on KB shape: per-relation fact counts, `names` categories,
  negated-set size, alternative-justification counts — all identical
  after root saturation on every corpus entry.

## Cross-links

- [02 — Determinism & order](02_determinism_and_order.md) — the ordering
  obligations this representation must not break.
- [05 — Matcher](05_matcher.md) — the consumer that makes the 4-byte
  `Value` pay.
- [06 — Saturation](06_saturation.md) — delta lists feed the semi-naive
  enqueue directly.
- [08 — Parallelism](08_parallelism.md) — `Arc<KbCore>` is what makes
  parallel enterings possible at all.
- [10 — Binary format](10_binary_format.md) — `.einb` is essentially
  this layout written to disk.
