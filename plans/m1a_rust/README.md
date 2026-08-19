# M1a — Rust port (ein.rs)

**Estimate:** ~5.5 months focused — 43 stages, ~25 weeks of stage
estimates (parity gate at ~week 17).
**Status:** **in progress** — promoted from placeholder 2026-08-17 with the
scope decision made (see § The decision); [P1a.0](p1a.0_conformance_harness/README.md)
shipped the same day. Slotted between M1 and M1b.
**Scope change 2026-08-18:** server mode is **dropped** — see
§ Non-goals; P1a.8 keeps only the `.einb` container.
**Depends on:** M1 (**shipped** 2026-06-17) — the engine semantics are
frozen: kernel rules, NAF at the closure/world boundary (S1.21.8),
branching, no-good learning, the set-indexed lattice engine.
**Blocks:** [M1b](../m1b_gui/README.md) — the GUI binds to *the engine
that ships*; landing ein.rs first means M1b binds once, and after M1b's
2026-08-18 stack decision it binds by linking these crates into a Tauri
backend rather than talking to a process.
[M2](../m2_nl_to_ir/README.md)'s NL frontend is unaffected (it stays
CPython for llama.cpp) but talks to ein.rs across a binding boundary
(P1a.9).

---

## The decision

The placeholder deferred "**Boundary A** (full port) vs **Boundary B**
(hot-loop port behind PyO3)". **Resolved 2026-08-17: Boundary A.** ein.rs
re-implements the whole stack — IR parser, KB, engine, renderers, CLI —
as a standalone binary. PyO3 becomes an *output* of the port (P1a.9), not
its boundary.

Two invariants govern every stage, and they pull in opposite directions
on purpose:

> **I1 — Outside, nothing changes.** ein.rs is a drop-in replacement for
> `ein`: same surface language, same CLI, same stdout bytes, same exit
> codes, same DOT, same markdown trace, same verdicts, same counters,
> same error messages. Any observable difference is a bug in ein.rs, not
> a design liberty. `ein.py/` stays in the repo permanently as the
> **oracle**.
>
> **I2 — Inside, everything is on the table.** Atoms and facts become
> integers, tuples become flat interned rows, the fork becomes a
> zero-copy layer, the matcher becomes a register machine, the search
> layer runs on many cores, and a loaded KB can be stored and mapped
> back from a binary file. None of that is allowed to leak through I1.

I1 is what makes I2 safe. A rewrite with a byte-exact oracle is a
*measurable* rewrite: every optimisation is either parity-preserving or
rejected, and "did I break the semantics?" is answered by a harness, not
by reading. That is why P1a.0 (the conformance harness) comes before a
single line of engine code.

### Why a port at all (recap)

The placeholder's three reasons stand, and the numbers below sharpen the
second:

1. **Distribution.** M1b (GUI) and M2 (NL) ship to users; PyPy adds a
   second interpreter to install, ein.rs ships one binary.
2. **The hot loop is data-model-bound, not interpreter-bound.** See
   § Baseline: `_bind_arg` allocates a fresh `dict` per bound variable
   and compares interned-by-accident Python strings. That cost is
   structural, and PyPy only shaves a constant off it.
3. **Concurrency.** The lattice layer is embarrassingly parallel and the
   GIL forbids it. P1a.7 turns 101 independent enterings into 101
   independent tasks.

A fourth reason arrived with the F9/F11 ledgers: **the remaining named
levers are ones Python cannot hold.** [F11](../followups/f11_deductive_layer_perf.md)
parks RETE beta-memories precisely because "a memory that must be copied
per fork can lose more than it saves" — a problem that dissolves the
moment a fork is an `Arc` + a delta instead of a dict copy (see
[design/03](design/03_data_model.md)). F11 names the Rust port as its own
most likely promotion trigger.

---

## Baseline — what ein.rs has to beat

