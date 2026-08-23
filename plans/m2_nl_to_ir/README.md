# M2 — EinAf: iterative autoformalization, through Level D

**Estimate:** ~6 months — 10 phases, 22 stage files, ~24 weeks of phase
estimates; **Level B at ~8 weeks**, Level C at ~19, Level D at the end plus
[M5](../m5_presentation/README.md). See § How deep this plan is.
**Status:** **reshaped 2026-08-23** around the research plan
[`EinAf.md`](EinAf.md), at the user's direction; nothing started. The folder
keeps its name — *NL → IR* is Level B of this milestone, and the links into
`m2_nl_to_ir/` are many.
**Depends on:** [M1a](../../docs/history/m1a_rust/README.md) — the kernel,
shipped. [M1d](../m1d_satisfiability/README.md) for one word:
[Q-M1d.6](../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)
is the engine saying *Contradiction* where the research plan's Stage A
forbids it to (§ A1 — *`unknown` must never silently become `false`,
`ambiguous`, or `contradiction`*); [P2.1](p2.1_kernel_as_instrumentation/README.md)
says what M2 does until that lands.
**Blocks:** [M5](../m5_presentation/README.md) — the paper is Stage P of the
research plan and M5 is where it is written; Stage O's result and Stage Q's
artifact are [P2.10](p2.10_result_artifact_demo/README.md)'s and feed it.
**Id:** M2 throughout. The six phases P2.1–P2.6 of the 2026-05 plan are
replaced, not renumbered — § The old plan, and where each piece went.

---

## The research question

The root [README](../../README.md) keeps two things apart: **Ein**, the
symbolic kernel, shipped and measured; and **EinAf**, the framework around it,
where a neural component proposes a formalization and the kernel judges it,
in a loop. This milestone is EinAf. The research plan
([`EinAf.md`](EinAf.md), the user's, kept verbatim at the milestone root the
way [`m1d/ideas.md`](../m1d_satisfiability/ideas.md) is) states the question
the loop exists to answer:

> **Can structured feedback from a symbolic reasoner improve an LLM's ability
> to construct correct formal theories from natural-language problem
> statements?** — and the stronger form: **which forms of symbolic feedback
> are most useful for detecting and repairing incorrect autoformalizations?**

