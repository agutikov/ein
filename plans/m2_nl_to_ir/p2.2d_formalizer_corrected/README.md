# P2.2d — The formalizer corrected

**Status:** **planned 2026-11-27** on the user's word, *"Ok I agree, do it"*, to the routing of P2.2's results at
its close (a new phase before P2.3 for what has no owner: the one shot's formalizer corrected where its own
arrangement made the defects). **Not started**: no stage runs, not even a survey, until the user says so.
**Depends on:** [P2.2](../p2.2_formalizer/README.md), closed: the contract and the steps by hand
([S2.2.1](../p2.2_formalizer/s2.2.1_contract.md), `einaf/contract/`), the grammars
([S2.2.3](../p2.2_formalizer/s2.2.3_gbnf.md)), the catalogue, the prompts, the check and the steps
([S2.2.4](../p2.2_formalizer/s2.2.4_passes.md)), `einaf from-text`, its answer rule and its records
([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)); [P2.2c](../p2.2c_decoding_and_model_alone/README.md): a
generation that ends, before any grammar is narrowed further; the first part of
[S2.5.3](../p2.5_harness/s2.5.3_metrics.md), the answer's comparison, if it has run, else S2.2.5's stand-in.
**Blocks:** [P2.3](../p2.3_benchmark/README.md): an instance is shaped by the contract (its answer queries, what
a `:source` is, the target shape), and the Level B run opens `test` with the formalizer as it then stands;
[P2.2b](../p2.2b_guided_thinking/README.md)'s runs with the model, whose baseline is this formalizer and whose
questions are dealt to the steps as they are cut.
**Carries:** proposals 1 to 7, 15, 16, 17, 25 and 26 of [the analysis of P2.2](../p2.2_formalizer/report/ANALYSIS.md#9-proposals-for-the-other-issues),
4 and 26 as one.
**Waits on the user for:** [Q-M2.35](../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md)
(S2.2d.1), [Q-M2.36](../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md) (ruled on S2.2d.2's
measurement), [Q-M2.37](../open_questions/kernel/Q-M2.37-where-do-a-library-s-descriptions-of-its-rules-live.md)
(S2.2d.3 runs under its default), [Q-M2.33](../open_questions/answer/Q-M2.33-is-a-class-with-several-rows-an-answer-of-several-values-or-the-undetermined-text.md)
and [Q-M2.38](../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md).

## Goal

**The one shot's formalizer is corrected where the miss was its own arrangement's and not the model's**, and each
correction either removes a defect by construction or names it in a sentence. The phase does not tune the model
towards the texts it has: it moves decisions out of the prompt, where the model ignored them, into the contract, a
grammar or EinAf's check, each held by a test with no model.

P2.2's one shot printed *The statements contradict each other.* for the Zebra puzzle, and no word problem was
answered for the text's reason. The machinery is not what failed: the steps by hand go through the same code to the
canonical sentence. Of what did fail, part is the model's reading of the text, which this phase does not claim to
mend, and part is EinAf's:

| what went wrong | whose | its stage here |
|---|---|---|
| two questions folded into one query over one variable; three questions lost to one conjunct with no row | the contract's *one answer query* (default 4) | S2.2d.1 |
| a fixed text scored right on a contradiction, or a missing row, of the model's own | the reader, which scores the string | S2.2d.1 |
| the ontology written twice and never revised; a stale membership kept; an import's list a step before its uses; the mechanism decided in one step and stated in the next | the cut of the steps | S2.2d.2 |
| `converse` paired with a rectangle, `top` over kinds with no member, the partition's link the neighbour relation; kinds empty or of unequal size, names of no kind | an entry says what a rule derives, not what it is for or needs; nothing evaluates a need | S2.2d.3 |
| a passive folded into an active, *next to* read as *in*; a rule that reads what it concludes; a `:why` that names an arrow its declaration lacks | the contract's rule has no enforcer | S2.2d.4 |
| `(solve :no-search)` beside obligations; a calculator's answer relation declared as the program's own | a grammar that offers what the program cannot use | S2.2d.5 |
| *the red house* made a sixth house; no example with a change over time; the categories of a refused program; the nearest miss not measured | the prompts; `einaf::contract`; the phase's own risk list | S2.2d.6 |

## What P2.2 found that this phase starts from

- **The model follows a shape and does not carry over a reading.** The `shops` example shows every one of these
  decisions made right, and the model made them wrong; a step's grammar kept out 11 misspelt selections and 7
  `:source` that are no sentence, and changed no answer
  ([L-M2.28](../findings/model/L-M2.28-the-prompts-first-example-is-the-zebra-puzzle-s-theory-in-miniature.md)).
  That is the argument for a grammar or a check over a prompt.
- **A grammar's refusal has a cost**: it led 7 calls into a loop
  ([L-M2.19](../findings/model/L-M2.19-a-grammar-s-refusal-sends-greedy-decoding-into-a-loop-it-does-not-enter-free.md)).
  So [S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md) comes first.
- **Most of the Zebra program's defects are silent in Ein**: of seven the kernel names two, one at a time, EinAf's
  check two more, and three nothing
  ([L-M2.24](../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md),
  [E-123](../evaluation/diagnostics/E-123-no-value-says-whether-a-selecting-fact-was-taken-up.md),
  [L-M2.18](../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)).
- **The steps help EinAf hold the output; they have not helped the model formalize**
  ([L-M2.25](../findings/einaf/L-M2.25-the-steps-partial-parts-are-written-twice-never-revised-and-a-step-early.md)).
- **The model reads what it is given**: given mechanics, it wrote mechanics; given modules with no description, it
  imported none ([L-M2.30](../findings/model/L-M2.30-modules-shown-with-no-description-were-imported-by-no-run.md)).

## The caution over every stage

**Every correction is drawn from the Zebra text and the thirteen word texts, so after any of them those texts are
a development set by construction** (research plan B4: *a prompt change is an experimental change*). What a change
is worth is shown only on texts no prompt was tuned against, which is
[S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)'s split. So the phase holds itself to what is right by construction or
held by a test with no model; its run of the plan's 29 again is a development number and labelled so; and no stage
reads that number as a result.

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2d.1](s2.2d.1_a_query_a_question.md) | A query a question, and what an answer says | on Q-M2.35's ruling: every query with a `:goal-text` an answer query, one a question of the text, in the contract, the prompts, step 1's grammar, `einaf::answer` and the check; held by an invariance on the Zebra steps by hand. The reader's two counts: right, and right with nothing of the check on the answer's path |
| [S2.2d.2](s2.2d.2_the_cut.md) | The cut of the steps | the six steps by hand cut strictly, as files a test judges, and the step grammars of a strict and a two-step cut, with no model; then the four cuts measured on the sixteen texts, and each step given the hand's earlier steps; Q-M2.36 with its measurement, for the user |
| [S2.2d.3](s2.2d.3_entries_described.md) | An entry described for its use, with what it needs | each catalogue entry says what it is for, each argument's role and need, what it gives the mechanism, what the kernel shows when a need is unmet and what it is written with; the check evaluates the needs at root; a module given beside the library read with its descriptions; measured against the catalogue as it stands and against the names alone |
| [S2.2d.4](s2.2d.4_check_findings.md) | What the check can see where the kernel is silent | findings of the contract's check for a statement whose relation is not its sentence's words, a rule that reads what it concludes where the answer has several rows, an answer that is a statement read back, a `:why` that names an arrow its declaration lacks; each measured on the recorded programs and the steps by hand, with no model |
| [S2.2d.5](s2.2d.5_grammars_from_the_program.md) | Grammars rendered from the program | step 3's grammar offers only the mechanism the program can have; an import lists an entry's names together and no declaration takes a name an offered module has; a statement's relation an alternative a sentence; each for the cut that stands, held on the recorded outputs with no model |
| [S2.2d.6](s2.2d.6_run_again_and_nearest_miss.md) | The prompts again, the run again, and the nearest miss | the prompts' next version (a description is not a new thing; an example with a change over time); the categories of a refused program; the plan's 29 run again, a development number; the model's Zebra program mended one defect at a time by hand, the kernel's word and the check's after each, for S2.4.3; the phase's report |
| [S2.2d.7](s2.2d.7_first_iteration_and_relaxation.md) | What the first iteration should leave, and how a program is relaxed | the user's question, [Q-M2.44](../open_questions/formalizer/Q-M2.44-what-should-the-first-formalization-leave-too-little-or-too-much.md): the recorded programs' errors classified (omitted, wrong, extra, theory); relaxation by cores with no model; statements as hypotheses under `--maximal`; a statement's confidence measured from its tokens' probabilities and, if run, stated in the model's dialect and stripped by EinAf; no keyword of Ein. Q-M2.44 with its numbers, for the user |

