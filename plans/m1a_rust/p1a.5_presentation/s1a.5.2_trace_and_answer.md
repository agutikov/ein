# S1a.5.2 — Trace and answer rendering

**Phase:** P1a.5 (Presentation and CLI)
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

- `ein solve --trace out.md` byte-identical for every corpus entry, in
  all four flag combinations (`--no-diagrams`, `--full-kb-snapshots`,
  `--reorder`, `--relevant`).
- `tests/golden/trace_3step.md` reproduces byte-for-byte.
- The solution table (`render_solution_table`) byte-identical for every
  verdict kind: Solution, Ambiguity (k>1, per-branch blocks),
  Contradiction (core + sources), Aborted.
- `render_why` substitution identical, including the positional `{?1}` /
  `{?2}` form and unresolved-placeholder behaviour.
- `parse(trace_to_ir(steps))` → `parse_trace_steps` round-trips on both
  sides.

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
