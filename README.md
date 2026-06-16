# Ein

Graph-based relation algebra solver — a prototype now being
modernised in light of neuro-symbolic and constrained-reasoning research.

Ein loads a puzzle as a **typed hypergraph** of relations, facts and
rules (written in [ein-lang](docs/kernel/ir/03-ein-lang/), an
S-expression IR), **saturates** the rules to a least fixpoint
(Datalog-style forward chaining), then **searches a commitment lattice**
(CSP/SAT-style branch-and-prune, with ATMS-style provenance and no-good
learning). One run, one verdict — **read from the result**, never chosen by a
mode flag. The count of distinct complete models `k` *is* the answer:

- `k = 1` → **the solution** — a unique complete model (certified unique once
  the search is exhausted).
- `k > 1` → **gaps** — the puzzle is under-determined: `k` distinct models, the
  residual ambiguity.
- `k = 0` → **a contradiction** — an over-constrained KB, reported with a
  minimal unsat core.

`solve` / `gaps` / `contradictions` are **three answers to one problem**, not
three different problem statements and not three commands. You run **`ein
solve`** and read whichever answer the puzzle yields; the stop policy (single /
`--solutions N` / `--exhaustive`) only controls how far the search runs. (An
earlier design split these into three functions that each *chose* their verdict
up front — and so disagreed with each other on the same input; that bug is what
collapsing to one engine fixed.)

Every derived fact carries provenance, so a solve can emit a
self-contained, human-readable markdown derivation trace. The engine's
design — and where each operation sits against the CS literature
(Datalog · CDCL/CSP · ATMS · Apriori) — is mapped in
[`docs/kernel/inference/architecture_and_algorithms.md`](docs/kernel/inference/architecture_and_algorithms.md).

The classic Zebra/Einstein puzzle is the running fixture:

```sh
$ ein solve examples/zebra2.ein
The Norwegian drinks the water; the Japanese keeps the zebra.  (a solution — pass --exhaustive to certify uniqueness)
```

## Layout

| path                          | what's in it                                                                          |
|-------------------------------|---------------------------------------------------------------------------------------|
| `ein.py/`                     | Python implementation (package, tests, pyproject)                                     |
| `ein.py/src/ein/ir/`          | ein-lang IR — Lark grammar, typed AST, parser, canonical dump, DOT renderer           |
| `ein.py/src/ein/kb/`          | typed-entity knowledge base — store + 7 indexes, entities, provenance DAG, imports    |
| `ein.py/src/ein/inference/`   | the engine — saturator, matcher/join-compiler, commitment-lattice search, no-goods, contradiction detector, verdict |
| `ein.py/src/ein/render/`, `trace/` | Graphviz DOT renderers + the markdown derivation-trace builder                   |
| `ein.py/src/ein/stdlib/`      | ein-lang standard library — relation-algebra rules (`closure`, `bijection`, `elim`, `algebra`, `typing`, `macro`) |
| `ein.py/src/ein/cli/`         | console script `ein` — `ir` \| `kb` \| `render` \| `solve` (the one solver command), plus the engine runners `saturate` \| `profile` \| `symmetric` |
| `ein.py/tests/`               | pytest suite (~1,300 tests)                                                           |
| `ein.py/pyproject.toml`       | PEP 621 metadata; deps `numpy`, `lark`; dev extras `pytest`, `pytest-cov`, `ruff`     |
| `examples/zebra.ein`, `zebra2.ein` | the Zebra puzzle as ein-lang; `zebra2.ein` (unified-`is-a` / `*-loc`) is the active acceptance target |
| [`examples/README.md`](examples/README.md) | the Wikipedia human Zebra solution traced step-by-step as Ein inference — **M1 target** for the engine, **M2 target** for the NL ⇄ IR round-trip |
| `examples/{features,branching,saturation,lattice,domain_elim}/` | focused fixtures per engine feature                              |
| `examples/broken/`            | curated parse-failure fixtures (`file:line:col` error messages)                       |
| `plans/`                      | milestone / phase / stage roadmap (M1 active)                                         |
| `docs/kernel/`                | kernel documentation — graph semantics, data model, surface language, inference engine |
| `docs/index/`                 | "awesome-list" catalogue of external tech across 12 topic files + knowledge graph     |
| `docs/ideas/`                 | ideas extracted from research notes                                                   |
| `utils/`                      | renderers for the knowledge graph (Graphviz + Cytoscape) + the VS Code ein-lang grammar + ad-hoc engine probe/measure scripts (moved from `demo/` in P1.11) |
| `nlp/`, `smt/`                | scratch areas for the upcoming rewrite (link-grammar, CVC4 submodules)                |
| `AGENTS.md`                   | guidance for AI coding agents (`CLAUDE.md` is a symlink to it)                         |
| `TODO.md`                     | live worklist                                                                         |

## Quickstart

### Install

```sh
./venv_install.sh                  # creates .venv, installs ein[dev]
# or pin an interpreter:
./venv_install.sh /usr/bin/python3.12
source .venv/bin/activate
```

Needs Python ≥ 3.10. Safe to re-run — an existing `.venv/` is reused. The
console script `ein` lands on the venv's PATH.

### Solve

```sh
ein solve <file>                 # print the solution (or the unsat core)
ein solve <file> --exhaustive    # certify unique / ambiguous / unsat
ein solve <file> --solutions N   # stop after N distinct solutions
ein solve <file> --stats         # + engine counters (k, enterings, layers, wall)
ein solve <file> --trace out.md  # + a self-contained markdown derivation trace (to a file)
```

