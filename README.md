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
| `ein.py/`               | Python implementation (package, tests, pyproject, demo scripts)              |
| `ein.py/src/ein_bot/ir/` | the IR — Lark grammar, typed AST, parser, dump, DOT renderer                |
| `ein.py/src/ein_bot/cli.py` | console script: `ein-bot ir parse | lint | dot`                          |
| `ein.py/tests/`         | pytest suite (~500 tests)                                                    |
| `ein.py/demo/`          | runnable demo scripts (bench_saturate.py, …)                                 |
| `ein.py/pyproject.toml` | PEP 621 metadata; deps `numpy`, `lark`; dev extras `pytest`, `pytest-cov`, `ruff` |
| `examples/zebra.ein`    | the Zebra puzzle as IR (the smoke-test fixture)                              |
| `examples/broken/`      | curated parse-failure fixtures (file:line:col error messages)                |
| `plans/`                | milestone / phase / stage roadmap (M1 active)                                |
| `docs/kernel/`          | kernel documentation — graph semantics, data model, surface language, inference engine |
| `docs/ir.md`            | thin redirect into `docs/kernel/` (kept for stable cross-references)         |
| `docs/PoC/`             | 2021 proof-of-concept — original `reasoning.py` archived                     |
| `docs/index/`           | "awesome-list" catalogue of external tech across 12 topic files + knowledge graph |
| `docs/ideas/`           | ideas extracted from research notes (9 files)                                |
| `utils/`                | renderers for the knowledge graph (Graphviz + Cytoscape)                     |
| `nlp/`, `smt/`          | scratch areas for the upcoming rewrite (link-grammar, CVC4 submodules)       |
| `venv_install.sh`       | bootstrap: create `.venv/` and install the project editable with dev extras  |
| `AGENTS.md`             | guidance for AI coding agents (`CLAUDE.md` is a symlink to it)               |
| `TODO.md`               | live worklist                                                                |

## Quickstart — Python CLI

The package installs a console script `ein-bot` with three subcommands
on the IR.

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
ein-bot ir parse <file>                              # canonical re-dump
ein-bot ir lint  <file>                              # parse-only check
ein-bot ir dot   <file> [--rule-mode=a|c] [--trace-view=a|b|c]
```

| subcommand    | effect                                                       |
|---------------|--------------------------------------------------------------|
| `ir parse`    | parse the file and emit the canonical text form to stdout (round-trippable through `dump_canonical`) |
| `ir lint`     | parse-only check; non-zero exit + `file:line:col` on stderr if malformed |
| `ir dot`      | render the parsed IR as a Graphviz `digraph` per [`docs/kernel/ir/03-ein-lang/04_dot_rendering.md`](docs/kernel/ir/03-ein-lang/04_dot_rendering.md) |
| `--rule-mode` | `a` side-by-side LHS/RHS clusters, `c` (default) overlay with dashed RHS |
| `--trace-view` | `a` (default) per-step, `b` aggregate, `c` derivation DAG    |

### Examples

```sh
# round-trip the bundled Zebra puzzle through the canonical printer
ein-bot ir parse examples/zebra.ein | head -20

# render it as an SVG
ein-bot ir dot examples/zebra.ein | dot -Tsvg -o /tmp/zebra.svg

# from Python — typed AST
python -c '
from pathlib import Path
from ein_bot.ir import parse, dump_canonical
forms = parse(Path("examples/zebra.ein").read_text())
print(len(forms), "top-level forms")
for f in forms:
    print(" ", f.head.name, "→", len(f.args), "children")
'
```

### IR file format

S-expression intermediate representation; the grammar is
`ein.py/src/ein_bot/ir/grammar.lark` and the spec is
[`docs/kernel/`](docs/kernel/) (graph semantics, data model, surface
language, inference engine — split per
[`docs/kernel/README.md`](docs/kernel/README.md)). Six top-level forms:

| head          | role (block name == provenance layer)                                |
|---------------|----------------------------------------------------------------------|
| `ontology`    | **implicit** assumptions: schema + reader-supplied context           |
| `facts`       | **explicit** problem statements (numbered conditions, `:source "(N)"`) |
| `reasoning`   | **derived** facts — engine working memory after a solve               |
| `rules`       | inference-rule definitions                                            |
| `query`       | what to ask the engine (`:mode` solve/gaps/contradictions)            |
| `trace`       | engine output — derivation log + branches                             |

Kernel meta-primitives (`=`, `instance`, `not`, `and`, `or`, `neq`)
are shape-pinned reserved words: wrong arity is a parse error.

### Development loop

```sh
pytest -q                  # ~200 tests
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

M1 P1.1 (IR language) is complete: grammar, typed AST, parser/dump,
golden snapshot tests, DOT renderer. Next up is P1.2 — the entity-
typed knowledge base (`Type`/`Relation`/`Rule`/`Instance`/`Fact` with
cross-references). The original single-file PoC
([`docs/PoC/reasoning.py`](docs/PoC/reasoning.py)) is preserved for
reference. The deep rewrite is tracked in
[`plans/`](plans/README.md); see [`AGENTS.md`](AGENTS.md) for context
aimed at coding agents.
