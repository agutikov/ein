# S1a.8.1 — The `.einb` container

**Phase:** P1a.8 (Binary KB container)
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
protocol's JSON. With the server dropped (2026-08-18) the only consumers
are the CLI, the library, and whatever [M1b](../../m1b_gui/README.md)
loads through it — which is why T1a.8.1.7's CLI surface *is* the
acceptance surface.

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

## What shipped — 2026-08-21

An eighth workspace crate, `ein-einb`, and the CLI surface that drives it.
`cargo test --workspace` is **575 tests** where it was 544.

| task | what landed |
|---|---|
| T1a.8.1.1 | `header.rs` — the 64-byte header, the 32-byte section table, ten section kinds, BLAKE3 over everything after the header |
| T1a.8.1.2 | `sections.rs` — `META`, `SYMBOLS`, `INTS`, `FACTS`, `PRESENT`, `PROV`, `PROGRAM`, `NOGOODS`; `solutions.rs` for `SOLUTIONS` |
| T1a.8.1.3 | `tables.rs` — the four translation tables, and `Maps::identity` so the fast path is *tested* rather than assumed |
| T1a.8.1.4 | `cast.rs` — the repo's only `unsafe`, four functions, every one with its alignment and length check written down |
| T1a.8.1.5 | `meta.rs` — engine semver, stdlib manifest hash, source digests, the `SolverConfig`, and `Freshness` |
| T1a.8.1.6 | `Solutions::of` / `SolutionNode::reconstitute` — the delta store, and `base.fork()` + delta to get a model back |
| T1a.8.1.7 | `ein kb save [--saturate]`, and a `.einb` accepted wherever a `.ein` path is, by magic bytes |

### The five decisions that are not in the design

**`PROGRAM` is canonical text, not the AST arenas.** design/10 §2 says "the
resolved, import-flattened form list (post-macro-expansion AST)" and does not
say in what encoding. Measured, the arenas for a resolved `zebra2` are 3 024
nodes and 3 024 optional `Loc`s — **past 60 KB before a single fact is stored,
against a 64 KB budget for the whole file** — while `dump_canonical` of the
same forms is **11 KB**. The stage's own note ("a second serialiser is a second
thing to keep in parity") points the same way: the frontend already has one.
What it costs is a parse of already-resolved text on open, measured at 202 µs
of the 681 µs cold open.

**No `INDEXES` section.** It is optional in design/10 and this writer never
emits one: every reverse index is a projection of the fact list in insertion
order, and `Kb::rebuild_indexes` *is* that projection — so a rebuilt index is
not equal to the original by argument, it is produced by the function that
defines what the original is. A reader still skips a kind-8 section rather than
refusing it. What this **did** turn up is that one derived structure is *not* a
projection: `rules_by_relation` is taken once at the end of `load` and then
shared by reference, so recomputing it from a saturated fact set produces a
larger map than the KB ever had. `Kb::rebuild_indexes_from` is the fix, and the
round trip is what found it.

**`PROV` is read before the registries are rebuilt.** Rebuilding pushes the
loader's own records into the arena, so anything read after it lands at an
offset; reading provenance first keeps `ProvId` the identity in the case that
matters. The file-name table has to be interned before both, because a
record's `Loc` names one.

**No compression.** The per-section `flags` word that would select it is in the
format and no compressor is behind it: a saturated `zebra2` is 56 KB against a
64 KB budget, and a compressed section forfeits the `mmap` the layout exists
for. A reader refuses a non-zero `flags` rather than guessing.

**`SOLUTIONS` has no CLI producer.** The section, its writer, its reader and
`reconstitute` are all there and tested; `ein kb save` will not write one and
nothing in `ein solve` reads one. That is
[F9](../../followups/f9_e_catalog.md)'s hazard handled structurally rather than
advisorily — a stored answer that no benchmark can reach cannot memoise a
puzzle — and `roundtrip.rs`'s last test is the guard that keeps it true.

### Two things a `.einb` does not carry, and why

`EqClasses` is the M1 placeholder for F4's e-graph and the engine never unions
— the only caller in the tree is one test — so a container of a KB with
equality classes would lose them, and there is no way to make one. And the
**rebuilt registries' `Loc`s** point into the stored canonical text rather than
the original file; provenance `Loc`s are stored and come back exact, which is
the half anything reads.

## Acceptance, checked

| criterion | |
|---|---|
| round-trip T1-identical for every corpus entry, empty **and** shared interner | `ein-einb/tests/roundtrip.rs` — **95 files, 91 of them through the remap**. Empty: `Kb::diff` compares fact order, belief, the negated set, all seven indexes, the primary map, the alternative lists and the no-goods. Shared: `ein_core::shape`, which names facts by position, because the ids necessarily move |
| cold open of a saturated `zebra2` under 1 ms | **0.614 ms** release, 0.833 ms in the dev profile `cargo test` builds. The test gates the design's millisecond on an optimised build and 5 ms otherwise, and prints the number either way — a shared runner at `opt-level = 1` would otherwise be measuring the runner |
| the file under 64 KB uncompressed | **57 688 bytes**: `Prov=31432 Program=10944 Facts=9176 Symbols=4120 Present=1520 Meta=139`. `section_sizes` is public so the number stays inspectable |
| truncated or bit-flipped → rejected by the digest, never mis-parsed | `corruption.rs`. Every 97-byte prefix refused; **3 348 single-bit flips**, all caught — which is where the header gap showed up: the digest covers everything *after* the header, so the reserved words are now required to be zero. `format.minor` is exempt on purpose, because a later minor is a file this reader must accept |
| fuzzed inputs rejected, never mis-parsed | 20 000 deterministic inputs — noise, noise wearing the magic, and real files with bytes smashed — through a reader with the digest **off**, which is `--trust-cache`'s contract. No panic, no hang; `Reader::count` is what stops a forged length from allocating 4 GB |
| an unknown section kind in the same major loads with that section ignored | `an_unknown_section_kind_is_ignored_rather_than_refused` forges the file a later minor would write — one *extra* section, every offset moved, the digest re-stamped |
| a `.einb` from another engine semver keeps `PROGRAM` and drops the derived sections | `invalidation.rs` — the KB comes back with the loaded fact count and its rules intact |
| a stdlib byte change is a cache miss, not a stale hit | both ways: `Meta::freshness` against an edited manifest's digest in `invalidation.rs`, and end-to-end in `ein-cli/tests/einb_cli.rs`, which copies `stdlib/` to a temp directory, adds one byte to `MANIFEST.sha256`, and points `$EIN_STDLIB` at it — stderr says `StaleStdlib`, the derived state is dropped, and the exit code is 0 because a cache miss is not an error |
| `ein solve zebra2.einb` byte-identical to `ein solve zebra2.ein` | `einb_cli.rs`, over four puzzles and five diagnostic flags (`--stats`, `--trace`, `--explain`, `--final-state`, `--dump-config`). **Two lines are normalised and no others**: the path `solve` echoes, which is a different file by construction, and `--stats`'s wall clock, which is not a property of the KB |

**The gate:** `cargo test --workspace` — **575 passed**, 0 failed.
`cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`
clean.
