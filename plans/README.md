# Plans

Forward-looking roadmap for Ein. The 2021 prototype informs the
design; the cleaned-up package skeleton (`src/ein/`) just unblocks
the work — the actual *implementation* of the design lives here.

The companion research notes (no implementation steps, no schedule)
stay in [`docs/ideas/`](ideas). Plans cross-link to those
ideas where they originate; they do not duplicate them.

## Roadmap at a glance

The big-picture narrative behind the milestone stack (user
framing 2026-05-24):

- **M1 — solve the problem stated in ein.** The first step:
  given a puzzle already encoded in the IR, solve it with the
  graph-native engine. This is what the existing
  M1 core graph reasoning
  delivers; the Zebra puzzle is the acceptance gate.
- **M2 — EinAf: iterative autoformalization, through Level D.** The
  second step, and since 2026-08-23 the whole of the research programme:
  drop the *human encodes the puzzle* assumption, then ask whether the
  kernel's feedback makes the model that replaces the human *better*.
  Level B is the old *NL → IR* — a one-shot formalizer on a benchmark it
  has not seen; Level C is the loop, the baselines and the ablations;
  Level D is the formal account, the frozen benchmark, the released
  records and (with M5) the paper. [M2](m2_nl_to_ir/README.md), reshaped
  around the research plan [`EinAf.md`](m2_nl_to_ir/EinAf.md).
- **M2's theory pass — ontology + rules induction from facts** (the thread
  [F4](followups/f4_cross_cutting.md) / [F7](followups/f7_rule_induction.md)
  carried as "M2+"). No longer beyond M2: the formalizer's theory pass
  ([S2.2.4](m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md)) selects the
  activators *that the puzzle implicitly assumes* from the stdlib catalogue
  and synthesises a rule only when the catalogue lacks the property, so the
  engine (a) actually *can* solve the puzzle rather than sitting on a
  half-typed KB and (b) reflects the common-sense implicits an NL statement
  leans on. Induction *from facts* (F7 B′, F5) stays a followup.

