# S1a.5.1 — DOT renderers

**Phase:** P1a.5 (Presentation and CLI)
**Status:** **shipped** 2026-08-18 — acceptance below, plus `render_why`
brought forward from [S1a.5.2](s1a.5.2_trace_and_answer.md) T6, because
`render/slice` labels every rule node with a rendered `:why` and cannot be
checked without one.
**Estimate:** 4 days
**Depends on:** [P1a.4](../p1a.4_search_layer/README.md)
**Implements:** `ein/kb/render.py`, `ein/ir/to_dot.py`,
`ein/render/{rules,constraints,slice,lattice_dag,dot_util,palette}.py`

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `ir_oracle.py`. It is gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

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

All met. The instrument is a `dot-shape` op on `utils/ir_oracle.py` and
its Rust half `ein-render`'s `shape::dot_shape`, following the shape the
loader settled on at [S1a.2.3](../p1a.2_kb_core/s1a.2.3_loader.md): both
implementations render the same **seventeen views** of the same file and
the texts are diffed. Unlike `kb-shape` or `plan-shape` it invents no
rendering — every view calls a renderer entry point exactly as a CLI
subcommand or `trace.linearize` calls it, so what is compared is the
artefact a user sees, which is the whole point of the byte gate.

| item | result |
|---|---|
| All 16 golden `.dot` files reproduce byte-for-byte | **yes**, `golden_dot.rs` — 15 under `ein.py/tests/golden/dot/` plus `kb_zebra_unified.dot`, read from `ein.py/` rather than re-checked-in, because a port that ships its own copy of the expected bytes proves only that it agrees with itself. `kb_provenance_dag` is the sixteenth and was already `derivation_dot.rs`'s at [S1a.2.4](../p1a.2_kb_core/s1a.2.4_provenance.md) |
| `ein render rules\|rule\|constraints\|lattice` for every corpus entry, both `--rule-mode`s and both `--view`s | the `rules` / `rules-overlay` / `constraints` / `lattice` / `lattice-full` views |
| `kb.to_dot()` across its keyword surface | the `kb` / `kb-origin` / `kb-none` / `kb-no-types` / `kb-no-instances` / `kb-since` views — the last one saturates a fork so the `since=` transition highlight has a *pair* of KBs to diff |
| `ir.to_dot` for every node kind and every `rule_mode` / `levi` combination | the `ir` / `ir-levi` / `ir-overlay` / `ir-trace-dag` / `ir-forms` views, plus the fixture table below |
| **The whole sweep** | **98 files, 1 390 views, 4.3 MB, 0 differences** — the corpus minus the four parse-error fixtures, times seventeen views, minus the load-error entries where both sides refuse. The 3 that diverge are [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers) reached through every view that runs the search, and the sweep **asserts** them: a ledger entry that stopped diverging fails as loudly as one that started |
| Graphviz accepts every emitted file | **yes** — `dot_wellformed.rs` pipes every non-empty view through `dot -Tsvg`, so a byte-match is not accidentally a match on two broken files. It skips *loudly* when Graphviz is absent |

`cargo test --workspace`: **252 passed** (239 at P1a.4's close), of which
**45** are differential against `ein.py` — three of them this stage's.
`./run_tests.sh` on the Python side: 1 505 + 21, unchanged; the only file
this stage touched there is `utils/ir_oracle.py`, which the suite does not
import.

### The node kinds a corpus of puzzles does not contain

The corpus is a set of puzzles, so it exercises what puzzles contain:
no `(trace …)` form anywhere, no nullary fact, no range or string in an
argument position, none of the deprecated `(ontology …)` / `(facts …)` /
`(reasoning …)` wrappers, no `(= …)` outside two stdlib modules. Each is
a distinct branch of `_emit_fact` / `value_label` / `to_dot`'s dispatch,
and a branch nothing renders is a branch nothing compares — so
`dot_parity.rs` carries **18 hand-written fixtures**, sent as `text`
rather than a path and swept through every parse view: **144 fixture
views, 0 differences**.

Three of them started as branches that turned out to be *unreachable*
and were replaced rather than kept: the grammar rejects `(query)` with
no keyword args (so `render_query`'s `"query"` fallback label cannot
fire), `(trace …)` admits only `step` events (so `render_trace`'s
`ellipse` shape cannot), and a `(rule …)` needs at least one clause.

### The answer T1a.5.1.1 asked for

`hash_color` hashes with **`hashlib.sha1`**, not Python's salted
`hash()` — so it is stable across `PYTHONHASHSEED`, it is not a bug on
the same footing as `state_digest`, and there was nothing to fix in
ein.py before porting it. The port folds the digest byte-by-byte under
the modulus instead of building a 160-bit integer; same residue, no
bignum.

### Two shape departures, no byte departures

- **The wrapper forms are slices.** ein.py's `to_dot` re-groups a flat
  program by *synthesising* `SForm(head=Atom("facts"), args=…)` and
  recursing. Synthesising a node here would put a `&mut Ast` in a
  renderer, so `render_facts_forms` and friends take the children
  directly. The wrapper only ever contributed the digraph name.
- **`render_lattice --view full` has no `kb_index` to read.** ein.py's
  full view reads `LatticeProof.kb_index` and falls back to the solution
  frontier, with a note, when it is empty — and it is *always* empty:
  `solve`'s own proof packaging never writes it, which is what the
  `--view full` help text tells the user. The fallback is the behaviour,
  not an edge case, so the port emits the note and has no `kb_index`.

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
