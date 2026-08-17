# 10 — `.einb`: the binary knowledge-base container

**Settles:** how a loaded/saturated KB, and the solutions derived from
it, are stored on disk.
**Phase:** [P1a.8](../p1a.8_server_mode/README.md).
**New surface** — no ein.py counterpart. The `.ein` text format remains
the only *authoring* format and the only thing anyone edits.

---

## 1. What it is for

- **Skip the front end.** Parse + load is 0.63 s of a 5.7 s CPython
  zebra2 run and will still be the dominant fixed cost once the engine is
  fast. `kb.open` on a saturated `.einb` should be a `mmap` plus a symbol
  remap — microseconds.
- **Give the server something to spill to.** Cache eviction
  ([09](09_server_mode.md) §6) needs a cold entry to be cheap to revive.
- **Store results.** A solved puzzle's models, stats and unsat core are
  worth keeping next to the KB that produced them.
- **Ship a corpus.** A benchmark suite or an M2 evaluation set can ship
  pre-loaded KBs so the measurement isolates the engine.

Explicit non-goal: an interchange format for other tools. `.einb` is a
private, versioned cache format. Anything crossing a tool boundary is
`.ein` text or the JSON of the event protocol.

---

## 2. Container

Little-endian, 8-byte-aligned sections, designed so that the hot sections
can be used **directly out of an mmap** with no parsing:

```
+--------------------------------------------------------------+
| Header  64 B                                                  |
|   magic      "EINB\0" + 3 pad                                 |
|   format     u16 major, u16 minor                             |
|   flags      u32   (compressed sections, has-indexes, …)      |
|   n_sections u32                                              |
|   digest     [u8; 32]  BLAKE3 of everything after the header  |
+--------------------------------------------------------------+
| Section table  n × 32 B  { kind: u32, off: u64, len: u64,     |
|                            usize_uncompressed: u64 }          |
+--------------------------------------------------------------+
| Sections …                                                    |
+--------------------------------------------------------------+
```

| kind | contents |
|---|---|
| `META` | engine semver, stdlib manifest hash, source digest(s), creation info, the `SolverConfig` in force |
| `SYMBOLS` | the interner: a text blob + `(start,len)` spans **in id order** |
| `INTS` | the int pool: canonical decimal blobs + `Option<i64>` fast values |
| `FACTS` | `Row { rel, args_at, arity }` array + the flat `Value` args arena |
| `PRESENT` | the presence bitset (which interned facts this KB believes) + the insertion-order `Vec<FactId>` |
| `PROV` | provenance arena + the primary map + the alternative-justification lists |
| `PROGRAM` | the **resolved, import-flattened form list** (post-macro-expansion AST), so rules/relations/macros/query/config rebuild without touching the filesystem |
| `INDEXES` *(optional)* | the six fact-derived indexes, so a saturated KB is usable without a rebuild |
| `NOGOODS` *(optional)* | learned clauses |
| `SOLUTIONS` *(optional)* | per solution node: `state_key`, the model's `PRESENT`-style bitset delta, goal bindings, stats; plus the verdict and unsat core |

Sections are individually optional and individually compressible
(zstd). A compressed section forfeits mmap for that section only —
`FACTS`/`SYMBOLS` stay raw by default because they are the ones worth
mapping.

---

## 3. Ids across the boundary

The one real design problem: `Symbol` and `FactId` are **process-local
integers**, and a file must be loadable into a process whose interner
already holds other content.

- `SYMBOLS` is stored **in id order**, so the file's `Symbol(i)` is its
  *i*-th entry. On load, intern each entry into the live interner and
  build a translation table `file_sym → live_sym` (a `Vec<Symbol>`).
- `FACTS` likewise stores rows in `FactId` order; interning each row
  (with translated symbols) yields `file_fact → live_fact`.
- Every other section refers to ids and is remapped through those two
  tables in one linear pass.
- **Fast path:** when the live interner is empty (a fresh process
  opening one file — the CLI case, and a server cache revival into a
  private arena), both tables are the identity and the pass is skipped
  entirely. That is the mmap-and-go case.

This is also why `.einb` cannot be shared between processes as raw
memory: it is *position*-independent but not *interner*-independent. Good
enough — the cost model is "one linear remap or none".

---

## 4. Versioning and invalidation

- `format.major` bumps on any layout change; a reader refuses a newer
  major and *ignores* unknown section kinds within its major (so minor
  bumps can add sections).
- `META` carries `stdlib_manifest_hash` and the source digest(s). A
  reader that has different inputs must treat the file as a **cache
  miss**, not an error — the server does exactly that
  ([09](09_server_mode.md) §6).
- `META` carries the engine semver. A `.einb` holding *derived* state
  (saturated facts, solutions) is only valid for the engine that produced
  it; on a version mismatch the reader keeps `PROGRAM` and drops the
  derived sections rather than trusting them. Conservative and cheap.
- The header digest is checked on open unless `--trust-cache`.

---

## 5. The solution store

`SOLUTIONS` makes "store solutions" concrete without duplicating KBs:
each solution node is stored as its **delta against the KB in the same
file** — the facts the branch added — plus its `state_key` and goal
bindings. Reconstituting a model is `base.fork()` + apply the delta,
which is the same layered structure the engine already runs on
([03](03_data_model.md) §5).

That makes the common server flow cheap end to end:

```
kb.open("zebra2.einb")        # mmap, no parse, no saturate
solve.result(...)             # already stored: verdict + models
kb.query(model, goal)         # a matcher run over a mapped KB
```

---

## 6. Acceptance for this design

- **Round-trip**: `save(kb) → open()` yields a KB that is T1-identical
  (fact set and order, provenance, indexes, counters) for every corpus
  entry, in both the empty-interner and shared-interner cases.
- **Cold open** of a saturated `zebra2.einb` < 1 ms.
- **Corruption**: a truncated or bit-flipped file is rejected by the
  digest, never mis-parsed (fuzzed).
- **Forward compat**: a file with an unknown section kind in the same
  major loads with that section ignored.
- **Size**: a saturated `zebra2` KB (378 facts + provenance) under
  64 KB uncompressed.

## Cross-links

- [03 — Data model](03_data_model.md) — `.einb` is that layout, serialised.
- [09 — Server mode](09_server_mode.md) — the consumer.
- [11 — Shared assets](11_shared_assets.md) — the stdlib manifest hash
  that keys invalidation.
