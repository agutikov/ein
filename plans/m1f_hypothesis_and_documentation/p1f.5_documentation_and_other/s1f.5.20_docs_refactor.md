# S1f.5.20 — `docs/ein/`: the tree a released system would have

**Phase:** [P1f.5](README.md) (Documentation ein does not have)
**Estimate:** ~23 days — the largest single item in M1e. § How to cut it
offers the split, and taking the split is the user's call.
**Depends on:** [P1e.2](../../m1e_review_processing/p1e.2_high/README.md)
[S1e.2.2](../../m1e_review_processing/p1e.2_high/s1e.2.2_code_doc_consistency.md) and
[P1e.3](../../m1e_review_processing/p1e.3_medium/README.md)
[S1e.3.7](../../m1e_review_processing/p1e.3_medium/s1e.3.7_code_doc_consistency.md) /
[S1e.3.8](../../m1e_review_processing/p1e.3_medium/s1e.3.8_documentation.md) — **hard**, and § Ordering
says why: those three stages triage and fix 38 pages, and a page fixed and then
moved is cheap where a move followed by a re-triage of 38 renamed files is not.
**Blocks:** nothing. Everything downstream of it wants it and nothing waits on
it.
**Source:** the user's note of 2026-08-28, reproduced in full in
§ The instruction.

---

## The instruction

Five things, and they are not one job:

1. **Re-tree.** `docs/ein/{user,reasoning,ein.rs,overview}`, `docs/lib` kept,
   `docs/history` **removed**. Reshape `docs/api`, `docs/guide`, `docs/kernel`,
   `docs/install.md` into it — *"connect each document to the new docs tree,
   but not copy. Rewrite instead"* — dropping Python/CPython/PyPy, dropping
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

**(3), (4) and (5) already have owners**, and this stage does not take them:
[M5](../../m5_presentation/README.md) Track A *is* the comparison, written
against [`docs/lib/`](../../../docs/lib/README.md)'s twelve catalogue files;
M5's outputs are the paper and the talk, and the paper is Stage P of
[`EinAf.md`](../../m2_nl_to_ir/EinAf.md); (5) is [M2](../../m2_nl_to_ir/README.md)
whole, with BBH at [P2.3](../../m2_nl_to_ir/p2.3_benchmark/README.md) and
[F13](../../followups/f13_puzzles_beyond_zebra/ideas.md). What they lack is not
a plan — it is a **home in the doc tree**, and this stage builds the two
directories and the index rows, empty, so the milestone that fills them has
somewhere to put the file. That is T7, and it is one day.

**This stage owns (1) and (2).**

## Context — the size of it

Measured at `9ba2349`, 2026-08-28.

| tree | files | lines | carries a milestone id | links into `plans/` or `docs/history/` |
|---|---:|---:|---:|---:|
| `docs/kernel` | 38 | 12 724 | 498 lines, 36 files | 105 lines, 24 files |
| `docs/api` | 7 | 1 412 | 41 lines | 19 lines |
| `docs/guide` | 5 | 693 | **1 line** | 0 |
| `docs/install.md` | 1 | 176 | 3 lines | 3 lines |
| `docs/history` | 43 | 20 734 | — it is the history | — |
| `docs/lib` | 13 | 2 507 | 0 | 0 — untouched by this stage |
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
statement of intent that is not the implementation. Six more measurement pages
carry CPython and PyPy columns that *nothing can re-take*, because the engine
that produced them was deleted at M1a S1a.10.5.

