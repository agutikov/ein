# P1.10 — Kernel documentation

**Estimate:** TBD per theme.
**Status:** **placeholder** — created 2026-05-24 from the
TODO.md scratchpad. The phase parks kernel-doc reorgs that
emerged after the M1 implementation surface stabilised. None of
these gate M1 acceptance; they make the kernel docs reflect what
M1 actually shipped (and what the user wants the M1 docs to teach
post-hoc).
**Depends on:** [P1.7](../p1.7_bootstrapping_zebra/) ships
("M1 done" — the kernel is locked) so documenting it doesn't
chase a moving target.
**Blocks:** nothing on M1's critical path; informs M1b (GUI) and
M2b (paper) by giving them a stable doc surface to point at.

The existing kernel docs at [`docs/kernel/`](../../../docs/kernel/)
are organised into three sub-trees (`ir/01-ein-graph/`,
`ir/02-data-model/`, `ir/03-ein-lang/`) plus a stub
`inference/`. P1.10 takes that structure further:

- splits data-model into idiomatic vs python-impl,
- adds inference-engine docs to match,
- separates user-facing vs developer-facing layers,
- adds an architecture overview with diagrams,
- renames `docs/index/` → `docs/lib/` for clarity,
- folds in a refined ein-model (atoms vs objects; 4-level
  KB representation).

## Stages

| ID         | Title                                                                       | File                                                                  |
|------------|-----------------------------------------------------------------------------|-----------------------------------------------------------------------|
| S1.10.A1   | IR docs: split data-model into idiomatic + python-impl                      | [s1.10.a1_ir_doc_split.md](s1.10.a1_ir_doc_split.md)                  |
| S1.10.A2   | Idiomatic data-model backfill                                               | [s1.10.a2_idiomatic_backfill.md](s1.10.a2_idiomatic_backfill.md)      |
| S1.10.B    | Kernel API reference                                                        | [s1.10.b_kernel_api_reference.md](s1.10.b_kernel_api_reference.md)    |
| S1.10.C    | Stdlib API reference                                                        | [s1.10.c_stdlib_api_reference.md](s1.10.c_stdlib_api_reference.md)    |
| S1.10.D    | Inference engine documentation                                              | [s1.10.d_inference_engine_docs.md](s1.10.d_inference_engine_docs.md)  |
| S1.10.E    | User vs developer docs split                                                | [s1.10.e_user_vs_dev_split.md](s1.10.e_user_vs_dev_split.md)          |
| S1.10.F    | Architecture overview + diagrams                                            | [s1.10.f_architecture_overview.md](s1.10.f_architecture_overview.md)  |
| S1.10.G    | `docs/index/` → `docs/lib/` rename                                          | [s1.10.g_docs_index_rename.md](s1.10.g_docs_index_rename.md)          |
| S1.10.H1   | Ein-model: atoms vs objects                                                 | [s1.10.h1_atom_vs_object.md](s1.10.h1_atom_vs_object.md)              |
| S1.10.H2   | Ein-model: 4-level KB representation                                        | [s1.10.h2_four_level_kb.md](s1.10.h2_four_level_kb.md)                |
| S1.10.H3   | Ein-model: self-describing KB types                                         | [s1.10.h3_self_describing_kb.md](s1.10.h3_self_describing_kb.md)      |

Suggested order: H1 → H2 → H3 (vocabulary first, then schema)
∥ A1 → A2 (data-model split, then backfill); B + C + D can
follow once A2 + H2 are in; E and F land after the bulk of
content stabilises; G can ship any time before M2b.

## Themes

### Theme A — IR doc 4-level split

Today's [`docs/kernel/ir/`](../../../docs/kernel/ir/) has three
levels:

```
ir/01-ein-graph/   — semantics (the graph)
ir/02-data-model/  — Python dataclasses + KB
ir/03-ein-lang/    — surface S-expression syntax
```

The user's 2026-05-24 framing: split level 2 into **idiomatic
data model** + **python implementation**, giving four:

```
ir/01-ein-graph/        — semantics (unchanged)
ir/02-data-model/       — idiomatic: data types, collections, indexes, …
ir/02b-python-impl/     — Python implementation specifics
ir/03-ein-lang/         — surface syntax (unchanged)
```

Why: the current `02-data-model/` mixes "what abstract shapes the
KB carries" with "how Python encodes them". A reader who wants
just the semantics + abstract data model shouldn't need to skim
`@dataclass` decorators; a reader implementing in another
language wants the abstract shapes without Python noise.

Likely stages:

