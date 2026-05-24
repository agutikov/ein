# M1b — GUI

**Estimate:** TBD.
**Status:** **placeholder** — slotted between M1 and M2.
**Depends on:** [M1](../m1_core_graph_reasoning/README.md) — needs
the kernel, IR parser, search-tree artefact, and DOT rendering
hooks to be stable.
**Blocks:** nothing on the M1-M3 critical path; M2 (NL → IR) can
proceed in parallel.

## Goal

A graphical front-end for inspecting ein puzzles and the engine's
reasoning. Not a runtime requirement — the engine is fully
operable from the CLI through M1 — but a substantial
productivity multiplier for puzzle authoring, debugging, and
trace-quality review.

The TUI / CLI surface (`bench_solve`, `bench_saturate`, DOT dumps,
markdown traces) covers the *machine-readable* output. M1b owns
the *human-readable* interactive view.

## Views

Three principal views, composable into 2- or 3-pane layouts.

### View 1 — ein-lang code

- **Source pane**: the puzzle as authored (`.ein` file).
- **Generated-states pane**: the post-saturation IR for any
  reasoning step, dumped via the round-trip `to_ir()` path.

### View 2 — ein-graph

- **Unified vs separate parts**: render the whole graph or focus on
  a single layer (ontology / fact / reasoning / rules) — mirrors
  the existing DOT renderings but interactive.
- **Compact vs detailed (Levi-bipartite) view**: toggle between
  the abstract entity view (instances + arrows) and the underlying
  Levi-bipartite graph (relation nodes as first-class vertices).
  See [ein model §3](../../docs/kernel/ir/01-ein-graph/03_ein_model.md#3-two-flavours-of-node).
- **Auto-layout** with different modes (DOT / FDP / SFDP / OSAGE,
  same engines `utils/render_knowledge_graph.sh` uses today, plus
  fCoSE per the Cytoscape view).
- **Manual layout**: drag nodes; remember positions across reloads
  (per-puzzle saved layout file).
- **GUI editor**: add / remove facts, relations, rules
  graphically; round-trip through the IR parser to keep the file
  authoritative.

### View 3 — branches (search tree)

The [SearchTree](../m1_core_graph_reasoning/p1.5_hypothesis_loop/README.md)
proof artefact, rendered as either:

- **Git mode** — DAG bottom-to-top, branches as dead-ends, the
  surviving chain as `main`. Reads like a commit graph.
- **Folders-tree mode** — top-to-bottom, hierarchical. Each
  "folder" is a state == ein-lang snapshot + graph view.

Both modes support:

- **Collapse branches** — hide dead sub-trees.
- **Collapse chains** — collapse straight saturation runs into a
  single edge.
- Every node/folder is a *state*: clicking opens View 1 + View 2
  for that point in reasoning.

## Layout modes

- **2-pane** — left/right split. Typical: branches tree on left,
  lang+graph tabs on right.
- **3-pane** — all three views simultaneously. Typical for
  trace-debugging: lang | graph | branches.

## Out of scope (deferred)

- Real-time engine integration (run-and-watch) — first cut is
  load-saved-artefact; live mode lands when there's a use case.
- Multi-puzzle workspace — single-file load is fine for M1b's
  ergonomic-multiplier framing.
- Authoring shortcuts beyond round-trip parse — power-user
  features (refactoring, code-mod) wait for usage signal.

## Acceptance (sketch)

Each view individually:

- Loads `examples/zebra2.ein` end-to-end; shows source + graph +
  saved search tree.
- Round-trips edits through the IR parser (View 1 / 2 edits
  produce identical-modulo-formatting `.ein` output).
- The graph view's compact ↔ detailed toggle matches the DOT
  rendering for the same KB state at both granularities.

Composed:

- A user can click a branch node in View 3 and see the matching
  IR + graph in Views 1 + 2 update.
- Manual graph layouts persist across reloads.

## Open questions

- **Stack choice** — desktop (Qt / Tk / Electron) vs browser (the
  Cytoscape.js view already lives in `utils/`) vs Jupyter
  widgets. The browser path reuses the existing Cytoscape work;
  desktop gives faster keyboard-driven editing.
- **State sync model** — is the `.ein` file or the in-memory KB
  the source of truth during a session? Affects what "undo"
  means.
- **Trace integration with View 3** — the markdown trace (P1.6
  S1.6.4) and the SearchTree DAG (S1.6.3) are two renderings of
  the same artefact; should View 3 toggle between them, or
  show side-by-side?

## Cross-links

- [M1 — core graph reasoning](../m1_core_graph_reasoning/README.md)
  — the kernel + artefacts M1b reads.
- [P1.6 rendering + trace](../m1_core_graph_reasoning/p1.6_rendering_and_trace/README.md)
  — the CLI-side rendering pipeline M1b reuses.
- [utils/render_knowledge_graph_cy.py](../../utils/render_knowledge_graph_cy.py)
  — the existing browser-Cytoscape renderer; closest existing
  point to View 2.
- [docs/index/08 — diagramming / visualization libraries](../../docs/index/08-diagramming-visualization-libraries.md)
  — external tech that could host M1b's views.
