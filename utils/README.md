# `utils/` — the scripts that are not the engine

**Eighteen scripts, 4 933 lines, all of them driving `ein.rs`.** Nothing here
is built, shipped or imported by the engine; everything here *runs* it, *reads*
it, or *renders* what it produced. If a check belongs in the gate it belongs in
`cargo test`, and several things that used to live here moved there —
[§ The census](#the-census) says which.

**Naming the binary.** Every script that runs the engine says which build:
`$EIN_BIN` or `--bin`, defaulting to `ein.rs/target/release/ein`. Three want a
build of their own — `fork_delta_verify.py` → `target-fd`, `spec_audit.py` →
`target-sa`, `profile_ein_rs.py` → `--profile profiling`, which it builds.
**None takes an `--impl`**: a flag with one value invites a reader to look for
the operand that is gone
([S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)).

---

## Renderers

Turning what the engine emits into something you can look at. `ein` writes DOT
and markdown to stdout; rasterising is a shell-script job, which is what these
are.

| | | needs |
|---|---|---|
| [`render_examples.sh`](render_examples.sh) | every `.ein` under `examples/` → per-rule DOT + the constraint-scope and commitment-lattice views, + SVG. 84 files, 248 rule DOTs, 79 lattices | `ein`, Graphviz |
| [`zebra2_trace.sh`](zebra2_trace.sh) | solve `zebra2.ein`, write its markdown derivation trace; `--svg` rasterises the inline `dot` blocks and rewrites the markdown to reference them | `ein`, Graphviz for `--svg` |
| [`render_knowledge_graph.sh`](render_knowledge_graph.sh) | `docs/lib/knowledge-graph.dot` → SVG/PNG, choosing among `dot` / `fdp` / `sfdp` / `osage` | Graphviz |
| [`render_knowledge_graph_cy.py`](render_knowledge_graph_cy.py) | the same graph as an interactive Cytoscape.js page (`docs/lib/knowledge-graph.cy/`) | — |
| [`package_vscode_ein.sh`](package_vscode_ein.sh) | [`vscode-ein/`](vscode-ein/) → an installable `.vsix` | `@vscode/vsce` |

> **`render_examples.sh` renders less than it used to**, and its header says
> why. Half of it called the Python API directly for the **IR-graph** and
> **unified-KB** views, because `ein ir dot` and `ein kb dot` were removed from
> the CLI in P1.7c. Both renderers are ported and alive — `ein_render::ir_dot`
> and `ein_render::kb_dot`, seventeen views between them, swept by
> `dot_wellformed.rs` — and nothing outside a test can ask for one. Putting
> them back is a decision about the shipping surface, not a `utils/` cleanup.

## Checks

Three things the gate cannot do from inside `cargo test` — **write** a file
it also checks, **grep** the source tree, and **search unboundedly** — and one
generator.

| | | |
|---|---|---|
| [`stdlib_manifest.py`](stdlib_manifest.py) | **writes** `stdlib/MANIFEST.sha256`, and verifies it per module without a toolchain. The writing is the half no test can do — a test that rewrote the file it checks would check nothing. What *is* a test: `ein-ir`'s `the_embedded_copy_matches_the_manifest`, which is not stale-able because `include_dir!` makes each module a build dependency | per-commit CI |
| [`check_hashmap_iteration.py`](check_hashmap_iteration.py) | the determinism grep — no iteration over a hash map at a site whose order could reach an output, `// determinism-ok: <reason>` the only escape. 152 files, 21 annotated | per-commit CI |
| [`fuzz_ein.py`](fuzz_ein.py) | the engine fuzzer: generate or mutate ein programs, then check the five properties one engine can check — `no-crash`, `diagnosed`, `terminates`, `deterministic`, `id-order`. Findings land in [`corpus/fuzz_findings/`](../corpus/fuzz_findings/README.md) | `ein`, and `cargo` for `id-order` |
| [`gen_unicode_printable.py`](gen_unicode_printable.py) | regenerates `ein-core/src/printable.rs` — CPython's `Py_UNICODE_ISPRINTABLE` as a binary-searchable table. Run it after a CPython upgrade | — |

> `fuzz_ein.py`'s strongest property is not its own: `id-order` runs
> `ein-render`'s `id_order_invariance` over a generated batch through the
> `EIN_ID_FILES` seam, because a second copy of that sweep here would be a
> second opinion about what an observable is. It found three things in its
> first sessions, all recorded rather than fixed.

## The M1a measurement set

Nine scripts, and the discipline matters as much as any of them: **run them
through `bench_env.sh`**, which prints the machine state every number was taken
under. A ratio measured on an E-core against one measured on a P-core is a
ratio between two machines.

| | |
|---|---|
| [`bench_env.sh`](bench_env.sh) | the fingerprint — CPU, governor, turbo, current MHz, loadavg, `perf_event_paranoid`, the commit — then `taskset` onto a P-core and the command. `--report` for the fingerprint alone |
| [`e2e_baseline.py`](e2e_baseline.py) | the milestone's workloads as **processes**: best, median, spread and peak RSS. `--bin LABEL=PATH` compares two *builds* of one engine, which is what it is for now — allocator arms, feature flags, `--profile`s. Three row sets: the milestone six, `--blind` (the enumerator the six never reach), `--startup` |
| [`profile_ein_rs.py`](profile_ein_rs.py) | `perf record --call-graph lbr` over a `--profile profiling` build, as self time **by symbol** and **by subsystem** — bucketed by the innermost enclosing engine frame, not the leaf, because `FactStore::get` has two callers and a leaf-only rule is wrong about half of them |
| [`criterion_table.py`](criterion_table.py) | criterion's `estimates.json` as one table — mean, sd, relative sd, CI — with `--max-rsd` as an **exit code**. The console output scrolls and buries the one column a "3× faster" claim needs |
| [`feature_matrix.py`](feature_matrix.py) | the `features.md` lever matrix: flip exactly one `SolverConfig` knob off the puzzle's own resolved config and re-solve. Round-robin over the cells, and a `control` cell byte-identical to the baseline — which is what prices the method now that no second column can disagree with it |
| [`fork_split.py`](fork_split.py) | what a run does at root versus per entering, split out of the `--events` stream at its `enter` events. The instrument behind `baseline.md` §9 |
| [`fork_delta_verify.py`](fork_delta_verify.py) | does the **resumed** fork saturator reach the same fixpoint? One binary, twice, `EIN_FORK_DELTA=0` and unset — [D3](../plans/m1a_rust/divergences.md)'s fixture |
| [`corpus_cost.py`](corpus_cost.py) | what every corpus cell costs, on the engine that ships — mean, sd, relative sd, n, per cell and summed per entry, with `--also` for a run the manifest does not declare and `--check` as an exit code. [S1a.9.0](../plans/m1a_rust/p1a.9_release/s1a.9.0_slow_corpus.md)'s instrument: it is what re-took the `slow` flag after two engines, and what `corpus/corpus.toml`'s `cost_ms` is regenerated with |
| [`spec_audit.py`](spec_audit.py) | how often a *speculated* entering would have been wrong. [S1a.7.0](../plans/m1a_rust/p1a.7_parallelism/s1a.7.0_speculation_audit.md)'s instrument, which measured the parallelism phase's central risk before any of it was built — 1 078 704 enterings speculated against layer-start root |

Results: [`baseline.md`](../plans/m1a_rust/p1a.6_performance/baseline.md),
[`scaling.md`](../plans/m1a_rust/p1a.7_parallelism/scaling.md) and
[`corpus_cost.md`](../plans/m1a_rust/p1a.9_release/corpus_cost.md).

> **The CPython and PyPy columns in those documents are frozen constants.**
> Nothing here can re-measure one: `bench_baseline.py`, `count_work.py` and
> `profile_solve.py` left with the engine they measured. Each document says
> which of its numbers are live and which are not; `baseline.md` has the table.

## Not scripts

- [`vscode-ein/`](vscode-ein/) — the ein-lang TextMate grammar for VS Code
  (`ein.tmLanguage.json` + `language-configuration.json`), packaged by
  `package_vscode_ein.sh`.
- `feature_matrix_results.json` — **git-ignored**, machine-local, rewritten by
  every `feature_matrix.py` run. The lever matrix's committed record is
  [`docs/kernel/inference/features.md`](../docs/kernel/inference/features.md).

---

## The census

`utils/` was **28 scripts** until M1a
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md).
Nineteen of them named `ein.py`, `pypy`, a venv or `PYTHONPATH`. Eleven went;
every one had a successor or a dead subject, and the table is here so that a
plan document citing one of them can be read.