- **S1.10.A1** — split the existing `02-data-model/` into
  idiomatic (collections, indexes, algorithms over the data) and
  python-impl (modules, classes, file layout).
- **S1.10.A2** — backfill the idiomatic level with mathematical
  pseudocode / diagrams for the index structures.

### Theme B — Kernel API reference

Today the kernel's **primitives and semantically-loaded atoms**
(`rule`, `relation`, `T`, `not`, `eq`, `neq`, `closed`, `absent`,
`forall`, `open`, `false`, `true`, …) are documented piecemeal
across the three sub-trees + the engine source. P1.10 ships a
dedicated reference page that lists each, its arity / signature,
its semantic role, and its rendering in `(rules …)` / `(facts …)`
contexts.

The reference should be **complete enough to author new puzzles
without reading source** — the M1-end-of-life test for a
kernel API doc.

### Theme C — Stdlib API reference

Parallel to Theme B but for the **stdlib** rules and macros
(closure auto-inference, the `imply` family, `converse`, the
general totality / `domain-elimination` library form, the macros
under S1.5.9). Lands once [P1.8 Theme A](../p1.8_ein_lang_modules/README.md)
ships the stdlib content — until then this is just an outline.

### Theme D — Inference engine documentation

Today's [`docs/kernel/inference/`](../../../docs/kernel/inference/)
is a stub. P1.10 fills it with two layers matching Theme A:

- **Idiomatic** — collections (the rule-cache, the activator
  index, the firing queue), indexes (`_facts_by_relation`,
  `_negated_facts`, `_rule_apps_by_rule`), algorithm diagrams +
  mathematical pseudocode for `compile_all` / `compile_for` /
  `_apply` / `saturate` / hypothesis loop.
- **Python implementation** — files, modules (`engine.py`,
  `saturator.py`, `hypgen.py`, `compile.py`, `match.py`,
  `solver.py`), data types, functions, classes.

Composes with the closures-and-saturation invariants surfaced by
[P1.5a](../p1.5a_zebra_solution/README.md) (the NAF-at-enqueue
race in particular needs a written invariant).

### Theme E — User vs developer docs split

The whole `docs/kernel/` tree currently mixes "what users of
ein-lang need to know" with "what implementers of the engine
need to know". Split into:

- **User docs** — ein-lang grammar, semantics, kernel API,
  stdlib API, examples, glossary. The contract for puzzle
  authors.
- **Developer docs** — the python-impl half (split out by
  Theme A), inference-engine python (Theme D), contributor-
  onboarding for engine internals.

Boundary lives roughly at "would a non-Python author care?". If
yes → user docs; if no → developer docs.

### Theme F — Software architecture overview + diagrams

A new high-level architecture page (probably
`docs/kernel/architecture.md`) covering:

- The data-flow diagram: `.ein` source → parser → AST → loader
  → `KnowledgeBase` → saturator + hypgen → trace renderer.
- Module dependencies (`ir/` ↔ `kb/` ↔ `inference/` ↔ rendering).
- Boundary diagrams for the M1/M2/M3 split (which modules each
  milestone adds / depends on).

This is the page someone unfamiliar with the codebase reads first
to know *where to look*. The existing
[`docs/kernel/README.md`](../../../docs/kernel/README.md) is the
reading-order doc; the architecture page is the structural one.

### Theme G — Rename `docs/index/` → `docs/lib/`

Pragmatic rename. The current `docs/index/` is *not* an index — it's
the external-tech library / catalogue (12 thematic files + a
knowledge graph). The name "index" reads as "table of contents"
which mismatches the content. `lib/` (library, in the sense of
"library of references the project draws from") matches the
intent.

Mechanics: `git mv docs/index docs/lib`; rewrite all 100-ish
cross-references that resolve via the path; regenerate the
knowledge-graph SVGs + Cytoscape view from the new location.
Trivial diff, but a *lot* of paths — schedule carefully.

### Theme H — Ein-model update: atoms vs objects, 4 levels

Refinement of [`docs/kernel/ir/01-ein-graph/03_ein_model.md`](../../../docs/kernel/ir/01-ein-graph/03_ein_model.md)
(the reflexive node algebra). Two changes from the user
2026-05-24:

#### H.1 — Differentiate **atoms** and **objects**

> *"atoms are names — `rule`, `not`, `T`, `relation`, `Alice`,
> `co-located` are all atoms. Objects are, as before,
> Levi-bipartite graph nodes without out-arrows and with
> in-arrows from facts."*

The current §2 ("four foundational terms") lists *node, arrow,
object, relation*. The refinement promotes **atom** to a fifth
term, distinguished from **object**:

