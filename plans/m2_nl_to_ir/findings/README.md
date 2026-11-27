# M2 — Findings

**What a finding of M2 is.** A thing the milestone finds that is neither a finding of Ein, which is
[the evaluation](../evaluation/README.md)'s, nor a question for the user, which is
[the open questions](../open_questions/README.md)': a thing of EinAf, of the plan, of the process, the gate or the
host, which the work has to know and nothing else holds.

**How it is kept.**

- **An id `L-M2.<n>`**, kept and never reused (M1f's findings were `L-n`, and are the followups' now). **One sequence
  for the milestone and its phases**: a phase takes the next free id, whichever folder it files in. **The next free id
  is L-M2.33.**
- **One file a finding**, `<topic>/L-M2.<n>-<slug>.md`, under a topic folder: its id and title, its status (open, or
  closed and by what), where it was found and its owner, then what was found (the record's words where a record
  states it), the evidence, and what it costs.
- **A phase keeps its findings in its own `findings/`** (every phase's folder has one). At the phase's close what is
  open moves here, under the same topic, and what is closed stays in the phase's folder, which M2 keeps (P2.1's,
  closed before that rule, went with its folder, where `git log` keeps it);
  at the milestone's close what is open goes to [the followups](../../followups/README.md).

## Index

None found at the milestone's level: M2's README and [its analysis](../analysis/README.md) state no finding of this
kind that something else does not hold (the host's, for one, is [`analysis/host.md`](../analysis/host.md)'s). Those
below are P2.1's, moved here open at its close, 2026-11-26, and P2.2's, moved here open at its close, 2026-11-27,
with the seven the analysis of that phase held and no ledger had (L-M2.24 to L-M2.30), filed at the same close.

| id | finding | topic | status | owner since |
|---|---|---|---|---|
| [L-M2.1](toolchain/L-M2.1-the-host-builds-with-rustc-1-100-not-the-pinned-1-97-1.md) | The host builds with rustc 1.100, not the pinned 1.97.1 | `toolchain` | open; the three lints it raised in the engine's code are closed by `acd04b13` | |
| [L-M2.2](toolchain/L-M2.2-einaf-has-no-toolchain-pin-ci-builds-it-with-the-runners.md) | EinAf has no toolchain pin: CI builds it with the runner's stable | `toolchain` | open | |
| [L-M2.4](einaf/L-M2.4-eins-api-change-leaves-einaf-unbuilt-until-its-own-commit.md) | Ein's API change leaves EinAf unbuilt until its own commit | `einaf` | open | [P2.2e](../p2.2e_kernel_fixes/README.md) meets it first |
| [L-M2.7](model/L-M2.7-a-lazy-grammar-is-fed-from-its-trigger-s-first-group.md) | A lazy grammar is fed from its trigger's first non-empty group, so the trigger is the grammar's own first text | `model` | open | [S2.2b.1](../p2.2b_guided_thinking/s2.2b.1_questions_by_hand.md), the thinking's grammar joined to a step's |
| [L-M2.8](model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md) | A kept context and a whole evaluation of one prompt part under greedy decoding | `model` | open | [S2.4.1](../p2.4_loop/s2.4.1_state_machine.md), [S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md), [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md) |
| [L-M2.10](einaf/L-M2.10-a-program-with-no-fixpoint-costs-the-bounded-call-its-clock-once-a-saturation.md) | A program with no fixpoint costs the bounded call its clock once for the load and once a question | `einaf` | open | [S2.4.1](../p2.4_loop/s2.4.1_state_machine.md) |
| [L-M2.16](model/L-M2.16-with-thinking-off-the-model-thinks-in-comment-lines.md) | With thinking off, the model thinks in comment lines, and its loops run to the budget there | `model` | open | [S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md); [P2.2b](../p2.2b_guided_thinking/README.md) |
| [L-M2.18](model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md) | A sentence's polarity and a thing's kind go wrong in step 1 with nothing to see them | `model` | open | [S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md); [P2.2b](../p2.2b_guided_thinking/README.md), the kinds asked of the thinking |
| [L-M2.19](model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md) | A grammar's refusal can send greedy decoding into a loop it does not enter with no grammar | `model` | open | [S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md) |
| [L-M2.21](model/L-M2.21-the-model-s-zebra-answer-query-asks-one-person-both-questions.md) | The model's Zebra answer query asks one person both questions | `model` | open | [Q-M2.35](../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md), [S2.2d.1](../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md): its cause is the contract's one answer query |
| [L-M2.22](model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md) | A fixed text scored right can rest on a contradiction, or a missing row, of the model's own making | `model` | open | [S2.2d.1](../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md), the second count; [S2.5.3](../p2.5_harness/s2.5.3_metrics.md) |
| [L-M2.24](model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md) | The model's Zebra program has seven independent defects, and three repairs leave R empty | `model` | open | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md); [S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md) |
| [L-M2.25](einaf/L-M2.25-the-steps-partial-parts-are-written-twice-never-revised-and-a-step-early.md) | The steps' two partial parts are written twice, never revised, and a step before their uses | `einaf` | open | [S2.2d.2](../p2.2d_formalizer_corrected/s2.2d.2_the_cut.md); [Q-M2.36](../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md) |
| [L-M2.26](einaf/L-M2.26-a-program-the-kernel-refuses-carries-no-categories.md) | A program the kernel refuses carries no categories | `einaf` | open | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) |
| [L-M2.27](einaf/L-M2.27-a-record-s-kernel-commit-is-null.md) | A record's `kernel.commit` is `null` | `einaf` | open | [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md) |
| [L-M2.28](model/L-M2.28-the-prompts-first-example-is-the-zebra-puzzle-s-theory-in-miniature.md) | The prompts' first example is the Zebra puzzle's theory in miniature | `model` | open | [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md); [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) |
| [L-M2.29](model/L-M2.29-the-model-alone-solves-the-zebra-puzzle-in-its-thinking-and-names-it.md) | The model alone solves the Zebra puzzle in its thinking, names it, and prints no answer | `model` | open | [S2.2c.2](../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md); [Q-M2.39](../open_questions/milestone/Q-M2.39-what-is-the-minimal-goal-s-instance-if-the-model-alone-solves-the-zebra-text.md) |
| [L-M2.30](model/L-M2.30-modules-shown-with-no-description-were-imported-by-no-run.md) | Modules shown with no description were imported by no run | `model` | open | [S2.2d.3](../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md); [Q-M2.37](../open_questions/kernel/Q-M2.37-where-do-a-library-s-descriptions-of-its-rules-live.md) |

