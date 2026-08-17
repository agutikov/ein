# 09 — Server mode

**Settles:** the resident-engine design — sessions, multiple KBs,
multiple queries against one KB, stored solutions, streaming progress.
**Phase:** [P1a.8](../p1a.8_server_mode/README.md).
**New surface** — nothing in ein.py corresponds, so this is the one
design doc with no parity obligation. The *engine* it drives is still the
byte-parity engine; the server only changes who calls it and how often.

---

## 1. Why a server

Four workloads that the one-shot CLI serves badly, each already visible
in the repo:

1. **Load is not free and it repeats.** `zebra2.ein` costs 0.20 s to
   parse and 0.43 s to load under CPython, and every `ein solve`
   invocation pays both. The conformance corpus runs thousands of
   invocations over the same handful of files; so does
   `utils/feature_matrix.py`.
2. **Multiple questions, one model.** `goal_bindings(kb, goal)` can
   project *any* pattern over a solved model — the docstring says so
   explicitly ("pass an explicit goal pattern to project a different
   question"). Today answering a second question means re-solving.
3. **[M1b](../../m1b_gui/README.md) needs a live engine.** A GUI wants
   incremental facts, per-step traces, cancellable solves and progress —
   i.e. long-lived state plus streaming, which a process-per-command CLI
   cannot give.
4. **[M2](../../m2_nl_to_ir/README.md)'s NL frontend stays CPython.**
   It needs a boundary. PyO3 ([P1a.9](../p1a.9_bindings_release/README.md))
   is the in-process option; a socket is the out-of-process one, and the
   two are complementary — PyO3 for tight loops, the server for a
   pipeline where the LLM process and the reasoner have different
   lifetimes.

---

## 2. Shape

```
ein serve [--listen stdio|unix:PATH|tcp:ADDR] [--workspace DIR]
          [--jobs N] [--max-memory MB] [--no-cache]
```

- **One process, many sessions.** No fork-per-request; the engine's
  shared state (`Arc<Program>`, `Arc<KbCore>`, interner, plan memo) is
  exactly what [08](08_parallelism.md) §6 already made `Sync`.
- **Transports.** `stdio` (line-delimited JSON-RPC 2.0) is the default
  and the one embedders want; `unix:` for local multi-client; `tcp:` for
  containers, bound to loopback unless explicitly overridden. An
  HTTP/1.1 + SSE facade is a thin adapter over the same method table,
  added when M1b picks its stack — not before.
- **Protocol.** JSON-RPC 2.0: requests, responses, and **server→client
  notifications** for streaming. Chosen over gRPC (no codegen
  dependency, trivially debuggable, already how LSP-shaped tools work)
  and over a bespoke binary protocol (premature; the payloads that
  matter are handles and small result sets, and `.einb`
  ([10](10_binary_format.md)) covers the bulk case).
- **Versioning.** `server.hello` returns `{protocol: "1.0", engine:
  "…", capabilities: [...]}`; clients pin a major. Every method is
  additive within a major.

---

## 3. Object model

Five handle kinds, all opaque strings, all reference-counted and
explicitly closable:

| handle | is | created by |
|---|---|---|
| `session` | a namespace + resource budget | `session.open` |
| `kb` | a loaded (optionally saturated) knowledge base | `kb.load`, `kb.fork`, `kb.saturate`, `kb.open` |
| `solve` | a running or finished search | `solve.start` |
| `model` | one solution node's KB | a `solve` result |
| `trace` | a linearised derivation | `trace.build` |

A `kb` handle is immutable-by-default: `kb.assert` and `kb.saturate`
return a **new** handle rather than mutating, which mirrors the engine's
append-only design and makes caching by content hash sound. Mutation in
place is available as `kb.assert(…, in_place: true)` for a GUI's
scratch KB, and is simply a handle whose core is not shared.

### Sessions

A session owns handles, a job budget (`max_jobs`, `max_memory`,
`max_time` per request), and a working directory for file-relative
imports. Closing a session drops its handles; the content-addressed
caches survive (they are keyed by content, not by session).

---

## 4. Methods

Grouped; the full IDL lands in
[S1a.8.2](../p1a.8_server_mode/README.md).

**Loading**

| method | notes |
|---|---|
| `kb.load {source \| path, base_dir}` | parse + resolve imports + load. Returns `{kb, digest, counts, warnings}`. Cache hit reported. |
| `kb.open {path.einb}` | mmap a stored KB ([10](10_binary_format.md)) — no parse, no load |
| `kb.save {kb, path}` | write `.einb` |
| `kb.close {kb}` | drop |

**Building**

| method | notes |
|---|---|
| `kb.assert {kb, forms}` | add facts (or whole forms) → new handle |
| `kb.retract` | **not offered.** The engine is append-only; retraction is `kb.fork` from an earlier handle. Stated in the docs so nobody looks for it. |
| `kb.fork {kb}` | O(1) branch |
| `kb.saturate {kb, max_steps}` | run the closure to a fixpoint → new handle + firing count; streams `firing` events when subscribed |

**Asking**

| method | notes |
|---|---|
| `kb.query {kb, goal}` | run a `:goal`-shaped pattern; returns binding rows. **This is the multi-query path** — no solve, no fork, just the matcher over a saturated KB |
| `kb.facts {kb, filter}` | paged fact listing (by relation, by provenance kind, by name participation) |
| `kb.contradictions {kb}` | the detector's records |
| `solve.start {kb, config, stop_after, max_set_size, budgets}` | returns a `solve` handle immediately; progress streams |
| `solve.await {solve}` / `solve.cancel {solve}` | |
| `solve.result {solve}` | verdict, `k`, stats, `model` handles, unsat core |

**Explaining / rendering**

| method | notes |
|---|---|
| `explain.fact {kb, fact, budget}` | minimal explanation |
| `explain.core {kb, facts}` | unsat core / smallest frontier |
| `trace.build {solve \| kb, options}` → `trace.markdown {trace}` | the same renderer `--trace` uses |
| `render.dot {kb \| rules \| lattice \| slice, options}` | the same DOT the CLI emits |

**Admin**

`server.hello`, `server.stats` (cache hit rates, resident KBs, memory),
`server.gc`, `session.open/close`.

---

## 5. Streaming: the event protocol, reused

The server does **not** invent a progress format. It emits the same
JSONL events the oracle protocol defines ([01](01_parity_contract.md)
§3), wrapped as JSON-RPC notifications:

```json
{"jsonrpc":"2.0","method":"event","params":{"solve":"s1","e":"enter","n":42,…}}
```

A client subscribes per handle and per event class (`solve.subscribe
{solve, classes:["enter","verdict"]}`). One protocol serves three
consumers — the parity harness, the CLI's `--verbose`/`--dump-states`,
and the GUI — which is why it is worth defining once, carefully, in
P1a.0 rather than three times.

---

## 6. Caches — content-addressed, and honest about it

Four layers, each keyed by a BLAKE3 digest of its input:

| cache | key | value |
|---|---|---|
| parse | `hash(source bytes)` | AST arena |
| program | `hash(source, resolved import set, stdlib manifest)` | loaded `Program` + KB |
| saturation | `hash(program, kb facts)` | saturated `KbCore` |
| **solutions** | `hash(saturated kb, config, stop_after, max_set_size)` | verdict + models + stats |

The stdlib manifest hash is part of the program key on purpose — a
stdlib edit must invalidate everything downstream
([11](11_shared_assets.md)).

**The honesty requirement.** [F9](../../followups/f9_e_catalog.md)
rejected a cross-call conflict cache with a specific argument: it
"memoises the puzzle rather than improving the reasoner", and its
measured win is available only when re-solving a byte-identical file —
"which in-repo means the very benchmark and acceptance loops a warm cache
would falsify." A server's solution cache is exactly that mechanism,
which is fine *as a product feature* and poison *as a measurement*. So:

- every response carries `{"cached": true|false, "cache_layer": …}`;
- `--no-cache` disables all four layers;
- the conformance harness and every benchmark run with `--no-cache`, and
  CI asserts it;
- `server.stats` reports hit rates so a user can see when they are
  measuring the cache.

Eviction is LRU by resident bytes with a configurable ceiling; `.einb`
spill to `--workspace` keeps a cold entry cheap to revive.

---

## 7. Concurrency, budgets, isolation

- Requests are handled on a bounded pool; a `solve` runs on the parallel
  search ([08](08_parallelism.md)) with the session's `max_jobs`.
- Every long call takes `max_time` / `max_enterings` and maps a budget
  cut to the engine's existing `Aborted` verdict rather than an error —
  the same `on_budget="verdict"` path the CLI can already request.
- Cancellation is cooperative: the engine checks a cancel flag at the
  same points it checks budgets (`_check_budget`), so no new interruption
  machinery is needed.
- **Isolation.** The server never evaluates user code; the only file
  access is import resolution, which is confined to the session's
  `base_dir` and the stdlib root. A `--sandbox` flag refuses
  file-relative imports entirely (source-only sessions), which is what a
  hosted M2 pipeline wants.
- Local by default: `unix:` sockets get 0600 permissions; `tcp:` binds
  loopback and requires `--allow-remote` to do otherwise. No
  authentication in v1 — if remote access is ever wanted, it goes behind
  a reverse proxy, and that decision is Q-M1a.12.

---

## 8. What the CLI becomes

`ein <cmd>` keeps working exactly as today (that is [01](01_parity_contract.md)
T3). Optionally, it grows a `--server` flag that routes the command
through a running daemon — same output, faster start. The CLI is then a
thin client, and the two code paths share everything below the argument
parser. Autostart (`ein solve --server=auto`) is deliberately **not** in
v1: an implicit background daemon is a support burden and a parity
hazard (a stale daemon serving an old stdlib). Explicit `ein serve`
only.

---

## 9. Not in scope

- **Distributed / multi-machine search.** The placeholder README
  mentioned a dropped distributed sketch; the contract that would enable
  it is the set-batch primitive `try_commitment_set` already provides, so
  the door stays open. Not v1.
- **Authentication / multi-tenancy.**
- **A query language beyond `:goal` patterns.** `kb.query` takes the
  same pattern the IR already has. A richer surface belongs in the
  language, not the server.
- **Live rule editing.** `kb.assert` covers facts; changing rules means
  a new `kb.load`, because plans are compiled per `(rule, activator)` and
  the memo is keyed on it.

---

## 10. Acceptance for this design

- `ein serve` + a client script reproduces the CLI's output for the whole
  corpus (T3, with `--no-cache`), proving the server path shares the
  engine rather than approximating it.
- Load-once/ask-many: 100 `kb.query` calls against one saturated
  `zebra2` KB cost < 1 % of 100 `ein solve` invocations.
- A cancelled `solve` releases its threads and memory within 100 ms.
- Cache correctness: a stdlib byte change invalidates every downstream
  entry (asserted by a test that edits a temp stdlib copy).
- Protocol: a golden request/response transcript per method, versioned
  with the protocol number.

## Cross-links

- [10 — Binary format](10_binary_format.md) — `kb.open` / `kb.save`.
- [08 — Parallelism](08_parallelism.md) — what makes one process serve
  many solves.
- [01 — Parity contract](01_parity_contract.md) §3 — the event protocol
  reused here.
- [M1b GUI](../../m1b_gui/README.md) — the first real client.
