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
- **`corpus/`** — the **corpus**: `corpus.toml` (one entry per `.ein`, with
  the runs it is exercised under) plus `fuzz_findings/`. A file under
  `examples/` or `stdlib/` with no entry fails a completeness check. What
  runs it is `cargo test`, and [`corpus/README.md`](corpus/README.md) has the
  table of readers. It was `conformance/` until M1a S1a.10.3, and the
  `--events` protocol it also held is now
  [`docs/kernel/inference/events.md`](docs/kernel/inference/events.md).
- **`ein.rs/`** — the Rust port ([M1a](plans/m1a_rust/README.md)), a
  drop-in replacement for `ein`, and since
  [P1a.10](plans/m1a_rust/p1a.10_single_implementation/README.md) the only
  implementation. Two of its seven crates are dev-only: `ein-corpus` (the
  manifest, the fixture helpers, the bench set) and `ein-parity` (the one
  implementation of what counts as a derivation's *narration* rather than
  its content — [design/01
  §5](plans/m1a_rust/design/01_parity_contract.md#5-legitimate-divergences-the-normalisation-list);
  `EIN_PARITY_STRICT=1` turns it off). **`ein.py/` was the oracle** until
  S1a.10.2 banked what only it proved; what remains of that argument is
  the ledger, the goldens under `tests/golden/from_ein_py/` — the last
  independent provenance in the repo — and the divergence list.
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

## Running the gate

```sh
cargo test --manifest-path ein.rs/Cargo.toml --workspace     # the whole gate
EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli  # + the 17 slow entries
EIN_ID_SEEDS=8    cargo test … -p ein-render --test id_order_invariance
EIN_BLESS=1       cargo test … --workspace                   # re-bank the goldens
```

Everything runs one engine. `cargo test --workspace` is the gate — the corpus
sweep through the CLI, the shape digests, the goldens, the manifest's own
invariants — and it needs **Graphviz** on `PATH`, because
`dot_wellformed.rs` is the only authority the DOT views have on being
well-formed and it fails rather than skips without it.

The `ein` binary links `snmalloc` by default since S1a.6.2 (worth 8–16 % of a
solve), so the build needs **`cmake` and a C++ compiler**;
`cargo build --release -p ein-cli --no-default-features` builds against the
system allocator and needs neither.

**The parity harness is gone** (M1a S1a.10.3). `ein-conformance run --impl-a …
--impl-b … --tier T0…T3` ran two implementations over the corpus and diffed
them; there is no second operand. What each tier proved and who owns it now is
[the oracle ledger](plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md).

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
