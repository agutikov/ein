# ein-bot

> **Personal learn-and-research project.** Not production software, not
> a library, not a benchmark. Code, notes, and the topic catalogue here
> exist to explore neuro-symbolic / constrained-reasoning ideas
> end-to-end, not to be reused as-is. Expect rough edges and frequent
> rewrites.

Graph-based Zebra-puzzle solver — a 2021 proof-of-concept now being
modernised in light of neuro-symbolic and constrained-reasoning research.

## Layout

| path                 | what's in it                                                                 |
|----------------------|------------------------------------------------------------------------------|
| `src/ein_bot/`       | the package — `State`, `VersionedState`, parser, DOT rendering, CLI          |
| `tests/`             | pytest suite + `tests/data/conditions.txt` (Zebra-puzzle fixture)            |
| `utils/`             | renderers for the knowledge graph (Graphviz + Cytoscape)                     |
| `docs/PoC/`          | 2021 proof-of-concept — graph encoding, inference rules, hypothesis testing, plus the original `reasoning.py` archived ([README](docs/PoC/README.md)) |
| `docs/index/`        | "awesome-list" catalogue of external tech across 11 topic files + a knowledge graph |
| `docs/ideas/`        | ideas extracted from research notes (8 files)                 |
| `nlp/`, `smt/`       | scratch areas for the upcoming rewrite (link-grammar, CVC4 submodules)       |
| `pyproject.toml`     | PEP 621 metadata; deps `numpy`; dev extras `pytest`, `pytest-cov`, `ruff`    |
| `venv_install.sh`    | bootstrap: create `.venv/` and install the project editable with dev extras  |
| `AGENTS.md`          | guidance for AI coding agents (`CLAUDE.md` is a symlink to it)               |
| `TODO.md`            | live worklist                                                                |

## Quickstart — Python CLI

The package installs a console script `ein-bot` that reads a *conditions
file* (whitespace-delimited `obj rel obj` triples, plus bare object
declarations) and prints either a DOT graph or the round-tripped text.

### Install

```sh
./venv_install.sh                  # creates .venv, installs ein-bot[dev]
# or pin an interpreter:
./venv_install.sh /usr/bin/python3.12
source .venv/bin/activate
```

The script needs Python ≥ 3.10. It's safe to re-run — an existing
`.venv/` is reused.

### Usage

```sh
ein-bot CONDITIONS_PATH [--no-color] [--dump]
```

| flag         | effect                                                       |
|--------------|--------------------------------------------------------------|
| *(default)*  | print a Graphviz `digraph` with HTML-coloured edge labels (one colour per relation, CRC-hashed for stability) |
| `--no-color` | print plain DOT (no `<font>` tags) — useful for piping into `dot` / `fdp` |
| `--dump`     | print the canonical text form instead of DOT (round-trippable through `parse()`) |

### Examples

```sh
# render the bundled Zebra puzzle as an SVG
ein-bot tests/data/conditions.txt --no-color | dot -Tsvg -o /tmp/zebra.svg

# inspect the round-tripped text form
ein-bot tests/data/conditions.txt --dump | head

# from Python
python -c '
from ein_bot import State, load_file
s = State()
load_file(s, "tests/data/conditions.txt")
print(len(s.objects), "objects,",
      sum(len(d) for r in s.relations.values() for d in r.values()), "edges")
'
```

### Conditions file format

```
# blank lines are skipped

Alice                       # bare object declaration
Bob is Person               # 3-token relation:  src REL dst
Carol moves toward Dan      # ≥3 tokens — middle tokens form a single relation
```

A line with exactly two tokens is rejected with `ValueError`.

### Development loop

```sh
pytest -q                  # run the 39-test suite
ruff check .               # lint
ruff check . --fix         # auto-fix what's safe
```

## Knowledge graph

The 11 topic files under `docs/index/` are summarised as a single graph in
[`docs/index/knowledge-graph.dot`](docs/index/knowledge-graph.dot)
(77 clusters, 318 nodes, 256 edges). Two views:

```sh
# static SVGs (dot / fdp / sfdp / osage) — requires graphviz
utils/render_knowledge_graph.sh svg all

# interactive Cytoscape.js page — open docs/index/knowledge-graph.cy/index.html
python utils/render_knowledge_graph_cy.py
```

## Status

The PoC has been split out of a single `reasoning.py` into a proper
package + pytest suite (this is the *refactor* step). The deep rewrite
(full hypothesis/inference engine, neuro-symbolic NL→IR pipeline) is
tracked in [`TODO.md`](TODO.md); see [`AGENTS.md`](AGENTS.md) for
context aimed at coding agents.
