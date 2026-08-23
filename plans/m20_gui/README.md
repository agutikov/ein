# M20 — GUI

**Estimate:** TBD.
**Status:** **placeholder with the stack decided** — renumbered from **M1b**
2026-08-23, which is also when it stopped claiming a slot: it depends on
[M1a](../m1a_rust/README.md) and blocks nothing, so it runs when someone
wants it rather than at a fixed point in the sequence. The views below are
the 2026-05 sketch; § Stack is the 2026-08-18 decision (merged from this
directory's `tauri.md`, now folded in here).
**Depends on:** [M1a](../m1a_rust/README.md) — the GUI binds to *the
engine that ships*, and after the port that engine is a set of Rust
crates. M1's kernel, IR parser, search-tree artefact and DOT rendering
hooks are the semantics it displays.
**Blocks:** nothing on the critical path; M2 (NL → IR) can proceed in
parallel.

## Goal

A graphical front-end for inspecting ein puzzles and the engine's
reasoning. Not a runtime requirement — the engine is fully
operable from the CLI through M1 — but a substantial
productivity multiplier for puzzle authoring, debugging, and
trace-quality review.

The TUI / CLI surface (`ein solve`, `ein saturate`, DOT dumps, markdown
traces) covers the *machine-readable* output. M20 owns the
*human-readable* interactive view.

---

## Stack

Decided 2026-08-18. The premise that settles it: **after M1a the engine
is Rust**, so a GUI written against a Rust host links the engine as
ordinary crates — no C ABI, no subprocess, no localhost server.

| layer | choice |
|---|---|
| desktop shell | **Tauri 2** |
| backend / semantic model | **Rust** — the existing `ein-*` crates |
| frontend | **TypeScript + React + Vite** |
| View 1 (code) | **Monaco Editor** |
| View 2 (graph) | **Cytoscape.js** + `cytoscape-fcose` |
| View 3 (branches) | React virtualized tree + an SVG/graph renderer |
| application state | **Rust `Session` is the source of truth**; **Zustand** holds UI projection only |
| IPC | Tauri **commands** (request/response) + **channels/events** (async) |
| persistence | `.ein` — semantic, authoritative · `*.layout.json` — purely visual |
| auto-layout | Graphviz `dot`/`fdp`/`sfdp`/`osage` (shelled out from Rust, `-Tjson`) + fCoSE (in the WebView) |
| packaging | Tauri bundler |

```text
┌───────────────────────────────────────────────┐
│                Tauri desktop app              │
│  WebView                                      │
│  ┌──────────────┬──────────────┬────────────┐ │
│  │ Monaco       │ Cytoscape.js │ SearchTree │ │
│  │ ein-lang     │ ein-graph    │ DAG/tree   │ │
│  └──────────────┴──────────────┴────────────┘ │
│             TypeScript / React                │
│                     │                         │
│               Tauri invoke/events             │
│                     │                         │
│              Rust application layer           │
│                     │                         │
│           ein-core / ein-ir / ein-infer       │
│                     │                         │
│        parser / saturator / solver / IR       │
└───────────────────────────────────────────────┘
```

Tauri is **not a UI framework** here. It is a native application host, a
Rust ↔ JS bridge, and a packaging/security boundary: the UI is HTML/JS in
the system WebView (WRY) inside a window (TAO), and the backend process
*is* Rust ([architecture][tauri-arch], [commands][tauri-commands]).

### Why not Electron, why not Qt

- **Electron** would insert Node.js and a bundled Chromium between a Rust
  engine and a React UI that neither needs. With Tauri the chain is
  `ein crates → Tauri Rust → invoke → React`; npm/pnpm is a frontend
  *build* dependency, and the production architecture has no Node in it.
- **Qt/QML** only makes sense if the answer to "Monaco and Cytoscape.js"
  is "don't". Keeping them under Qt means embedding the same web stack
  Tauri wraps far more naturally, and re-crossing a `Rust → C ABI → Qt`
  seam for every call. Qt stays a real option *only* for a different
  program: native rendering, native widgets, no web components.
- The old open question — "Qt vs Electron vs browser vs Jupyter" — was
  therefore the wrong axis. The live one was **Tauri + web frontend vs
  pure browser frontend**, and Tauri wins it on filesystem, native
  dialogs and menus, real windows and shortcuts, packaging as an
  executable, and direct engine calls instead of HTTP/WebSocket
  ([dialog / fs plugins][tauri-plugins]).

### The caveat

Tauri uses the *platform* WebView (WebView2/Edge on Windows —
[a prerequisite][tauri-prereq]), so the rendering engine is not identical
across operating systems the way a bundled Chromium is. For an IDE-shaped
UI of Monaco + Cytoscape + React + CSS grid that is acceptable, but
Linux/Windows/macOS testing is a real line item, not a formality.

### Consequence for the engine

**The surface it calls now has a page**: [`docs/api/rust.md`](../../docs/api/rust.md)
(M1a [S1a.9.4](../m1a_rust/p1a.9_release/s1a.9.4_documentation.md)) — the five
steps, which crate owns each, and a worked example that is a test the gate
runs. It is written for exactly this consumer, and M20 is the first of its
three.

The Tauri backend calls the engine's public Rust API in-process. That is
the reason M1a **dropped server mode** (2026-08-18): the GUI was the
server's first real client, and it turned out not to want one. See
[M1a § Non-goals](../m1a_rust/README.md#non-goals) and
[Q-M1a.11](../m1a_rust/open_questions.md#q-m1a11--server-wire-protocol).
For saved sessions the GUI can use `.einb`
([P1a.8](../m1a_rust/p1a.8_binary_container/README.md)) instead of
re-parsing.

---

## Where the boundary is

**Rust owns the semantics; TypeScript owns the presentation.** JS must
never learn what

```ein
f -0-> a
f -1-> b
```

*means*. It knows `GraphNode`, `GraphEdge`, `SourceRange`, `StateId`,
`RuleId`; Rust knows `EinGraph`, `Relation`, `Fact`, `Rule`,
`SearchState`, saturation. Otherwise the UI becomes a second
implementation of ein.

Two rules follow.

**1. The `Session` lives in Rust.** Source, IR, KB, SearchTree and edit
history are held by the backend behind a `SessionId`; the frontend store
is a projection/cache plus pure UI state (selection, layout mode, pane
sizes). *This closes the "state sync model" open question below.*

**2. Never ship the KB across the bridge.** No 20 MB JSON round-trip on
every edit. The frontend asks for **view models** of the state it is
showing:

```ts
invoke("graph_for_state", { sessionId, stateId, mode: "levi" })
// → { nodes: [...], edges: [...] }
```

Editing follows the same shape: Cytoscape does **not** mutate a KB, it
sends an intent (`add_relation { type, args }`), Rust applies it, dumps
via `to_ir()`, re-parses, validates, and returns a new revision plus
source/graph patches — which is exactly the acceptance criterion
"round-trip through the IR parser to keep the file authoritative".

### GUI API — commands and events

Worth formalising early, because it survives the later shift to live mode.

| direction | surface |
|---|---|
| frontend → Rust (commands) | `open_puzzle` · `save_puzzle` · `select_state` · `get_state_ir` · `get_state_graph` · `edit_source` · `add_fact` · `remove_fact` · `add_relation` · `remove_relation` · `run_layout` · `save_layout` · `undo` · `redo` |
| Rust → frontend (events) | `session-changed` · `parse-error` · `state-changed` · `search-tree-changed` |

Bulk graph data goes over `command → result`; progress and live-mode
streams go over channels/events. When real-time engine integration
arrives (currently deferred, below), the architecture does not change —
the engine's `--events` stream ([`docs/kernel/inference/events.md`](../../docs/kernel/inference/events.md))
is already the narration format to fan out.

---

## Views

Three principal views, composable into 2- or 3-pane layouts.

### View 1 — ein-lang code (Monaco)

- **Source pane**: the puzzle as authored (`.ein` file).
- **Generated-states pane**: the post-saturation IR for any
  reasoning step, dumped via the round-trip `to_ir()` path.
- Monaco brings syntax highlighting, bracket matching, diagnostics,
  hover, go-to-definition, selection ranges, folding and a diff view for
  free — the web stack's clearest win over native widgets.
- **The payoff is text ↔ graph linking.** Rust already carries spans, so
  a `FactDto { id, span }` becomes a Monaco range: click a graph edge →
  `FactId` → `revealRange`; move the cursor → source position → `FactId`
  → Cytoscape highlight. Likely the single most useful feature in the GUI.

### View 2 — ein-graph (Cytoscape.js + fCoSE)

- **Unified vs separate parts**: render the whole graph or focus on one
  population — given (`:source`), derived (`:rule`), background, or the
  rules themselves, which is what the provenance already partitions —
  mirrors the existing DOT renderings but interactive.
- **Compact vs detailed (Levi-bipartite) view**: toggle between
  the abstract entity view (instances + arrows) and the underlying
  Levi-bipartite graph (relation nodes as first-class vertices).
  See [ein model §3](../../docs/kernel/ir/01-ein-graph/03_ein_model.md#3-two-flavours-of-node).
- **Auto-layout**: keep the Graphviz engines (`dot` / `fdp` / `sfdp` /
  `osage`) that `utils/render_knowledge_graph.sh` uses today — Rust
  shells out and hands coordinates to the frontend — and add **fCoSE**,
  computed inside the WebView ([Cytoscape.js][cytoscape]).
- **Manual layout**: drag nodes; persist across reloads. Prefer saving
  it **semantically** rather than as raw XY — fCoSE supports fixed-node,
  alignment and relative-placement constraints, so a saved layout can say
  *"`person` is pinned, the houses align vertically, ontology sits above
  facts"* and let everything else re-layout around it:

  ```json
  {
    "fixed":         { "person": [500, 300] },
    "alignVertical": [["house1", "house2", "house3"]],
    "relative":      [["ontology", "above", "facts"]]
  }
  ```

- **GUI editor**: add / remove facts, relations, rules graphically;
  every edit is an intent to Rust, round-tripped through the IR parser.

### View 3 — branches (search tree)

The SearchTree proof artefact, rendered as either:

- **Git mode** — DAG bottom-to-top, branches as dead-ends, the
  surviving chain as `main`. Reads like a commit graph. SVG/React or a
  graph library — *not* Cytoscape.
- **Folders-tree mode** — top-to-bottom, hierarchical, an ordinary
  virtualized tree. Each "folder" is a state == ein-lang snapshot +
  graph view.

Two renderers, **one DTO** and one identity — `StateId`. All
cross-view synchronisation runs through it:

```text
click branch node → StateId(472) ──┬──→ Monaco: to_ir(state 472)
                                   └──→ Cytoscape: graph(state 472)
```

Both modes support **collapse branches** (hide dead sub-trees) and
**collapse chains** (straight saturation runs → a single edge).

## Layout modes

- **2-pane** — left/right split. Typical: branches tree on left,
  lang+graph tabs on right.
- **3-pane** — all three views simultaneously. Typical for
  trace-debugging: lang | graph | branches.

---

## Workspace layout

The GUI is a sibling of the CLI in the same Rust workspace, depending on
the same crates ([design/12](../m1a_rust/design/12_toolchain_and_layout.md) §2):

```text
ein.rs/
├── crates/
│   ├── ein-core/  ein-ir/  ein-infer/  ein-render/
│   └── ein-cli/
└── gui/
    ├── src/                    # TypeScript frontend
    │   ├── components/
    │   ├── views/              # LangView.tsx · GraphView.tsx · BranchView.tsx
    │   └── stores/session.ts
    ├── src-tauri/
    │   └── src/                # lib.rs · commands.rs · session.rs
    └── package.json
```

**No separate GUI-API crate at M20** unless one earns its place — the
Tauri layer uses the engine's public Rust API directly.

A later split is worth designing *toward*, not building yet: a shared
`ein-ui` package consumed by both a Tauri app and a browser app, where
the browser build opens pre-saved artefacts (`.ein` + `search-tree.json`
+ `states.json`) read-only and the Tauri build gets the editable,
run-capable backend. That maps exactly onto the phasing already in § Out
of scope: **first cut is load-saved-artefact; live mode lands when
there's a use case.**

## Out of scope (deferred)

- Real-time engine integration (run-and-watch) — first cut is
  load-saved-artefact; live mode lands when there's a use case, and the
  commands/events split above is what makes it a non-migration.
- Multi-puzzle workspace — single-file load is fine for M20's
  ergonomic-multiplier framing.
- Authoring shortcuts beyond round-trip parse — power-user
  features (refactoring, code-mod) wait for usage signal.
- A browser-hosted build. Designed toward (see § Workspace layout), not
  shipped at M20.

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
  IR + graph in Views 1 + 2 update — one `StateId`, three panes.
- Selecting a fact in Monaco highlights it in Cytoscape, and vice versa.
- Manual graph layouts persist across reloads, as constraints where the
  layout is expressible that way.

Stack-specific:

- The app bundles and runs on Linux, Windows and macOS, with the WebView
  differences ruled out by testing rather than assumed away.
- No engine semantics are implemented in TypeScript: the frontend's only
  ein-shaped types are ids, spans and view models.

## Open questions

- ~~**Stack choice** — desktop (Qt / Tk / Electron) vs browser vs Jupyter
  widgets.~~ **Closed 2026-08-18: Tauri 2 + React + Monaco + Cytoscape**,
  engine linked as Rust crates. See § Stack.
- ~~**State sync model** — is the `.ein` file or the in-memory KB the
  source of truth during a session?~~ **Closed 2026-08-18: the Rust
  `Session` is**, with `.ein` as the authoritative *file* format and
  every edit round-tripped through the parser. Undo/redo is therefore
  backend edit history, not a frontend stack.
- **Trace integration with View 3** — the markdown trace and the
  SearchTree DAG are two renderings of the same artefact; should View 3
  toggle between them, or show side-by-side? *(Still open.)*
- **Frontend framework** — React is the default for the IDE-shaped
  component ecosystem; Svelte would serve equally well. Not load-bearing,
  and cheap to revisit before the first stage.
- **Layout persistence granularity** — per-puzzle sidecar
  (`zebra2.ein.layout.json`) vs a `.ein/layouts/` directory. Cosmetic,
  decide at implementation.

## Cross-links

- [M1a — Rust port](../m1a_rust/README.md) — the engine this binds to;
  [design/12 § workspace](../m1a_rust/design/12_toolchain_and_layout.md)
  for the crate layout, [P1a.8](../m1a_rust/p1a.8_binary_container/README.md)
  for `.einb` saved sessions.
- [`docs/kernel/`](../../docs/kernel/README.md) — the semantics the views
  render; [ein model §3](../../docs/kernel/ir/01-ein-graph/03_ein_model.md#3-two-flavours-of-node)
  for compact vs Levi-bipartite.
- [`utils/render_knowledge_graph_cy.py`](../../utils/render_knowledge_graph_cy.py)
  — the existing browser-Cytoscape renderer; closest existing point to
  View 2, and the reason fCoSE is already familiar here.
- [docs/lib/08 — diagramming / visualization libraries](../../docs/lib/08-diagramming-visualization-libraries.md)
  — the catalogue Cytoscape.js and Graphviz were picked from.
- Tauri: [architecture][tauri-arch] · [calling Rust from the frontend][tauri-commands]
  · [dialog plugin][tauri-plugins] · [prerequisites / WebView2][tauri-prereq]
  · [Cytoscape.js][cytoscape].

[tauri-arch]: https://v2.tauri.app/concept/architecture/
[tauri-commands]: https://v2.tauri.app/develop/calling-rust/
[tauri-plugins]: https://v2.tauri.app/plugin/dialog/
[tauri-prereq]: https://v2.tauri.app/start/prerequisites/
[cytoscape]: https://js.cytoscape.org/
