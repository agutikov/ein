# P2.2 — The formalizer (Stage B)

**Status:** **closed 2026-11-27** on the user's word, *"Ok I agree, do it"*, to the routing of the phase's results
([§ The close](#the-close-2026-11-27)); its folder is kept, as M2 keeps a closed phase's. It was **started
2026-11-26** on the user's word, *"start next phase plans/m2_nl_to_ir/p2.2_formalizer, execute s2.2.0a and pause"*;
S2.2.0a ran to its end in a subagent, which committed its work; then, on the user's word the same day, *"Ok, I like
what I see, after s2.2.0a continue with other stages of the phase"*, the phase ran its other stages to its end,
S2.2.1 to S2.2.5, one by one, each in a subagent that committed its work, and stopped for the user's review with
[its report](report/README.md) and, on the user's questions, [an analysis of the phase against its goals](report/ANALYSIS.md).
**Depends on:** P2.1:
[S2.1.1](../../../docs/einaf/README.md#the-package) creates `einaf/`, the Cargo package every
stage here adds to, and writes the operational test of the seven outcomes;
[S2.1.2](../../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md) gives a refusal of the kernel a kind and a
place, as values; [S2.1.3](../../../docs/einaf/03-feedback.md) is the feedback object
`einaf from-text` reads the kernel through, and the bounded call every stage runs a generated program by. The
language (Rust,
[Q-M2.7](../open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)), the model (`qwen3.5:4b`,
[Q-M2.5](../open_questions/model/Q-M2.5-local-model-choice.md)) and what runs it (a Rust binding to llama.cpp,
[Q-M2.14](../open_questions/model/Q-M2.14-what-runs-the-model.md)) are the user's rulings: no stage here decides one.
**Blocks:** [P2.4](../p2.4_loop/README.md): the filter's GENERATE state is this phase's one shot, and the tool
([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)) writes Ein through this phase's runtime with no grammar;
[P2.5](../p2.5_harness/README.md): baseline B2 is this phase's output unchanged. The **Level B gate** is
[P2.3](../p2.3_benchmark/README.md)'s: a formalizer is Level B only on tasks it has not seen.
**Research plan:** [`EinAf.md` § Stage B](../EinAf.md#stage-b--define-autoformalization-as-an-explicit-task),
B1 to B4.
**Written to:** [the frame](../README.md#the-plan-as-re-planned), with
[the audit of this phase](../analysis/audit_p2.2.md) as the worklist: where a file here says what changed, the
audit's section has the evidence.

---

## The close (2026-11-27)

**The user's words.** On the report and the analysis: *"read results of p2.2 … analyze what can go into milestone
ledgers after phase close, what has to go into existing phases, what needs new phases p2.2* (before p2.3)?"*; and on
the routing that answered it: *"Ok I agree, do it"*. The coordinator read the second as closing the phase, which the
user may correct.

**The verdict the phase closes with** is [the analysis'](report/ANALYSIS.md#summary): partially met. What the phase
was to build and to measure is there and replays; of [its twelve acceptance items](#acceptance) 8 are met, 3
partially and 1 not.

| the item short at the close | where it is carried |
|---|---|
| 1, not met: the Zebra puzzle, one shot, to its answer | the minimal goal, open for the loop ([Q-M2.17](../open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md), [P2.4](../p2.4_loop/README.md)); the nearest miss is [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md)'s; whether the Level B gate's first criterion is held as written is the user's at that gate |
| 3, partially: the judge finds no event in any reference program | [Q-M2.22](../open_questions/answer/Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md): the Zebra steps by hand carry 11 events and the seating 5 under its default |
| 4, partially: every form of every recorded program in one category row | [L-M2.26](../findings/einaf/L-M2.26-a-program-the-kernel-refuses-carries-no-categories.md), [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) |
| 11, partially: Q-M2.6 and Q9 decided | each carries a default and its measurement, and is the user's to rule ([Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md), [Q9](../open_questions/formalizer/Q9-ontology-provenance.md)) |

**What the close moved**, by the ledgers' rule (what is open to the milestone, what is closed kept here):

- the thirteen questions, Q-M2.22 to Q-M2.34, to [the milestone's ledger](../open_questions/README.md), none ruled;
- the eight open findings to [the milestone's findings](../findings/README.md); the eleven closed stay in
  [`findings/`](findings/README.md);
- the twenty-four entries of the evaluation, all open or strengths, to
  [the milestone's evaluation](../evaluation/README.md);
- the defaults, mapped in [`defaults/p2.2.md`](../defaults/p2.2.md);
- and, filed at the close because [the analysis](report/ANALYSIS.md) held them and no ledger did: L-M2.24 to
  L-M2.30, Q-M2.35 to Q-M2.42 and E-128.

**Where the phase's work goes on.** Three phases planned at the close, none started, each before
[P2.3](../p2.3_benchmark/README.md): [P2.2c](../p2.2c_decoding_and_model_alone/README.md), a generation that ends
and the model alone; [P2.2d](../p2.2d_formalizer_corrected/README.md), the formalizer corrected;
[P2.2e](../p2.2e_kernel_fixes/README.md), Ein's two bugs and a clock. [P2.2b](../p2.2b_guided_thinking/README.md),
guided thinking, was planned before. The phases after them carry a section each on what this phase hands them.

**Where each proposal of the analysis went** (its numbers, [§§ 8 to 12](report/ANALYSIS.md#9-proposals-for-the-other-issues)).
A proposal stays a proposal: a stage that carries one says what it waits for.

| # | the proposal | where |
|---:|---|---|
| 1 | an answer query for each question | [Q-M2.35](../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md); [S2.2d.1](../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md) |
| 2 | a statement's relation made of its sentence's words | [S2.2d.4](../p2.2d_formalizer_corrected/s2.2d.4_check_findings.md), a finding; [S2.2d.5](../p2.2d_formalizer_corrected/s2.2d.5_grammars_from_the_program.md), a grammar |
| 3 | a description is not a new thing | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) |
| 4 | each catalogue entry states what it needs, and the check evaluates it | [S2.2d.3](../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md), with 26 |
| 5 | step 3's grammar offers only the mechanism the program can have | [S2.2d.5](../p2.2d_formalizer_corrected/s2.2d.5_grammars_from_the_program.md) |
| 6 | an import lists an entry's names together | [S2.2d.5](../p2.2d_formalizer_corrected/s2.2d.5_grammars_from_the_program.md) |
| 7 | a change over time: an example, a finding, a reading in the library | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md); [S2.2d.4](../p2.2d_formalizer_corrected/s2.2d.4_check_findings.md); [Q-M2.23](../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md) |
| 8 | one selecting fact a verb for the read-back | [Q-M2.23](../open_questions/kernel/Q-M2.23-does-the-library-grow-a-reading-of-a-literal-vocabulary.md) |
| 9 | stop a generation that loops | [S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md); [Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md), [Q-M2.31](../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md) |
| 10 | the target shape against *the text's own words* | [Q-M2.38](../open_questions/formalizer/Q-M2.38-is-a-description-abbreviated-to-its-word-or-written-as-a-term.md) |
| 11 | a step shown its own findings before the next | [Q-M2.41](../open_questions/formalizer/Q-M2.41-is-a-misread-statement-repaired-before-any-theory.md); [S2.4.2](../p2.4_loop/s2.4.2_repair.md); [P2.6](../p2.6_ablations/README.md)'s arm |
| 12, 18 | the model alone, under its own sampler, on five texts | [S2.2c.2](../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md) |
| 13 | the clock and the lever | [S2.2e.3](../p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md); [Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md) |
| 14 | E-104 and E-105 before the loop | [S2.2e.1](../p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md), [S2.2e.2](../p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md); [Q-M2.42](../open_questions/kernel/Q-M2.42-are-e-104-and-e-105-fixed-before-the-benchmark-is-built.md) |
| 15 | right, and right for the program's reason | [S2.2d.1](../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md); [S2.5.3](../p2.5_harness/s2.5.3_metrics.md) |
| 16 | the repair ladder, by hand, once | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md), for [S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md) |
| 17 | the small gaps of the acceptance | [S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) (the categories); [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md) (Ein's commit); the user (Q-M2.6, Q9) |
| 19, 21 | Ein as the checker of the model's own answer; that answer as a level of the ladder | [Q-M2.40](../open_questions/einaf/Q-M2.40-is-the-model-reasons-and-ein-checks-its-answer-an-arm.md); [S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md), [S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md) |
| 20 | think, then state | [P2.2b](../p2.2b_guided_thinking/README.md) |
| 22 | the tool probed early | [S2.2c.3](../p2.2c_decoding_and_model_alone/s2.2c.3_the_tool_probed.md) |
| 23 | where the model stops solving | [P2.7](../p2.7_failure_scaling_generalization/README.md)'s scaling, on [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md)'s generator |
| 24 | the minimal goal's instance | [Q-M2.39](../open_questions/milestone/Q-M2.39-what-is-the-minimal-goal-s-instance-if-the-model-alone-solves-the-zebra-text.md) |
| 25 | the cut of the steps, measured | [S2.2d.2](../p2.2d_formalizer_corrected/s2.2d.2_the_cut.md); [Q-M2.36](../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md) |
| 26 | a rule described for its use, by the library | [S2.2d.3](../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md); [Q-M2.37](../open_questions/kernel/Q-M2.37-where-do-a-library-s-descriptions-of-its-rules-live.md); [E-128](../evaluation/stdlib/E-128-a-library-cannot-say-what-a-selecting-fact-is-for-or-what-it-needs.md) |

## Goal

The neural component does **one difficult thing**: it turns a problem stated in English into an Ein program, one
shot, under a written contract, and the kernel answers. It does not solve. Repair is
[P2.4](../p2.4_loop/README.md)'s and the benchmark is [P2.3](../p2.3_benchmark/README.md)'s. The phase ends with
`einaf from-text`: a text becomes a program and **the short answer that program prints**, with the run's record
([S2.2.5](s2.2.5_from_text.md)). The command is EinAf's and the kernel stays free of any model
([the frame, § Ein, ein.rs and EinAf](../README.md#ein-einrs-and-einaf)).

**The formalization is a search for an ontology and a theory around facts that stay close to the text** (the
user, 2026-11-25: *"formalization is search of ontology and theory for given facts, so facts should be as close to
original text as possible and changed less then more variable rules"*), **made in three steps: the facts and a
partial ontology first**, then the complete ontology and a partial theory, then the complete theory. Those are the
user's notes on the whole phase, [below](#formalization-in-steps-the-users-notes-on-the-whole-phase), and every
stage carries its part of them. The phase begins with the one stage the user added before the others,
[S2.2.0a](s2.2.0a_zebra_formulations.md): it compares the two Zebra encodings, measures how the generic vocabulary
of `zebra.ein` fares against the literal projections of `zebra2.ein`, writes the text's own words four ways
(`zebra3.1.ein` to `zebra3.4.ein`),
and checks that a program written in the order the steps make it is read as Ein reads it in its usual order.

The phase also takes the milestone's cheapest early measurement: **whether the model writes loadable Ein with no
grammar at all** ([S2.2.2](s2.2.2_model_runtime.md)), which the user expects may be possible
([Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)) and which bears on every stage after it.

## Formalization in steps: the user's notes on the whole phase

The notes were first a stage of their own, S2.2.0f, planned on the user's word of 2026-11-25. The user, 2026-11-26:
*"re-plan s2.2.0f - it must be not a separate stage, it's my notes about whole formalizer phase"*. So they stand
here, above every stage, and each part of them is carried out by the stage it belongs to
([the table at the end of this section](#where-each-part-of-the-notes-is-carried-out)).

### The user's words (2026-11-25)

> p2.2 new stage before first stage - s2.2.0f. Formalization goes in several steps: 1 - facts as literal
> translation of original problem statements + partial ontology with identified relations declarations and part of
> additional required facts if needed (like instances, types, is-a, whatever else), partial here means it is
> expected to change in next step; 2 - complete ontology + partial theory, some imported rules, some hand-written;
> 3 - complete theory; First step is simple - need only to understand what words mean same objects, what are
> objects and what are relations, one fact per sentence or many facts per sentence, find out if we have objects of
> different types or same. Next steps obviously depend on previous. After formalization is done we can run solver
> and consume feedback.

and, with [S2.2.0a](s2.2.0a_zebra_formulations.md):

> The idea is that formalization is search of ontology and theory for given facts, so facts should be as close to
> original text as possible and changed less then more variable rules.

and, later the same day:

> Promt/context will contain ein syntax and semantic explanation anyway, otherwise LLM doesn't know what is it.
> So there would be examples
> Then first experiment would be with original zebra problem and with working zerba program as example - test for dummies.

Kept as stated.

### The three steps

The plan's first re-plan had four passes, **ontology → theory → instance → query**: the ontology first, so that the
facts name nothing that does not exist. The user's steps put **the facts first**: they are the part of a
formalization closest to the text, the part a model can write by reading, and the part that should change least;
the ontology and the theory are what is searched for, around them. A step's output is partial on purpose: it is
expected to change in the next step.

| step | what it writes | what it decides | what stays partial |
|---|---|---|---|
| **1** facts and a partial ontology | each statement as a fact (or several) in the text's own words, with `:source` its sentence; the declarations of the relations those facts use, each with `:why`; the memberships the facts need as far as they are seen (`instance`, `type`, `is-a`, *whatever else*) | **which words name the same object**; what is an object and what is a relation; one fact a sentence or several; whether the objects are of one kind or of several | the declarations' labels, the memberships, the kinds |
| **2** the complete ontology and a partial theory | every relation and membership the theory will read; imports, and the facts that select what they bring; some rules of the program's own | which library theory, which properties | the rules |
| **3** the complete theory | the rest of the rules; the hypothesis mechanism | — | nothing: the program is run |

**Next steps depend on the previous ones**, and once the program is complete the kernel is run on it and its
feedback consumed: that loop is [P2.4](../p2.4_loop/README.md)'s, and what it changes first follows the user's
principle, **the theory before the facts**.

### Five design questions, each with its default

Five things the steps leave to be decided. Each default is the plan's, for the user to correct, and
[S2.2.1](s2.2.1_contract.md)'s T2.2.1.7 writes the choice taken into the contract.

- **Where the questions go.** A question is a sentence of the text (*Who drinks water?*). Default: step 1
  translates it as literally as a statement, into the answer query over the facts' own relations with its
  `:goal-text` sentence, and the theory has to make that query answerable.
- **What the kernel is asked between steps.** A step's program can be run: step 1's under `--no-search` (root
  alone: does it load, is root alive; with no hypothesis mechanism a solve would be answered by the blind
  enumerator, [E-13](../evaluation/hypotheses/E-13-a-program-that-names-no-hypothesis-mechanism-is-answered-by.md)), step 2's for what it already derives. Default: each step's
  program is loaded and judged at root, its feedback kept in the record, and only the complete program's is the
  loop's; whether the model is shown a step's feedback before the next step is an arm of
  [P2.6](../p2.6_ablations/README.md).
- **The program's text is the steps' outputs one after another**, so that it reads in the order it was made: the
  facts, the ontology, then the theory (the user: *"the final program text follow it's synthesis order"*).
  [S2.2.0a](s2.2.0a_zebra_formulations.md)'s T2.2.0a.3 checks that Ein reads a program written so and answers it
  as it answers the same forms in their usual order; until then it is the default, and the assembly of
  [S2.2.4](s2.2.4_passes.md)'s T2.2.4.6 keeps it.
- **What a later step may change of an earlier one.** Partial means *expected to change*: step 2 completes the
  ontology step 1 began, and may rename a membership or a kind. Default: a later step adds to the facts and does
  not rewrite them; a rewritten fact is recorded as such, and counted.
- **What EinAf checks where the kernel cannot.** A fact of an undeclared relation and a form at another count load
  in silence ([E-12](../evaluation/language/E-12-an-undeclared-relation-and-a-form-of-a-declared-relation-at.md)); a selection under an unlisted name is inert
  ([E-11](../evaluation/stdlib/E-11-a-theory-selected-under-a-name-the-file-does-not-list-is.md)). The contract's check ([S2.2.4](s2.2.4_passes.md)'s T2.2.4.6) runs on
  each step's program, with the findings that apply to it.

### Where each part of the notes is carried out

| the notes | where |
|---|---|
| the three steps stated: each step's input, its output, what *partial* means for it, what it decides and may change, how it is checked, and the five questions above with the choice taken | [S2.2.1](s2.2.1_contract.md), T2.2.1.7, a section of the contract (it was S2.2.0f's T2.2.0f.1) |
| the Zebra puzzle through the three steps by hand, ending at `zebra3.1.ein`, the target shape; two or three other texts the same way; each step's program judged | [S2.2.1](s2.2.1_contract.md), T2.2.1.8, the contract's worked examples of the steps (T2.2.0f.2 and T2.2.0f.3) |
| *"Promt/context will contain ein syntax and semantic explanation anyway … So there would be examples"* | [S2.2.1](s2.2.1_contract.md), T2.2.1.3: B3's excerpt and examples, in every prompt of every arm |
| *"first experiment would be with original zebra problem and with working zerba program as example - test for dummies"* | [S2.2.2](s2.2.2_model_runtime.md), T2.2.2.6: the probe's first item, the phase's first measurement |
| *"First step is simple"*: step 1 alone with the model, set against the hand-written step 1 | [S2.2.2](s2.2.2_model_runtime.md), T2.2.2.8 (T2.2.0f.4) |
| a grammar a step | [S2.2.3](s2.2.3_gbnf.md), T2.2.3.3, and [Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md) |
| the steps, a prompt each; *select before synthesise* inside steps 2 and 3; the catalogue serving steps 2 and 3; the contract's check on each step's program; the program in synthesis order | [S2.2.4](s2.2.4_passes.md) |
| `einaf from-text` runs the steps, and the record keeps each step's program and its judgement | [S2.2.5](s2.2.5_from_text.md) |
| *"After formalization is done we can run solver and consume feedback"*: a repair changes the theory before the facts; a fact rewritten after step 1 is counted; the facts' faithfulness judged at step 1 | [P2.4](../p2.4_loop/README.md#the-theory-before-the-facts), [S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md) |
| what partial programs show of the kernel | each stage's task of what it found of Ein (T2.2.0f.6) |
| S2.2.1, S2.2.3, S2.2.4 and S2.2.5 re-planned to the steps, with the notes in P2.4 and S2.4.4 | done at the re-plan of 2026-11-26 that dissolved S2.2.0f (T2.2.0f.5) |

## What the formalizer emits

Research plan B1 allows *an Ein program* and, optionally, a decomposition first. The decomposition here is the one
the kernel's own input has, with two parts the plan of August did not carry: the hypothesis mechanism, which the
answer depends on, and the answer query's `:goal-text`, which is how the program prints its answer
([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)).

| part | what it is, in today's Ein | what the kernel holds it to |
|---|---|---|
| **ontology** | `(relation NAME LABEL* :why "…")` forms, and membership facts of a relation the program chooses | a `:why` on every relation, or the load refuses ([§ 2.10](../../../docs/ein/user/04-reference/07-diagnostics.md#210-a-form-with-no-why)); a label is no type and no form is checked against one ([Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)) |
| **theory** | `(import MODULE :symbols (NAME+))` and the facts that activate what was imported; a `rule` with its `:why` only where the catalogue has nothing | **nothing, where the spelling is wrong**: a short name the file lists from no import is the file's own, and the fact is inert with no message ([`04-stdlib/` § What to list](../../../docs/ein/user/04-reference/04-stdlib/README.md#what-to-list)) |
| **hypothesis mechanism** | what proposes the candidates: the library's obligations, a rule that concludes `(open F)`, a written `(open F)`, `(solve :no-search)`, or a `(solve :hypothesis-relations (…))` list for the blind enumerator | the mechanisms may not contradict one another ([§ 2.26](../../../docs/ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)); **a program that states none is answered by the blind enumerator**, in silence ([hypotheses, the writers](../../../docs/ein/user/03-syntax-and-semantics/09-hypotheses.md#where-candidates-come-from-the-writers)) |
| **instance** | `(R a b :source "…")` and `(not (R a b) :source "…")`, the text of each `:source` the sentence it states | a fact takes `:source`, `:rule` and `:using` and no other keyword ([Q-M1e2.28](../../../docs/decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)); names are ASCII; a number is a name, and a quantity that is computed is a numeral of the calculator, `(n z 5)` |
| **query** | `(query :goal PREMISE :goal-text "…")`: **the answer query**, the one query that carries a `:goal-text`, a sentence that says the answer in the problem's words, as every arm is asked for one ([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)); any other query is a check | a goal is held to what a rule's `:match` is held to, and names only relations the program writes ([§ 2.21](../../../docs/ein/user/04-reference/07-diagnostics.md#221-a-goal-is-held-to-what-a-rules-match-is-held-to)) |

**The five parts are made in the three steps** ([above](#the-three-steps)): the instance and the answer query in
step 1, with the ontology begun; the ontology completed and the theory begun in step 2; the theory and the
hypothesis mechanism completed in step 3. The program's text is their outputs in that order.

Four things follow, and each is a stage's subject.

**The theory part is selection before synthesis, inside steps 2 and 3, and selecting is spelling.**
[F12](../../followups/f12_rules_and_relations/ideas.md): *select a theory, do not invent properties*. The order
is the user's three neural actions ([F13](../../followups/f13_puzzles_beyond_zebra/ideas.md): reinterpretation,
theory selection, theory synthesis) with the third made expensive on purpose, and it is the activator induction
[F7 B](../../followups/f7_rule_induction.md#connection-to-m2) put on this milestone's critical path. What the plan
of August did not know is that a selection has a spelling the kernel does not check: `(transitive left-of)` selects
the library's rule only under a name the file lists from the module that declares it
([the audit, A.1 row 2 and runs s11, s12](../analysis/audit_p2.2.md#a1-the-language)). So the catalogue the model is
given says how each property is written to be active, and each of its entries is a program the kernel runs
([S2.2.4](s2.2.4_passes.md)).

**The hypothesis mechanism is part of the output.** A program of relations, facts and a query with no mechanism
does not sit idle, as the plan of August had it: the blind enumerator proposes a fact for every pair of guessable
atoms of each declared relation of one or two labels. Five forms of that kind, run at this re-plan, have seven
candidates and a region of 128 states, and `--single` answers `Ambiguity`
([S2.2.1](s2.2.1_contract.md) has the program). With `std.slots` selected the library's obligations propose the
candidates and the program writes no form for them. So the contract names the mechanism and the record says which
writer the kernel reported.

**Every form carries where it came from.** B2's five categories (*source-derived fact*, *generic theory*,
*generated auxiliary relation*, *generated inference rule*, *generated assumption*) are what makes an invented
constraint countable later ([S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)). The language decides how they are
marked: a fact takes no keyword but its three, so the category is not written in the program. It is computed from
the program, the input text and the catalogue, and kept in the record ([S2.2.1](s2.2.1_contract.md)).

**The model knows exactly what it is given, and the answer is the program's.** B3: the excerpt of Ein's
reference, the catalogue, the examples, the problem; each versioned and hashed. B4: every one of those is a field
of the record, and **a prompt change is an experimental change**. The answer is no text the model writes beside its
program: it is the answer query's `:goal-text`, filled by the kernel per row, with EinAf's formatters over the
values that do not read as text, in the form the answer line asks of every arm
([the frame's default 7](../README.md#the-defaults-the-re-plan-took)). A line is printed per row and for nothing
else, so where the answer query has no single class with a row, or the outcome gives no state to read, the
answer is a fixed text of EinAf's, chosen by the kernel's values ([S2.2.5](s2.2.5_from_text.md); the frame's
default 15).

### The target shape, and the program that is the model of it

[`examples/zebra3.1.ein`](../../../examples/zebra3.1.ein), the Zebra puzzle's statements in the text's own words with
one relation a verb, **is the shape the contract describes**, by default ([S2.2.0a](s2.2.0a_zebra_formulations.md)'s
table, [default 1](#defaults-taken-at-the-re-plan)): its nineteen statements are the sentences, each a fact of its
verb in its voice with the sentence as `:source` (`(drinks Ukrainian Tea)`, `(is-drunk-in Coffee Green)`), a
description written as the word it is built on; its one query asks the questions in the statements' own relations,
`(drinks ?who_water Water)` and `(owns ?who_zebra Zebra)`, with its `:goal-text`; its ontology is twenty-two
`relation` forms with `:why`, thirty-two memberships and the four facts of the row; its theory is two imports and
thirty-two selecting facts of the library's (`std.slots`' partition and `std.algebra`'s `includes`, `converse`,
`compose`, `top` and `meet`) and no rule; and it is written in that order. The library's obligations write H (`ein
show hyps`: *H, from `obligations`: 112 candidates*), and `ein solve --single` answers `Solution` with *“The
Norwegian drinks water. The Japanese owns the zebra.”*. The re-plan's model was
[`examples/zebra.ein`](../../../examples/zebra.ein) less its rule (its facts the theory's own `co-located`), and
[`examples/zebra2.ein`](../../../examples/zebra2.ein), which the plan of August named, carries fifteen rules of its
own ([the audit, B.0](../analysis/audit_p2.2.md#b0-the-four-passes-in-todays-language)); the other literal
vocabularies, `zebra3.2` (descriptions), `zebra3.3` (adjectives as relations) and `zebra3.4` (the verb as an
argument), write six, five and fourteen rules of their own
([the analysis](zebra_formulations/README.md#9-what-follows-for-the-formalizer)). The word problems of
[`examples/word_problems/`](../../../examples/word_problems/README.md) are the model of a sentence beside the fact
that states it. Their theory is four modules of that directory, the family's own: the main arms show the model
the library and not those ([S2.2.4](s2.2.4_passes.md)), so a word problem there is written over the calculator
with whatever rule the model needs.

### The formalizer and the two arrangements

The frame has two arrangements of one model and one kernel ([the arms](../README.md#the-arms)). This phase builds
what both use and the formalizer of one of them.

| | the filter (B2 to B5) | the tool (B6) |
|---|---|---|
| the runtime ([S2.2.2](s2.2.2_model_runtime.md)) | every call, thinking off | every turn, with a function declared to the model and the context kept between turns |
| the contract and what the model is given ([S2.2.1](s2.2.1_contract.md)) | the prompt of every step | the tool's function description, which carries what the filter's prompt holds of Ein ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)) |
| the grammars ([S2.2.3](s2.2.3_gbnf.md)) | the filter's mechanism: the whole output of a call held to Ein, a grammar a step | none: the model writes Ein as it can |
| the steps ([S2.2.4](s2.2.4_passes.md)) | GENERATE | none |
| the answer and its formatters ([S2.2.5](s2.2.5_from_text.md)) | what the program prints | what the tool's last run printed, where the record compares the two |

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2.0a](s2.2.0a_zebra_formulations.md) | Three Zebras: the generic vocabulary, the projections, and the text's own words | **ran to its end** 2026-11-26 ([its record](s2.2.0a_zebra_formulations.md#record), [the analysis](zebra_formulations/README.md)): `examples/zebra3.1.ein` to `zebra3.4.ein`, the target shape `zebra3.1` by default. Planned to end with: `zebra.ein` and `zebra2.ein` measured side by side for a formalizer (distance from the text, the theory and who writes it, the kernel's work and what it says back, what a mistake does); `examples/zebra3.ein`, its facts the sentences' own words, monotone and held to the published answer; each encoding written in its synthesis order (facts, ontology, theory) and held to the same answer; the target shape decided by the table |
| [S2.2.1](s2.2.1_contract.md) | The formalization contract: B1 to B4, and the steps | **ran to its end** 2026-11-26 ([its record](s2.2.1_contract.md#record), [the page](../../../docs/einaf/04-contract.md)): the contract, `docs/einaf/04-contract.md`; the categories computed by `einaf::contract`; the Zebra puzzle, a seating and three word problems through the three steps by hand under `einaf/contract/`, held by `einaf/tests/contract.rs`. Planned to end with: one page, `docs/einaf/contract.md`: the input, the five-part output made in three steps with the hypothesis mechanism and the answer query, the steps stated with the five design questions decided, what a flagged program is, the categories by step with which of them is a faithfulness event, what the model receives, what is recorded; the Zebra puzzle and other texts through the three steps by hand, each step's program a file a test judges |
| [S2.2.2](s2.2.2_model_runtime.md) | The model's runtime, and whether the model writes Ein | **ran to its end** 2026-11-26 ([its record](s2.2.2_model_runtime.md#record), [the probe's report](model_runtime/README.md), [the runtime's page](../../../docs/einaf/05-model.md)): `einaf::model`, the binding `llama-cpp-2` 0.1.161 behind the feature `cuda`, the model pinned by its SHA-256, the envelope, replay; held to `llama-server` (the prompt's tokens alike on fourteen requests, the text on eleven), repeatable; the first experiment (thinking off, `zebra3.1.ein` copied whole, *one*; thinking on, a loop to the budget); the probe's 87 runs, and step 1 with the model. Planned to end with: the binding chosen and built for the GPU, the model's file pinned by its SHA, the envelope every call returns, the binding held to `llama-server` on the same file; **the probe**, first the user's experiment (the Zebra puzzle with a working Zebra program as its example), then parse rate, load rate and the kinds of error of Ein written with no grammar, and step 1 alone set against the hand-written step 1 |
| [S2.2.3](s2.2.3_gbnf.md) | The grammars | **ran to its end** 2026-11-26 ([its record](s2.2.3_gbnf.md#record), [the report](grammars/README.md), [the page](../../../docs/einaf/06-grammars.md)), and **amended on the user's word** the same day ([the amendment](s2.2.3_gbnf.md#amended-on-the-users-word), [its records](grammars/narrowed/README.md)): the grammars a model writes under leave out `<`, `>`, `|` and the backtick but in a sentence of the input, and `/` in a name, so that no marker of the chat template and no fence can be written ([L-M2.15](findings/model/L-M2.15-the-model-s-grammar-admitted-the-chat-template-s-markers-and-the-fence.md)), a request for a whole program carrying `root`'s 383 rules; [Q-M2.30](../open_questions/model/Q-M2.30-does-a-grammar-the-model-writes-under-fix-its-layout.md), a fixed layout, for the user. `einaf::grammar`, the whole grammar `einaf/grammars/ein.gbnf` generated from `ein.l` and `ein.y` (two roots, `program` Ein's language exactly), accepting the corpus's 937 files exactly when the parser does and read alike by llama.cpp's sampler; a grammar a step (1, 2, 2b, 3) written on the program's vocabulary, each accepting the steps by hand; a grammar begun inside the fence and after the thinking at `</think>`, held to by the chosen token first, at no measurable cost; [Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md)'s default, *many*, for the user. Planned to end with: a GBNF grammar generated from `ein.l` and `ein.y`, held to the parser in both directions; a grammar a step, parameterised on the program's own vocabulary where the step has one; where a grammar begins and how an output ends; [Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md) decided |
| [S2.2.4](s2.2.4_passes.md) | The steps: the facts first, then the ontology, then the theory | **ran to its end** 2026-11-26 ([its record](s2.2.4_passes.md#record), [the report](steps/README.md), [the page](../../../docs/einaf/07-steps.md)): `einaf::catalogue` (nineteen entries, each run by the kernel), `einaf::prompts` (the excerpt v2, a template a step, three examples), `einaf::check` (the seven findings), `einaf::formalize` and `einaf steps run|check`, `einaf::kernel::root` (the writer of H); the model through the three steps on the sixteen texts under the steps' grammars, in one call, and with no grammar (64 runs): 14 of 16 programs load, the Zebra puzzle's dead at step 2 with a core naming the misread sentence, two word texts the right count among others; [Q9](../open_questions/formalizer/Q9-ontology-provenance.md)'s default, B with C, A the escape hatch; B2's mode, the steps ([Q-M2.32](../open_questions/formalizer/Q-M2.32-which-mode-does-baseline-b2-run-with.md)), for the user. Planned to end with: the catalogue of the library, each entry run by the kernel; three steps with their prompts; step 1 writing the facts, the answer query and a partial ontology; steps 2 and 3 selecting before they synthesise, and stating the hypothesis mechanism; the program assembled in synthesis order, each step's program checked and judged at root; [Q9](../open_questions/formalizer/Q9-ontology-provenance.md) decided |
| [S2.2.5](s2.2.5_from_text.md) | From a text to a short answer | **ran to its end** 2026-11-26 ([its record](s2.2.5_from_text.md#record), [the report](from_text/README.md), [the page](../../../docs/einaf/08-from-text.md)): `einaf from-text` (`einaf::from_text`), the steps and the least questions the answer needs; `einaf::answer`, the answer chosen by the answer query's classes and seven fixed texts; `einaf::format`, the formatters (the decimal numeral, the word theory's quantities by cases that are data), built and tested alone; every call recorded slim, no request body, and replayed; the Zebra puzzle **not met**: dead at step 2 (eleven houses for five places and four kinds with no member, either fault alone deadly: L-M2.23), *The statements contradict each other.*, its answer query one variable for two questions; the word problems, one shot: 11 of 13 load and 3 read right (each the undetermined text) with the library alone, 8 and 3 with the word theory's modules given (`bystander`'s *Carol has 9 apples.* the one right number, no module imported); E-126, E-127; Q-M2.33, Q-M2.34 for the user. Planned to end with: `einaf from-text`, one shot in three steps, the kernel run through EinAf's bounded call; the answer formatters, written to be [M1p](../../m1p_ein_to_natural_language/README.md)'s starting point; the fixed texts EinAf answers with where no program prints an answer; the record per run, each step's program and judgement in it; the Zebra puzzle and the word problems from their texts to their answers |

### Order

**S2.2.0a comes first** (the user, 2026-11-25: *"another stage even before - s2.2.0a"*): the target shape it
settles, the four `zebra3.n.ein` and its check of the synthesis order are what the stages after it read. **The user's notes on
the steps govern every stage** (the user, 2026-11-26, above); they were re-planned to them when S2.2.0f was
dissolved, and each says in its own words what of them it carries.

S2.2.1 and S2.2.2 do not wait for each other, but for one item: S2.2.2's step 1 with the model (T2.2.2.8) is set
against the hand-written step 1 of S2.2.1's T2.2.1.8. **S2.2.2's probe is the phase's first measurement**, and its
first item is the user's first experiment: it needs the package
([S2.1.1](../../../docs/einaf/README.md#the-package)), the kernel's diagnostics as values
([S2.1.2](../../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)), the model and the target shape, and
nothing of the contract but the shape its examples show. It runs before a grammar is built, because its result
says what a grammar would have prevented. S2.2.3 follows the probe; S2.2.4 follows S2.2.1 and S2.2.3; S2.2.5
closes the phase.

**How every stage reaches the kernel.** One module of EinAf calls the engine's crates, `einaf::kernel`
([S2.1.3](../../../docs/einaf/README.md#what-it-depends-on)'s acceptance), and a program is run only
through its bounded call, which sets the kernel's own clock and limit on every run (that stage's
T2.1.3.2b; [the frame's default 16](../README.md#the-defaults-the-re-plan-took)): nothing in a solve stops a saturation
that has no fixpoint, and from S2.2.2's probe on the programs are a model's. Where a file of this phase names a
function of the crates (`ein_ir::parse`, `ein_ir::dump_canonical`, `ein_core::render_why`), it names what that
module wraps, and a stage that needs one the module does not give adds it there.

**What two stages of P2.5 give first** ([P2.5 § Order](../p2.5_harness/README.md#order)): the record's layout
([S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md)) once `einaf/` exists, and the answer's comparison (the
first part of [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)) before this phase's numbers are taken.

## Acceptance

- `einaf from-text` on [`nl2smt/puzzles/zebra.txt`](../../../nl2smt/puzzles/zebra.txt) yields a program the kernel
  answers with the outcome *one*, whose answer names the Norwegian for the water and the Japanese for the zebra;
  one shot, the theory selected from the library, with the record.
- On the sixteen word problems of [`examples/word_problems/`](../../../examples/word_problems/README.md), one
  shot, the model shown the library alone: per problem, whether the output parsed, whether it loaded, its outcome
  among the seven, and the answer it printed beside the answer that directory's README states. Numbers from a
  record, not a demonstration.
- A right program is scored right: the phase's reference programs print their answers in the answer line's form
  and match the gold short answer exactly, and the mechanical judge finds no event in any.
- Every form of every program the phase recorded is in exactly one row of the contract's table of categories, by
  reading the program and the record alone.
- **The steps by hand**: the Zebra puzzle from its text to `zebra3.1.ein`, the target shape S2.2.0a settled
  through the three steps, and two or more other texts the same way, each step's program judged by a test of
  EinAf's gate step ([S2.2.1](s2.2.1_contract.md), T2.2.1.8).
- **The user's first experiment has a record**: the Zebra puzzle with a working Zebra program as its example,
  and step 1 alone on the same texts set against the hand-written step 1 ([S2.2.2](s2.2.2_model_runtime.md)).
- Every one-shot record holds each step's program and its judgement at root beside the complete program's.
- A record replays: the same fields give the same program, or the record names the field that moved.
- A program whose saturation has no fixpoint comes back *not decided* within the bound of EinAf's kernel call,
  from the probe on.
- No prompt holds a benchmark instance's solution: every corpus program a prompt quotes is listed in the record,
  and the list is handed to [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md) as out of every split. The first
  experiment's Zebra program is on the list; the Zebra instance is in no split
  ([S2.2.5](s2.2.5_from_text.md), T2.2.5.4).
- The probe's record exists, and [Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md) and
  [Q9](../open_questions/formalizer/Q9-ontology-provenance.md) are decided in the ledger, each with the measurement it
  rests on.
- [The evaluation of Ein](../evaluation/README.md) holds what each stage found of the kernel, and is shown at the
  phase's close.

## Risks

- **A model of 4.2B parameters may not select a theory in one shot.** The Zebra acceptance asks for `std.slots`
  spelled right, an activator of five arguments among its six, and eighteen facts, from a model that was never
  trained on Ein. If it is not met, the phase reports the nearest miss step by step and the one-shot number stands as
  baseline B2: a low B2 is a finding, the loop ([P2.4](../p2.4_loop/README.md)) is where the model gets a second
  chance, and whether the gate's first criterion is held as written is the user's at the Level B gate.
- **The failures that matter are silent.** An activator under an unlisted name, a program with no mechanism, a
  fact of an undeclared relation and a form at another count all load and answer something
  ([the audit, runs s04, s11, s15](../analysis/audit_p2.2.md#snippet-runs-the-table-sections-a-to-c-cite)). A
  grammar makes none of them impossible but the vocabulary ones. The contract's check
  ([S2.2.4](s2.2.4_passes.md)) is EinAf's own and is what tells them apart from a formalization that worked.
- **Literal facts move work into the theory**, and a theory of the program's own is the expensive move
  ([F12](../../followups/f12_rules_and_relations/ideas.md)): the steps may trade an easy step 1 for a hard step 3.
  [S2.2.0a](s2.2.0a_zebra_formulations.md)'s table and the hand-written steps of [S2.2.1](s2.2.1_contract.md)'s
  T2.2.1.8 measure the trade; no stage argues it.
- **A wrong coreference in step 1 is costly to undo** when the facts are the part that changes least. The one shot
  does not change one ([S2.2.4](s2.2.4_passes.md)); the loop has to be able to, at a cost it records
  ([P2.4](../p2.4_loop/README.md#the-theory-before-the-facts)).
- **A closed-world reading is one `absent` away from a flagged program.** *The youngest is the one who is older
  than nobody* is flagged where a commitment can add an `older` fact and is monotone under `(solve :no-search)`
  ([S2.2.1](s2.2.1_contract.md) has both runs). A formalizer will write such programs; a flagged program is an
  outcome of its own and never certified
  ([Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)).
- **A model's program may never finish.** A rule that builds a fact out of the fact it matched has no fixpoint,
  and neither the clock nor the facts budget of a solve stops it ([S2.2.5](s2.2.5_from_text.md) has the run). Every
  stage that runs a generated program does so through EinAf's bounded kernel call, and such a run is *not
  decided*.
- **The binding may not load the model.** The first check of [S2.2.2](s2.2.2_model_runtime.md) is that the
  llama.cpp it gets knows the architecture `qwen35`; the fallback is `llama-server` of the host's package, which
  does ([`host.md`](../analysis/host.md)).
- **A grammar and a thinking model do not compose as they stand** (the host probe): the arms that use a grammar
  run with thinking off, and a grammar that begins after the thinking is [S2.2.3](s2.2.3_gbnf.md)'s design.
- **What the model is given has to fit its context.** The reference's syntax page alone is over eleven thousand
  words ([the audit, A.4 row 30](../analysis/audit_p2.2.md#a4-paths-pages-other-plans)); the excerpt, the
  catalogue and the examples are cut to a context the device holds, and the context is a field of the record.

## Connections

- [`EinAf.md` § Stage B](../EinAf.md#stage-b--define-autoformalization-as-an-explicit-task); the frame's
  [arms](../README.md#the-arms), [outcomes](../README.md#the-outcomes) and
  [paths](../README.md#where-einafs-things-are).
- [The audit of this phase](../analysis/audit_p2.2.md), [the host](../analysis/host.md), and
  [the analysis, § 3, P2.2](../analysis/README.md#p22--the-formalizer-audit).
- [Idea 04](../../ideas/04-nlp-to-graph-to-solver-pipeline.md), the pipeline sketch
  ([§ Sketch of the pipeline](../../ideas/04-nlp-to-graph-to-solver-pipeline.md#sketch-of-the-pipeline));
  [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md)'s three neural actions, of which step 1 is the first,
  *reinterpretation*; [F16](../../followups/f16_autoformalization/ideas.md): *autoformalization is not
  translation*, and step 1 is the part that is;
  [idea 01](../../ideas/01-self-modifying-constraint-language.md), GBNF as the syntactic firewall;
  [`docs/lib/01`](../../../docs/lib/01-llm-constrained-generation.md).
- [F16](../../followups/f16_autoformalization/ideas.md), [F12](../../followups/f12_rules_and_relations/ideas.md),
  [F7 B](../../followups/f7_rule_induction.md#connection-to-m2), [F4 Q38](../../followups/f4_cross_cutting.md):
  the model as extractor of facts, relations and rules.
- The kernel's pages these stages read: [`02-syntax.md`](../../../docs/ein/user/04-reference/02-syntax.md), the
  forms; [`04-stdlib/`](../../../docs/ein/user/04-reference/04-stdlib/README.md) and
  [`stdlib/`](../../../stdlib/README.md), the library;
  [`14-natural-language.md`](../../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md), the four
  texts; [`04-api.md`](../../../docs/ein/ein.rs/04-api.md), what EinAf links.
- [M1p](../../m1p_ein_to_natural_language/README.md), which does not run and takes S2.2.5's formatters as its
  starting point.

## Defaults taken at the re-plan

Each is this re-plan's, recorded so that the user can correct it; none is a ruling.

1. **The model of the target shape is `examples/zebra3.1.ein`**: the statements the text's verbs in their voice, a
   description the word it is built on, the questions in the statements' relations, the theory selected from the
   library with no rule of its own, H by the library's obligations, the program in synthesis order. The Zebra
   acceptance accepts any formulation, since it is stated on the answer. The re-plan's default was `zebra.ein` less
   its rule, its activators and its config; **reopened by the user** (2026-11-25), and **amended by
   [S2.2.0a](s2.2.0a_zebra_formulations.md)** (2026-11-26) from [its table](zebra_formulations/README.md), as a
   default for the user: of the six encodings, `zebra3.1` is the one whose facts are the text's words (59 of the
   sentences' 83 words kept, against `zebra`'s 39 and `zebra2`'s 47) with no rule of its own (`zebra3.2`, closer to
   the text, 75, writes six; `zebra2`, the cheapest to run, fifteen), monotone and `Solution` under every question,
   whose cores name the sentences that contradict ([E-109](../evaluation/natural-language/E-109-a-sentence-written-as-a-rule-s-activating-fact-is-in-no.md));
   it costs the kernel twice `zebra`'s facts and fourteen times `zebra2`'s
   ([E-111](../evaluation/bounds/E-111-the-answer-in-the-statements-own-words-costs-the-kernel.md)), and its
   thirty-two selecting facts are thirty-two spellings a model can get wrong in silence
   ([E-107](../evaluation/diagnostics/E-107-a-theory-s-missing-activating-fact-is-reported-as-the.md),
   [E-110](../evaluation/stdlib/E-110-the-library-has-no-reading-of-a-literal-vocabulary.md)). Which axis weighs most
   is the user's.
2. **The hypothesis mechanism is a part of the contract of its own**, stated in step 3. A program that
   states none is recorded as the kernel reports it, `blind`, and is not refused.
3. **A category is computed, not written**: by EinAf, from the program, the input text and the catalogue, and kept
   in the record keyed by the form's canonical text. A fact outside the library's with no `:source` is a
   generated assumption; the forms B2 has no word for are rows of the contract's table.
4. **The answer query is the one query that carries a `:goal-text`**, and any other query is a check: the
   benchmark and the metrics read one ([P2.3](../p2.3_benchmark/README.md#an-instance),
   [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)). Its `:goal-text` is a sentence that says the answer, as the user
   ruled ([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)). A program with none has no answer; with two, the first is read and the check reports it. The model
   writes every `:why`, and EinAf never loads with the `:why` requirement off.
5. **Texts are English** (the frame's default 9): the Russian half of the old plan is dropped.
6. **S2.2.1 and S2.2.2 are independent**, but for step 1 with the model, which is set against S2.2.1's hand-written
   steps; and the probe, the user's first experiment its first item, is the phase's first measurement.
7. **This phase's own runs are greedy** (temperature 0), so that one run is the whole distribution. Which
   sampler chain the arms run with is [S2.5.2](../p2.5_harness/s2.5.2_baselines.md)'s, whose default is the
   model's own; the one shot runs under it unchanged.
8. **The reference run of `llama-server` is by hand, from a recorded request**; a Rust client of it is written
   only if the fallback is taken.
9. **The probe's items** are, first, the user's first experiment (the Zebra puzzle with a working Zebra program
   as its example, default 31); then a few small texts and the sixteen word problems over the library alone,
   which is the main arms' condition, and the sixteen again with the word theory given, which is the library
   ablation's widest and is labelled so; and step 1 alone (default 32). No item but the first is shown an example
   of its own text family; thinking off and on; the output read whole and by its first fenced block.
10. **The grammar is held to the parser in both directions** offline, and to llama.cpp's own sampler over the
    corpus by hand; the old echo round-trip through the model is dropped. No speed target is inherited.
11. **A grammar begins at the first token, thinking off**, in the arms; the start after the thinking is built for
    [P2.6](../p2.6_ablations/README.md)'s ablation.
12. **The catalogue is S2.2.4's**, each entry a program the kernel runs, and **holds the library's modules
    alone**: a family's own module is its canonical program's, is not shown to the model in the main arms, and
    is offered only under the widest condition of the library ablation
    ([S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md)).
13. **The steps are the formalizer; the whole program in one call is kept as a mode**, and both are recorded.
14. **EinAf rewrites nothing the model wrote**: a number is a numeral because the model wrote one.
15. **`einaf from-text` always asks the kernel**; the old *direct* mode is `--out`.
16. **With no formatter applied, EinAf's line is the kernel's, byte for byte**: that equality, over every
    corpus program with a `:goal-text`, is the formatters' first test.
17. **This phase's one-shot numbers are over what exists when it runs**: Zebra and the sixteen word problems,
    the seed set's second domain. The anchors of the eight families are
    [S2.3.1](../p2.3_benchmark/s2.3.1_families_and_seed_set.md)'s, after this phase in work order, and the
    one-shot run over the benchmark is the Level B run. The old `--gold` structural diff is dropped: an answer is
    compared as a short text, by [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)'s reader.
18. **The Zebra text is read where it is**, `nl2smt/puzzles/zebra.txt`; none is checked in under `examples/`.
19. **The reading list is the process's**, a file of the plan folder, not a page of `docs/einaf/`.
20. **Every run with the model writes a record in [S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md)'s
    layout**, the probe's among them: that stage runs once `einaf/` exists. The sidecar (the categories and the
    check's findings) is a file beside each program, which the layout has no line for yet.
21. **The model's runtime is behind a Cargo feature of `einaf`, off by default.** EinAf's gate step builds and
    tests without it, on a runner with no CUDA and no model file, by replaying recorded generations; an
    experiment builds with it.
22. **The fixed texts are worded in [S2.2.5](s2.2.5_from_text.md)** (the frame's default 15 leaves the words to
    it): two that are answers, *The statements do not determine the answer.* and *The statements contradict each
    other.*, and four lines that are not, for *incomplete*, *not decided*, *flagged* and *invalid*. Which text an
    outcome prints follows S2.5.3's answer rule, which reads the answer query's classes and not the kernel's
    word alone.
23. **A formatted value enters the answer by filling the query's template again**, never by editing the line
    the kernel filled: a value's text is not unique inside a line.
24. **The contract's check is built once, in [S2.2.4](s2.2.4_passes.md)**, and runs on each step's program with
    the findings that apply to it; the feedback ladder's lints
    ([S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md)) are its findings given a sentence each.
25. **The contract's table says which category is a faithfulness event**
    ([S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)): only a fact stated as holding with no sentence behind it, a
    membership no sentence states among them, which
    [Q-M2.22](../open_questions/answer/Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md) asks the
    user about, since the steps make memberships ontology.
    What a program needs to be well formed or to state its lattice (a declaration, an import, a `(solve …)`
    form, a closure, a written `(open F)`) is never one.
26. **This stage writes the answer lines its own runs need** (each asks for a sentence: how many, for the word
    problems; who drinks water and who owns the zebra, for Zebra), since
    [S2.5.2](../p2.5_harness/s2.5.2_baselines.md), which owns them, comes after it.
27. **The probes of this re-plan are in the files' own text** (the contract's worked example in three forms, the
    five forms with no mechanism, the rule with no fixpoint, a word problem over the calculator): this re-plan wrote only into its own folder, so
    none is banked under [`analysis/probes/`](../analysis/README.md) yet.

Taken at the re-plan of 2026-11-26, which dissolved S2.2.0f into the notes above on the user's word and re-planned
the stages to the steps:

28. **The five design questions of the steps take the defaults written beside them**
    ([above](#five-design-questions-each-with-its-default)); the old stage said *four* and listed five, the third
    (the program's text in synthesis order) added with S2.2.0a's T2.2.0a.3.
29. **The steps are stated in the contract, by [S2.2.1](s2.2.1_contract.md)'s T2.2.1.7**, which was T2.2.0f.1:
    the steps are what the formalizer emits and in what order, so they are the contract's content, written straight
    into `docs/einaf/contract.md` with no plan page between.
30. **The steps by hand are [S2.2.1](s2.2.1_contract.md)'s T2.2.1.8**, which was T2.2.0f.2 and T2.2.0f.3: the
    Zebra puzzle to `zebra3.1.ein` and two or three other texts, as the contract's worked examples and as files of
    `einaf/` a test judges, not probes under `analysis/`. They need no model and are read by four stages after
    them (S2.2.2's step 1, S2.2.3's step grammars, S2.2.4's replay tests, S2.2.5's Zebra reference), so the first
    stage with no model holds them; S2.2.4, where the prompts are built, was the other candidate, and comes after
    the probe that reads them.
31. **The user's first experiment is the first item of [S2.2.2](s2.2.2_model_runtime.md)'s probe** (T2.2.2.6):
    the Zebra text, and as the prompt's one example the target shape S2.2.0a settles, run by the kernel, beside its
    text; the excerpt of Ein's syntax and semantics in the prompt as in every prompt; the whole program in one call,
    thinking off and on. The family rule yields to it by the user's word; it is labelled as the item shown its own
    program, a floor, and none of its numbers enters a table.
32. **Step 1 with the model is [S2.2.2](s2.2.2_model_runtime.md)'s T2.2.2.8**, which was T2.2.0f.4: through the
    stage's runtime rather than `llama-server` by hand (the fallback is `llama-server` in any case), after the
    first experiment, and set against T2.2.1.8's hand-written step 1.
33. **What partial programs show of the kernel**, T2.2.0f.6, goes to each stage's own task of what it found of Ein
    (T2.2.1.6, T2.2.2.7, T2.2.4.10, T2.2.5.6).
34. **[S2.2.4](s2.2.4_passes.md) keeps its file name**, `s2.2.4_passes.md`, with the steps as its title: a step is
    one call of the model, or two where step 2 synthesises, and a rename would move every link to the file for no
    reader's gain.
35. **Step 1 is one call over the whole text**: which words name the same object is decided across sentences. The
    first re-plan's instance pass, a call a sentence with the sentence marked, is kept as a mode of step 1.
36. **Select before synthesise, inside steps 2 and 3**: step 2's first call selects, with `rule` forms off, and a
    second call with them on runs only on the marker of a missing property; step 3 writes the rules still missing,
    each after its marker, and the hypothesis mechanism. Where step 2's rules end and step 3's begin is what the
    hand-written steps show, and the prompts follow them.
37. **Step 2 writes the complete ontology whole**, and it stands in the assembled program in place of step 1's
    partial one; what it changed is recorded. No later step's output holds a step-1 statement: a fact a later step
    states for a sentence step 1 already stated otherwise is reported by the check and counted, and changing a
    step-1 fact is the loop's.
38. **A grammar a step** is [Q-M2.6](../open_questions/formalizer/Q-M2.6-one-gbnf-or-many.md)'s working answer as
    [S2.2.3](s2.2.3_gbnf.md) takes it. Step 1 writes its facts before their declarations, so no grammar closes its
    vocabulary and the contract's check holds a step-1 fact to a declaration; steps 2 and 3 are rendered on step 1's
    relations and names.
39. **`einaf from-text` runs the steps by default** (`--mode steps|whole`), and judges each step's program at root
    through the bounded call; the model is not shown that judgement before the next step (P2.6's arm).
40. **The step a form came from is a column of the sidecar**, beside its category, and the categories are given by
    step in the contract ([S2.2.1](s2.2.1_contract.md), T2.2.1.2).