### Order

S2.2d.1 first: it changes what a program's answer is, which every later stage's runs and the benchmark read.
S2.2d.2 next, since the cut decides what each step's grammar and prompt hold, and its oracle form is what S2.2d.3
measures descriptions in. S2.2d.3 and S2.2d.4 need no model to build and do not wait for each other. S2.2d.5
follows the cut that stands and S2.2d.4's first finding. S2.2d.7, added 2026-11-27 on the user's question, needs
no model but for its confidence arm and may run at any time after P2.2's records; its stated-confidence arm, if
the user wants it, rides on S2.2d.6's run. S2.2d.6 closes the phase.

**Where a stage waits for the user.** S2.2d.1 runs on Q-M2.35's ruling. S2.2d.7 ends on a question, Q-M2.44. S2.2d.2 ends on a question, Q-M2.36, and
the stages after it are written to hold under today's cut, which stands until the user rules. The others take the
defaults their files state.

**How the phase reaches the model and the kernel** is P2.2's: the runtime built with the feature `cuda`, the model
`qwen3.5:4b` and only it, greedy with thinking off as P2.2's runs were, with the stop on a loop; every program
through `einaf::kernel`'s bounded call; every plan of generations committed before its first generation.

## Acceptance

- Each correction is held by a test of EinAf's gate step with no model: on the steps by hand and the entry
  programs, no new finding and no refusal; on the model's recorded programs, the finding or the refusal the stage
  states.