Measured 2026-08-17 on the dev machine, `examples/` unmodified,
`master` @ `601f002`. Read the *ratios*; the absolutes are
machine-specific.

| workload | CPython 3.14 | PyPy 3.11 |
|---|---:|---:|
| `solve zebra2.ein` (default, `stop_after=1`), end-to-end | 1.87 s | — |
| `solve zebra2.ein -e` (exhaustive), end-to-end | 5.69 s | 4.07 s |
| `solve zebra.ein -e` (exhaustive), end-to-end | — | 8.15 s |
| — of which: parse | 0.20 s | 0.27 s |
| — of which: kb load (imports + macro expansion + index build) | 0.43 s | 0.37 s |
| — of which: root saturation | 0.09 s | 0.32 s |
| — of which: hypothesis search | 4.96 s | 7.18 s |

Attribution (CPython + cProfile, `utils/profile_solve.py --exhaustive`,
zebra2, 20.4 s profiled / 74 M calls):

| site | self | cumulative | calls |
|---|---:|---:|---:|
| `match._bind_arg` | 20 % | 6.4 s | 6.0 M |
| `match._bind_args` | 18 % | 10.7 s | 4.6 M |
| `builtins.isinstance` | 14 % | — | 31.9 M |
| `match._run_steps` | 6 % | 12.3 s | 1.0 M |
| `saturator._binding_key` (+ genexpr) | 7 % | 2.7 s | 445 k |
| `engine._hashable` | 4 % | 1.2 s | 2.5 M |
| **`saturator._admit_from_boundary` → `World.first_failing`** | — | **14.7 s (72 %)** | 3.2 k rounds / 33 k guard queries |
| `fork` / index copy | 0.01 % | 0.003 s | 206 |

Three readings drive the design:

- **The matcher is the machine.** 46 % of self time is the match/bind
  subsystem, most of it unification — `isinstance` dispatch on IR node
  types plus a `{**bindings, name: arg}` dict copy *per bound variable*.
  [design/05](design/05_matcher.md) replaces both: slot-numbered
  registers with a backtrack trail, and a 4-byte `Value` compared by
  integer equality.
- **NAF costs more than the closure.** `_admit_from_boundary` dominates
  the exhaustive run: the same guard sub-plans are re-queried at every
  quiescence, throttled only by the `_watch_stamp` invalidation check.
  This is where an incremental negative index pays
  ([design/06](design/06_saturation.md) § Boundary).
- **The Python fork is already free** (0.003 s / 206 calls) — so the COW
  work is *not* about beating the current fork, it is about making
  hundreds of thousands of forks affordable so P1a.7 can run them in
  parallel and P1a.6 can afford beta-memories.

**Targets** (all at `--jobs 1`, so they measure the port and not the
cores): ≥ 20× on `solve zebra2 -e` end-to-end vs PyPy, ≥ 50× on parse +
load, and the [`ein.py`](../../ein.py) acceptance gate under 5 s.

> **Re-measured again at [S1a.6.1](p1a.6_performance/s1a.6.1_profile_baseline.md)
> (2026-08-18), and this whole section is superseded by
> [p1a.6_performance/baseline.md](p1a.6_performance/baseline.md).** Two of the
> PyPy figures moved *up* — `zebra2 -e` 4.07 → **4.94 s**, `zebra -e` 8.15 →
> **8.79 s** as processes — and one, the 0.78 s for parse + load, cannot be
> derived from its own components on either interpreter. The attribution above
> is ein.py's and still describes ein.py; **it does not describe ein.rs**,
> whose exhaustive `zebra2` is 59.7 % saturation / 29.0 % matcher where the
> table above says 46 % matcher and a 72 % boundary — and whose `zebra -e` is
> 66.9 % matcher, so the two puzzles no longer agree either. Read the old numbers as
> the reason the port was started, not as a description of what it became.
>
> **Re-measured at [P1a.0](p1a.0_conformance_harness/README.md), as this
> section asked: the acceptance gate is 43.7 s under PyPy 3.11, not the
> ~91 s recorded at S1.21.8** — 21 tests, `./run_tests.sh
> --acceptance-only`, 2026-08-17. Some of the gap is the machine and some
> is S1.9.E23's fail-fast fork saturation, which landed after that
> recording and cut ~64 % of dead-fork saturation time; the split is not
> worth chasing. What matters is that the "under 5 s" target is **~9×**,
> not the ~18× the stale number implied. The target stands; the claim
> about it does not.

