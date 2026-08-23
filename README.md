# EinAf

[![per-commit](https://github.com/agutikov/ein/actions/workflows/per-commit.yml/badge.svg)](https://github.com/agutikov/ein/actions/workflows/per-commit.yml)
[![nightly](https://github.com/agutikov/ein/actions/workflows/nightly.yml/badge.svg)](https://github.com/agutikov/ein/actions/workflows/nightly.yml)

EinAf — a Neuro-Symbolic Automated Reasoning Framework for Iterative Autoformalization and Theory Synthesis.

TODO: format as table
Autoformalization — translates natural-language or otherwise informal problem statements into formal Ein representations: entities, relations, facts, constraints, rules, and goals.
Theory Library — maintains reusable formal theories, relation properties, reasoning patterns, and domain-specific knowledge that can be instantiated and composed for particular problems.
Theory Selection — identifies and retrieves existing theories, rules, and reasoning patterns relevant to a given problem and its current formalization.
Theory Synthesis — constructs new relations, constraints, rules, and theories when existing knowledge is insufficient, including specialization and composition of existing theories.
Theory Transformation and Specialization — adapts general theories to a particular problem context, derives specialized subtheories, and transforms representations into forms better suited for reasoning.
Symbolic Reasoning Kernel — executes formal reasoning over Ein representations, including saturation, deduction, rule application, constraint propagation, and fixed-point computation.
Hypothesis Search — explores alternative assumptions and candidate models through structured backtracking over the hypothesis lattice.
Constraint and Satisfiability Reasoning — enforces structural and semantic constraints, detects incompatible assignments, and searches for models satisfying the formalized theory.
Contradiction Detection and Analysis — identifies inconsistent states and traces contradictions back to the rules, facts, and hypotheses responsible for them.
Formal Verification — mechanically checks neural-generated formalizations, rules, theories, and candidate solutions against the symbolic semantics of Ein.
Solution and Model Generation — produces satisfying models, solutions, derived facts, proofs, or counterexamples depending on the problem.
Reasoning Introspection — exposes derivation traces, rule dependencies, relation dependencies, hypothesis branches, unsatisfiable cores, and other reasoning artifacts.
Neuro-Symbolic Feedback Loop — feeds symbolic results—solutions, contradictions, incomplete derivations, failed hypotheses, and structural information—back into the neural component to refine the formalization or synthesize/select better theories.

---

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
- `k = 0` → **a contradiction** — an over-constrained KB, reported with its
  unsat core: the smallest set of given facts from which one recorded
  contradiction follows (provenance-based, searched across every recorded
  derivation; not a subset-minimal MUS).

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
(Datalog · ATMS · Apriori — CDCL/CSP as analogs) — is mapped in
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
| `ein.rs/`                     | **the implementation** — a Cargo workspace of seven crates                            |
| `ein.rs/crates/ein-ir/`       | ein-lang IR — lexer, recursive-descent parser, typed AST, canonical dump, imports, macros |
| `ein.rs/crates/ein-core/`     | the value + fact model — interners, the layered KB store and its indexes, provenance   |
| `ein.rs/crates/ein-infer/`    | the engine — saturator, matcher/join-compiler, commitment-lattice search, no-goods, contradiction detector, verdict |
| `ein.rs/crates/ein-render/`   | Graphviz DOT renderers + the markdown derivation-trace builder                          |
| `ein.rs/crates/ein-cli/`      | the `ein` binary — `render` \| `saturate` \| `solve`                                  |
| `ein.rs/crates/{ein-corpus,ein-parity}/` | dev-only: the corpus manifest + fixture helpers, and the narration cut |
| `stdlib/`                     | ein-lang standard library — relation-algebra rules (`closure`, `bijection`, `elim`, `algebra`, `typing`, `macro`). Checked in once; the binary embeds a copy and `MANIFEST.sha256` is what keeps the two the same |
| [`corpus/`](corpus/README.md) | one entry per `.ein` file and the invocations it is exercised under                     |
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
| `nlp/`, `smt/`                | scratch areas — dependency-parsing notes, and three hand-written `.smt` encodings (the link-grammar and CVC4 submodules were deinitialised at M1a S1a.10.5) |
| `AGENTS.md`                   | guidance for AI coding agents (`CLAUDE.md` is a symlink to it)                         |
| [`utils/`](utils/)            | renderers, the VS Code ein-lang grammar, and the M1a measurement set                   |

## Quickstart

**New to Ein?** Start with the **[tutorial](docs/guide/)** — *Learn Ein by
solving the Zebra puzzle* — then come back here to install and run.

### Install

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml
ein.rs/target/release/ein --help
```

Needs a Rust toolchain, and — for the default build — `cmake` and a C++
compiler, because the binary links `snmalloc` (worth 8–16 % of a solve).
`cargo build --release -p ein-cli --no-default-features` builds against the
system allocator and needs neither. Signed binaries for three platforms are
[P1a.9](plans/m1a_rust/p1a.9_release/README.md)'s; **`pip install` is not a
channel** — the Python binding was deferred on 2026-08-21 for want of a
consumer ([Q-M1a.23](plans/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).

> **There was a Python implementation** — `ein.py/`, `./venv_install.sh`, a
> console script on a venv's PATH — and it was the reference for five phases
> of the port. It left the tree at M1a
> [S1a.10.5](plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md);
> `git show two-implementations` is the last revision that had both.

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

### Render (DOT)

```sh
ein render rules|rule|constraints|lattice <file>   # DOT views of rules / the search lattice
```

`render` emits Graphviz to stdout; rasterising to SVG is a shell concern (see
[`utils/render_examples.sh`](utils/render_examples.sh)).

**Two more views exist and have no CLI.** `ein_render::ir_dot` (the IR graph,
five variants) and `ein_render::kb_dot` (the whole KB on one page, six) are
ported, tested over the corpus by `dot_wellformed.rs`, and reachable only as
library calls: `ein ir dot` and `ein kb dot` were removed in P1.7c and never
came back. Until they do, `utils/render_examples.sh` renders what the CLI
renders, and putting them back is a decision about the shipping surface.

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
./run_tests.sh             # the gate: cargo test --workspace, 542 tests, ~1 m
./run_tests.sh --slow      # + the 118 slow corpus cells, + 8 id-space seeds
cd ein.rs && cargo fmt && cargo clippy --workspace --all-targets -- -D warnings
```

The gate needs **Graphviz** on `PATH`: `dot_wellformed.rs` is the only
authority the DOT views have on being well-formed, and it fails rather than
skips without it.

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
over-constrained variant) all read off one sound run.

**M1a rewrote the engine in Rust**, and since
[S1a.10.5](plans/m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
`ein.rs/` is the only implementation: `solve zebra2.ein -e` end-to-end went
from 4.9 s under PyPy to **199 ms**, and the gate from 312 tests in 9 m 13 s
to 542 in about a minute. What the Python engine proved that nothing else did
is banked — [the oracle ledger](plans/m1a_rust/p1a.10_single_implementation/oracle_ledger.md)
is the row-by-row record, including its four accepted losses.

P1.11
package/CLI restructure has shipped: the `ein-bot` → `ein` rename, the
`cli.py` → `cli/` split, and the `demo/` cleanup (durable bench runners
promoted to `ein` subcommands, one-off probes moved to `utils/`). The `search`
and `lattice` runner subcommands were then **merged into one sound `ein
solve`** (one engine, the verdict read from the result), replacing the unsound
`gaps_solve` / `contradictions_solve` entries that chose their verdict by
*which function was called*. In flight now is **M1a** — `ein.rs`, a Rust port
that is a byte-for-byte drop-in for `ein` (P1a.0–P1a.3 shipped); next are
**M1b** (a Tauri GUI over the ported engine) and **M2** (NL ⇄ IR round-trip).
The whole roadmap is tracked under
[`plans/`](plans/README.md); see [`AGENTS.md`](AGENTS.md) for orientation
aimed at coding agents.

The end-to-end target — what the engine reproduces by the close of M1, and
what NL ⇄ IR completes by the close of M2 — is annotated step by step in
[`examples/README.md`](docs/kernel/inference/zebra_walkthrough.md): the Wikipedia human-style Zebra
solution, each NL sentence paired with the firing ein rule, branch-depth
labels for the hypothesis points, and the no-good clauses learnt on
contradiction.