The end-to-end target the milestone stack converges on is the
worked solution in [`examples/README.md`](../docs/kernel/inference/zebra_walkthrough.md): the
human Wikipedia walkthrough of the Zebra puzzle annotated as ein.py
inference (NL ↔ ein rule ↔ branch-depth, plus learnt no-goods). M1
must reproduce the *inference* column — the rule firings, branches,
and contradictions that take the encoded `zebra2.ein` to the final
table. M2's ultimate ambition is the *full* row — `NL problem → facts
→ ontology+rules → solution → NL explanation of solution steps` —
i.e. NL parses into the same `(facts …)` / `(ontology …)` blocks the
engine consumes, and the engine's trace renders back into the same
NL paragraphs the README cites.

Two adjacent secondary milestones surface Ein externally, plus a Rust port
that came before both, the three milestones that came out of it — one created
from its last phases, two by promotion since — and one created out of a review
of the tree they left:

- **M1a — Rust port (ein.rs)** — **shipped 2026-08-23**, between M1 and M2; the
  engine that ships from M2 onward. Its plan folder is gone and the record is
  [`docs/history/m1a_rust/`](../docs/history/m1a_rust/README.md). Two invariants
  held the whole way: a **1:1 observable surface**
  (same language, same CLI, same bytes, with `ein.py` as the parity oracle
  until it had banked everything it could prove) and a **free hand inside**
  (integer-encoded atoms and facts, a register matcher, copy-on-write forks,
  multi-core search). All four landed. It ships a **library and a CLI**;
  server mode was dropped 2026-08-18 and the PyO3 binding deferred 2026-08-21
  for want of a consumer
  ([Q-M1a.23](../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).
- **M20 — GUI** ([m20_gui/](m20_gui/README.md)) after M1a —
  **Tauri 2 + React + Monaco + Cytoscape.js**, linking the ein.rs crates
  directly (stack decided 2026-08-18). Renumbered from M1b 2026-08-23.
- **M1c — External validation** — **shipped 2026-08-24**, and its record is
  [`docs/history/m1c_external_validation/`](../docs/history/m1c_external_validation/README.md).
  The check that is *not* relative to Ein's own past: the stdlib's rules got
  expectations of their own (`:expect` on `query`, `ein test`, 45 programs, and
  a coverage number in the gate). Created 2026-08-21 out of M1a's ex-P1a.11
  plus a benchmark phase that left for M10 on 2026-08-23; the plan tree was
  deleted the day it shipped.
- **M1d — From saturation to satisfiability** — **shipped 2026-08-27**, and
  its record is
  [`docs/history/m1d_satisfiability/`](../docs/history/m1d_satisfiability/README.md).
  Why an under-determined puzzle does not finish, and what saturation lacks to
  be a decision procedure: existence requirements (`total` / `surjective` with
  the force their names claim) as first-class obligations rather than as
  refutations at the extreme — plus the `Open` verdict, `--models key`, and a
  second traversal that reaches 32 models in **86** enterings where the lattice
  needs 17 204 592. Created 2026-08-21 out of M1a's ex-P1a.12 plus the F14
  note; the plan tree was deleted the day it shipped.
- **M1e — Review processing** ([m1e_review_processing/](m1e_review_processing/README.md))
  after M1d — the full-tree review of 2026-08-27, processed: **63 findings**
  in nine topics and **10 open questions**, one phase for the questions and one
  per severity. Its spine is that a finding is a claim until something holds
  it — the review's own verification stage was aborted, so sixty of the
  sixty-three are one reader's reading, and every task ends in a fixture, a
  test, a measured number or a written decision. The reports are carried
  verbatim in the milestone folder.
- **M1f — The structure of the hypothesis set, and the documentation ein does
  not have** ([m1f_hypothesis_and_documentation/](m1f_hypothesis_and_documentation/README.md))
  beside M1e — the two phases M1e carried that were **not** review processing,
  given an M-number on 2026-08-29. `P1f.10` derives a branch structure the
  theory could state in advance (the exclusion relation, groups, the restricted
  join) and may not move one model doing it; `P1f.5` writes pages that do not
  exist, removes `:priority`, and rebuilds the doc tree. Phase numbers 1–4 and
  6–9 are deliberately free.
- **M5 — paper + presentation** ([m5_presentation/](m5_presentation/README.md))
  after M2. Renumbered from M2b 2026-08-23.
- **M10 — External benchmarks** ([m10_external_benchmarks/](m10_external_benchmarks/README.md))
  after M1a — the same problems stated for Z3, CVC5, SWI-Prolog, Soufflé,
  Clingo and Lean, run by one harness, compared on the *answer* first and
  the clock second. M1c's P1c.2 until 2026-08-23, when it was promoted; the
  answers it establishes are checked back in as M1c's `:expect`, so the two
  are still one pipeline.

The followups in [`followups/`](followups/README.md) park the
research-level threads (self-modification F2/F5/F6, formal
foundations F1/F1b, rule induction F7, cross-cutting F4, three
task classes F3).

**Dropped 2026-08-18 — M3 (SMT integration).** The milestone wired a
`(hard-slice …)` handoff from the graph engine to Z3/CVC5 and lifted
models and unsat cores back into the IR (5 phases, ~1 month, Q25–Q30).
User decision: the *idea* is dropped, not merely the schedule — Ein
stays a graph-native reasoner with no solver back-end. The plan folder
is in git history. What stays: [`smt/`](../smt/) as a scratch area with
its CVC4 submodule, and [`docs/lib/02`](../docs/lib/02-solvers-csp-sat-smt.md)
as external-tech catalogue and [M5](m5_presentation/README.md) Track A's
*comparison* axis — which [M1c](../docs/history/m1c_external_validation/README.md)'s
[M10](m10_external_benchmarks/README.md) turns
into a corpus and a harness, scheduled 2026-08-21. Benchmarking Z3 is the
opposite of integrating it.

## Schema

Four-level hierarchy, mirroring `/home/user/work/acva/plans/` (out of tree):

```
Milestone  →  Phase  →  Stage  →  Task
   (M)         (P)       (S)       (T)
```

| level     | id form     | granularity        | artefact                                        |
|-----------|-------------|--------------------|-------------------------------------------------|
| Milestone | `M<n>`      | months             | directory with `README.md` + `open_questions.md` |
| Phase     | `P<m>.<p>`  | weeks              | sub-directory                                    |
| Stage     | `S<m>.<p>.<s>` | ≤ 1 week         | one Markdown file (`s<m>.<p>.<s>_<title>.md`)    |
| Task      | `T<m>.<p>.<s>.<t>` | hours to ~2 days | section inside the stage file                   |

A *task* is the unit of execution: a self-contained feature, an
investigation that ends in a written decision, or a measured
experiment. Tasks are listed under `## Tasks` inside their stage file
and use a stable id.

## Layout