These are targets, not promises; each phase records what it actually got
in [design/README](design/README.md) § Measured.

---

## Shared assets — one stdlib, one example corpus

Both implementations read the **same** `.ein` files. That is a hard
requirement, not a convenience: a forked stdlib would make every parity
result meaningless.

- `ein.py/src/ein/stdlib/*.ein` moves to repo-root **`stdlib/`**; the
  Python package keeps a build-time *copy* so wheels are unaffected, and
  a CI hash-manifest check fails if the copy drifts. ein.rs embeds the
  same tree in the binary (`include_dir!`).
- Resolution order is identical in both: `$EIN_STDLIB` → repo-root
  `stdlib/` when running from a checkout → the packaged/embedded copy.
- `examples/` stays where it is; both test suites enumerate it from the
  repo root, and the conformance corpus is derived from
  [`examples/README.md`](../../examples/README.md)'s catalog.

Full contract: [design/11](design/11_shared_assets.md).

---

## Phases

| phase | title | stages | est. | gate |
|---|---|---|---|---|
| [P1a.0](p1a.0_conformance_harness/README.md) ✅ | Conformance harness + shared assets | 4 | 2 w | **shipped 2026-08-17** — whole corpus 556 cells, 0 diff at T3; same across hash seeds |
| [P1a.1](p1a.1_ir_frontend/README.md) ✅ | IR frontend — lex, parse, AST, dump, macros, imports | 3 | 2 w | **shipped 2026-08-18** — dump / resolve / minimise / expand byte-identical on the corpus; 2.2 M fuzzer mutations, 0 diff; parse 1 003× |
| [P1a.2](p1a.2_kb_core/README.md) ✅ | KB core — interner, values, store, indexes, loader, provenance | 4 | 2.5 w | **shipped 2026-08-18** — 95 corpus files at KB-shape parity, every load error byte-identical; `fork` O(1) under a counting allocator; load 607×, RSS 15× |
| [P1a.3](p1a.3_deductive_core/README.md) ✅ | Deductive core — compile, match, saturate, world, contradiction | 4 | 3.5 w | **shipped 2026-08-18** — T2 on 64 files / 23 848 events, 0 diff; zebra 502 and zebra2 378 facts; `saturate_root` 31×, `match_hot` 55× |
| [P1a.4](p1a.4_search_layer/README.md) ✅ | Search layer — hypgen, lookahead, apriori, nogoods, lattice solve | 6 | 4 w | **shipped 2026-08-18** — 65 files at verdict + counter parity in three regimes; the three acceptance fixtures in 0.87 s; `solve zebra2 -e` 26× |
| [P1a.5](p1a.5_presentation/README.md) ✅ | Presentation — trace, DOT, dumps, CLI | 4 | 3 w | **shipped 2026-08-18** — T3 corpus-wide, 472/473 cells byte-identical, the one exception being D2; help *content* parity by structural diff (Q-M1a.13); **I1 discharged** |
| [P1a.6](p1a.6_performance/README.md) 🔄 | Performance — the optimisation programme | 11 | 3.5 w | **all four targets met 2026-08-19** (S1a.6.8, S1a.6.9) and held with **88 % of headroom** after S1a.6.12 — `solve zebra -e` **585.8 → 47.5 ms** across the phase, `zebra2 -e` → 28.9 ms, both ~**165× PyPy**. Parity is no longer *byte*-unbroken and that is a decision: a fork resuming root's saturation narrates a quarter as much ([D3](divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it), [Q-M1a.18](open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)), so S1a.6.10 moved the contract to *what a fork derives* — **T3 479/480, T2 243/244, D2 the only cell** — and S1a.6.11 replaced the elided bytes with twelve ein.rs goldens |
| [P1a.7](p1a.7_parallelism/README.md) | Parallelism — deterministic multi-core search + match | 5 | 2.5 w | `--jobs N` verdict- **and** counter-identical |
| [P1a.8](p1a.8_binary_container/README.md) | Binary KB container — `.einb`, mmap, solution store | 1 | 0.5 w | `ein solve x.einb` byte-identical to `ein solve x.ein` |
| [P1a.9](p1a.9_bindings_release/README.md) | Bindings + release — PyO3, packaging, docs | 4 | 1.5 w | M2 imports the engine and gets ein.rs |

