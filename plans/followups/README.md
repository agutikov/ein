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
| F6  | [Modify own harness code](f6_modify_own_harness.md)               | rung 3 of self-modification: engine emits patches to its own source           |
| F7  | [Rule taxonomy + rule induction](f7_rule_induction.md)            | when the rule library grows past hand-management. **Sub-track B left 2026-08-23** — it is M2's theory pass ([S2.2.4](../m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md)): activators *selected* from the stdlib catalogue, a rule synthesised only when the catalogue lacks the property. B′ (instance → type properties), C and D stay parked |
| F8  | [FCA / RCA, ODIS, TPTP](f8_FCA_RCA_odis_tptp/ideas.md)            | raw notes — formal concept analysis over the relation algebra, external corpora |
| F9  | [Hypothesis-loop E-catalog](f9_e_catalog.md) — **closed**         | nothing to trigger: all 28 entries settled (2026-06-15 + 2026-08-17). Kept as the ledger — read it before proposing a search-layer optimisation, it is where nine of them were measured and rejected |
| F11 | [Deductive-layer perf](f11_deductive_layer_perf.md)               | when a workload outgrows the matcher — RETE beta-memories + worst-case-optimal joins; the live perf work now that F9's search-layer catalog is closed |
| F14 | Saturation and satisfiability — **promoted**                       | **promoted 2026-08-21 → [M1d](../../docs/history/m1d_satisfiability/README.md)**, at the user's direction. The note is now [`docs/history/m1d_satisfiability/ideas.md`](../../docs/history/m1d_satisfiability/ideas.md) and its argument is the milestone's [§ What the note says the engine is missing](../../docs/history/m1d_satisfiability/README.md) plus phase [P1d.2](../../docs/history/m1d_satisfiability/README.md#p1d2--obligations). Nothing is parked here any more |
| F18 | [World-aware negatives](f18_world_aware_negatives.md)             | when a program needs the shape, or when M1f's S1f.10.8 rules that a refutation may rest on an `absent`. **Closed without being done** if that stage forbids the shape instead — `Prov::absent` is the starting point either way. **Forbidden 2026-11-02** (M1e2's S1e2.4.1); **closed without being done 2026-11-04**, when S1e2.2.5's refusal shipped with F2's *via*, and F21's D6 with it |
| F20 | [The three rule kinds are the three sentence forms](f20_rule_kinds_as_sentence_moods.md) | when M2's formalizer decides which rule kind a clause becomes, or idea 08 renders rules back into text: assertion → positive fact, negation → restriction, question → obligation (the user, 2026-10-16) |
| F21 | [Non-monotone programs: node functions and path dependence, decided](f21_non_monotone_programs_decided.md) | M1e2 P1e2.1b's deferred decisions (the user, 2026-11-02: detect, document, warn, decide nothing): what a node's state is (Q-M1e2.2, (A)–(E)), path dependence and incremental search, the hole report (Q-M1e2.7 (d)), the handling, learning before *forbid*'s refusal ships, the search past the oracle's limit (was S1e2.1c.3). When a route would extend a parent's fixpoint on a flagged program, when R is wanted past the limit on one, when S1e2.2.5 builds the refusal, or when the revisit cannot settle W8. **Since 2026-11-05** also the inventory's W-2, W-5 and W-6 (rows filed by S1e2.2.25) and five detector labels, S-6–S-9 and S-11 (Q-M1e2.16) |
| F22 | [The exact unsat core: re-derive what a full alternatives list refused](f22_exact_core_past_the_cap.md) | M1e2 S1e2.2.11's deferred residual (the user, 2026-11-04, Q-M1e2.11 (i)(b)): `W10/alt-cap-shared`, a saving between two facts behind a full list, which no bounded retention keeps. At M1e2's close, which names the row; when a corpus program's core moves under an uncapped build; or when an explanation consumer needs the minimal core. **Widened 2026-11-05**: Q-M1e2.15 (b)'s residual, and the inventory's W-8, W-9, W-10 and W23's exact core past the budget (Q-M1e2.16) |
| F23 | [Diagnostics in Ein's own words](f23_diagnostics_in_eins_words.md) | M1e2's inventory of 2026-11-05, on the user's ruling of that day (Q-M1e2.16 (4)): eleven diagnostics and spellings M1e2 found and did not fix (Python spellings, `at None`, an empty stderr from `ein test`, labels, pages that no longer say what holds), and two labels with no owner. None moves an answer. At a release, at M5's presentation work, when a stage touches the loader's or the compiler's error paths, or when a user meets one |
| F24 | [Written provenance: what a fact's `:rule` and `:using` may claim](f24_written_provenance.md) | M1f S1f.7.9's finding, moved here on the user's ruling of 2026-11-11: a written `:rule` may name no rule, and any written provenance is a derivation with no premises, so the fact leaves every core (`unsat core ∅`). R is not moved. When a consumer reads cores from programs it did not write, when a dump is meant to load on its own, when a stage touches a fact's clauses in `ein.y`, or at F22's start |
| F25 | [Roughness found outside the zebra programs](f25_roughness_found_outside_zebra.md) | the user's instruction of 2026-11-11: a finding a stage makes outside its scope that moves no answer of an existing zebra program is fixed in place if quick, or is a row here, and is not shown for review. When a corpus of real Ein programs exists (from M1f P1f.6's calculator onward) and one meets a row, or when the user asks for the list |
| F26 | [Consistency and completeness: Ein's two words, logic's, and common sense's](f26_consistency_and_completeness.md) | the user's instruction of 2026-11-11: Ein's alive and dead, owes and discharged, exhausted and closed, set beside logic's consistency and its several completenesses (Gödel's two theorems, the chase, closed worlds, undecidability once nested facts grow) and beside the everyday senses a word problem's gaps have. A sketch of questions, checked by nothing. When F1b is taken up, at M1f S1f.6.10, when M2 or M5 must say what a verdict certifies, or when F5 makes rules data |

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
  that would be destroyed by flattening (F8). The README still
  obeys the one-page rule; the stage files are the parked detail. This
  is for content *relocated* out of a closed milestone, not a licence to
  draft stages here. When the last stub goes, the directory goes: F9 was
  one of these until 2026-08-17 and is now a single file.
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
