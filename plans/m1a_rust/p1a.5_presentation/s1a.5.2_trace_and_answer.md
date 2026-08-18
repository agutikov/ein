# S1a.5.2 — Trace and answer rendering

**Phase:** P1a.5 (Presentation and CLI)
**Status:** **shipped** 2026-08-18 — acceptance below. T6 (`render_why`)
landed early, at [S1a.5.1](s1a.5.1_dot_renderers.md), because `render/slice`
labels every rule node with a rendered `:why`.
**Estimate:** 4 days
**Depends on:** [S1a.5.1](s1a.5.1_dot_renderers.md)
**Implements:** `ein/trace/{ast,linearize,relevance,render,answer}.py`,
`ein/inference/why.py`

## Context

Two user-facing artefacts, and they are the ones the project is *about*
— idea 08's human-style deductive trace and the solution table a reader
actually looks at. The trace is a self-contained markdown file with
inline DOT blocks; the answer is the table `ein solve` prints, whose text
comes entirely from puzzle templates (per-relation `:why`, per-query
`:goal-text`) with nothing hardcoded in the renderer.

The trace AST also round-trips: `trace_to_ir` emits `(trace (step …) …)`
forms the grammar parses back, and `parse_trace_steps` reads them. That
round-trip is a property test, not just a feature.

## Acceptance

All met. The instrument is a `trace-shape` op on `utils/ir_oracle.py` and
its Rust half `ein-render`'s `shape::trace_shape` — **three modes per
corpus entry, one solve each**, because the solve is the expensive part
and the renderers are the cheap part. The regime differs per mode on
purpose: `trace` solves *fast*, because that is what reaches a solution
and hands the renderer a spine to narrate; `answer` solves *exhaustively*,
because `Ambiguity` / `Contradiction` / `Aborted` live only there and they
are three of the table's four shapes.

| item | result |
|---|---|
| `--trace` byte-identical for every corpus entry, in all four flag combinations | **yes** — six markdown variants per file: the default, `--no-diagrams`, `--full-kb-snapshots`, `--reorder`, `--relevant`, and `--relevant --reorder`. The last two got their own rows because the stage said to; they are the ones nobody looks at |
| `tests/golden/trace_3step.md` reproduces byte-for-byte | **yes**, `golden_trace.rs`, from the committed file — and with it the reorder property ein.py asserts: same steps, grouped, each emitted once |
| The solution table for every verdict kind | **all four appear in the corpus** at this budget — Solution 36, Aborted 23, Ambiguity 4, Contradiction 2 — and each is rendered at *both* `exhausted` values, which is what exercises the `(not certified — pass --exhaustive)` qualifier |
| `render_why`, including the positional `{?1}` / `{?2}` form | rendered by every step's `:why` (named) and by the table's *rendered query facts* column (positional). Unresolved placeholders are **left as-is**, not blanked — pinned by a fixture in `why.rs` |
| `parse(trace_to_ir(steps))` → `parse_trace_steps` round-trips | **65 round-trips, all `ok`** — and asserted *separately* from the byte diff, because two implementations agreeing on `DIFFERS` would pass a byte diff and fail the property |
| **The whole sweep** | **65 files, 195 modes, 14.5 MB, 0 differences** — the same 65 the search layer reaches at [P1a.4](../p1a.4_search_layer/README.md); the rest are the files both loaders refuse. The one file that diverges is [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers), asserted in all three modes |

The `--relevant` prune is not a formality: on `zebra2` the trace goes from
**562 sections to 19**, which is the human-scale slice
`trace/relevance.py` promises and the reason idea-08's walkthrough is
comparable to an engine log at all.

### Three shape departures

- **A `TraceStep`'s facts are owned, not interned.** Everywhere else in
  the port a fact is a `FactId`. `parse_trace_steps` is the exception: it
  rebuilds facts naming relations and objects no KB ever held, so there
  is nothing to intern them into. `FactRef` is the owned
  `(relation_name, args)` shape ein.py's alias describes, and `fact_ref`
  converts one *out* of the KB where the linearizer needs it.
- **`linearize` takes a `root`.** ein.py reads provenance off the `Fact`
  object, which travels out of a fork that no longer exists; here
  provenance lives in the KB, so the linearizer needs one to resolve
  against when there is no spine KB — the `Contradiction`-without-a-
  solution case. It cannot change what is printed: the only field read is
  `:source`, which none but a load-time fact carries, and a load-time
  fact is in every fork's layer stack.
