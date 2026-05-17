# ein-bot

> **Personal learn-and-research project.** Not production software, not
> a library, not a benchmark. Code, notes, and the topic catalogue here
> exist to explore neuro-symbolic / constrained-reasoning ideas
> end-to-end, not to be reused as-is. Expect rough edges and frequent
> rewrites.

Graph-based Zebra-puzzle solver — a 2021 proof-of-concept now being
modernised in light of neuro-symbolic and constrained-reasoning research.

## Layout

| path           | what's in it                                                                 |
|----------------|------------------------------------------------------------------------------|
| `docs/PoC/`    | original 2021 proof-of-concept: graph encoding, inference rules, hypothesis testing ([README](docs/PoC/README.md)) |
| `docs/index/`  | "awesome-list"-style catalogue of external tech (LLM constraints, CSP/SAT/SMT, theorem proving, …) across 11 topic files, with a Graphviz + Cytoscape knowledge graph |
| `docs/ideas/`  | the user's own ideas extracted from research notes (8 files)                 |
| `reasoning.py` | current single-file PoC (slated for a full rewrite — see `TODO.md`)          |
| `utils/`       | renderers for the knowledge graph                                            |
| `nlp/`, `smt/` | scratch areas for the upcoming rewrite (link-grammar, CVC4 submodules)       |

## Knowledge graph

The 11 topic files are summarised as a single graph in
[`docs/index/knowledge-graph.dot`](docs/index/knowledge-graph.dot)
(77 clusters, 318 nodes, 256 edges). Two views:

```sh
# static SVGs (dot / fdp / sfdp / osage) — requires graphviz
utils/render_knowledge_graph.sh svg all

# interactive Cytoscape.js page — open docs/index/knowledge-graph.cy/index.html
python3 utils/render_knowledge_graph_cy.py
```

## Status

`reasoning.py` is the original PoC. The active work (planning and
rewriting) is tracked in [`TODO.md`](TODO.md); see
[`AGENTS.md`](AGENTS.md) for context aimed at coding agents.
