# `docs/ein/` — the tree a released system would have

> **Scaffolding.** This page is [P1f.20](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md)'s
> working contract, written by
> [S1f.20.2](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.2_the_disposition_table.md) for
> the user's review. **Nothing has moved yet.** When the phase closes, § Where each page went goes into the closing
> commit's message and this page is rewritten as the tree's index, with no plan, stage or history in it. Until
> then it names source paths, plans and milestones on purpose, and the release-voice check carries it in its
> baseline.

## The three perspectives (provisional)

The user's instruction, 2026-08-28: *"connect each document to the new docs tree, but not copy. Rewrite instead"*,
in three perspectives. Each paragraph of a source page is asked which one it belongs to.

- **`user/`, the black box**: *problem → [solve as a black box] → result*. What a program says, how to run it, what
  the answer means and every row it prints. No crate, module or algorithm.
- **`reasoning/`, implementation-agnostic**: what the engine computes and why it is right, stated so that a second
  implementation would be held to it. The question asked of every paragraph: *would this still be true of a
  second implementation?* If not, it is `ein.rs/`'s.
- **`ein.rs/`, the concrete implementation**: the crates, the design contracts they cite, the embedding API, the
  engine's own read-outs, and the evidence (a command and what it gives on today's engine).

Beside them: `overview/` (the comparison with other systems, M5's), `docs/demo/` (M5's), `docs/decisions/`
(every decision, the final one, ids kept, [S1f.20.8](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.8_decisions.md)),
and `docs/lib/`, unchanged. After the phase, `docs/kernel/`, `docs/api/`, `docs/guide/`, `docs/install.md` and
`docs/history/` do not exist, and no page, comment or test needs a plan to be read
([the user's rulings of 2026-11-15](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md#the-users-rulings-2026-11-15)).

## The tree

Redrawn against the tree of 2026-11-16 from the [map of 2026-08-28](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md#the-target-tree).
`←` is *rewritten from*, never *copied from*. Sources are short: `kernel/…`, `api/…`, `guide/…` and `install.md`
are under `docs/`; `m1a/…`, `m1c/…`, `m1d/…`, `m1e/…` are under `docs/history/`; `M1e2 …` and `P1f.6 …` are pages of
those finished plan folders. **(redrawn)** marks a line the 2026-08-28 map did not have or had otherwise; **new**
marks a page with no source page.

```text
docs/
  decisions/                        every decision, the final one, ids kept (S1f.20.8; layout in § For the review)
  ein/
    README.md                       ← kernel/README.md (reading order, audiences, § Keeping this true)
    user/                           problem → [solve as a black box] → result
      README.md                     new: the reading paths of kernel/README.md's audience table, user side
      install.md                    ← install.md                                        (redrawn: was in reference/cli.md)
      guide/
        README.md · 01–04           ← guide/ near verbatim (guide_programs.rs, guide_transcripts.rs move with them)
        05_jack_drinks_coffee.md    ← kernel/ir/01-ein-graph/04_jack_drinks_coffee.md
        06_zebra_walkthrough.md     ← kernel/inference/zebra_walkthrough.md
        examples.md                 ← kernel/ir/03-ein-lang/03_examples.md                 (redrawn)
        word-problems.md            ← P1f.6 word_problems.md · beyond_arithmetic.md (results only; see the question)  (redrawn)
      syntax-and-semantics/         every concept in four representations: ein-lang · graph · NL · math
        README.md                   ← kernel/ir/01-ein-graph/README.md (the format, fixed in objects.md first)
        objects.md                  ← 01_kb.md § 1 (objects)
        relations.md                ← 01_kb.md (relations) · 03_ein_model.md § 4
        facts.md                    ← 01_kb.md §§ 3–6 (where a fact came from, identity, branches)
        ontology.md                 ← 03_ein_model.md
        rules.md                    ← 02_rules.md §§ 1, 4, 5 · 02_patterns.md (what a pattern means)
        properties-and-operators.md ← 07_stdlib_api.md (std.algebra) · 08_self_describing.md § 3
        constraints.md              ← 02_rules.md § 3: the (false) / (not …) upper bound      (redrawn: was new)
        obligations.md              ← m1d/obligation_forms.md (form G) · m1d/domain_contract.md (C1–C4) · m1d/README.md (S1d.2.3, S1d.2.4)
        hypotheses.md               ← 01_grammar.md (hrule, the solve lists, :no-search) · defined_behaviour.md § 4.27   (redrawn)
        theory.md                   new: a rule set as a theory, and how rules interact
        macros.md                   ← 01_grammar.md (macro, import)
        query.md                    ← 01_grammar.md (solve, query, :expect) · m1c/README.md (S1c.1.2) · M1e2 region_language.md, region_examples.md § 1
        solutions-and-models.md     ← kernel/inference/solution_semantics.md (what R is, to its reader)
        natural-language.md         ← 09_natural_language.md                               (redrawn)
      reference/
        README.md                   ← kernel/ir/03-ein-lang/README.md
        grammar.md                  ← 00_ebnf.md (its two blocks written by grammar_pages.rs)   (redrawn: split from syntax.md)
        syntax.md                   ← 01_grammar.md · 02_patterns.md
        reserved-names.md           ← 06_reserved_names.md                                  (redrawn: split from syntax.md)
        stdlib.md                   ← 07_stdlib_api.md · P1f.6 design.md, binary_arithmetic.md, binary_logic.md, malformed_numbers.md
        cli.md                      ← M1e2 solve_interface.md §§ 1, 2, 4 · defined_behaviour.md § 5 · m1c/README.md (S1c.1.3)
        config.md                   ← configuration.md (its defaults block is ein solve --dump-config's)
        diagnostics.md              ← defined_behaviour.md §§ 1, 4
        read-out.md                 ← kernel/inference/read_out.md (read_out_reference.rs)   (redrawn: new row)
        read-out-examples.md        ← kernel/inference/read_out_examples.md (utils/read_out_examples.py)   (redrawn)
        answer-dump.md              ← kernel/inference/answer_dump.md                       (redrawn)
        region-dump.md              ← kernel/inference/region_dump.md                       (redrawn)
        rendering.md                ← 04_dot_rendering.md                                   (redrawn: no row on the map)
        glossary.md                 ← kernel/glossary.md
    reasoning/                      implementation-agnostic
      README.md                     new: the question asked of every paragraph, and the reading order
      static-ir/                    the program up to its loaded state
        names-and-atoms.md          ← 02-data-model/01_entities.md (identity by name)
        relations-and-facts.md      ← 02-data-model/02_store.md (the store as facts and negations)
        rules-as-statements.md      ← 02_rules.md §§ 2, 6.3 (T1–T3, DPO) · 05_four_level_kb.md
        values-and-order.md         ← defined_behaviour.md §§ 2, 3                          (redrawn)
      kb-evolution/                 after compilation, and onwards
        firing.md                   ← inference/README.md (design principles) · architecture_and_algorithms.md §§ 1–5
        saturation.md               ← inference/README.md (the alive-set invariant, d=0 completion) · m1a/design/06 § 1
        absent-and-naf.md           ← absent_semantics.md · inference/README.md (NAF) · architecture.md (the closure/worlds seam)
        hypothesis-generation.md    ← m1d/hypotheses_from_obligations.md (the ladder) · m1d/README.md (S1d.2.5) · m1d/completeness.md
        hypothesis-space.md         ← hypothesis_space.md (show_hyps.rs, h_space_view.rs)      (redrawn: was "+ P1f.10's results")
        solution-semantics.md       ← solution_semantics.md (the definition)
        finding-r.md                ← finding_r.md · M1e2 solve_interface.md §§ 3.1–3.3       (redrawn: was search.md ← algorithm_layer_n.md)
        monotonicity.md             ← monotonicity.md · M1e2 detector_proposal.md, site_census.md § 5   (redrawn)
        cores.md                    ← architecture_and_algorithms.md (the ATMS half) · 01_kb.md § 5   (redrawn: was atms-and-nogoods.md)
        satisfiability.md           ← m1d/the_boundary.md · m1d/completeness.md · m1d/README.md
      rule-evolution/
        rules-as-data.md            ← 08_self_describing.md (what is operational)
        analysis-of-rules.md        new: the rule graph and its strata (strata.rs; S1f.5.15 shipped it)
        transformations.md          new: a stub with its question in it
    ein.rs/                         the concrete implementation
      README.md                     ← api/README.md
      api.md                        ← api/rust.md (the marked region intact; embedding.rs moves with it)
      architecture.md               ← kernel/architecture.md · inference/implementation.md · 02-data-model/03_implementation.md · inference/README.md (as-built layout)
      algorithms.md                 ← architecture_and_algorithms.md §§ 6–7                  (redrawn: was ← features.md, deleted)
      events.md                     ← inference/events.md (events_reference.rs)             (redrawn: was read-outs.md)
      lattice-dump.md               ← inference/lattice_dump.md (the region run's nodes)    (redrawn: was read-outs.md)
      reserved-engine-strings.md    ← inference/reserved_engine_strings.md                  (redrawn: no row on the map)
      design/                       m1a's contracts, de-scaffolded and brought to the engine as it is
        README.md                   ← m1a/design/README.md
        determinism-and-order.md    ← m1a/design/02
        data-model.md               ← m1a/design/03 · 02_store.md (record shapes) · m1a/README.md (P1a.2)
        ir-frontend.md              ← m1a/design/04 (§ 1 rewritten: the parser is generated) · m1a/README.md (P1a.1, S1a.2.3, S1a.6.5)
        matcher.md                  ← m1a/design/05 · m1a/README.md (S1a.6.3, S1a.6.8)
        saturation.md               ← m1a/design/06 · m1a/README.md (S1a.6.9, S1a.6.12)
        search.md                   ← m1a/design/07 §§ 2, 6, 7 · M1e2 solve_interface.md § 3.4, alive_set_reach.md   (redrawn)
        parallelism.md              ← m1a/design/08 §§ 1, 6 · m1a/measurements/shared_state.md § 4 · m1a/README.md (P1a.7)
        binary-format.md            ← m1a/design/10 · m1a/README.md (P1a.8)
        shared-assets.md            ← m1a/design/11
        toolchain-and-layout.md     ← m1a/design/12 · m1a/README.md (P1a.10, S1a.6.6)
      benchmarks/                   evidence: a command, and what it gives on today's engine, on a named machine
        README.md                   new: utils/bench_env.sh and the machine
        corpus-cost.md              ← m1a/measurements/corpus_cost.md (utils/corpus_cost.py)
        stdlib-census.md            ← m1c/stdlib_census.md (utils/stdlib_census.py)
        region.md                   ← M1e2 region_census.md (utils/region_census.py), route1_reach.md (--example route1_reach)   (redrawn)
        non-monotone.md             ← M1e2 node_census.md, hole_census.md, taint_census.md (--example …)   (redrawn)
    overview/                       M5 Track A fills it; S1f.20.7 builds it
      README.md                     new
      inspirations.md               ← kernel/ir/03-ein-lang/05_inspirations.md
  demo/                             M5's; S1f.20.7 builds it
    README.md                       new
  lib/                              unchanged
```

Every destination above has a source row below or is marked *new*. Against the 2026-08-28 map:
- **`search.md` is `finding-r.md`**: its source was a walk with no code since M1e2 S1e2.1c.5; the search is now the
  routes over R, whose formulas are `finding_r.md`'s and whose contract is `solve_interface.md` § 3.
- **`algorithms.md` loses `features.md`**: every row of that page measures the walk. It takes the half of
  `architecture_and_algorithms.md` that names ein.rs's choices instead.
- **`read-outs.md` is three pages in two places**: the answer's files (`answer-dump`, `region-dump`) and the
  read-out with its golden are the user's result; `events.md` and `lattice-dump.md` narrate the engine and are
  ein.rs's. `lattice_diagrams.md` goes.
- **`atms-and-nogoods.md` is `cores.md`**: the no-good store went with the walk; cores and frontiers did not.
- **`syntax.md` is three pages** (`grammar.md`, `syntax.md`, `reserved-names.md`): `00_ebnf.md`'s blocks are
  generated and move intact, and the four sources together are 2 780 lines (`wc -l`, 2026-11-16).
- **The map's `syntax-and-semantics/config.md` and `reference/config.md` are one page**, `reference/config.md`;
  the `(config …)` form is `query.md`'s.
- **Pages the map has no row for**: `answer_dump.md`, `region_dump.md`, `monotonicity.md`, `hypothesis_space.md`,
  `finding_r.md`, `read_out.md`, `read_out_examples.md`, `09_natural_language.md`, `04_dot_rendering.md`,
  `reserved_engine_strings.md`, `standard_of_proof.md`, and the pages of finished plans that shipped code cites
  as its specification (§ Pages in finished plan folders).

## Where each page went

One row per source page. **Disposition**: *rewritten into* (the destination states it in the release voice; a row
naming several destinations splits the page), *evidence* (a re-takable command and what it gives on today's
engine, under `ein.rs/benchmarks/`), *deleted* (it lives in git), or *moved* (the one decided row,
`m1d/ideas.md`). Destinations are under `docs/ein/` unless they start `docs/` or `plans/`. **in** is the
page's inbound reference count from S1f.20.1's checker, every `.md`, Rust doc comment, `.ein` header and
`corpus.toml` line together (`…` until that checker lands on master; § The inbound counts).

**A test-held or generated page moves with its test or generator** and is not rewritten from a plan: its prose
loses the release-voice lines, its held part moves byte for byte. Those are `api/rust.md` (its `rust` block and
`<!-- prose -->` region, `embedding.rs`), `00_ebnf.md` (two blocks, `grammar_pages.rs`), `configuration.md` (the
defaults block, `config_reference.rs`), `read_out.md` and `read_out_examples.md` (`read_out_reference.rs`,
`read_out_examples.rs`, `utils/read_out_examples.py`), `hypothesis_space.md` (`show_hyps.rs`, `h_space_view.rs`),
`events.md` (`events_reference.rs`, `cli_semantics.rs`), `09_natural_language.md` (`natural_language_page.rs`),
`answer_dump.md` (`answer_dump.rs`), `region_dump.md` (`region_dump.rs`), `defined_behaviour.md` and
`configuration.md` (`test_cli.rs`), and the guide (`guide_programs.rs`, `guide_transcripts.rs`). **Six pages are
named in the binary's own output** (`cmdline.rs`'s help texts name `events.md`, `hypothesis_space.md`,
`04_dot_rendering.md`, `answer_dump.md`, `lattice_dump.md` and `region_dump.md`; `ein-render`'s
`dump/region.rs` writes `region_dump.md`'s path into every oracle dump), and `install.md` into every release's
notes (`release.yml`): those strings move in the commit that moves the page.

### `docs/kernel/` (48 pages; sources relative to `docs/kernel/`)

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `README.md` | `README.md` (this page, rewritten at the close) | rewritten into | … | reading order, audiences and § Keeping this true (the six checks) become the tree's index; § Which pages to trust (three states, banners) has no successor, since no page keeps a banner |
| `architecture.md` | `ein.rs/architecture.md`; the closure/worlds seam's agnostic half → `reasoning/kb-evolution/absent-and-naf.md` | rewritten into | … | the structural map; § Milestone boundaries is history and goes |
| `configuration.md` | `user/reference/config.md` | rewritten into | … | test-held (`config_reference.rs` diffs `FIELDS`, `test_cli.rs` names the schema); the defaults block moves as generated; the banner's chronology goes |
| `defined_behaviour.md` | `user/reference/diagnostics.md` (§§ 1, 4), `user/reference/cli.md` (§ 5), `reasoning/static-ir/values-and-order.md` (§§ 2, 3) | rewritten into | … | the split S1f.20.4 names: diagnostics for the user, orderings for a reimplementer; § 6 (*not defined, and filed*) is work, not docs (§ No obvious destination) |
| `glossary.md` | `user/reference/glossary.md` | rewritten into | … | |
| `standard_of_proof.md` | `docs/decisions/` (Q-M1e.1, Q-M1e.2) and `README.md` § Keeping this true | rewritten into | … | it is two ratified decisions; **least sure, and the user's to decide** (§ For the review) |
| `inference/README.md` | `reasoning/kb-evolution/{firing,saturation,absent-and-naf}.md`, `ein.rs/architecture.md` (as-built layout) | rewritten into | … | design principles, the alive-set invariant, NAF, d=0 completion; its six sections on gone machinery (the tree solver, unconditional facts, the walk) go |
| `inference/absent_semantics.md` | `reasoning/kb-evolution/absent-and-naf.md` | rewritten into | … | normative, pinned by `naf_semantics.rs`; cites `dual_rail.ein`, which lives only in M1e2's folder |
| `inference/algorithm_layer_n.md` | — | deleted | … | superseded with a banner; the walk it designs has had no code since M1e2 S1e2.1c.5; `finding_r.md` and `solution_semantics.md` state the current search |
| `inference/answer_dump.md` | `user/reference/answer-dump.md` | rewritten into | … | test-held (`answer_dump.rs`); named in `--dump-answer`'s help |
| `inference/architecture_and_algorithms.md` | `reasoning/kb-evolution/firing.md` (§§ 1–5), `reasoning/kb-evolution/cores.md` (the ATMS half), `ein.rs/algorithms.md` (§§ 6–7) | rewritten into | … | split by the question: the paradigms and operations hold of any implementation, where Ein sits and its levers are ein.rs's |
| `inference/domain_elim_vs_hypothesis.md` | — | deleted | … | measured through three entries removed 2026-06-16, harness removed; `examples/domain_elim/` holds the behaviour |
| `inference/events.md` | `ein.rs/events.md` | rewritten into | … | the `ein-events/1` protocol; test-held (`events_reference.rs`, `cli_semantics.rs`); named in `--events`' help; the walk's retired kinds go |
| `inference/features.md` | — | deleted | … | every row measures the walk; `feature_matrix.py` re-takes only against a binary at or before `7ec8a480`; *does it change R* is `configuration.md`'s |
| `inference/finding_r.md` | `reasoning/kb-evolution/finding-r.md` | rewritten into | … | the oracle and the routes as formulas: the map's `search.md` |
| `inference/hypothesis_space.md` | `reasoning/kb-evolution/hypothesis-space.md` | rewritten into | … | test-held (`show_hyps.rs`, `h_space_view.rs`); named in two help texts; the `h-space.json` section may go to `user/reference/` |
| `inference/implementation.md` | `ein.rs/architecture.md` | rewritten into | … | the module map |
| `inference/lattice_diagrams.md` | — | deleted | … | superseded with a banner: the walk's 2026-05 design, older than its companion |
| `inference/lattice_dump.md` | `ein.rs/lattice-dump.md` | rewritten into | … | its first section, the region run's `nodes.json`, is current; the walk's tree sections go; named in `--dump-lattice`'s help |
| `inference/monotonicity.md` | `reasoning/kb-evolution/monotonicity.md` | rewritten into | … | cites five probes that live only in M1e2's folder (§ Findings) |
| `inference/parity_baselines.md` | — | deleted | … | superseded with a banner; frozen wall clock of solvers removed in M1; the `branching` group holds the verdicts |
| `inference/read_out.md` | `user/reference/read-out.md` | rewritten into | … | test-held; moves with `read_out_reference.rs`, not rewritten from a plan |
| `inference/read_out_examples.md` | `user/reference/read-out-examples.md` | rewritten into | … | generated by `utils/read_out_examples.py`, whose own prose links M1e2's plan pages (the generator changes, not the page) |
| `inference/region_dump.md` | `user/reference/region-dump.md` | rewritten into | … | test-held (`region_dump.rs`); its path is written into every oracle dump |
| `inference/reserved_engine_strings.md` | `ein.rs/reserved-engine-strings.md` | rewritten into | … | engine-internal vocabulary, by its own first line |
| `inference/solution_semantics.md` | `reasoning/kb-evolution/solution-semantics.md`, `user/syntax-and-semantics/solutions-and-models.md` | rewritten into | … | the definition is reasoning's, what R means to its reader the user's (the map's *linked, not duplicated*); § 6, where the walk differed from R, goes |
| `inference/zebra_walkthrough.md` | `user/guide/06_zebra_walkthrough.md` | rewritten into | … | near verbatim; `guide_programs.rs` then reads its `lisp` blocks too |
| `ir/01-ein-graph/01_kb.md` | `user/syntax-and-semantics/{objects,relations,facts}.md`, `reasoning/kb-evolution/cores.md` (§ 5) | rewritten into | … | the four representations start here |
| `ir/01-ein-graph/02_rules.md` | `user/syntax-and-semantics/{rules,constraints}.md`, `reasoning/static-ir/rules-as-statements.md` | rewritten into | … | the anatomy for the user; T1–T3 and DPO for reasoning; § 8 (where it lives in code) for `ein.rs/architecture.md` |
| `ir/01-ein-graph/03_ein_model.md` | `user/syntax-and-semantics/ontology.md` | rewritten into | … | § 7's seams were resolved (2026-05-20): their answers are decisions, not prose |
| `ir/01-ein-graph/04_jack_drinks_coffee.md` | `user/guide/05_jack_drinks_coffee.md` | rewritten into | … | already in four representations: the format S1f.20.3 takes |
| `ir/01-ein-graph/05_four_level_kb.md` | `reasoning/static-ir/rules-as-statements.md` | rewritten into | … | |
| `ir/01-ein-graph/README.md` | `user/syntax-and-semantics/README.md` | rewritten into | … | an index |
| `ir/02-data-model/01_entities.md` | `reasoning/static-ir/names-and-atoms.md`; record shapes → `ein.rs/design/data-model.md` | rewritten into | … | |
| `ir/02-data-model/02_store.md` | `reasoning/static-ir/relations-and-facts.md`; record shapes → `ein.rs/design/data-model.md` | rewritten into | … | |
| `ir/02-data-model/03_implementation.md` | `ein.rs/architecture.md` | rewritten into | … | |
| `ir/02-data-model/README.md` | `reasoning/README.md` | rewritten into | … | an index |
| `ir/03-ein-lang/00_ebnf.md` | `user/reference/grammar.md` | rewritten into | … | generated: `grammar_pages.rs` writes its two blocks (`EIN_BLESS=1`); only the prose around them is edited |
| `ir/03-ein-lang/01_grammar.md` | `user/reference/syntax.md`, `user/syntax-and-semantics/{macros,query,hypotheses}.md` | rewritten into | … | the reference for the lexer and forms; the meaning of each form to its concept page |
| `ir/03-ein-lang/02_patterns.md` | `user/reference/syntax.md`, `user/syntax-and-semantics/rules.md` | rewritten into | … | |
| `ir/03-ein-lang/03_examples.md` | `user/guide/examples.md` | rewritten into | … | worked fragments; *"this was `docs/ir.md` § 5"* goes |
| `ir/03-ein-lang/04_dot_rendering.md` | `user/reference/rendering.md` | rewritten into | … | named in `ein render`'s help; § Reverse parse (*designed, never built*) is not docs (§ No obvious destination) |
| `ir/03-ein-lang/05_inspirations.md` | `overview/inspirations.md` | rewritten into | … | S1f.20.7 |
| `ir/03-ein-lang/06_reserved_names.md` | `user/reference/reserved-names.md` | rewritten into | … | |
| `ir/03-ein-lang/07_stdlib_api.md` | `user/reference/stdlib.md`, `user/syntax-and-semantics/properties-and-operators.md` (std.algebra) | rewritten into | … | has no section for `std.slots` or the four `std.bool.*` modules (§ Findings) |
| `ir/03-ein-lang/08_self_describing.md` | `reasoning/rule-evolution/rules-as-data.md` | rewritten into | … | the operational half; the design-only half is M12's work, not docs |
| `ir/03-ein-lang/09_natural_language.md` | `user/syntax-and-semantics/natural-language.md` | rewritten into | … | test-held (`natural_language_page.rs`) |
| `ir/03-ein-lang/README.md` | `user/reference/README.md` | rewritten into | … | an index |

**The diagrams** (`inference/diagrams/`, 28 files, not pages; each group goes with its page):

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `algorithm_layer_n.{dot,svg}` | — | deleted | … | with `algorithm_layer_n.md` |
| `lattice_3hyps_{contradiction,dedup,no_dedup}.{dot,svg}`, `lattice_4hyps_dedup.{dot,svg}` | — | deleted | … | with `lattice_diagrams.md` |
| `monotonicity_*.{dot,svg}` (7 pairs) | `reasoning/kb-evolution/diagrams/` | rewritten into | … | with `monotonicity.md`, which shows the command that redraws them |
| `region_02_genuine_3set_death.{dot,svg}` | `reasoning/kb-evolution/diagrams/` | rewritten into | … | with `solution_semantics.md` |
| `region_02_genuine_3set_death_no_lookahead.{dot,svg}` | — | deleted | … | the walk's `-e -L` run drawn; goes with § 6 |

### `docs/api/`, `docs/guide/`, `docs/install.md` (8 pages)

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `docs/api/README.md` | `ein.rs/README.md` | rewritten into | … | an index |
| `docs/api/rust.md` | `ein.rs/api.md` | rewritten into | … | the marked region moves intact with `embedding.rs`'s diff test, in one commit (S1f.20.5) |
| `docs/guide/README.md` | `user/guide/README.md` | rewritten into | … | near verbatim; `guide_programs.rs` and `guide_transcripts.rs` name the directory and each page |
| `docs/guide/01_objects_and_relations.md` | `user/guide/01_objects_and_relations.md` | rewritten into | … | near verbatim |
| `docs/guide/02_first_rules.md` | `user/guide/02_first_rules.md` | rewritten into | … | near verbatim |
| `docs/guide/03_rule_families.md` | `user/guide/03_rule_families.md` | rewritten into | … | near verbatim |
| `docs/guide/04_solving_the_whole_puzzle.md` | `user/guide/04_solving_the_whole_puzzle.md` | rewritten into | … | near verbatim |
| `docs/install.md` | `user/install.md` | rewritten into | … | *"Written at M1a S1a.9.3"* goes; `release.yml` writes its URL into every release's notes |

### `docs/history/` (77 files: 73 pages, a log and three probe files; sources relative to `docs/history/`)

**The specification-bearing pages** are the M1a design contracts (the crates cite `m1a_rust/design/` 85 times),
the sections of the milestone READMEs that code cites as its reason (`m1a_rust/README.md` 96 times,
`m1d_satisfiability/README.md` 25, `m1c_external_validation/README.md` 17), `shared_state.md` § 4, and M1d's
normative pages; each count by `grep -rhoE 'history/<path>' ein.rs/crates | wc -l` on 2026-11-16. None of them is
*deleted*: each is rewritten into the page that states its behaviour today, and only what describes gone code is
dropped. `m1e_review_processing/README.md` (30 by the same grep) is deleted: each citation names why a fix
exists, and the fixed behaviour is stated in `defined_behaviour.md` and `configuration.md` already.

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `README.md` | — | deleted | … | the directory goes (S1f.20.9) |
| `m1a_rust/README.md` | `ein.rs/design/*` (the sections code cites: P1a.1, P1a.2, S1a.2.3, S1a.6.3, S1a.6.5, S1a.6.6, S1a.6.8, S1a.6.9, S1a.6.12, P1a.7, S1a.7.1, S1a.7.2, S1a.7.5, P1a.8, P1a.10) | rewritten into | … | the record goes; the stage sections code cites as its reason are the only written intent for the resumed saturator, the per-entering snapshot and the `--jobs` contract |
| `m1a_rust/design/README.md` | `ein.rs/design/README.md` | rewritten into | … | |
| `m1a_rust/design/02_determinism_and_order.md` | `ein.rs/design/determinism-and-order.md` | rewritten into | … | 20 Rust citations; § 6a (the no-good store under `--jobs`) went with the walk |
| `m1a_rust/design/03_data_model.md` | `ein.rs/design/data-model.md` | rewritten into | … | 12 Rust citations |
| `m1a_rust/design/04_ir_frontend.md` | `ein.rs/design/ir-frontend.md` | rewritten into | … | § 1's decision (*hand-written, not generated*) was reversed by the grmtools parser; the lexer, parser and import sections are rewritten from `ein.l`/`ein.y` and imports by full name |
| `m1a_rust/design/05_matcher.md` | `ein.rs/design/matcher.md` | rewritten into | … | 10 Rust citations |
| `m1a_rust/design/06_saturation.md` | `ein.rs/design/saturation.md`; § 1 → `reasoning/kb-evolution/saturation.md` | rewritten into | … | 7 Rust citations |
| `m1a_rust/design/07_search_layer.md` | `ein.rs/design/search.md` (§§ 2, 6, 7) | rewritten into | … | the commitment lattice, apriori, no-goods and verdicts went with the walk; the generator (`hypgen.rs`, `oblgen.rs` cite § 2), canonical states and cores remain |
| `m1a_rust/design/08_parallelism.md` | `ein.rs/design/parallelism.md` (§§ 1, 6) | rewritten into | … | level 1 was the walk's enterings and levels 2–3 were never built; the contract and § 6 (what must be `Sync`, 5 citations) are current |
| `m1a_rust/design/10_binary_format.md` | `ein.rs/design/binary-format.md` | rewritten into | … | 13 Rust citations |
| `m1a_rust/design/11_shared_assets.md` | `ein.rs/design/shared-assets.md` | rewritten into | … | |
| `m1a_rust/design/12_toolchain_and_layout.md` | `ein.rs/design/toolchain-and-layout.md` | rewritten into | … | the crate list grew to eight; `release.yml` cites its § 4 |
| `m1a_rust/measurements/baseline.md` | — | deleted | … | the P1a.5 build measured; its 7 Rust citations become statements in the design pages (S1f.20.6) |
| `m1a_rust/measurements/corpus_cost.md` | `ein.rs/benchmarks/corpus-cost.md` | evidence | … | `utils/corpus_cost.py` runs on today's engine, and `slow = true` is its measured claim (`manifest.rs`, `corpus_cli.rs`) |
| `m1a_rust/measurements/feature_cost.md` | — | deleted | … | its instrument `e2e_baseline.py` is one of the walk's; AGENTS.md's snmalloc figure rests on it (§ Findings) |
| `m1a_rust/measurements/fork_delta_trace.md` | — | deleted | … | Q-M1a.18's evidence; the decision becomes its file |
| `m1a_rust/measurements/scaling.md` | — | deleted | … | measured with `ein solve -e -t`, the walk's flags; `cmdline.rs`'s citation becomes a statement |
| `m1a_rust/measurements/shared_state.md` | `ein.rs/design/parallelism.md` (§ 4, what it chooses) | rewritten into | … | 8 Rust citations to its choice of lock; its counts (an instrument build) go |
| `m1a_rust/open_questions.md` | `docs/decisions/` (S1f.20.8) | deleted | … | a ledger: its decisions become files first |
| `m1c_external_validation/README.md` | `user/syntax-and-semantics/query.md` (S1c.1.2, `:expect`), `user/reference/cli.md` (S1c.1.3, `ein test`) | rewritten into | … | 17 Rust citations to the claim's meaning and to `ein test` |
| `m1c_external_validation/open_questions.md` | `docs/decisions/` | deleted | … | a ledger |
| `m1c_external_validation/stdlib_census.md` | `ein.rs/benchmarks/stdlib-census.md` | evidence | … | `utils/stdlib_census.py` runs today; re-taken, not copied |
| `m1d_satisfiability/README.md` | `user/syntax-and-semantics/obligations.md` (S1d.2.3, S1d.2.4), `reasoning/kb-evolution/{hypothesis-generation,satisfiability}.md` (S1d.2.5, P1d.3, P1d.4) | rewritten into | … | 25 Rust citations, 7 of them to S1d.2.3, the form; § Eight measurements with no owner all measured the walk, and one was closed by M1e2 |
| `m1d_satisfiability/closure_census.md` | — | deleted | … | `closure_census.py` is one of the walk's instruments |
| `m1d_satisfiability/completeness.md` | `reasoning/kb-evolution/satisfiability.md` | rewritten into | … | `solution_semantics.md` cites it for why an obligation's alternatives are jointly exhaustive; the tree traversal it argues for is gone |
| `m1d_satisfiability/domain_contract.md` | `user/syntax-and-semantics/obligations.md` | rewritten into | … | C1–C4, normative by `solution_semantics.md` |
| `m1d_satisfiability/hypotheses_from_obligations.md` | `reasoning/kb-evolution/hypothesis-generation.md` | rewritten into | … | the ladder ships (`oblgen.rs`); its measurements used `EIN_OBLIGATION_CHOICE`, deleted with the walk |
| `m1d_satisfiability/ideas.md` | `plans/ideas/` | moved | … | decided by the phase: the user's own note, authoritative on intent |
| `m1d_satisfiability/lattice_exhaustive_run.log` | — | deleted | … | one run of the walk |
| `m1d_satisfiability/layer_census.md` | — | deleted | … | the walk's layers, by one of its instruments |
| `m1d_satisfiability/model_set_census.md` | — | deleted | … | read the walk's `verdict.solutions` |
| `m1d_satisfiability/obligation_forms.md` | `user/syntax-and-semantics/obligations.md` | rewritten into | … | form G, `(open ?R)`, as built; the menu of forms not taken goes |
| `m1d_satisfiability/open_questions.md` | `docs/decisions/` | deleted | … | a ledger |
| `m1d_satisfiability/openness_census.md` | — | deleted | … | one of the walk's instruments; `solution_semantics.md` restates the scope rule without it |
| `m1d_satisfiability/property_audit.md` | — | deleted | … | a 2026-08-25 audit of 73 rules; what each rule enforces today is `07_stdlib_api.md`'s (least sure) |
| `m1d_satisfiability/representations.md` | — | deleted | … | the pricing behind Q-M1d.5; the decision becomes its file |
| `m1d_satisfiability/the_boundary.md` | `reasoning/kb-evolution/satisfiability.md` | rewritten into | … | the second-order boundary, an argument of no implementation |
| `m1d_satisfiability/the_verdict.md` | — | deleted | … | Q-M1d.5's answer, a decision; the verdict it qualified is gone and `read_out.md` states `--models key` |
| `m1d_satisfiability/the_vocabulary.md` | — | deleted | … | *no keyword*: a decision, and its file |
| `m1e_review_processing/README.md` | — | deleted | … | the record; its 30 Rust citations name why a fix exists, and the fixed behaviour is stated in `defined_behaviour.md` and `configuration.md` already |
| `m1e_review_processing/open_questions.md` | `docs/decisions/` | deleted | … | a ledger (Q-M1e.1 and .2 are `standard_of_proof.md`) |
| `m1e_review_processing/review/open-questions.md` | `docs/decisions/` | deleted | … | the review's Q1–Q10, answered in M1e's P1e.1; their ids collide with the global `Q1`–`Q10` (§ For the review) |
| `m1e_review_processing/review/summary.md` | — | deleted | … | the review's reports, verbatim; every finding was dispositioned |
| `m1e_review_processing/review/architecture/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/code-doc-consistency/high.md` | — | deleted | … | same |
| `m1e_review_processing/review/code-doc-consistency/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/code-doc-consistency/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/correctness/high.md` | — | deleted | … | same; 2 Rust citations become statements |
| `m1e_review_processing/review/correctness/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/correctness/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/documentation/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/documentation/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/error-handling/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/error-handling/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/maintainability/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/maintainability/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/semantics/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/semantics/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/state-model/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/state-model/low.md` | — | deleted | … | same |
| `m1e_review_processing/review/tests/medium.md` | — | deleted | … | same |
| `m1e_review_processing/review/tests/low.md` | — | deleted | … | same |
| `m1e_review_processing/search_soundness_probes/README.md` | — | deleted | … | S1e.1.1's record; its rulings are Q-M1e decisions |
| `m1e_review_processing/search_soundness_probes/d1_q4_which_route_reaches_the_site.md` | — | deleted | … | a decision record; the fixtures are banked in `examples/ein-bugs/` |
| `m1e_review_processing/search_soundness_probes/d2_q6_which_decline_to_construct.md` | — | deleted | … | moved to P1f.10 as a stage, now closed |
| `m1e_review_processing/search_soundness_probes/d3_q_m1e8_file_or_take.md` | — | deleted | … | Q-M1e.8's record |
| `m1e_review_processing/search_soundness_probes/d4_q_m1e9_upward_closure.md` | — | deleted | … | Q-M1e.9's record; the refusal is `defined_behaviour.md` § 4.7, the witness `examples/ein-bugs/naf-upward-closure.ein` |
| `m1e_review_processing/search_soundness_probes/d5_does_t1_ratify_q_m1e2.md` | — | deleted | … | the ratification; its content is `standard_of_proof.md` |
| `m1e_review_processing/search_soundness_probes/d6_the_new_q5_fixture.md` | — | deleted | … | a stage file's earlier copy |
| `m1e_review_processing/search_soundness_probes/d7_the_diff_instrument.md` | — | deleted | … | a decision record |
| `m1e_review_processing/search_soundness_probes/d8_branching06_untyped_models.md` | — | deleted | … | a decision record |
| `m1e_review_processing/search_soundness_probes/d9_kernel_page_overclaims.md` | — | deleted | … | done 2026-08-28; the page it fixed carries the fix |
| `m1e_review_processing/search_soundness_probes/golden_audit.md` | — | deleted | … | the walk's goldens |
| `m1e_review_processing/search_soundness_probes/probes/naf_upward_closure.ein` | — | deleted | … | banked as `examples/ein-bugs/naf-upward-closure.ein` with a corpus entry |
| `m1e_review_processing/search_soundness_probes/probes/run_naf_upward_closure.sh` | — | deleted | … | drives the walk's flags |
| `m1e_review_processing/search_soundness_probes/probes/run_record_sites.sh` | — | deleted | … | drives `examples/ein-bugs/` fixtures the corpus already runs |

### The user-facing reference under `plans/`, and the pages of finished plan folders that the code or a current page cites as a statement

The stage file names `solve_interface.md` as *the one user-facing reference still under `plans/`*. It is not the
only page of a finished plan that shipped code cites as its specification: the rows below are every page of a
finished folder ([§ Plan folders](#plan-folders-finished-and-staying)) cited from outside `plans/` and
`docs/history/` other than a stage file, a ledger or a phase/milestone README. **Stage files** cited from outside
`plans/` (121 of them and 363 references on 2026-11-16, resolving every path in a tracked `.md`, `.rs`, `.ein`,
`.toml`, `.py`, `.sh`, `.yml` or `.txt` outside `plans/` and `docs/history/`) are reasons, not statements: S1f.20.6's
substitution 3 restates each citation as what the code does. The M1e2 `read_out.md` and `read_out_examples.md`
are records since S1f.5.16 and get no row, but code still cites the first (`described.rs`, `downset.rs`,
`trace/region.rs`): those citations are re-aimed at `user/reference/read-out.md`.

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md` | `user/reference/cli.md` (§§ 1, 2, 4), `reasoning/kb-evolution/finding-r.md` (§§ 3.1–3.3), `ein.rs/design/search.md` (§ 3.4) | rewritten into | … | the contract every mode is held to, and each mode's route (`tree.rs` cites § 3.2 as its specification); §§ 0, 6, 7 are rulings, and decisions |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md` | `user/syntax-and-semantics/query.md` | rewritten into | … | § 1, the claim rules, is what `claim.rs` implements; `01_grammar.md` and `read_out_examples.py` cite it |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/region_language.md` | `user/syntax-and-semantics/query.md` | rewritten into | … | §§ 3–5, `(solve …)`, the region claim and *a query asks one question* (`expect.rs` cites § 5); the rest is the ruling |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/definitions.md` | — | deleted | … | the table `solution_semantics.md` was written from, which carries it |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/region_census.md` | `ein.rs/benchmarks/region.md` | evidence | … | `utils/region_census.py` runs on today's `ein oracle --dump` (least sure: it was the evidence for a ruling) |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/route1_reach.md` | `ein.rs/benchmarks/region.md` | evidence | … | `--example route1_reach`, in AGENTS.md's command block |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/walk_and_region.md` | — | deleted | … | the walk |
| `plans/m1e2_correctness/p1e2.1a_the_solution_region/walk_census.md` | — | deleted | … | the walk |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/detector_proposal.md` | `reasoning/kb-evolution/monotonicity.md` (detection), `ein.rs/design/search.md` | rewritten into | … | `detector.rs` and `explore.rs` cite it as the detector's design |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md` | `reasoning/kb-evolution/monotonicity.md` | rewritten into | … | § 5, the grain, is `monotone.rs`'s cited definition; its numbers go to `non-monotone.md` |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md` | `ein.rs/benchmarks/non-monotone.md` | evidence | … | `--example node_census`; `monotonicity.md` cites its numbers (least sure) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md` | `ein.rs/benchmarks/non-monotone.md` | evidence | … | `--example hole_census` (least sure) |
| `plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md` | `ein.rs/benchmarks/non-monotone.md` | evidence | … | `--example taint_census` (least sure) |
| `plans/m1e2_correctness/p1e2.1c_the_region_search/alive_set_reach.md` | `ein.rs/design/search.md` | rewritten into | … | `states.rs` cites § 3 (states beside commitments), `downset.rs` and `explore.rs` § 9 |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/design.md` | `user/reference/stdlib.md` | rewritten into | … | the calculator as built (digits are names, the tables, requests and results); the proposal and prototype go (the user's question) |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_arithmetic.md` | `user/reference/stdlib.md` | rewritten into | … | `std.math.binary*` and `convert` as built; `examples/binary_arithmetic/` cites it |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/binary_logic.md` | `user/reference/stdlib.md` | rewritten into | … | `std.bool.*`, which `07_stdlib_api.md` lacks; `examples/binary_logic/` cites it |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/malformed_numbers.md` | `user/reference/stdlib.md` | rewritten into | … | the flags and *a malformed value is its representation's* |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md` | `user/guide/word-problems.md` | rewritten into | … | the results, beside `examples/word_problems/` (the user's question) |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/beyond_arithmetic.md` | `user/guide/word-problems.md` | rewritten into | … | the results, beside `examples/beyond_arith/` (the user's question) |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/import_clashes.md` | — | deleted | … | its proposals were ruled; imports by full name are `syntax.md`'s and the rulings decisions |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md` | — | deleted | … | closed or owned at the phase's close; `examples/` READMEs that cite it state the behaviour instead |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/close.md` | — | deleted | … | a close record |
| `plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/close.md` | — | deleted | … | a close record; its results are `hypothesis_space.md` |
| `plans/followups/f9_e_catalog.md` | — | deleted | … | closed; nine measured rejections of the walk's search-layer optimisations |
| `plans/followups/f18_world_aware_negatives.md` | — | deleted | … | closed without being done; the *forbid* refusal is `defined_behaviour.md` § 4.7 |
| `plans/m1j_nlp_smt_poc/README.md` | — | deleted | … | done; `nl2smt/README.md` is the page |

### The question ledgers

Each ledger's decisions become files under `docs/decisions/` ([S1f.20.8](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.8_decisions.md)),
the final answer only; its open rows stay in `plans/` as work. A ledger is deleted with its folder: now, if
the folder is finished; at its milestone's close, if not. The five ledgers under `docs/history/` are rows above.

| source | destination | disposition | in | reason |
|---|---|---|---:|---|
| `plans/m1e2_correctness/open_questions.md` | `docs/decisions/` | deleted | … | M1e2 closed; the most-cited ledger outside `plans/` (Q-M1e2.7, the region) |
| `plans/m1f_hypothesis_and_documentation/open_questions.md` | `docs/decisions/` | deleted | … | at M1f's close, with its folder; its answered rows become files now |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md` | `docs/decisions/` | deleted | … | numbered locally: a decision takes the next free Q-M1f id (S1f.20.8's default 3) |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md` | `docs/decisions/` | deleted | … | same |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md` | `docs/decisions/` | deleted | … | the user's replies, the answers to the two files above |
| `plans/open_questions.md` | `docs/decisions/` (answered rows) | rewritten into | … | the global `Q<n>` ledger stays, holding its open rows; an answered row becomes a decision |
| `plans/m1g_logic/open_questions.md` | `docs/decisions/` (answered rows) | rewritten into | … | M1g is open: the ledger stays until it closes |
| `plans/m1h_beyond_word_problems/open_questions.md` | `docs/decisions/` (answered rows) | rewritten into | … | open milestone |
| `plans/m2_nl_to_ir/open_questions.md` | `docs/decisions/` (answered rows) | rewritten into | … | open milestone |
| `plans/m10_external_benchmarks/open_questions.md` | `docs/decisions/` (answered rows) | rewritten into | … | open milestone |

Not ledgers, and without a row: M1e2's `s1e2.3.4_the_inherited_questions_and_the_record.md` and
`s1e2.4.1_the_question_explained{,.ru}.md` (stage files whose questions are rows of M1e2's ledger), and M1h's
`s1h.10.2_fifty_questions_in_five_groups.md` (a school text's questions, the phase's subject).

### Plan folders: finished and staying

**Finished** (S1f.20.9's default 1: a README that says *closed* or *done*); each is *deleted* by S1f.20.9 once its
decisions are files and nothing outside `plans/` links it. The evidence is the line that says so:

| folder | files | evidence | what must leave it first |
|---|---:|---|---|
| `plans/m1e2_correctness/` | 241 | README: *"**Closed 2026-11-08** on the user's go-ahead"* | its fourteen pages of the section above; the ledger's decisions; the probes current pages cite (§ Findings) |
| `plans/m1j_nlp_smt_poc/` | 1 | README: *"**done 2026-11-13**"* | `nl2smt/README.md`'s three links |
| `plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/` | 105 | README: *"**Closed on 2026-11-11**"* | stage citations (`s1f.5.14_nl_required.md` the most cited); S1f.5.15's strata are `analysis-of-rules.md`'s first content; the parser probe goes with it |
| `plans/m1f_hypothesis_and_documentation/p1f.6_calculator/` | 50 | README: *"**Closed 2026-11-15**"* | its nine pages and three ledgers of the sections above |
| `plans/m1f_hypothesis_and_documentation/p1f.7_findings_outside_scope/` | 13 | README: *"**closed 2026-11-11**"* | four `docs/kernel` links to its analysis (F24 holds the open part) |
| `plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/` | 1 | README: *"split and moved to M1g on 2026-11-13"*: a redirect stub, kept only because P1f.6's replies link it | goes with P1f.6 |
| `plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/` | 24 | README: *"**Closed 2026-11-16**"* | `close.md`'s two links |
| `plans/followups/f9_e_catalog.md` | 1 | *"**Status: closed 2026-08-17.**"* | 11 links (`docs/kernel`, `rs`, `utils`) |
| `plans/followups/f18_world_aware_negatives.md` | 1 | *"**Closed without being done, 2026-11-04.**"* | two `docs/kernel` links |

**Staying**, work not finished: `plans/README.md`, `plans/ideas.md`, `plans/ideas/`, `plans/open_questions.md`;
`plans/m1f_hypothesis_and_documentation/` (its README, ledger and `p1f.20_docs_refactor/`, until M1f's closing
commit); `m1g_logic/`, `m1h_beyond_word_problems/`, `m2_nl_to_ir/`, `m5_presentation/`, `m10_external_benchmarks/`,
`m12_rules_as_data/` (its `rule_census.md` is a generated result of a phase that ran, and stays with the open
milestone), `m15_mcp_server/`, `m20_gui/`; and every followup but F9 and F18. **F1b** says *promoted* to M1g with
the file *"kept as the statement of the question"*, and F2, F5, F7, F24 and F26 say *pulled into M12* with the file
staying: none says *done*, so by default 1 they stay.

## For the review

### Counts

By disposition, over the 175 rows above (the 28 diagram files as their 5 rows), by
`awk -F'|' 'NF >= 7 && $4 !~ /disposition|-/ {gsub(/^ +| +$/, "", $4); print $4}' docs/ein/README.md | sort | uniq -c`:

| disposition | rows |
|---|---:|
| rewritten into | 90 |
| evidence | 7 |
| deleted | 77 |
| moved | 1 |

Plus nine finished plan folders, deleted whole once their rows are done, and the staying list.

### The decisions' layout ([S1f.20.8](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.8_decisions.md) T2)

```text
docs/decisions/
  README.md              one line a subject; how an id is cited (a link in a page, the bare id in code)
  language/              names, values, strings, keywords, :why, macros, imports, the grammar's refusals
  answer/                R, a node's status, monotone and flagged programs, cores, counts, exit codes
  hypotheses/            the generator's ladder, hrules, the mechanisms, :no-search, the structure of H
  cli/                   commands, questions and flags, the read-out, dumps and renderings, ein test
  configuration/         the config principle, each lever, the budgets
  stdlib/                one folder a module family: algebra-and-bijection, slots, math, bool
  engine/                ein.rs as built: the parser, the store, parallelism, .einb, the allocator, packaging
  corpus-and-tests/      the manifest, :expect, the register, the stdlib suite, mutants
  gate/                  the tiers, the nightly, the runner's image, the time limits
  documentation/         the standard of proof (Q-M1e.1, Q-M1e.2), numbers in prose (Q-M1e.4), the docs tree
```

A file is `<subject>/<id>-<slug>.md`, `<slug>` the ledger heading's anchor with the id taken off, cut at a word
boundary under 60 characters: `answer/Q-M1e2.7-the-answer-is-a-region-of-the-lattice.md`. Each folder's
`README.md` lists its files, one line a decision. The ledgers' question headings, counted by
`grep -cE '^#{2,4} +(Q[-0-9]|[0-9]+\.|Question)' <ledger>` on 2026-11-16, are some 550 in all, 312 of them
M1f's; S1f.20.8's census classifies each row as decision, open or not carried, and gives the exact counts.

### The defaults the later stages take

[S1f.20.8's four](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.8_decisions.md#the-defaults-each-the-users-to-revise-at-s1f202s-review)
(one file a decision in subject folders; no mark for a default; a local number takes the milestone's next free id;
a decision is cited by its id) and
[S1f.20.9's six](../../plans/m1f_hypothesis_and_documentation/p1f.20_docs_refactor/s1f.20.9_the_history_sweep.md#the-defaults-each-the-users-to-revise-at-s1f202s-review)
(finished means *closed* or *done*; a measurement only as current state; no banner remains; the register stays as
test data; open E-n and D-n rows stay with their owners; commit messages untouched), each the user's to revise.
This table applies them as written.

### The rows I was least sure of

1. **`standard_of_proof.md` → two decisions.** It is Q-M1e.1 and Q-M1e.2 ratified, so by the rulings it is two
   decision files; but it is also a working rule AGENTS.md leans on, and a released tree might want it as a page
   (`docs/ein/README.md` § Keeping this true, or a contributor's page). Question 1.
2. **The evidence rows of M1e2's censuses** (`region_census`, `route1_reach`, `node`/`hole`/`taint_census`).
   Their instruments run today, so they qualify as evidence; but each was taken as the evidence for a ruling,
   which the rulings make decision history. Question 2.
3. **P1f.6's results pages.** The user said on 2026-11-12 *"enough investigations, just write down results in
   documentation"*, so these pages may be the documentation the user asked for; `07_stdlib_api.md` and the
   `examples/` READMEs state most of it already. Rewritten into `stdlib.md` and a guide page by default. Question 3.
4. **`events.md` and `lattice_dump.md` under `ein.rs/`.** Both are outputs a tool consumer reads, as the read-out is;
   they narrate the engine's own steps (enqueue, park, a route's nodes), which is why they are not `user/`'s.
5. **`solution_semantics.md` split in two.** The map said *linked, not duplicated*; the split is by the
   perspective question, and the risk is two pages that drift.
6. **`defined_behaviour.md` §§ 2–3 as reasoning.** They name ein.rs facts (the Mersenne Twister of `--shuffle`, the
   rule application's identity) that a reimplementer must match, so they are a contract, not an implementation map.
7. **`m1a/design/04` and `07`.** Each is rewritten to the engine as it is, which for 04 reverses its first decision
   and for 07 keeps three of nine sections: in both cases most of the destination is new writing, the failure mode
   *a rewrite that loses a specification* names, inverted.
8. **`property_audit.md` deleted.** It is the only table of which half of each algebra property a rule enforces;
   the per-rule reference states each rule's effect, not that table.
9. **The two walkthroughs under `user/guide/`.** `guide_programs.rs` reads every `lisp` block of every page in the
   guide's directory, so moving them there puts them under that test, which is good if they pass and work if not.

### Questions for the user to decide personally

1. **`standard_of_proof.md`**: two decision files (default), or a page of `docs/ein/`?
2. **Measurements**: keep the re-takable censuses as `ein.rs/benchmarks/` evidence, re-taken on today's engine
   (default for `corpus_cost`, `stdlib_census` and M1e2's five), or delete every census and keep only the command
   in `utils/README.md`?
3. **P1f.6's results pages**: documentation to rewrite (default), or investigation records to delete?
4. **The review's Q1–Q10** (M1e): they collide with the global `Q1`–`Q10`. By S1f.20.8's default 3 each answered one
   takes the next free Q-M1e id; the alternative is a prefix, `Q-M1e.R1`.
5. **The walk's seven instruments in `utils/`**, which this table leaves without a page: delete them in S1f.20.9?
   (§ Findings 5.)

### Sources with no obvious destination

- `defined_behaviour.md` § 6, *What is not defined, and is filed*: work items. A followup row, or nothing.
- `04_dot_rendering.md` § Reverse parse, *designed, never built*, and `08_self_describing.md`'s design-only half:
  designs nothing implements are plans, so a `plans/ideas/` note or M12's, not docs.
- `03_ein_model.md` § 7, the open design seams resolved on 2026-05-20: their answers are rows of the global ledger
  (Q27, Q28) and become decisions; the prose has nowhere to go.
- `kernel/architecture.md` § Milestone boundaries and `kernel/README.md` § Which pages to trust: history and a
  rule the rulings retire.
- `glossary.md`'s *Process / project terms* (three task classes, trace fidelity): `overview/` perhaps, once M5 writes it.
- `m1d/README.md` § Eight measurements with no owner: all measured the walk; deleted with the record.

### Findings outside this stage's scope

1. **`07_stdlib_api.md` has no section for `std.slots` or the four `std.bool.*` modules**, though it is the
   per-symbol reference: `grep -n '^## ' docs/kernel/ir/03-ein-lang/07_stdlib_api.md` against
   `find stdlib -name '*.ein'`.
2. **Current pages cite probes that live only in M1e2's finished folder**, which S1f.20.9 deletes:
   `absent_semantics.md:239` (`dual_rail.ein`), `monotonicity.md:906`, `:914`, `:1145`, `:1162`, `:1163`
   (`via_hole.ein`, `features05_scoped.ein`, `features12_scoped.ein`, `fifo_gap_priority.ein`, `probes/region.py`),
   and `utils/region.py:6`, `utils/README.md:85` (`probes/region.py`). Each is banked under `examples/` with a
   corpus entry, or the citation restated, before the folder goes.
3. **Shipped code cites finished-plan pages as its specification** beyond `solve_interface.md`: `claim.rs` →
   `region_examples.md` § 1, `expect.rs` → `region_language.md` § 5, `detector.rs`/`explore.rs` →
   `detector_proposal.md`, `monotone.rs` → `site_census.md` § 5, `states.rs` → `alive_set_reach.md` § 3, and
   `described.rs`, `downset.rs`, `trace/region.rs` → M1e2's `read_out.md`, a record since S1f.5.16. The table
   gives each a row.
4. **The generator of a golden page writes plan links**: `utils/read_out_examples.py:153`, `:206`, `:209`, `:356`
   write links to M1e2's `region_examples.md` and `region_language.md` into `read_out_examples.md`.
5. **AGENTS.md's *"snmalloc … worth 8–16 % of a solve"* has no instrument on today's engine**: it rests on
   `feature_cost.md`, taken by `utils/e2e_baseline.py`, which `utils/README.md` lists among the walk's seven
   instruments (`fork_split`, `e2e_baseline`, `feature_matrix`, `layer_census`, `openness_census`,
   `model_set_census`, `closure_census`) that re-take only against a binary at or before `7ec8a480`.
6. **`release.yml` cites `docs/history/` in comments** (lines 1, 23, 73) and writes `docs/install.md`'s URL into
   every release's notes (line 340): the install page's move and S1f.20.6 both touch it.

## The inbound counts

**Pending.** The **in** column comes from S1f.20.1's checker (`utils/doc_audit.py`, widened to Rust doc comments,
`.ein` headers and `corpus.toml`, with a per-target inbound-count mode), which is being built on `master`. Until it
lands every count is `…`; the command that fills them is written here when it does.
