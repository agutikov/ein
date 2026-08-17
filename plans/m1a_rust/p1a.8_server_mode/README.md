# P1a.8 — Server mode

**Milestone:** [M1a — Rust port](../README.md)
**Estimate:** 3 weeks (16 days of stages)
**Depends on:** [P1a.7](../p1a.7_parallelism/README.md)
**Note:** [S1a.8.2](s1a.8.2_protocol.md) settles Q-M1a.11 (wire
protocol) first, with [M1b](../../m1b_gui/README.md)'s stack in view;
every later stage assumes that decision.

## Goal

Make the engine resident: load a KB once, ask it many questions, keep
several KBs alive, store and reload them in a binary format, cache
solutions, and stream progress. This is the phase that turns ein.rs from
a faster CLI into infrastructure — the backend M1b needs and the boundary
M2 can talk to out of process.

Design: [design/09](../design/09_server_mode.md) (service) and
[design/10](../design/10_binary_format.md) (`.einb`).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.8.1](s1a.8.1_einb_container.md) | The `.einb` container | 3 d |
| [S1a.8.2](s1a.8.2_protocol.md) | Protocol and IDL | 2 d |
| [S1a.8.3](s1a.8.3_session_and_kb_lifecycle.md) | Sessions and KB lifecycle | 2 d |
| [S1a.8.4](s1a.8.4_query_and_inspect.md) | Querying and inspecting a resident KB | 2 d |
| [S1a.8.5](s1a.8.5_solve_jobs.md) | Solve jobs | 2 d |
| [S1a.8.6](s1a.8.6_streaming.md) | Streaming | 2 d |
| [S1a.8.7](s1a.8.7_caches.md) | Caches | 2 d |
| [S1a.8.8](s1a.8.8_cli_over_server.md) | CLI over server | 1 d |

## Acceptance for the phase

- `ein serve` + a client reproduces the CLI's output for the whole corpus
  at **T3, with `--no-cache`** — proof the server shares the engine
  rather than approximating it.
- Load-once/ask-many: 100 `kb.query` calls against one saturated zebra2
  KB cost < 1 % of 100 `ein solve` invocations.
- `.einb` round-trip is T1-identical for every corpus entry, in both the
  empty-interner and shared-interner cases; cold open of a saturated
  zebra2 under 1 ms.
- A stdlib byte change invalidates every downstream cache entry (tested
  by editing a temp stdlib copy).
- A cancelled solve releases threads and memory within 100 ms.
- Fuzzed `.einb` inputs are rejected by digest, never mis-parsed.

## Risks

- **The solution cache is a measurement hazard**, and
  [F9](../../followups/f9_e_catalog.md) says so explicitly about its
  ancestor: it memoises the puzzle rather than improving the reasoner.
  Mitigation is structural — `--no-cache` in every benchmark and CI run,
  a `cached` flag on every response, and hit rates in `server.stats`.
- **Scope creep toward a database.** The non-goals in
  [design/09](../design/09_server_mode.md) §9 (no retraction, no query
  language beyond `:goal`, no live rule editing, no auth) are there to
  keep this a three-week phase.

## Cross-links

- [design/09 — Server mode](../design/09_server_mode.md)
- [design/10 — Binary format](../design/10_binary_format.md)
- [M1b GUI](../../m1b_gui/README.md) — the first real client
- Q-M1a.11 (protocol), Q-M1a.12 (remote access)