## In a phase's folder

One line a phase whose `findings/` holds entries, so that every id is found from here.

- **L-M2.1 to L-M2.4 were P2.1's**: the host's toolchain against the pin, EinAf's missing pin, EinAf's load judged
  with no bound, and Ein's API change leaving EinAf unbuilt until its own commit. At the phase's close, 2026-11-26,
  the three open moved [here](#index), and L-M2.3, closed in the phase by S2.1.3 (`0ab1aef0`), went with its folder
  (`git log --diff-filter=D -- plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation`).
- **L-M2.5 to L-M2.23 were filed by P2.2's stages and its report**, in [its findings](../p2.2_formalizer/findings/README.md).
  At the phase's close, 2026-11-27, the eight open moved [here](#index) under their topics (L-M2.7, L-M2.8, L-M2.10,
  L-M2.16, L-M2.18, L-M2.19, L-M2.21 and L-M2.22), each with the stage that owns it since; the eleven closed stay in
  the phase's folder, which M2 keeps (L-M2.5, L-M2.6, L-M2.9, L-M2.11 to L-M2.15, L-M2.17, L-M2.20 and L-M2.23).
  **L-M2.24 to L-M2.30 were held by [the analysis of the phase](../p2.2_formalizer/report/ANALYSIS.md) and no
  ledger**, and are filed here at the same close.
- [P2.2b](../p2.2b_guided_thinking/findings/README.md), [P2.2c](../p2.2c_decoding_and_model_alone/findings/README.md),
  [P2.2d](../p2.2d_formalizer_corrected/findings/README.md): none yet, each planned and not started.
- **L-M2.31 and L-M2.32 are P2.2e's**: L-M2.31 filed by S2.2e.2 on EinAf's recorded model runs and the library's
  digest, L-M2.32 by S2.2e.4 on the guarantee of Q-M2.24 and the register's `W8/fifo-gap-root`
  ([the phase's index](../p2.2e_kernel_fixes/findings/README.md)).
- P2.3 to P2.10: none yet.
