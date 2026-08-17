# S1a.5.1 — DOT renderers

**Phase:** P1a.5 (Presentation and CLI)
**Estimate:** 4 days
**Depends on:** [P1a.4](../p1a.4_search_layer/README.md)
**Implements:** `ein/kb/render.py`, `ein/ir/to_dot.py`,
`ein/render/{rules,constraints,slice,lattice_dag,dot_util,palette}.py`

## Context

Six renderers producing Graphviz `digraph` text, all pinned by
checked-in goldens (15 `.dot` files under `ein.py/tests/golden/dot/`
plus `kb_zebra_unified.dot`). DOT is unforgiving in the useful way: a
single differing attribute is a diff, so this stage either passes
exactly or tells you precisely where.

The one shared identity scheme is `dot_util.hashed_id(prefix, seed)` =
`prefix + md5(seed)[:10]`, collapsed onto one definition in S1.7c.25.
**Reproducing the digest is the easy half**; the hard half is
reproducing each caller's `seed` construction — `fact_key`'s flat
`rel|arg,arg` form and `render/slice`'s deliberately *recursive* key are
different on purpose and were not merged.

## Acceptance

- All 16 golden `.dot` files reproduce byte-for-byte.
- `ein render rules|rule|constraints|lattice` stdout byte-identical for
  every corpus entry, in both `--rule-mode` values and both `--view`
  values.
- `kb.to_dot()` byte-identical across its keyword surface
  (`colour_by`, `include_types`, `include_instances`, `name`).
- `ir.to_dot` byte-identical for every node kind and every
  `rule_mode` / `levi` combination.
- Graphviz accepts every emitted file (`dot -Tsvg /dev/null` smoke test),
  so a byte-match is not accidentally a match on two broken files.

## Tasks

### Task T1a.5.1.1 — `dot_util` and `palette`

`esc`, `quote`, `multiline`, `hashed_id`, `digraph_open`, `fact_key`,
`fact_label`, `value_label`; `PALETTE` and `hash_color` (verify whether
`hash_color` uses a stable digest or Python `hash()` — if the latter, it
is a ein.py bug on the same footing as `state_digest` and it gets fixed
before the port copies it).

### Task T1a.5.1.2 — `kb/render.to_dot`

The unified KB view: `_schema_nodes` (type set vs instance set, with the
`_two_strs` heuristic), `_emit_type_node` / `_emit_instance_node` /
`_emit_is_a_edge`, `_emit_binary_fact` / `_emit_unary_fact` /
`_emit_hyperedge`, `_emit_fact_line`, `_suppress`, `_pick_colour`,
`_label_extra`, `_short_source`. Node ordering comes from the KB's fact
order plus `sorted(type_set)` / `sorted(insts - types)`.

### Task T1a.5.1.3 — `ir/to_dot`

`render_ontology` / `render_facts` / `render_reasoning` / `render_query`
/ `render_trace` (views `a` / dag / group) and the `to_dot` dispatcher,
including `_emit_fact`'s derived styling, `_atom_id` / `_atom_id_for_value`,
`_atom_arg_attrs`, the `_Builder` accumulator with its `fresh_h`
hyperedge counter, and the `levi` transform.

The `_Builder`'s counter makes ids position-dependent — emit in the same
order or every id shifts.

### Task T1a.5.1.4 — `render/rules`

`_RuleRenderer` with `panel` (LHS / RHS clusters), `_clause_lines`,
`_relation_lines`, `_constraint_line`, `_fresh_hyper`, `_ordered_nodes`,
`_node_homes`, `_shape_attrs`, `_nid`, `_decl_lines`; the two modes
`_render_sidebyside` and `_render_overlay`; `_extract` pulling
`(name, match, assert)` off a rule SForm; `render_rule` / `render_rules`.

### Task T1a.5.1.5 — `render/constraints` and `render/slice`

`render_constraints` (declared relations, the property-tag notes,
`_is_ontology_form`, `_render_group`); `render_slice` / `render_state` /
`render_solution` with their recursive `_key` and the `_touch` closure.

### Task T1a.5.1.6 — `render/lattice_dag`

`render_lattice(source, view, name)` over a `LatticeProof` or a
`LatticeSnapshotV1`: `_Cell`, `_make_cell`, `_combine`, `_cells` /
`_proof_cells` / `_snapshot_cells`, `_dedup`, `_commit_label`,
`_cell_id`. `--view full` falls back to `solution` when the run stored no
per-commitment SetNode DAG — reproduce the fallback, including its
message.

## Notes

- Build the golden diff harness before writing any renderer: point it at
  `ein.py/tests/golden/dot/` and let it tell you which of the 16 is
  closest to done.
- Every renderer takes its data from the engine, so a DOT diff can be a
  *search-layer* bug surfacing late. When one fails, check the T1/T2
  status of that corpus entry first.
