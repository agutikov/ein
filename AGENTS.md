# AGENTS.md

Guidance for AI coding agents working in this repo.

## What this project is

`ein-bot` is a graph-based reasoner for Zebra-style logic puzzles. The
2021 proof-of-concept (`reasoning.py`, with worked examples and
inference rules under `docs/PoC/`) is being modernised in light of
neuro-symbolic / constrained-reasoning research.

## Where things live

- **`docs/PoC/`** — original 2021 PoC. README explains the graph encoding,
  the *triangle* and *square* inference rules, and the hypothesis-testing
  loop. Treat as historical reference; don't modify unless explicitly
  asked.
- **`docs/index/`** — catalogue of external tech relevant to the rewrite
  (LLM constrained generation, CSP/SAT/SMT, theorem proving, category
  theory, graphs & rewrite systems, …). 11 thematic files + a knowledge
  graph (`knowledge-graph.dot` → SVGs and a Cytoscape.js page).
- **`docs/ideas/`** — the user's *own* ideas (8 files); each preserves
  user quotes and open questions. Authoritative on intent.
- **`reasoning.py`** — current single-file implementation. Scheduled for
  a full rewrite into a proper Python project; see `TODO.md`.
- **`utils/`** — renderers (`render_knowledge_graph.sh` for Graphviz,
  `render_knowledge_graph_cy.py` for Cytoscape).
- **`nlp/`, `smt/`** — scratch areas with submodules
  (`nlp/link-grammar`, `smt/CVC4`). Not used by `reasoning.py`.

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
