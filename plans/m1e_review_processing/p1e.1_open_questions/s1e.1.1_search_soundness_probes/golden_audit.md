# The golden audit — what pins `lattice/02` and the `branching/06`+`07` pair

[T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
step 4's product, taken 2026-08-28 against `a3f4e7b`. The milestone's rule is
that *a re-bless nobody predicted is a stop; a re-bless named in advance in a
stage file is a step* — so this is the list, and the counts are measured rather
than recalled.

**Counting convention.** For a golden, the number is **rows that name the
file**. For a Rust test, it is the number of **`#[test]` functions** that name
it, with the raw line count beside it, because a test may name a fixture on
several lines inside one function and only the function can fail.

## `examples/lattice/02_genuine_3set_death.ein`

The subject of Q5's ruling, and the entry whose `-L` cell states a falsehood.

| artefact | rows / tests | what it holds |
|---|---|---|
| `ein-cli/tests/golden/corpus_exits.txt` | **16 rows** | one per declared run, every one exit `0` — including `solve -L`, which is the false verdict |
| `ein-render/tests/golden/corpus_shapes.md5` | **45 rows** | the per-run KB-shape digests |
| `ein-infer/tests/lattice_semantics.rs` | **4 tests** (6 lines) | `a_layer_that_yields_complete_models_stops_descending`, `every_solution_record_is_independently_a_solution_node`, `the_fail_fast_fork_is_verdict_and_proof_neutral`, `the_sanity_check_passes_on_every_monotone_fixture` |
| `ein-infer/tests/search_invariants.rs` | **2 tests** | `coalescing_at_the_barrier_collapses_roots_layer_stack`, and one helper `run(…)` the tests share |
| `ein-cli/tests/model_set_report.rs` | **2 lines**, 3 tests in the file | the `--models key` surface |
| `ein-render/tests/presentation_semantics.rs` | **1 test** | `an_unexhausted_ambiguity_says_the_count_is_a_lower_bound` |
| `ein-cli/tests/config_reference.rs` | **1 line** | the configuration page's own example |
| `corpus/corpus.toml` | 1 entry | line 229 |

**Two corrections to the list the stage was working from.**

- It said *`corpus_exits.txt` (7 + 12 rows)*. The truth is **16 rows for
  `lattice/02`** and **7 + 7** for the branching pair — the 7/12 split was the
  branching entries' run counts read as the lattice one's.
- It named **`leftover_probe.rs`**, which does **not** mention `lattice/02` at
  all. It mentions `branching/06`, twice. And it missed
  **`config_reference.rs`**, which does.

## `examples/branching/06_lookahead_on.ein` and `07_lookahead_off.ein`

Evidence rather than subject since the reconnaissance (neither side exhausts),
but they are pinned in **more** places than the subject is — including two that
no bless can reach.

| artefact | `06` | `07` |
|---|---:|---:|
| `ein-cli/tests/golden/corpus_exits.txt` | 7 rows | 7 rows |
| `ein-render/tests/golden/corpus_shapes.md5` | 45 rows | 45 rows |
| `ein-infer/tests/search_invariants.rs` | 3 | 7 |
| `ein-cli/tests/model_set_report.rs` | 2 | — |
| `ein-cli/tests/leftover_probe.rs` | 2 | — |
| `ein-infer/tests/layer_census.rs` | — | 2 |
| `ein-infer/tests/worker_view.rs` | — | 1 |
| `ein-cli/tests/cli_semantics.rs` | — | 1 |
| `ein-corpus/src/manifest.rs` | — | 1 |
| `ein-infer/examples/{defer,flatten,shared_state}_probe.rs` | 1 each | 1 each |
| **`ein-infer/src/hypgen.rs`** | **1** | — |
| **`ein-render/src/models.rs`** | **1** | — |

**The last two rows are the finding.** They are **source comments** citing
these fixtures as evidence for a measured claim — the sort of citation
`EIN_BLESS=1` cannot touch and no golden diff will flag. If Q-M1e.8's fix moves
what `branching/06` answers, those two sentences become false silently. Any
audit that counts only goldens misses them, which is why this one greps `src/`
too.

## What the fix would move

Under [Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)'s
ruling the Q-M1e.8 fix is **(ii) re-saturate and re-check before recording**,
and separately [D3](d3_q_m1e8_file_or_take.md)'s maximality fix would record a
surviving commitment whose every superset died. Predicted, before it moves:

| | moves? | why |
|---|---|---|
| `corpus_exits.txt` | **only if an exit code changes** | `lattice/02 :: solve -L` goes from `Contradiction` to three solutions — both exit `0`, so the row does **not** move. A row moves only where a verdict crosses the `:expect`/exit boundary |
| `corpus_shapes.md5` | **no** | the KB shape is unchanged; the digest is over the loaded/saturated KB, not the verdict |
| `lattice_semantics.rs` | **yes**, at least `every_solution_record_is_independently_a_solution_node` | it asserts a property of the recorded set, and the recorded set is what changes |
| `presentation_semantics.rs` | **possibly** | it pins the *lower bound* wording, which the fix does not touch — but the `k` it reads may |
| `search_invariants.rs`, `model_set_report.rs`, `leftover_probe.rs`, `layer_census.rs` | **counter-sensitive** | they compare runs to each other rather than to a constant, so they move only if the fix changes enterings, which (ii) does not and the maximality fix does not either |
| the two `src/` comments | **yes, and nothing will tell you** | see above |

**No golden in this list moves for `EIN_BLESS=1` reasons alone.** What moves is
a handful of assertions about the recorded model set, and the honest statement
of the audit is that the fix is *cheaper to land than the stage assumed* — the
two big goldens (`corpus_exits.txt`, `corpus_shapes.md5`) hold, because neither
an exit code nor a KB shape is what the fix changes.
