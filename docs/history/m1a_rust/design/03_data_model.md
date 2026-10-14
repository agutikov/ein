# 03 — Data model: integers all the way down

**Settles:** how atoms, values, facts, indexes and forks are represented.
**Phase:** [P1a.2](../README.md#p1a2--kb-core).

---

## 1. Identity, and what it costs

A `Fact` is a relation name and an argument tuple, each argument a
symbol, an integer or a nested `Fact`, with identity on exactly those two
fields. Everything else — provenance, source text, location — takes no
part in it.

Held as objects, every identity comparison is a tuple compare that
recurses into string equality, every hash is a tuple hash over string
hashes, and unification needs a type dispatch per slot *because* a slot
may be `Var | Atom | Int | NestedPattern` and an arg may be a symbol, an
integer or a `Fact`.

The answer is uniform: **every one of those becomes a `u32`.**

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
fact argument. That last group matters — `Atom("foo")`,
`String("foo")`, `Var("x") → "?x"` and `Range(1,5) → "1..5"` all flatten
**into one symbol** before they become fact args, so `(likes A "foo")`
and `(likes A foo)` are *the same fact*. Interning gives that collapse
for free.

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
  reason: `INT: /-?[0-9]+/` accepts any width, and an ein-lang integer
  is unbounded. The pool stores the *canonical decimal form* (parse, then
  re-render — so `007` and `7` collapse, and so do `-0` and `0`) plus an
  `Option<i64>` fast field for the overwhelmingly common case. Two ints are equal iff their pool ids are.
- **Nested facts** are just `FactId`s. This is the design's quiet win:
  `(not (color-loc Red House-1))` is a fact whose single arg is a
  `Value` tagged `FACT`, so the negation index becomes a **bitset over
  `FactId`** and contradiction detection becomes one bit test.

**Ordering trap.** `Value`'s derived `Ord` (tag, then payload) is
identity order, not semantic order. Anywhere an observable depends on
sorting by name, the engine sorts by `(tag, rank[sym] | int_value)`.
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

| operation | cost |
|---|---|
| fact equality | `FactId == FactId` → `u32` compare |
| fact hash | `FactId` is the hash |
| "is `(rel, args)` believed here?" | intern lookup + presence bit — O(1) |
| keys of the justification map, the negated set, no-goods, apriori | `FactId` — dense `u32`, usable as a `Vec` index |
| a nested `Fact` arg | a `FactId` like any other |

Memory, zebra2 post-saturation (381 facts, mean arity ≈ 2.2):
~381 × 12 B rows + ~840 × 4 B args ≈ **8 KB**, contiguous.

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

A fork that copies the fact list and every index is cheap only while
there are few of them — an exhaustive zebra2 has only 101 enterings.
Forks have to be free for two reasons that do not exist yet: [P1a.7](../README.md#p1a7--parallelism) wants
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
  *concatenated iteration*, base first — the order a copy-then-append
  fork would produce.
- **Write** = append to `delta` only. `base` is never mutated, so it can
  be shared by `Arc` across threads with no lock.
- **`fork()`** = clone the `Arc` + an empty `Delta`. O(1).
- **`snapshot()`** = the same, plus copying the no-good store, which a
  snapshot copies and a fork shares — the distinction is about archival
  isolation.
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
> asks 644 166 times on an exhaustive `zebra2`, and which a flat store
> answers with one length. An exhaustive search reaches **35 layers**, and the
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
by every KB. There are no back-pointers from a registry entry to a KB
(accessors take `&Kb` explicitly), so a shared entity cannot see the
*root*'s facts from a fork.

### Why this is trivially correct

The KB is **append-only within a run** — the property S1.9.E23's
fail-fast and S1.21.8's monotone-growth argument already lean on. A
layer that only adds, over a base that never changes, cannot disagree
with a copy that was mutated in place. The invariant is checked in
debug builds by a `flatten()`-and-compare assertion.

---

## 6. The seven indexes, one by one

| index | purpose | structure |
|---|---|---|
| `by_rel` | relation extent; match candidates; watch-stamp sizes | `Vec<FactId>` per `Symbol`; size = `base.len + delta.len` |
| `by_rel_slot_val` | the participation index (RETE alpha-memory, S1.8.B-idx) | `Vec<FactId>` per `SlotKey` — and since T1a.6.3.0 the key reaches one level *inside* a nested argument (see note) |
| `negated` | `(not X)` membership → hypgen Tier-A filter, `contradicts` | **`BitSet` over `FactId`** |
| `rule_apps_by_rule` | activators for a rule | `Vec<FactId>` per `Symbol` |
| `rule_apps_on_rel` | property facts targeting a relation | `Vec<FactId>` per `Symbol` |
| `names` (`NameRef`) | participation (`as_head` / `as_arg`) → hypgen candidate objects + popularity scoring | `FxHashMap<Symbol, NameRef>` with `Vec<FactId>` lists |
| alternative justifications | alternative derivations, capped at 32, sorted by premise count | `FxHashMap<FactId, SmallVec<[ProvId; 4]>>` |

Note on the participation key: `(Symbol, u8, Value)` is 4+1+4 = 9 bytes
padded to 12. Packing to a `u64` needs the `Value`'s 32 bits and the
symbol's 32, which overflows — so either keep the 12-byte key with
`FxHash` (fine; it is one hash per Scan step) or hash the triple to a
`u64` and keep the exact key in the bucket for collision checking. Start
with the former; measure in [P1a.6](../README.md#p1a6--performance).

> **Measured, and the key grew a level instead of shrinking
> ([T1a.6.3.0](../README.md#s1a63--beta-memories-f11-d1), 2026-08-19).**
> The 12-byte key's *hashing* was never the cost; what it did not index was.
> The key covered the join types only — a `Fact`-valued argument was not
> keyed — so a `(not (R ?b ?i))` premise, which is `stdlib/slots.ein`'s and **99.1 %** of
> an exhaustive `zebra`'s candidates, walked the whole `not` extent. The key is
> now `(Symbol, slot: u16, inner: u16, Value)` — still 12 bytes, `inner` living
> in the padding — where `inner` names a position *inside* the nested fact or
> is `DIRECT`. Candidates 25.16 M → **1.17 M**, `solve zebra -e` 349 → 78 ms,
> and the whole firing sequence unchanged, because a narrowing changes which
> facts are *offered*, not which ones match.
>
> Two consequences worth carrying: each layer also holds a **2048-bit Bloom
> filter** over its keys, because with the lookup now the common case a fork 24
> layers deep was spending 15.6 % of the run hashing the same key per layer;
> and `Kb::index_sizes` reports the `DIRECT` postings only, so `saturate`'s
> snapshot keeps describing the *knowledge base* rather than ein.rs's indexing
> of it.

`NameRef.category` (`object` / `relation` / `rule`) is computed by
`Program::categorise` — including the S1.7.6 rule that `type` /
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
- `bindings` lands in the trace. The engine keeps `Symbol` pairs and
  renders at display time — a symbol as its text, an integer in decimal,
  a nested `Fact` as its `repr` (see
  [02](02_determinism_and_order.md) §7).
- The `MAX_ALT_JUSTIFICATIONS = 32` cap, the sorted-by-premise-count
  invariant, and the O(1) rejection fast path in
  `accepts_justification` are semantics (which explanation the
  minimality search can find), not tuning.
- `record_justification`'s rules — rule-kind only, non-empty premises,
  terminal primaries take no alternatives — are semantics too.

---

## 8. `EqClasses`

A placeholder union-find over names (no propagation fires; reserved for
[F4](../../../../plans/followups/f4_cross_cutting.md)'s e-graph): a
`Vec<u32>` union-find over `Symbol` with path compression and
union-by-first-argument, because `fork()`/`snapshot()` copy its parent
map and the copy is observable through `classes()`. It stays inert.

---

## 9. What this does *not* change

Stated because the temptation is real:

- **No structural sharing of args across facts** beyond the arena. A
  fact's args are contiguous; two facts with equal args do not share
  storage. Deduplicating them would save little and complicate the
  interner.
- **No hash-consing of provenance.** Two firings with the same premises
  produce two `Prov` records; the dedup is by
  `(rule, premises)` inside `record_justification` and stays there, not
  in the store.
- **No fact deletion, ever.** The append-only model is load-bearing for
  §5, for fail-fast (S1.9.E23), and for the monotone-growth argument
  behind the watch stamp. Retraction stays modelled as "fork a fresh KB".
- **No change to fact identity.** `(relation, args)`, provenance
  excluded — the same rule for fact equality, insertion dedup,
  `state_key`, and "is this fact new relative to that KB".

---

## 10. Acceptance for this design

- A `flatten()`-and-compare assertion holds after every saturation in
  debug builds (layered view ≡ materialised copy, list orders included).
- `intern` is bijective on the corpus: no two distinct
  `(rel, args)` share a `FactId`, and re-interning is stable across a
  fork.
- KB shape — per-relation fact counts, `names` categories, negated-set
  size, alternative-justification counts — pinned after root saturation
  on every corpus entry.

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
