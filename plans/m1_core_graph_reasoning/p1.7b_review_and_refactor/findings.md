# P1.7b — Review findings register

The code-grounded review that this phase acts on. Every finding cites
`file:line` and carries a stable id (`F-<area>-<n>`) the stage files
reference. Areas: **ENG** = monotonic search engine, **KER** = inference
kernel, **KB** = data model, **RTC** = render / trace / cli / ir.

Review method (2026-06-01): ruff (clean — this is *structural*, not lint,
debt), an AST span/branch/nesting scan over the whole `ein_bot` package,
four parallel subsystem reviews, then **direct code-path verification** of
every correctness-touching or deletion-bound claim (per the project rule
*"verify engine claims — read the code path, don't assert from
assumption"*). Claims marked ✅-verified were re-read at the cited lines by
the author, not just reported.

## How this maps to the requested axes

| requested axis | where it lands |
|---|---|
| **architecture** | F-ENG-1 (one loop = 3 fused state machines), F-KB-3/9 (add/index contract + typed index wrapper), F-RTC-1 (no shared DOT emitter), F-KER-6 (`FactId` home) |
| **too-long functions / files** | F-ENG-1 (`_explore_layers` 621 ln), F-KB-2 (`rebuild_indexes` 164 ln), F-KER-2/3 (`back_propagate`/`_compile_premise`), F-RTC-2/4/5/6 (`_build_parser`, `parse_trace_steps` depth 9, `linearize`, `to_dot`) |
| **dead code** | F-KER-9 (7 dead fns), F-ENG-2/3/10 (`verdict_entry`, stale `monotonic_solve`/`tree` docs, unused `early_terminate` hook), F-KER-1/8 (`Mode`/`is_solved` vestige, `Saturator` test-only surface), F-KB-11 (`type_names` always `()`) |
| **duplications** | F-RTC-1 (DOT helpers ×5), F-KER-5/7/10 (symmetric lookup ×4, two `_resolve`, provenance walker ×3), F-ENG-5/7/9/11 (entry post-amble, unsat-core ×4, stats pair, dumper methods) |
| **overcomplicated** | F-ENG-4/6 (`mode` neutraliser, `phase_2_done` goto), F-KER-4 (`_coerce` if-ladder), F-RTC-4 (depth-9 parser) |
| **Python best practices** | F-KER-4 (`from __future__ annotations` makes `is bool` dead), F-KB-13 (`Any`-typed to dodge cycle), F-ENG-13/14 (`type: ignore`, `assert` as control flow), F-RTC-1 (f-string vs join inconsistency) |
| **readability / extensibility / maintainability** | the decompositions above + F-KB-9 (index wrapper), F-ENG-8 (per-entry accumulators) |
| **performance** | F-KB-1/3/4/5/6 (the `fork`/`add_fact`/`_index_fact`/`snapshot` hot-path cluster), F-KER-15 (recomputed `_instance_like_objects`), F-ENG-12 (redundant `consistent()` re-check) |

## Two latent **correctness** bugs surfaced by the review

Not style — these are real, and their fixes are folded into the relevant
refactor stages (so the cleanup also closes them):

- **F-KER-4 — `(config :lattice-order-seed N)` raises at load.** ✅-verified.
  `config.py:25` has `from __future__ import annotations`, so `field.type`
  is always a **string**; `_coerce` (`config.py:205`) tests `field_type is
  int` (dead — always False) then `== "int"` (exact). A field typed
  `int | None` stringifies to `"int | None"`, matches no arm, and falls to
  the `raise ValueError(... unsupported type)` at `config.py:255`. Any
  `Optional` config flag set via the IR `(config …)` block crashes the
  loader. Fix: the dispatch-table rewrite in **S1.7b.3**.
- **F-KB-3 — `add_fact` + `_index_fact` double-index on the dedup path.**
  ✅-verified at `store.py:257-274` / `:462` and caller `firing.py:170-171`.
  `add_fact` dedups against `self.facts` and may return a *pre-existing*
  Fact; the caller then *unconditionally* calls `_index_fact(stored)`, which
  appends to `_facts_by_relation` / `names` / … with **no membership
  guard** → the same Fact lands in the indexes twice. External output is
  largely *masked* (the engine's downstream firing-key dedup in `engine.py`
  collapses the duplicate matches), which is why tests stay green — but the
  index state is corrupt (over-counts), grows unbounded with re-derivations,
  and costs hot-path time. Fix: make `add_fact` own indexing / return a
  was-new signal, in **S1.7b.4**.

---

## ENG — monotonic search engine (`inference/monotonic/`)

| id | pri | category | location | finding (✅ = author-verified) |
|---|---|---|---|---|
| **F-ENG-1** | **High** | architecture / long-function | `solver.py:602-1222` | ✅ `_explore_layers` is **621 lines**, 91 branches, nesting depth 7-9. It fuses the solve / gaps / contradictions flows through **~24 `entry ==` discriminator sites** (`:761,767,775,790,795,812,817,900,931,957,966,992,1013,1031,1040,1090,1108,1114,1130,1140,1149,1162,1174,1213`). The three flows barely overlap; a reader executes three programs at once. The single biggest debt in the codebase. |
| **F-ENG-2** | Med | dead-code | `contract.py:157-174`, `__init__.py:77,121` | ✅ `verdict_entry` has **no functional caller** (only `__all__` + re-export). Worse, it maps `Solution → "monotonic"` — the label of the *removed* `monotonic_solve`. Delete. |
| **F-ENG-3** | Med | readability (stale docs) | `solver.py:62-67,1240-1241`; `lattice.py:4`; `snapshot.py:113,119`; `state_dump.py:3-5`; `verdict.py` | ✅ Docstrings describe a *removed* world as current: `solver.py:66` "loop raises NotImplementedError if [gaps/contradictions]" (both are fully implemented in this file); `solver.py:1241` "Skeleton stage … both raise NotImplementedError"; `monotonic_solve` referenced in 9 src sites though removed 2026-05-31; `state_dump.py:4` "mirrors `tree.state_dump`" (tree engine deleted). Actively misleads. |
| **F-ENG-4** | Med | overcomplication | `solver.py:665-675` | ✅ `mode = Mode.CONTRADICTIONS if entry=="solve" else Mode.SOLVE` — a 10-line comment explains it passes a *deliberately wrong* mode so `is_solved` returns False everywhere on the solve path. `mode` now encodes two axes (which-entry × goal-semantics). See F-KER-1. |
| **F-ENG-5** | Med | duplication | `solver.py:231-272,1244-1297,1300-1360` | `solve`/`gaps_solve`/`contradictions_solve` are thin wrappers; the gaps/contradictions pair share a verbatim 4-line post-amble (`assert isinstance(verdict, X); assert proof; return verdict, proof.stats`) differing only in `X`. Extract `_lattice_public(entry, expected)`. |
| **F-ENG-6** | Med | overcomplication | `solver.py:825-828,1090-1171` | `phase_2_done = True; break` appears 7× — a boolean-as-goto that re-breaks the layer loop one level up. With ~12 exit points it is most of why the loop is unreadable. Replace with a `_Step` enum returned by the extracted per-candidate handler. |
| **F-ENG-7** | Med | duplication | `solver.py:225-228,541-544` and `:441-446,739-742` | Unsat-core synthesis duplicated 2×2: dead-core union (`verdict_of` vs `_finalise_lattice_verdict`) and source-frontier core (`_contradiction` vs `_root_dead`). Collapse to `_union_dead_cores` + `_source_frontier_core`. |
| **F-ENG-8** | Med | architecture | `solver.py:152-198` | `_LatticeLoopState` mixes solve-only fields (`solution_nodes`,`truncated`) and lattice-only fields (`solutions`,`kb_index`,`state_hash_merges`,`root_was_solved`); no run uses all. 30-line docstring caveats per-field-per-entry. Split into per-entry accumulators. |
| **F-ENG-9** | Low | duplication | `solver.py:129-149` vs `lattice.py:54-82` | `MonotonicStats` / `LatticeStats` share 11 byte-identical counter fields; `_build_lattice_stats` (`solver.py:464-495`) copies them by hand. Factor a `_BaseStats`. |
| **F-ENG-10** | Low | dead-code | `state_dump.py:299-303`; `solver.py:54-60` | ✅ `early_terminate` dumper hook is **never fired** by the solver (grep: 0 `dumper.early_terminate(` calls); the "six sites" docstring lists it anyway. Wire it at the real stop sites or delete the hook. |
| **F-ENG-11** | Low | duplication | `state_dump.py:305-358` vs `:869-892` | `_emit_timeline`/`summary`/`close` duplicated verbatim across `MonotonicDumper` and `LatticeDumper` (~50 ln; differ only by `default=str`). Extract a `_TimelineMixin`. |
| **F-ENG-12** | Low | performance | `solver.py:956-959` → `solution.py:70` | On the alive solve branch, `is_solution_node` re-runs `consistent()` = a full `ContradictionDetector(kb).detect()` on a kb `try_commitment_set` *already* proved consistent for `kind=="alive"`. One redundant detect per alive commitment (hot path). Call `complete()` directly there. |
| **F-ENG-13** | Low | best-practice | `solver.py:904,969` | Two `# type: ignore[arg-type]` papering over `Literal` non-narrowing through an `in`-tuple guard; vanish under the EntryPolicy split (F-ENG-1) or with explicit `==` narrowing. |
| **F-ENG-14** | Low | best-practice | `solver.py:1295-1296,1358-1359` | Public return-type contract enforced only by `assert` (stripped under `python -O` → opaque `AttributeError`). Fold into `_lattice_public` (F-ENG-5). |

Clean, no change needed: `sanity.py`, `snapshot.py` (modulo F-ENG-3), the
`lattice.py` proof records.

## KER — inference kernel (`inference/*.py`, excl. `monotonic/`)

| id | pri | category | location | finding (✅ = author-verified) |
|---|---|---|---|---|
| **F-KER-1** | **High** | dead-code / architecture | `verdict.py:59,144`; `solver.py:675` | The P1.7a leftover. `solve()` threads `Mode` *only to neutralise* `is_solved` (F-ENG-4); `Mode.GAPS` is never constructed by any entry. `is_solved` is still live for gaps/contradictions, so it can't be deleted outright — but the loop's terminator should be an **injected predicate** (`solve`→`is_solution_node`; others→`is_solved(…,SOLVE)`), after which `Mode` collapses to ≤2 values and the neutraliser hack dies. |
| **F-KER-2** | High | long-function / duplication | `back_prop.py:217-332` | ✅ `back_propagate` (116 ln) writes primary `(not h)`, bubbles to ancestors, writes the symmetric mirror, bubbles *that* — the ancestor-bubble loop appears **twice near-identically** (`:295-298` vs `:312-315`). Extract `_bubble_fact(fact, ancestors, …) -> int` + `_eager_abort_if_needed`. Body → ~40 ln. |
| **F-KER-3** | High | long-function | `compile.py:212-350` | `_compile_premise` (139 ln, 24 br) interleaves *desugaring* (`open`/`forall` → `absent`/`and`, `:244-292`) with *opcode lowering*. Extract `_desugar_open`/`_desugar_forall` + a head-dispatch dict + `_compile_relation`. |
| **F-KER-4** | **High** | dead-code / best-practice / **bug** | `config.py:205-258` | ✅ See the latent-bug box above. 4 permanently-dead `is`-branches + a load-time crash on `int | None` flags. Rewrite as a `{token: coercer}` dispatch keyed on `field.type.split("|")[0].strip()`. |
| **F-KER-5** | Med | duplication | `back_prop.py:179`, `hypgen.py:321`, `solution.py:42`, `solver.py:355` | Symmetric-relation lookup (`kb._facts_by_relation.get("symmetric",())` + `any/​set`) written 4×; `is_symmetric_relation` already exists but 3 sites don't call it. Promote `kb.is_symmetric(name)` / `kb.symmetric_relations()`. |
| **F-KER-6** | Med | duplication / architecture | `apriori.py:32`, `nogoods.py:53`, `back_prop.py:114`; `solution.py:24` | `FactId = tuple[str, tuple]` defined 3× and `solution.py` imports it **from `apriori`** — a domain-agnostic predicate coupled to the BFS module by accident of where the alias lives. Define once in a neutral home. |
| **F-KER-7** | Med | readability | `firing.py:76` vs `predicates.py:48` | Two `_resolve` in one package with **swapped** arg order (`(slot,bindings)` vs `(bindings,slot)`), both resolving `Var`/`Atom`/`Int` — a copy-paste-between-files footgun. Rename + align, or share a `_resolve_leaf`. |
| **F-KER-8** | Med | dead-code | `saturator.py:155,175,191` | ✅ `solved()` returns hardcoded `False` ("Plugged in by P1.5/P1.7" — but P1.7a put the terminator in `solution.is_solution_node`, so it never will be); `is_stalled`/`contradictions` have **no `src/` callers** (test-only). Delete `solved()`; decide on the other two. |
| **F-KER-9** | Med | dead-code | see below | ✅ Six functions with **zero callers** anywhere (grep-verified across src/tests/demo/acceptance): `_mode_from_query` (`verdict.py:169`), `filter_by_nogoods` (`nogoods.py:132`), `has_open_hypothesis` (`solution.py:56`), `derives_positive` (`firing.py:58`), `total_filtered` (`hypgen.py:90`), and `reaches_hypothesis` (`back_prop.py:132`, only docstring/`__all__` refs; `commitment.py` reimplemented it). Delete. |
| **F-KER-10** | Med | duplication | `back_prop.py:96`, `commitment.py:200` | Provenance-chain DFS (`_walk` / `_reaches_commitment`) triplicated (+ the dead `reaches_hypothesis` wrapper); differ only in the terminal test. One `_reaches(kb, fact, visited, is_terminal)`. |
| **F-KER-15** | Low | performance | `hypgen.py:272` | `_instance_like_objects(kb)` (full `_facts_by_relation` scan + set-diff) recomputed O(objects×relations×slots) times inside `_generate` though constant per call. Hoist to the top of `_generate`. |

Hygiene **positive**: no bare `except:`, no `except Exception`, no mutable
default args anywhere in the 21 kernel files; `@dataclass(frozen=True)`
usage is consistent. Lower-priority notes (L11-L14 in the raw review):
`Engine.step/saturate` are reference-only (used by tests/bench), `getattr`
defensive reads of a frozen `cfg` in `hypgen.py:440`, the
`Join.shared_vars`/`activator_args` "informational" fields.

## KB — data model (`kb/*.py`)

Hot-path map (grep-established): `fork()` runs per commitment branch;
`snapshot()` per recorded solution; `_index_fact()` after every fact add
(saturation loop); `rebuild_indexes()` at load + inside `snapshot`. Indexes
the inference layer actually *reads*: `_facts_by_relation` (19×),
`_negated_facts` (12×), `_rule_apps_by_rule`, `_facts_by_instance`, `names`.
**Never read by inference**: `_types_by_parent`, `_instances_by_type`,
`_rules_by_relation`, `_rules_by_type` (only the entity `@property`
accessors touch them), and `add_relation`/`add_rule`/`add_hrule` have **no
callers outside the loader** → those four indexes are immutable post-load.

| id | pri | category | location | finding (✅ = author-verified) |
|---|---|---|---|---|
| **F-KB-3** | **High** | architecture / **bug** / perf | `store.py:257-274,462`; callers `firing.py:171`,`back_prop.py:213`,`commitment.py:115`,`closed.py:81`,`solver.py:{348,400,1070}` | ✅ See latent-bug box. `add_fact` returns a possibly-pre-existing Fact; callers unconditionally `_index_fact` it → duplicate index entries (masked externally by firing-key dedup, but corrupt + hot-path cost). Make `add_fact` own indexing or return was-new. |
| **F-KB-4** | High | performance (hot path) | `store.py:269-273` | ✅ `add_fact` dedups by a linear `for existing in self.facts` scan — O(facts) on **every** add, inside saturation. `_fact_by_id` (`:570`) already does the O(deg) relation-index lookup. Reuse it. |
| **F-KB-5** | High | performance (hot path) | `store.py:469-529` | ✅ `_index_fact` rebuilds whole dicts via `{**self._facts_by_relation, rn:…}` per fact, and `{**self.names, name:…}` **per name per fact** → O(\|names\|) per add. `fork`/`snapshot` already give each kb its own dict objects, so in-place `d[k]=…` is safe and O(1). |
| **F-KB-2** | High | long-function | `store.py:278-441` | ✅ `rebuild_indexes` (164 ln, 61 br) builds 8 indexes inline and walks `self.facts` **twice** (`:351-356` and `:382-388` collect nearly the same head/arg groupings). Split into per-index builders; fuse the two fact walks. |
| **F-KB-1** | Med | performance (hot path) | `store.py:668-673` | ✅ `fork()` does `dict(...)` on the four post-load-immutable indexes (`_types_by_parent`,`_instances_by_type`,`_rules_by_relation`,`_rules_by_type`). Share by reference like `types`/`rules`. *(Magnitude is small — these dicts are tiny — but it is a free, provably-safe win once F-KB-9 encodes the invariant.)* |
| **F-KB-6** | Med | performance | `store.py:697-750` | `snapshot()` calls a full `rebuild_indexes()` per recorded solution though it copies `facts` verbatim from a kb whose indexes are already consistent. Shallow-copy the index dicts instead (warm, not as hot as fork). |
| **F-KB-7** | Med | long-function / duplication | `from_ir.py:361-445,126,204,274` | `load` (nesting depth 8) has an 8-way head dispatch + parallel block lists; `_ingest_ontology`/`_ingest_rules`/`_ingest_facts` each re-derive the "is-it-an-SForm-what's-the-head" preamble. Extract `_dispatch_blocks` + `_form_head` + `_build_rule`. |
| **F-KB-9** | Med | architecture | `store.py:175-194` | 8 loose `_*_by_*` dicts hand-maintained in 4 places (`__init__`/`rebuild`/`fork`/`_index_fact`); `_index_fact` *correctly* omits 4 of them but nothing **enforces** the load-invariance that makes that correct. A `_FactIndexes` (mutable, `add`/`copy`) + `_StaticIndexes` (shared, `copy`=identity) wrapper makes F-KB-1/5/6 structurally safe. |
| **F-KB-10** | Med | long-function | `render.py:165-294` | `to_dot` (130 ln) inlines 6 phases + the per-fact `is-a`/binary/hyperedge dispatch. Extract `_resolve_nodes` + `_emit_fact_line`. (Also the other half of F-RTC-1.) |
| **F-KB-11** | Low | dead-code (parked) | `pattern.py:46,51`; `store.py:429-441`; `entities.py:120` | ✅ `Pattern.type_names` is **never populated** (no `types.append` exists), so `_rules_by_type` is always `{}` and `Type.rules` always `()`. A deliberately-parked S1.7.6 seam — keep, but consolidate the 4 scattered notes into one and early-skip the empty `_rules_by_type` block. |
| **F-KB-8** | Low | duplication | `render.py:71`, `provenance.py:173` | Two md5 `(relation,args)→f_<hash>` node-id helpers differing only in hash length (10 vs 12) → can drift. Part of F-RTC-1's shared `node_id`. |
| **F-KB-13** | Low | best-practice | `store.py:98,133,141,149` | `Query.kw_pairs`/`config`/`alive`/`consume_stats` typed `Any` to dodge the `inference→kb` cycle; the module already uses `TYPE_CHECKING` imports — annotate properly to restore checker coverage on the solver's most-touched fields. |

## RTC — render / trace / cli / ir

| id | pri | category | location | finding (✅ = author-verified) |
|---|---|---|---|---|
| **F-RTC-1** | **High** | architecture / duplication | `render/dot_util.py:35` vs `kb/render.py:66`, `render/slice.py:71`, `render/lattice_dag.py:168`, `ir/to_dot.py:91`, `provenance.py:197` | ✅ `dot_util.py` exists to be the one DOT helper home but only `quote`/`value_label`/`fact_label`/shapes moved. **Line construction is re-rolled everywhere**: escape body duplicated 5× byte-identically (`kb/render._q` ≡ `dot_util.quote`), `_multiline` 2×, md5 node-id 3×, the `rankdir=LR; node[fontname=…]` preamble ~6×, and **two parallel emit idioms** (`ir/to_dot._Builder` class vs everyone's `lines:list[str]`+join). Promote a real `node()/edge()/cluster()/esc()/node_id()/multiline()/digraph()` API and route all six renderers through it. Highest-leverage RTC change. |
| **F-RTC-2** | High | long-function / duplication | `cli.py:270-423` + `:41-73,92,189,222` | `_build_parser` (154 ln) assembles every subparser inline (the `rule_mode_opts` dict at `:354` shows the smell was half-recognised); `_cmd_ir_*` repeat the parse-or-bail prelude (a `_parse_or_exit` helper *exists* but 3 sites predate it), and the KB-load-or-bail block is copy-pasted verbatim 3×. Split into `_add_*_parser` + add `_load_kb_or_exit`. |
| **F-RTC-4** | Med | long-function / overcomplication | `trace/ast.py:120-169` | ✅ `parse_trace_steps` — **nesting depth 9, the deepest in the codebase** (the `:bind` arm is its own parser nested 6 deep). Extract `_parse_bindings`/`_parse_using` + a key-dispatch; the recurring `Atom→name / scalar→value / else str` ternary (`:161-165`, also in `_sform_to_factref`) → `_atom_or_value`. |
| **F-RTC-3** | Med | architecture / duplication | `ir/to_dot.py:308-405` vs `trace/ast.py:143` | Two trace pipelines re-parse the same `(trace …)` SForm: `to_dot.render_trace` hand-scans `:using`/`:derives`/`:rule` (`:342-405`) while `parse_trace_steps` already produces `TraceStep`s. Both consumers are live (the `ir dot` path vs the markdown solve path) — but the SForm→fields extraction should be shared: have `render_trace` consume `parse_trace_steps`, deleting `_trace_premises` + the inline KwPair loops. |
| **F-RTC-5** | Med | long-function / duplication | `trace/linearize.py:118-213` | `linearize` (96 ln) dispatches on `(verdict-type × proof?)` with five near-identical hand-built `Trace(...)` and the `solution_dot/full_kb_dot` guarded-render lines spelled 3×. Convert to a `{VerdictType: _from_*}` table + a `_trace_diagrams(kb,…)` helper. |
| **F-RTC-6** | Med | long-function | `kb/render.py:165` (= F-KB-10) | duplicate of F-KB-10, listed here as the render-side view. |
| **F-RTC-7** | Med | readability | `ir/to_dot.py:122-130,200-216` | ✅ `_atom_arg_attrs` ends with an **unreachable** trailing `return f"shape={GROUND_SHAPE}"` (`:130`, after the `isinstance(Atom)` arm covers the last case); `_atom_id_for_value` has identical `Range`/`SForm` arms. Collapse to a typed dispatch. |
| **F-RTC-8** | Low | readability | `cli.py:105` | ✅ `",".join(sorted(set(_LAYER_BY_NAME.values()) and _LAYER_BY_NAME))` — relies on `and` short-circuit to return the dict; reads like a bug. It is just `sorted(_LAYER_BY_NAME)`. |
| **F-RTC-9** | Low | best-practice | `trace/answer.py:19,44` | `_nation_at` indexes private `kb._facts_by_relation`; imports underscore-private `_query_value` from `verdict`. Add a public `kb.facts_for(rel)` and promote `query_value`. |
| **F-RTC-10** | Low | duplication | `ir/dump.py:50`, `trace/ast.py:89` | S-expression string-escaper (`\\ \" \n \t \r`) duplicated, and `step_to_ir:89` only escapes `\\`+`\"` — a `why` with a newline round-trips inconsistently vs the canonical dumper. Share `escape_string_literal` (distinct from the DOT `esc` of F-RTC-1). |

### Non-findings (looked dead, verified **live** — do not delete)

✅ all grep-confirmed across src/tests/demo/acceptance:
- No dead tree-search renderer in `render/` — only docstrings noting the
  lattice *replaced* it.
- `parse_trace_steps`/`trace_to_ir`/`step_to_ir` — round-trip tested
  (`tests/trace/test_render.py:147`).
- `render_state`/`render_solution`/`render_slice` — live from `linearize`.
- `_render_trace_dag` / `ir dot --trace-view dag` — tested.
- trace-event AST handlers (`branch_open`/`symmetry_decl`/…) — real grammar
  productions, parser-tested.
- `colour_by="layer"` — reachable via `--colour-by`, tested.

---

## Metrics (baseline 2026-06-01, to re-measure at S1.7b.6)

- LOC (package): **13,337** across 51 files.
- Functions > 100 lines: **9** (`_explore_layers` 621, `rebuild_indexes`
  164, `_build_parser` 154, `_compile_premise` 139, `to_dot` 130,
  `back_propagate` 116, `proof_summary` 115, `validate_proof_for_explanation`
  109, `render_slice` 102).
- Functions nesting depth ≥ 5: **10** (worst: `parse_trace_steps` 9,
  `from_ir.load` 8, `_explore_layers` 7).
- Verified-dead functions: **7** (F-KER-9) + **1** unused hook (F-ENG-10).
- Stale `monotonic_solve` references in src: **9**; `tree`-engine doc refs: 2
  files.
- ruff: **clean** (this debt is invisible to the linter — the point of the
  phase).
- tests: **604** across 49 files; `run_tests.sh` green under PyPy.
