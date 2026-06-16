# P1.20 — Kernel documentation

**Estimate:** ≈ 21–30 person-days for Themes A–H; Themes I/J/K
unestimated (a bench matrix, a full Python API reference, and a
long-form Zebra guide — each multi-day). A multi-week,
**non-M1-gating** investment to schedule against M2.
**Status:** **executed 2026-06-16** — all 12 originally-staged stages
(A0, A1, A2, B–G, H1–H3) plus **Theme J** (J1/J2/J3, decomposed + executed
2026-06-16) done. Theme J shipped the [`docs/api/`](../../../docs/api/)
Python-embedding subtree (contract + 4 per-module pages + a verified
worked example), re-based against the live surface. **Theme I executed
2026-06-17** (I1–I4): config audit + gating decision (I1); four behaviour-
preserving `enable_*` flags added + full gate green (I2, the one P1.20
engine-code change); feature×config sweep via `utils/feature_matrix.py`
(I3); and [`docs/kernel/inference/features.md`](../../../docs/kernel/inference/features.md)
(I4). Measured headline: on zebra2 the fast path is lever-insensitive; only
`enable_singleton_writeback` is load-bearing (exhaustive search). **Theme K
executed 2026-06-17** (K1–K4) — the [`docs/guide/`](../../../docs/guide/)
learn-Ein-by-example tutorial (objects/relations/facts → rules → full solve,
four chapters), wired in as the newcomer entry point. **All P1.20 themes are
now executed.** Created 2026-05-24 from the TODO.md scratchpad,
stages written 2026-06-15, executed 2026-06-16.
The phase parks kernel-doc reorgs that emerged after the M1
implementation surface stabilised. None of these gate M1
acceptance; they make the kernel docs reflect what M1 actually
shipped (and what the user wants the M1 docs to teach post-hoc).

> **Re-base note (2026-06-16).** This plan was authored against a
> kernel snapshot that the late-May/June refactors have since moved
> past — the flat-form grammar (P1.7c), the merged sound `solve`
> (verdict read from `k`), the shipped stdlib (P1.8a S1.8a.f20), and
> the now-substantial `docs/kernel/inference/` (≈1.7k lines). Several
> themes were written as greenfield authoring over empty/stub docs;
> the real starting state is **mature-but-drifting** docs.
> **[S1.20.A0](s1.20.a0_reconcile_drift.md) is the pre-flight
> reconcile that re-bases the rest of the phase — run it first.**
**Depends on:** [P1.7](../p1.7_bootstrapping_zebra/) ships
("M1 done" — the kernel is locked) so documenting it doesn't
chase a moving target.
**Blocks:** nothing on M1's critical path; informs M1b (GUI) and
M2b (paper) by giving them a stable doc surface to point at.

