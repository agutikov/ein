# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this project is

Ein is a graph-based reasoner for Zebra-style logic puzzles. The
2021 prototype is being modernised in light of neuro-symbolic /
constrained-reasoning research.

## Where things live

- **`docs/kernel/`** — **canonical M1 kernel documentation**: graph
  semantics (`ir/01-ein-graph/`), Python data model (`ir/02-data-model/`),
  surface S-expression language (`ir/03-ein-lang/`, mostly what used
  to be `docs/ir.md`), inference engine (`inference/`, stub before P1.3).
  Start here for any "what does Ein reason about / how" question.
  See [`docs/kernel/README.md`](docs/kernel/README.md) for orientation.
- **`docs/api/`** — the **Python embedding API** reference (P1.20 Theme J):
  how to drive Ein *as a library* (`parse` → `KnowledgeBase` → `solve` →
  read verdict/trace). `ein.md` is the contract + worked example; per-module
  pages for `ir`/`kb`/`inference`/`trace`. Distinct from `docs/kernel/`
  (the IR *language*) and the engine internals.
- **`docs/guide/`** — **the newcomer tutorial** ("Learn Ein by solving the
  Zebra puzzle"): objects/relations/facts → rules → the full solve, four
  chapters (P1.20 Theme K). User-facing; references `docs/kernel/` +
  `docs/api/`, never explains internals; complements
  `inference/zebra_walkthrough.md`.
- **`docs/lib/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, reasoning benchmarks, …). 12
  thematic files + a knowledge graph (`knowledge-graph.dot` → SVGs
  and a Cytoscape.js page).
- **`plans/ideas/`** — the user's *own* ideas (10 numbered files +
  README; moved here from `docs/ideas/`); each preserves user quotes and
  open questions. Authoritative on intent.
- **`examples/`** — encoded Zebra puzzles (`zebra.ein` classic,
  `zebra2.ein` unified-is-a / `*-loc`, `zebra2-hints.ein` partial-state
  fixture) plus focused per-feature fixtures (`features/`, `branching/`,
  `saturation/`, `lattice/`, `domain_elim/`, `broken/`).
  [`examples/README.md`](examples/README.md) is a **catalog** — one line
  per file / sub-dir.
- **`docs/kernel/inference/zebra_walkthrough.md`** — the Wikipedia human
  Zebra walkthrough annotated as ein.py inference (NL↔ein rule↔branch-depth
  table, hypotheses with their contradictions and no-good clauses; **moved
  here from `examples/README.md`**). The **M1 target trace** for the engine
  and the **M2 target** for the NL ⇄ IR round-trip (NL problem → facts →
  ontology+rules → solution → NL explanation).
- **`stdlib/`** — the ein-lang standard library (`std.*`), **shared by
  both implementations** and the single source of truth. `MANIFEST.sha256`
  is what identifies a directory as the stdlib and what CI checks for
  drift; `ein.py/src/ein/stdlib/` is a build-time copy (git-ignored) so a
  wheel still works. Resolution in both engines: `$EIN_STDLIB` → the
  checkout → the packaged/embedded copy.
- **`conformance/`** — the **parity corpus**: `corpus.toml` (one entry per
  `.ein`, with the runs it is exercised under) and `EVENTS.md` (the
  `--events` protocol). A file under `examples/` or `stdlib/` with no entry
  fails a completeness check in both suites.
- **`ein.rs/`** — the Rust port ([M1a](plans/m1a_rust/README.md)), a
  drop-in replacement for `ein`. `crates/ein-conformance` is the parity
  harness — it shells out to both engines and links neither, and
  `crates/ein-parity` is the one implementation of what the two engines are
  *not* required to agree on ([design/01
  §5](plans/m1a_rust/design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list);
  `--strict` / `EIN_PARITY_STRICT=1` turns it off). **`ein.py/` is the
  oracle**: any observable difference is a bug in ein.rs, and every
  optimisation there is justified by "the harness says nothing changed" —
  except a fork's *narration*, which since
  [S1a.6.9](plans/m1a_rust/p1a.6_performance/s1a.6.9_fork_entry_delta.md)
  differs on purpose and is pinned by ein.rs's own goldens instead.
- **`ein.py/`** — Python implementation. `ein.py/src/ein/` is the
  package: IR parser + dumper under `ir/`; KB store + entities +
  provenance under `kb/`; inference engine + saturator + contradiction
  detector + hypothesis loop under `inference/`; the `ein` console
  script under `cli/` (subcommands `render` / `saturate` / `solve` — the
  `ir` / `kb` inspectors were removed, and the `profile` / `symmetric`
  engine runners moved to `utils/` scripts). `ein.py/tests/` is the pytest
  suite, `ein.py/pyproject.toml` is the build config.
- **`utils/`** — renderers (`render_knowledge_graph.sh` for Graphviz,
  `render_knowledge_graph_cy.py` for Cytoscape) + ad-hoc engine
  probe/measure scripts (`find_dead_defs.py`, `relation_algebra_examples.py`, …)
  + the promoted engine runners `profile_solve.py` (cProfile a `solve()`)
  and `symmetric_bench.py` (symmetric-closure micro-benchmark).
  **The M1a measurement set** is here too, and every one of these compares the
  two implementations rather than timing one:
  `bench_env.sh` (prints the machine state and pins to a P-core — run the
  others through it), `bench_baseline.py` (the eight-bench set, in-process,
  the Python half of `cargo bench`), `e2e_baseline.py` (the same workloads as
  *processes*, which is what the milestone's targets mean),
  `profile_ein_rs.py` (`perf` self time by symbol and by subsystem, bucketed
  like `profile_solve.py` so the two profiles read side by side),
  `count_work.py` (what ein.py *did* — the counterpart of
  `ein_core::counters`), `criterion_table.py` (criterion's standard
  deviations, with the 3 % gate as an exit code). Results:
  [`plans/m1a_rust/p1a.6_performance/baseline.md`](plans/m1a_rust/p1a.6_performance/baseline.md).
- **`nlp/`, `smt/`** — scratch areas with submodules
  (`nlp/link-grammar`, `smt/CVC4`). Not wired into the active
  `ein.py/` package.

## Running the parity harness

```sh
cd ein.rs && cargo build --release
./target/release/ein-conformance run \
    --impl-a "python3 -m ein.cli" --impl-b "python3 -m ein.cli" --tier T3
```

Python-vs-Python is not a curiosity — it is the gate: a harness that cannot
detect a difference between an implementation and itself cannot detect one
between two implementations either. The same shape with `--env-a
PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42` is the determinism sweep, which
is how hazards H1 and H4 were found. `conformance/README.md` has the tiers.

## Regenerating the knowledge graph

When `docs/lib/knowledge-graph.dot` changes, re-render both views:

```sh
utils/render_knowledge_graph.sh svg all     # 4 SVGs (dot/fdp/sfdp/osage)
python3 utils/render_knowledge_graph_cy.py  # elements.js + style.js + index.html
```

The Cytoscape view's `index.html` is a single self-contained file that
loads Cytoscape + fcose from unpkg CDN.

## Working priorities

The biggest unrealised idea is `plans/ideas/01-self-modifying-constraint-language.md`
(LLM ↔ harness loop on GBNF). The current-implementation-vs-target axis
is `05-zebra-puzzle-graph-reasoner` → `04-nlp-to-graph-to-solver-pipeline`
/ `06-inference-rules-completeness` / `08-human-style-deductive-trace`.

## Style

- The user is bilingual RU/EN but prefers EN in code and docs.
- Prefers dense, link-rich answers; few-but-substantive over many-but-thin.
- For plans/ideas/* extensions: keep the user's framing intact; do not
  cite "conversation-N msg M" (raw conversations were removed 2026-05-17).

`CLAUDE.md` is a symlink to this file — both AI tools see the same guidance.
