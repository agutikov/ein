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
  [M1 core graph reasoning](m1_core_graph_reasoning/README.md)
  delivers; the Zebra puzzle is the acceptance gate.
- **M2 — convert NL problem statements into IR facts.** The
  second step: drop the *human encodes the puzzle* assumption.
  M2 ships the NL → IR pipeline so the engine can be fed
  problem text directly. [M2 NL → IR](m2_nl_to_ir/README.md).
- **M2+ — ontology + rules induction from facts** (covered by
  [F4](followups/f4_cross_cutting.md) /
  [F7](followups/f7_rule_induction.md)). Beyond M2: induce the
  ontology + rule activators *that the puzzle implicitly
  assumes*, so the engine (a) actually *can* solve the puzzle
  rather than sitting on a half-typed KB and (b) reflects the
  common-sense implicits an NL statement leans on.

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

Two adjacent secondary milestones surface Ein externally,
plus a Rust port slotted before the GUI:

- **M1a — Rust port (ein.rs)** ([m1a_rust/](m1a_rust/README.md))
  between M1 and M1b — the engine that ships from M2 onward.
- **M1b — GUI** ([m1b_gui/](m1b_gui/README.md)) between M1a
  and M2.
- **M2b — paper + presentation** ([m2b_presentation/](m2b_presentation/README.md))
  after M2, before or after M3.

M3 (SMT integration) is the parallel hard-slice escape hatch;
the followups in [`followups/`](followups/README.md) park the
research-level threads (self-modification F2/F5/F6, formal
foundations F1/F1b, rule induction F7, cross-cutting F4, three
task classes F3).

## Schema

Four-level hierarchy, mirroring [`/home/user/work/acva/plans/`](../../acva/plans/):

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
├── m1_core_graph_reasoning/          MVP — the 2021 prototype, done properly
│   ├── README.md                     milestone overview (goal, phases, acceptance)
│   ├── open_questions.md             milestone-scoped questions
│   ├── p1.1_ir_language/
│   │   ├── README.md
│   │   ├── s1.1.1_grammar_design.md
│   │   ├── s1.1.2_parser_serialiser.md
│   │   └── s1.1.3_round_trip_tests.md
│   ├── p1.2_typed_hypergraph/        …
│   ├── p1.3_inference_rules/         …
│   ├── p1.4_constraints/             …
│   ├── p1.5_hypothesis_loop/         …
│   ├── p1.6_rendering_and_trace/     …
│   ├── p1.7_bootstrapping_zebra/     …
│   ├── p1.7a_solution_search_refactor/  sound solution model & search/result/stop refactor (the M1 gate)
│   └── p1.7b_review_and_refactor/    post-M1 debt paydown — code-cited review + refactor (non-gating)
├── m2_nl_to_ir/                      NL → IR — link-grammar / GBNF / llama.cpp
│   ├── README.md
│   ├── open_questions.md
│   └── p2.1_investigations/ …
├── m3_smt_integration/               graph engine → SMT slice; explanation recovery
│   ├── README.md
│   ├── open_questions.md
│   └── p3.1_ir_to_smt_lib/ …
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
| [M1](m1_core_graph_reasoning/README.md) | full (stages-as-files) | **shipped** — done 2026-06-17 (gate green) | ~3 months |
| [M1a](m1a_rust/README.md)               | placeholder README only | parked — Rust port (ein.rs); slots between M1 and M1b | TBD |
| [M1b](m1b_gui/README.md)                | placeholder README only | parked — slots between M1a and M2 | TBD |
| [M2](m2_nl_to_ir/README.md)             | medium (stage skeletons) | next | ~2 months after M1 |
| [M2b](m2b_presentation/README.md)       | placeholder README only | parked — paper + talk after M2 (or after M3) | TBD |
| [M3](m3_smt_integration/README.md)      | sketch (one stage per phase) | planned | ~1 month after M2 |
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
