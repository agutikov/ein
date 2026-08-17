# F10 — M1 refactor-debt tail (ex-P1.7c Track B)

The decompositions / unifications P1.7b identified but deferred, one stage
each. **Relocated** from `p1.7c_block_head_removal/` Track B when the M1
plan folder was deleted (P1.22 S1.22.99); Track A (S1.7c.1–.5, .8 — the
block-head removal) shipped 2026-06-02 and died with the folder.

**17 of the original 23 remain open.** The six that were assessed, done,
re-scoped or dropped during Track B were deleted rather than carried as
tombstones — see [§Closed](#closed--no-stage-file) for the one-line record
of each.

[`findings.md`](findings.md) — P1.7b's 40-finding, code-cited review — moved
with them: it is the source register every stage below cites.

## Trigger

**Before the [M1a Rust port](../../m1a_rust/README.md).** That is P1.7b's own
recommendation and the strongest reason to drain this: `ein.rs` should
transcribe the clean reference implementation, not the remaining scar
tissue. Otherwise these are opportunistic — each is behaviour-preserving,
independently landable, and gated by the same invariant (`run_tests.sh` +
`bench_solve_monotonic_pypy.sh`).

## The tail

The stage files are filed into **five group directories**, in the execution
order P1.7b suggested — the four ranked waves, then the remainder the
ordering never ranked.

| # | group | stages | rationale |
|---|---|---|---|
| ~~1~~ | ~~`1_trivial/`~~ | `.10` `.11` `.16` `.17` `.19` `.24` `.32` | **drained 2026-08-17** — see [§Group 1](#group-1--trivial--low-risk-drained-2026-08-17) |
| ~~2~~ | ~~`2_kb_store/`~~ | `.20` `.21` | **drained 2026-08-17** — see [§Group 2](#group-2--kbstorepy-drained-2026-08-17) |
| 3 | [`3_dot_emitter/`](3_dot_emitter/) | `.25` → `.26` | the RTC DOT pair, in that order — `.26` routes through `.25`'s API |
| 4 | [`4_trace_depth/`](4_trace_depth/) | `.29` | last of the ranked waves |
| 5 | [`5_unranked/`](5_unranked/) | `.12` `.13` `.14` `.18` `.27` | the five the suggested ordering does not rank — engine + CLI, `.18` is the only perf-shaped one |

| ID | title | finding | leverage / risk |
|---|---|---|---|
| **S1.7c.12** | Unify the provenance-chain DFS | F-KER-10 | low–med |
| **S1.7c.13** | `_lattice_public` post-amble | F-ENG-5 (+14) | low |
| **S1.7c.14** | Collapse unsat-core synthesis | F-ENG-7 | med |
| **S1.7c.18** | Drop redundant `consistent()` (perf) | F-ENG-12 | perf |
| **S1.7c.25** | Shared DOT emitter API | F-RTC-1 (+F-KB-8) | high (headline) |
| **S1.7c.26** | Decompose `to_dot` | F-KB-10 ≡ F-RTC-6 | med |
| **S1.7c.27** | Split `_build_parser` | F-RTC-2 | low–med |
| **S1.7c.29** | Flatten `parse_trace_steps` (depth 9) | F-RTC-4 | med |

Reuse the P1.7b acceptance gate as the invariant for every stage —
`run_tests.sh` (the `bench_solve_monotonic_pypy.sh` half of the original
gate no longer exists; `utils/profile_solve.py` is its replacement for the
perf-shaped `.18`).

## Drained — measured against HEAD, stub deleted

The stage files these replace were **not** open work: every one had already
landed during Track B (2026-06-02/03) and the done-status died with the M1
plan folder in P1.22 S1.22.99, which carried the *specs* across but not
their verdicts. Each line below names the landing commit and the
current-code evidence that its acceptance still holds.

### Group 1 — trivial / low risk (drained 2026-08-17)

- **S1.7c.10** (`FactId` neutral home, F-KER-6) — landed `a307f6b`.
  `FactId = tuple[str, tuple[object, ...]]` is defined once, at
  `kb/provenance.py:53`; `inference/apriori.py:31` now *imports* it from
  there, so the accidental solution→BFS coupling is gone. `ruff` clean.
- **S1.7c.11** (unify the swapped-arg `_resolve`, F-KER-7) — landed
  `b0b6a6c`. One implementation, `inference/resolve.py::resolve_leaf(slot,
  bindings, on_unbound)`, imported by `firing.py:32` and
  `predicates.py:39`. The genuine divergence — firing fails loud on an
  unbound `Var`, a predicate resolves it to `None` — became the
  `on_unbound` policy argument rather than a second copy.
- **S1.7c.16** (factor `_BaseStats`, F-ENG-9) — landed `56886a3`.
  `lattice.py:64` holds `_BaseStats`; `MonotonicStats` (`_state.py:50`) and
  `LatticeStats` (`lattice.py:91`) inherit it, and `_build_lattice_stats`
  (`_helpers.py:239`) copies via `fields(_BaseStats)` — the hand-maintained
  field list that could go stale is gone.
- **S1.7c.17** (`_TimelineMixin`, F-ENG-11) — landed `3496535`.
  `_serialise.py:147` carries `close` / `summary` / `_emit_timeline` once;
  mixed into `MonotonicDumper` (`state_dump.py:47`) and `LatticeDumper`
  (`_lattice_dump.py:66`).
- **S1.7c.19** (drop the two `type: ignore[arg-type]`, F-ENG-13) — landed
  `56886a3`. Zero `type: ignore` in `solver.py` and zero `[arg-type]`
  anywhere under `inference/monotonic/`. (The six left in `state_dump.py`
  are `[override]` on `ProgressDumper`'s narrowed signatures — a different
  suppression, never F-ENG-13's subject.)
- **S1.7c.24** (restore `Query` annotations, F-KB-13) — landed `78eca99`
  (typing) + `fc098a6` (tail). No `Any`-typed field survives in
  `store.py`: `Query` is just `kw_pairs: tuple[KwPair, ...]`, `config` is
  `SolverConfig | None` behind a `TYPE_CHECKING` forward ref (checker-only
  edge, no runtime cycle), and `alive` / `consume_stats` turned out to be
  dead vestiges of the removed back-prop tree-solver — **deleted** rather
  than typed, which is the stronger close.
- **S1.7c.32** (share the S-expr escaper, F-RTC-10) — landed `9e3660a`.
  `ir/strings.py::escape_string_literal` applies the full
  `\` `"` `\n` `\t` `\r` set for both `ir/dump.py:20` and `trace/ast.py:19`.
  The latent round-trip bug is locked by a **byte-level** regression test,
  `tests/trace/test_render.py::test_trace_control_chars_escaped_on_emit` —
  it asserts on emitted bytes precisely because a value round-trip alone is
  green pre-fix and proves nothing.

### Group 2 — `kb/store.py` (drained 2026-08-17)

- **S1.7c.20** (decompose `rebuild_indexes`, F-KB-2) — landed `fc098a6`,
  **half of it deliberately retired**. What shipped is the one live defect:
  the two walks over `self.facts` are fused into a single pass
  (`store.py:494-533`) that feeds every fact-derived index, the second walk
  having only rebuilt the head→facts grouping the first already held.
  What did *not* ship, and why: the spec's "164 ln / 8 indexes / split into
  per-index helpers / F-KB-11 `_rules_by_type` early-skip" was already
  stale when it was written — S1.7.23 had deleted `_rules_by_type`
  outright, and the function measures **62 code lines** today, not 164.
  fc098a6 assessed the helper-split as no longer load-bearing and dropped
  it. **Residue, stated plainly:** the spec's `< ~50 ln` bar is therefore
  unmet by ~12 lines, by decision rather than by omission. The three phases
  left (fused fact pass → `names` → `_rules_by_relation`) are each linear
  and distinct, so a split would buy naming, not structure.
- **S1.7c.21** (`snapshot` shallow-copy, F-KB-6) — landed `fc098a6`.
  `snapshot()` (`store.py:839`) routes through `_copy_fact_indexes_into`
  (`:891`), the single fork/snapshot copy contract; the per-solution
  `rebuild_indexes()` is gone. Soundness rests on `_index_fact` rebinding
  each key to a fresh immutable tuple rather than mutating in place, so the
  copy cannot leak across the source/snapshot boundary — and that is
  asserted, not assumed, by `test_kb_snapshot_indexes_match_rebuild`. The
  one behaviour delta is the `names` dict **key order**, now deterministic
  insertion order instead of the rebuild's per-process-random set order;
  never serialised, hence non-gated.

## Closed — no stage file

Two groups, both verified against the code and **not** carried here.
Recorded so a reader doesn't rediscover them as gaps.

**Assessed and closed during Track B (2026-06-03):**

- **S1.7c.15** (split `_LatticeLoopState`) — split *assessed and rejected*;
  one documented class kept instead.
- **S1.7c.22** (typed index wrappers) — re-scoped to a copy-helper and done.
- **S1.7c.23** (flatten `from_ir.load`) — superseded by Track A's flat-form
  loader rewrite.
- **S1.7c.28** (unify the two trace pipelines) — re-scoped and done.
- **S1.7c.30** (`linearize` dispatch table) — **won't-do**, user decision.
- **S1.7c.31** (public KB/verdict accessors) — done, both halves.

**Never opened, verified during the 2026-06-02 breakdown** — already closed
or deliberately dropped:

- **F-RTC-7** (`to_dot` unreachable return) — *done*: `_atom_arg_attrs` is a
  clean `Var`/`Wildcard`/default ladder; the trailing return is reachable.
- **F-RTC-8** (`cli.py` `and` short-circuit) — *done*.
- **F-KER-15** (`_instance_like_objects` hoist) — *moot*: the function no
  longer exists (subsumed by the name-free `_candidate_objects`, S1.7.23).
- **F-KB-11** (`type_names` vestige) — *parked, keep*: a deliberate S1.7.6
  seam, not a refactor deferral; folded into `.20`'s early-skip note.
- **EntryPolicy** (F-ENG-1 ideal) — *won't-do*: assessed a poor fit for
  three fixed entries (a re-dispatch sentinel = more indirection than the
  localized ladders).

## Connections

- [M1a Rust port](../../m1a_rust/README.md) — the trigger above.
- [F9 — hypothesis-loop E-catalog](../f9_e_catalog.md) (closed) — the other
  body relocated out of the M1 plans; that one is *feature* backlog, this
  one is *structural* debt.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat parser dispatches on (Track A's subject).