```
plans/
├── README.md                         this file (schema + index)
├── open_questions.md                 cross-milestone questions; sticky Q ids
├── ideas.md                          rolling scratchpad
├── m1_core_graph_reasoning/          (deleted at P1.22 — M1 shipped; see git history)
├── m1a_rust/                         (deleted 2026-08-23 — M1a shipped; the record
│                                     is docs/history/m1a_rust/, the plan tree is
│                                     in git history)
├── m1c_external_validation/          (deleted 2026-08-24 — M1c shipped; the record
│                                     is docs/history/m1c_external_validation/,
│                                     the plan tree is in git history)
├── m1d_satisfiability/               (deleted 2026-08-27 — M1d shipped; the record
│                                     is docs/history/m1d_satisfiability/,
│                                     the plan tree is in git history)
├── m1e_review_processing/            the 2026-08-27 review, processed
│   ├── README.md                     the 63-finding index, and its disposition column
│   ├── open_questions.md             Q-M1e.1–5 (the review's Q1–Q10 are P1e.1's)
│   ├── review/                       the reports, verbatim (the m1d/ideas.md precedent)
│   ├── p1e.1_open_questions/         the ten questions          (6 stage files)
│   ├── p1e.2_high/                   6 findings, 2 topics       (2)
│   ├── p1e.3_medium/                 36 findings, 9 topics      (9)
│   ├── p1e.4_low/                    21 findings, 8 topics      (8)
│   └── p1e.5_documentation_and_other/  what it shipped of a phase that left (1)
├── m1f_hypothesis_and_documentation/ the two phases M1e carried and did not need
│   ├── README.md                     (created 2026-08-29 out of M1e's P1e.1b + P1e.5)
│   ├── open_questions.md             Q-M1f.<n> — empty; the Q-M1e ids stayed in M1e
│   ├── p1f.5_documentation_and_other/  (was p1e.5)   3 stage files + 1 proposed
│   └── p1f.10_hypothesis_structure/    (was p1e.1b)  8
├── m2_nl_to_ir/                      EinAf — iterative autoformalization, Levels B → D
│   ├── README.md                     (the folder keeps its NL → IR name; Level B is that)
│   ├── open_questions.md             Q7–Q11, Q23–Q25 + Q-M2.1–4
│   ├── EinAf.md                      the research plan, verbatim (the m1d/ideas.md precedent)
│   ├── p2.1_kernel_as_instrumentation/   Stage A   (3 stage files)
│   ├── p2.2_formalizer/                  Stage B   (5)
│   ├── p2.3_benchmark/                   Stage C   (4)
│   ├── p2.4_loop/                        Stages E, F   (5)
│   ├── p2.5_harness/                     Stages D, H, N   (4)
│   ├── p2.6_ablations/                   Stage G   (1 — the link-grammar arm)
│   ├── p2.7_failure_scaling_generalization/   Stages I, J, K   (README)
│   ├── p2.8_representations/             Stage L   (README)
│   ├── p2.9_formal_account/              Stage M   (README)
│   └── p2.10_result_artifact_demo/       Stages O, Q   (README; P is M5's)
├── m5_presentation/                  paper + talk after M2 (was m2b_presentation)
│   └── README.md
├── m10_external_benchmarks/          Z3 / CVC5 / Prolog / Soufflé / Clingo / Lean
│   ├── README.md                     (was m1c_external_validation/p1c.2_*)
│   ├── open_questions.md
│   └── s10.1_problem_corpus.md …     5 stages, no phase level
├── m20_gui/                          the GUI — Tauri 2 + React + Monaco + Cytoscape
│   └── README.md                     (was m1b_gui)
├── m3_smt_integration/               (deleted 2026-08-18 — dropped; see git history)
└── followups/                        parking lot — neither MVP-blocking nor scheduled
    ├── README.md
    ├── f1_categorical_formulation.md
    ├── f2_self_modifying_language.md
    ├── f3_three_task_classes_first_class.md
    └── f4_cross_cutting.md
```

Stage files have a stable shape:

```markdown
# S<m>.<p>.<s> — <title>

**Phase:** P<m>.<p> (<title>)
**Estimate:** N days
**Depends on:** ...
**Implements idea:** [<idea>](../../ideas/<file>.md)

## Context
...

## Acceptance
- ...

## Tasks

### Task T<m>.<p>.<s>.<n> — <title>
...
```

## Status

