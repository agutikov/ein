# `utils/` — the scripts that are not the engine

**Twenty-four scripts, 8 228 lines, all of them driving `ein.rs`** — bar one,
[`doc_audit.py`](doc_audit.py), whose subject is `docs/kernel/` and which reads
the crates as text. Nothing here is built, shipped or imported by the engine;
everything here *runs* it, *reads* it, or *renders* what it produced. If a check belongs in the gate it belongs in
`cargo test`, and several things that used to live here moved there —
[§ The census](#the-census) says which.

**Naming the binary.** Every script that runs the engine says which build:
`$EIN_BIN` or `--bin`, defaulting to `ein.rs/target/release/ein`. Three want a
build of their own — `fork_delta_verify.py` → `target-fd`, `spec_audit.py` →
`target-sa`, `profile_ein_rs.py` → `--profile profiling`, which it builds.
**None takes an `--impl`**: a flag with one value invites a reader to look for
the operand that is gone
([S1a.10.4](../docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine)).

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

Five things the gate cannot do from inside `cargo test` — **write** a file
it also checks, **grep** the source tree, **search unboundedly**, **sweep
the whole corpus under `--events`**, and **read the documentation tree** — and
one generator.

| | | |
|---|---|---|
| [`stdlib_census.py`](stdlib_census.py) | what the seven `std.*` modules declare against what the corpus actually activates: 73 rules parsed for module / parameters / priority / guard shapes, then every corpus entry × every declared `solve` / `saturate` / `test` run under `--events`, `fire` counted by rule and split productive vs redundant. Two things it must get right — a file that declares its own `symmetric` is not exercising `std.algebra`'s, and `normal` elides redundant firings, so three rules read as zero there and fire at `verbose`. `--check` exits 1 while any rule is at zero. **It is no longer the gate** — [S1c.1.5](../docs/history/m1c_external_validation/README.md#s1c15--in-the-gate) made that `ein-infer/tests/stdlib_coverage.rs`, in-process, 0.04 s, and scoped to `tests/stdlib/` rather than to all 180 entries, because a rule that fires only inside `examples/zebra.ein` has no test. What stays here is the measurement the gate is a yes/no of: firings per rule, productive vs redundant, the sole-activator table, and `-k` for one directory's contribution. The measurement: [`stdlib_census.md`](../docs/history/m1c_external_validation/stdlib_census.md) — **38 of 73 rules never fired** on 2026-08-23 with `examples/zebra.ein` the sole activator of 20 more, and **0 of 73** on the [re-take](../docs/history/m1c_external_validation/stdlib_census.md#11-the-re-take--2026-08-24-and-the-zero-set-is-empty) after S1c.1.4 landed `tests/stdlib/` | `ein`, 37 s |
| [`layer_census.py`](layer_census.py) | what a **layer** of the search costs and what it buys — M1d [S1d.10.1](../docs/history/m1d_satisfiability/README.md#s1d101--why-it-does-not-finish)'s instrument, and the first aimed at the *structure* of the search rather than at its speed. Every corpus entry under `solve -e`, twice: once bare, for the `ms` and `MiB` a reader means by "does it finish", and once narrated, for the `layer` event's sixteen counters. `nogoods_emitted` always said what a layer's deaths *produced*; the column that did not exist is **`dropped_nogood`** — what the clauses then removed from the next layer's join. `--layers` prints the per-layer rows, `-k` picks an entry, and an entry that outlives `--timeout` is re-tried at `-m 3`, `-m 2`, `-m 1` rather than dropped, because the run that does not finish is the subject. Two things it has to get right: `--events` goes to a **FIFO** (an exhaustive `zebra2-minus-15 -m 3` narrates 72.6 M events and filled a 16 GiB `/tmp` at 7.1 GB), and a run is killed above `--max-rss-mb`, because four entries have no finite hypothesis space and reach 14 GB. The measurement: [`layer_census.md`](../docs/history/m1d_satisfiability/layer_census.md) | `ein` |
| [`openness_census.py`](openness_census.py) | what every corpus entry **owes**, and whether it is judged by discharge or by exhaustion — M1d [S1d.2.6](../docs/history/m1d_satisfiability/README.md#s1d26--verdicts-counters-corpus)'s instrument, and the evidence its **scope rule** is a measurement rather than a hope. Three numbers per entry from `--json-summary`'s `owes` block: `declared` (how many obligation rules the program *states* — the one nothing reported before, and the one that cannot be inferred, since `owes = 0` is equally true of a debt paid and a debt never stated), `root`, and the per-model tallies. `--scope` prints just the partition, `-k` one entry with its debts spelled out. Two things it has to get right: a run the manifest does not declare is **not run** (29 entries drop `solve` because it does not terminate on them, and timing those out would add rows about patience rather than openness), and nothing is re-derived from the event stream — T1d.2.4.5 already proved the engine's tally against a hand count, and a census that reconstructs its subject can disagree with it. The measurement: [`openness_census.md`](../docs/history/m1d_satisfiability/openness_census.md) — **92 of 121** entries that reach a fixpoint state no obligation and keep their word, 12 moved to `Open` | `ein`, ~2 min |
| [`model_set_census.py`](model_set_census.py) | what a model **set** is made of and whether it **factors** — M1d [S1d.3.1](../docs/history/m1d_satisfiability/README.md#s1d31--what-the-models-differ-in)'s instrument, and the first census whose subject is the *answer* rather than the search or the program. It reads `--json-summary`'s `verdict.solutions` as *k* fact sets and turns them into **decision variables**, which is the part that can be wrong and so is licensed rather than assumed: every varying positive atom is a Boolean, and only a relation the program declares `functional` / `bijective` permits collapsing `(R a ·)` into one multi-valued variable — read from the **models' own facts**, never from the source, so a declaration, a `bijective` fan-out and a rule-derived marker are one thing. Then the three granularities the phase asks about (by relation, by variable pair, by partition) and the one it did not: is the set a **free grid** over a small basis? The minimum determining set is a minimum hitting set of the pairwise difference masks, so both the search and the count run on machine words. Two things it has to get right: a deeper `-m` is tried **only where the run was cheap** (`--escalate-below` — the subject is the model set and `-m 5` is a default, but `features/01 -e` reaches 2.7 GB at `-m 8`), and a set the cap truncated is reported as one, because intersecting a *subset* of the models gives a superset of the core. The measurement: [`model_set_census.md`](../docs/history/m1d_satisfiability/model_set_census.md) — **13** entries have a model set, **2** partition (both two-object demos; the three-object one does not), 5 are a free grid, and `zebra2-minus-15`'s 23 varying variables are **one** component whose graph is K₂₃ minus five edges. **`--form {envelope,key,list,diagram}`** is S1d.3.2's half: it renders a model set as one of the candidate *representations*, because a representation argued about in prose and never printed is one nobody has read. `diagram` is a price rather than a picture — the exact reduced-MDD node count under five variable orders, a node being a distinct residual set. The pricing: [`representations.md`](../docs/history/m1d_satisfiability/representations.md). **Since M1d S1d.3.3 `--form key` has a twin in the engine** — `ein solve -e --models key` renders the same table from `ein-render/src/models.rs`, and the two agree row for row on all 32 of `zebra2-minus-15`, which is the closest thing this repo still has to an oracle diff: two independent implementations of the decision-variable rules and the hitting-set search, checked against each other rather than eyeballed. What stays here is the **census** — the four granularities, `diagram`, `envelope`, and the whole-corpus sweep the engine has no reason to carry | `ein`, ~9 min |
| [`closure_census.py`](closure_census.py) | who states a claim about a model **set**, and what stating one would cost — M1d [S1d.4.1](../docs/history/m1d_satisfiability/README.md#s1d41--what-closure-costs)'s instrument, and the fourth census: not the search, not the program, not the answer, but the **sentence a file writes about its own answer**. Its transport is `ein test --json-report`, the read-out the same stage added, because the one thing the stage insisted on is the one thing nothing could do — read a claim's *shape* off the **loaded program**. That is not pedantry: the stage's own reconnaissance grepped `:expect (or`, found two users, and one of them was a **header comment documenting the form**, so the corpus's count of set-closure claims went from two to **one**. Four tables — who claims (the denominator: 59 of 124 queries, 1 of 124 about a set), whether the claim is checkable (59 of 59, `exhausted` on all), what a claim would **cost to write** (`k × |goal extent| / |file|`, and its own arithmetic checked against the 38 claims that exist — never over-charges, 17 exact), and the **counterfactual `NOT CHECKED` set**, which is what an empty outcome column actually means. Two things it has to get right: `ein test`'s regime exactly (exhausting, `-m 5`, one job — otherwise table 4 is about a run nobody performs), and a capped model set reported as a **floor**, since the facts it would take to list a subset are a lower bound. The measurement: [`closure_census.md`](../docs/history/m1d_satisfiability/closure_census.md) — the write cost is worst on a **95-line feature demo at 4.28×**, not on the puzzle at 0.96×, and 10 of the 121 entries that reach a fixpoint could not certify a model set today | `ein`, ~3 min |
| [`stdlib_manifest.py`](stdlib_manifest.py) | **writes** `stdlib/MANIFEST.sha256`, and verifies it per module without a toolchain. The writing is the half no test can do — a test that rewrote the file it checks would check nothing. What *is* a test: `ein-ir`'s `the_embedded_copy_matches_the_manifest`, which is not stale-able because `include_dir!` makes each module a build dependency | per-commit CI |
| [`check_hashmap_iteration.py`](check_hashmap_iteration.py) | the determinism grep — no iteration over a hash map at a site whose order could reach an output, `// determinism-ok: <reason>` the only escape. 152 files, 21 annotated | per-commit CI |
| [`fuzz_ein.py`](fuzz_ein.py) | the engine fuzzer: generate or mutate ein programs, then check the **six** properties one engine can check — `no-crash`, `diagnosed`, `terminates`, `deterministic`, `id-order` and, since M1a T1a.7.2.6, `jobs` (the same program at `--jobs 8` answers as at `--jobs 1`; `--jobs 1` turns it off). Findings land in [`corpus/fuzz_findings/`](../corpus/fuzz_findings/README.md) | `ein`, and `cargo` for `id-order` |
| [`doc_audit.py`](doc_audit.py) | does a page in `docs/kernel/` still describe the engine that ships? — M1e [S1e.2.2](../plans/m1e_review_processing/p1e.2_high/s1e.2.2_code_doc_consistency.md)'s instrument, and the only script here whose subject is the documentation. Three questions, none of which needs to know what a milestone changed, which is the point: **identifiers** (every backticked `EIN_*` / `foo.rs` / `fn()` / `Type` / `snake_case` resolved against `ein.rs/crates/**` — a *report*, since `Human` and DOT node ids are not identifiers and no rule tells them apart), **links** (file and `#anchor`, GitHub-slugified) and **states** (which pages carry a superseded banner). The one it exists for is a fourth thing links cannot be: a **prose `§x.y`** written after a link or in its label is not part of the link, so no anchor checker sees one — and S1e.2.2 found six such citations naming four sections that `algorithm_layer_n.md` has never had, plus two into a `§1.5` that does not exist. `--check` exits 1 on the link half only. **Not a gate** — whether it becomes one is `DO-M2`'s question; the fifth check on [`docs/kernel/README.md` § Keeping this true](../docs/kernel/README.md), *run the commands a page shows*, has no instrument and found the most | — |
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
| [`bench_env.sh`](bench_env.sh) | the fingerprint — CPU, governor, turbo, current MHz, loadavg, `perf_event_paranoid`, the commit — then `taskset` onto a P-core and the command. `--report` for the fingerprint alone. **`--cores P:8` / `PT:8` / `E:8`** (M1a S1a.7.1) is the multi-core form a `--jobs N` number needs: on a hybrid CPU "8 cores" names three different machines, and this one resolves the set, reports how many *physical* cores it covers, and refuses a spec the machine cannot fill |
| [`e2e_baseline.py`](e2e_baseline.py) | the milestone's workloads as **processes**: best, median, spread and peak RSS. `--bin LABEL=PATH` compares two *builds* of one engine, which is what it is for now — allocator arms, feature flags, `--profile`s. Three row sets: the milestone six, `--blind` (the enumerator the six never reach), `--startup` |
| [`profile_ein_rs.py`](profile_ein_rs.py) | `perf record --call-graph lbr` over a `--profile profiling` build, as self time **by symbol** and **by subsystem** — bucketed by the innermost enclosing engine frame, not the leaf, because `FactStore::get` has two callers and a leaf-only rule is wrong about half of them |
| [`criterion_table.py`](criterion_table.py) | criterion's `estimates.json` as one table — mean, sd, relative sd, CI — with `--max-rsd` as an **exit code**. The console output scrolls and buries the one column a "3× faster" claim needs |
| [`feature_matrix.py`](feature_matrix.py) | the `features.md` lever matrix: flip exactly one `SolverConfig` knob off the puzzle's own resolved config and re-solve. Round-robin over the cells, and a `control` cell byte-identical to the baseline — which is what prices the method now that no second column can disagree with it |
| [`fork_split.py`](fork_split.py) | what a run does at root versus per entering, split out of the `--events` stream at its `enter` events. The instrument behind `baseline.md` §9 |
| [`fork_delta_verify.py`](fork_delta_verify.py) | does the **resumed** fork saturator reach the same fixpoint? One binary, twice, `EIN_FORK_DELTA=0` and unset — [D3](../docs/history/m1a_rust/divergences.md)'s fixture |
| [`corpus_cost.py`](corpus_cost.py) | what every corpus cell costs, on the engine that ships — mean, sd, relative sd, n, per cell and summed per entry, with `--also` for a run the manifest does not declare and `--check` as an exit code. [S1a.9.0](../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced)'s instrument: it is what re-took the `slow` flag after two engines, and what `corpus/corpus.toml`'s `cost_ms` is regenerated with |
| [`spec_audit.py`](spec_audit.py) | how often a *speculated* entering would have been wrong. [S1a.7.0](../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)'s instrument, which measured the parallelism phase's central risk before any of it was built — 1 078 704 enterings speculated against layer-start root |

Results: [`baseline.md`](../docs/history/m1a_rust/measurements/baseline.md),
[`scaling.md`](../docs/history/m1a_rust/measurements/scaling.md) and
[`corpus_cost.md`](../docs/history/m1a_rust/measurements/corpus_cost.md).

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
[S1a.10.4](../docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine).
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
  ([S1a.10.3](../docs/history/m1a_rust/README.md#s1a103--the-corpus-without-a-second-engine);
  542 cells until [S1a.9.0](../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced)
  un-flagged fourteen entries, 641 with `EIN_CORPUS_SLOW=1`).
- **The determinism sweep.** Two `PYTHONHASHSEED`s over one engine became
  `ein-render/tests/id_order_invariance.rs`, which permutes the **id space**
  instead and is stronger for it. `check_hashmap_iteration.py` above is the
  static half of the same question, and stayed.
