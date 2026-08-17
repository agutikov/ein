# S1a.8.2 — Protocol and IDL

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [P1a.7](../p1a.7_parallelism/README.md)
**Implements:** [design/09](../design/09_server_mode.md) §§2, 4
**Decides:** Q-M1a.11

## Context

The wire contract, defined before any of it is implemented, so the
handle model and the error taxonomy are decisions rather than accidents.

Recommendation on record: **JSON-RPC 2.0** over stdio / unix / tcp — no
codegen dependency, trivially debuggable, and LSP-shaped, which is a
well-understood pattern for exactly this (a long-lived analysis process
a tool talks to). Decide against gRPC (build-time codegen, heavy client
story) and a bespoke binary protocol (premature — the payloads that
matter are handles and small result sets, and `.einb` covers the bulk
case) at this stage rather than drifting into one.

## Acceptance

- `conformance/PROTOCOL.md` documents every method, its params, its
  result, and its errors, with a version number.
- A golden request/response transcript per method, versioned with the
  protocol number and diffed in CI.
- `server.hello` negotiation works: a client pinning major 1 gets a
  clear error from a major-2 server, and an unknown *method* is a
  well-formed JSON-RPC error rather than a disconnect.
- The error taxonomy maps every engine failure mode
  (`IRParseError`, `KBLoadError`, `CompileError`, budget abort, unknown
  handle, sandbox violation) to a stable code, with the engine's message
  text carried verbatim in `data`.

## Tasks

### Task T1a.8.2.1 — Decide and record Q-M1a.11

Write the decision with its alternatives and reasons into
[`open_questions.md`](../open_questions.md), informed by whatever
[M1b](../../m1b_gui/README.md) is leaning toward for its stack. If M1b is
undecided, JSON-RPC is the choice that keeps every option open (a gRPC or
HTTP facade over the same method table is a later adapter, not a
rewrite).

### Task T1a.8.2.2 — Handle model

Five kinds — `session`, `kb`, `solve`, `model`, `trace` — as opaque
strings, reference-counted, explicitly closable, scoped to a session.
Define the lifetime rules: what a closed session does to live handles,
what happens to a `model` whose `solve` was closed, and whether handles
are guessable (they should not be; use random ids even locally).

### Task T1a.8.2.3 — Method table

Loading (`kb.load` / `kb.open` / `kb.save` / `kb.close`), building
(`kb.assert` / `kb.fork` / `kb.saturate`), asking (`kb.query` /
`kb.facts` / `kb.contradictions` / `solve.*`), explaining
(`explain.fact` / `explain.core` / `trace.build` / `trace.markdown` /
`render.dot`), admin (`server.hello` / `server.stats` / `server.gc` /
`session.open` / `session.close`).

Document explicitly that **`kb.retract` does not exist** — the engine is
append-only and retraction is `kb.fork` from an earlier handle. Saying
so in the protocol doc stops it being asked for repeatedly.

### Task T1a.8.2.4 — Immutability semantics

`kb.assert` and `kb.saturate` return a **new** handle by default, which
is what makes content-addressed caching sound. `in_place: true` is
available for a GUI's scratch KB and yields a handle whose core is not
shared. Spell out which methods invalidate which handles.

### Task T1a.8.2.5 — Notifications

The `event` notification carrying the
[oracle event protocol](../design/01_parity_contract.md) §3 payload, plus
subscription methods (`solve.subscribe {solve, classes}`). One schema for
the harness, the CLI's dumps, and the GUI.

### Task T1a.8.2.6 — Transcripts

For each method, a checked-in request/response pair generated from a
real run, diffed in CI. These double as the protocol's documentation and
as the regression net for accidental shape changes.

## Notes

- Version the protocol independently of the engine. A protocol bump is a
  client-visible event; an engine release should not force one.
- Keep result payloads small and handle-shaped. Anything large (a full
  fact list, a model) is paged (`kb.facts`) or written to `.einb`; a
  protocol that returns megabytes of JSON becomes the bottleneck the
  server was built to remove.