| milestone | depth        | status   | rough estimate |
|-----------|--------------|----------|----------------|
| M1 | *(plans removed at P1.22 — git history)* | **shipped** — done 2026-06-17 (gate green) | ~3 months |
| [M1a](../docs/history/m1a_rust/README.md)               | *(plans removed 2026-08-23 — the record is `docs/history/m1a_rust/`)* | **shipped** — done 2026-08-23, all eleven phases closed. `ein.rs` is the only implementation: `solve zebra2.ein -e` end-to-end **4.53 s → 29.0 ms (157×)** with peak RSS 223 → 17 MB (the PyPy half frozen — nothing can re-measure it), `--jobs N` a further **3.17–4.40×** on 8 cores with every counter identical over 20 712 cells, and the gate **616 tests in 1 m 51 s** with no Python process in any of them | est. ~7 months; ran 2026-08-17 → 2026-08-23 |
| [M1c](../docs/history/m1c_external_validation/README.md) | *(plans removed 2026-08-24 — the record is `docs/history/m1c_external_validation/`)* | **shipped** — done 2026-08-24, one phase and five stages. `:expect` on `query`, `ein test` as the fourth subcommand, 45 programs in `tests/stdlib/`, and **38 of 73 never-firing stdlib rules → 0**, held by `cargo test` in 0.04 s. Its benchmark phase left for M10 on 2026-08-23 | est. ~2.5 weeks; ran 2026-08-23 → 2026-08-24 |
| [M1d](../docs/history/m1d_satisfiability/README.md) | *(plans removed 2026-08-27 — the record is `docs/history/m1d_satisfiability/`)* | **shipped** — done 2026-08-27, four phases and eighteen stages. A program can state a requirement, a state can say what it **owes**, the verdict word `Open` reports it, every verdict states whether its count is certified, and `EIN_TRAVERSAL=tree` reaches 32 models in **86** enterings against the lattice's 17 204 592. P1d.10 closed as it stood, three of six stages shipped | est. ~2 months; ran 2026-08-21 → 2026-08-27 |
| [M1e](m1e_review_processing/README.md)  | **full** — 5 phases, 26 stage files + 2 shipped | **running** — the 2026-08-27 review processed: 63 findings + 10 questions, each ending in **fixed / refuted / accepted / deferred** with a test, a probe, a written reason or an owner. **P1e.1 closed 2026-08-29**: all ten questions answered | ~11 weeks; began 2026-08-27 |
| [M1f](m1f_hypothesis_and_documentation/README.md) | **full** — 2 phases, 11 stage files (+1 proposed) | queued — the two phases M1e carried that processed no finding, moved out 2026-08-29. A branch structure derived from the program rather than paid for in deaths, and the pages a released system would have | ~12 weeks |
| [M2](m2_nl_to_ir/README.md)             | **full for P2.1–P2.5** — 10 phases, 22 stage files; P2.7–P2.10 phase READMEs | next — **reshaped 2026-08-23** around [`EinAf.md`](m2_nl_to_ir/EinAf.md): the kernel as instrumentation, the one-shot formalizer, the benchmark (Level B), the loop, baselines, ablations, failure analysis (Level C), representations, the formal account, the result and the demo (Level D) | ~6 months — Level B at ~8 weeks |
| [M5](m5_presentation/README.md)         | placeholder README only | parked — paper + talk after M2 (was M2b) | TBD |
| [M10](m10_external_benchmarks/README.md) | **full** — 5 stage files, no phase level | queued behind M1a — the same problems through Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean (was M1c's P1c.2) | ~2.5 weeks |
| [M20](m20_gui/README.md)                | README + stack decision | parked — depends on M1a, blocks nothing; Tauri stack settled 2026-08-18 (was M1b) | TBD |
| ~~M3 — SMT integration~~                | *(deleted 2026-08-18 — dropped; see git history)* | **dropped** | — |
| [followups](followups/README.md)        | theme files only | parking lot | unscheduled |

## Glossary

| term              | meaning                                                                 |
|-------------------|-------------------------------------------------------------------------|
| **IR**            | the project's S-expression intermediate representation (designed in P1.1) |
| **graph engine**  | the typed-hypergraph reasoner of M1 (the project's "core")              |
| **trace**         | a markdown + DOT log of the reasoning steps that produced an answer     |
| **ontology layer** | types, instances, value domains, a-priori inter-type relations           |
| **fact layer**    | the relations stated by the problem text                                |
| **reasoning layer** | derived relations, rejected hypotheses, hypothesis branches            |
| **task class**    | A=solve, B=gaps, C=contradictions (per [idea 03](ideas/03-three-task-classes.md)) |

## How to use this directory

- **When starting on a stage**: read the parent phase's `README.md`,
  then the stage file; tasks inside should be executable from the
  given context plus the linked idea note.
- **When parking a question**: add it to the nearest-scope
  `open_questions.md` with a fresh `Q<n>` id (don't reuse).
- **When a stage is done**: append `**Status:** done — <date>` under
  the heading. Don't delete; the trail is the project's memory.
- **When the plan changes**: edit in place. Plans are living
  documents; commit history is the audit trail.
