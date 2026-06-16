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
- **`docs/ir.md`** — thin redirect into `docs/kernel/`; kept so stable
  cross-references resolve.
- **`docs/index/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, reasoning benchmarks, …). 12
  thematic files + a knowledge graph (`knowledge-graph.dot` → SVGs
  and a Cytoscape.js page).
- **`docs/ideas/`** — the user's *own* ideas (9 files); each preserves
  user quotes and open questions. Authoritative on intent.
- **`examples/`** — encoded Zebra puzzles (`zebra.ein` classic,
  `zebra2.ein` unified-is-a / `*-loc`, `zebra2-hints.ein` partial-state
  fixture) plus [`examples/README.md`](examples/README.md): the
  Wikipedia human walkthrough annotated as ein.py inference (NL↔ein
  rule↔branch-depth table, hypotheses with their contradictions and
  no-good clauses). It is the **M1 target trace** for the engine and
  the **M2 target** for the NL ⇄ IR round-trip (NL problem → facts →
  ontology+rules → solution → NL explanation).
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
- **`nlp/`, `smt/`** — scratch areas with submodules
  (`nlp/link-grammar`, `smt/CVC4`). Not wired into the active
  `ein.py/` package.

## Regenerating the knowledge graph

When `docs/index/knowledge-graph.dot` changes, re-render both views:

```sh
utils/render_knowledge_graph.sh svg all     # 4 SVGs (dot/fdp/sfdp/osage)
python3 utils/render_knowledge_graph_cy.py  # elements.js + style.js + index.html
```

The Cytoscape view's `index.html` is a single self-contained file that
loads Cytoscape + fcose from unpkg CDN.

## Working priorities

The biggest unrealised idea is `docs/ideas/01-self-modifying-constraint-language.md`
(LLM ↔ harness loop on GBNF). The current-implementation-vs-target axis
is `05-zebra-puzzle-graph-reasoner` → `04-nlp-to-graph-to-solver-pipeline`
/ `06-inference-rules-completeness` / `08-human-style-deductive-trace`.

## Style

- The user is bilingual RU/EN but prefers EN in code and docs.
- Prefers dense, link-rich answers; few-but-substantive over many-but-thin.
- For docs/ideas/* extensions: keep the user's framing intact; do not
  cite "conversation-N msg M" (raw conversations were removed 2026-05-17).

`CLAUDE.md` is a symlink to this file — both AI tools see the same guidance.