| | why it went | what answers its question now |
|---|---|---|
| `py_oracle.py` | CPython's `repr` / `format` / `sorted` behind a JSON-Lines protocol — two engines by construction | `ein-core/tests/golden/{repr_values,repr_escapes,float_format}.txt`, banked at S1a.10.2 |
| `ir_oracle.py` | ein.py's frontend behind the same protocol | `corpus_shapes.md5` — the same ops, digested |
| `mutant_ein.py` | the D3 event cut's negative control, which needed the harness to be its differ | `ein-infer/tests/event_cut_control.rs`, the same three mutations in-process |
| `bench_baseline.py` | the Python half of `cargo bench` | `cargo bench` — with one half, it *is* the set |
| `count_work.py` | the Python half of the work counters | `ein_core::counters`, behind `ein-infer`'s `counter_cost` example |
| `profile_solve.py` | cProfile around one ein.py `solve()` | `profile_ein_rs.py`, which keeps its eight subsystem buckets on purpose — `baseline.md` §3 is written in them |
| `measure_match_skips.py` | sized a semi-naive-matching win **in ein.py** | ein.rs solved that differently; nothing to size |
| `measure_redundant_firings.py` | sized the fork-resume win in ein.py | ein.rs took it at S1a.6.9; `fork_split.py` is the ein.rs-side instrument |
| `symmetric_bench.py` | `__symmetric__` mirror vs the stdlib rule, in-process Python | `feature_matrix.py`'s `no-symmetric-mirror` cell |
| `find_dead_defs.py` | a Python tool reading Python source | dies with its subject |
| `relation_algebra_examples.py` | ditto — worked `std.algebra` examples through the Python API | the fixtures under `examples/saturation/` |

Two more things left this directory rather than the repo:

- **The corpus runner.** `ein-conformance run --tier T0…T3` swept the corpus
  over two engines; the sweep is `ein-cli/tests/corpus_cli.rs` now — 622 cells
  as processes in 3.6 s, inside `cargo test`
  ([S1a.10.3](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md);
  542 cells until [S1a.9.0](../plans/m1a_rust/p1a.9_release/s1a.9.0_slow_corpus.md)
  un-flagged fourteen entries, 641 with `EIN_CORPUS_SLOW=1`).
- **The determinism sweep.** Two `PYTHONHASHSEED`s over one engine became
  `ein-render/tests/id_order_invariance.rs`, which permutes the **id space**
  instead and is stronger for it. `check_hashmap_iteration.py` above is the
  static half of the same question, and stayed.
