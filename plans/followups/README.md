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
| F1b | [Logical formulation](f1b_logical_formulation.md) — **promoted**  | **promoted 2026-11-11 → [M1g](../m1g_logic/README.md)**, on the user's instruction of that day. Its PFL.1, PFL.3 and PFL.4 are M1g's [P1g.4](../m1g_logic/p1g.4_first_order_logic/README.md) and PFL.2 its [P1g.3](../m1g_logic/p1g.3_proof_systems/README.md). The file is kept: it is the statement of the question (*"which fragment does ein cover?"*, sibling to F1 from the FOL / relation-algebra angle) |
| F2  | [Self-modifying constraint language](f2_self_modifying_language.md) | rung 1 of self-modification: grammar evolves via LLM ↔ harness loop          |
| F3  | [Three task classes as first-class operations](f3_three_task_classes_first_class.md) | once M1.P1.5 ships the mode-selection skeleton; surface to users               |
| F4  | [Cross-cutting](f4_cross_cutting.md)                              | rule-learning, versioned grammars, LLM-as-policy, scope-creep ideas             |
| F5  | [Operate IR rules as data](f5_rules_as_data.md)                   | rung 2 of self-modification: rules rewrite rules, induce rules from facts. **Pulled into [M12](../m12_rules_as_data/README.md) 2026-11-12**, with what F2, F7, F8, F24 and F26 say of it; the file stays |
| F6  | [Modify own harness code](f6_modify_own_harness.md)               | rung 3 of self-modification: engine emits patches to its own source           |
| F7  | [Rule taxonomy + rule induction](f7_rule_induction.md)            | when the rule library grows past hand-management. **Sub-track B left 2026-08-23** — it is M2's theory pass ([S2.2.4](../m2_nl_to_ir/p2.2_formalizer/s2.2.4_passes.md)): activators *selected* from the stdlib catalogue, a rule synthesised only when the catalogue lacks the property. B′ (instance → type properties), C and D stay parked |
| F8  | [FCA / RCA, ODIS, TPTP](f8_FCA_RCA_odis_tptp/ideas.md)            | raw notes — formal concept analysis over the relation algebra, external corpora |
| F9  | Hypothesis-loop E-catalog — **closed, deleted** | nothing to trigger: all 28 entries settled (2026-06-15 + 2026-08-17). It was the ledger of the walk's search-layer optimisations, nine of them measured and rejected; deleted with the finished plans at M1f's S1f.20.9 (`git log --diff-filter=D -- plans/followups/f9_e_catalog.md`) |
| F11 | [Deductive-layer perf](f11_deductive_layer_perf.md)               | when a workload outgrows the matcher — RETE beta-memories + worst-case-optimal joins; the live perf work now that F9's search-layer catalog is closed |
| F14 | Saturation and satisfiability — **promoted**                       | **promoted 2026-08-21 → M1d**, at the user's direction. The note is now [`plans/ideas/11-saturation-and-satisfiability.md`](../ideas/11-saturation-and-satisfiability.md) and its argument is the milestone's § What the note says the engine is missing plus phase P1d.2. Nothing is parked here any more |
| F18 | World-aware negatives — **closed, deleted** | deleted with the finished plans at M1f's S1f.20.9 (git history). It was: when a program needs the shape, or when M1f's S1f.10.8 rules that a refutation may rest on an `absent`. **Closed without being done** if that stage forbids the shape instead — `Prov::absent` is the starting point either way. **Forbidden 2026-11-02** (M1e2's S1e2.4.1); **closed without being done 2026-11-04**, when S1e2.2.5's refusal shipped with F2's *via*, and F21's D6 with it |
| F20 | The three rule kinds are the three sentence forms — **promoted** | **promoted 2026-11-18 → M1h's [P1h.6](../m1h_beyond_word_problems/p1h.6_statements_negations_questions/README.md)**, on the user's instruction (*"M1h - pull F20 there"*), with the user's idea of an open fact read as a question; the note moved with it, [`f20_rule_kinds_as_sentence_moods.md`](../m1h_beyond_word_problems/p1h.6_statements_negations_questions/f20_rule_kinds_as_sentence_moods.md) |
| F21 | Non-monotone programs: node functions and path dependence, decided — **pulled into M1n, moved** | moved 2026-11-19, on the user's instruction (*"pull F21 and related followups and ideas"*), with its probes, into [M1n](../m1n_non_monotone_programs/README.md): [`f21_non_monotone_programs_decided.md`](../m1n_non_monotone_programs/f21_non_monotone_programs_decided.md); its five register rows owned by M1n's S1n.2.1 |
| F22 | [The exact unsat core: re-derive what a full alternatives list refused](f22_exact_core_past_the_cap.md) | M1e2 S1e2.2.11's deferred residual (the user, 2026-11-04, Q-M1e2.11 (i)(b)): `W10/alt-cap-shared`, a saving between two facts behind a full list, which no bounded retention keeps. At M1e2's close, which names the row; when a corpus program's core moves under an uncapped build; or when an explanation consumer needs the minimal core. **Widened 2026-11-05**: Q-M1e2.15 (b)'s residual, and the inventory's W-8, W-9, W-10 and W23's exact core past the budget (Q-M1e2.16) |
| F23 | [Diagnostics in Ein's own words](f23_diagnostics_in_eins_words.md) | M1e2's inventory of 2026-11-05, on the user's ruling of that day (Q-M1e2.16 (4)): eleven diagnostics and spellings M1e2 found and did not fix (Python spellings, `at None`, an empty stderr from `ein test`, labels, pages that no longer say what holds), and two labels with no owner. None moves an answer. At a release, at M5's presentation work, when a stage touches the loader's or the compiler's error paths, or when a user meets one |
| F24 | [Written provenance: what a fact's `:rule` and `:using` may claim](f24_written_provenance.md) | M1f S1f.7.9's finding, moved here on the user's ruling of 2026-11-11: a written `:rule` may name no rule, and any written provenance is a derivation with no premises, so the fact leaves every core (`unsat core ∅`). R is not moved. When a consumer reads cores from programs it did not write, when a dump is meant to load on its own, when a stage touches a fact's clauses in `ein.y`, or at F22's start |
| F25 | [Roughness found outside the zebra programs](f25_roughness_found_outside_zebra.md) | the user's instruction of 2026-11-11: a finding a stage makes outside its scope that moves no answer of an existing zebra program is fixed in place if quick, or is a row here, and is not shown for review. When a corpus of real Ein programs exists (from M1f P1f.6's calculator onward) and one meets a row, or when the user asks for the list |
| F26 | [Consistency and completeness: Ein's two words, logic's, and common sense's](f26_consistency_and_completeness.md) | the user's instruction of 2026-11-11: Ein's alive and dead, owes and discharged, exhausted and closed, set beside logic's consistency and its several completenesses (Gödel's two theorems, the chase, closed worlds, undecidability once nested facts grow) and beside the everyday senses a word problem's gaps have. A sketch of questions, checked by nothing. When F1b is taken up, at M1f S1f.6.10, when M2 or M5 must say what a verdict certifies, or when F5 makes rules data. **Taken up by [M1g](../m1g_logic/README.md), 2026-11-11**: its glossary, decidability table and chase reading are S1g.4.5, the word for *did not stop* S1g.6.1, the Gödel section S1g.5.4; the common-sense table stays here, for M2 |
| F27 | [A cryptarithm theory, `std.math.crypt`](f27_cryptarithm_theory.md) | M1f S1f.20.8, on the user's ruling at S1f.20.2's review (item 11): P1f.6's question 42, decided *yes* and not built (Q-M1f.337, `pending`), kept here so that it outlives P1f.6's folder. The column rules with the digit tables read in every direction, written once, SEND + MORE = MONEY its example. When a stage adds a module under `stdlib/math/`, a corpus program writes its own column rules, a benchmark of puzzle families beside the zebra starts, or a search-order stage wants its cheapest witness |
| F28 | [The language, the engine and the standard library: what M1f left](f28_language_engine_stdlib_left_by_m1f.md) | M1f's close, 2026-11-25, on the user's word (*"hand over all remain open findings to followups and then close M1f"*): nineteen findings of the milestone's findings page, under its ids (L-19 to L-137), four **the user's call** (a relation variable free in a `:match`, L-56; a label inert to the blind enumerator, L-57; a macro's hygiene, L-62; a rule concluding `(relation a)`, L-19). When the user takes up a call, a stage changes the loader, the compiler, the macro expander or the `open` facts, or a stage on `std.debts` or the calculator's shared rules |
| F29 | [The renderers: what M1f left](f29_renderers_left_by_m1f.md) | M1f's close, 2026-11-25, the same word: nine findings about what the views draw (L-35 to L-138), four **the user's call** (L-35 with L-63, L-76, L-111, L-112); none misreports an answer. At a stage on the renderers, M5's presentation or M20's GUI |
| F30 | [The documentation pages and the decisions: what M1f left](f30_pages_and_decisions_left_by_m1f.md) | M1f's close, 2026-11-25, the same word: seventeen findings about the pages of `docs/ein/` and the decisions (L-21 to L-121), eleven **the user's call**. When a stage writes on a page a row names, at a second read of the pages, or when the user takes up a call |
| F31 | [Tests, dead code, comments and examples: what M1f left](f31_tests_dead_code_comments_and_examples.md) | M1f's close, 2026-11-25, the same word: twenty-six findings (L-22 to L-149), ten **the user's call**; most of the rest *with the next batch of code*. At the next batch of code commits in a crate a row names, or a stage that adds a test sweep |
| F32 | [The process: the release voice where the check does not read, the gate, the census instruments](f32_release_voice_gate_and_censuses.md) | M1f's close, 2026-11-25, the same word: eleven findings about the process (L-2 to L-152), three **the user's call** (the voice check reading strings, hashes and case ids; the census instruments' side). At a change to the voice check, the gate's steps or the CI image, or before the censuses are taken again |
| F33 | [The defaults M1f's phases took that no question carries](f33_defaults_m1f_took.md) | M1f's close, 2026-11-25, the same word: the three lists of defaults the milestone's findings page kept (the documentation tree, nested facts, the review's rulings built), each standing until the user revises it. When the user revises one, or a stage changes what a default describes |

**F12, F13, F15, F16 and F17 are raw `ideas.md` notes**, added after this
index was written and not yet worked into one-page themes: rules and relations,
puzzles beyond zebra, math formulae, autoformalization, formal verification.
They are directories holding the user's notes and nothing else. F15's export of rules as formulas is
taken up by [M1g](../m1g_logic/README.md) (S1g.4.2 writes the target by hand, S1g.6.3 is the exporter); the
notes stay as they are. F13 is the one
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
  one of these until 2026-08-17, then a single file until it was deleted.
  F21's fourteen probe programs, which its page and three censuses under
  `utils/censuses/` run, moved with it into M1n:
  [`../m1n_non_monotone_programs/f21_probes/`](../m1n_non_monotone_programs/f21_probes/).
- **Closing an entry keeps the reason, not the stub.** A settled entry's
  spec is history; what a future reader needs is one line saying *why* —
  especially for a measured rejection, which is otherwise indistinguishable
  from an unexplored gap. F9's ledger is the worked example.
- If a followup starts to acquire concrete tasks, promote it: move
  to a milestone folder under `plans/m<n>_*/p<n>.<m>_*/` and write
  proper stage files. **A note that turns out to be the *reason* for a
  milestone rather than the content of one phase lands at the milestone
  root instead**, verbatim: F14 became
  [`m1d_satisfiability/ideas.md`, now `plans/ideas/11`](../ideas/11-saturation-and-satisfiability.md) on
  2026-08-21, with the milestone README doing the analysis and the phases
  carrying the work.

## Connections

The followups span the *parking lot* set of
[`docs/ideas/`](../ideas) topics — specifically the
categorical formulation (07), self-modifying language (01),
three task classes (03), and the cross-cutting questions that
recurred across multiple ideas.