One command, one sound engine: the verdict is **read from the result** —
`k = 0 / 1 / >1` distinct models is reported as *no solution (with an unsat
core) / the solution / ambiguous (k models)*. There is no mode flag (those are
three answers to one problem, [above](#ein)); the only choice is the **stop
policy** — single (default) / `--solutions N` / `--exhaustive`. Other knobs:
`--max-set-size N` (commitment-set depth cap), `--print-final-state` (dump the
model facts, or the unsat-core facts), and the trace shapers `--relevant`
(goal-relevant slice) / `--reorder` (cluster by target entity) /
`--no-diagrams` — which apply to the `--trace` file.

### Inspect the IR / KB

```sh
ein ir parse <file>     # parse → canonical re-dump (round-trippable; --resolve splices imports)
ein ir lint  <file>     # parse-only check; non-zero exit + file:line:col on error
ein ir dot   <file>     # the parsed IR as Graphviz DOT (one digraph per top-level form)
ein kb dot   <file>     # the loaded KB as one unified DOT graph (--layers, --colour-by)
ein render rules|rule|constraints|lattice <file>   # DOT views of rules / the search lattice
```

All `dot` / `render` commands emit Graphviz to stdout; rasterising to SVG is
a shell concern (see [`utils/render_examples.sh`](utils/render_examples.sh)).

```sh
# render the Zebra KB as an SVG
ein kb dot examples/zebra2.ein | dot -Tsvg -o /tmp/zebra.svg

# from Python — the typed AST
python -c '
from pathlib import Path
from ein.ir import parse, dump_canonical
forms = parse(Path("examples/zebra.ein").read_text())
print(len(forms), "top-level forms")
'
```

### ein-lang at a glance

A `.ein` file is a **flat** sequence of S-expression forms — no block
wrappers (since P1.7c). Each form is classified by its head:

| head                  | role                                                                              |
|-----------------------|-----------------------------------------------------------------------------------|
| `relation`            | declare a typed relation + signature                                              |
| `rule` / `hrule`      | inference / hypothesis rule (`:match` → `:assert`, with `:why`)                   |
| `query`               | what to ask the engine (`:mode solve\|gaps\|contradictions`, `:goal`)             |
| `config`              | engine knobs                                                                      |
| `import` / `macro`    | module include / pattern-macro sugar (P1.8)                                       |
| `trace`               | engine-emitted derivation log (parsed back for rendering)                         |
| *anything else*       | a **fact** (`(is-a …)`, `(right-of …)`, …), layered `ontology`/`fact`/`reasoning` from its provenance (`:source` → fact, `:rule`/`:using` → reasoning, else ontology) |

Kernel meta-primitives (`=`, `instance`, `not`, `and`, `or`, `neq`) are
shape-pinned reserved words: wrong arity is a parse error. The full spec is
[`docs/kernel/`](docs/kernel/README.md) (graph semantics, data model,
surface language, inference engine).

### Development loop

```sh
pytest -q                  # ~1,300 tests
ruff check .               # lint
ruff check . --fix         # auto-fix what's safe
```

## Knowledge graph

The topic files under `docs/index/` are summarised as a single graph in
[`docs/index/knowledge-graph.dot`](docs/index/knowledge-graph.dot). Two views:

```sh
# static SVGs (dot / fdp / sfdp / osage) — requires graphviz
utils/render_knowledge_graph.sh svg all

# interactive Cytoscape.js page — open docs/index/knowledge-graph.cy/index.html
python utils/render_knowledge_graph_cy.py
```

## Status

The **M1 engine runs end-to-end**: ein-lang IR (P1.1), the typed-hypergraph
KB with provenance (P1.2), the saturation engine (P1.3), contradiction
detection (P1.4), the hypothesis loop / commitment-lattice search
(P1.5–P1.5b), DOT + markdown trace rendering (P1.6), the bootstrapped Zebra
solve (P1.7), and ein-lang modules + the relation-algebra stdlib (P1.8) are
in place, with semi-naive saturation for performance (P1.8a). The Zebra
puzzle solves correctly — its solution, its gaps, and its contradiction (on an
over-constrained variant) all read off one sound run; ~1,300 tests are green.

[P1.11](plans/m1_core_graph_reasoning/p1.11_package_restructure/README.md)
package/CLI restructure has shipped: the `ein-bot` → `ein` rename, the
`cli.py` → `cli/` split, and the `demo/` cleanup (durable bench runners
promoted to `ein` subcommands, one-off probes moved to `utils/`). The `search`
and `lattice` runner subcommands were then **merged into one sound `ein
solve`** (one engine, the verdict read from the result), replacing the unsound
`gaps_solve` / `contradictions_solve` entries that chose their verdict by
*which function was called*. Next
milestones are **M2** (NL ⇄ IR round-trip) and **M3** (SMT integration).
The whole roadmap is tracked under
[`plans/`](plans/README.md); see [`AGENTS.md`](AGENTS.md) for orientation
aimed at coding agents.

The end-to-end target — what the engine reproduces by the close of M1, and
what NL ⇄ IR completes by the close of M2 — is annotated step by step in
[`examples/README.md`](examples/README.md): the Wikipedia human-style Zebra
solution, each NL sentence paired with the firing ein rule, branch-depth
labels for the hypothesis points, and the no-good clauses learnt on
contradiction.
