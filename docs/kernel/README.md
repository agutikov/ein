# Ein kernel — documentation

The **kernel** is the part of Ein that's locked down by M1: the
graph it reasons over, the data structures that hold the graph in
memory, the surface language users write, and (placeholder, P1.3)
the inference engine that fires rules.

Everything above the kernel — NL → IR (M2), SMT slice (M3), the
self-modifying constraint language (followup F2) — *consumes* the
kernel. Everything below — the Python implementation in
`src/ein/` — *implements* the kernel. This tree is the contract
between them.

## Reading order

The four sub-trees layer on each other:

1. **[`ir/01-ein-graph/`](ir/01-ein-graph/)** — the **semantics**.
   What Ein *reasons about*: nodes, edges, hyperedges, rewrite
   rules. No syntax, no Python — pure graph theory tailored to the
   project's needs. Read this first to understand what the system
   thinks in.

2. **[`ir/02-data-model/`](ir/02-data-model/)** — the **in-memory
   representation**. The Python dataclasses (`Type`, `Instance`,
   `Relation`, `Rule`, `Fact`, `Pattern`, `Provenance`, …) that hold
   the graph; the `KnowledgeBase` store with its registries, reverse
   indexes, layer views, hypothesis forks, derivation DAGs. Maps the
   semantics in (1) onto concrete code shapes.

3. **[`ir/03-ein-lang/`](ir/03-ein-lang/)** — the **surface
   syntax**. The S-expression IR that users author and the engine
   dumps. Lexical rules, six top-level forms (`ontology`, `facts`,
   `reasoning`, `rules`, `query`, `trace`), the pattern sub-language,
   worked examples, and DOT rendering. Most of the historical
   `docs/ir.md` lives here.

4. **[`inference/`](inference/)** — the **rule firing engine**.
   Stub before P1.3. Becomes the pattern matcher, saturation loop,
   hypothesis branching, contradiction analysis, and trace
   generation. The substrate is the data model (2); the language to
   define rules is (3); the engine is described here.

The order is also the order of **conceptual precedence**: the graph
is canonical (see [feedback memory `graph-canonical`](../../../.claude/projects/-home-user-work-ein/memory/feedback_graph_canonical.md)
in your local memory store) — the data model and the syntax are
*views* of it, the engine *transforms* it.

## What's M1 vs later milestones

This tree describes the **M1 kernel** — what's locked down for the
Zebra-acceptance milestone.

- `01-ein-graph` is stable: graph + 3 rule families.
- `02-data-model` is stable through M1; F4 promotion targets
  (compound node kinds, e-graph) are noted at the seams.
- `03-ein-lang` is stable; the IR-encoding final call (classic
  `(type …)`/`(instance …)` vs unified `is-a`) is **explicitly
  deferred to P1.7 S1.7.2** — both encodings stay valid through every
  M1 stage.
- `inference/` is documented:
  [`architecture_and_algorithms.md`](inference/architecture_and_algorithms.md)
  (as-built O1–O9) + [`python_impl.md`](inference/python_impl.md) (module map).
  The engine shipped P1.3–P1.5b.

## Audience & reading paths

Each page leans **user** (puzzle authors) or **dev** (engine
contributors); some serve both. The dev-only pages carry an
explicit audience banner.

| audience | pages |
|----------|-------|
| **newcomer** | [`../guide/`](../guide/) — *Learn Ein by solving the Zebra puzzle*, a from-zero tutorial. Start here if you're new; it links into the pages below as you go. |
| **user** | `ir/01-ein-graph/` (semantics); `ir/03-ein-lang/` (the language — grammar, patterns, `06_reserved_names` kernel-API + card, `07_stdlib_api`); `ir/02-data-model/{01_entities,02_store}` (the abstract model) |
| **dev**  | `ir/02-data-model/03_python_impl.md`; `inference/python_impl.md`; `inference/architecture_and_algorithms.md`; [`architecture.md`](architecture.md) |
| **embedder** | [`../api/`](../api/) — the Python embedding contract ([`ein.md`](../api/ein.md) + per-module `ir`/`kb`/`inference`/`trace` pages). Driving Ein *as a library*, distinct from authoring puzzles (user) or changing the engine (dev). |
| **both** | this README, [`glossary.md`](glossary.md), the per-subtree READMEs |

- **Newcomer path** (never seen Ein): the [guide](../guide/) end-to-end
  (Ch.1 → Ch.4), then the user path below for depth.
- **User path** (author a puzzle): glossary → `01-ein-graph` →
  `03-ein-lang` (grammar → patterns → `06_reserved_names` →
  `07_stdlib_api`) → `02-data-model/01_entities`.
- **Dev path** (change the engine): the user path, then
  `architecture.md` → `02-data-model/03_python_impl` → `inference/`
  (`architecture_and_algorithms` → `python_impl` → the README invariants).
- **Embedder path** (call Ein from Python): [`../api/ein.md`](../api/ein.md)
  (the five-step flow + worked example), then the per-module pages as
  needed; `01-ein-graph` + `03-ein-lang` for the puzzles you load.

## Cross-references

- **Glossary**: [`glossary.md`](glossary.md) — definitions for terms
  this tree uses with technical meaning (homoiconic, Levi-bipartite,
  T1/T2/T3 rules, ATMS, e-graph, encoding-agnostic, …).
- **Architecture**: [`architecture.md`](architecture.md) — the
  structural "where does X live?" map: data-flow, package
  dependencies, milestone boundaries, and a change cookbook.
- Plans roadmap: [`plans/m1_core_graph_reasoning/`](../../plans/m1_core_graph_reasoning/).
- Ideas (the user's framing of the project's *goals*): [`plans/ideas/`](../../plans/ideas).
- External tech index: [`docs/lib/`](../lib/).
- Source of truth for parsing: [`ein.py/src/ein/ir/grammar.lark`](../../ein.py/src/ein/ir/grammar.lark).
- Source of truth for the KB: [`ein.py/src/ein/kb/`](../../ein.py/src/ein/kb/).
- **End-to-end target trace**:
  [`inference/zebra_walkthrough.md`](inference/zebra_walkthrough.md) — the human
  Wikipedia Zebra solution annotated as ein.py inference (NL ↔ ein
  rule ↔ branch-depth, contradictions, learnt no-goods). The
  *inference* column is what the M1 kernel + engine must reproduce;
  the *whole row* (NL ⇄ IR ⇄ solution ⇄ NL explanation) is what M2
  closes.

## Conventions

- All ein code blocks use ```` ```lisp ```` (the IR is an
  S-expression dialect). Graphviz dumps use ```` ```dot ````.
- ASCII / box-art diagrams sit alongside DOT examples for inline
  reading.
- File numbers (`01_`, `02_`, …) indicate intended reading order
  within a directory; they're stable.
- Cross-references inside the kernel tree use relative paths that
  resolve regardless of repo root.
