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
  than aspirational is the test in [§ Keeping this
  true](#keeping-this-true) below: every code identifier the page names
  resolves in `ein.rs/crates/`, and every environment variable and CLI
  invocation it shows runs.
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

## Keeping this true

**The failure mode this section exists to prevent has a name.** M1a
[S1a.10.6](../history/m1a_rust/README.md#s1a106--the-docs-after-the-oracle) was
a doc pass over this tree, and it missed **every** page M1e's review later
found — `algorithm_layer_n.md`, `lattice_dump.md`, `02_store.md`,
`02_patterns.md`, `04_dot_rendering.md`, `inference/README.md`, this README —
plus four more the review itself did not name (`reserved_engine_strings.md`,
`02_rules.md`, `01_entities.md`, `features.md`). The reason is structural, not
carelessness: **a doc pass driven by what the milestone changed cannot catch a
page describing machinery removed two milestones ago**, because nothing in the
milestone touched it. S1a.10.6 audited what the oracle's departure invalidated,
and it did that well.

So the checks below are deliberately **milestone-independent**. None of them
needs to know what changed; each asks a question of a page in isolation.

| | the check | what it caught, M1e S1e.2.2 |
|---|---|---|
| 1 | **Every code identifier resolves.** Every backticked `EIN_*` / `foo.rs` / `fn()` / `Type` / `snake_case` / `a::b` is findable in `ein.rs/crates/`. Report, not gate, and it can never be one: `Human` and DOT node ids are not identifiers, and — the sharper reason — **a page that correctly says a name is gone still names it**, so the count *rises* as the tree gets more honest. It went 86 → 92 across S1e.2.2's own fixes. Skim it for a name that looks like the engine's and is not accounted for; never count it | a `Mode` enum with three members that no crate has; `add_type` / `add_instance`; `from_dot`; a `_kb` back-pointer a page's own §5 says is gone. **Four of the eleven pages that needed work were found only this way** |
| 2 | **Every environment variable greps non-empty.** A special case of (1), worth naming because it is the cheapest and the most often wrong | `EIN_RENDER_LEVI`, claimed by `04_dot_rendering.md` and read by no code path, ever |
| 3 | **Every link and anchor resolves** — file *and* `#fragment`, GitHub-slugified — **and points inside the repository**, which is a finding of its own because `exists()` on a target that escapes the checkout answers for the machine rather than for the tree: three `../../acva/…` links passed this check on the author's workstation and failed CI (2026-09-01), and the fix at such a site is the repo's standing one — a reference that cannot be a link is still fine as `` `code` ``. **In the gate since S1e.3.8**, over the whole tree | 4 broken anchors here, all from a heading that grew words and a link that did not follow — and **264** once it was pointed at the rest of the repo |
| 4 | **Every prose `§x.y` names a heading that exists.** A section number written *after* a link, or inside its label, is **not part of the link**, so no link checker sees it. In the gate with (3) | **six** citations naming **four** sections `algorithm_layer_n.md` has never had, plus two into a `§1.5` that does not exist. This class had survived every previous pass |
| 5 | **Every command a page shows runs, and produces what the page says.** No script does this; it is a shell and ten minutes | a CLI line promising two artifacts it has never written; a claim that `(instnce ?a ?T)` is caught at parse time, which the parser accepts and the engine happily solves |
| 6 | **Every page is in one declared state** — § Which pages to trust above, with the state visible from the page rather than from a plan | three P1.5b design pages reading as live specification |

Checks 1–4 and 6 are [`utils/doc_audit.py`](../../utils/doc_audit.py):

```sh
python3 utils/doc_audit.py                      # all of it, as a report
python3 utils/doc_audit.py --links --check      # exit 1 on 3 or 4
python3 utils/doc_audit.py --identifiers -k inference/
```

**Checks 3 and 4 are in the gate since M1e S1e.3.8**, which is `DO-M2`'s
answer, and they are scoped to the **whole tree** rather than to this one — a
relative link resolves or it does not wherever it is written:

```sh
python3 utils/doc_audit.py --links --check docs plans README.md AGENTS.md \
    corpus/README.md tests/README.md examples/README.md stdlib/README.md \
    utils/README.md c/README.md
```

That is a step of `./run_tests.sh` and of `per-commit.yml`, diffed by
`gate_steps.rs` like every other. Pointing it outside `docs/kernel/` is what
earned it the place: **264** findings the first time, **251** of them in
`docs/history/m1d_satisfiability/` — a milestone record moved out of `plans/`
on 2026-08-27 with its links still aimed at the tree it left. 280 pages,
0.29 s.

Checks 1, 2 and 6 stay reports and check 1 can never be a gate, for the reason
its row gives. Check 5 has no instrument and is the one that found the most,
which is worth remembering when the temptation is to automate the cheap half
and call the pass done.

**One thing the checks cannot see**, and the reason `02_patterns.md` was the
hardest page in the triage: they find machinery that was **removed**, because
the identifier used to resolve and stopped. Machinery that was **planned and
never built** resolves at no point in the repo's history, reads exactly like a
description of something real, and is caught only by check 1 — by a reader
noticing that a name which *ought* to be there is not. `unique-remaining`,
`no-remaining-option`, `forbidden-by-exclusion` and five siblings sat in this
tree from P1.2 to M1e, described as *"the M1 starter set"*.

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
