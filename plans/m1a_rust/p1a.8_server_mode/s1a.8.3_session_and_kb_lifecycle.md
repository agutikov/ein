# S1a.8.3 — Sessions and KB lifecycle

**Phase:** P1a.8 (Server mode)
**Estimate:** 2 days
**Depends on:** [S1a.8.2](s1a.8.2_protocol.md),
[S1a.8.1](s1a.8.1_einb_container.md)
**Implements:** [design/09](../design/09_server_mode.md) §§3, 7

## Context

The daemon's spine: process startup, transports, the session registry,
the handle table, and the KB-building methods. Everything here is
plumbing over an engine that already works — which is why it is two days
and why the interesting content is the *isolation* rules, not the
machinery.

## Acceptance

- `ein serve --listen stdio|unix:PATH|tcp:ADDR` starts, serves, and shuts
  down cleanly (in-flight requests finish or are cancelled, sockets
  removed).
- Handles are per-session; closing a session releases them and their
  memory (verified by an RSS check).
- Multiple sessions with different `base_dir`s resolve file-relative
  imports independently.
- `--sandbox` refuses file-relative imports with a clear error while
  `std.*` still resolves.
- Unix sockets are created 0600; TCP binds loopback unless
  `--allow-remote`.
- A fuzzed / malformed request stream never panics the server.

## Tasks

### Task T1a.8.3.1 — Process and transports

Startup, the three transports behind one framed-message trait, graceful
shutdown on SIGINT/SIGTERM, and a bounded request pool. Keep the async
runtime (if any) confined to `ein-server`; the engine stays synchronous
and `Send` ([design/12](../design/12_toolchain_and_layout.md) §2).

### Task T1a.8.3.2 — Session registry

`session.open` / `session.close`, each session owning: a handle table, a
`base_dir`, a resource budget (`max_jobs`, `max_memory`,
per-request `max_time` / `max_enterings`), and a sandbox flag. Sessions
are independent; nothing but the content-addressed caches is shared.

### Task T1a.8.3.3 — Handle table

Typed, reference-counted, with the lifetime rules from
[S1a.8.2](s1a.8.2_protocol.md) T1a.8.2.2. Unknown or stale handle → the
protocol's stable error code, never a panic.

### Task T1a.8.3.4 — Loading

`kb.load {source | path, base_dir}` → parse + resolve imports + load,
returning counts, warnings and the content digest.
`kb.open {path.einb}` → mmap + remap. `kb.save`. Errors carry the
engine's verbatim message text — an embedder should see exactly what the
CLI would print.

### Task T1a.8.3.5 — Building

`kb.assert {kb, forms}` (facts or whole forms, returning a new handle by
default), `kb.fork` (O(1)), `kb.saturate {kb, max_steps}` (returning a
new handle plus the firing count, streaming `firing` events when
subscribed).

`kb.assert` needs a decision recorded: asserting a `(rule …)` form
invalidates the plan memo's assumptions for that KB, so v1 **rejects**
non-fact forms with a clear message pointing at `kb.load`
([design/09](../design/09_server_mode.md) §9's "no live rule editing").

### Task T1a.8.3.6 — The filesystem seam

All file access goes through the `StdlibSource` / `FileSource` trait from
[S1a.1.3](../p1a.1_ir_frontend/s1a.1.3_macros_and_imports.md), so
`--sandbox` is a different implementation rather than a scattered set of
checks. Path traversal outside `base_dir` is refused even without
`--sandbox`.

## Notes

- Handle ids should be random even for a local socket — a predictable
  handle space is an unnecessary sharp edge if the socket is ever
  proxied.
- Resist per-request process isolation. The whole point is a shared
  resident `Arc<KbCore>`; isolation lives at the session level, and the
  engine's purity (no globals beyond the append-only interner/memo) is
  what makes that safe.
