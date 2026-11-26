# P2.5 — Baselines, metrics, records (Stages D, H, N)

**Depends on:** P2.1: the feedback object
([S2.1.3](../../../docs/einaf/03-feedback.md)) is the kernel's side of every record;
[P2.2](../p2.2_formalizer/README.md): the model's envelope
([S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md)) is the model's side, and the one-shot is arm B2;
[P2.3](../p2.3_benchmark/README.md): the splits, and each instance's gold class, short answer and `:expect` claim;
[P2.4](../p2.4_loop/README.md): B3 to B5 are the filter at F0, F2 and F8, and B6 is the tool
([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)). The first stage runs before the phase it belongs to: § Order.
**Blocks:** [P2.6](../p2.6_ablations/README.md) and [P2.7](../p2.7_failure_scaling_generalization/README.md):
both are more rows of the table this phase defines, over the same records.
**Feeds:** [the evaluation of Ein](../evaluation/README.md): [S2.5.3](s2.5.3_metrics.md) is where the kernel's cost
and limits get numbers.
**Research plan:** [`EinAf.md` § Stage D](../EinAf.md#stage-d--establish-baselines),
[§ Stage H](../EinAf.md#stage-h--define-rigorous-metrics),
[§ Stage N](../EinAf.md#stage-n--reproducibility).

---

## Three stages of the plan, one phase, and why

Stages D, H and N are the experimental method: what is compared, what is measured, what is kept. They are one
phase because none of the three is usable without the other two (an arm without a metric is a demo, a metric
without a record is a number nobody can take again), and because all three are **defined before the first table
and not changed after it**. The research plan puts reproducibility at N, near its end. This milestone puts the
record first, by the discipline the project already measures under: a time is read only with the machine state it
was taken in ([`utils/bench_env.sh`](../../../utils/bench_env.sh) prints it, the power source among it), and the
corpus's `cost_ms` is a measured claim checked in both directions.

**What the phase is for is the user's core idea as a measurement**
([README § The two main ideas](../README.md#the-two-main-ideas-the-user-2026-11-25)): the same model reasoning in
text and reasoning through Ein, at a similar budget. [S2.5.4](s2.5.4_first_table.md)'s table is the first
measurement of a chain of formal reasoning against a chain of thought.

### Order

- [S2.5.1](s2.5.1_experiment_record.md), the record, runs once `einaf/` exists
  ([S2.1.1](../../../docs/einaf/README.md)) and is in use from P2.2's first run on
  the seed set, so that the one-shot numbers are already evidence.
- [S2.5.3](s2.5.3_metrics.md), the metrics, runs in two parts. H1, the rates of H2, H4 and the readings of
  *certified* are written before P2.2's acceptance numbers are taken, so that the one-shot rates are already in
  the final vocabulary. Faithfulness, H3 and the tool's episode metrics are written when
  [P2.4](../p2.4_loop/README.md) has defined the transcript and the faithfulness events they are folds over.
- [S2.5.2](s2.5.2_baselines.md), the arms, needs both arrangements and waits for P2.4.
- [S2.5.4](s2.5.4_first_table.md) is the phase's close.

### The record (N)

One run is one **immutable directory** under `einaf/experiments/`
([README § Where EinAf's things are](../README.md#where-einafs-things-are)): what was run (`config.json`), the
machine it ran on, the instances with the split's digest, every call of the model with its envelope, every program
the model wrote, every question asked of the kernel with the feedback object that answered it, the transcript of
every episode, every answer as it was given, and the metrics derived from those. Every table and plot of the
milestone is generated **from records only**: a number that cannot be traced to a record is in no table, because
the reporter has no other input.

What a record says of the kernel changed with the kernel
([`audit_p2.5_p2.6.md`](../analysis/audit_p2.5_p2.6.md) § B.1): the commit is git's, since neither the binary nor
the library states one; the library is named by its manifest's SHA and by which of its three places answered; the
resolved config is two flags; the work is `saturations`, `facts` and `layers`, and it depends on **which question
was asked**, which is therefore a field of every kernel line; and no surface of the kernel carries a time or a
memory figure, so EinAf takes both itself. The model's side is the envelope of
[S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md): the model's file by its SHA, the binding and its llama.cpp,
the sampler chain, the seed, the chat template, thinking on or off. [S2.5.1](s2.5.1_experiment_record.md).

### The arms (D)

[The frame's table](../README.md#the-arms), B0 to B6, with what each isolates:

| arm | what the model does | Ein | what it isolates |
|---|---|---|---|
| **B0** | answers, thinking off | none | what the model knows |
| **B1** | reasons in text, a chain of thought, and answers | none | what reasoning in text buys: **the side the others are compared with** |
| **B2** | writes one Ein program | the filter, one shot | formalization alone |
| **B3** | writes a program, and again while the kernel does not pass it, told only *try again* (F0) | the filter | more inference, no information |
| **B4** | the same, told the verdict (F2) | the filter | one word of information |
| **B5** | the same, told everything the kernel found (F8) | the filter | everything the kernel can say |
| **B6** | reasons in text with Ein offered as a function it may call | the tool | Ein when the model chooses it |

Three comparisons carry the table, and each is read by a rule written before the run:

- **B5 against B1, and B6 against B1**: a chain of formal reasoning against a chain of thought, the filter's and
  the tool's.
- **B5 against B6**: Ein forced against Ein offered.
- **B3, B4, B5**: what the kernel is allowed to say, with everything else held. The filter's strategy is *repair*
  in all three; *regenerate* and the alternatives are rows of [P2.6](../p2.6_ablations/README.md).

The comparison is never *the model against Ein*: B5 spends more than B0. It is **one model, one split, a similar
budget of generated tokens**, thinking included, with the kernel's work recorded beside it and not traded against
it ([README § The defaults](../README.md#the-defaults-the-re-plan-took), 6). [S2.5.2](s2.5.2_baselines.md) makes
*similar* a number with a tolerance. Where the research plan says *where possible add another symbolic backend*,
that is [P2.8](../p2.8_representations/README.md), later and on its own; *another structured reasoning approach*
is B6.

### The metrics (H)

Four layers, defined once, computed from records by one module ([S2.5.3](s2.5.3_metrics.md)):

| layer | metrics | on which |
|---|---|---|
| **H1** final task | answer accuracy on **the answer as a sentence**, scored by what it states, for every arm alike, with exact match and containment beside it; accuracy on the instances with one answer; ambiguity detection; contradiction detection; for the arms that run Ein, the gold `:expect` claim checked by the kernel, and the seven outcomes against the gold class | all instances |
| **H2** formalization | parse success; load success; acceptance; the flagged rate, in a column of its own; faithfulness; constraint recall and precision | the arms that run Ein; the last two against the canonical program, exact on synthetic instances |
| **H3** repair | fraction repaired; iterations to repair; new errors introduced; hallucination rate; repair-cycle rate; for the tool, calls an episode and whether the answer is what the last run printed | every episode, from its transcript |
| **H4** system | calls of the model; tokens, prompt and generated, thinking apart; wall; the kernel's questions, saturations and facts; the kernel's own wall and peak memory; **cost per correctly solved instance** | every run |

And the two numbers the milestone turns on:

```text
P(answer correct         | Ein certifies)     is certification a reliability signal?
P(formalization faithful | Ein certifies)     or a formally valid lie?
```

**What *certifies* means is open, and the user's**
([Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)). Until it is decided
no table fixes it. Every record carries **each reading** a final program has (the word `--single` says over all
of R; the word it says over the maximal line; a description's two counts, `k_s` and `k_c`, with whether the run
exhausted), beside the instance's family and shape, and both probabilities are computed under each. A flagged
program ([Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)) is certified under none
and has a column of its own. The gap between the two probabilities is the research plan's distinction between
faithfulness and satisfiability as a number, and every table that shows one shows both.

**H4's kernel half is the second main idea's**: what Ein cost and where it stopped in this application are
numbers of [the evaluation of Ein](../evaluation/README.md), taken by [S2.5.3](s2.5.3_metrics.md) and shown at the
phase's close.

## What P2.2 hands this phase

*Written 2026-11-27, at [P2.2's close](../p2.2_formalizer/README.md#the-close-2026-11-27).* The files of this phase
were planned before P2.2 ran. What that
phase found that bears on them is in each stage's file, under *What P2.2 hands this stage*; this is the map. None of
it is a ruling: where an item rests on a question, the question's default stands.

**Two stages of this phase were to run before P2.2's numbers and did not** ([§ Order](#order)): the record, and
the first part of the metrics. P2.2 took its numbers with a stand-in for each, and says so. By [the order
before P2.3](../README.md#the-order-before-p23) both come before [P2.2d](../p2.2d_formalizer_corrected/README.md)'s
run of the plan's 29 again, and before the benchmark's check of an
instance's printed answer.

| stage | what it reads |
|---|---|
| [S2.5.1](s2.5.1_experiment_record.md) | P2.2's stand-in layout and its four schemas; the slim call; `kernel.commit` ([L-M2.27](../findings/einaf/L-M2.27-a-record-s-kernel-commit-is-null.md)) and a refused program's categories ([L-M2.26](../findings/einaf/L-M2.26-a-program-the-kernel-refuses-carries-no-categories.md)); how a generation ended ([S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md)); what is committed |
| [S2.5.2](s2.5.2_baselines.md) | B1 taken early on five texts ([S2.2c.2](../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md)); the chain and a grammar under it ([Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md), [Q-M2.28](../open_questions/model/Q-M2.28-is-a-token-held-to-its-grammar-by-the-chain-s-choice-first.md)); B2's mode and cut ([Q-M2.32](../open_questions/formalizer/Q-M2.32-which-mode-does-baseline-b2-run-with.md), [Q-M2.36](../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md)); a puzzle the model knows by name ([L-M2.29](../findings/model/L-M2.29-the-model-alone-solves-the-zebra-puzzle-in-its-thinking-and-names-it.md)) |
| [S2.5.3](s2.5.3_metrics.md) | the stand-in reader; several rows in one class ([Q-M2.33](../open_questions/answer/Q-M2.33-is-a-class-with-several-rows-an-answer-of-several-values-or-the-undetermined-text.md), [E-124](../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md)); the second count ([L-M2.22](../findings/model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md), [S2.2d.1](../p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md)); a score a question ([Q-M2.35](../open_questions/formalizer/Q-M2.35-is-there-an-answer-query-for-each-question-of-the-text.md)); which text a dead root prints ([E-118](../evaluation/stdlib/E-118-a-numeral-written-with-two-digits-in-one-place-is-refuted-as-a-contradiction.md), [E-107](../evaluation/diagnostics/E-107-a-theory-s-missing-activating-fact-is-reported-as-the.md)) |

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.5.1](s2.5.1_experiment_record.md) | The experiment record and the canonical workflow | the record's layout and `einaf-record/1`; `einaf run`, `einaf evaluate`, `einaf report`; records immutable, tables generated from them; the kernel and the model each named so that a number says what it came from |
| [S2.5.2](s2.5.2_baselines.md) | The arms B0 to B6 and the budget match | seven arms as seven derived configs; the budget as a number with a tolerance, measured on `dev`; one reader of the answer for every arm, with its own error rate measured |
| [S2.5.3](s2.5.3_metrics.md) | The four metric layers | every metric defined once, computed from records, with a fixture whose value is known by hand; both conditional probabilities under each reading of *certified*; the kernel's cost and limits as numbers |
| [S2.5.4](s2.5.4_first_table.md) | The first main table | B0 to B6 on `val`, per family and aggregate, H1 to H4: the Level C gate's first row, and the first measurement of a chain of formal reasoning against a chain of thought |

## Acceptance

- A record exists for every number in every table of the milestone, and `einaf report` regenerates the tables from
  the records with no other input.
- The seven arms run on the `val` split under the matching rule, and the budget each spent is a column of the
  table beside the budget it was given, with the kernel's work in columns of its own.
- An answer is scored as a short text by one rule for every arm; the rule's agreement with a hand-read sample is
  measured and stated with the table.
- Each metric has a fixture worked by hand; both conditional probabilities are reported with their denominators,
  under each reading of *certified*, and a flagged program is in neither numerator.
- The first main table is committed with its records, and its pre-registration says what the table is evidence
  for and what it is not, before any ablation runs.
- EinAf's own gate step holds what needs no model: the record's tests, the metrics' fixtures, the report's
  regeneration. A run that needs the model's file and the GPU is an experiment, by hand, in no tier.
- What the phase found of Ein is in [the evaluation of Ein](../evaluation/README.md), and is shown at the close.

## Risks

- **Reading an answer is a metric of its own.** A free text compared with a gold short answer favours whichever
  side is stricter, and in the filter the answer's sentence is one the model wrote as its `:goal-text`.
  [S2.5.2](s2.5.2_baselines.md) measures the reader against a hand-read sample across every arm, and
  [S2.5.3](s2.5.3_metrics.md) holds it to the kernel's own check of the gold claim where a program exists.
- **Budget matching is a judgement.** Tokens are not calls and calls are not wall. The primary is generated
  tokens; prompt tokens, calls, wall and the kernel's work are reported beside it; *similar* is a number with a
  tolerance, stated before the table.
- **A reading of *certified* chosen after the table is a result chosen after the table.** The table shows every
  reading side by side and selects none; the user selects, on that table.
- **The first table is not the result.** It is `val`, and the table names the split it is on. `test` has two
  sanctioned openings, the Level B run and [P2.10](../p2.10_result_artifact_demo/README.md)'s
  ([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)); nothing of this phase is one of them.
- **A kernel bug met in a run is a finding, not noise.** A wrong answer goes to
  [the wrong-answer register](../../wrong_answers/README.md) with its witness and to the evaluation; the record
  that met it is kept, and the table says which cells it touched.

## Connections

- [`EinAf.md` § Stage D](../EinAf.md#stage-d--establish-baselines),
  [§ Stage H](../EinAf.md#stage-h--define-rigorous-metrics),
  [§ Stage N](../EinAf.md#stage-n--reproducibility).
- [The frame](../README.md#the-plan-as-re-planned): the arms, the seven outcomes, where EinAf's things are; and
  [the ledger](../open_questions/README.md): Q-M2.10, Q-M2.11, Q-M2.12, Q-M2.13, Q-M2.14.
- [`analysis/audit_p2.5_p2.6.md`](../analysis/audit_p2.5_p2.6.md): what these stages said of the kernel against
  what the tree says, claim by claim; a stage looks there for the evidence of what its file says changed.
- [`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md) and
  [`10-answer-dump.md` § The run summary](../../../docs/ein/user/04-reference/10-answer-dump.md#the-run-summary):
  every key the kernel's answer has and what it counts;
  [`04-api.md`](../../../docs/ein/ein.rs/04-api.md): the same answer as the values EinAf links.
- [`utils/bench_env.sh`](../../../utils/bench_env.sh): the machine state a time of this project is read with.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md): the four oracle ablations (*NL or IR, known or unknown
  theory*), which are four more arms in S2.5.2's vocabulary and [P2.6](../p2.6_ablations/README.md)'s.
- [F26](../../followups/f26_consistency_and_completeness.md): the finer classes behind the benchmark's three,
  which the tables carry as metadata; [F22](../../followups/f22_exact_core_past_the_cap.md): the exact core,
  which the level that renders a core would read.
- [M5](../../m5_presentation/README.md): § 6 of the paper, *Experimental method*, is this phase written up.

## Defaults taken at the re-plan

Each is this re-plan's, recorded so that it can be corrected; none is a ruling.

1. **The texts a record used are kept by hash beside the records** (`einaf/experiments/store/`): prompts,
   grammars, the chat template. `einaf/prompts/` and `einaf/grammars/` hold the working copies.
2. **A record has taken files and derived files.** `einaf run` writes the taken ones once. The derived ones
   (`metrics.json`; the readings and the gold claim's verdict; the kernel's cost) are functions of the record and
   of the kernel at the record's commit, each rewritten only by the command that owns it, which writes its own
   commit into it.
3. **The machine state is EinAf's own capture**, the fields `utils/bench_env.sh` prints and the GPU's, since
   that script has no line for a GPU and EinAf does not read the process.
4. **The arms are values of the package, not files of their own**: an arm's name is derived from its fields and
   checked. The frame names no folder for them.
5. **B1 is the stronger on `dev` of the model's own thinking mode and an instructed chain of thought**, fixed
   before `val`. B1 runs to the matched budget by *reconsider* turns, and its answer after every turn is kept, so
   the same episode holds B1 at its first answer.
6. **B6 thinks as B1 does, knows of Ein what B5's prompt says, and is told at F8 what B5 is told**, so that B6
   differs from B1 by the tool and the knowledge of it, and from B5 by who decides that Ein runs.
   [S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md) owns the episode, and [P2.4](../p2.4_loop/README.md) took the
   same default.
7. **The budget is two numbers, both from B5 on `dev`, per family**: one cap of generated tokens an instance
   for every arm that iterates (B3, B4, B5, B6), set so that it ends few of B5's episodes; and, for B1, which
   does not stop of itself, B5's mean spend an instance within a tenth, so that the two arms' totals match. The
   old plan gave B1 and B3 B5's median and left B4 and B5 uncapped.
8. **One sampler chain for every arm, the model's own defaults**, with greedy decoding run beside it on `dev`.
   [S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md) leaves the choice to S2.5.2.
9. **The answer line is written here, one a family**, and given to every arm in the same words.
10. **The answer rule** ([S2.5.3](s2.5.3_metrics.md)): what short text is scored for an arm with a program, read
    off the outcome and the answer query's classes; a parameter of the evaluation, so another rule scores the
    same records. *Incomplete* is no answer by default. The class each reading's word says is reported in a
    column beside the rule's. P2.3 and P2.4 both leave this to S2.5.3.
11. **The score is what a sentence states** (the user's ruling on [Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md), after this list was written:
    the answer is a sentence, not a bare value). Exact match, string equality after normalisation, and
    containment stand beside it; the values a gold sentence binds and their domain are fields of the benchmark's
    instance ([S2.3.1](../p2.3_benchmark/s2.3.1_families_and_seed_set.md)).
12. **The gold claim on a model's program has three verdicts**: held, refuted, and *not comparable* where the
    program does not name what the claim names. *Not comparable* is counted apart, never as wrong.
13. **The readings of *certified* are taken of an episode's final program**, by three questions (`--single`,
    `--maximal --single`, a bare solve) under the default budget, outside the budget of generated tokens.
14. **The kernel's own wall and memory are taken by a replay**: each recorded question asked again in a process
    that holds no model, one program a process, with its work held equal to the record's.
15. **EinAf's per-commit gate step regenerates the committed table from the records**; whether it also evaluates
    every record again is decided by what that costs on the runner.
16. **The status paragraph S2.5.4 hands over carries no date**: the record ids, the shape selected, one sentence.
