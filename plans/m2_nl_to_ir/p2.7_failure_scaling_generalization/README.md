# P2.7 — Failure analysis, scaling, generalization (Stages I, J, K)

**Depth:** a phase README, on purpose. Its three stages get their files, and their final names, when
[P2.5](../p2.5_harness/README.md) has numbers ([README § How deep this plan is](../README.md#how-deep-this-plan-is)).
**Depends on:** [P2.5](../p2.5_harness/README.md) and [P2.6](../p2.6_ablations/README.md), whose records the
failure sample is drawn from and the curves are computed over; [P2.3](../p2.3_benchmark/README.md), whose
generators produce every set of this phase with other arguments, and whose knobs are its x-axes;
[P2.4](../p2.4_loop/README.md), both arrangements, since the sample and the combined curves are of the filter and
of the tool ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)).
**Blocks:** the **Level C gate**, which closes here and is a stop
([Q-M2.15](../open_questions.md#q-m215--where-does-m2-stop-for-the-next-iteration-of-ein));
[P2.10](../p2.10_result_artifact_demo/README.md), which tests on the `test` split the sentence this phase writes.
**Feeds:** [the evaluation of Ein](../ein_evaluation.md). The failure sample's symbolic, interface and
representation sides and Stage J's solver curve are the largest measured input it gets.
**Research plan:** [`EinAf.md` § Stage I](../EinAf.md#stage-i--failure-taxonomy),
[§ Stage J](../EinAf.md#stage-j--scaling-experiments),
[§ Stage K](../EinAf.md#stage-k--generalization-experiments).

---

## Three stages of the plan, one phase, and why

Stages I, J and K share a question the aggregate tables cannot answer: **where does the system fail**, and is the
limit the language side or the search side. The failure taxonomy says *what kind*, the scaling curves say *at what
size*, the generalization sets say *under what change of surface*. They are one phase because they read the same
records and because each one's finding changes what the other two look for: a taxonomy dominated by *direction
reversal* makes the paraphrase set (K1) the next experiment, and a scaling curve on which the solver degrades first
makes the failure sample at the largest size the one to read.

**The phase is also where the milestone's second main idea is measured most directly**
([README § The two main ideas](../README.md#the-two-main-ideas-the-user-2026-11-25)). Two of the taxonomy's five
sides are about the kernel, the *symbolic* and the *interface*, and a third, the *representation*, is about the
language; Stage J's solver curve is the kernel's limits, measured on programs nobody wrote by hand. What they find
is written as entries of [the evaluation of Ein](../ein_evaluation.md), in its five kinds, and nothing of the
kernel is fixed here ([README § The defaults the re-plan took](../README.md#the-defaults-the-re-plan-took), 10
and 11).

## What the kernel gives this phase today

Checked at the re-plan. [`analysis/audit_p2.7_p2.10_neighbours.md` § B.1](../analysis/audit_p2.7_p2.10_neighbours.md#b1-p27--failure-analysis-scaling-generalization)
has each row with its evidence, and § B.0 the runs. **A stage begins by re-taking what it leans on**: the files
were written against the kernel at M1f's close, and [S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md)
changes it.

| this phase needs | what there is |
|---|---|
| a solver-only run of a canonical program | a question about R: a bare `ein solve` (describe), `--single`, `--minimal`, `--maximal`, `-n N`, `--count`; for EinAf the same as calls of the library ([`04-api.md`](../../../docs/ein/ein.rs/04-api.md)) |
| the run's effort | `work.saturations` and `work.facts` (facts beyond root) in `ein-summary/2` ([`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md)). **No surface carries a time or a memory figure**: EinAf takes the clock around its own call |
| whether the question was decided | `region.exhausted`, `region.stopped` (why, facts spent and projected), and under `--single` the `decided` row and `verdict.word`, which is `null` where the run stopped first |
| the bounds | `--region-budget`, facts derived beyond root, 2²⁰ by default, exit 0 with what was established; `-T`, seconds, exit 2; `--region-limit`, the cube a flagged program gets |
| a count of answers | three, and they differ (§ S2.7.2) |
| a depth of guessing, a depth of proof | no such number on any surface (§ S2.7.2) |
| a class for a flagged program | the seventh [outcome](../README.md#the-outcomes): an answer is printed and nothing is claimed of it, so such a run is neither a solver success nor a solver failure |
| somewhere to send a wrong answer of the kernel | [the wrong-answer register](../../wrong_answers/README.md): a row with its witness, in a commit of its own, held by a gate step |

**What a run costs depends on the question asked of it**, so a solver curve is a curve *of a question*. Re-taken
at the re-plan, on the binary of the working tree:

```sh
ein solve examples/zebra2.ein                        # describe: k_s 1, k_c 261 982, 120 saturations
ein solve examples/zebra2.ein --single               # Solution, 7 saturations
ein solve examples/zebra2-minus-15.ein --single      # Ambiguity, decided, 39 saturations
ein solve examples/zebra2-minus-15.ein --maximal     # 32 states, all of the line, 108 saturations
ein solve examples/zebra2-minus-15.ein --count       # k_s 32; k_c a lower bound; stopped at the budget, 11 222 saturations
ein solve examples/features/11_expect_ambiguity.ein  # k_s 7, k_c 7, a maximal line of 2 states
```

**One limit a generated program can meet that no bound of `ein solve` covers.** Termination of a saturation is
assumed, not given ([`02-saturation.md` § The fixpoint](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md#the-fixpoint)):
a rule that nests its conclusion deeper than its premise has no fixpoint. Probed at the re-plan on a program of
the shape [M1g's S1g.6.1](../../m1g_logic/p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md)
keeps as its witness: `ein solve FILE --no-search -T 1` and `ein solve FILE --single --region-budget 1000` both
ran until an outer `timeout` ended them. The clock is checked before each step of work, and by the probe a
saturation is inside one step; the budget counts facts beyond root. The library's saturator takes a step cap,
which `ein saturate --max-steps` uses and the routes over R do not. For this phase it is a point of the solver curve that never comes back, so
every solver-side run is bounded by EinAf from outside, and the bound firing is the outcome *not decided* with its
reason recorded. How the loop itself is protected is [P2.4](../p2.4_loop/README.md)'s.

## The three stages

### S2.7.1 — The failure taxonomy (I)

A sample of failures of a stated size, **stated before it is drawn**, from the records of B5, B6 and the ablations
on the validation split, stratified by family, by gold class, by arm and by
[outcome](../README.md#the-outcomes). Each sampled failure is read whole: the text, every program the model wrote,
every reply of the kernel raw (the feedback object) and as rendered at the level the run used, and the gold. It is
classified under the research plan's sixteen categories (lexical interpretation, entity extraction, relation
extraction, direction reversal, quantifier error, cardinality error, missing constraint, invented constraint,
incorrect generic rule, type construction error, query construction error, unsupported semantics, solver
limitation, repair regression, repair hallucination, answer extraction) and then filed under one of five sides:

| side | what it is | where it goes |
|---|---|---|
| **neural** | the model misread the text or miswrote the program | the phase's reading; K1 to K5 and [P2.6](../p2.6_ablations/README.md)'s ablations are its follow-up |
| **representation** | Ein cannot say it, or can say it only as a program the kernel answers nothing for | the evaluation: a **gap** where the language lacks the form; a **limit** where the faithful program is one the static check flags (a closed-world reading written with `absent` over a relation a commitment can fill), of which nothing is claimed ([Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md); [M1n](../../m1n_non_monotone_programs/README.md) owns what such an answer is) |
| **symbolic** | the kernel answered wrong, or did not answer | wrong by Ein's own rules: a **bug**, and where it is a wrong answer also a row of [the register](../../wrong_answers/README.md), by its rules. Stopped by the budget or the clock, or not returning: a **limit**, with the program |
| **interface** | what the model was shown misled it | the kernel's surface did not carry what was needed: a **gap**. The kernel did as designed and the design worked against the application (a theory selected under a name the file does not list is inert, with no message): a **weakness**. EinAf's feedback object or a renderer was wrong: EinAf's fault, corrected in `einaf/` as a new version of the harness, with the records it touched named |
| **benchmark ambiguity** | the gold was wrong | the generator or the adapter is corrected, which makes a new version of the benchmark ([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)); the instance and the correction are recorded |

A strength is recorded the same way: where the sample shows a reply of the kernel that a repair turned on, that is
an entry of the evaluation with its record.

**The taxonomy is read against the language before the sample is drawn.** Two of its sixteen categories name what
Ein does not have as such: *type construction error* (Ein has no types,
[Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md): what
is there to get wrong is a declaration, its labels and the facts a theory reads as kinds) and *cardinality error*
where it means a count of arguments (a form of a declared relation at another count loads in silence,
[Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md), so nothing
tells the loop). **It is allowed to grow**, and a category added is recorded with the instance that forced it.
The candidates today's kernel and the two arrangements make likely, none added until an instance forces it: a
theory not activated (spelled under a name the file does not list); no hypothesis mechanism written, so that the
blind enumerator answered; `std.contradiction` not imported, so that a fact beside its negation killed nothing; a
flagged program written; and, in the tool, Ein never called, its reply not used, or an answer that is not what the
last run printed.

**Who reads.** By default the stage's agent reads and labels every sampled failure, the label recording who gave
it, and the user reads what stays ambiguous and whatever the user picks. What is built for the user's reading
lists the failures still open, each with every artefact of the kernel, raw and rendered. This sample is also where
the human-read part of [Q-M2.3](../open_questions.md#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory)
is sized.

**Output:** the table of categories by side; the sample with its labels, a record under `einaf/experiments/`,
kept whole; a reading of each side; and for the three sides about Ein, the entries added to the evaluation.

### S2.7.2 — Scaling (J)

The generators of [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md) at increasing size, one knob swept at a time,
over the research plan's eight (entities, facts, rules, relation arity, constraint density, model count, proof
depth, hypothesis depth), and the size ladder of an external set where [P2.3](../p2.3_benchmark/README.md)'s
adapters carry one. Three curves a knob, measured apart, each per family:

- **formalizer degradation**: B2's formalization metrics against the canonical program
  ([S2.5.3](../p2.5_harness/s2.5.3_metrics.md)), no reply of the kernel reaching the model; beside it B1's answer
  accuracy on the same instances, which is how a chain of thought degrades;
- **solver degradation**: the canonical program alone, the formalizer not in the loop. Per question, since the
  cost is the question's: the question the loop asks ([S2.4.1](../p2.4_loop/s2.4.1_state_machine.md) fixes
  which) and a description. Measured: `work.saturations`, `work.facts`, EinAf's clock around the call, and
  whether the question was decided, at the default `--region-budget`. A point at which the run stops is plotted
  as *not decided* with its reason, never dropped;
- **combined**: B5 and B6, accuracy and generated tokens against difficulty, with the kernel's work beside it.

**Three of the eight knobs have no single reading on today's kernel**
([audit § B.1](../analysis/audit_p2.7_p2.10_neighbours.md#b1-p27--failure-analysis-scaling-generalization)).
S2.7.2 decides each, by what the generators can set and what the kernel reports on the generated instances; the
defaults it starts from:

| knob | why it has no single reading | the default |
|---|---|---|
| **model count** | R has two counts, `k_s` (states) and `k_c` (commitments), which differ by orders of magnitude on one program, and a third reading, the states of the maximal line. Where a program states no obligation R holds partial commitments, so `k_s` counts more than the full assignments. A description may stop at the budget with `k_c` a lower bound and the minimal line not established | the x-axis is **the generator's own count**, the number of answers it built the instance to have, checked against **the states of the maximal line** (`--maximal`), which is defined whether or not the program owes, and which on the clue-dropped Zebra is established where a description stops at the budget (the runs above). `k_s` and `k_c` are recorded where a description finishes and reported beside it. If the user's decision on [Q-M2.10](../open_questions.md#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) names another reading, the axis follows it |
| **hypothesis depth** | no depth exists: a depth cap changed answers and is retired. What a description reports nearest to it: `region.minimal.least`, the size of the least commitment that reaches R; `region.root_owes`, the instances root owes, which on a monotone program bounds the length of any branch ([`10-satisfiability.md` § 4](../../../docs/ein/reasoning/02-kb-evolution/10-satisfiability.md#4-why-an-obligations-alternatives-are-jointly-exhaustive)); `region.universe.candidates`, the size of H, which every question reports. The first two are a description's keys, `null` under `--single` and under a line's question, and `least` is `null` where the description stops first | plot against `region.minimal.least` where the description finishes, with `root_owes` and the candidates recorded at every point; the generator's knob that moves it is how much of the instance is given |
| **proof depth** | no surface carries the depth of a derivation. The summary's `work` has saturations, facts and layers; the steps of a derivation are in `--events` and in the library's provenance | the x-axis is **the generator's own chain length**, exact by construction. If a stage needs the engine's figure, EinAf computes it from the library's provenance of the answer's facts, and the missing number is an entry of the evaluation |

The other five are properties of the generated program that its metadata states; *relation arity* is the count of
arguments, by which a relation is overloaded.

**The boundary the plan asks about**, *is the limiting factor language understanding or symbolic search?*, is
where the first two curves cross, if they do; the phase reports the crossing or its absence per family. With a
model of this size the crossing may lie outside the range the generators reach; the solver curve then stands by
itself, as the evaluation's limits.

**How the kernel's curve is taken.** Through [`utils/bench_env.sh`](../../../utils/bench_env.sh), as every
measurement of the kernel is; with no model loaded, on AC power, at `-j 1`; each point bounded from outside
(§ What the kernel gives this phase today). A second budget is the stage's choice. The solver curve the plan of
August said was *already measured once* was of the engine's retired search, and its table is deleted with its
plan ([audit § A.3](../analysis/audit_p2.7_p2.10_neighbours.md#a3-p27), 26): this stage's is the first.

### S2.7.3 — Generalization (K1 to K5)

Five sets, each the benchmark's generators with one argument changed, each run at B1, B2, B5 and B6 and reported
as **invariance**: the drop from the base set. B1 is in every set because the milestone's experiment is a chain of
formal reasoning against a chain of thought, and a drop is read against the same drop of B1.

- **K1, paraphrase.** Three or more surface templates a statement type, the minimum
  [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md) is told to write. A set paraphrased by a model is added only if
  the stage can check that each paraphrase keeps the canonical program's meaning; the paraphraser is then a second
  value of the record's model field.
- **K2, structure.** Prompts developed on small instances, tested on larger ones: the size bins of the frozen
  split ([S2.3.4](../p2.3_benchmark/s2.3.4_splits.md)), not a set of its own.
- **K3, transfer.** Two cuts. Across families: developed on ordering and attribute matching, tested on the others;
  the families the standard library has no theory for are the hard half, by design. Across domains: developed on
  the puzzle families, tested on the word problems, and the reverse. **Which families have no theory is read off
  the library a record names**, by its manifest: today they are object tracking and temporal reasoning, and
  [M1h's P1h.1](../../m1h_beyond_word_problems/p1h.1_temporal_and_causal_theories/README.md) plans a timeline and
  a causal theory as library modules. It has not started. If a module for a family lands between two runs the hard
  half is stated again, and runs over different manifests are not put in one figure.
- **K4, novel vocabulary.** `left-of` becomes `zorps`, with its meaning stated in the text: what separates
  relational reasoning from lexical priors. On the kernel's side this is an invariance already held: the kernel
  reads no theory's name, and every corpus program keeps its answer with every other name renamed
  ([Q-M1f.436](../../../docs/decisions/engine/Q-M1f.436-the-kernel-reads-no-theorys-name.md)). So the drop on K4
  is the model's. What is renamed is the problem's own names; a library activator keeps its spelling, and names
  stay ASCII.
- **K5, adversarial irrelevant information.** Irrelevant, redundant, distracting and superficially similar
  statements inserted at a stated rate. The canonical program with the inserted statements encoded is run beside
  the one without, and its answer, its work and what the read-out's `outside` row names are recorded, so that a
  change on the solver's side is seen apart from the model's.

**K2 and K3 bind earlier phases.** A prompt developed on a family or a size it is later tested on makes that
set no test, so what each prompt was developed on has to be known when this phase runs: the prompt's hash is a
field of every record ([S2.5.1](../p2.5_harness/s2.5.1_experiment_record.md)), and which split, families and
sizes a prompt saw has to be kept with the prompt from [P2.2](../p2.2_formalizer/README.md) on. Where it was not
kept, the set is reported as no test.

## The Level C gate

The gate closes here, with [P2.5](../p2.5_harness/README.md)'s table and [P2.6](../p2.6_ablations/README.md)'s
ablations, and **it is a stop**. What is put in front of the user:

1. the main table, the ablations and the classified failure sample, which are what Level C asks for
   ([README § Acceptance](../README.md#acceptance), 6 to 9); the curves of S2.7.2 and S2.7.3, which the research
   plan counts towards Level D and this phase produces;
2. **the sentence the claim will be**, in one of the three shapes of
   [§ Stage O](../EinAf.md#stage-o--produce-the-central-research-result), as the validation split supports it,
   naming the arms and the conditions, before [P2.10](../p2.10_result_artifact_demo/README.md) tests it on `test`;
3. **[the evaluation of Ein](../ein_evaluation.md), written up**: the user says whether EinAf goes on or an
   iteration of Ein comes first ([Q-M2.15](../open_questions.md#q-m215--where-does-m2-stop-for-the-next-iteration-of-ein));
4. the table [Q-M2.10](../open_questions.md#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) waits for,
   both conditional probabilities under each reading of *certified*, per family, if the user has not decided on
   it before;
5. what the plan defers to this gate: the order of [P2.8](../p2.8_representations/README.md) and
   [M10](../../m10_external_benchmarks/README.md), and the stages of P2.8 to P2.10.

## Acceptance

- A failure sample of a size stated before it was drawn, labelled, a record under `einaf/experiments/`; the split
  into five sides with a reading of each; every *benchmark ambiguity* traced to a correction and a benchmark
  version.
- Every failure filed under *symbolic*, *interface* or *representation* is an entry of the evaluation or is
  covered by one; every wrong answer of the kernel is a row of the register.
- Scaling curves for every knob of [S2.3.2](../p2.3_benchmark/s2.3.2_generators.md)'s metadata, three curves
  each, per family; for model count, proof depth and hypothesis depth the reading plotted is stated with the
  figure; the crossing reported, or its absence.
- K1 to K5, each as a drop from the base set at B1, B2, B5 and B6, with the separation of development and test
  honoured (K2, K3).
- A flagged program is counted in a column of its own in every table and curve of the phase.
- The Level C gate's five items shown.

## Risks

- **Reading failures is the slowest thing in the milestone and the most valuable.** The sample's size is stated
  before it is drawn and is not enlarged afterwards to reach a significance.
- **A kernel finding is a temptation to fix the kernel.** It is recorded and the application works with the kernel
  as it is; the kernel changes inside M2 only where a ruling says so. Every record names Ein's commit
  ([Q-M2.1](../open_questions.md#q-m21--when-is-the-kernel-frozen)), so a figure says which kernel it is of.
- **The model may be the limit at every size.** Then no curve crosses, the taxonomy is mostly *neural*, and the
  phase's result about the kernel is the solver curve alone. That is a result, and it is reported as one.
- **A description may not finish.** Past the budget `k_c` is a lower bound and the minimal line is not
  established; a curve plots what the run established and marks the rest.
- **K4 may be where the model collapses.** That is a result about lexical priors and is reported as one.
- **A time read off this machine is not a time.** On battery, or with the model loaded beside the kernel, a
  figure is of the machine's state; hence `bench_env.sh` and the kernel's curve taken alone.

## Connections

- [`EinAf.md` § Stage I](../EinAf.md#stage-i--failure-taxonomy),
  [§ Stage J](../EinAf.md#stage-j--scaling-experiments),
  [§ Stage K](../EinAf.md#stage-k--generalization-experiments).
- [The evaluation of Ein](../ein_evaluation.md), where what this phase finds of the kernel goes;
  [the wrong-answer register](../../wrong_answers/README.md).
- [`analysis/audit_p2.7_p2.10_neighbours.md`](../analysis/audit_p2.7_p2.10_neighbours.md) § A.3 and § B.1: what
  of the old text was stale, with the evidence.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md): an external set that ships one task at several sizes
  is close to a ready experiment on scaling.
- [M1h](../../m1h_beyond_word_problems/README.md), not started: its P1h.1 would give two of K3's families a
  theory, and its P1h.10 plans fifty questions graded by difficulty, which is scaling material if it has run.
- [M1n](../../m1n_non_monotone_programs/README.md), what a flagged program's answer is;
  [M1g's P1g.6](../../m1g_logic/p1g.6_engine_requirements/README.md), where requirements on the engine are
  collected from another side, a saturation that does not stop among them;
  [F28](../../followups/f28_language_engine_stdlib_left_by_m1f.md), what M1f left of the language and the engine;
  [F26](../../followups/f26_consistency_and_completeness.md), the finer classes a gold class carries as metadata;
  [F22](../../followups/f22_exact_core_past_the_cap.md), why a core may differ between two runs of one program.

## Defaults taken at the re-plan

Each is the re-plan's, recorded so that the user can correct it.

1. **Model count is plotted against the generator's own count**, checked against the states of the maximal line;
   `k_s` and `k_c` are recorded beside it.
2. **Hypothesis depth starts from `region.minimal.least`**, with `root_owes` and the candidates recorded; the
   stage decides.
3. **Proof depth is the generator's chain length**; the engine's figure, if needed, is computed by EinAf.
4. **The solver curve is per question**, at the default budget, bounded from outside, taken with no model loaded.
5. **B1 and B6 join B2 and B5** in the generalization sets, and B1 stands beside B2 on the formalizer's curve.
6. **The five sides map onto the evaluation's five kinds** as the table of S2.7.1 says.
7. **The agent labels, the user reads what stays ambiguous.**
8. **The sample is a record kept whole**; a page built for the user's review lists what is still open.
9. **K3 has a second cut, across domains**, the puzzle families against the word problems.
10. **The gate is at the end of the phase**, all three stages done, although Level C asks only for the first.

Dropped as moot: the curve *already measured once* and the review register named as precedent, neither of which
exists ([audit § A.3](../analysis/audit_p2.7_p2.10_neighbours.md#a3-p27), 26 and 28); *a finding for M1d*, which
is closed (27); and the phase's estimate (the README's default 13).