The existing kernel docs at [`docs/kernel/`](../../../docs/kernel/)
are organised into three sub-trees (`ir/01-ein-graph/`,
`ir/02-data-model/`, `ir/03-ein-lang/`) plus an `inference/`
sub-tree (≈1.7k lines, but a stale README — see Theme D). P1.20
takes that structure further:

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
| S1.20.A0   | **Reconcile kernel docs against the live kernel** (pre-flight re-base)       | [s1.20.a0_reconcile_drift.md](s1.20.a0_reconcile_drift.md)            |
| S1.20.A1   | IR docs: split data-model into idiomatic + python-impl                      | [s1.20.a1_ir_doc_split.md](s1.20.a1_ir_doc_split.md)                  |
| S1.20.A2   | Idiomatic data-model backfill                                               | [s1.20.a2_idiomatic_backfill.md](s1.20.a2_idiomatic_backfill.md)      |
| S1.20.B    | Kernel API reference                                                        | [s1.20.b_kernel_api_reference.md](s1.20.b_kernel_api_reference.md)    |
| S1.20.C    | Stdlib API reference                                                        | [s1.20.c_stdlib_api_reference.md](s1.20.c_stdlib_api_reference.md)    |
| S1.20.D    | Inference engine documentation                                              | [s1.20.d_inference_engine_docs.md](s1.20.d_inference_engine_docs.md)  |
| S1.20.E    | User vs developer docs split                                                | [s1.20.e_user_vs_dev_split.md](s1.20.e_user_vs_dev_split.md)          |
| S1.20.F    | Architecture overview + diagrams                                            | [s1.20.f_architecture_overview.md](s1.20.f_architecture_overview.md)  |
| S1.20.G    | `docs/index/` → `docs/lib/` rename                                          | [s1.20.g_docs_index_rename.md](s1.20.g_docs_index_rename.md)          |
| S1.20.H1   | Ein-model: atoms vs objects                                                 | [s1.20.h1_atom_vs_object.md](s1.20.h1_atom_vs_object.md)              |
| S1.20.H2   | Ein-model: 4-level KB representation                                        | [s1.20.h2_four_level_kb.md](s1.20.h2_four_level_kb.md)                |
| S1.20.H3   | Ein-model: self-describing KB types                                         | [s1.20.h3_self_describing_kb.md](s1.20.h3_self_describing_kb.md)      |
| S1.20.I1   | Config audit + feature inventory + gating decision                          | [s1.20.i1_config_audit.md](s1.20.i1_config_audit.md)                  |
| S1.20.I2   | Add the missing config flags (code)                                         | [s1.20.i2_add_flags.md](s1.20.i2_add_flags.md)                        |
| S1.20.I3   | Variant fixtures + bench sweep                                              | [s1.20.i3_bench_sweep.md](s1.20.i3_bench_sweep.md)                    |
| S1.20.I4   | `features.md` result table + narrative                                      | [s1.20.i4_features_doc.md](s1.20.i4_features_doc.md)                  |
| S1.20.J1   | Embedding contract + entry-point reference                                  | [s1.20.j1_embedding_contract.md](s1.20.j1_embedding_contract.md)      |
| S1.20.J2   | Per-module Python API reference pages                                       | [s1.20.j2_per_module_reference.md](s1.20.j2_per_module_reference.md)  |
| S1.20.J3   | Worked embedding example + integration                                      | [s1.20.j3_worked_example.md](s1.20.j3_worked_example.md)              |
| S1.20.K1   | Guide on-ramp: objects, relations, facts (NL ↔ IR ↔ graph)                  | [s1.20.k1_onramp.md](s1.20.k1_onramp.md)                              |
| S1.20.K2   | Guide: first rules (graph before / after)                                   | [s1.20.k2_first_rules.md](s1.20.k2_first_rules.md)                    |
| S1.20.K3   | Guide: the rule families (on zebra2)                                        | [s1.20.k3_rule_families.md](s1.20.k3_rule_families.md)                |
| S1.20.K4   | Guide: solving the whole puzzle + integration                              | [s1.20.k4_full_solve.md](s1.20.k4_full_solve.md)                      |

Suggested order: **A0 first** (reconcile drift — it re-bases every
other theme). Then H1 → H2 → H3 (vocabulary first, then schema)
∥ A1 → A2 (data-model split, then backfill); B + C + D can
follow once A2 + H2 are in; E and F land after the bulk of
content stabilises; G can ship any time before M2b. Theme J
(Python embedding API) is staged J1 → J2 → J3 (J1 first — it
re-bases the surface and fixes the location/facade decisions the
other two build on). Theme I (feature matrix) is staged I1 → I2 → I3
→ I4 (I1 audit/decide → I2 code, gated by I1's decision → I3 measure
→ I4 write). Theme K (Zebra guide) shipped K1 → K4 (on-ramp → first rules →
rule families → full solve) — the learn-by-example tutorial in `docs/guide/`.

## Themes

### Theme A — data-model doc split (idiomatic vs python-impl)

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

- **S1.20.A1** — split the existing `02-data-model/` into
  idiomatic (collections, indexes, algorithms over the data) and
  python-impl (modules, classes, file layout).
- **S1.20.A2** — backfill the idiomatic level with mathematical
  pseudocode / diagrams for the index structures.

### Theme B — Kernel API reference

Today the kernel's **primitives and semantically-loaded atoms**
(`rule`, `relation`, `T`, `not`, `eq`, `neq`, `absent`, `forall`,
`open`, `false`, … + the closed declarator set `relation` · `rule`
· `hrule` · `query` · `config` · `trace` · `macro` · `import`) are
documented piecemeal across the three sub-trees + the engine
source. P1.20 ships a dedicated reference page that lists each, its
arity / signature, its semantic role, and where it may appear
(declarator vs `:match`/`:assert` vs fact).

