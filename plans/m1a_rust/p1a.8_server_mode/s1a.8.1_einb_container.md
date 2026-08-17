# S1a.8.1 — The `.einb` container

**Phase:** P1a.8 (Server mode)
**Estimate:** 3 days
**Depends on:** [P1a.7](../p1a.7_parallelism/README.md)
**Implements:** [design/10](../design/10_binary_format.md)

## Context

A binary knowledge base: the interner, the int pool, the fact rows and
args, the presence bitset, provenance, the resolved program, and
optionally the indexes, no-goods and solutions — laid out so the hot
sections can be used **directly out of an mmap**.

It is a private, versioned cache format, not an interchange format.
Anything crossing a tool boundary stays `.ein` text or the event
protocol's JSON.

The one real design problem is that `Symbol` and `FactId` are
process-local integers, so a file must be loadable into a process whose
interner already holds other content. The answer is store-in-id-order +
a linear remap, with an identity fast path when the target interner is
empty.

## Acceptance

- **Round-trip**: `save(kb) → open()` is T1-identical (fact set *and*
  order, provenance, indexes, counters) for every corpus entry, in both
  the empty-interner and shared-interner cases.
- Cold open of a saturated `zebra2.einb` under 1 ms; the file under
  64 KB uncompressed.
- A truncated or bit-flipped file is rejected by the digest, never
  mis-parsed — fuzzed.
- A file with an unknown section kind in the same major version loads
  with that section ignored.
- An `.einb` written by a different engine semver keeps `PROGRAM` and
  drops the derived sections.

## Tasks

### Task T1a.8.1.1 — Header and section table

64-byte header (magic, format major/minor, flags, section count,
BLAKE3 of everything after the header) and a 32-byte-per-entry section
table (kind, offset, length, uncompressed length). Little-endian only,
with an explicit check.

### Task T1a.8.1.2 — Section writers/readers

`META`, `SYMBOLS`, `INTS`, `FACTS`, `PRESENT`, `PROV`, `PROGRAM`, and the
optional `INDEXES` / `NOGOODS` / `SOLUTIONS`. `FACTS` and `SYMBOLS` stay
raw (mmap-friendly) by default; optional zstd per section behind the
`einb` feature's compression flag.

### Task T1a.8.1.3 — The remap

`SYMBOLS` in id order → intern each into the live interner → a
`Vec<Symbol>` translation table. `FACTS` in `FactId` order → intern each
(with translated symbols) → a `Vec<FactId>` table. Every other section
remapped in one linear pass. **Identity fast path** when the live
interner is empty: skip the pass entirely.

### Task T1a.8.1.4 — Zero-copy casts

The one place `unsafe` is permitted
([design/12](../design/12_toolchain_and_layout.md) §2): casting a
`&[u8]` to `&[Row]` / `&[Value]`. Confine it to one module, with
alignment and length checks, `#[repr(C)]` on every cast type, and a
fuzz target that feeds arbitrary bytes at the reader.

### Task T1a.8.1.5 — `META` and invalidation

Engine semver, stdlib manifest hash, source digests, creation info, and
the `SolverConfig` in force. Readers treat differing inputs as a **cache
miss**, not an error; a semver mismatch keeps `PROGRAM` and drops derived
sections.

### Task T1a.8.1.6 — `SOLUTIONS`

Per solution node: `state_key`, the model's fact delta against the file's
own KB, goal bindings, stats; plus the verdict and unsat core.
Reconstituting a model is `base.fork()` + apply the delta — the same
layered structure the engine already runs on.

### Task T1a.8.1.7 — CLI surface

`ein kb save <file.ein> <out.einb>` and `ein solve <file.einb>` (accept
`.einb` wherever a `.ein` path is accepted, dispatching on magic bytes
rather than extension). Keep the *output* byte-identical to solving the
text file — that is a T3 check, and it is the strongest evidence the
round-trip is faithful.

## Notes

- Do not add a `.einb` → `.ein` exporter. `_kb_to_ein_text` already
  exists for that ([S1a.5.3](../p1a.5_presentation/s1a.5.3_state_dumps.md)),
  and a second serialiser is a second thing to keep in parity.
- Resist adding a "portable" mode that avoids the remap by storing
  strings inline everywhere. The remap is one linear pass over data that
  is already in cache; the portability it would buy has no consumer.
