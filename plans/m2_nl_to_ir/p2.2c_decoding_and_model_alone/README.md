# P2.2c — A generation that ends, and the model alone

**Status:** **planned 2026-11-27** on the user's word, *"Ok I agree, do it"*, to the routing of P2.2's results at
its close (a new phase before P2.3 for what has no owner: a generation that stops when it loops, and the model
alone measured). **Not started**: no stage runs, not even a survey, until the user says so.
**Depends on:** [P2.2](../p2.2_formalizer/README.md), closed: the runtime `einaf::model`, its envelope and its
replay ([S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md), [05-model](../../../docs/einaf/05-model.md)); the
recorded generations that ran to their budget (S2.2.2's probe, S2.2.4's steps, S2.2.5's one shot); the reading of
an output the budget cut ([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md), `Cut::KeepForms`); the answer lines and
the stand-in reader of an answer.
**Blocks:** [P2.2b](../p2.2b_guided_thinking/README.md), whose thinking runs meet the same loop and whose
*budgeted* way assumes a generation EinAf can end; [P2.2d](../p2.2d_formalizer_corrected/README.md), which
narrows grammars further and runs the plan's 29 again.
**Informs:** [Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md),
[Q-M2.31](../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md),
[Q-M2.39](../open_questions/milestone/Q-M2.39-what-is-the-minimal-goal-s-instance-if-the-model-alone-solves-the-zebra-text.md)
and [Q-M2.40](../open_questions/einaf/Q-M2.40-is-the-model-reasons-and-ein-checks-its-answer-an-arm.md);
[S2.5.2](../p2.5_harness/s2.5.2_baselines.md), whose B1 this takes early on a few texts;
[S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md), the tool.
**Carries:** proposals 9, 12, 18 and 22 of [the analysis of P2.2](../p2.2_formalizer/report/ANALYSIS.md#9-proposals-for-the-other-issues).

## Goal

Two things nothing else in the plan owns before the loop, and both are needed before it.

**A generation that loops is stopped, with a stop of its own.** Under greedy decoding the model falls into a
repetition and writes it to its budget: every one of the probe's 36 runs with thinking on, 7 of the steps' 49
calls, 8 of the one shot's 88, which were 82 % of that stage's generation time
([L-M2.19](../findings/model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md),
[L-M2.16](../findings/model/L-M2.16-with-thinking-off-the-model-thinks-in-comment-lines.md)). Each costs its call
about four minutes and gives no form, or a cut one. Nothing in EinAf ends such a generation today.

**The model alone is measured where the minimal goal needs it.** The goal's premise is that the model does not
solve the Zebra puzzle by reasoning in text. So far that is *decoded greedily, it prints no answer*: it reached
the right grid in its thinking and then looped, and it named the puzzle
([L-M2.29](../findings/model/L-M2.29-the-model-alone-solves-the-zebra-puzzle-in-its-thinking-and-names-it.md)).
Whether it solves the text under its own sampler, and a puzzle it cannot have seen, is better known before the loop
is built towards the goal than after.

## What P2.2 found that this phase starts from

- **A loop is the grammar's refusal met by greedy decoding, or thinking left alone.** Under the steps' grammars 7
  of 49 calls ran to the budget and with no grammar none of 48 did: each is a call whose grammar refused a token,
  or whose prompt carried such a loop. Thinking on, every run looped, cycles of 11 to 911 tokens begun between
  tokens 249 and 8 901.
- **Where it loops**: in comment lines between forms, which every step's grammar allows; inside a form (a premise
  repeated, a numeral); a whole fact repeated (718 times, the Zebra puzzle in one call).
- **What the readers do with a cut output**: `einaf from-text` keeps the forms written before the cut and drops
  the one the cut stopped in ([L-M2.17](../p2.2_formalizer/findings/model/L-M2.17-an-output-the-budget-cuts-under-a-grammar-does-not-parse.md),
  closed); `einaf steps run` ends the run, so that S2.2.4's records replay as recorded.
- **The model's card asks for sampling with thinking on**, and warns against greedy decoding for this reason; the
  phase's runs were greedy so that one run is the whole distribution (P2.2's default 7).
