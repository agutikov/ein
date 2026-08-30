# Ein kernel — documentation

The **kernel** is the part of Ein that's locked down by M1: the
graph it reasons over, the data structures that hold the graph in
memory, the surface language users write, and the inference engine
that fires rules.

Everything above the kernel — NL → IR (M2), the GUI (M20), the
self-modifying constraint language (followup F2) — *consumes* the
kernel. Below it there is one implementation, [`ein.rs`](../../ein.rs/)
(M1a). This tree is the contract between them.

> **Since M1a [P1a.10](../history/m1a_rust/README.md#p1a10--one-implementation)
> this tree is the only statement of intent that is not also the
> implementation.** It was written when there were two engines and a harness
> that checked they agreed; now a claim here is checked by
> `cargo test --workspace` and by nothing else. Behaviours that used to be
> defined by "whatever the Python engine does" are stated in
> [`defined_behaviour.md`](defined_behaviour.md), and what it takes to call a
> claim here settled is [`standard_of_proof.md`](standard_of_proof.md) — two
> rules, ratified 2026-08-28.

## Reading order

The four sub-trees layer on each other:

1. **[`ir/01-ein-graph/`](ir/01-ein-graph/)** — the **semantics**.
   What Ein *reasons about*: nodes, edges, hyperedges, rewrite
   rules. No syntax, no code — pure graph theory tailored to the
   project's needs. Read this first to understand what the system
   thinks in.

2. **[`ir/02-data-model/`](ir/02-data-model/)** — the **in-memory
   representation**. The entity kinds (`Relation`, `Rule`, `Fact`,
   `Pattern`, `Provenance`, …) that hold the graph; the knowledge-base
   store with its registries, indexes, the fact view, hypothesis forks,
   derivation DAGs. Maps the semantics in (1) onto concrete shapes.

3. **[`ir/03-ein-lang/`](ir/03-ein-lang/)** — the **surface
   syntax**. The S-expression IR that users author and the engine
   dumps. Lexical rules, the **flat sequence of forms** classified by
   head (P1.7c removed the `(ontology …)` / `(facts …)` /
   `(reasoning …)` / `(rules …)` block wrappers, and the closed
   declarator set is `relation` · `rule` · `hrule` · `query` ·
   `config` · `macro` · `import`, plus the engine-emitted `trace`;
   **any other head is a fact**), the pattern sub-language, worked
   examples, and DOT rendering. Most of the historical `docs/ir.md`
   lives here.

4. **[`inference/`](inference/)** — the **rule firing engine**: the
   pattern matcher, the saturation loop and its NAF boundary,
   hypothesis generation and the commitment-lattice search,
   contradiction analysis, the obligation pass, and trace generation.
   It shipped across P1.3–P1.6. The substrate is the data model (2);
   the language to define rules is (3); the engine is described here.

The order is also the order of **conceptual precedence**: the graph is
canonical — the data model and the syntax are *views* of it, the engine
*transforms* it.

## What's M1 vs later milestones

This tree describes the **M1 kernel** — what's locked down for the
Zebra-acceptance milestone.

- `01-ein-graph` is stable: graph + 3 rule families.
- `02-data-model` is stable through M1; F4 promotion targets
  (compound node kinds, e-graph) are noted at the seams.
- `03-ein-lang` is stable; the IR-encoding final call (one generic link
  relation vs typed attribute relations) was **explicitly deferred to
  P1.7 S1.7.2** and stays deferred on purpose — the two are *two
  ontologies for one puzzle*, both valid ein-lang and, since S1.22.1a,
  both solving to the same model. Keeping the pair is how the project
  tells which of the engine's reasoning power is general and which is an
  artefact of one encoding.
- `inference/` is documented:
  [`architecture_and_algorithms.md`](inference/architecture_and_algorithms.md)
  (as-built O1–O9) + [`implementation.md`](inference/implementation.md) (module map)
  + [`absent_semantics.md`](inference/absent_semantics.md) (the normative
  `(absent P)` / NAF semantics — worlds, fire-time evaluation, corollaries;
  P1.21 R4). The engine shipped P1.3–P1.5b.

## Which pages to trust — the three states

Every page in this tree is in exactly **one** of three states, and the state is
visible from the page itself. Triaged page by page at M1e
[S1e.2.2](../../plans/m1e_review_processing/p1e.2_high/s1e.2.2_code_doc_consistency.md)
(2026-08-30), 40 pages:

- **current** — every claim holds of the engine that ships. No banner; this is
  the default and 37 of the 40 pages are here. What makes it checkable rather
  than aspirational is the test in § *Keeping this true* below: every code
  identifier the page names resolves in `ein.rs/crates/`, and every environment
  variable and CLI invocation it shows runs.
- **superseded** — the page describes something that was true and is not. It
  keeps a banner at the top saying **what** it described, **when** it stopped
  being true, and **where** the current statement is. Three pages, all of them
  M1 P1.5b's design record:

  | page | describes | current statement |
  |---|---|---|
  | [`inference/algorithm_layer_n.md`](inference/algorithm_layer_n.md) | the three sibling solve entries and the per-candidate flow of the 2026-05 design | [`architecture_and_algorithms.md`](inference/architecture_and_algorithms.md) §2, [`implementation.md`](inference/implementation.md) |
  | [`inference/lattice_diagrams.md`](inference/lattice_diagrams.md) | the same design's search-lattice data model, as diagrams | same, plus [`lattice_dump.md`](inference/lattice_dump.md) for the artifact the search writes |
  | [`inference/parity_baselines.md`](inference/parity_baselines.md) | the tree-vs-monotonic wall-clock table of 2026-05-28 | the `branching` corpus group, swept by `corpus_cli.rs` |

  A superseded page is **not** rewritten to match today's engine. A page
  rewritten that way is neither a record nor a specification, and the design it
  held stops being recoverable — which matters here, because
  [`docs/history/m1a_rust/design/07_search_layer.md`](../history/m1a_rust/design/07_search_layer.md)
  cites the first of the three as the contract the Rust port had to reproduce.
- **moved to [`docs/history/`](../history/README.md)** — for a page that belongs
  to a shipped milestone's record. **A page is moved *into* an existing
  milestone record; it is never made into one.** That is why the three above are
  bannered in place rather than moved: they are M1's, and M1 is the milestone
  `docs/history/README.md` records as having no entry — *"what survived its plan
  tree went to `docs/kernel/inference/` and `plans/followups/` at P1.22"*.

Two pages also carry a *partial* banner, which is the same rule applied below
page granularity: [`ir/03-ein-lang/08_self_describing.md`](ir/03-ein-lang/08_self_describing.md)
marks which of its four levels are operational and which are design, and
[`inference/domain_elim_vs_hypothesis.md`](inference/domain_elim_vs_hypothesis.md),
[`inference/features.md`](inference/features.md) and
[`inference/README.md`](inference/README.md) scope their frozen measurements and
their removed-machinery sections the same way. A page may be current and still
contain a section that is not, provided the section says so where a reader will
meet it.

## Audience & reading paths

Each page leans **user** (puzzle authors) or **dev** (engine
contributors); some serve both. The dev-only pages carry an
explicit audience banner.

| audience | pages |
|----------|-------|
| **newcomer** | [`../guide/`](../guide/) — *Learn Ein by solving the Zebra puzzle*, a from-zero tutorial. Start here if you're new; it links into the pages below as you go. |
| **user** | `ir/01-ein-graph/` (semantics); `ir/03-ein-lang/` (the language — grammar, patterns, `06_reserved_names` kernel-API + card, `07_stdlib_api`); `ir/02-data-model/{01_entities,02_store}` (the abstract model) |
| **dev**  | `ir/02-data-model/03_implementation.md`; `inference/implementation.md`; `inference/architecture_and_algorithms.md`; [`architecture.md`](architecture.md); [`defined_behaviour.md`](defined_behaviour.md) |
| **embedder** | Driving Ein *as a library*, distinct from authoring puzzles (user) or changing the engine (dev). From **Rust**: link the crates — [`../api/rust.md`](../api/rust.md), whose worked example is a test the gate runs. From anywhere else: the `ein` binary plus `--json-summary` / [`--events`](inference/events.md). The five *Python* pages under [`../api/`](../api/) are **history** — the contract of the engine that was, kept whole because [Q-M1a.23](../history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)'s trip-wires would restore it; there is no module to import. |
| **both** | this README, [`glossary.md`](glossary.md), the per-subtree READMEs, and [`configuration.md`](configuration.md) — the `(config …)` flags a puzzle may set, the CLI options and the `EIN_*` environment, with *does it change the answer* on every flag row |

- **Newcomer path** (never seen Ein): the [guide](../guide/) end-to-end
  (Ch.1 → Ch.4), then the user path below for depth.
- **User path** (author a puzzle): glossary → `01-ein-graph` →
  `03-ein-lang` (grammar → patterns → `06_reserved_names` →
  `07_stdlib_api`) → `02-data-model/01_entities`.
- **Dev path** (change the engine): the user path, then
  `architecture.md` → `02-data-model/03_implementation` → `inference/`
  (`architecture_and_algorithms` → `implementation` → the README
  invariants), and `defined_behaviour.md` before you change any output.
- **Embedder path** (call Ein from another program):
  [`../api/rust.md`](../api/rust.md) (the five steps, one worked example,
  then per-area detail), then `01-ein-graph` + `03-ein-lang` for the puzzles
  you load. From a non-Rust program the surface is the binary —
  `--json-summary` and [`--events`](inference/events.md). The Python pages
  beside it are history; read their banner before anything else in them.

## Cross-references

- **Glossary**: [`glossary.md`](glossary.md) — definitions for terms
  this tree uses with technical meaning (homoiconic, Levi-bipartite,
  T1/T2/T3 rules, ATMS, e-graph, encoding-agnostic, …).
- **Architecture**: [`architecture.md`](architecture.md) — the
  structural "where does X live?" map: data-flow, crate
  dependencies, milestone boundaries, and a change cookbook.
- **Configuration**: [`configuration.md`](configuration.md) — every knob, on
  all three surfaces: the 17 `(config …)` flags, the 52 CLI options, the
  `EIN_*` environment, and the precedence between them. Each flag row carries
  its default, what it changes, **whether it changes the answer**, and how far
  it may be depended on. The defaults block is `ein solve --dump-config`'s own
  output; the page is diffed against `FIELDS` by `cargo test`.
- Plans roadmap: [`plans/README.md`](../../plans/README.md) — what has **not**
  been built yet. Two milestones have shipped and left it: M1 (2026-06-17,
  plan folder removed at P1.22, in git history) and M1a (2026-08-23, whose
  record is [`docs/history/m1a_rust/`](../history/m1a_rust/README.md)).
- Ideas (the user's framing of the project's *goals*): [`plans/ideas/`](../../plans/ideas).
- External tech index: [`docs/lib/`](../lib/).
- Source of truth for parsing: [`ir/03-ein-lang/00_ebnf.md`](ir/03-ein-lang/00_ebnf.md)
  — the complete grammar, in EBNF. It was `grammar.lark` until M1a S1a.10.5.
- Source of truth for the KB: [`ein-core`](../../ein.rs/crates/ein-core/src/).
- **Defined behaviour**: [`defined_behaviour.md`](defined_behaviour.md) — the
  thirteen diagnostics, orderings and error strings whose only statement, until
  the second engine left, was a Python source file.
- **End-to-end target trace**:
  [`inference/zebra_walkthrough.md`](inference/zebra_walkthrough.md) — the human
  Wikipedia Zebra solution annotated as ein inference (NL ↔ ein
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
- A claim on these pages is subject to
  [`standard_of_proof.md`](standard_of_proof.md): a behaviour is refuted only
  by a banked probe, and an argument for leaving something as it is holds only
  while its premise is enforced by something that fails when it stops being
  true.