- The contract's page says what was changed ([04-contract](../../../docs/einaf/04-contract.md)), and the steps by
  hand under `einaf/contract/` follow it.
- The questions the phase measures for (Q-M2.36, and Q-M2.37's descriptions) carry their measurements; none is
  ruled by the phase.
- The plan's 29 run again on the corrected formalizer, beside P2.2's 29, with the same reader and the second
  count; labelled a development number in every table that shows it.
- **The nearest miss has a record**: the least edit of the model's Zebra program that answers, defect by defect,
  with what the kernel and the check said after each.
- What the phase finds of Ein is in [its evaluation](evaluation/README.md); of the model and EinAf, in
  [its findings](findings/README.md); and the phase ends with a report for the user's review.

## Risks

- **Corrections fitted to sixteen texts.** The caution above; the split is where a change is priced.
- **A narrower grammar starts more loops.** Every refusal is a place greedy decoding can leave its own text;
  S2.2d.5 reports, for each rendering, the calls whose first choice it refused and what they wrote next.
- **A check that speaks too much.** A finding on a right program teaches the loop to repair what is not broken.
  Each new finding is held to none on the steps by hand and the entry programs, and a verb's forms are allowed
  before a relation is called not its sentence's.
- **The cut is the user's notes.** A measurement on sixteen texts, one greedy run each, is thin ground for
  changing them; S2.2d.2 says what it found and the user rules.
- **Descriptions written by whoever wrote the library.** [The analysis § 12.3](../p2.2_formalizer/report/ANALYSIS.md#123-the-eight-rules-in-the-words-of-their-use)'s
  draft is checked by nothing, and its first reader is the user.
- **What step 1 gets wrong in the statements stays.** Reading *next to* as *in* is named by S2.2d.4's finding and
  refused by S2.2d.5's grammar where the words make it mechanical; a name made up for a description, a kind, a
  polarity are the model's, and [P2.2b](../p2.2b_guided_thinking/README.md)'s and
  [S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)'s to see.

## Defaults taken at the plan

Each is the plan's, recorded so that the user can correct it; none is a ruling.

1. **The phase changes EinAf alone.** A reading the library lacks (a change over time, one selecting fact a verb
   for the read-back) is [Q-M2.23](../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md)'s,
   whose default, no module, stands; what Ein owes is [P2.2e](../p2.2e_kernel_fixes/README.md)'s.
2. **The target shape stays `zebra3.1.ein`** ([Q-M2.38](../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md)'s
   default), and S2.2d.6 states its abbreviation as the rule it is.
3. **Today's cut stands until the user rules on S2.2d.2's measurement**; the strict and the two-step cut are
   modes beside `--mode steps|whole`, and no stage after S2.2d.2 depends on which is taken.
4. **Descriptions are written in EinAf's tree**, beside each catalogue entry, in a layout a library could take
   over, until Q-M2.37 says where a library's descriptions live.
5. **A new finding of the check is a finding first and a grammar's rule second**: it is measured on the records
   before anything refuses a token for it.
6. **The run again is the plan of S2.2.5's 29, unchanged**: the same texts, the same two conditions, the same
   bounds, greedy with thinking off, so that the two runs differ by the formalizer and the stop on a loop alone.
7. **EinAf's kernel call is as P2.2 left it** ([Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)'s
   default, no lever), so a dead root on a link several rules derive costs its minute; S2.2d.6 records what that
   came to.
8. **A run's artefacts are not committed** ([what is committed](../README.md#what-is-committed)).

## Connections

- [The analysis of P2.2](../p2.2_formalizer/report/ANALYSIS.md): §§ 8, 9, 11 and 12, and § 9.7's order, whose
  first group is this phase.
- [The contract](../../../docs/einaf/04-contract.md), [the grammars](../../../docs/einaf/06-grammars.md),
  [the steps](../../../docs/einaf/07-steps.md), [from a text to a short answer](../../../docs/einaf/08-from-text.md):
  the pages this phase changes.
- [P2.6](../p2.6_ablations/README.md): G8 puts a model in front of the steps and G9 varies which library is
  listed; neither cuts the steps another way or varies how a rule is described, which is why those are here.
- [P2.4](../p2.4_loop/README.md): the loop repairs a program; this phase corrects what writes the first one.

## Ledgers

[`findings/`](findings/README.md), [`open_questions/`](open_questions/README.md) and
[`evaluation/`](evaluation/README.md), one item a file under a topic, ids the milestone's next free.