44 stages (S1a.6.8 added by S1a.6.1's profile, S1a.6.5 shortened by it,
S1a.6.12 written at S1a.6.5 against the profile that had named it since
S1a.6.3), 130 days of stage estimates ≈ 26 weeks. The **parity gate**
(end of P1a.5) is at ~week 17; everything after it is speed, scale and
distribution on an engine that is already a drop-in replacement.

> **P1a.8 was "Server mode" until 2026-08-18** — 8 stages, 3 weeks:
> daemon, sessions, JSON-RPC, streaming, a solution cache and `ein <cmd>
> --server`. Dropped: nothing downstream needs a resident process.
> [M1b](../m1b_gui/README.md) settled on Tauri, whose backend *is* a Rust
> process linking these crates directly; [M2](../m2_nl_to_ir/README.md)
> crosses into CPython through PyO3 (P1a.9); the CLI is the only other
> consumer. The one deliverable that was never about the daemon — the
> `.einb` container — stays as a single stage. The seven server stages and
> `design/09` are in git history.

Ordering rationale: **parity first, speed second, scale third.** P1a.0–5
land a slower-than-Python but byte-identical engine; only then does
P1a.6 start trading representation for time, with the harness watching.
Doing it the other way round means every regression is ambiguous.

---

## Non-goals

- **Re-deriving M1's semantics.** Every invariant M1 established
  (S1.5a.1 NAF re-eval, S1.5a.1a determinism, S1.7.23 no kernel type
  system, S1.21.8 closure/boundary, P1.21 R2 root stability) is a *port
  target*, not a redesign target. Where a Python behaviour looks wrong,
  the fix belongs in ein.py first — then both ports move together.