> **Re-base (S1.20.A0).** The grammar is **flat-form** since P1.7c —
> the former `(ontology …)`/`(facts …)`/`(reasoning …)`/`(rules …)`
> block wrappers are **gone** (`ir/grammar.lark`); a fact is any head
> outside the closed declarator set, and its layer is per-fact. Drop
> the "rendering in `(rules …)` / `(facts …)` contexts" framing.

The reference should be **complete enough to author new puzzles
without reading source** — the M1-end-of-life test for a
kernel API doc.

### Theme C — Stdlib API reference

Parallel to Theme B but for the **stdlib** rules and macros. The
stdlib has **shipped** (P1.8a S1.8a.f20):
[`ein.py/src/ein/stdlib/`](../../../ein.py/src/ein/stdlib/) carries
`algebra`, `bijection`, `closure`, `elim`, `typing`, and `macro`
modules (+ a README), imported and exercised by
`examples/zebra2.ein`. So this theme is **unblocked** — write it
against the real modules (the `bijective`/`functional`/`total`/…
checks in `std.algebra`, the elimination + negative-completion in
`std.bijection` / `std.elim`, the closure rules, the `forall` /
`open` macros), not as an outline awaiting P1.8.

### Theme D — Inference engine documentation

> **Re-base (S1.20.A0).** `docs/kernel/inference/` is **not** a stub —
> it already carries ≈1.7k lines across five files
> (`architecture_and_algorithms.md`, `domain_elim_vs_hypothesis.md`,
> `lattice_dump.md`, `reserved_engine_strings.md`, and a 665-line
> `README.md`). But that `README.md` is itself **stale**: its banner
> still said "Stub — becomes load-bearing when P1.3 ships" (P1.3
> shipped), its "Planned structure" lists `01_matcher.md … 05_trace.md`
> files that were never created, and its "What lives where today"
> table claims shipped features are "not yet".

So Theme D is **reconcile + restructure**, not greenfield:

- **Reconcile the stale `README.md`** — drop the stub banner,
  replace the never-built `01–05` "planned structure" with the
  as-built file list, and correct the "M1 stubs" table to the
  shipped engine (cross-ref `architecture_and_algorithms.md`, which
  is already the as-built reference).
- **Idiomatic layer** — collections (rule-cache, activator index,
  firing queue), indexes (`_facts_by_relation`, `_negated_facts`,
  `_rule_apps_by_rule`), algorithm diagrams + pseudocode for
  `compile_all` / `compile_for` / saturate / the commitment-lattice
  search.
- **Python implementation** — files / modules (`engine.py`,
  `saturator.py`, `hypgen.py`, `compile.py`, `match.py`,
  `monotonic/solver.py`), data types, classes.

Composes with the NAF-at-enqueue invariant surfaced by
[P1.5a](../p1.5a_zebra_solution/README.md) (which the live engine
already resolves via fire-time `absents_still_pass` re-eval — that
resolution needs a written invariant here).

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

- **S1.20.H1** — write up the atom/object distinction; update
  glossary; update `03_ein_model.md` §2/§3.
- **S1.20.H2** — write up the 4-level representation; identify
  which existing model-doc files it touches.
- **S1.20.H3** — sketch what "self-describing KB types in ein
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

> **Re-base (S1.20.A0).** The flag names below are **stale** (written
> 2026-05-27, pre-lattice-rewrite): `enable_alive_inherit`,
> `enable_eager_root_bubble`, and `enable_path_condition_nogoods` **no
> longer exist** in `ein.py/src/ein/inference/config.py`. Audit from
> the live config; the real levers are `enable_pre_branch_lookahead`,
> `enable_lookahead_kill_cache`, `enable_back_prop_unconditional`,
> `hypgen_scoring` (+ `hypgen_rel_weight` / `hypgen_obj_weight`),
> `candidate_order_seed`, `lattice_order`, `lattice_sanity_check`.
> Also re-scope the bench: post-rewrite `zebra2` solves in ≈2 s with
> 0 nogoods, so a **3600 s per-cell** timeout is wildly oversized —
> keep it only as the outer "broken if off" guard, bench perf levers
> with a ≈60–120 s budget, and split the table into
> **correctness-load-bearing** (puzzle breaks / won't terminate when
> off) vs **perf-only** (measured slowdown factor).

Steps:

1. **Audit current `SolverConfig`.** Catalogue every flag from the
   live `config.py` (see the re-base note for the current set). For
   every flag with a `True` default, generate a paired
   `examples/zebra2-noFLAG.ein` variant that flips it off via its
   `(config …)` block.
