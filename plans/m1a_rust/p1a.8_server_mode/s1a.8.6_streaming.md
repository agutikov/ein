# S1a.8.6 — Streaming

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [S1a.8.5](s1a.8.5_solve_jobs.md)
**Implements:** [design/09](../design/09_server_mode.md) §5

## Context

The server does **not** invent a progress format. It emits the same
events the [oracle protocol](../design/01_parity_contract.md) §3
defines, wrapped as JSON-RPC notifications. One schema then serves three
consumers — the parity harness, the CLI's `--verbose` / `--dump-states`,
and the GUI — which is the reason it was worth defining carefully in
[P1a.0](../p1a.0_conformance_harness/README.md) rather than three times.

The engineering content here is not the schema, it is **flow control**:
an exhaustive zebra2 solve produces ~40 k firings plus ~194 k redundant
ones, and a client that subscribes to everything over a socket will
either drown or become the bottleneck.

## Acceptance

- A client subscribed to `enter` + `verdict` sees exactly the events the
  `--events` file contains for the same run, in the same order.
- A slow client does not slow the solve: back-pressure policy is
  explicit, applied, and reported (dropped counts per class).
- Subscribing after a job started delivers a snapshot of the current
  state plus subsequent events — no silent gap.
- Unsubscribing stops delivery within one event.
- Event emission adds < 2 % to a solve with one subscriber at the
  `normal` level.

## Tasks

### Task T1a.8.6.1 — Subscription

`solve.subscribe {solve, classes, level}` / `solve.unsubscribe`, where
`classes` is the event-kind list and `level` is
`{normal, verbose}` (verbose includes redundant `fire` events —
[S1a.0.2](../p1a.0_conformance_harness/s1a.0.2_oracle_event_protocol.md)).
Also `kb.subscribe` for `kb.saturate`'s firing stream.

### Task T1a.8.6.2 — The emitter seam

The engine's event emission is already an `Option<Writer>`-shaped hook.
The server plugs a fan-out sink into it: no subscribers → the hook is
`None` and the cost is zero, matching the CLI's behaviour exactly.

Per-subscriber class filtering happens at the sink, not in the engine —
the engine must not know how many clients exist, or its behaviour
becomes client-dependent.

### Task T1a.8.6.3 — Back-pressure

Bounded per-subscriber queue with a documented policy: **drop oldest of
the lowest-priority class, count the drops, and report them in a
`dropped` notification**. Never block the engine; never grow without
bound. `verdict` and `enter` events are never dropped; `fire` events are
the first to go.

The alternative — blocking the solve until the client catches up — is
rejected explicitly: it would make wall-clock depend on the observer,
which breaks every measurement the project relies on.

### Task T1a.8.6.4 — Late subscribers

On subscribe, send a synthetic state snapshot (current layer, enterings
so far, counters) followed by live events, so a client that attaches
mid-run has a coherent picture rather than a suffix.

### Task T1a.8.6.5 — Batching

Coalesce events into framed batches (by count or by a small time window)
to amortise the per-message overhead on the wire. Batching must not
reorder or drop within a class, and the batch boundary must not be
observable in the reconstructed sequence.

### Task T1a.8.6.6 — Parity check

A test that runs a corpus entry twice — once with `--events FILE`, once
through a subscribed server client — and asserts the two event sequences
are identical at `verbose` with no subscriber-side drops. This is what
keeps the "one protocol, three consumers" claim true.

## Notes

- Progress under `-v` in the CLI is a *different rendering* of the same
  events (`ProgressDumper`), not a different source. If the two ever
  disagree, the dumper is reading state the events do not carry, and the
  fix is to carry it.
- Do not add per-event timestamps to the schema used for parity diffing;
  put them in a separate optional field the differ normalises away
  ([design/01](../design/01_parity_contract.md) §5).