- **The runtime has the turns and the function**: a tool call's round trip ran through the binding in the host
  probe, on a kept context ([L-M2.8](../findings/model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md)).

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2c.1](s2.2c.1_a_generation_that_ends.md) | A generation that ends | a detector in `einaf::model` that ends a generation whose tail repeats one cycle, a stop reason of its own, a field of a request so that every record replays; held with no model on every recorded generation (it fires on each recorded loop and on no generation that ended by itself; the forms kept are the forms the budget's cut keeps); a bound on comment lines and a repetition sampler built as options and measured on the calls that looped |
| [S2.2c.2](s2.2c.2_the_model_alone.md) | The model alone, measured | the model reasoning in text, thinking on, under its own sampler with several seeds and greedy with the stop, on the Zebra text, its two edits, the same puzzle with every noun renamed and the sentences reordered, and a puzzle of the same size it cannot have seen: which it solves, which it recalls, where it fails alone; Q-M2.39 and Q-M2.27 with their measurements, for the user |
| [S2.2c.3](s2.2c.3_the_tool_probed.md) | The tool, probed | a few episodes with Ein offered as a function, on the texts S2.2c.2 found the model fails alone on: whether it calls, what it writes, whether the reply changes its answer; what that says of which arrangement leads P2.4. It has an item only where S2.2c.2 leaves it one |

### Order

S2.2c.1 first: S2.2c.2's greedy runs and every later phase's runs read its stop. S2.2c.2 follows; S2.2c.3 follows
S2.2c.2 and reads its table. The phase does not wait for [P2.2e](../p2.2e_kernel_fixes/README.md): no stage here
runs a program of the model's through the kernel but S2.2c.3's few.

**How the phase reaches the model** is P2.2's: the runtime built with the feature `cuda`, the model `qwen3.5:4b`
and only it, every plan of generations committed before its first generation, every run a record, replayed with no
model in EinAf's gate step where a test reads it.

## Acceptance

- A generation whose tail repeats ends with the stop `loop`, where its request asks for it; a request that names
  no such stop generates as before, and every record of P2.2 replays as recorded.
- On every recorded generation of P2.2, with no model: the detector stops each one that ran to its budget in a
  loop, at a token the test names, and none that ended by itself; and on the one shot's eight, the forms the
  reader keeps are the forms the budget's cut keeps.
- The fifteen calls that looped under a grammar, run again with the model: what each writes under the stop alone,
  with a bound on comment lines, and under a chain with a repetition sampler; a plan committed first.
- The model alone has a record on each of the five texts under each decoding, with the answer it gave, whether
  that is right, and whether its thinking named the puzzle; the report says where the minimal goal's premise
  stands.
- Q-M2.27, Q-M2.31 and Q-M2.39 each carry the measurement they rest on; none is ruled by the phase.
- What the phase finds of Ein is in [its evaluation](evaluation/README.md); of the model and EinAf, in
  [its findings](findings/README.md).

## Risks

- **A stop on a loop hides the loop's cause.** The seven steps' calls loop because a grammar refused a token and
  greedy decoding went on from a text it would not have written; a stop saves the budget and gives the same cut
  output. The record keeps the token the grammar refused, and S2.2c.1 reports what each call wrote before its loop.
- **A detector that fires on a list.** A program of fifteen facts of one relation repeats a shape and not a cycle;
  the rule is over the same tokens repeated, and is held on every recorded generation that ended by itself.
- **Sampling makes one run a draw.** Under the model's own chain an answer is one of a distribution, so a text is
  *solved k of n*; the count of seeds is fixed in the plan, before the first generation.
- **A renamed puzzle may still be recalled, and a new one may be easier.** The renaming keeps the structure a
  trained model knows; the unseen puzzle is written to the same size and the same kinds of clue. Both are read
  beside the original, and neither alone is taken for *the model reasons*.
- **Five texts are not a benchmark.** These are records of where the goal's other side stands, in no split and at
  no matched budget; the arm is [S2.5.2](../p2.5_harness/s2.5.2_baselines.md)'s.

## Defaults taken at the plan

Each is the plan's, recorded so that the user can correct it; none is a ruling.

1. **The stop on a loop is a field of a request**, off where a request names none, as a grammar's `check` is, so
   that P2.2's records replay as made. **This phase's own greedy runs set it**, which is
   [Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)'s
   third alternative taken for the phase's runs alone; what the arms run with stays S2.5.2's.
2. **The bound on comment lines and the repetition sampler are built as options and measured, and adopted by no
   default**: [Q-M2.31](../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md)'s
   default, no bound, stands until the user rules on the measurement.
3. **The model alone runs under the model's own chain**, seeded, several seeds a text, with greedy and the stop
   beside it; thinking on; no Ein and no example in the prompt; the answer line S2.2.5 wrote for the Zebra texts.
4. **The renamed puzzle and the unseen puzzle are written by hand**, each with a program by hand that the kernel
   answers *one* with the gold sentence; they are in no split, and are listed for
   [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md) as texts development has seen.
5. **S2.2c.3 is a probe and not the tool**: one function, the kernel's reply shown as `ein solve` prints it, cut
   to a stated size; no level of the ladder, no arm, no matched budget. [S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)
   builds the tool.
6. **Where the model stops solving as a puzzle grows** (the analysis' proposal 23) stays
   [P2.7](../p2.7_failure_scaling_generalization/README.md)'s scaling, on [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md)'s
   generator; this phase takes five texts by hand.
7. **A run's artefacts are not committed** ([what is committed](../README.md#what-is-committed)): each stage
   writes them under a `runs/` folder; its plan, texts, scripts and report are committed.

## Connections

- [The analysis of P2.2](../p2.2_formalizer/report/ANALYSIS.md): § 9.5, the runtime and the decoding; § 10, the
  model solves the puzzle and cannot state it.
- [P2.2b](../p2.2b_guided_thinking/README.md): a thinking with a shape may not loop; this phase is what ends one
  that does.
- [The frame § Acceptance](../README.md#acceptance): the minimal goal, whose two sides are two records of EinAf's.

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic, ids the milestone's next free.