2. **Add missing flags.** Any kernel feature whose impact we
   want to measure but which isn't behind a config flag yet
   gets one (default = current behaviour; the negation is the
   "feature off" case).
3. **Bench matrix.** Run `ein_pypy.sh` (the bench runner; the old
   `bench_solve_pypy.sh` was retired) on every variant. Record
   verdict, enterings / layers (no more tree-node count — the search
   is a commitment lattice), solve time, RSS.
4. **Result table** in `features.md`: one row per flag,
   columns for [default-on solve, flag-off solve, slowdown
   factor, "broken if off" sentinel].

Composes with Theme D (inference engine documentation) — the
features table cross-links into the per-feature narrative.

> **Re-base (audited 2026-06-16, decomposing this theme).** Two findings
> the 2026-05-27 sketch missed: (1) only **two** fields are True-default
> booleans (`enable_pre_branch_lookahead`, `enable_lookahead_kill_cache`) —
> the rest of `SolverConfig` are value-choices (`hypgen_scoring`,
> `lattice_order`), ints, or default-`False`, so "flip every True-default
> flag" undercounts and mis-shapes the matrix; (2) the features most worth
> measuring — **CDCL no-goods, the `__symmetric__` mirror, singleton-death
> writeback, forced-positive promotion** — are **hardcoded always-on, not
> behind any flag**, so measuring them requires *adding* flags (engine
> code). That makes **I2 the one P1.20 stage that changes engine code**
> (behaviour-preserving at default). Full audit in
> [S1.20.I1](s1.20.i1_config_audit.md).

Likely stages:

- **S1.20.I1** — [config audit + feature inventory + gating decision](s1.20.i1_config_audit.md):
  classify every lever (correctness / perf / value-choice / diagnostic),
  inventory the un-gated always-on features, and decide which get a flag.
- **S1.20.I2** — [add the missing config flags (code)](s1.20.i2_add_flags.md):
  thread behaviour-preserving `enable_*` flags through the engine; prove
  default-on is unchanged. *Conditional on I1's decision.*
- **S1.20.I3** — [variant fixtures + bench sweep](s1.20.i3_bench_sweep.md):
  one zebra2 variant per lever, run under `ein_pypy.sh solve --stats` with
  a budget; capture the `MonotonicStats` counters + time + RSS.
- **S1.20.I4** — [`features.md` result table + narrative](s1.20.i4_features_doc.md):
  correctness-load-bearing vs perf-only tables, value-choice swaps,
  SHA-stamped provenance; cross-link Theme D + Theme J.

### Theme J — Ein API reference

User direction 2026-05-27. The Python-facing API for embedding
Ein in another project: how to construct a `KnowledgeBase`,
load IR, call `Saturator(kb).saturate()`, run `solve(kb,
mode=…)`, iterate `Firing`s, read provenance. Differs from
Theme B (kernel API) which is the IR-level surface; Theme J is
the Python-level surface a downstream user would import.

Lands as `docs/api/ein.md` (top-level Python package surface)
plus `docs/api/<module>.md` per public module. Composes with
Theme E (user vs developer split) — Theme J is *user-facing
embedding docs*, distinct from Theme D's engine internals.

> **Re-base (audited 2026-06-16, decomposing this theme).** The
> 2026-05-27 sketch above names APIs that have since moved. The live
> surface: `import ein` exposes **only `__version__`** (no facade —
> the contract is per-subpackage `ein.ir` / `ein.kb` /
> `ein.inference.*` / `ein.trace`); `solve` lives in
> `ein.inference.monotonic.solver` and takes **no `mode=`** (the
> verdict type is read from the solution count) —
> `solve(root_kb, *, stop_after=None, max_set_size=5, config=None, …)`.
> The full audit table (with file:line anchors) lives in
> [S1.20.J1](s1.20.j1_embedding_contract.md).

Likely stages:

- **S1.20.J1** — [embedding contract + entry-point reference](s1.20.j1_embedding_contract.md):
  re-base the surface, resolve the facade/location questions, write
  `docs/api/ein.md` (the five-step load → solve → read-verdict flow).
- **S1.20.J2** — [per-module reference pages](s1.20.j2_per_module_reference.md):
  `docs/api/{ir,kb,inference,trace}.md`, one entry per public symbol
  (signature · params · returns · semantics · example) + the
  `SolverConfig` knob table.
