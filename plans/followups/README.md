# Followups

Themes that are neither MVP-blocking nor on the M1-M2-M3 schedule.
Park here so the ideas don't get lost between
[`docs/ideas/`](../../docs/ideas/) (raw notes) and
[`plans/m*/`](../) (scheduled work).

Each file is one *theme* — a coherent direction of follow-up work
the user might pick up after M3, or in parallel if motivation
strikes.

## Index

| F   | Title                                              | Trigger                                                                        |
|-----|----------------------------------------------------|--------------------------------------------------------------------------------|
| F1  | [Categorical formulation](f1_categorical_formulation.md)         | when M1 stabilises and the engine's rule set is fixed — formalise post-hoc      |
| F2  | [Self-modifying constraint language](f2_self_modifying_language.md) | rung 1 of self-modification: grammar evolves via LLM ↔ harness loop          |
| F3  | [Three task classes as first-class operations](f3_three_task_classes_first_class.md) | once M1.P1.5 ships the mode-selection skeleton; surface to users               |
| F4  | [Cross-cutting](f4_cross_cutting.md)                              | rule-learning, versioned grammars, LLM-as-policy, scope-creep ideas             |
| F5  | [Operate IR rules as data](f5_rules_as_data.md)                   | rung 2 of self-modification: rules rewrite rules, induce rules from facts     |
| F6  | [Modify own harness code](f6_modify_own_harness.md)               | rung 3 of self-modification: engine emits patches to its own Python source    |

The three self-modification followups (F2 / F5 / F6) share a unifying
view: [`docs/ideas/10-generic-self-modification.md`](../../docs/ideas/10-generic-self-modification.md).

## Working agreement

- A followup is *not* a stage. No `Tasks` section unless it gets
  promoted into a milestone phase.
- Each file is a *one-page* placeholder: what the theme is, why
  we're not doing it now, what would trigger promotion, what
  prior art / connections matter.
- If a followup starts to acquire concrete tasks, promote it: move
  to a milestone folder under `plans/m<n>_*/p<n>.<m>_*/` and write
  proper stage files.

## Connections

The four followups span the *parking lot* set of
[`docs/ideas/`](../../docs/ideas/) topics — specifically the
categorical formulation (07), self-modifying language (01),
three task classes (03), and the cross-cutting questions that
recurred across multiple ideas.