So: deleting the directory as a directory is right, and deleting its contents
is not. The rule this stage applies — and it is the repo's own, from
[Q-M1e.3](../../m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
— is **dissolve, don't delete**: every page is triaged into

- **(a) becomes a `docs/ein/` page**, rewritten as an as-built design document
  with the milestone scaffolding removed — the design contracts, and the M1d
  pages the engine is built on;
- **(b) becomes evidence** under `docs/ein/ein.rs/benchmarks/` — the four
  re-takable censuses and the measurements, which stay *as measurements* with
  their date and their machine;
- **(c) is deleted**, and lives in git — the phase/stage records, the open-question
  ledgers, the oracle ledger, the divergence list.

After it, `docs/history/` does not exist, no `docs/ein/` page says *M1d P1d.2*,
and no crate links to a file that is gone. That is the user's instruction
carried out; the sentence above is the one place this plan disagrees with a
literal reading of it, and it disagrees by preserving specifications rather
than by keeping a directory.

## The target tree

The user's outline, with the current pages mapped onto it. `→` is *rewritten
into*, never *copied*.

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
        analysis-of-rules.md      ← new — and S1f.5.6's stratification is its first content
        transformations.md        ← TBD, a stub with the question in it
    ein.rs/
      api.md                      ← docs/api/rust.md (marker-guarded region intact)
      architecture.md             ← kernel/architecture.md + inference/implementation.md
                                    + ir/02-data-model/03_implementation.md
      design/                     ← history/m1a_rust/design/ 01–12, de-scaffolded
      algorithms.md               ← inference/features.md (the levers and what they cost)
      read-outs.md                ← inference/{lattice_dump,lattice_diagrams,events}.md
      benchmarks/                 ← history/m1a/measurements/ + the four M1d censuses
                                    + history/m1c/stdlib_census.md
    overview/                     ← M5 Track A fills it; T7 builds it empty
      README.md                   the axes; one page per comparand
      inspirations.md             ← ir/03-ein-lang/05_inspirations.md
  demo/                           ← M5 fills it; T7 builds it empty
  lib/                            unchanged
```

Four pages have no row above and each is a decision, not an oversight:

- **`docs/api/{ein,ir,kb,inference,trace}.md`** — 1 019 lines of *Python*
  embedding contract, filed as history with a 🏛 banner and kept alive only
  because [Q-M1a.23](../../../docs/history/m1a_rust/open_questions.md)'s three
  trip-wires cite them as the specification a binding would be rebuilt from.
  The instruction *drop Python* and that deferral cannot both hold. **Proposed:
  delete, and record in the same commit that Q-M1a.23's trip-wires now resolve
  to git.** It is the sharpest single collision in the stage and it wants the
  user's word.
- **`inference/parity_baselines.md`** — parity against an engine that no longer
  exists. Delete.
- **`measurements/{baseline,scaling}.md`** — 4 106 lines whose CPython and PyPy
  columns are frozen constants. *Drop Python* would delete the only columns
  that cannot be re-taken. **Proposed: keep the pages under
  `ein.rs/benchmarks/`, and state at the top that the two columns are a
  historical constant** — a measurement is not a Python reference in the sense
  the instruction means.
- **`docs/history/m1d_satisfiability/ideas.md`** — the user's own, and
  `AGENTS.md` says it is authoritative on intent *"as `plans/ideas/*` is"*.
  **Move to `plans/ideas/`**, which is where the repo already keeps that class.

## Ordering

Run **after** P1e.2 S1e.2.2 and P1e.3 S1e.3.7 / S1e.3.8. Those three stages
triage 38 kernel pages into current / bannered / history and fix what drifted;
this stage moves and rewrites the survivors. Reversing the order means every
finding lands on a path that no longer exists, and means triaging a tree twice.

The one thing this stage should do **early**, out of order, is T1 — the link
checker — because P1e.2 and P1e.3 are themselves editing the 127 cross-tree
references in the table above and neither has a way to know when it breaks
one.

## Acceptance

- **`docs/history/` does not exist**, and no reference to it does — not in a
  `.md`, not in a Rust doc comment, not in a `.ein` comment, not in
  `corpus.toml`, not in `AGENTS.md`.
- **`docs/ein/` exists with the three perspectives**, every page reachable from
  `docs/ein/README.md`, and every one of the ~55 source pages accounted for in
  the disposition table by exactly one of *rewritten into / evidence / deleted*.
- **Two checks, both in `./run_tests.sh` beside the existing five:**
  - `utils/check_links.py` — every relative link in every tracked `.md`, and
    every repo-relative path in a Rust doc comment, resolves to a file that
    exists. Anchors too where the target is a heading. **There is no link
    checker in this repo today**, and a 55-page re-tree with 464 inbound code
    links cannot be done by hand without one.
  - `utils/check_release_voice.py` — no file under `docs/ein/`, and no doc
    comment in a shipped crate, contains a milestone / phase / stage / task id
    (`M1x`, `P1x.y`, `S1x.y.z`, `T…`, `Q-M1x.n`) or a path into `plans/`. This
    is the user's rule (2) turned into a gate; without it the voice comes back
    one commit at a time.
- **The ~2 034 history-referencing lines are resolved**, each into one of:
  a link to a `docs/ein/` page, a plain factual statement with no id, or a
  deletion. The per-file count is the census T2 produces, and the closing count
  is **0 outside `plans/`**.
- `./run_tests.sh` green, `RUSTDOCFLAGS="-D warnings" cargo doc` green — the
  second matters more than usual: 464 doc comments change, and rustdoc's own
  history here is that it had never run until M1c S1c.1.5 and found nineteen
  defects on the first try.
- **`docs/lib/` is untouched**, and `docs/demo/` + `docs/ein/overview/` exist
  as indexes with their owning milestone named in the first line.

## Tasks

### Task T1f.5.20.1 — The link checker, first and out of order

`utils/check_links.py`, then a `run_tests.sh` step. Three link classes:

- markdown links, with and without an anchor, in every tracked `.md`;
- repo-relative paths inside Rust doc comments — the `](../../../docs/…)`
  form the crates use 464 times;
- bare paths in `.ein` header comments and in `corpus.toml`.

It exits 1 on a target that does not exist, and reports the referrer. Run it
**before** anything moves and record the baseline: the tree is not known to be
clean, and [DO-M2](../../m1e_review_processing/README.md#the-findings) is *dangling references across
the doc tree, incl. anchors that never existed*. A checker that starts red is
fine; a checker whose starting number nobody wrote down is not.

### Task T1f.5.20.2 — The disposition table

One row per source file — ~55 doc pages plus the 43 under `docs/history/` —
with: destination path, disposition (*rewritten into* / *evidence* / *deleted*),
inbound link count, and the one-line reason. It is the stage's contract and it
is reviewed **before** a single file moves.

Produce the inbound counts mechanically (T1's checker already parses them);
produce the destinations by hand. The table lives at
`docs/ein/README.md` § Where each page went until the stage closes, and then
in the commit message.

### Task T1f.5.20.3 — `docs/ein/user/` — the black box

The cheapest and most valuable third. `docs/guide/` moves nearly verbatim (**1**
history line in 693), and the reference pages are assembly rather than writing.

Two things are genuinely new and are the reason the perspective split is worth
doing:

- **`syntax-and-semantics/` states each concept in four representations** —
  ein-lang, graph, NL, math — as the user's outline asks. That is a *format*
  decision applied uniformly, and it is the thing no current page does: the
  grammar states the syntax, `01-ein-graph/` states the graph, nothing states
  the maths, and the NL is [S1f.5.5](s1f.5.5_nl_required.md)'s subject. Pick
  the format once, in `objects.md`, and hold it.
- **`theory.md`** — a rule set as a *theory*, and how rules interact. Nothing
  in the tree says this today, and it is the concept
  [M2](../../m2_nl_to_ir/README.md)'s theory selection and theory synthesis are
  about. It may be the single most load-bearing new page in the stage.

### Task T1f.5.20.4 — `docs/ein/reasoning/` — implementation-agnostic

The hard third, because the source material is *not* implementation-agnostic:
`inference/architecture_and_algorithms.md` (821 lines) and the eleven design
contracts describe `ein.rs`. The split is not a move, it is a **rewrite with a
question asked of every paragraph**: *would this still be true of a second
implementation?* If yes it is `reasoning/`; if no it is `ein.rs/`.

Three known traps:

- `defined_behaviour.md` is *"the thirteen diagnostics, orderings and error
  strings whose only statement used to be a Python source file"* — it is
  simultaneously a user-facing reference (the diagnostics) and an
  implementation-agnostic contract (the orderings). It splits.
- The M1d pages are the best implementation-agnostic writing in the repo
  (`the_boundary`, `completeness`, `the_verdict`, `obligation_forms`) and they
  are also milestone documents in voice. De-scaffolding them is most of the
  wordcount and none of the difficulty.
- `rule-evolution/analysis-of-rules.md` is *empty until something fills it*.
  [S1f.5.6](s1f.5.6_rule_priority.md) is what fills it — the rule dependency
  graph, the strata, and what a rule set's structure decides. Sequence the two
  so this page is written once.

### Task T1f.5.20.5 — `docs/ein/ein.rs/` — the concrete implementation

Mostly relocation, with one rule: **`docs/api/rust.md`'s marker-guarded region
moves intact**, and the test that diffs it against
[`embedding.rs`](../../../ein.rs/crates/ein-cli/tests/embedding.rs) moves with
it. It is the repo's working precedent for a page that cannot rot, and breaking
it in a refactor about documentation quality would be the stage's worst
possible outcome.

`benchmarks/` collects the six measurements and the five censuses; each keeps
its date, its commit and its `utils/` re-take script, and gains nothing else.

### Task T1f.5.20.6 — De-historicise the code

The largest mechanical piece: **963 id-bearing lines in 175 `.rs` files**, plus
303 in `.ein` headers and 47 in `corpus.toml`.

Four substitutions, in decreasing frequency:

1. `(M1d P1d.2 S1d.2.4)` → a link to the `docs/ein/` page that states the
   behaviour. This is the user's own example and it is the common case.
2. A link into `docs/history/m1a_rust/design/NN` → the same file's new home
   under `docs/ein/ein.rs/design/`. Mechanical, and T1's checker verifies it.
3. A sentence whose *subject* is the development event — *"Empty before S1a.6.9
   and not because there was nothing to say"* — where the id is load-bearing
   for the paragraph's meaning. These do not substitute; they get **rewritten
   as statements about the code** (*"A fork that resumes root's saturation does
   not re-derive root's closure, so the givens are told first"*) and the
   history is dropped. There are on the order of a hundred and they are the
   only slow ones.
4. A reference to a question that is still open — 13 links to
   `m1a_rust/open_questions.md`, 2 to `m1d`'s. An open question is a
   development artefact by definition; it moves to
   [`plans/open_questions.md`](../../open_questions.md) and the code comment
   states the *behaviour* and its uncertainty without the id.

`AGENTS.md` and `README.md` are the last file pair touched, and they are
exempt from the release-voice check: they are addressed to someone working
*on* the repo, not to someone using the released system, and the user's rule
is explicitly about code, tests and docs.

### Task T1f.5.20.7 — The two empty homes

`docs/ein/overview/README.md` and `docs/demo/README.md`: an index, the axes,
and a first line naming the milestone that fills it —
[M5](../../m5_presentation/README.md) Track A for the comparison against z3 /
CVC4 / Lean / K / OpenCog / Prolog / Certora, and M5 for *"From Zebra puzzle to
Autoformalization"*, whose five-beat structure the user's note already gives
(enumeration with hardcoded predicates in C → link grammar + SMT → IR as a
reasoning substrate → Ein → theory selection, construction, neural-guided
synthesis). Move `ir/03-ein-lang/05_inspirations.md` under `overview/` as its
first real content, and link `c/README.md` — the three C baselines and the
3 668 465× table *are* beat one of that article, already written and already
measured.

One day. Writing more would be writing M5's milestone for it.

## How to cut it

23 days is a phase, not a stage, and the phase table would be:

| would-be stage | days | ends with |
|---|---:|---|
| the two checks | 3 | `check_links.py` + `check_release_voice.py` in `run_tests.sh`, and the baseline numbers written down |
| the disposition table | 2 | one row per source file, reviewed before anything moves |
| `docs/ein/user/` | 5 | the black-box perspective, guide included |
| `docs/ein/reasoning/` | 6 | the implementation-agnostic perspective |
| `docs/ein/ein.rs/` | 3 | the concrete one, `rust.md`'s marked region intact |
| de-historicise the code | 5 | 0 ids outside `plans/`; `cargo doc` green |
| the two empty homes | 1 | `overview/` and `demo/`, indexed and owned |

Taking it as one stage is defensible only if it is taken as one *commit
series* with the checks in from the start. **Recommendation: split it**, and
run the first two rows early — during P1e.2 — because they are what makes
P1e.2's own edits verifiable.

## Risks

- **The tree is rebuilt while three stages are editing it.** The dependency is
  stated hard for this reason. If the user wants this first, then S1e.2.2 and
  S1e.3.7/3.8 must be re-aimed at the new paths *in their stage files* before
  this starts — not discovered afterwards.
- **A rewrite that loses a specification.** Half of `docs/history` is the only
  written intent for code that ships. The disposition table exists to make each
  such loss a decision with a name on it; the failure mode is a page rewritten
  into a summary of itself. The repo already states the rule —
  *a page rewritten to describe the current engine is neither history nor a
  specification* — and this stage is the largest opportunity to break it.
- **Deleting `docs/api`'s five Python pages closes a deferral that was kept
  cheap on purpose.** Q-M1a.23's whole argument is that a deferral is cheap to
  reverse *only while the specification survives it*. Deleting them is a real
  cost paid for a real gain, and it is the user's call, not this stage's.
- **The release-voice check is a blunt instrument.** A grep for `S1a.` will
  hit a test named `s1a_6_9_resumed_saturation`, a golden file name, and a
  changelog. Scope it to doc comments and `docs/ein/`, allow an explicit
  `release-voice-ok:` escape the way `check_hashmap_iteration.py` allows
  `determinism-ok:` — and note that that check's *first finding was the check
  itself*, which is the failure mode to expect here too.
- **Scope, honestly.** This is a fifth of M1e's entire estimate for work that
  fixes no finding. M1e's [acceptance](../../m1e_review_processing/README.md#acceptance-for-the-milestone)
  does not name it and is unchanged if it is cut whole.

## Connections

- [`AGENTS.md`](../../../AGENTS.md) — the largest single consumer of the
  current tree, 59 lines of `plans/` and `docs/history/` links, and the file
  that will read most differently afterwards.
- [`README.md`](../../../README.md) — the root page, which already keeps
  **Ein** and **EinAf** apart and is the natural top of `docs/ein/`.
- [Q-M1e.3](../../m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
  — *who owns a page that should be neither fixed nor deleted* — the ruling
  this stage's triage is an application of, at scale.
- [S1e.5.1](../../m1e_review_processing/p1e.5_documentation_and_other/s1e.5.1_config_reference.md) and
  [S1e.5.2](../../../docs/kernel/inference/solution_semantics.md) — two pages
  P1f.5 writes into the *old* tree; both are `docs/ein/user/` pages by this
  stage's map, and both are cited by anchor text rather than section number so
  the move does not break them.
- [M5](../../m5_presentation/README.md) Track A and
  [`EinAf.md`](../../m2_nl_to_ir/EinAf.md) — the owners of items 3, 4 and 5 of
  the instruction, and the reason this stage builds two empty directories
  instead of five populated ones.
