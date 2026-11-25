# P2.2 — The formalizer (Stage B)

**Depends on:** [P2.1](../p2.1_kernel_as_instrumentation/README.md):
[S2.1.1](../p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md) creates `einaf/`, the Cargo package every
stage here adds to, and writes the operational test of the seven outcomes;
[S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md) gives a refusal of the kernel a kind and a
place, as values; [S2.1.3](../p2.1_kernel_as_instrumentation/s2.1.3_feedback_object.md) is the feedback object
`einaf from-text` reads the kernel through, and the bounded call every stage runs a generated program by. The
language (Rust,
[Q-M2.7](../open_questions.md#q-m27--what-language-is-the-frontend-written-in)), the model (`qwen3.5:4b`,
[Q-M2.5](../open_questions.md#q-m25--local-model-choice)) and what runs it (a Rust binding to llama.cpp,
[Q-M2.14](../open_questions.md#q-m214--what-runs-the-model)) are the user's rulings: no stage here decides one.
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

## Goal

The neural component does **one difficult thing**: it turns a problem stated in English into an Ein program, one
shot, under a written contract, and the kernel answers. It does not solve. Repair is
[P2.4](../p2.4_loop/README.md)'s and the benchmark is [P2.3](../p2.3_benchmark/README.md)'s. The phase ends with
`einaf from-text`: a text becomes a program and **the short answer that program prints**, with the run's record
([S2.2.5](s2.2.5_from_text.md)). The command is EinAf's and the kernel stays free of any model
([the frame, § Ein, ein.rs and EinAf](../README.md#ein-einrs-and-einaf)).

The phase also takes the milestone's cheapest early measurement: **whether the model writes loadable Ein with no
grammar at all** ([S2.2.2](s2.2.2_model_runtime.md)), which the user expects may be possible
([Q-M2.13](../open_questions.md#q-m213--who-calls-the-kernel)) and which bears on every stage after it.

## What the formalizer emits

Research plan B1 allows *an Ein program* and, optionally, a decomposition first. The decomposition here is the one
the kernel's own input has, with two parts the plan of August did not carry: the hypothesis mechanism, which the
answer depends on, and the answer query's `:goal-text`, which is how the program prints its answer
([Q-M2.12](../open_questions.md#q-m212--what-prints-the-answer)).

| part | what it is, in today's Ein | what the kernel holds it to |
|---|---|---|
| **ontology** | `(relation NAME LABEL* :why "…")` forms, and membership facts of a relation the program chooses | a `:why` on every relation, or the load refuses ([§ 2.10](../../../docs/ein/user/04-reference/07-diagnostics.md#210-a-form-with-no-why)); a label is no type and no form is checked against one ([Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)) |
| **theory** | `(import MODULE :symbols (NAME+))` and the facts that activate what was imported; a `rule` with its `:why` only where the catalogue has nothing | **nothing, where the spelling is wrong**: a short name the file lists from no import is the file's own, and the fact is inert with no message ([`04-stdlib/` § What to list](../../../docs/ein/user/04-reference/04-stdlib/README.md#what-to-list)) |
| **hypothesis mechanism** | what proposes the candidates: the library's obligations, a rule that concludes `(open F)`, a written `(open F)`, `(solve :no-search)`, or a `(solve :hypothesis-relations (…))` list for the blind enumerator | the mechanisms may not contradict one another ([§ 2.26](../../../docs/ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)); **a program that states none is answered by the blind enumerator**, in silence ([hypotheses, the writers](../../../docs/ein/user/03-syntax-and-semantics/09-hypotheses.md#where-candidates-come-from-the-writers)) |
| **instance** | `(R a b :source "…")` and `(not (R a b) :source "…")`, the text of each `:source` the sentence it states | a fact takes `:source`, `:rule` and `:using` and no other keyword ([Q-M1e2.28](../../../docs/decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md)); names are ASCII; a number is a name, and a quantity that is computed is a numeral of the calculator, `(n z 5)` |
| **query** | `(query :goal PREMISE :goal-text "…")`: **the answer query**, the one query that carries a `:goal-text`, a sentence that says the answer in the problem's words, as every arm is asked for one ([Q-M2.12](../open_questions.md#q-m212--what-prints-the-answer)); any other query is a check | a goal is held to what a rule's `:match` is held to, and names only relations the program writes ([§ 2.21](../../../docs/ein/user/04-reference/07-diagnostics.md#221-a-goal-is-held-to-what-a-rules-match-is-held-to)) |

Four things follow, and each is a stage's subject.

**The theory part is selection before synthesis, and selecting is spelling.**
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

[`examples/zebra.ein`](../../../examples/zebra.ein), the `std.slots` formulation, **less its one rule (`guess`),
that rule's five activators and its `(config …)`**, is the shape the contract describes: two imports with
`:symbols`, five `relation` forms with `:why`, six library activators, the `type` and `instance` facts, eighteen
condition facts with `:source`, one `query` with its `:goal-text`. It has no `rule` form, the library's
obligations write H (`ein show hyps`: *H, from `obligations`: 112 candidates*), and `ein solve --single` answers
`Solution` with *“The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”* (the audit's run
s22, run again at this re-plan). [`examples/zebra2.ein`](../../../examples/zebra2.ein), which the plan of August
named as the program to reproduce, is not of that shape: it carries fifteen rules of its own, and twelve of its
condition facts are activators of them
([the audit, B.0](../analysis/audit_p2.2.md#b0-the-four-passes-in-todays-language)). The word problems of
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
| the contract and what the model is given ([S2.2.1](s2.2.1_contract.md)) | the prompt of every pass | the tool's function description, which carries what the filter's prompt holds of Ein ([S2.4.6](../p2.4_loop/s2.4.6_ein_as_a_tool.md)) |
| the grammars ([S2.2.3](s2.2.3_gbnf.md)) | the filter's mechanism: the whole output of a call held to Ein | none: the model writes Ein as it can |
| the passes ([S2.2.4](s2.2.4_passes.md)) | GENERATE | none |
| the answer and its formatters ([S2.2.5](s2.2.5_from_text.md)) | what the program prints | what the tool's last run printed, where the record compares the two |

## Stages

| ID | title | ends with |
|---|---|---|
| [S2.2.1](s2.2.1_contract.md) | The formalization contract: B1 to B4 | one page, `docs/einaf/contract.md`: the input, the five-part output with the hypothesis mechanism and the answer query, what a flagged program is, the categories with which of them is a faithfulness event, what the model receives, what is recorded |
| [S2.2.2](s2.2.2_model_runtime.md) | The model's runtime, and whether the model writes Ein | the binding chosen and built for the GPU, the model's file pinned by its SHA, the envelope every call returns, the binding held to `llama-server` on the same file; **the probe**: parse rate, load rate and the kinds of error of Ein written with no grammar |
| [S2.2.3](s2.2.3_gbnf.md) | The grammars | a GBNF grammar generated from `ein.l` and `ein.y`, held to the parser in both directions; the restricted grammars, parameterised on a program's own vocabulary; where a grammar begins and how an output ends; [Q-M2.6](../open_questions.md#q-m26--one-gbnf-or-many) decided |
| [S2.2.4](s2.2.4_passes.md) | The passes: ontology, theory, instance, query | the catalogue of the library, each entry run by the kernel; four passes with their prompts; the theory pass selecting, stating the hypothesis mechanism and synthesising only on a named condition; the contract's check on the assembled program; [Q9](../open_questions.md#q9--ontology-provenance) decided |
| [S2.2.5](s2.2.5_from_text.md) | From a text to a short answer | `einaf from-text`, one shot, the kernel run through EinAf's bounded call; the answer formatters, written to be [M1p](../../m1p_ein_to_natural_language/README.md)'s starting point; the fixed texts EinAf answers with where no program prints an answer; the record per run; the Zebra puzzle and the word problems from their texts to their answers |

### Order

S2.2.1 and S2.2.2 do not wait for each other. **S2.2.2's probe is the phase's first measurement**: it needs the
package ([S2.1.1](../p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md)), the kernel's diagnostics as
values ([S2.1.2](../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md)) and the model, and nothing of
the contract but the shape its examples show. It runs before a grammar is built, because its result says what a
grammar would have prevented. S2.2.3 follows the probe; S2.2.4 follows S2.2.1 and S2.2.3; S2.2.5 closes the phase.

**How every stage reaches the kernel.** One module of EinAf calls the engine's crates, `einaf::kernel`
([S2.1.3](../p2.1_kernel_as_instrumentation/s2.1.3_feedback_object.md)'s acceptance), and a program is run only
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
- A record replays: the same fields give the same program, or the record names the field that moved.
- A program whose saturation has no fixpoint comes back *not decided* within the bound of EinAf's kernel call,
  from the probe on.
- No prompt holds a benchmark instance's solution: every corpus program a prompt quotes is listed in the record,
  and the list is handed to [S2.3.4](../p2.3_benchmark/s2.3.4_splits.md) as out of every split.
- The probe's record exists, and [Q-M2.6](../open_questions.md#q-m26--one-gbnf-or-many) and
  [Q9](../open_questions.md#q9--ontology-provenance) are decided in the ledger, each with the measurement it
  rests on.
- [The evaluation of Ein](../ein_evaluation.md) holds what each stage found of the kernel, and is shown at the
  phase's close.

## Risks

- **A model of 4.2B parameters may not select a theory in one shot.** The Zebra acceptance asks for `std.slots`
  spelled right, an activator of five arguments among its six, and eighteen facts, from a model that was never
  trained on Ein. If it is not met, the phase reports the nearest miss pass by pass and the one-shot number stands as
  baseline B2: a low B2 is a finding, the loop ([P2.4](../p2.4_loop/README.md)) is where the model gets a second
  chance, and whether the gate's first criterion is held as written is the user's at the Level B gate.
- **The failures that matter are silent.** An activator under an unlisted name, a program with no mechanism, a
  fact of an undeclared relation and a form at another count all load and answer something
  ([the audit, runs s04, s11, s15](../analysis/audit_p2.2.md#snippet-runs-the-table-sections-a-to-c-cite)). A
  grammar makes none of them impossible but the vocabulary ones. The contract's check
  ([S2.2.4](s2.2.4_passes.md)) is EinAf's own and is what tells them apart from a formalization that worked.
- **A closed-world reading is one `absent` away from a flagged program.** *The youngest is the one who is older
  than nobody* is flagged where a commitment can add an `older` fact and is monotone under `(solve :no-search)`
  ([S2.2.1](s2.2.1_contract.md) has both runs). A formalizer will write such programs; a flagged program is an
  outcome of its own and never certified
  ([Q-M2.11](../open_questions.md#q-m211--what-is-a-flagged-program-to-the-loop)).
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
- [Idea 04](../../ideas/04-nlp-to-graph-to-solver-pipeline.md), the pipeline sketch;
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

1. **The model of the target shape is `examples/zebra.ein`** less its rule, its activators and its config, not
   `examples/zebra2.ein`; the Zebra acceptance accepts either formulation, since it is stated on the answer.
2. **The hypothesis mechanism is a part of the contract of its own**, stated by the theory pass. A program that
   states none is recorded as the kernel reports it, `blind`, and is not refused.
3. **A category is computed, not written**: by EinAf, from the program, the input text and the catalogue, and kept
   in the record keyed by the form's canonical text. A fact outside the library's with no `:source` is a
   generated assumption; the forms B2 has no word for are rows of the contract's table.
4. **The answer query is the one query that carries a `:goal-text`**, and any other query is a check: the
   benchmark and the metrics read one ([P2.3](../p2.3_benchmark/README.md#an-instance),
   [S2.5.3](../p2.5_harness/s2.5.3_metrics.md)). Its `:goal-text` is a sentence that says the answer, as the user
   ruled ([Q-M2.12](../open_questions.md#q-m212--what-prints-the-answer)). A program with none has no answer; with two, the first is read and the check reports it. The model
   writes every `:why`, and EinAf never loads with the `:why` requirement off.
5. **Texts are English** (the frame's default 9): the Russian half of the old plan is dropped.
6. **S2.2.1 and S2.2.2 are independent**, and the probe is the phase's first measurement.
7. **This phase's own runs are greedy** (temperature 0), so that one run is the whole distribution. Which
   sampler chain the arms run with is [S2.5.2](../p2.5_harness/s2.5.2_baselines.md)'s, whose default is the
   model's own; the one shot runs under it unchanged.
8. **The reference run of `llama-server` is by hand, from a recorded request**; a Rust client of it is written
   only if the fallback is taken.
9. **The probe's items** are a few small texts and the sixteen word problems over the library alone, which is
   the main arms' condition, and the sixteen again with the word theory given, which is the library ablation's
   widest and is labelled so; no item is shown an example of its own text family; thinking off and on; the
   output read whole and by its first fenced block.
10. **The grammar is held to the parser in both directions** offline, and to llama.cpp's own sampler over the
    corpus by hand; the old echo round-trip through the model is dropped. No speed target is inherited.
11. **A grammar begins at the first token, thinking off**, in the arms; the start after the thinking is built for
    [P2.6](../p2.6_ablations/README.md)'s ablation.
12. **The catalogue is S2.2.4's**, each entry a program the kernel runs, and **holds the library's modules
    alone**: a family's own module is its canonical program's, is not shown to the model in the main arms, and
    is offered only under the widest condition of the library ablation
    ([S2.6.4](../p2.6_ablations/s2.6.4_representation_ablations.md)).
13. **The passes are the formalizer; the whole program in one call is kept as a mode**, and both are recorded.
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
24. **The contract's check is built once, in [S2.2.4](s2.2.4_passes.md)**; the feedback ladder's lints
    ([S2.4.3](../p2.4_loop/s2.4.3_feedback_ladder.md)) are its findings given a sentence each.
25. **The contract's table says which category is a faithfulness event**
    ([S2.4.4](../p2.4_loop/s2.4.4_faithfulness.md)): only a fact stated as holding with no sentence behind it.
    What a program needs to be well formed or to state its lattice (a declaration, an import, a `(solve …)`
    form, a closure, a written `(open F)`) is never one.
26. **This stage writes the answer lines its own runs need** (each asks for a sentence: how many, for the word
    problems; who drinks water and who owns the zebra, for Zebra), since
    [S2.5.2](../p2.5_harness/s2.5.2_baselines.md), which owns them, comes after it.
27. **The probes of this re-plan are in the files' own text** (the contract's worked example in three forms, the
    five forms with no mechanism, the rule with no fixpoint, a word problem over the calculator): this re-plan wrote only into its own folder, so
    none is banked under [`analysis/probes/`](../analysis/README.md) yet.
