# F10 — M1 refactor-debt tail (ex-P1.7c Track B) — **closed 2026-08-17**

The decompositions / unifications P1.7b identified but deferred, one stage
each. **Relocated** from `p1.7c_block_head_removal/` Track B when the M1
plan folder was deleted (P1.22 S1.22.99); Track A (S1.7c.1–.5, .8 — the
block-head removal) shipped 2026-06-02 and died with the folder.

**All 23 are settled; nothing here is open.** The tail read as "17 open"
only because the P1.22 relocation carried the stage *specs* across but not
their verdicts, which lived in the deleted folder. Re-measuring all 17
against HEAD on 2026-08-17 found every one already landed during Track B
(2026-06-02/03) — **no code change was needed**, and the drain is a
bookkeeping correction, not a refactor. See
[§Drained](#drained--measured-against-head-stub-deleted) for the per-stage
record and [§Closed](#closed--no-stage-file) for the six settled earlier.

Two entries are worth reading before touching the code they name:
**`.20`** is the one acceptance criterion *retired* rather than met, and
**`.25`**'s headline (one shared DOT emitter) was measured and **rejected**.

[`findings.md`](findings.md) — P1.7b's 40-finding, code-cited review — is
what survives as the live artifact: the register every stage cited, plus
the two latent correctness bugs the review surfaced and the axis map from
"architecture / dead code / duplications / perf" to finding ids.

## Trigger — spent

**Before the [M1a Rust port](../../../docs/history/m1a_rust/README.md)** was the trigger:
`ein.rs` should transcribe the clean reference implementation, not the
remaining scar tissue. That condition is now satisfied in advance — the
port inherits the drained tree, so F10 imposes nothing on it. What the port
should still read is `findings.md`, for the *shape* of the debt this
codebase accumulates rather than any outstanding item.

## The tail — empty

The stage files were filed into **five group directories** in the execution
order P1.7b suggested (the four ranked waves, then the remainder the
ordering never ranked), then drained group by group.

| # | group | stages | rationale |
|---|---|---|---|
| ~~1~~ | ~~`1_trivial/`~~ | `.10` `.11` `.16` `.17` `.19` `.24` `.32` | **drained 2026-08-17** — see [§Group 1](#group-1--trivial--low-risk-drained-2026-08-17) |
| ~~2~~ | ~~`2_kb_store/`~~ | `.20` `.21` | **drained 2026-08-17** — see [§Group 2](#group-2--kbstorepy-drained-2026-08-17) |
| ~~3~~ | ~~`3_dot_emitter/`~~ | `.25` → `.26` | **drained 2026-08-17** — see [§Group 3](#group-3--the-rtc-dot-pair-drained-2026-08-17) |
| ~~4~~ | ~~`4_trace_depth/`~~ | `.29` | **drained 2026-08-17** — see [§Group 4](#group-4--trace-depth-drained-2026-08-17) |
| ~~5~~ | ~~`5_unranked/`~~ | `.12` `.13` `.14` `.18` `.27` | **drained 2026-08-17** — see [§Group 5](#group-5--the-unranked-remainder-drained-2026-08-17) |

Nothing is left in the table. The gate every stage was measured against is
`run_tests.sh` — the `bench_solve_monotonic_pypy.sh` half of P1.7b's
original invariant no longer exists (`utils/profile_solve.py` replaced it).

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

### Group 3 — the RTC DOT pair (drained 2026-08-17)

- **S1.7c.25** (shared DOT emitter API, F-RTC-1 + F-KB-8) — prep `c758e9f`,
  refactor `10d0d75`. The headline — "route all six renderers through one
  `node()/edge()/cluster()`" — was **measured and rejected**, and the stage
  file carried that verdict in its own status block. The line builders
  diverge past the point where one helper pays: `provenance` has neither a
  quote-fn nor edge attrs, `ir/_Builder` is optional-attrs, `rules` and
  `lattice` clusters are bespoke, and the preambles differ; a parameterised
  emitter would carry more knobs than the call sites it replaced.
  What did land, and holds today, is the byte-safe consolidation:
  `dot_util.hashed_id` (the single `prefix + md5(seed)[:10]` scheme,
  collapsing four hand-rolled copies), `dot_util.fact_key`, and
  `dot_util.digraph_open` — used across `kb/render.py:52`,
  `kb/provenance.py:38`, `render/slice.py:33`, `render/lattice_dag.py:37`,
  `render/constraints.py:41`; `import hashlib` survives only in
  `dot_util` itself and `palette` (unrelated colour hashing). **F-KB-8**
  went with it: `provenance`'s id was `md5[:12]` against everyone else's
  `[:10]`.
  The lasting artifact is the golden harness the prep built —
  `tests/render/test_golden_dot.py`, **15 cases** against
  `tests/golden/dot/*.dot` with an `UPDATE_GOLDEN` refresh path. It is what
  made byte-identity checkable at all (before it, only `kb/render.to_dot`
  had a golden), and it is why `.26` below could be verified rather than
  argued. Green on this run.
- **S1.7c.26** (decompose `to_dot`, F-KB-10 ≡ F-RTC-6) — landed `3e2ba39`.
  `to_dot` (`kb/render.py:211`) is **26 code lines** against a `< ~50` bar;
  the phases moved out to `_emit_schema_nodes` (`:274`) and the per-fact
  `is-a`/binary/hyperedge dispatch to `_emit_fact_line` (`:302`). Output
  byte-identity is held by the `kb_render_to_dot` golden.

### Group 4 — trace depth (drained 2026-08-17)

- **S1.7c.29** (flatten `parse_trace_steps`, F-RTC-4) — landed `2d368c1`.
  The repo's deepest function is no longer in `trace/ast.py` at all: an AST
  scan puts the module's **maximum nesting at 3** (`step_to_ir`), with
  `parse_trace_steps` itself at 2 — from the 9 the finding named, against a
  `< 5` bar. The `:bind` sub-parser and the `:using` arm became
  `_parse_bindings` (`:133`) and `_parse_using` (`:125`), the per-step body
  became `_parse_step` (`:144`, nesting 0), and the recurring
  `Atom→name / scalar→value / else str` ternary collapsed into
  `_atom_or_value` (`:102`). The round-trip gate the stage names,
  `tests/trace/test_render.py::test_trace_ast_round_trips`, is green
  (26 passed on this run).

### Group 5 — the unranked remainder (drained 2026-08-17)

Three landed and hold; two landed and were then **outlived by the code they
targeted** — worth reading, because a reader who greps for the symbols the
stage names will find nothing and could mistake that for a gap.

- **S1.7c.12** (unify the provenance DFS, F-KER-10) — landed `28cbd51` as
  one `_reaches(…, is_terminal)` with the two callers passing their
  terminal predicate; then **retired outright**. P1.21 R2 removed the
  unsound unconditional-fact extraction, which was `reaches`' sole caller,
  and `inference/back_prop.py` went with it — `walk_premises`'
  docstring in `kb/provenance.py` records exactly that. The two visited-set
  walks left in that module are not the duplicated pair: `walk_premises` is
  an accumulating closure walk, `find_provenance_cycles` a path-stack cycle
  detector. Different algorithms, no shared body to extract.
- **S1.7c.13** (`_lattice_public` post-amble, F-ENG-5 + F-ENG-14) — landed
  `9f0f66d`, now **moot on both halves**. The `gaps_solve` /
  `contradictions_solve` sibling entries — which chose the verdict up front
  — were removed in favour of the single `solve()` (`solver.py:69-70`
  carries the reasoning), so there is no duplicated post-amble left to
  share. F-ENG-14 dissolved with it: `solve()` returns `_explore_layers(…)`
  directly and there is no bare `assert isinstance` in the public return
  path, so nothing degrades under `python -O`. The contract test the stage
  demanded went out with the entries it guarded, which is the house rule —
  removing a special case removes its tests.
- **S1.7c.14** (collapse unsat-core synthesis, F-ENG-7) — landed `56886a3`
  and intact. Both helpers live in `_state.py`: `_union_dead_cores`
  (`:112`) and `_source_frontier_core` (`:121`), with the former called at
  `_state.py:168` and the latter at `_helpers.py:210` and `:350`. The 2×2
  duplication is one implementation each.
- **S1.7c.18** (drop the redundant `consistent()`, F-ENG-12) — landed
  `d0a9ac6`. The alive branch calls `complete(result.kb)` directly
  (`solver.py:340`), and the comment above it states the invariant the
  saving rests on: `try_commitment_set` returns `kind="alive"` only after
  its post-saturation `ContradictionDetector.detect()` came back empty, and
  `result.kb` is that unmutated fork — so `is_solution_node`'s consistency
  half would re-run a full detect on a kb already proved consistent. The
  same path got a second, independent win later: `complete()` now
  short-circuits on the generator's first candidate (`b9e7a60`, F9 E16),
  measured at 54 ms of a 1.7 s zebra2 solve.
- **S1.7c.27** (split `_build_parser` + `_load_kb_or_exit`, F-RTC-2) —
  landed `7ce72b0` and then carried cleanly through P1.11's `cli.py` →
  `cli/` folder move, which is the stronger end state the stage's
  "internal-only, does not pre-empt the P1.11 move" note anticipated.
  `_build_parser` (`cli/__init__.py:56`) is 10 lines delegating to
  `render.add_parser` / `solve.add_parser` / `_add_delegated`; each
  subcommand owns its own parser in its own module. `_load_kb_or_exit` and
  `_parse_or_exit` share `cli/_common.py` on one sentinel convention
  (return `None`, never `sys.exit`). `ein --help` and both subcommand
  helps render as expected.

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

## Why this directory survives its stages

The followups working agreement retires a directory when its last stub
goes. F10 keeps one because [`findings.md`](findings.md) is **not** a stage
spec: it is a 40-finding code-cited review register, too long for the
one-page rule and too specific to inline. The stage files were the parked
detail and are gone; the register is the parked *evidence* and stays.

## Connections

- [M1a Rust port](../../../docs/history/m1a_rust/README.md) — the (now spent) trigger
  above; the port inherits a drained tree.
- [F9 — hypothesis-loop E-catalog](../f9_e_catalog.md) (closed) — the other
  body relocated out of the M1 plans, and closed the same way: measure,
  delete the stub, keep the reason. That one is *feature* backlog, this one
  is *structural* debt.
- [`docs/kernel/ir/03-ein-lang/06_reserved_names.md`](../../../docs/kernel/ir/03-ein-lang/06_reserved_names.md)
  — the declarator set the flat parser dispatches on (Track A's subject).
