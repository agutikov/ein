# M2 — EinAf: iterative autoformalization, through Level D

**Status:** **started 2026-11-25** on the user's word, *"start plans/m2_nl_to_ir from plans analysis"*, by an
analysis of this plan against the project ([`analysis/`](analysis/README.md)); the user answered its questions
the same day ([the ledger](open_questions/README.md) holds each ruling) and stated the milestone's two main ideas
([below](#the-two-main-ideas-the-user-2026-11-25)); and the plan is **re-planned** to them, 2026-11-25, on the
user's word, *"Do not start any phase. Do re-plan if ready and no open questions"*. The user then answered what
the re-plan asked (what of the kernel may change, the answer as a sentence, the minimal goal, one tree), and those
rulings are written in below: *"Yet not start stages, write down decisions and stop"*. **P2.1 is closed
2026-11-26** on the user's word, *"after p2.1 complete - close it and start p2.2, there implement s2.2.0a, and re-plan
s2.2.0f - it must be not a separate stage, it's my notes about whole formalizer phase"*, then *"don't start p2.2, just
close p2.1 and stop"*; it was started 2026-11-25 on the user's word, *"start implementation of M2, everything in p2.1,
then stop and let me review; each stage in subagents, subagents commit their work"*, and ran its four stages to their
end; its plans are deleted (`git log --diff-filter=D -- plans/m2_nl_to_ir/p2.1_kernel_as_instrumentation`), what it
left open is the milestone's, and [the defaults it took](defaults/p2.1.md) stand. **From 2026-11-26 a phase of M2
that closes keeps its folder** (the user's rule inside the milestone: *"In this milestone M2 we do not delete phase
folders and content when close phase. Only will delete the whole milestone in it's end."*); P2.1's was deleted
before it. **P2.2 is started
2026-11-26** on the user's word, *"start next phase plans/m2_nl_to_ir/p2.2_formalizer, execute s2.2.0a and pause"*; S2.2.0a ran to its end, and then, *"Ok, I like what I see, after s2.2.0a continue with other stages of the phase"*,
**the phase runs its other stages to its end and stops there for the user's review; no other phase starts until the
user says so.** S2.2.1 to S2.2.5 ran to their end (S2.2.4, stopped on the user's word while the user asked about the grammar, ran again on *"continue the phase"*), and the phase stopped for the user's review with [its report](p2.2_formalizer/report/README.md) and, on the user's questions, [an analysis against its goals](p2.2_formalizer/report/ANALYSIS.md). **P2.2 is closed 2026-11-27** on the user's word, *"close p2.2"*, after *"Ok I agree, do it"* to the routing of its results ([its close](p2.2_formalizer/README.md#the-close-2026-11-27)): what it left open is the milestone's, [its defaults](defaults/p2.2.md) stand, and its folder is kept. **[P2.2b](p2.2b_guided_thinking/README.md), guided thinking, is planned 2026-11-26** on the user's word, *"add new phase p2.2b for guided thinking"*, and **[P2.2c](p2.2c_decoding_and_model_alone/README.md), [P2.2d](p2.2d_formalizer_corrected/README.md) and [P2.2e](p2.2e_kernel_fixes/README.md) are planned 2026-11-27** at that close, for what P2.2's results left with no owner before the benchmark: a generation that ends and the model alone, the formalizer corrected, Ein's two bugs. **P2.2e is started 2026-11-27** on the user's word, *"start p2.2e"*: its stages run to its end and it stops there for the user's review. **None of the other three is started, and no phase starts until the user says so.** **[P2.2f](p2.2f_text_analysis/README.md), the text analysed by trained networks before it is formalized (AMR first, the check first), is planned 2026-11-27** on the user's word and **started the same day** (*"start p2.2f in worktree"*), in git worktrees beside P2.2e, to S2.2f.2 (S2.2f.3 waits for S2.2c.1), with [Q11](open_questions/formalizer/Q11-link-grammar-value.md) ruled the same day: no link grammar and no strictly pre-defined grammar. **S2.2f.1, the check, is done 2026-11-27**: three AMR parsers and two UD parsers run here within 4 GB of the card each and are right on 50 to 56 of its 68 cells, where the model's step 1 was right on 43 at best, so the phase goes on to S2.2f.2; coreference, word senses and IBM's document parser do not ([the check's report](p2.2f_text_analysis/check/README.md)). **S2.2f.2 is done the same day**: the analysis recorded for the six texts and given to step 1 by EinAf (`--analysis`), as text or as Ein facts; the phase stops before S2.2f.3, which waits for S2.2c.1. **[P2.2g](p2.2g_names_across_an_import/README.md), names across an import, is planned 2026-11-27** on the user's word (*"create a new phase for this question"*), for Q-M2.43, moved into it from P2.2e with the user's correction (*"imported is (m.relation m.likes m.Person m.Drink) and program's is (relation m.likes Person Drink)"*); not started.
**Depends on:** the kernel as it stands after M1f's close
([Q-M2.1](open_questions/kernel/Q-M2.1-when-is-the-kernel-frozen.md)). No other milestone runs for it:
[M1p](../m1p_ein_to_natural_language/README.md), Ein to natural language, stays unstarted
([Q-M2.12](open_questions/answer/Q-M2.12-what-prints-the-answer.md)).
**Blocks:** [M5](../m5_presentation/README.md): the paper is Stage P of the research plan and M5 is where it is
written; Stage O's result and Stage Q's artifact are [P2.10](p2.10_result_artifact_demo/README.md)'s and feed it.
**Feeds:** the next iteration of Ein's own development, by [the evaluation of Ein](evaluation/README.md) this
milestone keeps ([Q-M2.15](open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)).
**Id:** M2 throughout. The folder keeps its name, *NL → IR*, which is Level B of this milestone.
**Read beside it:** [the reading list](reading.md), with notes on papers under [`reading/`](reading/), and
[the registry of benchmarks](benchmarks.md): every benchmark M2 names, its source, where it is named and what M2
does with it.

---

## The two main ideas (the user, 2026-11-25)

The user, 2026-11-25, with the answers to this milestone's questions:

> The core idea is to enrich the reasoning steps with solver so the LLM that synthesize problem formalization
> (facts+ontology+theory) checks if it's formalization works, ideally the final test problem answer must be short
> and must be produced by ein program NL output, and we experiment with some kind of replacing chain of thought
> with chain of formal reasonings; On one hand ein works like a tool for LLM, on other hand it works like filter
> of thoughts

and, the same day, with the next answers:

> Another main idea of M2 - evaluate ein.rs in this auto-frmalization application, find/highlight it's
> weaknesses, strength, gaps, bugs and limits, and then go to next iteration of ein/ein.rs development with this
> feedback

Kept as stated. The first names as the core what the note of 2026-11-11,
[the reasoning as a program](#the-reasoning-as-a-program-the-users-idea-2026-11-11), proposed. What the plan takes
from each, the coordinator's reading, which the user may correct:

- **The formalizer checks its own work with the solver.** What the model writes is the formalization (facts, an
  ontology, a theory), and the kernel's reply is how it finds out whether the formalization works. That is the
  loop of the research plan, said from the model's side.
- **The answer is Ein's, and short**: what the program prints through its query's `:goal-text`, with EinAf's
  formatters over the values that do not read as text
  ([Q-M2.12](open_questions/answer/Q-M2.12-what-prints-the-answer.md)).
- **The experiment is a chain of formal reasoning in place of a chain of thought**: the model reasoning in text
  (the research plan's B1) against the same model reasoning through Ein, at a similar budget.
- **Two roles for Ein**, in the user's words ([Q-M2.13](open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)). *A tool
  for the LLM*: the model *"can decide to use it on it's own, it is recommened but optional"*. *A filter of
  thoughts*: the model *"is forced to write ein code and can not avoid it"*. Both are arms.
- **The milestone measures Ein as well as the loop.** Every stage that drives the kernel records what it found of
  it, under the user's five words (strengths, weaknesses, gaps, bugs, limits), in
  [the evaluation of Ein](evaluation/README.md); the evaluation is a deliverable of the milestone beside the research
  result, and what the next iteration of Ein's development starts from.

## The research question

The root [README](../../README.md) keeps two things apart: **Ein**, the
symbolic kernel, shipped; and **EinAf**, the application over it,
where a neural component proposes a formalization and the kernel judges it,
in a loop. This milestone is EinAf. The research plan
([`EinAf.md`](EinAf.md), the user's, kept verbatim at the milestone root the
way M1d kept [the note it began with](../ideas/11-saturation-and-satisfiability.md)) states the question
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

**The second measured thing is the kernel itself**: what it was good at in this application and where it was
weak, missing, wrong or too small, kept in [the evaluation of Ein](evaluation/README.md) (the second of the two main
ideas).

## Ein, ein.rs and EinAf

The user's ruling on [Q-M2.8](open_questions/einaf/Q-M2.8-where-does-einaf-live.md): EinAf is a tree of its own in
this repository, *"outside of ein, because ein.rs is an implementation of ein and is a dependency of einaf, but
inside the development process"*.

| | what it is | where | depends on |
|---|---|---|---|
| **Ein** | the language, what a program means and what its answer is; and **ein.rs**, an implementation of it | `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/ein/`, `docs/decisions/`; `ein.rs/` | nothing in the repository |
| **EinAf** | this milestone's subject: a model proposes a formalization and the kernel judges it, in a loop | `einaf/`, `docs/einaf/`, both made by [S2.1.1](../../docs/einaf/README.md) | ein.rs, as a library |
| the process | how both are developed | `plans/`, `utils/`, the gate and CI, `AGENTS.md` | reads both |

**Nothing of Ein reads, links or names EinAf.** A change the kernel takes in this milestone is stated in Ein's
terms, under Ein's rules, with its decision under `docs/decisions/`; what EinAf decides stays in
[the ledger](open_questions/README.md) until the thing is built, and is then a page of `docs/einaf/`.

## The plan as re-planned

The research plan [`EinAf.md`](EinAf.md), the user's and kept verbatim, stands as it is: the question, the four
levels, the stages A to Q. The ten phases and their order stand. What was re-planned is everything the phases say
about the kernel, which they described as M1a shipped it, and everything the user's rulings changed. This section
is the frame every phase and stage file is written to.

### What the rulings settle

| | the ruling | in the plan |
|---|---|---|
| the kernel | the one M1f left ([Q-M2.1](open_questions/kernel/Q-M2.1-when-is-the-kernel-frozen.md)); it grows a machine surface for diagnostics, in its own terms ([Q-M2.9](open_questions/kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md)); and it may gain any feature and any bug fix that leaves Ein's syntax and semantics as they are ([Q-M2.16](open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)) | [S2.1.2](../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md) is the first such change, the diagnostics and a bound that reaches inside a saturation; a stage that needs another names it; every record names the commit and the library's manifest |
| the language | Rust ([Q-M2.7](open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)) | EinAf is one Cargo package, `einaf/`, that links `ein-core`, `ein-ir`, `ein-infer` and `ein-render` by path and stands outside the engine's workspace; the kernel's answer reaches it as values, not as JSON read back |
| the model | local, `qwen3.5:4b` ([Q-M2.2](open_questions/model/Q-M2.2-must-the-model-be-local.md), [Q-M2.5](open_questions/model/Q-M2.5-local-model-choice.md)), run in EinAf's process through a Rust binding to llama.cpp, not through ollama ([Q-M2.14](open_questions/model/Q-M2.14-what-runs-the-model.md)) | [S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md); the model's file, by its SHA, is a field of every record, and another suitable model is another value of it |
| the two arrangements | the tool and the filter, both arms ([Q-M2.13](open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)) | [P2.4](p2.4_loop/README.md): the filter is the state machine, the tool is [S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md); [S2.5.2](p2.5_harness/s2.5.2_baselines.md) compares them |
| the answer | Ein's own, by `:goal-text`, with EinAf's formatters; M1p does not run; **a sentence**, *"like zebra's “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”"*; and EinAf's two fixed texts where no program prints one ([Q-M2.12](open_questions/answer/Q-M2.12-what-prints-the-answer.md)) | the query's `:goal-text`, a sentence, is in the contract ([S2.2.1](p2.2_formalizer/s2.2.1_contract.md)); the formatters are [S2.2.5](p2.2_formalizer/s2.2.5_from_text.md)'s; a gold answer is the sentence its canonical program prints, with the values it binds, beside an `:expect` claim ([P2.3](p2.3_benchmark/README.md)); a sentence is scored by what it states ([S2.5.3](p2.5_harness/s2.5.3_metrics.md)) |
| the minimal goal | the model, which does not solve the Zebra puzzle by reasoning in text, solves it *"with help of Ein or through Ein"* ([Q-M2.17](open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)) | the first line of [the acceptance](#acceptance); the model alone on the puzzle is a record of EinAf's, with its context and budget stated |
| a flagged program | an outcome of its own, never certified ([Q-M2.11](open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)) | the seventh outcome below; its sites are fed back; no canonical program of the benchmark is flagged |
| alternatives | a hypothesis, `(open F)`; there is no `hrule` ([Q8](open_questions/formalizer/Q8-ambiguous-parses.md)) | [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) keeps the experiment between the two strategies |
| *certified unique* | open until experiments ([Q-M2.10](open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)) | every record carries each reading an instance has (`--single`'s word, the maximal line's, a description's two counts) with its family and shape, and both conditional probabilities are computed under each |

### The arms

[Stage D](EinAf.md#stage-d--establish-baselines)'s family, B0 to B5, with the tool added as it invites (*where
possible add another … structured reasoning approach*). One model, one benchmark split, a similar budget of
generated tokens for each.

| arm | what the model does | Ein |
|---|---|---|
| **B0** | answers the question, thinking off | none |
| **B1** | reasons in text, a chain of thought, and answers | none: **the side the others are compared with** |
| **B2** | writes one Ein program | **the filter**, one shot: the answer is what the program prints; where it prints none, one of two fixed texts of EinAf's, or no answer (default 15) |
| **B3** | writes a program, and again when the kernel does not pass it, told only *try again* (F0) | the filter |
| **B4** | the same, told the verdict (F2) | the filter |
| **B5** | the same, told everything the kernel found (F8) | the filter |
| **B6** | reasons in text with Ein offered as a function it may call, recommended and optional | **the tool** |

- **In the filter the program is the reasoning.** B2 to B5 run with the model's thinking off and its whole output
  held to Ein: by the harness, which accepts nothing else, and by a grammar over the tokens
  ([S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md)). The filter with its grammar off (the user: *"writing ein program
  may be not impossible without GBNF"*) and the filter after a chain of thought are ablations
  ([P2.6](p2.6_ablations/README.md)).
- **In the tool nothing holds the syntax.** The model writes Ein as it can; what the tool replies is one level of
  the feedback ladder, the same object the filter renders ([S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md)); and
  the record says how often the model called it, and whether its answer is what its last run printed.
- **The experimental variable is still what the kernel is allowed to say**, F0 to F8: the arms are the points of
  it Stage D names, and [P2.6](p2.6_ablations/README.md) has the rest.

### The outcomes

What the kernel can say of a program, as EinAf reads it off the library's values. The research plan's
[A1](EinAf.md#a1-freeze-the-semantic-boundary-of-ein) has six; today's kernel says each for itself, and has a
seventh. [`analysis/` § 2](analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk) has
each row run; [S2.1.1](../../docs/einaf/01-outcomes.md) writes the operational test.

| outcome | the kernel's way of saying it |
|---|---|
| **invalid** | a refusal at the parse or the load: today text on stderr, after [S2.1.2](../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md) a value with a kind and a place |
| **inconsistent** | R is empty and every node is dead: `Contradiction` under `--single`, a core and the rules that refuted under a description |
| **incomplete** | a state owes: `Open`, with what is owed. It is the kernel's word and not the problem's class: an empty R whose every node owes is also what a problem with no solution looks like where no rule refutes it |
| **several** | `Ambiguity`, with two states that differ |
| **one** | `Solution`; over which part of R is [Q-M2.10](open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)'s |
| **not decided** | a budget or the clock stopped the run before the question was decided; today neither reaches inside a saturation that has no fixpoint, and [S2.1.2](../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md) makes them; never read as one of the five above |
| **flagged** | the program is not monotone: an answer is printed and nothing is claimed of it ([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)); never certified |

Beside the outcome stands the query's own axis: a query's classes, their rows, and whether the rows agree. *An
ambiguous theory with one answer* is a state the outcome alone does not say.

### Where EinAf's things are

One tree for EinAf and its documentation apart, as the user ruled ([Q-M2.8](open_questions/einaf/Q-M2.8-where-does-einaf-live.md)); the folders inside are the re-plan's. [S2.1.1](../../docs/einaf/README.md)
made `einaf/` (the package, `src/` and `tests/`) and `docs/einaf/`; the other folders are made by the stages that need
them.

| path | what |
|---|---|
| `einaf/` | the Cargo package: the library and the `einaf` command |
| `einaf/src/` | the kernel as EinAf reads it (the feedback object), the model's runtime, the formalizer, the formatters, the loop, the tool, the record, the metrics, the benchmark's generators |
| `einaf/prompts/`, `einaf/grammars/` | the prompts, versioned by hash; the grammars, generated |
| `einaf/benchmark/` | the instances: a text, its gold class and short answer, its canonical program with an `:expect`; the frozen splits |
| `einaf/experiments/` | the records, immutable, one a run, kept on the host that ran them and never committed ([below](#what-is-committed)) |
| `docs/einaf/` | EinAf's documentation |

The model's file is not in the repository: a record names it by its SHA, and a run is given its path.

#### What is committed

**The user's rule of 2026-11-26**, which replaces the ruling of the same day on what a run commits: *"If something
can be re-generated without agent then we must not commit it, except if I explicitely ask for. For example
documentation renderings - can be re-generated, but in order to read documentation we need renderings present."*;
and of experiments, *"content of folders like plans/m2_nl_to_ir/p2.2_formalizer/report/out,
plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs, plans/m2_nl_to_ir/p2.2_formalizer/steps/runs considered experiments
artefacts and should not be committed into the repo; scripts, benchmarks, automation and analysis tools - should be
committed; results, reports, analysis - also should be committed"*.

- **Committed**: the plan of a run, before its first generation, and its texts and inputs; the scripts, the
  benchmarks, the automation and the analysis tools; the results (a table, a result page), the reports and the
  analyses; and EinAf's test fixtures (`einaf/tests/fixtures/`), which are the tests' inputs.
- **Not committed**: a run's artefacts, everything a script, a tool or a model run writes and writes again with no
  agent: the record of each call, the generations, the programs a model wrote, a run's JSON, outputs and stderr, the
  renderings of them, a report script's outputs. A stage writes them under a folder named `runs/` or `out/`, which
  [`.gitignore`](.gitignore) keeps out of git, and they stay on the host that ran it. A page may link into them (the
  link check takes a missing target git ignores for an output); on a checkout that has none, a replay needs the run
  made again with the model first.
- **The rule holds from 2026-11-26 on**: what was committed before it stays committed (S2.2.0a to S2.2.5's records,
  the phase report's `out/`) until the user asks otherwise (*"I did not ask to delete anything right now. But we must
  follow this rule in future"*). **The plans of P2.5, P2.7, P2.9 and P2.10 that speak of committed records** under
  `einaf/experiments/` were written before the rule: it holds over them, and each is re-aimed to it when its phase is
  planned to run.

### The defaults the re-plan took

Each is the coordinator's, recorded so that the user can correct it. Four have been ruled on since and say so
(2, 7, 11 and 15), and one was replaced by a ruling (16).

1. **The ten phases keep their ids and their order** (four more, `P2.2b` to `P2.2e`, were planned between P2.2 and
   P2.3 after P2.2 ran: [the order before P2.3](#the-order-before-p23)). P2.1 to P2.5 are at stage depth, P2.6 has one stage file,
   P2.7 to P2.10 are phase READMEs, as before (§ How deep this plan is).
2. **One tree a project** (ruled: *"One tree - yes, preferred, docs/einaf/ - separately"*): EinAf's code,
   grammars, benchmark and records are folders of `einaf/`; the plan's `grammars/`, `benchmark/`, `experiments/`
   at the root are not made, and `packaging/` is dropped with the Compose service.
3. **The benchmark's generators are Rust**, in the package, so that EinAf has one toolchain.
4. **EinAf has a gate step of its own**, for what needs no model. A run that needs the model's file and the GPU is
   an experiment, by hand, with its record, in no tier.
5. **`llama-server` is the reference** a binding is compared with, and the fallback if none loads the model.
6. **The budget is generated tokens**, thinking included, with the kernel's work recorded beside it and not traded
   against it.
7. **An answer is a sentence, scored by what it states** (ruled: the re-plan's default was a bare value, and the
   user meant *"a sentence, like zebra's"*). Every arm is asked for a sentence or two in the problem's words; a
   program's answer query prints one through its `:goal-text`; a gold answer is the sentence its canonical
   program prints, with the values that sentence binds. An answer is right when it states each of those values and
   no other in its place: how that is read off a model's free sentence is
   [S2.5.3](p2.5_harness/s2.5.3_metrics.md)'s to design, and equality of two sentences is not the score. For the
   arms that run Ein the gold `:expect` claim is checked too, by the kernel, where the model's program names what
   the claim names; where it does not, the claim is *not comparable*, which is neither held nor refuted.
8. **The benchmark keeps the research plan's three classes** (one answer, ambiguous, unsatisfiable) as an
   instance's gold class, with [F26](../followups/f26_consistency_and_completeness.md)'s finer split as metadata;
   the word problems of `examples/word_problems/` are its second domain from the start.
9. **Texts are English and names ASCII**: a text in another language is out of the benchmark until Ein's names
   are not ASCII only.
10. **A stage that drives the kernel adds to [the evaluation of Ein](evaluation/README.md)**; a wrong answer goes to
    the wrong-answer register as well; every level's gate is a stop where the evaluation is shown
    ([Q-M2.15](open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)).
11. **The kernel may be changed inside M2 where Ein's syntax and semantics stay as they are** (ruled,
    [Q-M2.16](open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md): features such as logs, traces,
    measurements, timeouts and limits, and bug fixes; the re-plan's default was *only where a ruling says so*).
    Each such change is a task of the stage that needs it, made under Ein's rules, and the entry of the
    evaluation it closes says so. A change to the language waits for the iteration of Ein.
12. **P2.8 states its dependency on [M10](../m10_external_benchmarks/README.md) as work not done**, and the order
    of the two is decided at the Level C gate. **P2.9 does not wait for M5's venue**: its kernel half cites
    [`docs/ein/reasoning/`](../../docs/ein/reasoning/README.md) and does not restate it.
13. **No file of the plan carries an estimate, or a date as its status** (the user, 2026-11-08).
14. **Every stage begins by checking what its file says of the tree**, as the stages of this project do: the
    files are written against the kernel at M1f's close, and S2.1.2 changes it.
15. **Where no program prints the answer, EinAf says it in a fixed text of its own** (ruled: *"OK"*). A query's `:goal-text` is
    filled once a row, so nothing is printed where the answer is that the statements do not determine it, or that
    they contradict each other. For those two EinAf has one fixed text each, chosen by the answer query's classes
    over R (not by the verdict word: a program with several states and one answer prints its own) and never
    written by the model; an owing R, a run not decided, a flagged program and an invalid one give **no answer**
    ([S2.2.5](p2.2_formalizer/s2.2.5_from_text.md) words the texts, [S2.5.3](p2.5_harness/s2.5.3_metrics.md) has
    the rule). It is the one place where the answer is not *the Ein program's own output*
    ([Q-M2.12](open_questions/answer/Q-M2.12-what-prints-the-answer.md)), and is said so.
16. **The kernel bounds its own run, and EinAf calls it in its own process.** Nothing in a solve stops a
    saturation that has no fixpoint today, and the programs are a model's; by the ruling of default 11 the clock
    and the budget are made to reach inside a saturation
    ([S2.1.2](../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md)). A worker process that EinAf can
    stop, the re-plan's first answer, is built only if a probe shows a case those bounds do not stop
    ([S2.1.3](../../docs/einaf/03-feedback.md#4-what-the-bounds-do-not-stop)).
17. **The benchmark's `test` split is opened twice**: at the Level B gate, for the one-shot run, read in
    aggregate only, and for the result ([P2.10](p2.10_result_artifact_demo/README.md)). Everything between is
    developed on the other splits ([S2.3.4](p2.3_benchmark/s2.3.4_splits.md)).
18. **A probe a finding rests on is kept**, under [`analysis/probes/`](analysis/README.md), so that an entry of
    the evaluation names a program and a command that can be run again.

## Levels, and the phases that reach them

The research plan defines four levels. Level A is the kernel and is **shipped**, with one named gap. The other
three are this milestone, in order, each with a gate that the phases before it have to pass, and each gate a stop
where [the evaluation of Ein](evaluation/README.md) is shown.

| level | the plan's definition | reached by | gate |
|---|---|---|---|
| **A — symbolic engine** | representation, saturation, constraints, model search, provenance, diagnostics, testing | shipped; P2.1 closed the one gap Stage A named that was still open, a machine surface for diagnostics | the kernel treated as *instrumentation*: the seven outcomes told apart on values, a diagnostic with a kind and a place, and EinAf's feedback object over both |
| **B — autoformalization system** | previously unseen NL tasks automatically become executable Ein theories and answers | [P2.2](p2.2_formalizer/README.md) + [P2.3](p2.3_benchmark/README.md) | one-shot `einaf from-text` on the benchmark's *test* split, unseen by prompt development, three or more reasoning families, every run recorded, each answer a short text Ein printed |
| **C — research system** | iterative feedback, several domains, credible baselines, quantitative benchmarks, ablations, failure analysis; *supports an empirical claim* | [P2.4](p2.4_loop/README.md) – [P2.7](p2.7_failure_scaling_generalization/README.md) | the main table (B0 to B6 under a matched budget), the feedback ablations, a classified failure sample, and one claim of the three admissible shapes in § Stage O |
| **D — complete research artifact** | C plus formal treatment, a heterogeneous frozen benchmark, generalization and scaling, faithfulness analysis, reproducible infrastructure, released raw results, a paper, a minimal demo | [P2.8](p2.8_representations/README.md) – [P2.10](p2.10_result_artifact_demo/README.md) + [M5](../m5_presentation/README.md) | the seven layers of § Stage Q, each independently usable; the paper is M5's |

The plan's own words on the last row, kept because they set the bar correctly: *Level D is not feature
completeness. Ein can still have an enormous roadmap.* It is reached when there is a well-defined hypothesis, a
system that can test it, controlled evidence that distinguishes competing explanations, a formal account, and
enough released material for someone else to reproduce or challenge the result.

## The research plan's stages, and where each lives

`EinAf.md` is organised as seventeen stages A–Q. They are not all this
milestone's, and the ones that are do not all get a phase of their own; this
table is the map from the plan to the folder.

| stage | what it asks for | where it lives |
|---|---|---|
| **A** kernel as experimental foundation | the semantic boundary frozen (A1), invariants as tests (A2), a versioned machine interface (A3), a diagnostic vocabulary apart from presentation (A4), validation suites (A5) | A1, A2 and A5 are the kernel's and exist: [`docs/ein/`](../../docs/ein/README.md), the gate, [`07-diagnostics.md`](../../docs/ein/user/04-reference/07-diagnostics.md); A1's sixth outcome is the kernel's own since [Q-M1d.6](../../docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md). What of A3 and A4 was missing, a diagnostic on a machine surface, was **P2.1**'s, closed ([Q-M2.9](../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)) |
| **B** autoformalization as an explicit task | the contract (B1), source vs generated (B2), what the model may know (B3), prompt versioning (B4) | **[P2.2](p2.2_formalizer/README.md)** |
| **C** a heterogeneous benchmark | eight families (C1), external + synthetic (C2), unique / ambiguous / unsat (C3), difficulty knobs (C4), frozen splits (C5) | **[P2.3](p2.3_benchmark/README.md)** |
| **D** baselines | B0 direct answer … B5 full loop, under a matched inference budget | [S2.5.2](p2.5_harness/s2.5.2_baselines.md), with B6, Ein as a tool |
| **E** the loop | the state machine (E1), repair vs regenerate (E2), faithfulness (E3), termination (E4) | **[P2.4](p2.4_loop/README.md)**: the filter; the tool beside it is [S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md) |
| **F** feedback levels F0–F8 | the experimental variable | [S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md) |
| **G** ablations G1–G9 | retry control, verdict, unsat core, provenance, model difference, feedback size, depth, representation, library | **[P2.6](p2.6_ablations/README.md)** |
| **H** metrics | four layers, and the two conditional probabilities | [S2.5.3](p2.5_harness/s2.5.3_metrics.md) |
| **I** failure taxonomy | a classified sample; neural / representation / symbolic / interface / benchmark | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **J** scaling | formalizer vs solver vs combined degradation | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **K** generalization | paraphrase, structure, domain, novel vocabulary, adversarial noise | [P2.7](p2.7_failure_scaling_generalization/README.md) |
| **L** compare representations | Ein / SMT / Datalog / code / proof assistant **as LLM targets** | **[P2.8](p2.8_representations/README.md)** — seeded by [M10](../m10_external_benchmarks/README.md)'s hand-written encodings, which answer a different question |
| **M** formal model | `K = (O, R, F, Γ, P)`, `lfp(T_Γ)`, `E(T) → (v, d)`, `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))` | **[P2.9](p2.9_formal_account/README.md)**: the loop's half, over [`docs/ein/reasoning/`](../../docs/ein/reasoning/README.md); the categorical reading is [F1](../followups/f1_categorical_formulation.md)'s and the logical one [M1g](../m1g_logic/README.md)'s; the paper's sections on the model consume it |
| **N** reproducibility | one canonical workflow; every experiment an immutable record | [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) — **before the first table, not after the last** |
| **O** the central result | one claim, of three admissible shapes, one of them negative | **[P2.10](p2.10_result_artifact_demo/README.md)** |
| **P** the paper | twelve sections | [M5](../m5_presentation/README.md) |
| **Q** the public artifact | seven layers; the one-command demo | **[P2.10](p2.10_result_artifact_demo/README.md)** + M5 |

## Phases

In work order ([below the table](#the-order-before-p23)); the ids are in the plan's order but for the four `P2.2x`, which were planned after P2.2 ran and come before P2.3.

| ID | title | stages | what it ends with |
|---|---|---|---|
| P2.1 | The kernel as instrumentation — Stage A | S2.1.1 EinAf's tree and the outcomes; S2.1.2 the kernel's diagnostics as values; S2.1.3 the feedback object; S2.1.4 four findings fixed (planned after the report, ran to its end) | `einaf/` builds and is tested in the gate; the kernel says a refusal, an abort, a load's notes and a core's sources as values and on a machine surface; EinAf's feedback object over them, one for every corpus program. **Closed 2026-11-26** on the user's word, its folder deleted; [its defaults](defaults/p2.1.md) stand |
| [P2.2](p2.2_formalizer/README.md) | The formalizer — Stage B | [S2.2.0a](p2.2_formalizer/s2.2.0a_zebra_formulations.md) three Zebras (added by the user); [S2.2.1](p2.2_formalizer/s2.2.1_contract.md) the contract, and the steps; [S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md) the model's runtime, and whether the model writes Ein; [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) the grammars; [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) the steps; [S2.2.5](p2.2_formalizer/s2.2.5_from_text.md) from a text to a short answer; all under [the user's notes on formalization in steps](p2.2_formalizer/README.md#formalization-in-steps-the-users-notes-on-the-whole-phase) | `einaf from-text`, one shot: a text becomes a program in three steps (the facts and a partial ontology, then the complete ontology and a partial theory, then the complete theory, with the answer query among the facts) and the short answer Ein prints, with the run's record and each step's program in it; the Zebra puzzle from its text to its canonical answer, and the word problems to theirs. **Closed 2026-11-27** on the user's word, its folder kept: the machinery built and tested in the gate, the Zebra puzzle one shot not met (*The statements contradict each other.*), no word problem answered for the text's reason ([its close](p2.2_formalizer/README.md#the-close-2026-11-27)); [its defaults](defaults/p2.2.md) stand |
| [P2.2b](p2.2b_guided_thinking/README.md) | Guided thinking: the model decomposes, Ein solves | [S2.2b.1](p2.2b_guided_thinking/s2.2b.1_questions_by_hand.md) the questions, the analysis by hand, and the thinking's grammar; [S2.2b.2](p2.2b_guided_thinking/s2.2b.2_step1_guided.md) step 1 with guided thinking; [S2.2b.3](p2.2b_guided_thinking/s2.2b.3_steps_and_verdict.md) steps 2 and 3, the one call, and the verdict | the model's thinking asked the user's questions (objects, relations, properties, attributes, kinds, groups, sets, orders, processes, dependencies) instead of solving, by an instruction, a scaffold, a grammar over the thinking and a budget; measured on step 1, then steps 2 and 3 and the one call, against thinking off and thinking left alone; the verdict for the user. **Planned 2026-11-26** on the user's word, not started |
| [P2.2c](p2.2c_decoding_and_model_alone/README.md) | A generation that ends, and the model alone | [S2.2c.1](p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md) a generation that ends; [S2.2c.2](p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md) the model alone, measured; [S2.2c.3](p2.2c_decoding_and_model_alone/s2.2c.3_the_tool_probed.md) the tool, probed | a stop of its own for a generation whose tail repeats, held on every recorded generation with no model; the model reasoning in text under its own sampler on the Zebra text, its two edits, a renamed and an unseen puzzle, which says where the minimal goal's premise stands; a few episodes with Ein offered as a function where the model fails alone. **Planned 2026-11-27** at P2.2's close, not started |
| [P2.2d](p2.2d_formalizer_corrected/README.md) | The formalizer corrected | [S2.2d.1](p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md) a query a question, and what an answer says; [S2.2d.2](p2.2d_formalizer_corrected/s2.2d.2_the_cut.md) the cut of the steps; [S2.2d.3](p2.2d_formalizer_corrected/s2.2d.3_entries_described.md) an entry described for its use, with what it needs; [S2.2d.4](p2.2d_formalizer_corrected/s2.2d.4_check_findings.md) what the check can see where the kernel is silent; [S2.2d.5](p2.2d_formalizer_corrected/s2.2d.5_grammars_from_the_program.md) grammars rendered from the program; [S2.2d.6](p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md) the prompts again, the run again, and the nearest miss; [S2.2d.7](p2.2d_formalizer_corrected/s2.2d.7_first_iteration_and_relaxation.md) what the first iteration should leave, and relaxation (the user's question, Q-M2.44) | the one shot's formalizer corrected where the miss was its own arrangement's: each correction removes a defect by construction or names it in a sentence, held by a test with no model; the cut of the steps measured, for the user to rule; the plan's 29 run again as a development number; the least edit of the model's Zebra program that answers. **Planned 2026-11-27** at P2.2's close, not started |
| [P2.2e](p2.2e_kernel_fixes/README.md) | Ein's two bugs, and a clock | [S2.2e.1](p2.2e_kernel_fixes/s2.2e.1_declared_again_under_an_import.md) a relation declared again, under an import; [S2.2e.2](p2.2e_kernel_fixes/s2.2e.2_slots_neighbours.md) `std.slots`: two neighbours do not share a place; [S2.2e.3](p2.2e_kernel_fixes/s2.2e.3_clock_and_a_dead_root.md) the clock, and a root that dies; [S2.2e.4](p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md) the order of a program's forms, and two rows of the table; [S2.2e.5](p2.2e_kernel_fixes/s2.2e.5_recordings_follow_the_library.md) EinAf's recordings follow the library (the user's choice for L-M2.31); [S2.2e.6](p2.2e_kernel_fixes/s2.2e.6_a_core_only_where_printed.md) a dead root's core taken only where a question prints it (the user's word for E-130); [S2.2e.7](p2.2e_kernel_fixes/s2.2e.7_two_more_cores_only_where_printed.md) two more cores taken only where a question prints them (the user's word for E-131) | changes of Ein that leave its syntax and semantics as they are: E-104 and E-105 closed, so that nothing of the benchmark is written against a library rule known wrong; the clock reaching a root that dies, and no question taking a core it does not print (S2.2e.6, done; S2.2e.7 for two more); on the user's word, the order of a program's forms a guarantee Ein states. **Planned 2026-11-27** at P2.2's close, **started** the same day |
| [P2.2f](p2.2f_text_analysis/README.md) | The text analysed by trained networks before it is formalized: AMR first | [S2.2f.1](p2.2f_text_analysis/s2.2f.1_the_check.md) the check, what trained parsers read right where step 1 went wrong; [S2.2f.2](p2.2f_text_analysis/s2.2f.2_the_analysis_written.md) the analysis written for the formalizer, as text and as Ein facts; [S2.2f.3](p2.2f_text_analysis/s2.2f.3_step1_with_the_analysis.md) step 1 given the analysis; [S2.2f.4](p2.2f_text_analysis/s2.2f.4_verdict.md) the verdict | feasible or not on this host (8 GB of GPU memory, speed no question), arm by arm; what S2.6.4 measures as its input-side arm in place of link grammar (Q11, ruled). Planned 2026-11-27 on the user's word and **started** the same day, in worktrees; S2.2f.1 and S2.2f.2 done, S2.2f.3 waits for S2.2c.1 |
| [P2.2g](p2.2g_names_across_an_import/README.md) | Names across an import: what a module's facts are in the program that imports it | [S2.2g.1](p2.2g_names_across_an_import/s2.2g.1_every_name_a_module_writes.md) every name a module writes, today; [S2.2g.2](p2.2g_names_across_an_import/s2.2g.2_the_users_reading_worked_out.md) the user's reading worked out, with the options for how a program imports `m.relation`, to the user's ruling; [S2.2g.3](p2.2g_names_across_an_import/s2.2g.3_the_ruling_made_true.md) the ruling made true | Q-M2.43 ruled, and what an import means settled with it: a module's declarator, labels and constants in the program that imports it. Planned 2026-11-27 on the user's word, not started |
| [P2.3](p2.3_benchmark/README.md) | The benchmark — Stage C | [S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md), [S2.3.2](p2.3_benchmark/s2.3.2_generators.md), [S2.3.3](p2.3_benchmark/s2.3.3_external_benchmarks.md), [S2.3.4](p2.3_benchmark/s2.3.4_splits.md) | eight families and the word problems; generators with an exact gold answer and a canonical program whose `:expect` the kernel checks; instances with one answer, ambiguous and unsatisfiable; external adapters; frozen splits. **Level B gate** |
| [P2.4](p2.4_loop/README.md) | The loop — Stages E, F | [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) the filter's state machine; [S2.4.2](p2.4_loop/s2.4.2_repair.md) repair; [S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md) the feedback ladder; [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) faithfulness; [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) alternatives as hypotheses; [S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md) Ein as a tool | both arrangements: the filter, with every transition logged, repair as well as regenerate, F0 to F8 as nine named sets of the blocks one object renders into, termination and cycle detection; the tool, an episode in which the model calls Ein when it likes; faithfulness judged against the source in both |
| [P2.5](p2.5_harness/README.md) | Baselines, metrics, records — Stages D, H, N | [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md), [S2.5.2](p2.5_harness/s2.5.2_baselines.md), [S2.5.3](p2.5_harness/s2.5.3_metrics.md), [S2.5.4](p2.5_harness/s2.5.4_first_table.md) | the immutable experiment record; B0 to B6 under a matched budget; the four metric layers; **the first main table**, which is the first measurement of a chain of formal reasoning against a chain of thought |
| [P2.6](p2.6_ablations/README.md) | Ablations — Stage G | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), and three more named in the phase | G1 to G9, each a table with its pre-registered reading; the grammar off, thinking before the program, and the analysis arm (P2.2f's, in place of link grammar) among them |
| [P2.7](p2.7_failure_scaling_generalization/README.md) | Failure analysis, scaling, generalization — Stages I, J, K | named when P2.5 has numbers | a classified failure sample; degradation curves for the formalizer, the solver and the pair; K1 to K5. **Level C gate** |
| [P2.8](p2.8_representations/README.md) | Representations compared — Stage L | named at the Level C gate | the five narrow questions of Stage L answered for Ein against SMT, Datalog, code and a proof assistant as targets a model writes |
| [P2.9](p2.9_formal_account/README.md) | The formal account — Stage M | named at the Level C gate | the loop's half of the conceptual model, written independently of the Rust, over the kernel's half that `docs/ein/reasoning/` already states |
| [P2.10](p2.10_result_artifact_demo/README.md) | The result, the artifact, the demo — Stages O, Q | named at the Level C gate | one claim with its evidence; the seven layers released; one command that shows the loop; the evaluation of Ein written up. **Level D gate**, with M5 |

### The order before P2.3

The four phases planned after P2.2 ran come before [P2.3](p2.3_benchmark/README.md), each started only on the
user's word. The order the routing of P2.2's results proposed, the user's to change:

1. **[P2.2e](p2.2e_kernel_fixes/README.md) and [P2.2c](p2.2c_decoding_and_model_alone/README.md)**, in either
   order: neither reads the other. P2.2e's second stage is what P2.3 waits for; P2.2c's first is what every later
   run with the model reads.
2. **The record and the answer's comparison**, [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) and the first
   part of [S2.5.3](p2.5_harness/s2.5.3_metrics.md), which [P2.5's own order](p2.5_harness/README.md#order) has run
   before P2.2's numbers: P2.2 took its numbers with stand-ins for both.
3. **[P2.2d](p2.2d_formalizer_corrected/README.md)**: the contract P2.3's instance is shaped by, and the formalizer
   the Level B run opens `test` with.
4. **[P2.2b](p2.2b_guided_thinking/README.md)**'s runs with the model, on the corrected formalizer and the cut that
   stands; its first stage needs no model and may run at any time.

**[P2.2f](p2.2f_text_analysis/README.md)**, the text analysed by trained networks (AMR first), planned 2026-11-27 on
the user's word, is not in this order: nothing of P2.3 waits for it, its first stage, the check, needs no model and
may run at any time, and when it runs is the user's.

**[P2.2g](p2.2g_names_across_an_import/README.md)**, names across an import, planned 2026-11-27 on the user's word for
Q-M2.43, is not in this order either, unless its first stage finds that P2.3's programs would read otherwise under a
ruling.

## How deep this plan is

**P2.2 to P2.5 are at stage depth**, twenty stage files (and the six `P2.2x`, twenty-seven more: three of P2.2b's, planned 2026-11-26; seventeen of P2.2c's, P2.2d's and P2.2e's, planned 2026-11-27 from P2.2's results and the user's questions since; four of P2.2f's and three of P2.2g's, planned the same day): nineteen re-planned 2026-11-25 against the kernel
at M1f's close with [the analysis](analysis/README.md) and its six audits as the worklist, and S2.2.0a
added by the user after; P2.1's four, at the same depth, are deleted with its folder at its close; S2.2.0f, added the same day, is since 2026-11-26 the user's notes on the whole of P2.2. **P2.6 has one stage
file**, [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), which carried the link-grammar experiment
whole until Q11 was ruled and now carries the analysis arm P2.2f finds feasible; its other stages are paragraphs in the phase README. **P2.7 to P2.10 are phase READMEs only.** That is on
purpose: a stage file for G7 written before the first main table exists would be a guess about which depth
matters, and the plan says what the guess-free version is (*plot marginal improvement against cost*), which is
one line and not a file. A phase gets its stage files when the phase before it has numbers.

**A stage file is a plan, not a record of the tree**: each begins by checking what it says of the kernel against
the kernel, and a phase's start re-reads its files against what the phases before it built. Each phase README
ends with the defaults its re-plan took, which are the coordinator's and the user's to correct.

## Acceptance

**The minimal goal** (the user, [Q-M2.17](open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)): the
chosen model, which does not solve the Zebra puzzle by reasoning in text, solves it from its text *with the help
of Ein or through Ein*, by the tool or by the filter, the loop included; the two sides are two records of EinAf's,
the model alone run with its context and budget stated. *"It would be already success."* Every stage that runs
the model on the puzzle says where the goal stands.

Then per level; each gate is checked by a phase named above. **A criterion the model misses is reported as
missed, with its record**, and whether a level is reached is the user's to say at its gate.

**Level B** ([P2.3](p2.3_benchmark/README.md) closes it):

1. `einaf from-text`, one shot, with the record: on the Zebra puzzle's text, the minimal goal's first attempt, a
   program the kernel solves to the canonical answer, or the record of where it stopped; on a word problem, the
   sentence its gold states. The kernel stays free of any model.
2. One-shot formalization on the benchmark's **test** split, three or more families, reported per family with
   parse, load and outcome rates and answer accuracy; the split is unseen by prompt development
   ([S2.3.4](p2.3_benchmark/s2.3.4_splits.md)).
3. The three classes are *distinguished*: on a synthetic instance whose gold class is ambiguous or unsatisfiable
   the one-shot system reports that class, not a guess.
4. Every run is an immutable record: the model's file by its SHA, the binding and its llama.cpp, the sampler,
   prompt hashes, grammar hashes, Ein's commit, the library's manifest, seeds, raw generations, programs, the
   kernel's feedback objects ([S2.5.1](p2.5_harness/s2.5.1_experiment_record.md)).
5. [The evaluation of Ein](evaluation/README.md) is written up as the phases found it, and shown.

**Level C** ([P2.7](p2.7_failure_scaling_generalization/README.md) closes it):

6. The main table: B0 to B6 on the validation split, one model, a matched budget, with the four metric layers and
   both conditional probabilities under each reading of *certified*.
7. The feedback ablations G1 to G9 and the three the rulings added (the grammar off, thinking before the program,
   the level the tool replies at), each with the reading it was pre-registered to have.
8. A failure sample of stated size, classified by the taxonomy, split into neural, representation, symbolic,
   interface and benchmark ambiguity; what of it is the kernel's joins the evaluation.
9. One claim in the form of § Stage O (positive, narrow, or negative) with the table that supports it and the
   one that would have refuted it.

**Level D** ([P2.10](p2.10_result_artifact_demo/README.md) closes it, with M5):

10. The formal account ([P2.9](p2.9_formal_account/README.md)); the benchmark frozen and versioned; scaling and
    generalization curves; the faithfulness analysis; the records released; the paper
    ([M5](../m5_presentation/README.md)); one command that runs the demo in § Stage Q.

## The reasoning as a program (the user's idea, 2026-11-11)

The user, 2026-11-11:

> idea for M2 - LLM's reasoning steps (chain of thoughts, tree of thoughts) could be not plain text but the
> program in formal language (ein in our case) and LLM improves it step by step using solver output, so the
> output became not only plain answer but the whole reasoning logic implemented as self-explained ein program

Kept here as stated; no phase or stage is changed by it yet. Read against the plan above, the coordinator's
reading, which the user may correct:

- **The loop is already this one.** `x → T₀ → E(T₀) → T₁ → ⋯ → Tₙ` improves an Ein program step by step on the
  kernel's output. What the idea adds is **what the loop is for**: the plan measures `Tₙ` as a route to a
  correct answer, and the idea makes `Tₙ` **the output**, the reasoning itself, in place of the free-text
  chain of thought a model would otherwise write beside its answer.
- **Self-explained** has a meaning in Ein since M1f S1f.5.14: every relation and rule carries a `:why`, and a
  derived fact reads as English from its justification
  ([`14-natural-language.md`](../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)). So a program is a
  chain of thought whose every step the kernel has checked, and whose trace is the explanation.
- **A tree of thoughts** has a counterpart the kernel already computes, the hypothesis lattice: a branch is a
  commitment, a dead branch has a core that says why, and the answer is the region R. Whether a model's
  branching should be written as hypotheses is open.
- **It bears on faithfulness** ([S2.4.4](p2.4_loop/s2.4.4_faithfulness.md)): a reasoning the reader can read
  rule by rule is one whose invented constraint can be seen.

**The first worked instances exist**, under [`examples/word_problems/`](../../examples/README.md):
elementary-school word problems (*"Bob has 5 apples. He gave 2 to Alice. How many apples they have
together?"*) as Ein programs written by hand. The arithmetic is an imported theory, the meaning of the words is
rules (*gave* is a subtraction, *together* a sum), and a missing quantity is detected and named instead of
assumed. Each problem carries its sentences as the `:source` of the facts they were encoded as, which is the pair
this milestone's formalizer has to produce, and each program is an example of the output this idea asks for. They
are the benchmark's second domain from the start ([P2.3](p2.3_benchmark/README.md)), among them instances whose
right answer is *not enough information*.

## Open questions

[`open_questions/`](open_questions/README.md), one file a question under its topic, the index listing the open ones
first and the user's rulings after them.

**The milestone's ledgers are three folders**: [`open_questions/`](open_questions/README.md),
[`evaluation/`](evaluation/README.md), what the application finds of Ein, and [`findings/`](findings/README.md), what it
finds that is neither Ein's nor a question (`L-M2.n`). Each holds one file an item under a topic folder, and an index.
**Every phase keeps the same three of its own**, beside its README, its ids taken from the milestone's sequences; at
the phase's close what is open moves into the milestone's folder of the same name, under the same topic, and what is
closed stays in the phase's folder, which M2 keeps until the milestone's own close (the user's rule, above). P2.2's
close, 2026-11-27, was the first under that rule.

**Ruled by the user, 2026-11-25:** the kernel M2 reads is the one M1f left
([Q-M2.1](open_questions/kernel/Q-M2.1-when-is-the-kernel-frozen.md)); the model is local
([Q-M2.2](open_questions/model/Q-M2.2-must-the-model-be-local.md)), is `qwen3.5:4b`
([Q-M2.5](open_questions/model/Q-M2.5-local-model-choice.md)) and runs through a Rust binding to llama.cpp
([Q-M2.14](open_questions/model/Q-M2.14-what-runs-the-model.md)); the harness is Rust
([Q-M2.7](open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)); EinAf is a tree of its own,
outside Ein and inside the development process ([Q-M2.8](open_questions/einaf/Q-M2.8-where-does-einaf-live.md)); the
kernel grows a machine surface for diagnostics, in its own terms
([Q-M2.9](open_questions/kernel/Q-M2.9-does-the-kernel-grow-a-machine-surface-for-diagnostics.md)); a flagged program is
an outcome of its own ([Q-M2.11](open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)); the answer is
Ein's, by `:goal-text` and EinAf's formatters, and M1p does not run
([Q-M2.12](open_questions/answer/Q-M2.12-what-prints-the-answer.md)); Ein is both a tool and a filter
([Q-M2.13](open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)); the kernel may gain features and bug fixes that leave
Ein's syntax and semantics as they are
([Q-M2.16](open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md)); and the minimal goal is the Zebra
puzzle solved by the model with Ein
([Q-M2.17](open_questions/milestone/Q-M2.17-what-is-the-milestones-minimal-goal.md)).

**Open, and the user's:** what *certified unique* means on R, decided after experiments, with the rule that
scores an answer beside it
([Q-M2.10](open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)); and where M2 stops for
the next iteration of Ein, decided at a level's gate
([Q-M2.15](open_questions/milestone/Q-M2.15-where-does-m2-stop-for-the-next-iteration-of-ein.md)). Neither holds a phase.

**Open, and a stage's:** Q8, by experiment, its form settled; Q9 (Q11 is ruled, 2026-11-27: no link grammar, no strictly pre-defined grammar);
[Q-M2.3](open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md), the unit of
faithfulness where there is no gold theory;
[Q-M2.4](open_questions/answer/Q-M2.4-is-the-fixed-point-syntactic-or-semantic.md), the fixed point; and Q-M2.6, one
grammar or many. **Q10 is re-homed to Stage L**, and **Q7 stays outside the research question**.

**Open, and raised by P2.1**, by its report, and the milestone's since the phase's close:
[Q-M2.18](open_questions/einaf/Q-M2.18-are-einafs-fmt-and-clippy-steps-of-the-gate.md), EinAf's fmt and
clippy as gate steps; [Q-M2.19](open_questions/kernel/Q-M2.19-do-ein-summary-2-and-ein-test-report-6-keep-their-numbers.md),
the schema numbers; [Q-M2.20](open_questions/kernel/Q-M2.20-what-are-max-steps-name-unit-and-default.md),
`--max-steps`' name, unit and default; [Q-M2.21](open_questions/kernel/Q-M2.21-a-missing-files-line-the-pages-python-spelling-or-eins-own.md),
the missing file's line. Each holds a default the phase took ([its defaults](defaults/p2.1.md)).

**Open, and raised by P2.2**, the milestone's since the phase's close, 2026-11-27, each with the default the phase
took ([its defaults](defaults/p2.2.md)): Q-M2.22 to Q-M2.34, by its stages (what a faithfulness event is and what a
`:source` quotes, Q-M2.22, Q-M2.25, Q-M2.26; the library and the kernel, Q-M2.23, Q-M2.34, and Q-M2.24, ruled 2026-11-27 (an answer does not depend on the order of a program's forms: a guarantee Ein states); the decoding and
the grammars, Q-M2.27 to Q-M2.31; B2's mode, Q-M2.32; an answer of several rows, Q-M2.33); and Q-M2.35 to Q-M2.42, by
[the analysis of the phase](p2.2_formalizer/report/ANALYSIS.md) and the routing of its results (a query a question,
the cut of the steps, where a library's descriptions live, the target shape's abbreviation, the minimal goal's
instance, a third arm, a clause of *the theory before the facts*, and when Ein's two bugs are fixed).
[The ledger's index](open_questions/README.md#open) has each with what it waits for. **Q-M2.35 holds
[S2.2d.1](p2.2d_formalizer_corrected/s2.2d.1_a_query_a_question.md)**, and Q-M2.22, Q-M2.25, Q-M2.26 and Q-M2.38
shape an instance of the benchmark ([S2.3.1](p2.3_benchmark/s2.3.1_families_and_seed_set.md)); no other holds a
phase.

## Connections

- [`EinAf.md`](EinAf.md) — the research plan, verbatim; this README is its
  map and the phases are its schedule. Edit it only as the user's note.
- [The root `README.md`](../../README.md), the repository's page: Ein, ein.rs and EinAf, and what depends on
  what.
- [`findings/`](findings/README.md): what the milestone finds that is neither Ein's nor a question for the user.
- [`defaults/`](defaults/README.md): the defaults a closed phase took that still stand, one file a phase: P2.1's
  whole, P2.2's as a map into its kept folder.
- [The evaluation of Ein](evaluation/README.md): what this application found of the kernel, for the iteration of
  Ein that follows.
- [`analysis/`](analysis/README.md): the plan against the project, the six audits and the host, which the
  re-plan was written from; an audit is where a stage looks for the evidence of what its file says changed.
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
- The verdict vocabulary
  ([Q-M1d.6](../../docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md),
  [Q-M1d.1](../../docs/decisions/answer/Q-M1d.1-a-search-stops-early-only-on-a-decided-question.md));
  `:expect`, the form a benchmark instance's gold is written in, and `ein test`, which checks it;
  [M10](../m10_external_benchmarks/README.md) — the hand-written encodings
  P2.8 starts from, and the benchmark direction that is *not* this one
  (formal-language shaped, not M2-gated).
- [`docs/ein/ein.rs/04-api.md`](../../docs/ein/ein.rs/04-api.md): what EinAf links;
  [`08-read-out.md`](../../docs/ein/user/04-reference/08-read-out.md), the read-out and `ein-summary/2`;
  [`--events`](../../docs/ein/ein.rs/05-events.md) (`ein-events/1`).
- [`examples/word_problems/`](../../examples/README.md): word problems as hand-written Ein programs over the
  calculator, each sentence beside the facts it was encoded as: the first instances of § The reasoning as a
  program, and the benchmark's second domain.
- The neighbours that are on this milestone's ground and run only on the user's word:
  [M1p](../m1p_ein_to_natural_language/README.md), Ein to natural language, which takes EinAf's formatters as its
  starting point; [M1h](../m1h_beyond_word_problems/README.md), which hands over pairs of sentence and facts;
  [M1n](../m1n_non_monotone_programs/README.md), which owns what a flagged program's answer is;
  [M15](../m15_mcp_server/README.md), an MCP server, another way a model would call Ein;
  [M12](../m12_rules_as_data/README.md), rules as data. The followups that name a consumer like this one as
  their trigger: [F22](../followups/f22_exact_core_past_the_cap.md), the exact core;
  [F23](../followups/f23_diagnostics_in_eins_words.md), diagnostics, whose D-18 and L-125 are S2.1.2's;
  [F24](../followups/f24_written_provenance.md), written provenance in a core;
  [F26](../followups/f26_consistency_and_completeness.md), the four classes;
  [F27](../followups/f27_cryptarithm_theory.md), a benchmark of puzzle families.
- [M20](../m20_gui/README.md) — displays whatever the loop records; not a
  dependency either way.
