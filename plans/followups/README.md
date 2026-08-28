# Followups

Themes that are neither MVP-blocking nor on the M1-M2 schedule.
Park here so the ideas don't get lost between
[`docs/ideas/`](../ideas) (raw notes) and
[`plans/m*/`](../) (scheduled work).

Each file is one *theme* — a coherent direction of follow-up work
the user might pick up after the scheduled milestones, or in parallel if
motivation strikes.

## Index

| F   | Title                                              | Trigger                                                                        |
|-----|----------------------------------------------------|--------------------------------------------------------------------------------|
| F1  | [Categorical formulation](f1_categorical_formulation.md)         | when M1 stabilises and the engine's rule set is fixed — formalise post-hoc      |
| F1b | [Logical formulation](f1b_logical_formulation.md)                | sibling to F1 from the FOL / relation-algebra angle — "which fragment does ein cover?" |
| F2  | [Self-modifying constraint language](f2_self_modifying_language.md) | rung 1 of self-modification: grammar evolves via LLM ↔ harness loop          |
| F3  | [Three task classes as first-class operations](f3_three_task_classes_first_class.md) | once M1.P1.5 ships the mode-selection skeleton; surface to users               |
| F4  | [Cross-cutting](f4_cross_cutting.md)                              | rule-learning, versioned grammars, LLM-as-policy, scope-creep ideas             |
| F5  | [Operate IR rules as data](f5_rules_as_data.md)                   | rung 2 of self-modification: rules rewrite rules, induce rules from facts     |
| F6  | [Modify own harness code](f6_modify_own_harness.md)               | rung 3 of self-modification: engine emits patches to its own Python source    |
| F7  | [Rule taxonomy + rule induction](f7_rule_induction.md)            | when the rule library grows past hand-management. **Sub-track B left 2026-08-23** — it is M2's theory pass ([S2.2.4](../m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md)): activators *selected* from the stdlib catalogue, a rule synthesised only when the catalogue lacks the property. B′ (instance → type properties), C and D stay parked |
| F8  | [FCA / RCA, ODIS, TPTP](f8_FCA_RCA_odis_tptp/ideas.md)            | raw notes — formal concept analysis over the relation algebra, external corpora |
| F9  | [Hypothesis-loop E-catalog](f9_e_catalog.md) — **closed**         | nothing to trigger: all 28 entries settled (2026-06-15 + 2026-08-17). Kept as the ledger — read it before proposing a search-layer optimisation, it is where nine of them were measured and rejected |
| F10 | [M1 refactor-debt tail](f10_m1_refactor_tail/README.md) — **closed** | nothing to trigger: all 23 stages settled (2026-08-17 re-measured the 17 the P1.22 relocation left looking open — every one had already landed, no code change needed). Kept for `findings.md`, P1.7b's 40-finding review register, and for the two stages whose *verdicts* matter: `.20`'s retired length bar and `.25`'s rejected shared-emitter headline |
| F11 | [Deductive-layer perf](f11_deductive_layer_perf.md)               | when a workload outgrows the matcher (or the Rust port reaches it) — RETE beta-memories + worst-case-optimal joins; the live perf work now that F9's search-layer catalog is closed |
| F14 | Saturation and satisfiability — **promoted**                       | **promoted 2026-08-21 → [M1d](../../docs/history/m1d_satisfiability/README.md)**, at the user's direction. The note is now [`docs/history/m1d_satisfiability/ideas.md`](../../docs/history/m1d_satisfiability/ideas.md) and its argument is the milestone's [§ What the note says the engine is missing](../../docs/history/m1d_satisfiability/README.md) plus phase [P1d.2](../../docs/history/m1d_satisfiability/README.md#p1d2--obligations). Nothing is parked here any more |
| F18 | [World-aware negatives](f18_world_aware_negatives.md)             | when a program needs the shape, or when M1e's S1e.1b.8 rules that a refutation may rest on an `absent`. **Closed without being done** if that stage forbids the shape instead — `Prov::absent` is the starting point either way |

**F12, F13, F15, F16 and F17 are raw `ideas.md` notes**, added after this
index was written and not yet worked into one-page themes: rules and relations,
puzzles beyond zebra, math formulae, autoformalization, formal verification.
They are directories holding the user's notes and nothing else. F13 is the one
[M10](../m10_external_benchmarks/README.md)
cross-links — it is the *other* benchmark direction, NL-shaped and M2-gated —
and F14 is the one that has left.

The three self-modification followups (F2 / F5 / F6) share a unifying
view: [`docs/ideas/10-generic-self-modification.md`](../ideas/10-generic-self-modification.md).

## Working agreement

- A followup is *not* a stage. No `Tasks` section unless it gets
  promoted into a milestone phase.
- Each file is a *one-page* placeholder: what the theme is, why
  we're not doing it now, what would trigger promotion, what
  prior art / connections matter.
- **Exception — directory followups.** A followup may be a *directory*
  with a `README.md` index when it carries already-written stage specs
  that would be destroyed by flattening (F8, F10). The README still
  obeys the one-page rule; the stage files are the parked detail. This
  is for content *relocated* out of a closed milestone, not a licence to
  draft stages here. When the last stub goes, the directory goes: F9 was
  one of these until 2026-08-17 and is now a single file. **Unless a
  non-stub artifact justifies it** — F10 outlived all 17 of its stubs the
  same day but kept its directory for `findings.md`, a 40-finding review
  register that is too long to inline and is not a stage spec.
- **Closing an entry keeps the reason, not the stub.** A settled entry's
  spec is history; what a future reader needs is one line saying *why* —
  especially for a measured rejection, which is otherwise indistinguishable
  from an unexplored gap. F9's ledger is the worked example.
- If a followup starts to acquire concrete tasks, promote it: move
  to a milestone folder under `plans/m<n>_*/p<n>.<m>_*/` and write
  proper stage files. **A note that turns out to be the *reason* for a
  milestone rather than the content of one phase lands at the milestone
  root instead**, verbatim: F14 became
  [`m1d_satisfiability/ideas.md`](../../docs/history/m1d_satisfiability/ideas.md) on
  2026-08-21, with the milestone README doing the analysis and the phases
  carrying the work.

## Connections

The followups span the *parking lot* set of
[`docs/ideas/`](../ideas) topics — specifically the
categorical formulation (07), self-modifying language (01),
three task classes (03), and the cross-cutting questions that
recurred across multiple ideas.
