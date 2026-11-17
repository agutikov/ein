# S1f.20.4 — `docs/ein/reasoning/` — implementation-agnostic

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 6 days.
**Status:** **done 2026-11-17** ([§ Record](#record-2026-11-17)), in parallel with S1f.20.3 and S1f.20.5 on a
branch of its own; the split sources it shares with them are left for S1f.20.6. It was `T1f.5.20.4` until
2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed. Its
`rule-evolution/analysis-of-rules.md` is sequenced with
[S1f.5.15](../p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md), whichever runs second.

## What it writes

Every page the table sends to `docs/ein/reasoning/`.

The hard third, because the source material is *not* implementation-agnostic.
`inference/architecture_and_algorithms.md` (821 lines on 2026-08-28) and the eleven design contracts describe
`ein.rs`. The split is not a move: it is a **rewrite with a question asked of every paragraph**: *would this
still be true of a second implementation?* If yes, it is `reasoning/`; if no, it is `ein.rs/`
([S1f.20.5](s1f.20.5_ein_rs.md)).

Four known traps:

- **`defined_behaviour.md`** is the diagnostics, orderings and error strings. It is both a user-facing
  reference (the diagnostics) and an implementation-agnostic contract (the orderings), so it splits.
- **The M1d pages** are the best implementation-agnostic writing in the repo (`the_boundary`, `completeness`,
  `the_verdict`, `obligation_forms`), and they are also milestone documents in voice. De-scaffolding them is
  most of the wordcount and none of the difficulty.
- **`rule-evolution/analysis-of-rules.md`** is *empty until something fills it*. S1f.5.15 fills it: the rule
  dependency graph, the strata, and what a rule set's structure decides. Sequence the two so the page is
  written once.
- **`kb-evolution/search.md`** was mapped from `inference/algorithm_layer_n.md`, a page about a walk that has
  had no code since M1e2 S1e2.1c.5. What it states now is the region R and its routes
  ([`solution-semantics.md`](../../../docs/ein/reasoning/kb-evolution/solution-semantics.md)), as the redrawn table
  says.

## How it moves a page

Each page moves with its inbound links, in one commit, and its release-voice count falls to 0 with it
([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).

## The method

### Task T1f.20.4.1 — `static-ir/` and `kb-evolution/`

Each page of the table's rows, every paragraph asked the question above.

### Task T1f.20.4.2 — `rule-evolution/`

`rules-as-data.md`, `analysis-of-rules.md` with S1f.5.15's content or after it, and `transformations.md` as a
stub with its question in it.

## Acceptance

- Every row the table sends to `reasoning/` is written there.
- No paragraph under `reasoning/` names a crate, a module or a type of `ein.rs`.
- The link check is green, and the release-voice baseline has no line for a page under
  `docs/ein/reasoning/`.
- `./run_tests.sh` is green.

## Record 2026-11-17

### How it ran

The index and a stub for each of the seventeen pages first (`b3db3a33`), so that pages written in parallel could
link one another by path. Then five forks, each in a worktree of its own: `static-ir/`; firing, saturation,
`absent` and cores; solution semantics, finding R and the hypothesis space; monotonicity; hypothesis generation
and satisfiability. The coordinator wrote `rule-evolution/` and the index. The API's usage limit stopped every
agent once mid-work; each was resumed where it stopped, nothing lost. Each fork branch was merged, not rebased:
four modify/delete conflicts (`finding_r.md`, `monotonicity.md`, `05_four_level_kb.md`, `08_self_describing.md`,
each a page one fork deleted and another re-aimed a link in) taken as the deletion, and two plan rows merged by
hand (M1g P1g.4's README, M12's README). Every commit after a merge has the tracked link check at 0 findings and the
release-voice check passing.

### The pages

| page | rewritten from | it states |
|---|---|---|
| `README.md` | `02-data-model/README.md` (deleted) and new | the question asked of every paragraph, *What holds it*, the reading order in three parts |
| `static-ir/names-and-atoms.md` | `01_entities.md` §§ 1–1.3, 4, 6 | what a name is, the kinds of named thing, a relation as an object, an argument a name or a fact, identity stated once |
| `static-ir/relations-and-facts.md` | `02_store.md` §§ 3, 5, 7, 8, 9 · `01_entities.md` § 3 · `02_rules.md` § 3.1 | a fact, its negation and neither; the store only grows; loading; a fork; provenance as an OR of derivations; equality |
| `static-ir/values-and-order.md` | `defined_behaviour.md` §§ 2.1, 2.2, 2.4, 2.5, 3.2 | no output depends on numbering; the total order of values; H in text order; a state is its facts; a rule application's identity; how a value is printed |
| `static-ir/rules-as-statements.md` | `02_rules.md` §§ 2, 6.3 · `05_four_level_kb.md` (deleted) | first-order, relation-polymorphic and structural rules; a rule as a rewrite (the DPO span); the four levels |
| `kb-evolution/firing.md` | `architecture_and_algorithms.md` §§ 1–5 · `inference/README.md` § Design principles | what Ein answers, the three paradigms, two layers, the main steps, the objects, the core operations, analogs, the design principles |
| `kb-evolution/saturation.md` | `inference/README.md` (alive-set invariant, d=0 completion) · `m1a/design/06` § 1 · `defined_behaviour.md` § 3.3 | the fixpoint, closure and boundary, the agenda, a node resuming root, what is derived before anything is supposed, the alive-set invariant |
| `kb-evolution/absent-and-naf.md` | `absent_semantics.md` (deleted) · `inference/README.md` § NAF · `architecture.md` § The closure/worlds seam (agnostic half) · A&A O3 | worlds, the definition, the boundary, what follows, what is not provided, *forbid*, stratification (a paragraph pointing at the analysis of rules), worked examples |
| `kb-evolution/hypothesis-generation.md` | `m1d/hypotheses_from_obligations.md` · `m1d/README.md` § S1d.2.5 | when H is built, the ladder, hrules, the obligations rung (instances, union, declines, debts only a commitment brings, stuck root), the blind enumerator, scoping and the mechanisms, what H does not depend on |
| `kb-evolution/hypothesis-space.md` | `hypothesis_space.md` (deleted; test-held) | the objects (pairs, the theory's share, groups and covers, owed groups and grids, coupling), the domain, the cost, how to see it, `h-space.json`, what each reading does not tell |
| `kb-evolution/solution-semantics.md` | `solution_semantics.md` §§ 1–4 | the vocabulary in dependency order, R, ∅, the two counts, shape, the worked `lattice/02` with its diagram, model, the two lines |
| `kb-evolution/finding-r.md` | `finding_r.md` (deleted) · `solve_interface.md` §§ 3.1–3.3, § 5 | a node, the oracle, a flagged program, the licences with their arguments, what a run knows dead, each question's route, one-state proofs, why two routes cannot disagree |
| `kb-evolution/monotonicity.md` | `monotonicity.md` (deleted) · `detector_proposal.md` (detection) · `site_census.md` § 5 | R as its node set; state, status, program; the static check (what it reads, the theorem, the grain, the verdict relative to root); the cube; the ladder; the comparison; the drawings; paths; the causes; *forbid*; the detector; how each kind is solved |
| `kb-evolution/cores.md` | A&A O5, O6 and O8's surviving point · `01_kb.md` § 5 | a contradiction, a derivation record, the proof graph, the core, how far minimality goes, what a refutation tells a search |
| `kb-evolution/satisfiability.md` | `m1d/the_boundary.md` · `m1d/completeness.md` · `m1d/README.md` §§ P1d.3, P1d.4 | the two bounds as sentences about one state, the second-order boundary, claims about the set of states, joint exhaustiveness of an obligation's alternatives, a state of R as a model as H scopes it, a set of models and its count |
| `rule-evolution/rules-as-data.md` | `08_self_describing.md` (deleted) | the toolbox (declarators and facts), the four levels in Ein, what describes itself, where it stops |
| `rule-evolution/analysis-of-rules.md` | new: `strata.rs`, S1f.5.15's T3/T4 record | the dependency graph, strata, negative cycles and parity, what the graph decides, where it is shown, the other readings of a rule set |
| `rule-evolution/transformations.md` | new | the question; what Ein does to a rule set at load; the rewrites under which R is held unchanged |

No paragraph names a crate, module, type or function of `ein.rs` (an `awk` over each page outside its *What holds
it* finds none); a page cites the tests that hold its claims in that closing table only.

### The table's rows, by status

**Done, the source deleted with every inbound reference re-aimed**: `inference/absent_semantics.md`,
`inference/finding_r.md`, `inference/hypothesis_space.md` (its tests' path constants, the two help strings and
`help_shape.txt` moved with it), `inference/monotonicity.md`, `ir/01-ein-graph/05_four_level_kb.md`,
`ir/02-data-model/README.md`, `ir/03-ein-lang/08_self_describing.md`; the diagrams `monotonicity_*` (7 pairs) and
`region_02_genuine_3set_death` moved to `docs/ein/reasoning/kb-evolution/diagrams/`.

**Split and left** for S1f.20.6, the references to the sections taken re-aimed from project files:
`defined_behaviour.md` (§§ 2.1, 2.2, 2.4, 2.5, 3.2, 3.3 taken; § 2.3, the float format, and § 3.1, `--shuffle`, a
retired flag, not carried), `inference/README.md` (Design principles, the alive-set invariant, d=0 completion, NAF),
`architecture_and_algorithms.md` (§§ 1–5, O3, O5, O6, O8's surviving point), `architecture.md` (the seam's
agnostic half), `solution_semantics.md` (§§ 1–4), `01_kb.md` (§ 5), `02_rules.md` (§§ 2, 3.1, 6.3), `01_entities.md`
and `02_store.md` (their agnostic halves). `region_02_genuine_3set_death_no_lookahead.{dot,svg}` stays with
`solution_semantics.md` § 6.

**Rewritten, the source left** for S1f.20.9: `m1a/design/06` § 1, `m1d/README.md` (S1d.2.5, P1d.3, P1d.4),
`m1d/completeness.md`, `m1d/hypotheses_from_obligations.md`, `m1d/the_boundary.md`, `solve_interface.md`
(§§ 3.1–3.3 and § 5), `detector_proposal.md` (detection), `site_census.md` (§ 5). The code that cited them as its
specification cites the pages: `oblgen.rs`, `hypgen.rs`, `match_.rs`, `tree.rs`, `explore.rs`, `scope.rs`,
`monotone.rs`, `detector.rs`, `strata.rs`, `models.rs`, `summary.rs`, `test.rs` and their tests.

**Not done**: none.

### Anchor map, for the merge with S1f.20.3 and S1f.20.5

Their rewritten pages link the deleted sources at these anchors; each goes to `docs/ein/reasoning/…`:

- `inference/finding_r.md#N-…` → `kb-evolution/finding-r.md`, §§ 1–7 the same slugs; `#8-what-holds-it` →
  `#what-holds-it`.
- `inference/hypothesis_space.md#X` → `kb-evolution/hypothesis-space.md#X`, every heading kept;
  `#what-holds-this-page` → `#what-holds-it`.
- `inference/absent_semantics.md`: `#worlds` → `absent-and-naf.md#worlds`; `#evaluation-points` →
  `#the-boundary`; `#explicitly-not-provided` → `#what-is-not-provided`; `#the-stratification-diagnostic` →
  `rule-evolution/analysis-of-rules.md#5-where-it-is-shown`; the corollaries → `#what-follows`.
- `inference/monotonicity.md`: `#1-where-it-starts-r-is-its-node-set` → `#1-r-is-its-node-set`; `#2-the-ladder` →
  `#5-the-ladder`; `#3-the-comparison` → `#6-the-two-kinds-compared`; `#4-the-drawings` → `#7-the-drawings`;
  `#5-detection`, `#51-static` → `#3-the-static-check`, `#31-what-it-reads`; `#52-on-the-cube` →
  `#4-what-the-cube-shows`; `#53-the-detector` → `#11-the-detector`; `#6-how-to-solve` →
  `#12-how-each-kind-is-solved`; `#61-…`, `#62-…` → `#121-three-words`, `#122-per-question`; `#7-…`, `#71-…`,
  `#76-…` → `#2-the-state-the-status-and-the-program`; `#72-…` → `#81-three-constructions-of-a-node`; `#73-…` →
  `#82-path-independent--monotone-given-cumulativity`; `#74-…` → `#84-a-state-difference-needs-an-absent`; `#75-…`
  → `#85-the-status-dead-follows-the-state-owes-does-not`; `#77-…` → `#10-forbid-what-is-refused-at-load`; `#78-…`
  → `#what-holds-it`; `#79-…` → `#9-the-causes`; `#64-…` and § 8 have no successor.
- `ir/01-ein-graph/05_four_level_kb.md` → `static-ir/rules-as-statements.md#the-four-levels-a-knowledge-base-describes-itself-in`.
- `ir/03-ein-lang/08_self_describing.md` → `rule-evolution/rules-as-data.md`; old § 3 →
  `#3-what-already-describes-itself`, old § 5 → `#4-where-it-stops-a-rule-is-not-data`.
- Split sources, the sections taken: `solution_semantics.md#1N-…`, `#2N-…`, `#3-model` → the same anchors in
  `kb-evolution/solution-semantics.md` (`#2-the-answer--the-region-r` → `#2-the-answer-the-region-r`,
  `#4-restrictions-and-obligations--the-two-lines` → `#4-restrictions-and-obligations-the-two-lines`);
  `inference/README.md` Design principles → `firing.md#the-design-principles`, the M1 invariant and
  `defined_behaviour.md` § 3.3 → `saturation.md#the-alive-set-invariant`, d=0 →
  `saturation.md#what-saturation-derives-before-anything-is-supposed`, NAF → `absent-and-naf.md#the-boundary`;
  `architecture_and_algorithms.md` §§ 1–5 → `firing.md` (`#what-ein-answers`, `#the-main-steps`, `#the-objects`,
  `#the-core-operations`, `#analogs-in-other-fields`), O5/O6 → `cores.md`; `01_kb.md` §§ 5, 5.1, 5.2 →
  `cores.md#a-derivation-record`, `#the-proof-graph`, `#the-core`; `02_rules.md` § 2 →
  `rules-as-statements.md#three-kinds-by-what-a-pattern-ranges-over` (2.1–2.3 → `#first-order`,
  `#relation-polymorphic`, `#structural`), § 6.3 → `#a-rule-as-a-rewrite`; `defined_behaviour.md` § 2.1 →
  `values-and-order.md#values-are-totally-ordered`, § 2.4 → `#a-state-is-its-facts`, §§ 2.2, 2.5 →
  `#how-a-value-is-printed`, § 3.2 → `#a-rule-applications-identity`; `01_entities.md` and `02_store.md` section by
  section into `names-and-atoms.md` and `relations-and-facts.md`, each heading naming its subject.
- Plan pages: `solve_interface.md` § 3.1 → `finding-r.md#4-a-monotone-program-three-properties-and-the-licences`,
  § 3.2 → `#6-the-routes-question-by-question`, § 3.3 → `#3-a-program-the-static-check-flags`, § 5 →
  `#8-why-two-routes-cannot-disagree`; `site_census.md` § 5 → `monotonicity.md#33-the-grain`;
  `detector_proposal.md` → `monotonicity.md#11-the-detector`.

**Links from these pages into pages S1f.20.3 and S1f.20.5 move**, to re-aim after the merge:
`kernel/inference/read_out.md` (three), `kernel/inference/region_dump.md#the-limit`,
`kernel/ir/03-ein-lang/04_dot_rendering.md#region-rendering--the-cube-and-a-region-run` (two),
`kernel/defined_behaviour.md` (two, one at § 4.7) and `#4-errors-and-exit-codes`,
`kernel/configuration.md#34-the-option-that-decides-loading` (two) and `#33-what-an-option-can-do-to-the-answer`,
`kernel/ir/03-ein-lang/06_reserved_names.md`, `01_grammar.md#relation-declarator` and `#premise-forms-in-match`,
`kernel/ir/01-ein-graph/02_rules.md`, and `history/m1d_satisfiability/domain_contract.md` (into S1f.20.3's
`obligations.md`).

### L-6

Every citation restated; no probe banked, so no corpus or golden change. `dual_rail.ein` in the `absent` page: the
four forms the *forbid* line names, held by `refutation_under_absent.rs`, with
`examples/ein-bugs/naf-upward-closure.ein` the refused witness. `via_hole.ein`: `stale-naf-fork`'s `p-and-q-clash`.
`fifo_gap_priority.ein`: its cause, a reader at an earlier priority, does not exist without `:priority`, and its row
goes. `dual_rail.ein`, `features12_scoped.ein`, `features05_scoped.ein` and `s1e2.1b.7_probes/` in
`monotonicity.md` were cited only by option (E)'s rewrites, open work, not carried. `utils/region.py` and
`utils/README.md` lose the sentence naming `probes/region.py`.

### Tests and generated files changed

`show_hyps.rs`, `h_space_view.rs`, `cmdline.rs`'s two help strings and `golden/help_shape.txt` (the page's move);
doc comments only elsewhere (30 Rust files in all). `.ein` comments in eleven examples and three M1e2 probes, every
line count kept, none quoted in `fuzz_seeds.txt`. `corpus/corpus.toml` (three notes), `corpus/wrong_answers.toml`
(one `reading`), `utils/read_out_examples.py` and the page it writes (a link), `utils/guard_split.py`,
`utils/region.py`, `utils/README.md`, `examples/README.md`, `stdlib/README.md`, and links in eleven decision files.

### Findings outside the stage

1. **A rule may conclude a fact of `relation`**, `(relation a)`, which declares nothing, while `01_grammar.md` says
   `(relation ?R)` *"means precisely declared relation"*: a rule can forge the membership fact. `rules-as-data.md`
   states it. Witness: a program with `(rule r () :match (p ?x) :assert (relation ?x) …)` and `(p a)` under
   `ein saturate --dump` derives `(relation a :rule r)`. S1f.20.3's, with the grammar's page.
2. **Stale comments on gone machinery**, S1f.20.6's: `ein-core/src/prov.rs:156-158` (negative premises *"not yet
   interpreted by any walk"*, which `detector.rs` and `region.rs` read); `detector.rs:378-386` and the headers of
   `examples/lattice/10_fifo_gap.ein`, `11_fifo_gap_static.ein` (`:priority`, *priority 200*);
   `examples/branching/15_*`, `16_*` (the walk's *maximal alive commitment*, `complete`, `k`);
   `ein-infer/src/mt19937.rs`'s header (`--shuffle`, a retired flag); `docs/kernel/README.md:88-96` (a count of
   kernel pages); `docs/kernel/inference/README.md:65` (the as-built listing names `absent_semantics.md`).
3. **No decision file for the relation grain** of the static check: `monotonicity.md` § 3.3 states it, held by
   `region.rs`. `grep -rl grain docs/decisions` prints nothing.
4. **Two claims rest on thin holders**: *exactly one of `p`, `q` is derived* on `features/07_unstratifiable.ein` is
   held only by `corpus_shapes.md5`'s digest; *the cube checks the static check* only by
   `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged` (six fixtures) and the
   oracle-equality tests.
5. **`monotonicity.md`'s cause c8 rests on an open register row** (`f07-rules-swapped`, held by
   `wrong_answers.rs`): when S1f.20.9 moves open rows to `plans/`, its citation is re-aimed.
6. Places that narrate what `defined_behaviour.md` § 3.2 or § 2.5 once said were left for S1f.20.6 rather than
   re-aimed, which would attach a past claim to the new page: `rule_semantics.rs:684,702`, the
   `binding-key-*.ein` comments, `corpus.toml:1081`, `examples/README.md:203`, `corpus/fuzz_findings/`'s three
   pages, `utils/fuzz_engine.py:84`.

### Verification, on the merged branch

Targeted, as the brief asked; `./run_tests.sh` is the coordinator's, after the three parallel stages merge.
`python3 utils/doc_audit.py --links --check --tracked`: 0 findings, links and decision ids.
`python3 utils/check_release_voice.py`: passes; `--list docs/ein/reasoning` prints nothing; the baseline is re-banked
in its own commit, every count it lowers a file this stage touched. `grep -nE 'utils/|run_tests|\.github|AGENTS|plans/'`
over the eighteen pages prints nothing. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`: green.
`cargo test --profile gate`: `ein-cli`'s `show_hyps`, `h_space_view`, `help_surface`, `config_reference`,
`read_out_examples`, `read_out_reference`, `wrong_answers`, `strata_diagnostic`, `model_set_report`; `ein-infer`'s
`strata`, `detector`, `obligation_reports`, `obligation_rung`, `naf_semantics`; `ein-corpus`: every one passes.

### Defaults taken, each the user's to revise

1. A page cites its tests only in a closing *What holds it* table, never in a paragraph.
2. `h-space.json`'s account stays in `hypothesis-space.md` with its test, not in `user/reference/`.
3. `solution-semantics.md` keeps its numbered sections, so *§ 1.4*-style citations still resolve.
4. Not carried: `defined_behaviour.md` § 2.3 (the float format of `ein saturate`'s timing columns, which no
   perspective owns) and § 3.1 (`--shuffle`, retired); solve_interface's L6, which the engine does not print; the
   guard-split prototype (a `utils/` script, its references re-aimed at `monotonicity.md` § 12); the open options of
   `monotonicity.md` § 8 and option (E).
5. `monotonicity.md`'s causes table drops the old c11 (a reader at an earlier priority); c12, c13 become c11, c12.
   The `three_components` drawing is kept, labelled as the cube with the refusal set aside.
6. Plan pages that cite dropped sections of `monotonicity.md` cite it as text at `50739fdb`, not as links.
7. `transformations.md` states its question and answers it as far as today holds, rather than standing empty.
8. The stratification diagnostic's home is `analysis-of-rules.md`; `absent-and-naf.md` keeps a paragraph.
9. Findings are kept here, not as L-rows of the phase's findings page, which S1f.20.3 and S1f.20.5 append to in
   parallel.
