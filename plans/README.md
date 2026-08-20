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

Two adjacent secondary milestones surface Ein externally, plus a Rust
port slotted before the GUI and the two milestones that came out of it:

- **M1a — Rust port (ein.rs)** ([m1a_rust/](m1a_rust/README.md))
  between M1 and M1b — the engine that ships from M2 onward. Two
  invariants: a **1:1 observable surface** (same language, same CLI,
  same bytes, with `ein.py` kept as the parity oracle) and a **free
  hand inside** (integer-encoded atoms and facts, a register matcher,
  copy-on-write forks, multi-core search). It ships a **library and a
  CLI**; server mode was dropped 2026-08-18.
- **M1b — GUI** ([m1b_gui/](m1b_gui/README.md)) between M1a
  and M2 — **Tauri 2 + React + Monaco + Cytoscape.js**, linking the
  ein.rs crates directly (stack decided 2026-08-18).
- **M1c — External validation** ([m1c_external_validation/](m1c_external_validation/README.md))
  after M1a — the two checks that are *not* relative to Ein's own past:
  the stdlib's rules get expectations of their own (`:expect` on
  `query`, `ein test`), and a small cross-language corpus runs the same
  problems through Z3, CVC5, SWI-Prolog, Soufflé, Clingo and Lean —
  answers first, times second. Created 2026-08-21 out of M1a's ex-P1a.11
  plus a new benchmark phase.
- **M1d — From saturation to satisfiability** ([m1d_satisfiability/](m1d_satisfiability/README.md))
  after M1a — why an under-determined puzzle does not finish, and what
  saturation lacks to be a decision procedure: existence requirements
  (`total` / `surjective` with the force their names claim) as
  first-class obligations rather than as refutations at the extreme.
  Created 2026-08-21 out of M1a's ex-P1a.12 plus the F14 note.
- **M2b — paper + presentation** ([m2b_presentation/](m2b_presentation/README.md))
  after M2.

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
as external-tech catalogue and [M2b](m2b_presentation/README.md) Track A's
*comparison* axis — which [M1c](m1c_external_validation/README.md)'s
[P1c.2](m1c_external_validation/p1c.2_external_benchmarks/README.md) turns
into a corpus and a harness, scheduled 2026-08-21. Benchmarking Z3 is the
opposite of integrating it.

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
├── m1_core_graph_reasoning/          (deleted at P1.22 — M1 shipped; see git history)
├── m1a_rust/                         the Rust port — design docs + 11 phases
│   ├── README.md
│   ├── open_questions.md
│   ├── divergences.md
│   ├── design/ …                     11 numbered design docs
│   └── p1a.0_conformance_harness/ …
├── m1b_gui/                          the GUI — Tauri 2 + React + Monaco + Cytoscape
│   └── README.md
├── m1c_external_validation/          checks that are not relative to Ein
│   ├── README.md
│   ├── open_questions.md
│   ├── p1c.1_stdlib_conformance/     (was m1a_rust/p1a.11_*)
│   └── p1c.2_external_benchmarks/    Z3 / CVC5 / Prolog / Soufflé / Clingo / Lean
├── m1d_satisfiability/               what saturation lacks to decide
│   ├── README.md
│   ├── open_questions.md
│   ├── ideas.md                      the user's note (was followups/f14_*)
│   ├── p1d.1_exhaustive_search/      (was m1a_rust/p1a.12_*)
│   ├── p1d.2_obligations/
│   └── p1d.3_model_sets/
├── m2_nl_to_ir/                      NL → IR — link-grammar / GBNF / llama.cpp
│   ├── README.md
│   ├── open_questions.md
│   └── p2.1_investigations/ …
├── m2b_presentation/                 paper + talk after M2
│   └── README.md
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
| [M1a](m1a_rust/README.md)               | **full** — 11 design docs + 11 phases + 54 stage files | **in progress** — Rust port (ein.rs): 1:1 surface parity + internal rewrite; P1a.0–P1a.6 shipped, P1a.7 paused, P1a.10 under way | ~7 months |
| [M1b](m1b_gui/README.md)                | README + stack decision | parked — slots between M1a and M2; Tauri stack settled 2026-08-18 | TBD |
| [M1c](m1c_external_validation/README.md) | **full** — 2 phases + 10 stage files | queued behind M1a — stdlib expectations (`ein test`) + benchmarks against the field | ~1 month |
| [M1d](m1d_satisfiability/README.md)     | mixed — P1d.1 at stage depth, P1d.2 / P1d.3 phase READMEs | queued behind M1a — exhaustive search over many models + existence obligations | ~2 months |
| [M2](m2_nl_to_ir/README.md)             | medium (stage skeletons) | next | ~2 months after M1 |
| [M2b](m2b_presentation/README.md)       | placeholder README only | parked — paper + talk after M2 | TBD |
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
