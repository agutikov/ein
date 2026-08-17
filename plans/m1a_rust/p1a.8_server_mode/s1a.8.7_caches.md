# S1a.8.7 — Caches

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [S1a.8.1](s1a.8.1_einb_container.md),
[S1a.8.3](s1a.8.3_session_and_kb_lifecycle.md)
**Implements:** [design/09](../design/09_server_mode.md) §6

## Context

Four content-addressed layers — parse, program, saturation, solutions —
each keyed by a BLAKE3 digest of its input, each an obvious win for a
resident process, and the last of which is a **measurement hazard** the
repo has already ruled on once.

[F9](../../followups/f9_e_catalog.md) rejected a cross-call conflict
cache (ex-E20 ≈ incremental SAT) *on purpose*: it "memoises the puzzle
rather than improving the reasoner", and its measured +57 % is available
only when re-solving a byte-identical file — "which in-repo means the
very benchmark and acceptance loops a warm cache would falsify."

A server's solution cache is that mechanism. It is legitimate as a
product feature and poison as a measurement, so the honesty machinery
below is not optional garnish — it is the reason the cache is allowed to
exist at all.

## Acceptance

- Every response carries `{"cached": bool, "cache_layer": …}`.
- `--no-cache` disables all four layers, and the conformance harness and
  every benchmark run with it — asserted in CI, not just documented.
- A stdlib byte change invalidates every downstream entry (tested by
  editing a temporary stdlib copy and re-loading).
- `server.stats` reports per-layer hit rate, entry count and resident
  bytes.
- Eviction respects the configured ceiling under a synthetic load of many
  distinct KBs, with no unbounded growth.
- A cache hit is byte-identical to a cold computation for every corpus
  entry (the strongest possible check, and cheap to run).

## Tasks

### Task T1a.8.7.1 — Keys

- parse: `hash(source bytes)`
- program: `hash(source, resolved import set, stdlib manifest hash)`
- saturation: `hash(program, kb fact set)`
- solutions: `hash(saturated kb, config, stop_after, max_set_size)`

Including the stdlib manifest hash in the program key is what makes
invalidation automatic ([design/11](../design/11_shared_assets.md) §3);
including the *resolved import set* is what makes a file-relative import
change invalidate too.

### Task T1a.8.7.2 — Storage and eviction

In-memory LRU by resident bytes with a configurable ceiling, plus
`.einb` spill to `--workspace` so a cold entry is cheap to revive
([S1a.8.1](s1a.8.1_einb_container.md)). Entries are immutable, so
eviction never needs to write back — it just drops or spills.

### Task T1a.8.7.3 — The honesty surface

`cached` / `cache_layer` on every response; `--no-cache` at the process
level and `no_cache: true` per request; `server.stats` hit rates;
`server.gc` to force eviction. A benchmark that forgets `--no-cache`
should be *obvious* from its output, not silently 50× faster.

### Task T1a.8.7.4 — Concurrency

Two clients loading the same source concurrently must not both do the
work: single-flight per key (one computes, the rest await). Must not
deadlock when a computation itself asks for a lower cache layer.

### Task T1a.8.7.5 — Correctness harness

Run the whole corpus twice through a warm server and diff cold-vs-warm
responses at T3. Any difference is a cache-key bug, and a cache-key bug
is a wrong-answer bug.

### Task T1a.8.7.6 — Documentation

A short section in the server docs stating plainly: the solution cache
memoises *this exact question about this exact KB*; it does not make the
reasoner better; and it must be off when measuring. Link
[F9](../../followups/f9_e_catalog.md) so the reasoning is one click away.

## Notes

- Resist caching *partial* search state across calls (a warm no-good
  store, a retained alive set). That is precisely the rejected E20 shape,
  and unlike the four layers here it is not a pure function of its key —
  it would make a solve's result depend on what was solved before it.
- The parse and program layers are uncontroversial and carry most of the
  practical win (0.63 s per invocation under CPython). If the solution
  layer ever looks like trouble, it can be dropped without losing that.
