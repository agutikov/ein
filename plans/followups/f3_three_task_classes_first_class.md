# F3 — Three task classes as first-class user-facing operations

**Theme owner:** the user.
**Trigger:** M1.P1.5 ships the mode-selection skeleton (already in
M1's acceptance). F3 is about *surfacing* the three as polished user
commands. *(The trigger used to include "M3 connects all three to SMT";
M3 was dropped 2026-08-18, which removes a second implementation of the
three classes but nothing F3 depended on.)*

## What this is

Per [`docs/ideas/03-three-task-classes.md`](../ideas/03-three-task-classes.md):
three distinct things you can ask of a constraint graph —
**solve**, **gaps**, **contradictions**. M1 implements all three as
engine modes. F3 promotes them from "modes you pass via flag" to
**first-class verbs** in the CLI and the trace output — and adds the
implicit *fourth* class the user named (explain).

## Current state after M1

- `ein solve --mode=solve|gaps|contradictions FILE`
  — works, but verbose.
- Engine returns `Solution | Ambiguity | Contradiction` — already
  three-shaped.
- Trace renderer can show any of the three; no specialised UX per
  mode.

## What F3 adds

- **First-class CLI verbs**:
  `ein solve …` / `ein gaps …` / `ein why-not …`
  (this last one is the "what's wrong with my puzzle" verb from
  [idea 03 §Open questions point 4](../ideas/03-three-task-classes.md#open-questions)).
- **Explain** (the implicit fourth class —
  [idea 03 §The implicit fourth class](../ideas/03-three-task-classes.md#the-implicit-fourth-class)) —
  `ein explain --fact "..."` shows the derivation DAG ending
  at a target fact.
- **Specialised trace UX per mode**:
  - `gaps` highlights divergent values per branch;
  - `contradictions` foregrounds the smallest recorded contradiction
    frontier (`frontier.smallest_contradiction_frontier`; not a
    subset-minimal MUS);
  - `explain` shows only the relevant derivation slice.

## Open questions promoted

- Q34: Should `gaps` accept *partial* problems where the
  ontology is given but only some facts are stated? (Today's M1
  accepts this; F3 would polish the UX.)
- Q35: Should the trace for `gaps` mode include *counterfactual*
  reasoning ("if condition (3) were stronger, the answer would
  be unique")? Promotes M1.P1.5 GAPS into a richer notion.

## Why deferred

M1 ships all three as engine modes. F3 is *polish* — better verbs,
better trace UX, the fourth class. None of that is needed to run the
engine; it's all to make the engine *delightful* to use.

## Prior art / connections

- [Idea 03](../ideas/03-three-task-classes.md) — the full
  argument and the table.
- [Idea 02](../ideas/02-graph-as-formal-substrate.md) — the
  underlying *graph as primary* claim is what makes all three
  classes natural on the same substrate.
- [`docs/lib/02-solvers-csp-sat-smt.md`](../../docs/lib/02-solvers-csp-sat-smt.md) §8 — solver-side support.