The loop is `x → T₀ → E(T₀) → T₁ → E(T₁) → ⋯ → Tₙ`, with `x` the text, `Tᵢ`
an Ein program, `E` the kernel returning a verdict and a diagnostic, and the
formalizer `A_θ(x, Tᵢ, E(Tᵢ)) → Tᵢ₊₁` ([`EinAf.md` § Stage M](EinAf.md#stage-m--formalize-the-conceptual-model)).
The quantity measured is `P(faithful(Tₙ) ∧ correct(Tₙ))` **as a function of
what `E` is allowed to say** — and the central experimental variable is
therefore not the model, not the prompt, but the **information the kernel
returns**, graded F0 (*try again*) through F8 (the combined structured
diagnostic). Nothing in the plan assumes F8 is best; § Stage F says so, and
[P2.6](p2.6_ablations/README.md) is where it is not assumed.

Two premises the plan inherits from the user's notes and this milestone holds
rather than re-argues:

- **Autoformalization is not translation**
  ([F16](../followups/f16_autoformalization/ideas.md)). A puzzle's text states
  its facts and leaves its *theory* implicit — that the houses are linearly
  ordered, that each attribute is a bijection onto them. So the formalizer's
  output is `ontology + theory + instance + query`, and the theory part is
  **selected** from the stdlib before it is ever **synthesised**
  ([F12](../followups/f12_rules_and_relations/ideas.md): *select a theory, do
  not invent properties*). [F7 B](../followups/f7_rule_induction.md#connection-to-m2)
  flagged this as being on M2's critical path and the old plan did not carry
  it; [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) does.
- **Formal validity is not faithfulness** ([F17](../followups/f17_formal_verification/ideas.md),
  `Spec ≠ Intent`). The kernel can certify that a theory has exactly one model
  and cannot certify that the theory says what the English said. The plan's
  most dangerous failure mode — *Ein: ambiguous; LLM: invents a constraint;
  Ein: unique* — is a loop **succeeding by its own measure while failing the
  task**, which is why faithfulness is a first-class metric
  ([S2.4.4](p2.4_loop/s2.4.4_faithfulness.md)), why the benchmark contains
  instances whose *correct* answer is "ambiguous"
  ([S2.3.2](p2.3_benchmark/s2.3.2_generators.md)), and why
  `P(correct | Ein certifies unique)` and `P(faithful | Ein certifies unique)`
  are reported as two numbers ([S2.5.3](p2.5_harness/s2.5.3_metrics.md)).

## Levels, and the phases that reach them

The research plan defines four levels. Level A is the kernel and is
**shipped** — with one named gap. The other three are this milestone, in
order, each with a gate that the phases before it have to pass.

| level | the plan's definition | reached by | gate |
|---|---|---|---|
| **A — symbolic engine** | representation, saturation, constraints, model search, provenance, diagnostics, testing | M1 + [M1a](../../docs/history/m1a_rust/README.md), shipped; [P2.1](p2.1_kernel_as_instrumentation/README.md) closes the interface gap Stage A names | the kernel treated as *instrumentation*: a versioned structured protocol, a diagnostic vocabulary, and the six outcomes distinguished — with `unknown` distinct from the other five |
| **B — autoformalization system** | previously unseen NL tasks automatically become executable Ein theories and answers | [P2.2](p2.2_formalizer/README.md) + [P2.3](p2.3_benchmark/README.md) | one-shot `einaf from-text` on the benchmark's *test* split — unseen by prompt development, three or more reasoning families, every run recorded |
| **C — research system** | iterative feedback, several domains, credible baselines, quantitative benchmarks, ablations, failure analysis; *supports an empirical claim* | [P2.4](p2.4_loop/README.md) – [P2.7](p2.7_failure_scaling_generalization/README.md) | the main table (B0–B5 under matched budget), the feedback ablations, a classified failure sample, and one claim of the three admissible shapes in § Stage O |
| **D — complete research artifact** | C plus formal treatment, a heterogeneous frozen benchmark, generalization and scaling, faithfulness analysis, reproducible infrastructure, released raw results, a paper, a minimal demo | [P2.8](p2.8_representations/README.md) – [P2.10](p2.10_result_artifact_demo/README.md) + [M5](../m5_presentation/README.md) | the seven layers of § Stage Q, each independently usable; the paper is M5's |

The plan's own words on the last row, kept because they set the bar
correctly: *Level D is not feature completeness. Ein can still have an
enormous roadmap.* It is reached when there is a well-defined hypothesis, a
system that can test it, controlled evidence that distinguishes competing
explanations, a formal account, and enough released material for someone
else to reproduce or challenge the result.

## The research plan's stages, and where each lives

`EinAf.md` is organised as seventeen stages A–Q. They are not all this
milestone's, and the ones that are do not all get a phase of their own; this
table is the map from the plan to the folder.

| stage | what it asks for | where it lives |
|---|---|---|
| **A** kernel as experimental foundation | the semantic boundary frozen (A1), invariants as tests (A2), a versioned machine interface (A3), a diagnostic vocabulary apart from presentation (A4), validation suites (A5) | A1/A2/A5 are the kernel's and exist — [`docs/kernel/`](../../docs/kernel/README.md), the gate, [`defined_behaviour.md`](../../docs/kernel/defined_behaviour.md); what is missing is named in [S2.1.1](p2.1_kernel_as_instrumentation/s2.1.1_census.md). A3/A4 are **[P2.1](p2.1_kernel_as_instrumentation/README.md)**. A1's sixth outcome, `unknown`, is [M1d Q-M1d.6](../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s |
| **B** autoformalization as an explicit task | the contract (B1), source vs generated (B2), what the model may know (B3), prompt versioning (B4) | **[P2.2](p2.2_formalizer/README.md)** |
| **C** a heterogeneous benchmark | eight families (C1), external + synthetic (C2), unique / ambiguous / unsat (C3), difficulty knobs (C4), frozen splits (C5) | **[P2.3](p2.3_benchmark/README.md)** |
| **D** baselines | B0 direct answer … B5 full loop, under a matched inference budget | [S2.5.2](p2.5_harness/s2.5.2_baselines.md) |
| **E** the loop | the state machine (E1), repair vs regenerate (E2), faithfulness (E3), termination (E4) | **[P2.4](p2.4_loop/README.md)** |
| **F** feedback levels F0–F8 | the experimental variable | [S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md) |
| **G** ablations G1–G9 | retry control, verdict, unsat core, provenance, model difference, feedback size, depth, representation, library | **[P2.6](p2.6_ablations/README.md)** |
| **H** metrics | four layers, and the two conditional probabilities | [S2.5.3](p2.5_harness/s2.5.3_metrics.md) |
| **I** failure taxonomy | a classified sample; neural / representation / symbolic / interface / benchmark | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **J** scaling | formalizer vs solver vs combined degradation | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **K** generalization | paraphrase, structure, domain, novel vocabulary, adversarial noise | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **L** compare representations | Ein / SMT / Datalog / code / proof assistant **as LLM targets** | **[P2.8](p2.8_representations/README.md)** — seeded by [M10](../m10_external_benchmarks/README.md)'s hand-written encodings, which answer a different question |
| **M** formal model | `K = (O, R, F, Γ, P)`, `lfp(T_Γ)`, `E(T) → (v, d)`, `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))` | **[P2.9](p2.9_formal_account/README.md)** — with [F1](../followups/f1_categorical_formulation.md) / [F1b](../followups/f1b_logical_formulation.md); M5 § 4–5 consume it |
| **N** reproducibility | one canonical workflow; every experiment an immutable record | [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) — **before the first table, not after the last** |
| **O** the central result | one claim, of three admissible shapes, one of them negative | **[P2.10](p2.10_result_artifact_demo/README.md)** |
| **P** the paper | twelve sections | [M5](../m5_presentation/README.md) |
| **Q** the public artifact | seven layers; the one-command demo | **[P2.10](p2.10_result_artifact_demo/README.md)** + M5 |

## Phases

In work order, which is id order.

| ID | title | stages | est. | what it ends with |
|---|---|---|---|---|
| [P2.1](p2.1_kernel_as_instrumentation/README.md) | The kernel as instrumentation — Stage A | 3 | 1.5 wk | `ein-feedback/1`: the versioned feedback object and its diagnostic vocabulary; the boundary (Q25) decided; the census of what Stage A asks for against what ships |
| [P2.2](p2.2_formalizer/README.md) | The formalizer — Stage B | 5 | 3 wk | `einaf from-text`, one-shot: NL → ontology + theory + instance + query under the contract, with every run's record; Zebra from its text to its canonical answer |
| [P2.3](p2.3_benchmark/README.md) | The benchmark — Stage C | 4 | 3 wk | eight families, generators with exact ground truth and a canonical theory, unique / ambiguous / unsat instances, external adapters, frozen splits. **Level B gate** |
| [P2.4](p2.4_loop/README.md) | The loop — Stages E, F | 5 | 3 wk | the state machine with every transition logged; repair as well as regenerate; the feedback ladder F0–F8 as nine renderers of one object; faithfulness judged against the source; termination and cycle detection |
| [P2.5](p2.5_harness/README.md) | Baselines, metrics, records — Stages D, H, N | 4 | 2.5 wk | the immutable experiment record; B0–B5 under a matched budget; the four metric layers; **the first main table** |
| [P2.6](p2.6_ablations/README.md) | Ablations — Stage G | 4 | 3 wk | G1–G9, each a table with its pre-registered reading; the link-grammar A/B runs here as one arm |
| [P2.7](p2.7_failure_scaling_generalization/README.md) | Failure analysis, scaling, generalization — Stages I, J, K | 3 | 3 wk | a classified failure sample; degradation curves for the formalizer, the solver and the pair; K1–K5. **Level C gate** |
| [P2.8](p2.8_representations/README.md) | Representations compared — Stage L | — | 2 wk | the five narrow questions of Stage L answered for Ein against SMT, Datalog, code and a proof assistant as LLM targets |
| [P2.9](p2.9_formal_account/README.md) | The formal account — Stage M | — | 1.5 wk | the conceptual model written independently of the Rust |
| [P2.10](p2.10_result_artifact_demo/README.md) | The result, the artifact, the demo — Stages O, Q | — | 1.5 wk | one claim with its evidence; the seven layers released; one command that shows the loop. **Level D gate**, with M5 |

## How deep this plan is

**P2.1 – P2.5 are at stage depth** — 21 stage files, written 2026-08-23, the
ones that carry the old plan's content forward re-targeted to the crates and
the ones that are new written from the research plan. **P2.6 has one stage
file**, [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), because
it carries the old P2.5 link-grammar experiment whole; its other three stages
are paragraphs in the phase README. **P2.7 – P2.10 are phase READMEs only.**
That is on purpose and follows [M1d](../m1d_satisfiability/README.md#how-deep-this-plan-is)'s
precedent: a stage file for G7 written before the first main table exists
would be a guess about which depth matters, and the plan says what the
guess-free version is — *plot marginal improvement against cost* — which is
one line, not a file. The phases get their stage files when the phase before
them has numbers.

The estimates are stage-file sums for P2.1 – P2.5 and the research plan's
scope for the rest. They are **not** the old plan's two months: that plan
ended where Level B begins.

## The old plan, and where each piece went

The 2026-05 plan was *NL → IR*: six phases, two months, written when ein was
Python. It was right about more than it was wrong about, and the reshape
keeps what it got right in the stage that now needs it. The census:

| old | what it was | where it went |
|---|---|---|
| P2.1 investigations + decisions (S2.1.1 survey; S2.1.2–6 the six verdicts under `docs/decisions/M2-*.md`) | a fortnight of deciding before building | **Each decision moves to the stage that needs it**: Q25 → [S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md); Q8 → [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md); Q9 → [S2.2.4](p2.2_formalizer/s2.2.4_passes.md), where "library of ontologies + override" becomes theory *selection* from the stdlib catalogue; Q23 / Q24 → [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) / [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md); Q10 → [P2.8](p2.8_representations/README.md), which is that question asked properly; Q7 stays a question and leaves the research question (§ Open questions). **`docs/decisions/` is dropped** — it never existed, and a verdict lands beside its question in [`open_questions.md`](open_questions.md), as M1a's did. The reading list is [S2.2.1](p2.2_formalizer/s2.2.1_contract.md)'s |
| P2.2 LLM infra (S2.2.1 compose service + model pin; S2.2.2 Python client) | a `llama-server` container and a thin client | [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md), the same stage in the language Q25 picks; the reproducibility envelope becomes the first field of the experiment record ([S2.5.1](p2.5_harness/s2.5.1_experiment_record.md)) |
| P2.3 GBNF for IR (S2.3.1 generator; S2.3.2 task grammars) | the IR lifted into GBNF so the LLM can only emit valid IR | [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) — generated from [`00_ebnf.md`](../../docs/kernel/ir/03-ein-lang/00_ebnf.md) rather than from a Python parser; one grammar per pass, the facts grammar parameterised on the ontology's instance lists, kept whole |
| P2.4 pipeline (S2.4.1 two-stage extraction; S2.4.2 validator + re-prompt; S2.4.3 ambiguity as hypotheses; S2.4.4 `ein from-text`) | the pipeline, with one repair loop inside it | **Split along the plan's seam between Stage B and Stage E.** The passes → [S2.2.4](p2.2_formalizer/s2.2.4_passes.md), plus the *theory* pass the old plan lacked; the CLI → [S2.2.5](p2.2_formalizer/s2.2.5_from_text.md), one-shot only and renamed `einaf from-text`; the validator → [S2.4.2](p2.4_loop/s2.4.2_repair.md), where it is feedback level **F1** of nine and "≤ 3 attempts" becomes Stage E's termination policy; ambiguity as hypotheses → [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) |
| P2.5 link-grammar experiment (S2.5.1 runner; S2.5.2 A/B with a pre-registered rule) | the user's question Q11, measured | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md) — one arm of the representation ablation, same rule, same submodule note |
| P2.6 evaluation harness (S2.6.1 five puzzles + gold IR + a gold trace; S2.6.2 harness + CI) | five puzzles, three metrics | the five puzzles are the **seed set** of [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md); the harness is [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md); IR-F1 becomes constraint precision / recall against a canonical theory ([S2.5.3](p2.5_harness/s2.5.3_metrics.md)), which synthetic instances make exact; the gold trace and trace coverage stay as the Zebra smoke test |

What the old plan assumed and this one does not: that the frontend is
Python calling the engine in-process (gone with `ein.py/`;
[Q25](open_questions.md#q25--what-language-is-the-frontend-written-in)); that
"valid IR" was the hard part (it is the easy part — GBNF makes it free, and
the plan's § Stage B says *syntax is the relatively easy part*); that one
repair loop with three attempts was the loop (it is F1 of a ladder, and the
ladder is the experiment); that five puzzles were a benchmark (they are a seed
set, and a benchmark without ambiguous and unsatisfiable instances cannot
catch the failure mode E3 names). What it got right and this plan keeps: the
local model under GBNF, the `:source` quote on every extracted fact — which
is B2's *source-derived* category a year early — the ontology-parameterised
grammar, the pre-registered decision rule, and reproducibility as a
first-class deliverable.

## Acceptance

Per level; each gate is checked by a phase named above.

**Level B** ([P2.3](p2.3_benchmark/README.md) closes it):

1. `einaf from-text examples/zebra.txt` produces a program the kernel solves
   to the canonical Zebra answer, one-shot, with the record — the old plan's
   first criterion (`ein from-text` there; the command is the harness's, the
   kernel stays LLM-free) kept as the smoke test.
2. One-shot formalization on the benchmark's **test** split, three or more
   families, reported per family with parse / load / verdict rates and answer
   accuracy — the split unseen by prompt development
   ([S2.3.4](p2.3_benchmark/s2.3.4_splits.md)).
3. The three verdict classes are *distinguished*: on synthetic instances with
   gold verdict Ambiguity or Contradiction the one-shot system reports that
   verdict, not a guess — the old "a deleted word yields gaps, not a parse
   failure", generalised and scored.
4. Every run is an immutable record: model, model SHA, prompt hashes, grammar
   hashes, Ein commit, stdlib manifest, seeds, raw generations, programs,
   kernel outputs ([S2.5.1](p2.5_harness/s2.5.1_experiment_record.md)).

**Level C** ([P2.7](p2.7_failure_scaling_generalization/README.md) closes it):

5. The main table: B0–B5 on the validation split, same model, matched
   inference budget, with the four metric layers and both conditional
   probabilities.
6. The feedback ablations G1–G9, each with the reading it was pre-registered
   to have.
7. A failure sample of stated size, classified by the taxonomy, split into
   neural / representation / symbolic / interface / benchmark-ambiguity.
8. One claim in the form of § Stage O — positive, narrow, or negative — with
   the table that supports it and the one that would have refuted it.

**Level D** ([P2.10](p2.10_result_artifact_demo/README.md) closes it, with
M5):

9. The formal account ([P2.9](p2.9_formal_account/README.md)); the benchmark
   frozen and versioned; scaling and generalization curves; the faithfulness
   analysis; the records released; the paper ([M5](../m5_presentation/README.md));
   one command that runs the demo in § Stage Q.

## Open questions

[`open_questions.md`](open_questions.md). The old set — Q7–Q11, Q23–Q25 —
keeps its ids and each entry now names the stage that decides it; **Q10 is
re-homed to Stage L** and **Q7 leaves the research question** (the NL
explanation side is M5's demo, not the loop's). Four new ones arrived with the
reshape, as `Q-M2.<n>`: when the kernel is frozen for an experiment
([Q-M2.1](open_questions.md#q-m21--when-is-the-kernel-frozen)), whether the
model must be local ([Q-M2.2](open_questions.md#q-m22--must-the-model-be-local)),
what the unit of faithfulness is where there is no gold theory
([Q-M2.3](open_questions.md#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory)),
and whether the fixed point is syntactic or semantic
([Q-M2.4](open_questions.md#q-m24--is-the-fixed-point-syntactic-or-semantic)).

## Connections

- [`EinAf.md`](EinAf.md) — the research plan, verbatim; this README is its
  map and the phases are its schedule. Edit it only as the user's note.
- [`README.md` § EinAf](../../README.md) — the thirteen components with their
  built / scheduled / open columns; the two *scheduled* cells that pointed at
  the old S2.4.2 / S2.4.3 now point at P2.4.
- [F13](../followups/f13_puzzles_beyond_zebra/ideas.md) — the benchmark
  ladder, the loop `(Pᵢ, Tᵢ) → Ein → Dᵢ` and its *semantic* fixed point, the
  three neural actions, the four oracle ablations — the user's note the
  plan's Stages C, E and G grew from. [F16](../followups/f16_autoformalization/ideas.md)
  (autoformalization ≠ translation), [F17](../followups/f17_formal_verification/ideas.md)
  (`Spec ≠ Intent`), [F12](../followups/f12_rules_and_relations/ideas.md)
  (select, do not invent), [F7 B](../followups/f7_rule_induction.md#connection-to-m2)
  (activators on the critical path) — the premises named above.
- [Idea 04](../ideas/04-nlp-to-graph-to-solver-pipeline.md) — the
  architecture sketch the old plan realised and this one keeps as Level B;
  [idea 01](../ideas/01-self-modifying-constraint-language.md) /
  [F2](../followups/f2_self_modifying_language.md) — GBNF; the grammar *in*
  the loop is rung 1 and stays a followup, this milestone ships static
  grammars.
- [M1d](../m1d_satisfiability/README.md) — the verdict vocabulary
  ([Q-M1d.6](../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false),
  [Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted));
  [M1c](../m1c_external_validation/README.md) — `:expect`, the form a
  benchmark instance's gold verdict is written in;
  [M10](../m10_external_benchmarks/README.md) — the hand-written encodings
  P2.8 starts from, and the benchmark direction that is *not* this one
  (formal-language shaped, not M2-gated).
- [`docs/api/rust.md`](../../docs/api/rust.md) — what a Rust loop links;
  [`docs/install.md`](../../docs/install.md), `--json-summary`
  (`ein-summary/1`) and [`--events`](../../docs/kernel/inference/events.md)
  (`ein-events/1`) — what a Python one drives; Q25 chooses.
- [M20](../m20_gui/README.md) — displays whatever the loop records; not a
  dependency either way.
