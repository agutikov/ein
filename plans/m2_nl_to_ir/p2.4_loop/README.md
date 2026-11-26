# P2.4 — The loop (Stages E, F)

**Depends on:** P2.1: the outcome function
([S2.1.1](../../../docs/einaf/01-outcomes.md)), the kernel's diagnostics as values
([S2.1.2](../../../docs/decisions/diagnostics/Q-M2.9-a-refusal-is-a-value-with-a-kind-a-place-and-its.md)) and the feedback object
([S2.1.3](../../../docs/einaf/03-feedback.md)), the one value every level of the ladder
renders and the tool returns, with the bounded kernel call through which both arrangements reach the kernel.
[P2.2](../p2.2_formalizer/README.md): the one-shot formalizer, in its three steps, is GENERATE
([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)), which also words the answer's fixed texts; the contract's
check is EinAf's ([S2.2.4](../p2.2_formalizer/s2.2.4_passes.md)); the model's runtime gives thinking on or off a
call and the context kept between the turns of one episode
([S2.2.2](../p2.2_formalizer/s2.2.2_model_runtime.md)); the grammar holds the filter's output to Ein
([S2.2.3](../p2.2_formalizer/s2.2.3_gbnf.md)). [P2.3](../p2.3_benchmark/README.md): the seed set and its
canonical programs, which the fixtures and the canonical judge read.
**Blocks:** [P2.5](../p2.5_harness/README.md): B2 to B5 are the filter at one shot, F0, F2 and F8, and B6 is the
tool; [P2.6](../p2.6_ablations/README.md): every ablation is a parameter named here.
**Research plan:** [`EinAf.md` § Stage E](../EinAf.md#stage-e--build-the-iterative-formalization-loop) and
[§ Stage F](../EinAf.md#stage-f--make-symbolic-feedback-progressively-richer); the tool is what
[§ Stage D](../EinAf.md#stage-d--establish-baselines) invites (*where possible add another … structured reasoning
approach*).
**Rulings it carries out:** [Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md) (the tool and the
filter), [Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md) (a flagged program),
[Q8](../open_questions/formalizer/Q8-ambiguous-parses.md) (an alternative is a hypothesis),
[Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md) (the answer is Ein's),
[Q-M2.14](../open_questions/model/Q-M2.14-what-runs-the-model.md) (the model runs in EinAf's process).

---

## What closes here

The one-shot formalizer becomes `Tᵢ₊₁ = A_θ(x, Tᵢ, E(Tᵢ))`, in **two arrangements of one model and one kernel**
([the frame](../README.md#the-arms), [Q-M2.13](../open_questions/einaf/Q-M2.13-who-calls-the-kernel.md)):

| | **the filter** | **the tool** |
|---|---|---|
| who decides that Ein runs | the harness, on every generation: a state machine | the model, when it likes: the prompt recommends it and the model may answer without it |
| what the model writes | an Ein program and nothing else, thinking off | text, and an Ein program where it chooses to call |
| what holds the syntax | the harness, which accepts a program only, and a grammar over the tokens | nothing: a parse error is what the tool returns |
| what the kernel's reply is | the feedback object, rendered at the run's level into the next prompt | the same object, rendered at the run's level, as the function's result |
| the answer | what the Ein program prints; where the kernel does not pass the program, a fixed text of EinAf's that names the outcome, or none | the model's last message; whether it is what its last run of Ein printed is recorded |
| arms | B2 to B5 | B6 |
| built by | [S2.4.1](s2.4.1_state_machine.md), [S2.4.2](s2.4.2_repair.md) | [S2.4.6](s2.4.6_ein_as_a_tool.md) |

Both are built **as instruments**: every step is a logged record, every knob a named parameter, and nothing is
tuned here. What either is worth is [P2.5](../p2.5_harness/README.md)'s table and
[P2.6](../p2.6_ablations/README.md)'s ablations. Shared by both: the feedback ladder
([S2.4.3](s2.4.3_feedback_ladder.md)), the faithfulness judge ([S2.4.4](s2.4.4_faithfulness.md)), and a sentence
with two readings written as hypotheses ([S2.4.5](s2.4.5_alternatives_as_hypotheses.md)).

### The filter's state machine, on this kernel

The research plan's [E1](../EinAf.md#e1-define-the-state-machine) has GENERATE, PARSE, ANALYZE, SOLVE and ANSWER,
with three repair edges. EinAf links the engine ([Q-M2.7](../open_questions/einaf/Q-M2.7-what-language-is-the-frontend-written-in.md)),
so the machine reads no exit code and no line of text: an edge is taken on a field of the feedback object, and
the object's first field is one of [the seven outcomes](../README.md#the-outcomes).

```text
GENERATE ──► CHECK ──► ASK ──► one ─────────────────────────────► ANSWER
 (S2.2.5)      │        │
               │        ├── several, inconsistent, incomplete ──► REPAIR ──► CHECK …
               │        ├── flagged: its sites are fed back ────► REPAIR
               │        └── not decided ────────────────────────► ANSWER, as not decided
               └── invalid ─────────────────────────────────────► REPAIR
```

- **Seven outcomes, each with an edge.** *Flagged* is an outcome of its own and is read before any word the run
  printed: the kernel answers a flagged program and claims nothing of the answer, so the machine never takes it
  for *one* ([Q-M2.11](../open_questions/answer/Q-M2.11-what-is-a-flagged-program-to-the-loop.md)). *Not decided* is a
  budget or the clock stopping the run, and is never read as one of the others; after S2.1.2 an abort is a value.
- **Which question the machine asks is a choice with a cost.** A bare solve describes R, `--single` asks whether
  it is one state, `--maximal` lists the maximal line; on the under-determined Zebra variant `--single` is decided
  after two states and a description runs to the facts budget
  ([`audit_p2.4.md`](../analysis/audit_p2.4.md) § B.0). The default is root alone, then `--single` over all of R,
  with the wider questions asked once, of the final program, so that the record carries every reading of *one
  answer* ([Q-M2.10](../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) is the
  user's, after experiments). [S2.4.1](s2.4.1_state_machine.md) confirms it by measurement.
- **A run is its transcript**: one record a transition, with the program, the feedback object of every question
  asked, the level, the prompt's hash, the raw generation and the tokens, replayable without the model.
- **A kernel call may not return.** Nothing in `ein solve` stops a saturation that has no fixpoint
  (*termination is assumed, not given*:
  [`02-saturation.md`](../../../docs/ein/reasoning/02-kb-evolution/02-saturation.md)), and one rule that builds a
  fact out of the fact it matched is such a program. Both arrangements reach the kernel only through the bounded
  call of `einaf::kernel`, P2.1's, and a call a bound stopped is *not decided*; S2.4.1 says what the loop needs
  of it and has the probe.

### The ladder, level by level

The feedback level is **the experimental variable**: nine levels, one object. Against what the kernel supplies
today ([the analysis](../analysis/README.md#p24--the-loop-audit); [`audit_p2.4.md`](../analysis/audit_p2.4.md)
§ B.1 has each cell's evidence):

| level | the research plan's content | the kernel supplies | EinAf computes |
|---|---|---|---|
| **F0** | *try again* | nothing | a fixed sentence |
| **F1** | syntactic: undefined relation, type mismatch, arity mismatch | a parse error with line, column, notes and ranked repairs; a load refusal, a value with a kind and a place after S2.1.2; a load's notes. **None of the plan's three is a refusal**: an undeclared relation and a form at another count load in silence, and Ein has no types | nothing new: the findings of the contract's check ([S2.2.4](../p2.2_formalizer/s2.2.4_passes.md)), shown under a parameter and never as the kernel's words |
| **F2** | the verdict | the outcome, of a named part of R; root's standing (in R, owing, dead); the query's own axis | one sentence |
| **F3** | cardinality | the states listed and whether they are all; H's size; how many instances root owes; a query's classes. `k_s` and `k_c` under a description or `--count` only | nothing, but saying *or more* where a count is a lower bound |
| **F4** | model difference | two states under `Ambiguity`, each as its facts over the candidates' relations; a query's classes by state. **Nothing computes a difference of two states** | the difference, and its wording |
| **F5** | unsat core | a dead root's core with its status, its sources and the rules that refuted, **under a description only**, not canonical; what an owing root owes | the wording, and the caveats |
| **F6** | dependency | partial counterparts: what is owed, the relations H leaves out, the saturation rules' relation graph (library only) | reachability from the goal, over a graph EinAf completes |
| **F7** | provenance | each fact's recorded justifications, the firings, the trace's steps; an obligation and a hypothesis rule never fire, and a firing's premises omit its absences and its activator | the chain for the facts a lower block named |
| **F8** | the combined diagnostic | all of the above | the union, under a size cap |

**Every level renders the same object**, so a level is a parameter of one run and an ablation is two runs. Three
things the kernel says have no rung in the research plan's ladder (what is owed, where a program is flagged, who
wrote H) and [S2.4.3](s2.4.3_feedback_ladder.md) gives each one. A fact reads as English through what Ein has
today (a stated fact's `:source`, a derived fact's rule's `:why`, a query's `:goal-text`) and EinAf's formatters
for values ([S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md)); M1p does not run
([Q-M2.12](../open_questions/answer/Q-M2.12-what-prints-the-answer.md)).

### Repair, faithfulness, termination, alternatives

**Repair or regenerate** ([E2](../EinAf.md#e2-repair-instead-of-regenerate)). Two values of one parameter:
*regenerate* gives the model the text and the feedback; *repair* gives it its own last program too. In the kept
context a repair is one more turn, so it pays for the feedback's tokens only. The reply *unchanged* is the same
program again, which the machine reads as a fixed point. [S2.4.2](s2.4.2_repair.md).

**Faithfulness** ([E3](../EinAf.md#e3-protect-against-feedback-induced-hallucination)). The dangerous path is
*several → the model invents a constraint → one*. Every program and every repair is judged against the source,
in both arrangements, and no finding of the judge is fed back: the loop's only reviewer is the kernel.
[S2.4.4](s2.4.4_faithfulness.md) builds the judge and decides
[Q-M2.3](../open_questions/answer/Q-M2.3-what-is-the-unit-of-faithfulness-without-a-gold-theory.md).

**Termination** ([E4](../EinAf.md#e4-termination-policy)): the seven conditions as predicates over the
transcript. A cycle is found by two identities, the program's canonical dump and what the program answers;
which of them stops the run is [Q-M2.4](../open_questions/answer/Q-M2.4-is-the-fixed-point-syntactic-or-semantic.md),
decided in S2.4.1.

**Alternatives** ([Q8](../open_questions/formalizer/Q8-ambiguous-parses.md)). A reading the formalizer cannot resolve is a
hypothesis, `(open F)`, with a reading owed; the corpus has the pattern. What is left is the experiment between
emitting both readings and committing to one. [S2.4.5](s2.4.5_alternatives_as_hypotheses.md).

### The theory before the facts

The one shot is made in three steps, the facts and a partial ontology first, then the complete ontology and a
partial theory, then the complete theory
([the user's notes on the formalizer](../p2.2_formalizer/README.md#formalization-in-steps-the-users-notes-on-the-whole-phase)),
and the same notes hand this phase what comes after them: *"After formalization is done we can run solver and
consume feedback"*, under the principle that the facts are *"as close to original text as possible and changed
less then more variable rules"*. Three things follow, in both arrangements where they apply:

- **A repair changes the theory before the facts.** The repair prompt ([S2.4.2](s2.4.2_repair.md)) says so beside
  the research plan's instruction: a step-1 fact is changed only where the feedback names it (a fact in a core, a
  sentence a refuted fact quotes) and no change of the ontology or the theory answers it. Nothing enforces it, since
  a grammar cannot tell the text's fact from the theory's; what a repair did to step 1's facts is counted.
- **A fact rewritten after step 1 is counted.** Against the step-1 program the record keeps, a step-1 statement a
  repair removed or changed is the note `rewritten_fact` of the faithfulness judge
  ([S2.4.4](s2.4.4_faithfulness.md)), counted per run and per level beside the hallucination rate; the event
  `deleted_source_fact` stays for a sentence left with no fact.
- **The facts' faithfulness is judged at step 1**, where they are closest to the text: the mechanical judge's
  `:source` check and the categories of step 1's program are the facts' judgement, and each later program is
  judged for what it adds and for what it changed of them. The tool has no steps, and a program it carries is
  judged whole.

### What the phase records of Ein

The ladder is where the kernel's diagnostics are judged as feedback, and the tool is where a model meets Ein with
nothing between them. Each stage has a task that writes what it found (a strength, a weakness, a gap, a bug, a
limit) into [the evaluation of Ein](../evaluation/README.md), and the phase shows the page at its close.

## What P2.2 hands this phase

*Written 2026-11-27, at [P2.2's close](../p2.2_formalizer/README.md#the-close-2026-11-27).* The files of this phase
were planned before P2.2 ran. What that
phase found that bears on them is in each stage's file, under *What P2.2 hands this stage*; this is the map. None of
it is a ruling: where an item rests on a question, the question's default stands.

**What the one shot says of the loop** ([the analysis of P2.2](../p2.2_formalizer/report/ANALYSIS.md), § 6). As a
baseline B2 is near zero on
P2.2's texts: no Zebra, no word problem answered for the text's reason. The loop inherits more than a repair: of
the seven defects of the model's Zebra program the kernel speaks of two, one at a time, its core naming a sentence
by the accident of which house it took, and the other five it cannot point at
([L-M2.24](../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md)).
Feedback made of the
kernel's word alone would not carry the model from that program to the reference; the contract's check and the
faithfulness judge have to carry most of it, and *the theory before the facts* meets a case where the facts are
what is wrong ([Q-M2.41](../open_questions/formalizer/Q-M2.41-is-a-misread-statement-repaired-before-any-theory.md)).

| stage | what it reads |
|---|---|
| [S2.4.1](s2.4.1_state_machine.md) | GENERATE and how a generation ends ([S2.2c.1](../p2.2c_decoding_and_model_alone/s2.2c.1_a_generation_that_ends.md), [Q-M2.36](../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md)); the bounded call's clock a question's ([L-M2.10](../findings/einaf/L-M2.10-a-program-with-no-fixpoint-costs-the-bounded-call-its-clock-once-a-saturation.md)) and its leak ([E-126](../evaluation/bounds/E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md), [Q-M2.34](../open_questions/kernel/Q-M2.34-does-einaf-s-kernel-call-set-the-lever-that-keeps-one-justification.md)); root alone on an unfinished program ([E-114](../evaluation/hypotheses/E-114-a-partial-program-s-h-is-set-by-its-relations-count-of-places.md)); *one* that is not one value ([E-124](../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md)); a kept context ([L-M2.8](../findings/model/L-M2.8-a-kept-context-and-a-whole-evaluation-of-one-prompt-part.md)) |
| [S2.4.2](s2.4.2_repair.md) | the principle's hard case ([Q-M2.41](../open_questions/formalizer/Q-M2.41-is-a-misread-statement-repaired-before-any-theory.md)); regenerate under the cut that stands ([L-M2.25](../findings/einaf/L-M2.25-the-steps-partial-parts-are-written-twice-never-revised-and-a-step-early.md)); the repairs nothing points at ([E-124](../evaluation/answer/E-124-one-state-may-answer-a-question-of-one-value-with-a-row-a-pass.md), [L-M2.18](../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md)) |
| [S2.4.3](s2.4.3_feedback_ladder.md) | the check's findings as lints ([S2.2d.3](../p2.2d_formalizer_corrected/s2.2d.3_entries_described.md), [S2.2d.4](../p2.2d_formalizer_corrected/s2.2d.4_check_findings.md)); the ladder by hand ([S2.2d.6](../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md), [L-M2.24](../findings/model/L-M2.24-the-model-s-zebra-program-has-seven-defects-and-three-repairs-leave-r-empty.md)); what the kernel's word does not say ([E-107](../evaluation/diagnostics/E-107-a-theory-s-missing-activating-fact-is-reported-as-the.md), [E-116](../evaluation/answer/E-116-a-query-with-no-row-does-not-say-which-of-its-premises-matched.md), [E-118](../evaluation/stdlib/E-118-a-numeral-written-with-two-digits-in-one-place-is-refuted-as-a-contradiction.md), [E-123](../evaluation/diagnostics/E-123-no-value-says-whether-a-selecting-fact-was-taken-up.md), [E-109](../evaluation/natural-language/E-109-a-sentence-written-as-a-rule-s-activating-fact-is-in-no.md)) and what it does ([E-125](../evaluation/diagnostics/E-125-a-dead-root-s-core-names-the-sentence-the-model-misread.md)) |
| [S2.4.4](s2.4.4_faithfulness.md) | what no check reads ([L-M2.18](../findings/model/L-M2.18-a-sentence-s-polarity-and-a-thing-s-kind-go-wrong-in-silence.md), [E-108](../evaluation/answer/E-108-a-clue-with-its-arguments-swapped-is-answered-one-with.md)); the unit's two questions ([Q-M2.22](../open_questions/answer/Q-M2.22-is-a-membership-no-sentence-states-a-faithfulness-event.md), [Q-M2.25](../open_questions/answer/Q-M2.25-is-a-clause-of-a-sentence-a-source-or-only-the-whole.md)); a refused program's categories ([L-M2.26](../findings/einaf/L-M2.26-a-program-the-kernel-refuses-carries-no-categories.md)); a right string for a wrong reason ([L-M2.22](../findings/model/L-M2.22-a-fixed-text-scored-right-can-rest-on-a-contradiction-of-the-model-s-own.md)) |
| [S2.4.6](s2.4.6_ein_as_a_tool.md) | the tool probed first ([S2.2c.3](../p2.2c_decoding_and_model_alone/s2.2c.3_the_tool_probed.md)); Ein written with no grammar; thinking that loops ([Q-M2.27](../open_questions/model/Q-M2.27-do-the-phase-s-thinking-runs-stay-greedy-when-greedy-thinking-loops.md)); a third arrangement ([Q-M2.40](../open_questions/einaf/Q-M2.40-is-the-model-reasons-and-ein-checks-its-answer-an-arm.md), [L-M2.29](../findings/model/L-M2.29-the-model-alone-solves-the-zebra-puzzle-in-its-thinking-and-names-it.md)) |

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.4.1](s2.4.1_state_machine.md) | The filter's state machine and the transition log | the states and an edge for each of the seven outcomes; the questions asked of the kernel, chosen by measurement; the transition record; the machine on the bounded kernel call; termination as seven predicates; Q-M2.4 decided; a run replayed without the model |
| [S2.4.2](s2.4.2_repair.md) | Repair as well as regenerate | the two prompts; the *unchanged* reply; a diff of the program a repair, in the record; how each strategy uses the kept context |
| [S2.4.3](s2.4.3_feedback_ladder.md) | The feedback ladder F0 to F8 | nine levels as sets of blocks over the feedback object; the difference of two states, the dependency analysis and the chains built in EinAf; each level re-aimed at what the kernel supplies; the kernel's diagnostics judged as feedback |
| [S2.4.4](s2.4.4_faithfulness.md) | Faithfulness | the mechanical judge always, the canonical judge where a canonical program exists, a human-judged sample, a model-assisted arm only with its agreement reported; the faithfulness events, in both arrangements; Q-M2.3 decided |
| [S2.4.5](s2.4.5_alternatives_as_hypotheses.md) | Alternatives as hypotheses | the corpus's pattern as a template the formalizer writes; the emission path; `readings` as a parameter of a run, for P2.6 to compare |
| [S2.4.6](s2.4.6_ein_as_a_tool.md) | Ein as a tool | an episode in which the model reasons in text and calls Ein when it likes: the function, what it returns, the episode's record with its calls counted, and whether the answer is what the last run printed |

## Acceptance

- **In EinAf's step of the gate, with a replayer in the model's place**: every acceptance line below but the
  next. **By hand, with the model and a record** (an experiment, in no tier): the filter runs the seed set at
  every level F0 to F8, regenerate and repair, and the tool runs it once.
- Every run is a transcript a reader replays without the model, with the same transitions and the same
  identities.
- Each of the seven outcomes is met by a fixture and takes its edge; each of the seven termination conditions is
  reached by a fixture run, and the transcript names which. A flagged fixture is never an ANSWER as *one*; a run
  the budget stopped, and a kernel call a bound stopped, end as *not decided*.
- The final record of every episode carries each reading of *one answer* that was asked, and none that was not.
- On a synthetic instance whose gold class is ambiguous, a loop at F2 that ends in *one* carries an
  `invented_constraint` event: the E3 failure is detected on the benchmark's own ground truth, before any table.
- The tool runs an episode end to end on a seed instance: a call with a malformed program gets a parse error
  back, a call with a valid one gets the level's rendering, an episode with no call is recorded as one, and the
  record says whether the final answer is what the last run printed.
- A level's rendering shows nothing the feedback object does not hold, and the filter and the tool render one
  object to the same text at the same level.
- [The evaluation of Ein](../evaluation/README.md) holds what the six stages found, and is shown.

## Risks

- **The ladder's content leaks the answer.** F4 shows how two states differ, and a formalizer that copies a
  state's facts into the program has repaired by looking. The renderer shows the difference and not the states,
  the judge reports the fact the text does not state, and G5 reports the rate.
- **The loop's stop rule selects its final programs.** A loop that stops on one reading of *one answer* hands the
  metrics programs chosen by that reading. The reading is a parameter of the run, and the experiment Q-M2.10
  waits for runs under each.
- **The two arrangements differ in more than who calls.** Thinking, the grammar, the prompt's knowledge of Ein
  and the level all could differ between B5 and B6. The defaults below hold all but the arrangement equal where
  they can, and each is a field of the record.
- **F6 is the one analysis that is new, and it is easy to overbuild.** It is reachability over a relation graph
  and nothing else; the engine's own graph is of the saturation rules alone and is no part of its contract
  ([`04-api.md` § 6](../../../docs/ein/ein.rs/04-api.md#6-what-is-not-the-contract)).
- **A model-written program is run by EinAf.** A saturation that never ends, a description of a cube the blind
  enumerator wrote: every call goes through the bounded call, and what the kernel itself could not bound is an
  entry of the evaluation.

## Defaults taken at the re-plan

Each is the re-plan's, recorded so that the user or the stage can correct it; none is a ruling.

1. **The loop's question is root alone, then `--single` over all of R**; the maximal line's word and a
   description are asked once, of the final program, for the record's three readings. S2.4.1 confirms or changes
   it by measurement.
2. **The reading the filter stops on is a parameter of the run** (all of R by default; the maximal line; and, if
   the seed set can pay for a description an iteration, the answer query's classes agreeing over a complete
   set), so that no table fixes *certified* before the user decides Q-M2.10.
3. **The kernel passes a program whose outcome is *one* and whose answer query has a row**, and the filter's
   answer is then what the program prints. Where an episode ends on a program the kernel does not pass, its
   answer is a fixed short text that names the outcome, EinAf's and not the model's, or none:
   [S2.2.5](../p2.2_formalizer/s2.2.5_from_text.md) words the texts, and the rule that scores an answer is
   [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)'s.
4. **A flagged program is repaired, not answered**: its sites are static diagnostics and stand at F1 and above,
   the word *flagged* at F2 and above. If the run ends flagged, what the kernel printed is recorded and is not
   the arm's answer.
5. **Every kernel call is bounded, and a call a bound stopped is *not decided*.** The bounded call is
   `einaf::kernel`'s, P2.1's to build, over the kernel's own bounds: the clock and a limit that reach inside a
   saturation, which the user's ruling on [Q-M2.16](../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md) lets the kernel have and S2.1.2 gives it. A worker process
   of EinAf's own is built only if those do not stop a case.
6. **A level is a set of blocks**: F0 to F3 each include the one below; F4, F5 and F6 each add one block to F3;
   F7 is F4 and F5 with the chains of the facts they name; F8 is every block. Any set of blocks is a value of
   the parameter, so that P2.6's pairs differ in one block.
7. **What is owed stands with the core at F5, and who wrote H with the counts at F3**: the research plan's ladder
   has no rung for either.
8. **F1 shows the findings of the contract's check**, which P2.2 builds and leaves to the ladder to show or not:
   marked as EinAf's, under a parameter, since in Ein an undeclared relation and a form at another count are no
   errors ([Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md),
   [Q-M1f.447](../../../docs/decisions/language/Q-M1f.447-a-relation-is-overloaded-by-its-count-of.md)). The
   check's finding of a `:source` that is no sentence of the text is the faithfulness judge's and is not shown.
9. **The episode is one conversation**: a repair is appended to the kept context with the whole history; *the
   last program and its feedback only* is the parameter's other value.
10. **The reply *unchanged* is the same program again**, so that the filter's output is Ein and nothing else.
11. **No finding of the faithfulness judge is fed back to the model.** The old `:source` pre-check is dropped.
12. **`readings` is a parameter of its own** (`commit`, `alternatives`), beside `strategy` and not a third value
    of it: it says what GENERATE writes, not what REPAIR is given.
13. **B6 is B1 with the function offered, and is told of Ein what B5 is told**: B1's thinking setting and answer
    line ([S2.5.2](../p2.5_harness/s2.5.2_baselines.md)); in the function's description, what the filter's prompt
    holds of Ein (the excerpt of the reference, the catalogue, the examples); F8 as the level; `:why` required as
    the kernel requires it.
14. **The tool's levels start at F1**: the ruling says a parse error is what the tool returns, and F0 would
    withhold it. A program the kernel passes is answered with what it prints at every level; of any other,
    a flagged one among them, the result says in one fixed line that no answer was printed, and the level says
    the rest.
15. **The tool's call is the model's native function call**, the program one string argument; a fenced block
    read as a call is the fallback if the stage measures that the model cannot quote a program.
16. **EinAf's tests of a renderer are over feedback objects written by hand and properties over the corpus**; a
    banked rendering is of a monotone program, and none banks a flagged program's answer.
17. **Where a stage's answer lands**: Q-M2.3 and Q-M2.4 are answers about EinAf, so each is written beside its
    question in [the ledger](../open_questions/README.md) and becomes a page of `docs/einaf/` with the code.

Taken 2026-11-26, when the formalizer's steps became the user's notes on the whole of P2.2
([§ The theory before the facts](#the-theory-before-the-facts)):

18. **A repair is told to change the theory before the facts**, in its prompt beside the research plan's
    instruction; nothing enforces it, and what a repair did to step 1's facts is counted.
19. **A step-1 statement a repair removed or changed is the note `rewritten_fact`**, counted per run and per
    level; `deleted_source_fact` stays the event, for a sentence left with no fact.
20. **The facts' faithfulness is judged at step 1** in the filter; the tool, which has no steps, is judged on each
    program whole.
21. **Regenerate runs the one shot's steps again**, the rendered feedback in each step's prompt; repair is one turn
    that writes the whole program, as [S2.4.2](s2.4.2_repair.md) has it.

## Connections

- [`EinAf.md` § Stage E](../EinAf.md#stage-e--build-the-iterative-formalization-loop),
  [§ Stage F](../EinAf.md#stage-f--make-symbolic-feedback-progressively-richer),
  [§ Stage D](../EinAf.md#stage-d--establish-baselines).
- [The frame](../README.md#the-plan-as-re-planned): the arms, the outcomes, where EinAf's things are.
- [`analysis/audit_p2.4.md`](../analysis/audit_p2.4.md): this phase's audit, claim by claim (§ A, what was
  stale; § B.0, the runs; § B.1, the ladder against the engine; § C, the standing rules; § D, what today's
  kernel asks that the old files did not); [`analysis/host.md`](../analysis/host.md): the model emitted a
  function call, and a grammar over a thinking model's whole output took the thinking.
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md): the loop `(Pᵢ, Tᵢ) → Ein → Dᵢ` and its semantic
  fixed point; [F16](../../followups/f16_autoformalization/ideas.md): the diagnostics a reasoner can return that
  a compiler cannot; [F17](../../followups/f17_formal_verification/ideas.md): `Spec₀ → verify → counterexample →
  Spec₁`.
- The followups a consumer like this one triggers: [F22](../../followups/f22_exact_core_past_the_cap.md), the
  exact core; [F24](../../followups/f24_written_provenance.md), written provenance in a core;
  [F23](../../followups/f23_diagnostics_in_eins_words.md), what a load says and leaves silent;
  [F26](../../followups/f26_consistency_and_completeness.md), the classes of an answer.
- The neighbours that do not start by this phase: [M15](../../m15_mcp_server/README.md), an MCP server, another
  way a model would call Ein; [M1p](../../m1p_ein_to_natural_language/README.md), a fact as a sentence;
  [M1n](../../m1n_non_monotone_programs/README.md), what a flagged program's answer is.
- [`04-api.md`](../../../docs/ein/ein.rs/04-api.md): the values an answer is;
  [`08-read-out.md`](../../../docs/ein/user/04-reference/08-read-out.md): each row and key;
  [`07-diagnostics.md`](../../../docs/ein/user/04-reference/07-diagnostics.md): the refusals;
  [`05-events.md`](../../../docs/ein/ein.rs/05-events.md): the event stream.