| term     | refined definition (2026-05-24)                                          |
|----------|--------------------------------------------------------------------------|
| **atom** | a *name* — a lexical token used to identify nodes. `rule`, `not`, `Alice`, `co-located` are all atoms. |
| **object** | a *node* — a graph vertex with no out-arrows and with in-arrows from facts. Identified by an atom. |
| **relation** | (unchanged) a node containing two or more outbound arrows. |
| **node**, **arrow** | (unchanged) |

The relation between atom and object: an atom *labels* an object
node. Two occurrences of the same atom across the KB identify the
same object node (per the identity rule §2). The atom is the
*name*; the object is the *thing named*.

Why it matters: the current model treats `rule`, `relation`,
`not` as atoms-acting-as-relation-heads but doesn't have a clean
word for "the name itself". This refinement gives one. Bears on
F5 (rules-as-data) where atoms-as-first-class become important.

#### H.2 — Declare the 4-level KB representation

> *"4 levels — objects, facts, relations, rules. Idea: create
> self-describing model of KB types in IR ein lang."*

Add a §9 to `03_ein_model.md` (or a sibling doc) laying out the
4-level KB:

| level         | what lives here                                       |
|---------------|-------------------------------------------------------|
| **L0 objects**| Named atoms standing for entities (instances + types). |
| **L1 facts**  | Propositions over objects: `(R o1 o2)`.                |
| **L2 relations** | Declarations + property tags: `(relation R T1 T2)`, `(symmetric R)`, `(functional R)`. |
| **L3 rules**  | First-class rewriting machinery: `(rule NAME ... :match ... :assert ...)`. |

Each level *consumes* the levels below: rules pattern-match facts
and assert facts; relation declarations type the facts; facts name
objects.

The idea's punchline: the **KB types themselves** (L0..L3) should
be expressible *in ein-lang*. A self-describing model — the same
language that authors puzzles can introspect them. Composes with
F5 (rules-as-data) and F1b (logical formulation).

Likely stages:

- **S1.10.H1** — write up the atom/object distinction; update
  glossary; update `03_ein_model.md` §2/§3.
- **S1.10.H2** — write up the 4-level representation; identify
  which existing model-doc files it touches.
- **S1.10.H3** — sketch what "self-describing KB types in ein
  lang" looks like (probably a `(meta …)` block or stdlib file).
  Bears on F5.

### Theme I — Kernel feature × config matrix

User direction 2026-05-27: *"separate file for kernel
inference feature list absolutely required to solve zebra in
reasonable time. Add config options for every (if not yet),
write ein files with different config options and measure
solution time with 3600 s timeout. Collect into table with
time and stats showing impact of every option disabled."*

The output is a measurement-backed reference page (likely
`docs/kernel/inference/features.md`) that tells a puzzle
author: "these config knobs are load-bearing; disabling them
slows / breaks zebra2 by ⟨factor⟩."

Steps:

1. **Audit current `SolverConfig`.** Catalogue every flag
   (`enable_back_prop_unconditional`, `enable_alive_inherit`,
   `enable_eager_root_bubble`, `enable_path_condition_nogoods`,
   `enable_pre_branch_lookahead`, `candidate_order_seed`,
   `hypgen_scoring`, etc.). For every flag with a `True`
   default, generate a paired `examples/zebra2-noFLAG.ein`
   variant that flips the flag off via its `(config …)` block.
2. **Add missing flags.** Any kernel feature whose impact we
   want to measure but which isn't behind a config flag yet
   gets one (default = current behaviour; the negation is the
   "feature off" case).
3. **Bench matrix.** Run `bench_solve_pypy.sh` on every
   variant with a 3600 s timeout. Record verdict, tree-node
   count, solve time, RSS.
4. **Result table** in `features.md`: one row per flag,
   columns for [default-on solve, flag-off solve, slowdown
   factor, "broken if off" sentinel].

Composes with Theme D (inference engine documentation) — the
features table cross-links into the per-feature narrative.

### Theme J — Ein API reference

User direction 2026-05-27. The Python-facing API for embedding
ein-bot in another project: how to construct a `KnowledgeBase`,
load IR, call `Saturator(kb).saturate()`, run `solve(kb,
mode=…)`, iterate `Firing`s, read provenance. Differs from
Theme B (kernel API) which is the IR-level surface; Theme J is
the Python-level surface a downstream user would import.

Lands as `docs/api/ein.md` (top-level Python package surface)
plus `docs/api/<module>.md` per public module. Composes with
Theme E (user vs developer split) — Theme J is *user-facing
embedding docs*, distinct from Theme D's engine internals.

