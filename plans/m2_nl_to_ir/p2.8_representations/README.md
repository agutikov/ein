# P2.8 — Representations compared (Stage L)

**Depth:** a phase README, on purpose. Its stages are named and written after the Level C gate, because the five
questions below are worth asking only of a representation whose loop has numbers
([README § How deep this plan is](../README.md#how-deep-this-plan-is)).
**Depends on:** the **Level C gate** ([P2.7](../p2.7_failure_scaling_generalization/README.md));
[P2.4](../p2.4_loop/README.md)'s filter, which this phase runs with another judge in the kernel's place;
[P2.3](../p2.3_benchmark/README.md)'s instances and [P2.5](../p2.5_harness/README.md)'s record and metrics.
**On [M10](../../m10_external_benchmarks/README.md) it depends as on work not done**: M10 has run no stage, so
there is no set of systems it installed, no runner and no corpus of encodings to reuse
(§ What exists). **The order of the two is the user's, decided at the Level C gate**
([README § The defaults the re-plan took](../README.md#the-defaults-the-re-plan-took), 12).
**Blocks:** nothing on the critical path. Level D's *comparison* is this phase, and the paper's discussion of the
boundary between the neural and the symbolic (§ 9.4 of [Stage P's outline](../EinAf.md#stage-p--research-paper))
reads it.
**Feeds:** [the evaluation of Ein](../evaluation/README.md). Ein beside four other targets, each written by the same
model, is the widest view the milestone gets of what Ein's representation is good and bad at.
**Research plan:** [`EinAf.md` § Stage L](../EinAf.md#stage-l--compare-symbolic-representations).
[Q10](../open_questions/formalizer/Q10-direct-llm-constraint.md) lives here.

---

## The question, and the one it is not

Stage L asks whether Ein's representation has specific advantages **as a target a model writes**. It does not ask
whether Ein is a better solver. The research plan is explicit: *do not attempt to prove that one representation is
universally superior*; ask five narrow questions instead. The model is the one of the plan, `qwen3.5:4b`, in
EinAf's process ([Q-M2.5](../open_questions/model/Q-M2.5-local-model-choice.md),
[Q-M2.14](../open_questions/model/Q-M2.14-what-runs-the-model.md)); the instances, the prompt skeleton and the budget of
generated tokens are the same for every target.

| the question | how it is measured | where the number comes from |
|---|---|---|
| Which representation is easiest for a model to synthesise? | the share of one-shot outputs the target's own tool accepts, and the share whose answer is right | B2 in each target's language |
| Which produces the most useful repair diagnostics? | repair success under each tool's **native** reply (a solver's `unsat` and its core, an empty relation, a traceback, a goal the proof assistant could not close) against Ein's feedback levels as [P2.6](../p2.6_ablations/README.md) measured them | the filter with the kernel swapped for the target's tool, and the renderer for the tool's output |
| Which detects semantic incompleteness? | on the instances whose gold class is *ambiguous*: a system that returns one model of an under-determined encoding says nothing; one that enumerates, or says that it cannot decide, does | the benchmark's three gold classes, every target run on each ([README § The defaults the re-plan took](../README.md#the-defaults-the-re-plan-took), 8) |
| Which yields the highest verified-answer precision? | `P(correct ∣ the system certifies)` a target, with each target's own notion of certifying, stated before the run | [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)'s definition, applied a target at a time |
| Which requires the least generated formal text? | generated tokens an instance, and a correctly solved instance | the record |

**For Ein, three of the five have more than one value today, and the phase reports each.** *Certifies* has the
readings of [Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) until the
user decides, and a flagged program is certified under none
([Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)). *Detects incompleteness* is said
by the kernel in three ways: `Ambiguity`, with two states that differ; `Open`, with what is owed; and the
read-out's `outside` row, which names the relations H holds no candidate of
([`10-satisfiability.md` § 5](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#5-a-state-of-r-is-a-model-of-the-program-as-h-scopes-it)).
*Easiest to synthesise* is measured with the filter's grammar on and with it off.

**The first question is asked of this model, not of the representations in the abstract.** The user's remark on
[Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md): the model was most probably not trained to write
Ein, and Ein's semantics are near Datalog's and its syntax Lisp's and SMT-LIB's. The other four targets are
widely published languages, which a model has most probably met. So the table compares each target *as this
model meets it*, and the reading says so where the number appears.

### Q10, asked properly

[Q10](../open_questions/formalizer/Q10-direct-llm-constraint.md) was *when is direct LLM → constraint emission
acceptable?* Stage L is the form in which it can be answered: not *is the intermediate representation ever
skipped*, but *what does each target cost the model, and what does each target tell it back*. The old working
answer's use of a second target, **agreement between two targets as a check on either**, is one of this phase's
readings: where two targets certify different answers for one instance, the instance is listed and read as
[P2.7](../p2.7_failure_scaling_generalization/README.md) reads a failure. The program the model wrote was wrong,
or the tool was; if the tool is Ein, that is the symbolic side, with its entry in the evaluation.

## What exists

Checked at the re-plan; [`analysis/audit_p2.7_p2.10_neighbours.md` § B.2](../analysis/audit_p2.7_p2.10_neighbours.md#b2-p28--representations-compared)
and § A.4 have the evidence, and [`analysis/host.md`](../analysis/host.md) the host.

| it needs | what there is |
|---|---|
| the systems, installed | **On this host**: `z3`, `cvc5`, `cvc4`, `souffle`, `clingo`, `swipl`, `racket` and `link-parser` are on `PATH`, and Lean 4 with `lake` after `source ~/.elan/env` ([`host.md` § The solvers and parsers the plan names](../analysis/host.md#the-solvers-and-parsers-the-plan-names) has the versions). Not installed: `minizinc`, `vampire`, `eprover`, `prover9`, `mace4`. A fact about one machine on one day: the stage re-takes it. The CI workflows install none of them |
| a runner | **none.** M10's S10.3 plans one as a crate of the engine's workspace, `ein-bench`; it does not exist |
| hand-written encodings a target, as few-shot examples | **one target of five, two problems**: the three SMT-LIB files of [`nl2smt/smt/`](../../../nl2smt/README.md#the-hand-written-encodings), hand-written in 2021, whose content does not change because M10 counts them as its corpus's. No file of Datalog, answer-set, Prolog or Lean is in the tree. [`lean.md`](../../m10_external_benchmarks/lean.md) is a note on the Zebra puzzle in Lean, reading for that target's contract |
| a pipeline around the SMT target | **a second route from the sentences to SMT, with no model**: [`nl2smt/`](../../../nl2smt/README.md) parses the Zebra puzzle's sentences with NLTK or link grammar, converts them and runs a solver, with tests. It knows this puzzle's words and sentence shapes and no others, and by its own page *decides nothing for M2* |
| the loop with another judge | nothing yet: [P2.4](../p2.4_loop/README.md) builds the filter, and this phase gives it a second judge |
| the decision it must not reopen | [Q2](../../../docs/decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md): Ein hands no problem to an external solver |

## What is shared with M10, what is not, and the order

[M10](../../m10_external_benchmarks/README.md) plans the same problems stated for six systems **by hand**, from
published encodings with their provenance, answers compared first and clocks second. This phase has the model
write the encodings, compares what the model can do with each target, and never times the solvers against each
other.

- **Shared ground**: the systems; the rules of a fair encoding M10's
  [S10.1](../../m10_external_benchmarks/s10.1_problem_corpus.md) states (provenance a file, idiomatic to its
  system and not transliterated, never tuned), which bind this phase's few-shot examples, the one thing of each
  target it writes by hand; and the discipline that each of the other four targets' tools runs as a process.
  Ein is linked, as everywhere in EinAf, which changes nothing that is compared here, since no clock is.
- **Not shared**: M10's answers are established by systems that agree with each other; this phase's answers are
  the benchmark's gold, and a target's agreement with it is the measurement.

**The order is the user's, at the Level C gate.** What each order means for this phase:

| | M10's S10.1 to S10.3 first | P2.8 first |
|---|---|---|
| the few-shot examples | taken from M10's corpus, with its provenance | written here, under S10.1's rules, for the four targets that have none |
| the driver of the tools | EinAf may link M10's runner, a crate of ein.rs | a driver in `einaf/src/`, which M10 cannot link: nothing of Ein reads EinAf |
| what waits | Level D waits for three stages of a milestone that is not started and is itself written against the retired search (`-m 38`, `--max-set-size`) | nothing; M10, when it runs, writes its own runner |

**The default this README is written to is the second**: P2.8 does not wait for M10, names what it takes from
M10's ground, and builds the rest in EinAf. The user may reverse it at the gate.

**The no-back-end decision stands.** A target that beats Ein on a row is a finding about targets, and M10's own
sentence applies: *every time a table shows a solver winning by 100×, the next thought is "so call it"*. That is
the dropped M3, and Q2 is its decision.

## Stages, in shape

Named and written after the Level C gate.

**S2.8.1 — The five targets' contracts.** For each target: what the model writes; the tool that judges it and its
version, a field of the record; what *accepted* means; what *certifies* means, stated before any run; the tool's
native reply as a renderer; how the short answer is taken from the tool's output, to be compared with the gold
text as every arm's is; the few-shot examples with their provenance; and whether a grammar holds the syntax.

| target | the tool here | what there is to decide in its contract |
|---|---|---|
| **Ein** | the library, as in every other phase | nothing new: its contract is [S2.2.1](../p2.2_formalizer/s2.2.1_contract.md)'s |
| **SMT-LIB** | `z3`, `cvc5` | who asks the second question: a solver answers `sat` with one model, so *ambiguous* is seen only if a second query excludes it, and whether the model or the harness writes that query is the contract's |
| **Datalog** | `souffle` | whether the target can state a choice at all; if it cannot, whether the answer-set solver that is installed, `clingo`, stands in, as a sixth row and not in Datalog's place |
| **general-purpose code** | Python | what certifies: a program that prints an answer has run, and no more than that, unless it enumerates |
| **a proof assistant** | Lean 4 | what the model writes: a statement the checker decides, or a statement with its proof; a checked proof certifies the statement, and whether the statement says what the text said is the same question as for every other target |

**S2.8.2 — The run.** B2 and the filter's loop, each target under its native reply, on the synthetic families,
the three gold classes included; one model, one set of instances, a similar budget. A run needs the model's file
and the GPU, so it is an experiment by hand with its record, in no tier of the gate.

**S2.8.3 — The five answers.** Five small tables, each with the reading it was registered to have before the run,
and the caveats where the numbers appear: a proof assistant is not a solver; Datalog may be unable to state a
choice ([M10 § Risks](../../m10_external_benchmarks/README.md#risks)). If that second finding recurs here it is
evidence from another direction for what obligations and hypotheses add to a saturator
([Q-M1d.2](../../../docs/decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)), and a
strength for the evaluation.

## Acceptance

- Five representations, each with a contract, a run on the same instances, and a row in each of the five tables.
- Each table read by the rule registered for it before the run; no aggregate winner.
- The *ambiguous* class reported for every target, since it is the one class on which the targets differ in kind
  and not in degree.
- For Ein, every row under each reading of *certified* the user has not ruled out, with the grammar on and off,
  and flagged programs in a column of their own.
- Every instance on which two targets certify different answers is listed.
- What the comparison shows of Ein, for and against, is in [the evaluation of Ein](../evaluation/README.md).

## Risks

- **The model knows four of the targets and not the fifth.** The comparison is of targets as this model meets
  them; the grammar-on row and the examples in the prompt are what Ein gets in exchange, and the reading names
  the asymmetry instead of correcting for it.
- **The bias of an encoding moves into the few-shot examples.** Whoever writes five sets knows one target best.
  M10's rules for a fair encoding apply, and each example carries its provenance.
- **Certifying is not one thing across targets.** Each target's notion is written down before its run; a number
  is compared only with numbers under a notion of the same strength, and the table says which those are.
- **Comparison invites integration.** See the no-back-end decision above.
- **A floor.** A model of this size may fail every formal target alike, and then the tables separate nothing.
  That is reported as it is; a second model is a second value of the record's model field and a replication, and
  taking one is the user's decision.
- **The tools on this host are not the tools on another.** Each record names its tools by version, and no tier
  of the gate runs any of this.

## Connections

- [`EinAf.md` § Stage L](../EinAf.md#stage-l--compare-symbolic-representations).
- [M10](../../m10_external_benchmarks/README.md), not started: the systems, the rules of a fair encoding, and the
  warning that comparison invites integration; [`nl2smt/`](../../../nl2smt/README.md), the sentences to SMT by
  parser and converter, a reference point for the SMT target on one puzzle and no arm of this experiment.
- [`analysis/host.md`](../analysis/host.md): what is installed.
- [Q2](../../../docs/decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md), the hand-off, answered
  *never*; [Q-M1a.23](../../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md), by
  which a benchmark compares Ein with other systems as processes.
- [F26](../../followups/f26_consistency_and_completeness.md), which names this phase for *semantic
  incompleteness* and keeps the finer classes of an under-determined problem;
  [F17](../../followups/f17_formal_verification/ideas.md), the survey of verification tools that is the reading
  list for the proof assistant; [`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md),
  [`03`](../../../docs/lib/03-theorem-proving-formal-methods.md).
- Neighbours on this ground, none started: [M1g](../../m1g_logic/README.md), which offers a formula language with
  checked semantics as a formal target a formalizer can be measured against;
  [M1k's P1k.20](../../m1k_notations_as_facts/p1k.20_s_expressions_as_facts/README.md), SMT-LIB and other
  S-expressions read as Ein facts.

## Defaults taken at the re-plan

Each is the re-plan's, recorded so that the user can correct it.

1. **P2.8 does not wait for M10**: it writes its own few-shot examples under M10's rules and its own driver in
   `einaf/`. The order stays the user's, at the Level C gate.
2. **The targets are Stage L's five**, with the tools this host has: SMT-LIB by `z3` and `cvc5`, Datalog by
   `souffle`, code as Python, the proof assistant Lean 4. `clingo` and `swipl` are installed and are no target
   unless the stage adds one with its reason.
3. **No target but Ein is held by a grammar.** Ein is run with its grammar on and off, the off row being the one
   compared with the others; whether SMT-LIB or Datalog gets a grammar of its own is the stage's.
4. **The filter is the arrangement compared**, one shot and looped. A target's tool offered to the model as a
   function, B6's arrangement ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)), is the stage's to add if
   Level C's numbers make the tool part of the claim.
5. **Ein's row is reported under each reading of *certified*** until the user decides Q-M2.10.
6. **`nl2smt/` is a reference point, not an arm**: whether its output is run beside the model's SMT on its own
   puzzle texts is the stage's.

Dropped as moot: M10's runner *reused rather than rewritten* and its encodings for six systems as the seed, since
neither exists ([audit § A.4](../analysis/audit_p2.7_p2.10_neighbours.md#a4-p28), 29 and 30); *M1d's premise*,
which shipped as the obligations (32); and the phase's estimate.
