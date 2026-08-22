# P1a.8 — Binary KB container

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 0.5 weeks (3 days of stages)
**Depends on:** [P1a.7](../p1a.7_parallelism/README.md)
**Scope change 2026-08-18:** this phase used to be **Server mode** —
daemon, sessions, JSON-RPC, streaming, solution cache, `ein <cmd>
--server` (8 stages, 3 weeks). The server is **dropped**: a Rust library
plus a Rust CLI covers every consumer M1a has ([M1b](../../m1b_gui/README.md)
links the crates directly — see its § Stack — and [M2](../../m2_nl_to_ir/README.md)
crosses the boundary through PyO3 in [P1a.9](../p1a.9_release/README.md)).
What survives is the one deliverable that was never about the daemon:
the `.einb` container. The seven server stages and `design/09` are in
git history.

## Goal

Store a loaded — and optionally saturated — KB on disk in a form the
engine can `mmap` back with no parse and no load: the interner, the int
pool, the fact rows and args, the presence bitset, provenance, the
resolved program, and optionally the indexes, no-goods and solutions.

`.einb` is a **private, versioned cache format**, not an interchange
format. `.ein` text stays the only authoring format and the only thing
anyone edits; anything crossing a tool boundary is `.ein` or the event
protocol's JSON.

Design: [design/10](../design/10_binary_format.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.8.1](s1a.8.1_einb_container.md) ✅ | The `.einb` container | 3 d |

**Shipped 2026-08-21** — one stage, one crate (`ein-einb`, the eighth
workspace member and the only one that is not `#![forbid(unsafe_code)]`), and
`ein kb save`. Every acceptance item below is met; the numbers and the five
decisions the design left open are in
[the stage record](s1a.8.1_einb_container.md#what-shipped--2026-08-21).

## Acceptance for the phase — all met

- ✅ `.einb` round-trip is T1-identical for every corpus entry, in both the
  empty-interner and shared-interner cases; cold open of a saturated
  zebra2 under 1 ms. **95 files, 91 through the remap; 0.614 ms; 57 688
  bytes.**
- ✅ `ein solve zebra2.einb` is byte-identical to `ein solve zebra2.ein` at
  T3 — the strongest evidence the round-trip is faithful. **Four puzzles,
  five diagnostic flags, two normalised lines: the echoed path and the
  wall clock.**
- ✅ A stdlib byte change is a **cache miss**, not a stale hit (tested by
  editing a temp stdlib copy). **Both ways — the digest directly, and a
  child process with `$EIN_STDLIB` pointed at an edited copy.**
- ✅ Fuzzed `.einb` inputs are rejected by digest, never mis-parsed.
  **20 000 inputs with the digest off, 3 348 single-bit flips with it on;
  the flips found a real gap — the header is not under the digest, so its
  reserved words are now required to be zero.**

## Risks

- **Storing a solution is a measurement hazard**, and
  [F9](../../followups/f9_e_catalog.md) says so explicitly about its
  ancestor: a stored answer memoises the puzzle rather than improving the
  reasoner. Mitigation is structural, and it came out **stronger** than
  planned: rather than a run that opened a `SOLUTIONS` section saying so in
  its stats, there is no way for a run to open one. `ein kb save` will not
  write the section and nothing in `ein solve` reads it — the store is a
  library API with a test standing over the CLI to keep it that way.
- **Scope creep back toward a server.** A cache format that grows a
  lookup protocol is the daemon again under another name. `.einb` is a
  file the CLI reads and writes; there is no resident process.

## Cross-links

- [design/10 — Binary format](../design/10_binary_format.md)
- [design/03 — Data model](../design/03_data_model.md) — `.einb` is that
  layout, serialised
- [design/11 — Shared assets](../design/11_shared_assets.md) §
  invalidation — the stdlib manifest hash lands in `META`