- **S1.20.J3** — [worked example + integration](s1.20.j3_worked_example.md):
  a copy-paste, verified-to-run zebra2 script; reading-order wiring;
  the `inference/__init__.py` docstring reconcile.

If [M1a Rust port](../../m1a_rust/README.md) ships, the API
reference moves to ein.rs's surface (PyO3 binding + native
Rust) and Theme J's Python-side becomes the legacy reference.

### Theme K — Ein Zebra guide

User direction 2026-05-27, **re-framed 2026-06-17**: the Ein Zebra Guide is
a **learn-Ein-by-example tutorial** — "how to solve a Zebra puzzle with
Ein", for a complete beginner. It is **user-facing** and **references** the
kernel + api docs and the inference explanation rather than re-deriving
them; it never explains kernel internals or the search process. It is the
designed **complement** of
[`inference/zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md):
the guide teaches the pieces incrementally; the walkthrough is the full
solved trace it hands off to.

> **Re-base note.** The 2026-05-27 spec was a *rule reference catalogue*
> (rules grouped by family, each with ein-form + NL + compact/Levi graph
> pairs). That survives — as the guide's *middle* chapters
> ([K2](s1.20.k2_first_rules.md)/[K3](s1.20.k3_rule_families.md)) — but the
> re-frame adds a **from-zero on-ramp** ([K1](s1.20.k1_onramp.md): objects →
> relations → facts in NL ↔ IR ↔ graph, *before* any rule), orders by
> **difficulty** not family, and lands in a new **`docs/guide/`** (a
> newcomer entry point, sibling of `kernel/`/`api/`/`lib/`) — not
> `docs/kernel/zebra_guide.md` (a tutorial reads awkwardly inside the kernel
> "contract" tree). The per-rule unit is unchanged: **ein-lang `(rule …)`
> form + plain-English + compact & Levi graph before/after** (P1.6's rule
> renderer).

The guide's recurring teaching device is the **NL ↔ ein-lang IR ↔ graph**
triad, established on objects/relations (K1) and reused per rule (K2/K3).

Likely stages:

- **S1.20.K1** — [on-ramp](s1.20.k1_onramp.md): objects → relations →
  facts, tri-representation, on a tiny example; creates `docs/guide/`.
- **S1.20.K2** — [first rules](s1.20.k2_first_rules.md): symmetric /
  transitive / co-located, each with graph before/after.
- **S1.20.K3** — [rule families](s1.20.k3_rule_families.md): domain-
  elimination, disjunctive-prune, spatial adjacent-via, negative-completion
  on real `zebra2` fragments (stdlib machinery vs puzzle-specific rules).
- **S1.20.K4** — [the whole puzzle](s1.20.k4_full_solve.md): assemble +
  `ein solve` + hand off to the walkthrough; `docs/guide/` integration.

Composes with P1.6 S1.6.1 (rule rendering) — the graph pairs reuse the same
renderer.

## Out of scope

- Anything that *changes the kernel semantics* — that work
  belongs in M1's earlier phases. P1.20 is *documentation* of the
  shipped kernel.
- Multi-language SDK docs — Python is the only implementation
  through M1; if/when a second implementation lands, the
  idiomatic-vs-impl split makes that documentation trivial.

## Acceptance (per theme; tentative)

- **A:** `docs/kernel/ir/` shows the data-model split
  (01 / 02 / 02b / 03); cross-links resolve.
- **B + C:** a puzzle author can implement an unfamiliar puzzle
  using only the API + stdlib reference pages (round-trip with
  the existing examples confirms).
- **D:** the stale `inference/README.md` is reconciled (no "stub"
  banner; as-built file list; corrected status table), and the
  engine doc covers the same surface the P1.3 + P1.5 stages
  described, flattened into reference form rather than
  chronological design notes.
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
  surface. A downstream user can import `ein`,
  load a `.ein`, run `solve`, and read the verdict without
  reading the kernel internals.
- **K:** `docs/guide/` is a learn-Ein-by-example tutorial — a newcomer
  authors objects/relations/facts (NL ↔ IR ↔ graph), then rules (simple →
  the zebra2 families, each with compact & Levi graph before/after), then
  solves the whole puzzle and hands off to
  `inference/zebra_walkthrough.md`. User-facing; references the kernel/api
  docs, never explains internals.

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
- [`docs/lib/`](../../../docs/lib/) — Theme G's rename
  result (formerly `docs/index/`).
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
