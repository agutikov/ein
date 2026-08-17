# S1a.8.8 — CLI over server

**Phase:** P1a.8 (Server mode)
**Estimate:** 1 day
**Depends on:** [S1a.8.7](s1a.8.7_caches.md)
**Implements:** [design/09](../design/09_server_mode.md) §8

## Context

`ein <cmd> --server` routes an ordinary CLI invocation through a running
daemon: same arguments, same output, faster start. It closes the loop on
the phase — if the server can reproduce the CLI byte-for-byte, the server
demonstrably *shares* the engine rather than approximating it.

Autostart is deliberately out of v1. An implicit background daemon is a
support burden and a parity hazard (a stale daemon serving an old
stdlib), so the daemon is always explicit.

## Acceptance

- For every corpus entry × run-matrix cell,
  `ein <cmd> --server` output is byte-identical to `ein <cmd>` — with
  `--no-cache`, so the comparison measures the engine and not the cache.
- Startup for a warm daemon: `ein solve --server` first output under
  10 ms for `zebra2.ein` (versus parse+load every time).
- With no daemon reachable, `--server` fails with a clear message and a
  distinct exit code — it does **not** silently fall back, because a
  silent fallback makes a timing measurement meaningless.
- File-relative imports resolve against the *client's* working directory,
  not the daemon's.

## Tasks

### Task T1a.8.8.1 — Client transport

Connect over `$EIN_SERVER` (a `unix:` / `tcp:` address) or `--server
<addr>`. One `session.open` per invocation with the client's cwd as
`base_dir`, and a `session.close` on exit (including on error paths).

### Task T1a.8.8.2 — Command routing

Map each subcommand onto protocol calls: `solve` →
`kb.load` + `solve.start` + subscribe + `solve.result`;
`saturate` → `kb.load` + `kb.saturate` + the firing stream;
`render` → `kb.load` + `render.dot`.

The **rendering stays client-side** where it can: the server returns
verdicts, stats and handles; the CLI formats them with the same code it
uses locally. That is what makes byte-identity structural rather than a
second implementation of the printers.

### Task T1a.8.8.3 — Streaming to `-v` / `--dump-states`

Subscribe to the event classes the local dumpers consume and drive the
same `ProgressDumper` / `MonotonicDumper` / `LatticeDumper` from the
stream. If the events do not carry enough for a dumper to reproduce its
local output, that is a gap in the event schema and it gets fixed there
— not by adding a bespoke server response.

### Task T1a.8.8.4 — Paths and file access

Sources are sent by **path** when the daemon can read them and by
**content** otherwise (`--send-source`), with `base_dir` always the
client's. Under `--sandbox` the daemon accepts content only. Document
which mode is in force in the error messages, since "file not found" from
a daemon on another cwd is otherwise baffling.

### Task T1a.8.8.5 — Exit codes and errors

Preserve the local mapping (0 / 1 / 2), and map transport failures to a
distinct fourth code so a script can tell "the puzzle failed" from "the
daemon is not there".

### Task T1a.8.8.6 — The conformance pass

Add `--server` as a run-matrix dimension for a representative subset of
the corpus, run nightly. This is the acceptance criterion above, made
permanent.

## Notes

- Keep `--server` off the default path forever. The CLI's local mode is
  the reference implementation, and every golden, benchmark and
  acceptance number is measured through it.
- If a future `ein serve --autostart` is wanted, it needs a story for
  staleness (the daemon must refuse to serve a session whose stdlib
  manifest hash differs from the client's). Record that requirement now
  so it is not rediscovered later.