If [M1a Rust port](../../m1a_rust/README.md) ships, the API
reference moves to ein.rs's surface (PyO3 binding + native
Rust) and Theme J's Python-side becomes the legacy reference.

### Theme K — Ein Zebra guide

User direction 2026-05-27. A long-form walkthrough of
`examples/zebra2.ein`, grouping rules into related families
(propagation / negative-completion / disjunctive-prune /
domain-elimination / spatial-endpoint / typecheck …). For
each rule:

- **Ein-lang form** — the literal `(rule …)` block from
  `zebra2.ein` plus a one-paragraph plain-English summary.
- **NL framing** — how the rule's intent reads in the
  Wikipedia Zebra walkthrough (cross-link `examples/README.md`).
- **Compact graph before / after** — small DOT showing the
  KB state immediately before and after the rule fires.
  Single-instance illustration, not full puzzle state.
- **Canonical Levi-bipartite graph before / after** — same
  pair under the Levi rendering, so the reader can flip
  between the two views. (See P1.6's
  default-compact-Levi-by-flag framing.)

Lands as `docs/kernel/zebra_guide.md` (or a small folder
under `docs/kernel/` if the rule families warrant per-family
pages). Composes with P1.6 S1.6.1 (rule rendering) — the
graph pairs reuse the same rule renderer.

## Out of scope

- Anything that *changes the kernel semantics* — that work
  belongs in M1's earlier phases. P1.10 is *documentation* of the
  shipped kernel.
- Multi-language SDK docs — Python is the only implementation
  through M1; if/when a second implementation lands, the
  idiomatic-vs-impl split makes that documentation trivial.

## Acceptance (per theme; tentative)

- **A:** `docs/kernel/ir/` shows the 4-level split; cross-links
  resolve.
- **B + C:** a puzzle author can implement an unfamiliar puzzle
  using only the API + stdlib reference pages (round-trip with
  the existing examples confirms).
- **D:** the inference engine doc covers the same surface the
  P1.3 + P1.5 stages described, but flattened into reference
  form rather than chronological design notes.
- **E:** the user-vs-dev split is consistent across the tree;
  each page declares its intended audience.
- **F:** a contributor reading only the architecture page can
  locate where to make a typical change (add a new rule family;
  add a new IR top-level form; etc.).
- **G:** all `docs/index/` paths return 404 or redirect;
  knowledge graph regenerates cleanly from `docs/lib/`.
- **H:** the atom/object distinction is reflected in
  `03_ein_model.md` and the glossary; the 4-level table is in
  the canonical doc.
- **I:** `features.md` carries the config-flag matrix with
  measured impact per flag against zebra2 (3600 s timeout
  per cell). Authors know which features they need on.
- **J:** `docs/api/ein.md` documents the Python embedding
  surface. A downstream user can import `ein_bot`,
  load a `.ein`, run `solve`, and read the verdict without
  reading the kernel internals.
- **K:** `docs/kernel/zebra_guide.md` walks every rule
  family in `zebra2.ein` with ein form + NL + compact +
  Levi graphs before/after.

## Open questions

- **Theme split timing.** Themes H + A + E are interlocking
  (H refines the model docs that A reorganises that E splits).
  Probably ship in that order (H first → A → E); document the
  ordering before starting any of them.
- **Theme G timing.** The `docs/index/` rename is best done
  *before* M2b (paper) because the paper will cite the catalogue
  by name. After M2b would leave a permanent "see docs/index in
  the published paper" footgun.

## Cross-links

- [`docs/kernel/`](../../../docs/kernel/) — the docs this phase
  reorganises.
- [`docs/kernel/ir/01-ein-graph/03_ein_model.md`](../../../docs/kernel/ir/01-ein-graph/03_ein_model.md)
  — Theme H's primary target.
- [`docs/index/`](../../../docs/index/) — Theme G's rename
  target.
- [`docs/kernel/inference/README.md`](../../../docs/kernel/inference/README.md)
  — Theme D's empty target page.
- [P1.8 Theme A — stdlib](../p1.8_ein_lang_modules/README.md) —
  Theme C depends on the stdlib content shipped there.
- [F5 — rules as data](../../followups/f5_rules_as_data.md) — H.2's
  "self-describing KB types in ein lang" is the precursor.
- [F1b — logical formulation](../../followups/f1b_logical_formulation.md)
  — H's clarity is upstream of F1b's FOL fragment characterisation.
- [M2b — presentation + paper](../../m2b_presentation/README.md) —
  Theme G should ship *before* M2b's write-up cites the catalogue.
