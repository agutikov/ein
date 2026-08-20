# S1a.10.2 — the Python suite, file by file

**Stage:** [S1a.10.2](s1a.10.2_port_the_suite.md) · **Phase:** [P1a.10](README.md) · **Written:** 2026-08-20

The stage's first acceptance item: *every Python test file has a disposition,
and the dispositions are written down per file, not per suite.* This is that
table, produced by reading all 1,538 tests in 10 directories rather than by
reading their names.

---

## The numbers

| | |
|---|---:|
| Python test files | **89** (plus 15 ein.rs files that are themselves differential) |
| pytest tests behind them | **1,538** |
| distinct **behaviours** they assert | **353** |
| — already owned by a named ein.rs test | 78 |
| — needing a new Rust test | **275** (266 to write, 9 already written in this stage's acceptance port) |
| named Python-only subjects that die | **96** |

**1,538 tests → 353 behaviours** is the whole point of the exercise, and it is
not a coverage loss. Three things collapse: parametrisation (`test_shuffle_invariance.py` alone is 303 tests over 2 behaviours),
duplication across M1's phases (the same claim asserted in `test_compile.py`
for the raise and again in `test_compile_negative.py` for the message), and
tests whose subject is the Python implementation rather than the language.

## How to read a row

| disposition | files | meaning |
|---|---:|---|
| **port** | 31 | every claim in the file is semantics with no Rust owner |
| **split** | 46 | genuinely mixed: some claims port, some die with their subject |
| **already-covered** | 8 | an ein.rs test already asserts all of it, without the oracle — the test is named, and [§ The eight claims](#the-eight-already-covered-claims-checked) checks each one |
| **delete-with-subject** | 4 | the file's whole subject is the Python implementation |

A row's **behaviours** column is how many falsifiable claims survive it. Zero
in a `port` row would be a contradiction; zero in a `delete-with-subject` row
is the point.

---


## `inference` — 41 files, 456 tests, 193 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_absent_semantics.py` | 9 | port | 8 | `ein-infer/tests/naf_semantics.rs` |
| `test_algebra.py` | 9 | port | 7 | `ein-infer/tests/algebra_semantics.rs` |
| `test_apriori.py` | 17 | port | 8 | `ein-infer/tests/search_semantics.rs` |
| `test_closed.py` | 5 | already-covered | 0 | `ein-infer/src/closed.rs::a_second_pass_closes_nothing_new`; `ein-infer/src/compile.rs::a_negated_assert_names_the_relation_it_negates`; `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest` |
| `test_commitment.py` | 8 | port | 8 | `ein-infer/tests/search_semantics.rs` |
| `test_compile.py` | 12 | port | 7 | `ein-infer/tests/compile_semantics.rs` |
| `test_compile_negative.py` | 6 | port | 6 | `ein-infer/tests/compile_semantics.rs` |
| `test_config.py` | 11 | split | 3 | `ein-infer/tests/compile_semantics.rs` |
| `test_contradiction.py` | 22 | split | 6 | `ein-infer/tests/search_semantics.rs` |
| `test_converse_typecheck.py` | 7 | port | 5 | `ein-infer/tests/algebra_semantics.rs` |
| `test_demos.py` | 73 | port | 4 | `ein-infer/tests/search_semantics.rs` |
| `test_dies_immediately.py` | 8 | port | 3 | `ein-infer/tests/search_semantics.rs` |
| `test_engine.py` | 4 | already-covered | 0 | `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-infer/src/firing.rs::a_multi_assert_shares_one_provenance`; `ein-infer/src/compile.rs::a_negated_assert_names_the_relation_it_negates` |
| `test_explain.py` | 19 | port | 7 | `ein-infer/tests/explain_semantics.rs` |
| `test_forall.py` | 6 | port | 3 | `ein-infer/tests/rule_semantics.rs` |
| `test_frontier.py` | 6 | port | 4 | `ein-infer/tests/search_semantics.rs` |
| `test_guided_hypgen.py` | 16 | split | 2 | `ein-infer/tests/search_semantics.rs` |
| `test_infer_closure.py` | 8 | split | 3 | `ein-infer/tests/explain_semantics.rs` |
| `test_match.py` | 10 | split | 3 | `ein-infer/tests/rule_semantics.rs` |
| `test_mixed_type_hypothesis.py` | 3 | delete-with-subject | 0 | `ein.inference.apriori.layer_1`'s `sorted(alive)` raising `TypeError` on incomparable candidate tuples — divergence D2 / |
| `test_multi_assert.py` | 6 | split | 2 | `ein-infer/tests/rule_semantics.rs` |
| `test_naf_deps.py` | 12 | split | 2 | `ein-infer/tests/naf_semantics.rs` |
| `test_open.py` | 5 | split | 1 | `ein-infer/tests/naf_semantics.rs` |
| `test_predicates.py` | 8 | split | 2 | `ein-infer/tests/rule_semantics.rs` |
| `test_reflective_rule.py` | 4 | split | 1 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_relation_algebra.py` | 26 | port | 22 | `ein-infer/tests/algebra_semantics.rs` |
| `test_relation_arity.py` | 16 | split | 10 | `ein-infer/tests/compile_semantics.rs` |
| `test_rule_library.py` | 3 | split | 2 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_rules.py` | 19 | split | 9 | `ein-infer/tests/rule_semantics.rs` |
| `test_saturator.py` | 15 | split | 8 | `ein-infer/tests/explain_semantics.rs` |
| `test_saturator_fork_parity.py` | 7 | split | 6 | `ein-infer/tests/explain_semantics.rs` |
| `test_saturator_naf.py` | 5 | split | 4 | `ein-infer/tests/naf_semantics.rs` |
| `test_solution.py` | 4 | split | 3 | `ein-infer/tests/search_semantics.rs` |
| `test_state_key.py` | 6 | split | 2 | `ein-infer/tests/search_semantics.rs` |
| `test_stdlib_bijection.py` | 10 | port | 9 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_stdlib_domain_elim.py` | 7 | port | 5 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_symmetric_hypothesis.py` | 2 | port | 2 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_symmetric_native.py` | 8 | split | 3 | `ein-infer/tests/stdlib_semantics.rs` |
| `test_typed_blind_solve.py` | 3 | port | 2 | `ein-infer/tests/search_semantics.rs` |
| `test_why.py` | 8 | split | 1 | `ein-infer/tests/explain_semantics.rs` |
| `test_world_boundary.py` | 23 | split | 10 | `ein-infer/tests/naf_semantics.rs` |

## `inference/lattice` — 10 files, 370 tests, 35 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_contradictions_backbone.py` | 8 | split | 6 | `ein-infer/tests/lattice_semantics.rs` |
| `test_gaps_backbone.py` | 7 | split | 4 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_dumper.py` | 9 | split | 3 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_fixtures.py` | 9 | split | 4 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_proof.py` | 8 | split | 3 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_sanity.py` | 7 | port | 5 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_scoring.py` | 8 | split | 3 | `ein-infer/tests/lattice_semantics.rs` |
| `test_lattice_skeleton.py` | 4 | split | 1 | `ein-infer/tests/lattice_semantics.rs` |
| `test_p16_contract.py` | 7 | split | 4 | `ein-infer/tests/lattice_semantics.rs` |
| `test_shuffle_invariance.py` | 303 | split | 2 | `ein-infer/tests/lattice_semantics.rs` |

## `inference/monotonic` — 4 files, 24 tests, 23 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_monotonic_budget.py` | 6 | split | 5 | `ein-infer/tests/monotonic_semantics.rs` |
| `test_monotonic_cdcl.py` | 6 | split | 6 | `ein-infer/tests/monotonic_semantics.rs` |
| `test_monotonic_dumper.py` | 8 | port | 8 | `ein-infer/tests/monotonic_semantics.rs` |
| `test_root_stability_naf.py` | 4 | port | 4 | `ein-infer/tests/naf_semantics.rs` |

## `kb` — 10 files, 236 tests, 23 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `conftest.py` | 0 | delete-with-subject | 0 | the `zebra_kb` / `zebra2_kb` session fixtures (pytest fixture plumbing, not a claim about the engine) |
| `test_entities.py` | 21 | split | 2 | `ein-ir/tests/kb_semantics.rs` |
| `test_imports.py` | 35 | split | 7 | `ein-ir/tests/kb_semantics.rs` |
| `test_layers.py` | 14 | split | 1 | `ein-ir/tests/kb_semantics.rs` |
| `test_load_negative.py` | 64 | split | 1 | `ein-ir/tests/kb_semantics.rs` |
| `test_provenance.py` | 31 | split | 3 | `ein-ir/tests/kb_semantics.rs` |
| `test_render.py` | 22 | already-covered | 0 | `ein-render/tests/golden_dot.rs::the_zebra_unified_golden_reproduces`; `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-render/tests/dot_wellformed.rs::graphviz_accepts_every_view_of_every_corpus_file` |
| `test_stdlib_resolution.py` | 7 | split | 3 | `ein-ir/tests/kb_semantics.rs` |
| `test_store.py` | 39 | split | 6 | `ein-ir/tests/kb_semantics.rs` |
| `test_store_indexing.py` | 3 | already-covered | 0 | `ein-core/src/kb.rs::a_re_derived_fact_is_indexed_once`; `ein-core/src/kb.rs::incremental_indexing_and_a_rebuild_agree` |

## `ir` — 1 files, 21 tests, 4 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_macros.py` | 21 | split | 4 | `ein-ir/tests/ir_semantics.rs` |

## `render` — 4 files, 63 tests, 7 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_golden_dot.py` | 15 | already-covered | 0 | `ein-render/tests/golden_dot.rs::the_per_form_ir_goldens_reproduce`; `ein-render/tests/golden_dot.rs::the_query_and_trace_goldens_reproduce`; `ein-render/tests/golden_dot.rs::the_rule_and_constraint_goldens_reproduce` |
| `test_lattice_dag.py` | 14 | split | 4 | `ein-render/tests/presentation_semantics.rs` |
| `test_rules_dot.py` | 21 | split | 2 | `ein-render/tests/presentation_semantics.rs` |
| `test_slice_dot.py` | 13 | port | 1 | `ein-render/tests/presentation_semantics.rs` |

## `trace` — 3 files, 28 tests, 5 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_answer.py` | 8 | port | 3 | `ein-render/tests/presentation_semantics.rs` |
| `test_idea08_acceptance.py` | 4 | already-covered | 0 | `ein-render/tests/idea08_acceptance.rs::the_zebra2_library_defines_the_walkthrough_rules`; `ein-render/tests/idea08_acceptance.rs::the_generic_library_defines_the_walkthrough_rules`; `ein-render/tests/idea08_acceptance.rs::the_zebra2_trace_exhibits_the_walkthrough_rules` |
| `test_render.py` | 16 | split | 2 | `ein-render/tests/presentation_semantics.rs` |

## `integration` — 1 files, 11 tests, 4 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_zebra_parse.py` | 11 | split | 4 | `ein-cli/tests/cli_semantics.rs` |

## `tests/ (top level)` — 11 files, 308 tests, 50 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `conftest.py` | 0 | delete-with-subject | 0 | ein.py/tests/conftest.py — the pytest conftest module (no fixtures defined) |
| `load_negative.py` | 0 | delete-with-subject | 0 | the `tests.load_negative` helper module: `load_error`, `fixtures`, and the `encode` direction of the placeholder codec ( |
| `test_cli.py` | 13 | split | 3 | `ein-cli/tests/cli_semantics.rs` |
| `test_corpus_manifest.py` | 9 | already-covered | 0 | `ein-conformance/src/corpus.rs::every_ein_file_has_an_entry`; `ein-conformance/src/corpus.rs::every_entry_names_a_real_file`; `ein-conformance/src/corpus.rs::paths_are_unique` |
| `test_events.py` | 13 | port | 11 | `ein-cli/tests/cli_semantics.rs` |
| `test_examples_load.py` | 72 | already-covered | 0 | `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-conformance/src/corpus.rs::every_ein_file_has_an_entry` |
| `test_ir_ast.py` | 78 | port | 4 | `ein-ir/tests/ir_semantics.rs` |
| `test_ir_parser.py` | 76 | port | 11 | `ein-ir/tests/ir_semantics.rs` |
| `test_ir_to_dot.py` | 24 | port | 5 | `ein-render/tests/presentation_semantics.rs` |
| `test_solve_cli.py` | 18 | split | 12 | `ein-cli/tests/cli_semantics.rs` |
| `test_vscode_grammar.py` | 5 | port | 4 | `ein-cli/tests/cli_semantics.rs` |

## `acceptance/` — 4 files, 21 tests, 9 behaviours

| file | tests | disposition | beh. | where it lands |
|---|---:|---|---:|---|
| `test_bench_solve_mode.py` | 2 | port | 2 | `ein-cli/tests/acceptance_cli.rs (done)` |
| `test_mode_consistency.py` | 9 | port | 2 | `ein-infer/tests/acceptance.rs (done)` |
| `test_zebra_three_classes.py` | 6 | port | 4 | `ein-infer/tests/acceptance.rs (done)` |
| `test_zebra_two_ontologies.py` | 4 | port | 1 | `ein-infer/tests/acceptance.rs (done)` |

---

## The eight "already covered" claims, checked

The stage says this disposition deserves suspicion, and it names the reason: a
claim of coverage without a named test is not a claim. Every one of the eight names a test, and every name resolves to a real `#[test]` in the
workspace — checked mechanically against all 327 of them, along with the 78 per-behaviour
`already_covered_by` claims: **293 of 314 coverage claims name a
real ein.rs test, and the other 21 name `corpus_shapes.md5`**, whose owner is
`ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`.
No claim named a test that does not exist.

| Python file | tests | owner |
|---|---:|---|
| `test_corpus_manifest.py` | 9 | `ein-conformance/src/corpus.rs::every_ein_file_has_an_entry`; `ein-conformance/src/corpus.rs::every_entry_names_a_real_file`; `ein-conformance/src/corpus.rs::paths_are_unique`; `ein-conformance/src/corpus.rs::groups_are_from_the_vocabulary`; … (1 more) |
| `test_examples_load.py` | 72 | `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-conformance/src/corpus.rs::every_ein_file_has_an_entry` |
| `inference/test_closed.py` | 5 | `ein-infer/src/closed.rs::a_second_pass_closes_nothing_new`; `ein-infer/src/compile.rs::a_negated_assert_names_the_relation_it_negates`; `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest` |
| `inference/test_engine.py` | 4 | `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-infer/src/firing.rs::a_multi_assert_shares_one_provenance`; `ein-infer/src/compile.rs::a_negated_assert_names_the_relation_it_negates`; … (1 more) |
| `render/test_golden_dot.py` | 15 | `ein.rs/crates/ein-render/tests/golden_dot.rs::the_per_form_ir_goldens_reproduce`; `ein.rs/crates/ein-render/tests/golden_dot.rs::the_query_and_trace_goldens_reproduce`; `ein.rs/crates/ein-render/tests/golden_dot.rs::the_rule_and_constraint_goldens_reproduce`; … (1 more) |
| `trace/test_idea08_acceptance.py` | 4 | `ein.rs/crates/ein-render/tests/idea08_acceptance.rs::the_zebra2_library_defines_the_walkthrough_rules`; `ein.rs/crates/ein-render/tests/idea08_acceptance.rs::the_generic_library_defines_the_walkthrough_rules`; … (1 more) |
| `kb/test_render.py` | 22 | `ein-render/tests/golden_dot.rs::the_zebra_unified_golden_reproduces`; `ein-render/tests/corpus_shapes.rs::every_corpus_rendering_reproduces_its_digest`; `ein-render/tests/dot_wellformed.rs::graphviz_accepts_every_view_of_every_corpus_file`; … (1 more) |
| `kb/test_store_indexing.py` | 3 | `ein-core/src/kb.rs::a_re_derived_fact_is_indexed_once`; `ein-core/src/kb.rs::incremental_indexing_and_a_rebuild_agree` |

### The one `delete-with-subject` inside `tests/inference/`

The stage flags this too, and there is exactly one:
**`tests/inference/test_mixed_type_hypothesis.py`** (3 tests). Its subject is
[D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers) —
`apriori.layer_1`'s `sorted(alive)` raising `TypeError: '<' not supported
between instances of 'int' and 'str'`. That is a CPython comparison rule, not a
claim about ein-lang, and the file's own docstring already says ein.rs answers
the input instead. The **fixture survives**: `examples/ein-bugs/mixed-type-hypothesis.ein`
stays a corpus entry and its answer is pinned by `corpus_shapes.md5`
(`::hyp`, `::hyp+closed`). What dies is the assertion that ein.py raises, and
the `crash-parity` group that existed to compare the raise.

---

## What dies with its subject

96 named subjects. The project's rule is *removing a special case removes its
tests*; at this scale the "special case" is an implementation, so each deletion
names what it tested rather than which test file went.

| from | subject |
|---|---|
| `test_zebra_three_classes.py` | the `ProgressDumper(label=…, progress_every=10, stream=sys.stderr)` wiring in the `_solve` helper, and with it `run_tests.sh`'s Phase 2 (`"${PY}" -m pytest -s -v acceptance/`) and its `--acceptance-on |
| `test_zebra_two_ontologies.py` | the `ProgressDumper` label/progress plumbing in this file's `_solve` helper — same subject as `test_zebra_three_classes.py`'s, listed there |
| `conftest.py` | ein.py/tests/conftest.py — the pytest conftest module (no fixtures defined) |
| `test_cli.py` | `ein.cli.main`'s argparse subcommand dispatcher — the `{render,solve,saturate}` help metavar and the `SystemExit` non-zero code for the removed `ir` / `kb` subcommands and the `profile` / `symmetric`  |
| `test_solve_cli.py` | `ein.cli.solve`'s argparse `--help` rendering — `test_help_lists_stop_policy_and_drops_modes` reads flag names out of formatted help text, and `test_removed_mode_flags_error` asserts argparse's `unrec |
| `load_negative.py` | the `tests.load_negative` helper module: `load_error`, `fixtures`, and the `encode` direction of the placeholder codec (ein.rs has no `encode`; blessing an `.expected` is manual there) |
| `test_zebra_parse.py` | The second copy of "an unbalanced paren is a parse error" — `test_malformed_zebra_variant_is_rejected` hand-corrupts `zebra2.ein` to make an input `examples/broken/unclosed_paren.ein` already is, and  |
| `test_config.py` | ein.inference.config._coerce's fallback branch for a dataclass field whose annotation string is outside {bool,int,float,str} — the 'unsupported type' ValueError, which exists only because `from __futu |
| `test_contradiction.py` | `ein.inference.saturator.Saturator.contradictions()` — a Python delegating helper; ein.rs has the free function `contradiction::detect(kb, terms)` and no method on the saturator (test_saturator_contra |
| `test_contradiction.py` | `ein.inference.contradiction.Contradiction`'s defaulted `kind="pair"` dataclass field — the claim is "call sites that omit the keyword still get `pair`"; ein.rs's `Kind` is a required enum field with  |
| `test_contradiction.py` | `KnowledgeBase._index_fact` / `_fact_by_id` as the way a test writes a fact — the `_put` helper this file is built on |
| `test_solution.py` | `ein.inference.solution`'s module-level `generate_hypotheses` binding — the monkeypatch seam `test_complete_short_circuits_on_the_first_candidate` rebinds to count generator pulls. ein.rs's `hypgen::g |
| `test_state_key.py` | `ein.inference.canon.state_digest`-as-identity and the `_Collide` tuple wrapper that forces every state into one dict bucket. ein.rs's dedup is `FxHashMap<Box<[FactId]>, usize>`, where a collision is  |
| `test_state_key.py` | `ein.inference.monotonic._helpers.state_key`'s module-attribute indirection — the seam the test patches. In ein.rs `record_node` calls `crate::canon::state_key` directly. |
| `test_symmetric_native.py` | `ein.inference.saturator.SYMMETRIC`'s value asserted as a constant (`test_symmetric_constant`). ein.rs has the same `pub const SYMMETRIC: &str = "__symmetric__"` (`saturator.rs:50`), but the name is l |
| `test_symmetric_native.py` | The `subprocess` + `PYTHONHASHSEED` driver (`_DRIVER`, `test_multi_marker_mirror_order_survives_hash_seeds`). ein.rs has no salted hash to perturb; the successor instrument permutes the id space inste |
| `test_why.py` | `ein.inference.why.render_why`'s `str()` coercion of a non-string binding value (`test_integer_binding_str_converted`). ein.rs's signature is `render_why(template: &str, bindings: &[(String, String)]) |
| `test_why.py` | `test_empty_template_returns_empty` and `test_no_refs_pass_through` as separate cases — the Rust scanner copies every non-reference character and `a_positional_slot_is_a_reference_and_a_bare_brace_is_ |
| `test_world_boundary.py` | `ein.inference.world.World` — the boundary-world dataclass and its `commitment` field (`test_world_carries_its_commitment`). ein.rs has no `World` type at all (the only `struct World` in the workspace |
| `test_world_boundary.py` | `ein.inference.world.project(env, scope)` — the dict-restriction helper (`test_project_restricts_to_scope`). Its observable, the scope projection, is `NafGuard.scope_of` in ein.rs and is asserted twic |
| `test_world_boundary.py` | `ein.inference.compile.split_naf`'s tuple-returning signature — `split_naf(steps) -> (positive, guards)` and the identity on a guard-free plan (`test_split_naf_on_a_plan_with_no_guards_is_identity`).  |
| `test_world_boundary.py` | The `inspect.getsource` scan of `ein.inference.predicates` (`test_watch_stamp_is_blind_to_predicate_guards`'s second half, which greps each registered callable's source for `kb` / `_facts_by`). ein.rs |
| `test_world_boundary.py` | `Saturator._queue` / `_parked` / `_enqueue_pass` as directly-inspected attributes (`test_guarded_candidates_are_parked_not_queued`'s first two assertions). ein.rs's equivalents are private fields of a |
| `test_rule_library.py` | `Rule.params` — the list whose length `test_sibling_exclusive_is_two_param` counts. ein.rs's equivalent shape is pinned instead by the `plan` and `dot[rules]` digests, which render the substituted par |
| `test_rules.py` | `compile_rule(...).extra_match_plans` / `.steps` / `.assert_templates` — the Python plan record `test_or_in_match_keeps_one_rule_with_multiple_plans` reaches into. ein.rs has no `extra_match_plans`; d |
| `test_rules.py` | The `_add_synthetic_hyp` helper — hand-built `Fact(...)` + `Provenance.from_hypothesis(branch=1)` + `kb.add_fact` + `kb.rebuild_indexes()`. Those are ein.py constructors; the equivalent state is reach |
| `test_saturator_fork_parity.py` | `ein.inference.world.World`'s `commitment` parameter — a deliberately inert constructor argument, pinned by monkeypatching `World.__init__`. ein.rs has no `World` type in `ein-infer` at all (the `Worl |
| `test_saturator_fork_parity.py` | `Saturator._parked` / `_park_stamp` and the `sat.naf_rounds = 999` attribute pokes in `test_boundary_state_does_not_leak_into_state_key`. In ein.rs `canon::state_key` returns `Box<[FactId]>` built fro |
| `test_saturator.py` | `Saturator.__init__`'s optional `engine=` parameter and the `Engine.cache` dict it shares — `test_saturator_constructs_without_engine` and `test_saturator_accepts_existing_engine` assert `isinstance(s |
| `test_mixed_type_hypothesis.py` | `ein.inference.apriori.layer_1`'s `sorted(alive)` raising `TypeError` on incomparable candidate tuples — divergence D2 / hazard H2, a CPython comparison rule that ein.rs does not have |
| `test_mixed_type_hypothesis.py` | the `crash-parity` claim that this fixture is a crash at all: it is an ordinary corpus entry under ein.rs, pinned by `mixed-type-hypothesis.ein::{hyp,hyp+closed}` |
| `test_predicates.py` | `ein.inference.predicates._REGISTRY` and its `register(name, fn)` hook — a mutable module-global registry extended at runtime; ein.rs's registry is the closed `enum Pred` with `names() -> [&'static st |
| `test_macros.py` | `KnowledgeBase.macros` as a Python mapping — `set(kb.macros) == {"co"}` and `kb.macros["co"].params == ("a", "b")` assert the shape of a dict of `ein.ir.macros.Macro` dataclasses. The registry itself  |
| `test_contradictions_backbone.py` | `_record_setnode` (ein.py/src/ein/inference/monotonic/solver.py) and its `entry="contradictions"` state-key MERGE path |
| `test_contradictions_backbone.py` | `SetNode` / `_LatticeLoopState.kb_index` / `LatticeStats.state_key_merges` as a *mechanism* — ein.rs hard-wires `kb_index` empty and `state_key_merges: 0` (crates/ein-render/src/dump/lattice.rs:285,31 |
| `test_contradictions_backbone.py` | the `(verdict, stats)` Python return-tuple shape and the `isinstance(..., MonotonicStats/LatticeProof)` dataclass checks |
| `test_gaps_backbone.py` | the `(verdict, stats)` tuple shape and `isinstance(stats, MonotonicStats)` / `isinstance(verdict.proof.stats, LatticeStats)` — Python return-type wiring |
| `test_lattice_dumper.py` | `LatticeDumper.root_saturating`'s existence as a duck-typed attribute — `test_dumper_survives_a_long_root_saturation` is a regression test for an `AttributeError` on a Python object missing a method t |
| `test_lattice_proof.py` | `_record_setnode`'s `entry="gaps"` per-commitment keying and `entry="contradictions"` merge (same subject as test_contradictions_backbone.py) |
| `test_lattice_proof.py` | `SolutionRecord.kb.facts is not kb.facts` — a Python identity check on list sharing between a snapshot and root |
| `test_lattice_scoring.py` | `order_candidates(candidates, mode="score-sum")` raising `ValueError("requires kb")` when `kb=None` — a Python optional-argument guard; the Rust signature is `order_candidates(kb: &Kb, terms: &Terms,  |
| `test_lattice_scoring.py` | `test_lattice_order_deterministic_under_same_mode` — two runs of one deterministic engine; subsumed by `ein-render/tests/id_order_invariance.rs::no_observable_depends_on_the_order_ids_were_assigned_in |
| `test_lattice_skeleton.py` | `ein.inference.monotonic.__init__`'s re-export list (`solve`, `LatticeProof`, `SolutionRecord`, `DeadCommitment`, `SetNode`, `LatticeStats`, `LatticeDumper`) — a Python module-layout assertion |
| `test_lattice_skeleton.py` | `LatticeDumper`'s duck-typed hook signatures (`root_initial` / `layer_start` / `entering(..., outcome=, facts_merged=, nogood_emitted=, nogood_subsumed=)` / `layer_end` / `proof_summary` / `summary` / |
| `test_p16_contract.py` | `validate_proof_for_explanation`'s clause 1 (`verdict.proof is proof`) and its `AssertionError` message — a Python parameter-swap guard for a function that takes the proof and the verdict separately;  |
| `test_p16_contract.py` | clause 5's `SetNode` invariants (`canonical_set in labels`, multilabel only under `Contradiction`) — same subject as `_record_setnode`, which ein.rs does not have |
| `test_shuffle_invariance.py` | `lattice_snapshot(verdict, kb)` raising `ValueError` when `verdict.proof` is None — a Python guard on an `Optional` field; ein.rs's `lattice_snapshot(&solved.answer, proof: &LatticeProof, kb, terms)`  |
| `test_shuffle_invariance.py` | `LatticeSnapshotV1`'s frozen-dataclass hashability (`assert isinstance(hash(snap), int)`) — a Python dataclass property asserted so a test harness could pool snapshots in a `set` |
| `test_shuffle_invariance.py` | `test_lattice_snapshot_default_seed_idempotent` — two default-seed runs of one deterministic engine; subsumed by `ein-render/tests/id_order_invariance.rs::no_observable_depends_on_the_order_ids_were_a |
| `test_lattice_dag.py` | `render_lattice(proof, view: str)`'s `ValueError("unknown lattice view …")` — ein.rs takes `LatticeView`, a two-variant enum whose only string entry point is `LatticeView::parse -> Option`, clap-valid |
| `test_lattice_dag.py` | `render_trace(form, view: str)`'s `ValueError("unknown trace view …")` — same, `TraceView::parse -> Option` (test_trace_view_invalid_raises) |
| `test_lattice_dag.py` | `render_lattice`'s polymorphic first argument (a `Verdict` *or* a `LatticeProof`, unwrapped by `getattr(v, 'proof', v)`) — ein.rs's `LatticeSource::{Proof,Snapshot}` makes the caller say which (test_r |
| `test_lattice_dag.py` | `LatticeProof.kb_index` — a Python attribute asserted to be `{}`; the observable it stood for (a `full` view degrades to the solution frontier plus a note) is `dot[lattice-full]` in the corpus manifes |
| `test_rules_dot.py` | `render_rule(form, mode: str)`'s `ValueError` on an unknown mode — ein.rs takes `RuleMode`, a two-variant enum, clap-validated as `--rule-mode {sidebyside,overlay}` (test_unknown_rule_mode_raises) |
| `test_render.py` | `render_markdown(trace, mode: str)`'s `ValueError("unknown trace mode …")` — ein.rs takes `Mode`, a two-variant enum (test_render_markdown_rejects_bad_mode) |
| `test_render.py` | `ein.trace.ast`'s *second* string escaper — `_fact_to_sexpr`'s private `\`/`"`-only escape, the S1.7c.32 bug's actual subject. ein.rs has one escaper: `trace/ast.rs` calls `ein_ir::dump::escape_string |
| `conftest.py` | the `zebra_kb` / `zebra2_kb` session fixtures (pytest fixture plumbing, not a claim about the engine) |
| `test_entities.py` | `Relation`/`Rule`/`Fact` as frozen dataclasses with field-derived `__eq__`/`__hash__`/`__repr__` (`test_relation_identity_by_name_and_signature`, `test_rule_identity_by_name_only`, `test_fact_identity |
| `test_entities.py` | `ein.kb.entities._attach` / `_detach` and the `_kb` field's `compare=False, hash=False, repr=False` exclusions (`test_attach_does_not_break_equality`, `test_attach_can_be_undone`, `test_attach_does_no |
| `test_entities.py` | the detached-entity accessors that answer `()`/`None` with no KB attached (`test_detached_relation_has_no_rules_or_facts`, `test_detached_rule_has_no_applications`, `test_detached_fact_args_pass_throu |
| `test_entities.py` | `Fact.arg_entities`' str/int passthrough returning a nested `Fact` as-is (`test_nested_fact_arg_entities_returns_fact_as_is`) — ein.rs args are `Value`s with a `Tag::Fact` variant, there is no entity- |
| `test_imports.py` | `compile_rule(...).naf_guards` / `ein.inference.compile.AbsentGuard` structural introspection (`((outer,),) = plan.naf_guards`, `any(isinstance(s, AbsentGuard) for s in outer.sub_steps)`) — ein.rs has |
| `test_imports.py` | `resolve_and_minimize`'s Python return shape — `{f.args[0].name for f in forms if f.head.name == "macro"}` walks `SForm`/`Atom` objects; the ein.rs equivalent is `dump_canonical` text, already digeste |
| `test_layers.py` | `FactView.__repr__` (`"all"`, `"len=71"`) — a Python repr; the fact count it encodes is re-listed as a behaviour under `test_store.py` |
| `test_layers.py` | `FactView.matching(pattern=…)` — a `NotImplementedError` stub left as the P1.3 seam; there is no such method in ein.rs and no caller ever reached it |
| `test_layers.py` | the shared-entity back-pointer caveat (`test_fork_entity_back_pointer_caveat`): a `Relation` entity shared with a fork answers `.facts` for the *root*. `ein-core/src/entities.rs`'s module note states  |
| `test_layers.py` | Python container object-identity assertions: `fork.facts is not kb.facts`, `fork._facts_by_relation is not kb._facts_by_relation`, `fork.names is not kb.names`, `fork.relations is kb.relations` — ein. |
| `test_load_negative.py` | the `UPDATE_GOLDEN=1` capture-and-skip path (`expected_path.write_text(encode(str(exc.value), path) + "\n")`) — a Python re-bless workflow with no ein.rs counterpart for `.expected` files |
| `test_load_negative.py` | `encode ∘ decode == id` on the five machine-specific fixtures (`test_placeholders_round_trip`) — `encode` exists only in `tests/load_negative.py`; ein.rs implements decode inline and has no encode dir |
| `test_provenance.py` | the `Provenance` frozen dataclass's derived `__eq__`/`__hash__` and its `loc: … field(compare=False, hash=False, repr=False)` exclusion (`test_provenance_equality`, `test_provenance_loc_excluded_from_ |
| `test_provenance.py` | `Fact.source` / `.rule_name` / `.using` / `.premises` back-compat properties (`TestFactProperties`, 4 tests) — `using`/`premises_raw` are `(relation, args)` tuples resolved to `Fact` objects by a scan |
| `test_provenance.py` | `KnowledgeBase._fact_by_id(rel, args)` — the Python extent-scan lookup helper; ein.rs interns to a `FactId` and probes |
| `test_provenance.py` | `DerivationDAG.__len__` / `__iter__` (`test_dag_iter_and_len`) — Python container protocol over `dag.nodes` |
| `test_stdlib_resolution.py` | `ein.kb.imports._cached_macro_names` (the `functools.lru_cache` on the resolved root) and the autouse `_clear_macro_cache` fixture — ein.rs's `Resolver::stdlib_macro_names` re-reads and re-parses `mac |
| `test_stdlib_resolution.py` | `test_a_packaged_copy_does_not_shadow_the_checkout` — the subject is the wheel's `ein/stdlib/` directory, which carries the marker verbatim and sits on the upward walk, and the explicit skip `_stdlib_ |
| `test_stdlib_resolution.py` | `test_the_checkout_is_the_source_of_truth`'s `len(list(SHARED.glob("*.ein"))) == 7` — the module count is owned by `the_embedded_copy_has_no_extra_modules`' set equality plus `assert!(checked >= 7)` |
| `test_store.py` | `KnowledgeBase.__repr__` (`"rules="`, `"facts=71"`) and `__len__` (relations + rules + facts) — Python object protocol |
| `test_store.py` | `Fact.arg_entities`' str/Relation resolution (`test_fact_arg_entities_resolution`) and `Relation.signature` as a Python tuple of names |
| `test_store.py` | `_copy_fact_indexes_into`'s object-identity contract (`copy._rules_by_relation is kb._rules_by_relation`, `copy._facts_by_relation is not kb._facts_by_relation`, `copy.names is not kb.names`, `copy._n |
| `test_store.py` | `snapshot()._nogoods` as a Python `set` of `frozenset`s (`assert snap._nogoods == set()`) — ein.rs's no-good store is an `RwLock`-shared structure whose fork/snapshot split is already asserted by `a_f |
| `test_store.py` | `KnowledgeBase._fact_by_id` and `kb.facts.clear()` / `rebuild_indexes()` as a test-only mutation route |
| `test_monotonic_budget.py` | `ein.inference.monotonic.BudgetExceededError` — the exception class and its re-export from the package root. ein.rs signals a budget cut with `SolveError::Budget { reason, stats }`, a value, not an ex |
| `test_monotonic_budget.py` | `Aborted.stats is stats` — CPython object identity between the verdict's stats and the returned stats. In ein.rs `Solved { answer: Answer::Aborted { reason }, stats }` holds one `MonotonicStats` by va |
| `test_monotonic_cdcl.py` | `emit_nogood`'s `min_size` **default of 2** and the tree-search caller it existed for. `test_emit_nogood_min_size_1_accepts_singleton`'s second half (`assert emit_nogood(kb2, one) is False` — "the def |
| `dot_parity.rs` | the `DIVERGENT` list — the assertion that ein.py raises where ein.rs answers (D2); ledger §8 says losing it "is the point of the phase" |
| `dot_parity.rs` | the `NARRATED_SLICES` list — a two-engine D3 claim, re-derived inside one engine by ein-render/tests/id_order_invariance.rs (ledger §5: 44 dying-fork + 22 derivation-body movements, "Nothing had to be |
| `trace_parity.rs` | the `DIVERGENT` list — the assertion that ein.py raises on the two ein-bugs fixtures (D2) |
| `trace_parity.rs` | the `narrated > 0` load-bearing check on the two-engine narration cut — re-owned by id_order_invariance.rs |
| `dump_parity.rs` | the `DIVERGENT` list (D2) |
| `dump_parity.rs` | the `narrated > 0` load-bearing check on the two-engine cut |
| `hypgen_parity.rs` | the `sweep()` harness and its `Oracle::start` / `divergent` / `Compare` machinery |
| `hypgen_parity.rs` | the `DIVERGENT` lists on `lattice`, `solve*` and `commit*` — the claim that ein.py's `sorted(alive)` raises on the two ein-bugs fixtures (D2); ein.rs's answers on both files are pinned by 43 manifest  |
| `hypgen_parity.rs` | `Compare::Narrated` and its `narrated > 0` load-bearing assertion — a two-engine D3 cut, superseded inside one engine by id_order_invariance.rs and by the manifest digesting the unnarrowed text |
| `dump_parity.rs` | ein.py's own `dump ∘ parse` fixed-point property (`dump_then_parse_is_a_fixed_point_in_ein_py_too`) — the test runs no ein.rs code; its subject is `ein/ir/dump.py` + `ein/ir/parser.py` |
| `fuzz_parity.rs` | the differential arm itself — "the parser accepts/rejects what `lark` does, with `lark`'s message" on generated input (ledger §6 L1); `utils/fuzz_ein.py`'s header must also stop advertising "four pari |
| `help_parity.rs` | ein.py's `argparse` parser surface — the eight `add_parser`/`add_arguments` trees in `ein/cli/`, including `_events.add_arguments` putting `--events` and `--events-level` on `saturate`. `utils/ir_orac |

---

## The 42 differential ein.rs tests

`cargo test --workspace` is P1a.10's stated gate and
[S1a.10.1](oracle_ledger.md#2-the-finding-46--of-einrss-own-integration-tests-are-differential)
found that 42 of its integration tests start a Python process. They are the
same kind of problem as the Python suite and get the same treatment.

| file | tests | disposition | per-test decisions |
|---|---:|---|---:|
| `ein-infer/tests/saturate_parity.rs` | 1 | port | 2 |
| `ein-ir/tests/load_parity.rs` | 4 | port | 5 |
| `ein-ir/tests/parse_parity.rs` | 3 | port | 3 |
| `ein-ir/tests/imports_parity.rs` | 7 | port | 5 |
| `ein-core/tests/values_parity.rs` | 3 | port | 3 |
| `ein-core/tests/cpython_parity.rs` | 3 | port | 3 |
| `ein-render/tests/dot_parity.rs` | 2 | port | 2 |
| `ein-render/tests/trace_parity.rs` | 2 | port | 2 |
| `ein-render/tests/dump_parity.rs` | 2 | port | 3 |
| `ein-infer/tests/compile_parity.rs` | 2 | port | 2 |
| `ein-infer/tests/hypgen_parity.rs` | 13 | split | 10 |
| `ein-infer/tests/match_parity.rs` | 1 | already-covered | 0 |
| `ein-ir/tests/dump_parity.rs` | 5 | split | 4 |
| `ein-ir/tests/fuzz_parity.rs` | 1 | port | 3 |
| `ein-cli/tests/help_parity.rs` | 3 | split | 2 |

---

## Where the behaviours land

One new test file per subject, and one agent wrote each — the partition is by
*target file* precisely so that thirteen concurrent ports could not collide.

| target | behaviours |
|---|---:|
| `ein-infer/tests/search_semantics.rs` | 37 |
| `ein-infer/tests/algebra_semantics.rs` | 30 |
| `ein-infer/tests/lattice_semantics.rs` | 29 |
| `ein-cli/tests/cli_semantics.rs` | 27 |
| `ein-infer/tests/naf_semantics.rs` | 24 |
| `ein-ir/tests/ir_semantics.rs` | 18 |
| `ein-infer/tests/explain_semantics.rs` | 18 |
| `ein-render/tests/presentation_semantics.rs` | 16 |
| `ein-infer/tests/stdlib_semantics.rs` | 16 |
| `ein-ir/tests/kb_semantics.rs` | 16 |
| `ein-infer/tests/compile_semantics.rs` | 14 |
| `ein-infer/tests/rule_semantics.rs` | 11 |
| `ein-infer/tests/monotonic_semantics.rs` | 10 |
| `ein-infer/tests/acceptance.rs (done)` | 7 |
| `ein-cli/tests/acceptance_cli.rs (done)` | 2 |
| **total** | **275** |

---

# What actually happened

*Written 2026-08-20, when the stage shipped. Everything above is the plan; this
is the outcome, including the four places the plan was wrong.*

## The numbers, after

| | before | after |
|---|---:|---:|
| `cargo test --workspace` | 312 tests, 9 m 13 s | **566 tests, 1 m 07 s** |
| — integration tests | 91, in 28 files | **341, in 41 files** |
| — of them differential | **42** | **0** |
| — unit tests | 221 | 225 |
| `corpus_shapes.md5` | 4 228 renderings | **5 209** |
| corpus entries | 111 | **129** |
| pytest | 1 538 | 1 538 (unchanged — it dies in S1a.10.5, not here) |

**The runtime is the headline and it is not an optimisation.** Nine of those
ten minutes were 42 tests starting a Python process per corpus file; the gate
did not get faster, it stopped paying for a second engine. The stage's
acceptance asked that the runtime "stay inside the gate's current budget" and
it is a ninth of it, so no test needed marking `slow` and the budget statement
in `run_tests.sh` is a smaller number rather than a longer list.

## The four places the plan above was wrong

1. **`lattice_semantics.rs` and `cli_semantics.rs` were not written.** The
   session that produced this document was interrupted; the first was a
   corpus probe left over from measuring, the second had its plumbing and
   three of its twenty-seven claims. Both are complete now — 29 and 26 tests.

2. **Six ported claims did not run.** They compiled and asserted nothing, or
   asserted something the engine does not do:
   `:using ((p a b))` is not the grammar (the headless list is
   [deferred syntax](../../../docs/kernel/ir/03-ein-lang/05_inspirations.md));
   `(hrule not …)` is a *parse* error, so only `absent` / `eq` / `false` /
   `relation` can reach the reserved-name guard; `zebra.ein` has no rule
   called `transitive`; a declaration's own companion facts are background;
   two `firings < 0` placeholders where the Python bounds are 100 and 400;
   a "two survivors" fixture that excluded two of three colours. A port that
   is not run is a port that has not been done.

3. **The 42 needed more than a disposition.** The table above assigns each a
   destination; six of them needed a *substitution* invented — a reference
   computed rather than fetched (`sorted()` is code-point order; `str(int(x))`
   is nine lines), a table frozen while the oracle agreed, or a corpus op that
   did not exist. `Op::Load` and `Op::Saturate` are the second kind: the two
   surfaces whose only owner was a differential sweep, added to
   `corpus_ops` and blessed with a live ein.py agreeing on 73 files and
   24 276 saturation events.

4. **`dot_parity`'s eighteen fixtures were worth more than the plan said.**
   Moved to `examples/syntax/` (seventeen; the eighteenth is a load-negative
   and sits with its siblings), they are digested under **every** op rather
   than the eight parse views the diff used — 768 renderings where the test
   compared 144. They were verified against ein.py on all 43 ops before
   blessing: 714 cells compared, 54 refused on both sides, 0 differences.

## What the corpus found on its first day

`examples/syntax/equality.ein` falsified a counter identity that had held for
440 cells. `summary_properties.rs` said *an unsat core is reported exactly for
a `Contradiction`*, and this file is a `Contradiction` with an empty core: two
`=` forms, no rule, no hypothesis that ever completes, and a depth cap that
cuts at layer 5 with every commitment still alive. `k = 0` with `exhausted
false` is "no model **within the cap**", which is not a refutation — nothing
died, so nothing is blamed. Restated as two implications, with a floor on the
cells that do report a core so neither holds vacuously.

That is the argument for moving fixtures into the corpus rather than keeping
them inline in a test, made by the corpus rather than by this document.

## Where the 42 went

| was | is | what took the bytes |
|---|---|---|
| `ein-infer/tests/saturate_parity.rs` | *gone* | `corpus_shapes.md5` `::saturate`, with its 50-file / 3 000-event floor |
| `ein-ir/tests/load_parity.rs` | `load_semantics.rs` | `::load`; plus `layering_holds_after_every_load` and two checked-in tables |
| `ein-ir/tests/parse_parity.rs` | `grammar_decisions.rs` | `ir[parse]`; plus four `.expected` files **written from ein.py** and a 78-row decision table |
| `ein-ir/tests/imports_parity.rs` | `imports_semantics.rs` | `ir[resolve|minimize|expand]`; plus a module-path table and a `stdlib/macro.ein` re-parse |
| `ein-core/tests/values_parity.rs` | `values_semantics.rs` | a reference computed in the test |
| `ein-core/tests/cpython_parity.rs` | `cpython_tables.rs` | three frozen tables — the stage's weakest substitution, and the file says so |
| `ein-render/tests/dot_parity.rs` | *gone* | `dot[*]`; the 18 fixtures became corpus entries |
| `ein-render/tests/trace_parity.rs` | `trace_roundtrip.rs` | `trace[*]` + `golden_trace.rs`; the IR round-trip survives |
| `ein-render/tests/dump_parity.rs` | `dump_shape.rs` | `dump[*]` + `golden_dump.rs`; the abort policy survives |
| `ein-infer/tests/compile_parity.rs` | *gone* | `::plan`; the four messages were already in `compile_semantics.rs` |
| `ein-infer/tests/hypgen_parity.rs` | `hypgen_coverage.rs` | eleven sweeps → the manifest; the two coverage floors survive |
| `ein-infer/tests/match_parity.rs` | *gone* | `::match` |
| `ein-ir/tests/dump_parity.rs` | `dump_goldens.rs` | `ir[parse]` / `ir[dump-compact]`; the two ein.py goldens survive |
| `ein-ir/tests/fuzz_parity.rs` | `fuzz_properties.rs` | accepted loss L1; three self-checkable properties survive |
| `ein-cli/tests/help_parity.rs` | `help_surface.rs` | retired — but replaced with a golden of `help_shape()` rather than left to a count |

Nothing outside `ein-oracle`'s own source now names `Oracle`, `IR_ORACLE` or
`PY_ORACLE`.

## What S1a.10.2 did that the ledger had filed under S1a.10.5

[§4](oracle_ledger.md#4-what-the-removal-must-relocate) is a defect list: five
ein.rs tests read files under `ein.py/` **without running Python**, so they
would have stayed green until the commit that deleted the tree and failed
there. The nineteen files are
`ein.rs/crates/{ein-ir,ein-render}/tests/golden/from_ein_py/` now, moved by
`git mv` and never regenerated, with a README that states the rule: *never
re-bless a file in that directory* — every other golden in the tree says
"ein.rs still renders what ein.rs rendered", and these are the only bytes in
the repo a second implementation produced. The four Python tests that read the
same files were re-pointed, so `./run_tests.sh` stayed green.

The one item of §4 that remains is `ein-conformance/src/corpus.rs::tracked`
scanning `ein.py/src/ein/stdlib` as a fallback stdlib location, which is a
code path rather than a file and belongs to the removal.
