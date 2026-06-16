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
solve · examples/zebra2.ein
──────────────────────────────────────────────────────────────
  solutions (k)   1   (not certified — pass --exhaustive)
  verdict         Solution

  query bindings
    ?h_water    = House-1      ?who_water  = Norwegian
    ?h_zebra    = House-5      ?who_zebra  = Japanese

  query facts                       rendered
    (drink-loc Water House-1)       Water is drunk in House-1
    (nation-loc Norwegian House-1)  the Norwegian lives in House-1
    (pet-loc Zebra House-5)         the Zebra is kept in House-5
    (nation-loc Japanese House-5)   the Japanese lives in House-5

  result
    The Norwegian drinks water in House-1; the Japanese owns zebra in House-5
```

Every word of that answer comes from the **puzzle**, not the engine: each
`(relation … :why "{?1} … {?2}")` template renders a fact, and the
`(query … :goal-text "…")` template renders the headline from the goal
variables. A relation with no `:why` simply prints as its IR s-expression —
there is no built-in relation→verb vocabulary.

## Layout

| path                          | what's in it                                                                          |
|-------------------------------|---------------------------------------------------------------------------------------|
| `ein.py/`                     | Python implementation (package, tests, pyproject)                                     |
| `ein.py/src/ein/ir/`          | ein-lang IR — Lark grammar, typed AST, parser, canonical dump, DOT renderer           |
| `ein.py/src/ein/kb/`          | typed-entity knowledge base — store + 7 indexes, entities, provenance DAG, imports    |
| `ein.py/src/ein/inference/`   | the engine — saturator, matcher/join-compiler, commitment-lattice search, no-goods, contradiction detector, verdict |
| `ein.py/src/ein/render/`, `trace/` | Graphviz DOT renderers + the markdown derivation-trace builder                   |
| `ein.py/src/ein/stdlib/`      | ein-lang standard library — relation-algebra rules (`closure`, `bijection`, `elim`, `algebra`, `typing`, `macro`) |
| `ein.py/src/ein/cli/`         | console script `ein` — `render` \| `saturate` \| `solve` (the operational commands; `ir`/`kb` removed, `profile`/`symmetric` → `utils/` scripts) |
| `ein.py/tests/`               | pytest suite (~1,300 tests)                                                           |
| `ein.py/pyproject.toml`       | PEP 621 metadata; deps `numpy`, `lark`; dev extras `pytest`, `pytest-cov`, `ruff`     |
| `examples/zebra.ein`, `zebra2.ein` | the Zebra puzzle as ein-lang; `zebra2.ein` (unified-`is-a` / `*-loc`) is the active acceptance target |
| [`examples/README.md`](examples/README.md) | catalog of the example fixtures — one-line description per file / sub-dir |
| `examples/{features,branching,saturation,lattice,domain_elim}/` | focused fixtures per engine feature                              |
| [`docs/kernel/inference/zebra_walkthrough.md`](docs/kernel/inference/zebra_walkthrough.md) | the Wikipedia human Zebra solution traced step-by-step as Ein inference — **M1 target** for the engine, **M2 target** for the NL ⇄ IR round-trip (moved here from `examples/README.md`) |
| `examples/broken/`            | curated parse-failure fixtures (`file:line:col` error messages)                       |
| `plans/`                      | milestone / phase / stage roadmap (M1 active)                                         |
| [`docs/guide/`](docs/guide/)  | **start here** — *Learn Ein by solving the Zebra puzzle*, a from-zero tutorial (objects → rules → full solve) |
| `docs/kernel/`                | kernel documentation — graph semantics, data model, surface language, inference engine |
| `docs/lib/`                 | "awesome-list" catalogue of external tech across 12 topic files + knowledge graph     |
| `plans/ideas/`                | ideas extracted from research notes (moved from `docs/ideas/`)                        |
| `utils/`                      | renderers for the knowledge graph (Graphviz + Cytoscape) + the VS Code ein-lang grammar + ad-hoc engine probe/measure scripts (moved from `demo/` in P1.11) |
| `nlp/`, `smt/`                | scratch areas for the upcoming rewrite (link-grammar, CVC4 submodules)                |
| `AGENTS.md`                   | guidance for AI coding agents (`CLAUDE.md` is a symlink to it)                         |
| `TODO.md`                     | live worklist                                                                         |

## Quickstart

**New to Ein?** Start with the **[tutorial](docs/guide/)** — *Learn Ein by
solving the Zebra puzzle* — then come back here to install and run.

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

### Render (DOT) + inspect from Python

```sh
ein render rules|rule|constraints|lattice <file>   # DOT views of rules / the search lattice
```

`render` emits Graphviz to stdout; rasterising to SVG is a shell concern (see
[`utils/render_examples.sh`](utils/render_examples.sh)). The IR / KB themselves
are inspected from Python — the `ir parse|lint|dot` and `kb dot` subcommands
were removed, but their renderers stay at `ein.ir.to_dot` /
`KnowledgeBase.to_dot`:

```sh
# render the Zebra KB as an SVG (the package renderer; was `ein kb dot`)
python -c 'import sys; from pathlib import Path
from ein.ir import parse; from ein.kb import KnowledgeBase
p = Path("examples/zebra2.ein")
sys.stdout.write(KnowledgeBase.from_ir(parse(p.read_text()), base_dir=p.parent).to_dot())' \
  | dot -Tsvg -o /tmp/zebra.svg

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

The topic files under `docs/lib/` are summarised as a single graph in
[`docs/lib/knowledge-graph.dot`](docs/lib/knowledge-graph.dot). Two views:

```sh
# static SVGs (dot / fdp / sfdp / osage) — requires graphviz
utils/render_knowledge_graph.sh svg all

# interactive Cytoscape.js page — open docs/lib/knowledge-graph.cy/index.html
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
[`examples/README.md`](docs/kernel/inference/zebra_walkthrough.md): the Wikipedia human-style Zebra
solution, each NL sentence paired with the firing ein rule, branch-depth
labels for the hypothesis points, and the no-good clauses learnt on
contradiction.
