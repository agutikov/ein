# P1f.20 — `docs/ein/`: the tree a released system would have

**Milestone:** [M1f](../README.md)
**Estimate:** ~5 weeks — 7 stages, **25 days**: the sum of the split the stage carried, which had put the
whole at ~23.
**Status:** created 2026-11-08 on the user's instruction: *"convert
plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.20_docs_refactor.md into new phase
p1f.20"*. **Not started.**
**Was S1f.5.20**, a stage of [P1f.5](../p1f.5_the_no_owner_items/README.md), until 2026-11-08. Since it
was written on 2026-08-28, its § How to cut it had proposed splitting it into seven stages and left taking
the split to the user. This phase takes it. No stage had run, so nothing shipped is renumbered:
`T1f.5.20.1`–`.7` are `S1f.20.1`–`.7`, in the same order.
**Depends on:** M1e's [S1e.2.2](../../../docs/history/m1e_review_processing/README.md#s1e22--code--doc-the-canonical-tree),
[S1e.3.7](../../../docs/history/m1e_review_processing/README.md#s1e37--code--doc-consistency-medium) and
[S1e.3.8](../../../docs/history/m1e_review_processing/README.md#s1e38--documentation-medium). The dependency
was hard: those three stages triaged and fixed 38 pages, and a page fixed and then moved is cheap, where a
move followed by a re-triage of 38 renamed files is not. **Satisfied 2026-08-31**: all three ran. It runs
after [P1f.5](../p1f.5_the_no_owner_items/README.md), which edits kernel pages this phase moves. M1f follows
[M1e2](../../m1e2_correctness/README.md), so this phase runs on the tree M1e2 leaves
([§ What moved since it was written](#what-moved-since-it-was-written)).
**Blocks:** nothing. Everything downstream of it wants it and nothing waits on it.
**Source:** the user's note of 2026-08-28, restated in [§ The instruction](#the-instruction).

---

## The instruction

Five things, and they are not one job:

1. **Re-tree.** `docs/ein/{user,reasoning,ein.rs,overview}`, `docs/lib` kept,
   `docs/history` **removed**. Reshape `docs/api`, `docs/guide`, `docs/kernel`,
   `docs/install.md` into it — *"connect each document to the new docs tree,
   but not copy. Rewrite instead"* — dropping
   references to plans / open questions / history, referencing only `ein.rs/`
   and `docs/ein/`, and separating three perspectives: **user surface**
   (problem → solve-as-a-black-box → result), **implementation-agnostic
   reasoning design**, **the concrete `ein.rs` implementation**.
2. **De-historicise the code.** In code, tests and docs, replace
   *"totality OBLIGATIONS: total-owed / surjective-owed (M1d P1d.2 S1d.2.4)"*
   with a reference to a design page — so that *"code, comments and docs [are]
   considered as a complete released state of the software produced from design
   docs, rather than an intermediate development state produced by a number of
   decisions made during the development process."*
3. `docs/ein/overview` — a feature comparison with z3, CVC4, LEAN, K, OpenCog,
   Prolog, Certora.
4. `docs/demo` — the article and presentation *"From Zebra puzzle to
   Autoformalization"*.
5. **EinAF** — LLM autoformalization, semantic analysis, theory synthesis, BBH.

**(3), (4) and (5) already have owners**, and this phase does not take them:
[M5](../../m5_presentation/README.md) Track A *is* the comparison, written
against [`docs/lib/`](../../../docs/lib/README.md)'s twelve catalogue files;
M5's outputs are the paper and the talk, and the paper is Stage P of
[`EinAf.md`](../../m2_nl_to_ir/EinAf.md); (5) is [M2](../../m2_nl_to_ir/README.md)
whole, with BBH at [P2.3](../../m2_nl_to_ir/p2.3_benchmark/README.md) and
[F13](../../followups/f13_puzzles_beyond_zebra/ideas.md). What they lack is not
a plan — it is a **home in the doc tree**, and this phase builds the two
directories and the index rows, empty, so the milestone that fills them has
somewhere to put the file. That is [S1f.20.7](s1f.20.7_the_two_empty_homes.md), and it is one day.

**This phase owns (1) and (2).**

## Context — the size of it

Measured at `9ba2349`, 2026-08-28. [S1f.20.1](s1f.20.1_the_two_checks.md) re-takes it, with the commands
written down, before anything moves.

| tree | files | lines | carries a milestone id | links into `plans/` or `docs/history/` |
|---|---:|---:|---:|---:|
| `docs/kernel` | 38 | 12 724 | 498 lines, 36 files | 105 lines, 24 files |
| `docs/api` | 7 | 1 412 | 41 lines | 19 lines |
| `docs/guide` | 5 | 693 | **1 line** | 0 |
| `docs/install.md` | 1 | 176 | 3 lines | 3 lines |
| `docs/history` | 43 | 20 734 | — it is the history | — |
| `docs/lib` | 13 | 2 507 | 0 | 0 — untouched by this phase |
| `ein.rs/crates/**/*.rs` | 193 | — | **963 lines, 175 files** | **469 lines, 142 files** |
| `stdlib` + `examples` + `tests` `.ein` | 197 | — | 303 lines, 104 files | 24 lines |
| `corpus/corpus.toml` | 1 | 1 150 | 47 lines | 1 line |
| `examples` + `stdlib` + `utils` `README.md` | 3 | 441 | 55 lines | 25 lines |
| `AGENTS.md` | 1 | 632 | 78 lines | 59 lines |
| `README.md` | 1 | 645 | 45 lines | 92 lines |

That is **2 034 lines across 330 files carrying a development-history
reference**. Counted the other way — by link rather than by line — the crates
alone hold **464 doc-comment links into `docs/history/`**: 368 into
`m1a_rust`, 69 into `m1d_satisfiability`, 27 into `m1c_external_validation`.

### The collision, stated once

`docs/history` is not only a record. **106 of those 464 links point at
`m1a_rust/design/`** — the eleven contracts
[`AGENTS.md`](../../../AGENTS.md) describes as *"the contracts the crates cite
as their specification"*, and for several modules they are the **only**
statement of intent that is not the implementation.

So: deleting the directory as a directory is right, and deleting its contents
is not. The rule this phase applies — and it is the repo's own, from
[Q-M1e.3](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
— is **dissolve, don't delete**: every page is triaged into

- **(a) becomes a `docs/ein/` page**, rewritten as an as-built design document
  with the milestone scaffolding removed — the design contracts, and the M1d
  pages the engine is built on;
- **(b) becomes evidence** under `docs/ein/ein.rs/benchmarks/` — the
  re-takable censuses and the measurements, which stay *as measurements* with
  their date and their machine;
- **(c) is deleted**, and lives in git — the phase/stage records, the open-question
  ledgers.

After it, `docs/history/` does not exist, no `docs/ein/` page says *M1d P1d.2*,
and no crate links to a file that is gone. That is the user's instruction
carried out; the sentence above is the one place this plan disagrees with a
literal reading of it, and it disagrees by preserving specifications rather
than by keeping a directory.

## What moved since it was written

The plan was written on 2026-08-28. Taken on 2026-11-08:

- **The link check's markdown half exists.** `utils/doc_audit.py --links --check` has been a gate step since
  M1e S1e.3.8 (`DO-M2`), over `docs`, `plans` and the tree's READMEs (`per-commit.yml`). So
  [S1f.20.1](s1f.20.1_the_two_checks.md) extends that script rather than writing a second checker. It does not read
  a Rust doc comment, a `.ein` header or `corpus.toml`, so those three are still unbuilt. So is the
  release-voice check.
- **`docs/history/` grew from 43 pages to 73** (`find docs/history -name '*.md' | wc -l`). M1e's record moved
  in on 2026-10-14, as a fourth entry. M1e2's may be a fifth by the time this phase runs, since M1f follows
  it.
- **`docs/kernel/` grew from 38 pages to 43.** `inference/answer_dump.md`, `inference/region_dump.md` and
  `inference/monotonicity.md` came with M1e2, and the map below has no row for them.
- **Part of the map is stale.** `inference/algorithm_layer_n.md`, `lattice_diagrams.md` and
  `parity_baselines.md` are superseded, with banners. The walk they describe has had no code since M1e2
  S1e2.1c.5 (2026-11-03), so `search.md ← inference/algorithm_layer_n.md` maps a page onto a search that is
  gone: the answer is now the region R and its routes
  ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)). `inference/features.md`'s
  levers are mostly retired ([`configuration.md`](../../../docs/kernel/configuration.md) lists them).
- **User-facing references live under `plans/`.** The read-out's vocabulary,
  [`read_out.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md), its golden page
  [`read_out_examples.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md), and
  [`solve_interface.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) describe what
  `ein solve` prints, and the map has no row for any of them. Since
  [S1f.5.16](../p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) the first two are records, and the live
  pages are [`docs/kernel/inference/read_out.md`](../../../docs/kernel/inference/read_out.md) and its
  [examples](../../../docs/kernel/inference/read_out_examples.md); `solve_interface.md` is the one left.
- **`CLAUDE.md` is a symlink to `AGENTS.md`.** Both names are one file.

So [S1f.20.2](s1f.20.2_the_disposition_table.md) redraws the target tree against the tree as it is when the
phase runs. The map below is the 2026-08-28 one, kept as the starting point.

## The target tree

The user's outline, with the pages of 2026-08-28 mapped onto it. `→` is *rewritten into*, never *copied*.

```text
docs/
  ein/
    README.md                     ← new: the three perspectives, and how to read them
    user/                         "problem → [solve as a black box] → result"
      syntax-and-semantics/       every concept in four representations:
                                    ein-lang · graph · NL · math
        objects.md                ← ir/01-ein-graph/01_kb.md (objects half)
        relations.md · facts.md   ← ir/01-ein-graph/01_kb.md · ir/02-data-model/01_entities.md
        ontology.md               ← ir/01-ein-graph/03_ein_model.md
        rules.md                  ← ir/01-ein-graph/02_rules.md
          properties-and-operators.md   ← ir/03-ein-lang/07_stdlib_api.md (the algebra half)
          constraints.md          ← the (false) / (not …) upper bound
          obligations.md          ← history/m1d/obligation_forms.md (the (open ?R) lower bound)
        theory.md                 ← new: a rule set, and how rules interact
        macros.md · query.md      ← ir/03-ein-lang/01_grammar.md, split
        solutions-and-models.md   ← inference/solution_semantics.md  (S1e.5.2, already written)
        config.md                 ← S1e.5.1's configuration reference
      guide/                      ← docs/guide/ 01–04, near-verbatim
        …plus 04_jack_drinks_coffee.md and inference/zebra_walkthrough.md
      reference/
        syntax.md                 ← ir/03-ein-lang/{00_ebnf,01_grammar,02_patterns,06_reserved_names}
        config.md · stdlib.md     ← S1e.5.1 · stdlib/README.md + 07_stdlib_api.md
        cli.md                    ← docs/install.md + inference/events.md + the 50 options
        diagnostics.md            ← kernel/defined_behaviour.md
        glossary.md               ← kernel/glossary.md
    reasoning/                    implementation-agnostic
      static-ir/                  the KB up to the AST state
        names-and-atoms.md · relations-and-facts.md   ← ir/02-data-model/{01_entities,02_store}
        rules-as-statements.md    ← ir/01-ein-graph/{02_rules,05_four_level_kb}
      kb-evolution/               after compilation, and onwards
        firing.md · saturation.md ← inference/{README,architecture_and_algorithms}
        absent-and-naf.md         ← inference/absent_semantics.md
        search.md                 ← inference/algorithm_layer_n.md + history/m1a/design/07
        atms-and-nogoods.md       ← inference/architecture_and_algorithms.md (the ATMS half)
        hypothesis-generation.md  ← history/m1d/hypotheses_from_obligations.md
                                    + P1f.10's exclusion / groups results
        satisfiability.md         ← history/m1d/{the_boundary,completeness,the_verdict}
        solution-semantics.md     ← inference/solution_semantics.md (linked, not duplicated)
      rule-evolution/
        rules-as-data.md          ← ir/03-ein-lang/08_self_describing.md
        analysis-of-rules.md      ← new — and S1f.5.15's stratification is its first content
        transformations.md        ← TBD, a stub with the question in it
    ein.rs/
      api.md                      ← docs/api/rust.md (marker-guarded region intact)
      architecture.md             ← kernel/architecture.md + inference/implementation.md
                                    + ir/02-data-model/03_implementation.md
      design/                     ← history/m1a_rust/design/ 02–12, de-scaffolded
      algorithms.md               ← inference/features.md (the levers and what they cost)
      read-outs.md                ← inference/{lattice_dump,lattice_diagrams,events}.md
      benchmarks/                 ← history/m1a/measurements/ + the four M1d censuses
                                    + history/m1c/stdlib_census.md
    overview/                     ← M5 Track A fills it; S1f.20.7 builds it empty
      README.md                   the axes; one page per comparand
      inspirations.md             ← ir/03-ein-lang/05_inspirations.md
  demo/                           ← M5 fills it; S1f.20.7 builds it empty
  lib/                            unchanged
```

One page has no row above, and it is a decision, not an oversight:

- **`docs/history/m1d_satisfiability/ideas.md`** — the user's own, and
  `AGENTS.md` says it is authoritative on intent *"as `plans/ideas/*` is"*.
  **Move to `plans/ideas/`**, which is where the repo already keeps that class.

## Two rules across the stages

The milestone asks for `./run_tests.sh` green at every stage boundary. A refactor spread over seven stages
meets that only by two rules, which the single stage did not need:

1. **A page moves with its inbound links, in one commit.** The link check is a gate step. So each of
   S1f.20.3 to S1f.20.5 rewrites every link to a page it moves, in the commit that moves it, from a `.md`, a
   Rust doc comment, a `.ein` header and `corpus.toml` alike. This is substitution 2 of
   [S1f.20.6](s1f.20.6_dehistoricise_the_code.md), done by the stage that moves the page. S1f.20.6 does the
   other three.
2. **The release-voice check is a ratchet.** [S1f.20.1](s1f.20.1_the_two_checks.md) builds it, and it cannot
   pass before S1f.20.6 has run. So it reads a banked baseline of each file's count, and fails where a count
   rises or a new file appears. Each stage lowers the baseline in the commit that lowers a count. S1f.20.6
   brings every count to 0 and deletes the baseline. `tests/mutation_survivors.txt`, a banked set the gate
   holds, is the precedent.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1f.20.1](s1f.20.1_the_two_checks.md) | The two checks: every link resolves, and the release voice holds | 3 d | the link check reads Rust doc comments, `.ein` headers and `corpus.toml` too; the release-voice check is in `run_tests.sh` as a ratchet; the baseline numbers written down |
| [S1f.20.2](s1f.20.2_the_disposition_table.md) | The disposition table | 2 d | one row per source page, the target tree redrawn, reviewed by the user before anything moves |
| [S1f.20.3](s1f.20.3_user.md) | `docs/ein/user/` — the black box | 5 d | the black-box perspective, guide included |
| [S1f.20.4](s1f.20.4_reasoning.md) | `docs/ein/reasoning/` — implementation-agnostic | 6 d | the implementation-agnostic perspective |
| [S1f.20.5](s1f.20.5_ein_rs.md) | `docs/ein/ein.rs/` — the concrete implementation | 3 d | the concrete one, `rust.md`'s marked region intact |
| [S1f.20.6](s1f.20.6_dehistoricise_the_code.md) | De-historicise the code, and dissolve `docs/history/` | 5 d | 0 ids outside `plans/`; `docs/history/` gone; `cargo doc` green |
| [S1f.20.7](s1f.20.7_the_two_empty_homes.md) | The two empty homes | 1 d | `overview/` and `demo/`, indexed and owned |

**The order.** S1f.20.1, then S1f.20.2, before anything moves. Then S1f.20.3, S1f.20.4 and S1f.20.5, in any
order, each moving its pages with their links. S1f.20.4's `analysis-of-rules.md` is sequenced with
[S1f.5.15](../p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md), so the page is written once. S1f.20.6
runs after all three. S1f.20.7 can run any time after S1f.20.2, since it moves `05_inspirations.md`.

The stage's own § Ordering said to run after M1e's S1e.2.2, S1e.3.7 and S1e.3.8. Those three triaged 38 kernel
pages into current, bannered or history and fixed what drifted; this phase moves and rewrites the survivors.
Reversing the order would mean every finding lands on a path that no longer exists, and the tree is triaged
twice. That is satisfied. The same rule holds for M1e2's edits to kernel pages: fix first, then move.

## Acceptance for the phase

- **`docs/history/` does not exist**, and no reference to it does — not in a
  `.md`, not in a Rust doc comment, not in a `.ein` comment, not in
  `corpus.toml`, not in `AGENTS.md`.
- **`docs/ein/` exists with the three perspectives**, every page reachable from
  `docs/ein/README.md`, and every source page accounted for in the disposition
  table by exactly one of *rewritten into / evidence / deleted*.
- **Two checks are in `./run_tests.sh`**: the link check, over every tracked `.md`, every repo-relative path
  in a Rust doc comment, and the bare paths in `.ein` headers and `corpus.toml`; and the release-voice check,
  with its baseline empty and gone.
- **The history-referencing lines are resolved**, each into one of:
  a link to a `docs/ein/` page, a plain factual statement with no id, or a
  deletion. The per-file count is the census S1f.20.1 takes, and the closing count
  is **0 outside `plans/`**.
- `./run_tests.sh` green at every stage boundary, and `RUSTDOCFLAGS="-D warnings" cargo doc` green. The
  second matters more than usual: 464 doc comments change, and rustdoc's own history here is that it had
  never run until M1c S1c.1.5 and found nineteen defects on the first try.
- **`docs/lib/` is untouched**, and `docs/demo/` + `docs/ein/overview/` exist
  as indexes with their owning milestone named in the first line.

## Risks

- **The tree is rebuilt while another milestone edits it.** This was M1e's three stages when the plan was
  written. It is now [P1f.5](../p1f.5_the_no_owner_items/README.md), M1e2's P1e2.5 until 2026-11-08, which
  edits kernel pages: S1f.5.5 generates `00_ebnf.md`'s blocks, and S1f.5.11 removes `:where` from six pages.
  P1f.5 runs first, so the order holds: fix first, then move. If the user wants this phase first, those
  stage files must be re-aimed at the new paths *before* it starts, not afterwards.
- **A rewrite that loses a specification.** Half of `docs/history` is the only
  written intent for code that ships. The disposition table exists to make each
  such loss a decision with a name on it; the failure mode is a page rewritten
  into a summary of itself. The repo already states the rule —
  *a page rewritten to describe the current engine is neither history nor a
  specification* — and this phase is the largest opportunity to break it.
- **The release-voice check is a blunt instrument.** A grep for `S1a.` will
  hit a test named `s1a_6_9_resumed_saturation`, a golden file name, and a
  changelog. Scope it to doc comments and `docs/ein/`, allow an explicit
  `release-voice-ok:` escape the way `check_hashmap_iteration.py` allows
  `determinism-ok:` — and note that that check's *first finding was the check
  itself*, which is the failure mode to expect here too.
- **Scope, honestly.** This is 25 days of work that fixes no finding and moves no answer. M1f's acceptance
  is unchanged if it is cut whole.

## Connections

- [`AGENTS.md`](../../../AGENTS.md) — the largest single consumer of the
  current tree, and the file that will read most differently afterwards. `CLAUDE.md` is the same file.
- [`README.md`](../../../README.md) — the root page, which already keeps
  **Ein** and **EinAf** apart and is the natural top of `docs/ein/`.
- [Q-M1e.3](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
  — *who owns a page that should be neither fixed nor deleted* — the ruling
  this phase's triage is an application of, at scale.
- [S1e.5.1](../../../docs/history/m1e_review_processing/README.md#s1e51--the-configuration-reference) and
  [S1e.5.2](../../../docs/kernel/inference/solution_semantics.md) — two pages
  written into the *old* tree; both are `docs/ein/user/` pages by this
  phase's map, and both are cited by anchor text rather than section number so
  the move does not break them.
- [S1f.5.15](../p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) — writes the first content of
  `reasoning/rule-evolution/analysis-of-rules.md`.
- [M5](../../m5_presentation/README.md) Track A and
  [`EinAf.md`](../../m2_nl_to_ir/EinAf.md) — the owners of items 3, 4 and 5 of
  the instruction, and the reason this phase builds two empty directories
  instead of five populated ones.
