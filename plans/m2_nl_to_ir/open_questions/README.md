# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`plans/open_questions.md`](../../open_questions.md).

Two id forms share this ledger. **Q7–Q11** are sticky ids from the cross-milestone pool, assigned when the
milestone was *NL → IR*; they keep their numbers. **Q-M2.n** is the milestone-scoped form, which
[M10](../../m10_external_benchmarks/open_questions.md) uses too: Q-M2.1–4 arrived with the 2026-08-23 reshape around
[`EinAf.md`](../EinAf.md), Q-M2.8–11 with the analysis M2 started from, 2026-11-25
([`analysis/`](../analysis/README.md)), Q-M2.12–13 with the user's answers of that day, Q-M2.14–15 with the
answers that followed them, Q-M2.16 with the re-plan, Q-M2.17 with the user's answers to the re-plan, Q-M2.18–21 with
P2.1's report, filed in the phase's folder and moved here at its close ([below](#questions-of-a-phase)), Q-M2.22 with P2.2's re-plan to the
user's steps, Q-M2.23 and Q-M2.24 with S2.2.0a, Q-M2.25 and Q-M2.26 with S2.2.1, Q-M2.27 with S2.2.2, Q-M2.28 and Q-M2.29 with S2.2.3, and Q-M2.30 with S2.2.3's amendment, filed in that phase's folder.

**Q-M2.5, Q-M2.6 and Q-M2.7 were this ledger's Q23, Q24 and Q25 until 2026-11-25.** Those three ids were already the
root ledger's, given to questions of M1's, and two of them are decision files under `docs/decisions/` (a premise
says what a variable ranges over; Ein has no ordinal form). An id is never reused, so M2's three took the next free
ids of the milestone, and every file of this folder that cited them cites the new ones.

**How it is kept.** One file a question, `<topic>/<id>-<slug>.md`, under the topic it is about: its id and title,
its status (open, or ruled by the user and when), and its text. The index below lists the open questions first and
the user's rulings after them. A ruling is the user's words
as given, then what the coordinator reads them to mean, **which the user may correct**; the options a ruled
question weighed are not kept here (`git show 5d678042:plans/m2_nl_to_ir/open_questions.md` has those of the
analysis, `git show 6a3c9a75:plans/m2_nl_to_ir/open_questions.md` those of Q-M2.12 and Q-M2.13).

**Where an answer lands.** An answer about Ein is a file under `docs/decisions/`, under the id of its question,
written with the change that makes it true; an answer about EinAf or about how the work is done stays here, beside
its question. EinAf is a project of its own, outside Ein ([Q-M2.8](einaf/Q-M2.8-where-does-einaf-live.md)), so an answer
about the harness is the second kind.

## Index

### Open

| Q | title | decided |
|---|---|---|
| [Q7](answer/Q7-llm-as-surface-generator.md) | Is the surface generator (NL output) allowed to be an LLM? | **outside the research question**, parked with its working answer; the answer's own words are ruled, [Q-M2.12](answer/Q-M2.12-what-prints-the-answer.md) |
| [Q8](formalizer/Q8-ambiguous-parses.md) | Where do ambiguous NL parses go — branched on the IR, or rejected? | [S2.4.5](../p2.4_loop/s2.4.5_alternatives_as_hypotheses.md), by experiment; the form is settled, a hypothesis `(open F)` (the user, 2026-11-25) |
| [Q9](formalizer/Q9-ontology-provenance.md) | Per-puzzle declared ontology vs ontology inferred from text | [S2.2.4](../p2.2_formalizer/s2.2.4_passes.md) — reframed as theory *selection*; its default, for the user's review: B with C in the main arms (the text's vocabulary, the catalogue's theories), A the escape hatch |
| [Q10](formalizer/Q10-direct-llm-constraint.md) | When is direct LLM → constraint emission acceptable (skip the IR)? | [P2.8](../p2.8_representations/README.md) — Stage L is this question asked properly |
| [Q11](formalizer/Q11-link-grammar-value.md) | Does link-grammar enrich LLM input usefully, or is it dead weight? | [S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md), by experiment |
| [Q-M2.3](answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md) | What is the unit of faithfulness without a gold theory? | [S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md) |
| [Q-M2.4](answer/Q-M2.4-is-the-fixed-point-syntactic-or-semantic.md) | Is the fixed point syntactic or semantic? | [S2.4.1](../p2.4_loop/s2.4.1_state_machine.md) |
| [Q-M2.6](formalizer/Q-M2.6-one-gbnf-or-many.md) | One GBNF grammar per task, or one grammar for everything? | [S2.2.3](../p2.2_formalizer/s2.2.3_gbnf.md); its default, for the user's review: *many*, a grammar a step, the whole grammar kept for a program in one call |
| [Q-M2.10](answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) | What does *Ein certifies a unique model* mean on R? | **the user, after experiments** (*"it depends on problem shape"*) |
| [Q-M2.15](milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md) | Where does M2 stop for the next iteration of Ein? | **the user**, at a level's gate; until then every gate is a stop |
| [Q-M2.18](einaf/Q-M2.18-are-einafs-fmt-and-clippy-steps-of-the-gate.md) | Are EinAf's fmt and clippy steps of the gate? | **the user**; P2.1's default stands meanwhile ([its defaults](../defaults/p2.1.md)) |
| [Q-M2.19](kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md) | Do `ein-summary/2` and `ein-test-report/6` keep their numbers? | **the user**; P2.1's default stands meanwhile |
| [Q-M2.20](kernel/Q-M2.20-what-are-max-steps-name-unit-and-default.md) | What are `--max-steps`' name, unit and default? | **the user**; P2.1's default stands meanwhile |
| [Q-M2.21](kernel/Q-M2.21-a-missing-files-line-the-pages-python-spelling-or-eins-own.md) | A missing file's line: the page's Python spelling, or Ein's own words? | **the user**; P2.1's default stands meanwhile |

### Ruled by the user

| Q | title | the user, 2026-11-25 |
|---|---|---|
| [Q-M2.1](kernel/Q-M2.1-when-is-the-kernel-frozen.md) | When is the kernel frozen? | *"use current state - after M1f closed"* |
| [Q-M2.2](model/Q-M2.2-must-the-model-be-local.md) | Must the model be local? | *"yes"* |
| [Q-M2.5](model/Q-M2.5-local-model-choice.md) | Which local model? | *"Qweb3.5:4b"*, read as `qwen3.5:4b` |
| [Q-M2.7](einaf/Q-M2.7-what-language-is-the-frontend-written-in.md) | What language is the frontend written in? | *"Rust"* |
| [Q-M2.8](einaf/Q-M2.8-where-does-einaf-live.md) | Where does EinAf live? | *"./einaf/, ./docs/einaf/, etc..."*, then *"own tree, in this repository, outside of ein … but inside the development process"*, then *"One tree - yes, preferred, docs/einaf/ - separately"* |
| [Q-M2.9](kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md) | Does the kernel grow a machine surface for diagnostics? | *"(b)"*: it does, in its own terms |
| [Q-M2.11](answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md) | What is a flagged program to the loop? | *"(a)"*: an outcome of its own, never certified |
| [Q-M2.12](answer/Q-M2.12-what-prints-the-answer.md) | What prints the answer? | *"don't run M1p now, use as much of existing ein NL features … use additional external formatters"*; the answer is *"a sentence, like zebra's"*; EinAf's fixed texts where no program prints one, *"OK"* |
| [Q-M2.13](einaf/Q-M2.13-who-calls-the-kernel.md) | Who calls the kernel: the harness, the model, or both? | both: the tool, *"recommened but optional"*, and the filter, *"forced to write ein code"* |
| [Q-M2.14](model/Q-M2.14-what-runs-the-model.md) | What runs the model? | *"llama-cpp with rust wrapper, not ollama"* |
| [Q-M2.16](kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md) | What of the kernel may change inside M2? | *"we can change kernel without changing ein language syntax and semantics … bug fixes allowed"* |
| [Q-M2.17](milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md) | What is the milestone's minimal goal? | the model, *"with help of Ein or through Ein"*, solves the Zebra puzzle: *"this is a minimal goal"* |

The stage files are re-planned to these rulings (M2's README, [§ The plan as re-planned](../README.md#the-plan-as-re-planned)):
no stage still decides a question that is ruled here.

## Topics

| topic | what it holds | questions |
|---|---|---|
| [`kernel/`](kernel/) | what of Ein the milestone reads and may change: when the kernel is frozen, its machine surface, what a stage may change in it | [Q-M2.1](kernel/Q-M2.1-when-is-the-kernel-frozen.md), [Q-M2.9](kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md), [Q-M2.16](kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md), [Q-M2.19](kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md), [Q-M2.20](kernel/Q-M2.20-what-are-max-steps-name-unit-and-default.md), [Q-M2.21](kernel/Q-M2.21-a-missing-files-line-the-pages-python-spelling-or-eins-own.md) |
| [`einaf/`](einaf/) | EinAf itself: the language it is written in, where it lives, who calls the kernel | [Q-M2.7](einaf/Q-M2.7-what-language-is-the-frontend-written-in.md), [Q-M2.8](einaf/Q-M2.8-where-does-einaf-live.md), [Q-M2.13](einaf/Q-M2.13-who-calls-the-kernel.md), [Q-M2.18](einaf/Q-M2.18-are-einafs-fmt-and-clippy-steps-of-the-gate.md) |
| [`model/`](model/) | the model: local, which one, what runs it | [Q-M2.2](model/Q-M2.2-must-the-model-be-local.md), [Q-M2.5](model/Q-M2.5-local-model-choice.md), [Q-M2.14](model/Q-M2.14-what-runs-the-model.md) |
| [`formalizer/`](formalizer/) | the formalizer and its inputs: ambiguous readings, ontologies, direct emission, link grammar, the grammar at decoding | [Q8](formalizer/Q8-ambiguous-parses.md), [Q9](formalizer/Q9-ontology-provenance.md), [Q10](formalizer/Q10-direct-llm-constraint.md), [Q11](formalizer/Q11-link-grammar-value.md), [Q-M2.6](formalizer/Q-M2.6-one-gbnf-or-many.md) |
| [`answer/`](answer/) | the answer and what certifies it: the words it is printed in, faithfulness, the fixed point, *unique*, a flagged program | [Q7](answer/Q7-llm-as-surface-generator.md), [Q-M2.3](answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), [Q-M2.4](answer/Q-M2.4-is-the-fixed-point-syntactic-or-semantic.md), [Q-M2.10](answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md), [Q-M2.11](answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md), [Q-M2.12](answer/Q-M2.12-what-prints-the-answer.md) |
| [`milestone/`](milestone/) | the milestone: where it stops for Ein, its minimal goal | [Q-M2.15](milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md), [Q-M2.17](milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md) |

## Questions of a phase

**Each phase keeps an `open_questions/` folder of its own**, beside its README, kept as this one is: one file a
question under a topic folder, and an index. A question a phase raises takes **the next free id of this ledger's
sequence**, `Q-M2.<n>`, whichever folder it is filed in. **At the phase's close** what is open moves here, under the
same topic, and what is ruled goes with the phase's folder, where `git log` keeps it.

| phase | its questions |
|---|---|
| P2.1 | none: Q-M2.18 to Q-M2.21, raised by its report, moved here open at its close, 2026-11-26, its folder deleted (`git log --diff-filter=D -- plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation`) |
| [P2.2](../p2.2_formalizer/README.md) | Q-M2.22, raised by the re-plan to the user's steps, Q-M2.23 and Q-M2.24, raised by S2.2.0a, Q-M2.25 and Q-M2.26, raised by S2.2.1, Q-M2.27, raised by S2.2.2, Q-M2.28 and Q-M2.29, raised by S2.2.3, Q-M2.30, raised by S2.2.3's amendment, Q-M2.31 and Q-M2.32, raised by S2.2.4, and Q-M2.33 and Q-M2.34, raised by S2.2.5: [its open questions](../p2.2_formalizer/open_questions/README.md) |
| [P2.2b](../p2.2b_guided_thinking/README.md) | none yet: planned, not started; its design questions, each with its default, are in [its README](../p2.2b_guided_thinking/README.md#design-questions-each-with-its-default) |
| [P2.3](../p2.3_benchmark/README.md) to [P2.10](../p2.10_result_artifact_demo/README.md) | none yet |

**The next free id is Q-M2.35.**