- **A "Rusty" reinterpretation of the IR.** No new syntax, no new
  keywords, no relaxed grammar. `grammar.lark` stays the spec of record
  (M2's GBNF lift reads it); the Rust parser is checked *against* it.
- **Deleting ein.py.** It is the oracle and the reference for M2
  experiments. It stays, and stays green.
- **Dropping PyPy support.** It keeps working; it is simply no longer
  the deployment target.
- **A resident server.** Dropped 2026-08-18 (see the phase table).
  ein.rs ships a **library and a CLI**; an embedder that wants
  load-once/ask-many holds the engine in its own process — which is
  exactly what M1b's Tauri backend and M2's PyO3 binding do. No daemon,
  no wire protocol, no solution cache, no `--server` flag.
- **New reasoning features.** Anything that changes what the engine can
  prove belongs in a followup ([F2](../followups/f2_self_modifying_language.md),
  [F4](../followups/f4_cross_cutting.md), [F7](../followups/f7_rule_induction.md)),
  not here.

---

## Design docs

The technical substance lives in [`design/`](design/README.md):

| doc | what it settles |
|---|---|
| [01 — Parity contract](design/01_parity_contract.md) | what "1:1" means, the four tiers, the oracle event protocol, the corpus |
| [02 — Determinism & order](design/02_determinism_and_order.md) | every order-sensitive iteration site in ein.py, and how ein.rs reproduces it |
| [03 — Data model](design/03_data_model.md) | interning, `Value`/`FactId` as integers, row storage, the layered COW KB |
| [04 — IR frontend](design/04_ir_frontend.md) | hand-written lexer/parser, AST arena, dumper, macro expander, import resolver |
| [05 — Matcher](design/05_matcher.md) | plan bytecode, register bindings + trail, indexes, beta-memories, WCOJ |
| [06 — Saturation](design/06_saturation.md) | closure/boundary loop, semi-naive delta, queues, incremental NAF |
| [07 — Search layer](design/07_search_layer.md) | hypgen, lookahead, apriori, nogoods, the monotonic loop |
| [08 — Parallelism](design/08_parallelism.md) | the four parallel levels and how each stays deterministic |
| [10 — Binary format](design/10_binary_format.md) | `.einb` container, mmap, versioning, content addressing |
| [11 — Shared assets](design/11_shared_assets.md) | one stdlib, one example corpus, drift checks |
| [12 — Toolchain & layout](design/12_toolchain_and_layout.md) | crates, dependencies, build, CI, benches |

---

## Open questions

Live questions carry `Q-M1a.<n>` ids in
[`open_questions.md`](open_questions.md). The load-bearing ones at
promotion time: parse-error message parity (Q-M1a.3) and whether
`--jobs > 1` may move counters (Q-M1a.7). The two server questions
(Q-M1a.11 wire protocol, Q-M1a.12 remote access) were **closed moot
2026-08-18** with the server itself. P1a.3 added one the design docs did
not anticipate: [Q-M1a.17](open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate),
where Win B's ≥ 80 % target met its own measurement and lost.

P1a.6 answered the one nobody had written down before S1a.6.9 forced it.
**Q-M1a.18** — may a fork stop re-narrating the root's fixpoint? — resolved
**yes, in ein.rs only**, and the principle that moved with it is bigger than
the question: the contract's hard requirement is that the two engines produce
the same *answer*, not the same bytes. T0 and T1 stay exact and are compared
more carefully than before; narration parity was a means that had served its
purpose, and ein.rs's regression coverage moved to checked-in goldens.

P1a.4 closed the two it was blocking on. **Q-M1a.4** — `sorted()` over
mixed-type fact args — became the ledger's
[D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
once `layer_1` was reachable: exactly one corpus file diverges, exactly
the predicted one, and the parity sweep *asserts* the divergence rather
than tolerating it. **Q-M1a.5** — CPython's `random.shuffle` — was
resolved by porting MT19937, checked by table and then on every corpus
entry through a seeded `solve` regime.

## Cross-links

- [`docs/kernel/`](../../docs/kernel/README.md) — the specification
  ein.rs implements. `inference/architecture_and_algorithms.md` §O1–O9 is
  the operation-by-operation map every design doc here refers back to.
- [`docs/api/ein.md`](../../docs/api/ein.md) — the Python embedding
  contract P1a.9's PyO3 surface must keep.
- [F11 — deductive-layer perf](../followups/f11_deductive_layer_perf.md)
  — D1 (beta-memories) and D2 (WCOJ); this milestone is their promotion
  trigger. Absorbed by [P1a.6](p1a.6_performance/README.md).
- [F10 — M1 refactor-debt tail](../followups/f10_m1_refactor_tail/README.md)
  — closed 2026-08-17; nothing left blocking the port.
- [F9 — E-catalog](../followups/f9_e_catalog.md) — the *rejected*
  search-layer optimisations. Read before proposing one here: most were
  measured inert against a complete cardinality-BFS, and a Rust rewrite
  does not change that arithmetic.
- [M1b GUI](../m1b_gui/README.md) · [M2 NL → IR](../m2_nl_to_ir/README.md)
  — the downstream consumers.