- **`goal_bindings` and `query_value` went to `ein-infer`, not here.**
  They are `inference/verdict.py`'s, they run the matcher, and
  [design/12](../design/12_toolchain_and_layout.md) §1 says `ein-infer`
  never formats — it does not say the renderer may reason. ein.py builds
  a synthetic `JoinPlan` around `compile_pattern(goal, {})`; there is no
  free-standing pattern compiler here, so the goal is wrapped in a
  parameter-less synthetic rule named `<query>`, which compiles to the
  same steps.

### What the no-proof branches needed

`linearize` has three branches for a verdict with no `LatticeProof`, and
the CLI cannot reach any of them: `--trace` sets `store_lattice=True`. So
the sweep has a third mode that solves *without* it — otherwise a third
of the linearizer would have shipped uncompared, which is exactly how the
port acquires a bug that only an embedder ever sees.

## Tasks

### Task T1a.5.2.1 — Trace AST

`TraceStep` (n, rule, derived, premises, bindings, why, dot),
`FactRef`, `derived_label` / `premise_labels`, `_arg_to_sexpr` /
`_fact_to_sexpr`, `step_to_ir`, `trace_to_ir`; and the reader
`parse_trace_steps` with `_parse_step` / `_parse_using` /
`_parse_bindings` / `_sform_to_factref` / `_atom_or_value`, reusing
`SForm.leading_symbol` and `SForm.kw_map` rather than re-scanning args
(S1.7c.28).

String escaping goes through `escape_string_literal`
([S1a.1.2](../p1a.1_ir_frontend/s1a.1.2_ast_and_dumper.md)) — the trace
writer and the IR dumper share it deliberately.

### Task T1a.5.2.2 — Linearisation

`linearize(verdict, diagrams, full_kb_snapshots, relevant)` →
`Trace(steps, reductios)`: `_build_steps` over the firings,
`_step_from_firing` (numbering, the per-step DOT slice, the `:why`
rendering), `_reductio` for each refuted commitment (its
`_commitment_label`, the learned clause rendered as
`", ".join(sorted(fact_label(...)))`, the core sources
`sorted({f.source for f in core if f.source})`), and `_target_entity`
for the reorder grouping.

### Task T1a.5.2.3 — Relevance pruning

`relevant_firings(firings, kb)`: seed from the goal relations
(`_solution_relations`, `_collect_goal_relations`, `_seed_keys`) and walk
backwards over premises, keeping only the goal-relevant slice. Keys are
`_key(fact)` tuples; in Rust they are `FactId`s and the visited set is a
bitset — but the *kept set and order* must be identical.

### Task T1a.5.2.4 — Markdown rendering

`render_markdown(trace, mode, diagrams)`: `_render_step`,
`_premises_line`, `_render_reductio`, `_dot_block`, and the `reorder`
mode's `_render_reordered` (cluster by target entity instead of engine
order). Heading levels, blank-line placement and code-fence language tags
all count.

### Task T1a.5.2.5 — The answer table

`render_answer(verdict, exhausted)` and
`render_solution_table(verdict, stats, exhausted, source)`:
the header, the `k` line with its "(not certified — pass --exhaustive)"
qualifier, the query-bindings block, the two-column *query facts /
rendered* table (`_two_col` with its column widths), the `result` line
from `:goal-text`, `_solution_block`, `_rule(width=62)`,
`_goal_text` / `_query_goal` / `_ground` / `_render_fact` / `_sexpr` /
`_conjuncts`, and the contradiction path's sorted core + source list.

### Task T1a.5.2.6 — `render_why`

`{?var}` substitution against the firing's bindings, plus the positional
`{?1}` / `{?2}` form used by relation-level `:why` templates. Unresolved
placeholders keep ein.py's behaviour exactly (verify: left as-is vs
blanked — whichever it is, pin it with a fixture).

## Notes

- The trace embeds DOT produced by
  [S1a.5.1](s1a.5.1_dot_renderers.md), so that stage lands first; a
  trace diff with a DOT bug underneath wastes a day.
- `--relevant` and `--reorder` are the two flags whose output nobody
  looks at often. Give them their own corpus rows, or they will be the
  ones that quietly differ.
