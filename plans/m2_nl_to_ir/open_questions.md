# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

Two id forms share this file. **Q7–Q11** are sticky ids from the cross-milestone pool, assigned when the
milestone was *NL → IR*; they keep their numbers. **Q-M2.n** is the milestone-scoped form, which
[M10](../m10_external_benchmarks/open_questions.md) uses too: Q-M2.1–4 arrived with the 2026-08-23 reshape around
[`EinAf.md`](EinAf.md), Q-M2.8–11 with the analysis M2 started from, 2026-11-25
([`analysis/`](analysis/README.md)), Q-M2.12–13 with the user's answers of that day, Q-M2.14–15 with the
answers that followed them, Q-M2.16 with the re-plan, and Q-M2.17 with the user's answers to the re-plan.

**Q-M2.5, Q-M2.6 and Q-M2.7 were this file's Q23, Q24 and Q25 until 2026-11-25.** Those three ids were already the
root ledger's, given to questions of M1's, and two of them are decision files under `docs/decisions/` (a premise
says what a variable ranges over; Ein has no ordinal form). An id is never reused, so M2's three took the next free
ids of the milestone, and every file of this folder that cited them cites the new ones.

**How it is kept.** The open questions come first and the user's rulings after them. A ruling is the user's words
as given, then what the coordinator reads them to mean, **which the user may correct**; the options a ruled
question weighed are not kept here (`git show 5d678042:plans/m2_nl_to_ir/open_questions.md` has those of the
analysis, `git show 6a3c9a75:plans/m2_nl_to_ir/open_questions.md` those of Q-M2.12 and Q-M2.13).

**Where an answer lands.** An answer about Ein is a file under `docs/decisions/`, under the id of its question,
written with the change that makes it true; an answer about EinAf or about how the work is done stays here, beside
its question. EinAf is a project of its own, outside Ein ([Q-M2.8](#q-m28--where-does-einaf-live)), so an answer
about the harness is the second kind.

## Index

### Open

| Q | title | decided |
|---|---|---|
| [Q7](#q7--llm-as-surface-generator) | Is the surface generator (NL output) allowed to be an LLM? | **outside the research question**, parked with its working answer; the answer's own words are ruled, [Q-M2.12](#q-m212--what-prints-the-answer) |
| [Q8](#q8--ambiguous-parses) | Where do ambiguous NL parses go — branched on the IR, or rejected? | [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md), by experiment; the form is settled, a hypothesis `(open F)` (the user, 2026-11-25) |
| [Q9](#q9--ontology-provenance) | Per-puzzle declared ontology vs ontology inferred from text | [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) — reframed as theory *selection* |
| [Q10](#q10--direct-llm--constraint) | When is direct LLM → constraint emission acceptable (skip the IR)? | [P2.8](p2.8_representations/README.md) — Stage L is this question asked properly |
| [Q11](#q11--link-grammar-value) | Does link-grammar enrich LLM input usefully, or is it dead weight? | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), by experiment |
| [Q-M2.3](#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory) | What is the unit of faithfulness without a gold theory? | [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) |
| [Q-M2.4](#q-m24--is-the-fixed-point-syntactic-or-semantic) | Is the fixed point syntactic or semantic? | [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) |
| [Q-M2.6](#q-m26--one-gbnf-or-many) | One GBNF grammar per task, or one grammar for everything? | [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) |
| [Q-M2.10](#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) | What does *Ein certifies a unique model* mean on R? | **the user, after experiments** (*"it depends on problem shape"*) |
| [Q-M2.15](#q-m215--where-does-m2-stop-for-the-next-iteration-of-ein) | Where does M2 stop for the next iteration of Ein? | **the user**, at a level's gate; until then every gate is a stop |

### Ruled by the user

| Q | title | the user, 2026-11-25 |
|---|---|---|
| [Q-M2.1](#q-m21--when-is-the-kernel-frozen) | When is the kernel frozen? | *"use current state - after M1f closed"* |
| [Q-M2.2](#q-m22--must-the-model-be-local) | Must the model be local? | *"yes"* |
| [Q-M2.5](#q-m25--local-model-choice) | Which local model? | *"Qweb3.5:4b"*, read as `qwen3.5:4b` |
| [Q-M2.7](#q-m27--what-language-is-the-frontend-written-in) | What language is the frontend written in? | *"Rust"* |
| [Q-M2.8](#q-m28--where-does-einaf-live) | Where does EinAf live? | *"./einaf/, ./docs/einaf/, etc..."*, then *"own tree, in this repository, outside of ein … but inside the development process"*, then *"One tree - yes, preferred, docs/einaf/ - separately"* |
| [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics) | Does the kernel grow a machine surface for diagnostics? | *"(b)"*: it does, in its own terms |
| [Q-M2.11](#q-m211--what-is-a-flagged-program-to-the-loop) | What is a flagged program to the loop? | *"(a)"*: an outcome of its own, never certified |
| [Q-M2.12](#q-m212--what-prints-the-answer) | What prints the answer? | *"don't run M1p now, use as much of existing ein NL features … use additional external formatters"*; the answer is *"a sentence, like zebra's"*; EinAf's fixed texts where no program prints one, *"OK"* |
| [Q-M2.13](#q-m213--who-calls-the-kernel) | Who calls the kernel: the harness, the model, or both? | both: the tool, *"recommened but optional"*, and the filter, *"forced to write ein code"* |
| [Q-M2.14](#q-m214--what-runs-the-model) | What runs the model? | *"llama-cpp with rust wrapper, not ollama"* |
| [Q-M2.16](#q-m216--what-of-the-kernel-may-change-inside-m2) | What of the kernel may change inside M2? | *"we can change kernel without changing ein language syntax and semantics … bug fixes allowed"* |
| [Q-M2.17](#q-m217--what-is-the-milestones-minimal-goal) | What is the milestone's minimal goal? | the model, *"with help of Ein or through Ein"*, solves the Zebra puzzle: *"this is a minimal goal"* |

The stage files are re-planned to these rulings (the README's [§ The plan as re-planned](README.md#the-plan-as-re-planned)):
no stage still decides a question that is ruled here.

---

## Q7 — LLM as surface generator?

Per [idea 08 §Open questions point 4](../ideas/08-human-style-deductive-trace.md#open-questions).

**Options:**

- **A**: Pure templates. Deterministic. Verifiable but stilted.
- **B**: LLM polish on top of templates. Drift risk.
- **C**: LLM end-to-end from `TraceStep` to prose. Most natural;
  worst verifiability.

**Working answer**: A for the *reasoning* layer (no LLM mid-proof);
B for the surface narration (only at the end, with a "render-only"
prompt that cannot change the rule firings). C never.

**Status after the reshape (2026-08-23):** the question is real and is not
this milestone's. The research plan's loop ends at *verified answer*
([`EinAf.md` § Stage Q](EinAf.md#stage-q--public-research-artifact)) and the
trace → NL direction appears nowhere in Stages A–Q; the old plan carried it as
a "stretch goal". It is the natural last slide of
[M5](../m5_presentation/README.md)'s demo and a thing
[M20](../m20_gui/README.md) would display, and it is parked here with its
working answer until one of them asks for it. The engine's `:why` templates
already render every fact and the headline (rendering A, shipped); what is
open is only B.

**Since (the user's answers, 2026-11-25).** The user's statement of the core idea
([README](README.md#the-two-main-ideas-the-user-2026-11-25)) asks that *"the final test problem answer must be short and
must be produced by ein program NL output"*. That is rendering A, the engine's own templates, and it puts the
answer's words inside the research question; what stays parked here is B, an LLM's polish. Ein to natural language
is [M1p](../m1p_ein_to_natural_language/README.md)'s, which does not run now: the answer is what Ein prints today
with EinAf's formatters over its values, the user's ruling on [Q-M2.12](#q-m212--what-prints-the-answer).

## Q8 — Ambiguous parses

Per [idea 04 §Multiple-variant complication](../ideas/04-nlp-to-graph-to-solver-pipeline.md#multiple-variant-complication)
and [idea 04 §Open questions point 3](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: branched on the IR. The NL frontend emits all plausible parses, and the engine treats each as
it treats any other hypothesis.

**The user, 2026-11-25:** *"there is no `hrule` anymore, rules emitting open facts replaced them"*.

So the form is settled, and it is nothing new: an alternative is a hypothesis, `(open F)`, which a rule concludes
([Q-M1f.567](../../docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)) or the
program writes at root. The `(hypothesis-parse ?id …)` wrapper this entry proposed, its hypothesis levels and the
`hrule` it named are gone. The corpus already encodes a sentence with two readings this way, each reading a
candidate and root owing one of them ([`readings.ein`](../../examples/word_problems/readings.ein),
[`boxes12.ein`](../../examples/word_problems/boxes12.ein), [`boxes13.ein`](../../examples/word_problems/boxes13.ein);
[Q-M1f.332](../../docs/decisions/corpus/Q-M1f.332-a-word-with-two-readings-always-has-one-chosen.md)).

**What is still open is the experiment the reshape added.** A reading the formalizer could not resolve may be
emitted as alternatives and pruned by the kernel (the working answer), **or** committed one way and caught by the
loop when the kernel answers several states or none and the feedback names the sentence
([S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md)). These are two strategies for the same sentence, and which one the
loop should prefer is measured, not decided: [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) is the stage
and [P2.6](p2.6_ablations/README.md) is where the two are compared. S2.4.5 no longer has a form to choose.

## Q9 — Ontology provenance

Per [idea 04 §Open questions point 2](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Options:**

- **A**: Per-puzzle declared ontology — user supplies/curates a
  small `ontology.ein` alongside the text.
- **B**: Inferred from text — LLM extracts type declarations.
- **C**: Library of standard ontologies (zebra, sudoku, einstein)
  + override.

**Working answer**: C for the three demo puzzles in M2's
acceptance; B for everything else, with the LLM's inferred
ontology subject to user review before solving. A always available
as escape hatch.

**Reframed by the reshape.** The question was asked about *ontologies* —
types and instances — and the answer turned out to be about *theories*. What
a Zebra text leaves implicit is not that there are houses and colours (the
LLM reads that off the text) but that the houses are linearly ordered and
every attribute is a bijection onto them; that is a theory, the stdlib
already has it (`std.bijection`, `std.slots`, `std.algebra`), and the
formalizer's job is to **select** it by asserting `(bijective pet-loc)`
rather than to write it ([F12](../followups/f12_rules_and_relations/ideas.md),
[F16](../followups/f16_autoformalization/ideas.md)). Option C is therefore
the stdlib catalogue in the prompt (the plan's B3), option B is the ontology
pass, and option A is `--ontology PATH` on `einaf from-text`, kept as the
escape hatch. "User review before solving" does not survive: the loop's
reviewer is the kernel, and a human in the loop is the one thing Stage D's
matched-budget comparison cannot price. [S2.2.4](p2.2_formalizer/s2.2.4_passes.md).

## Q10 — Direct LLM → constraint?

Per [idea 04 §Open questions point 4](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: never the *default*, and with the SMT milestone
dropped (2026-08-18) there is no in-repo solver path to compare against
either. If the question is revisited it is as a *diagnostic*: prompt the
LLM to emit constraints for an external solver and check that its answer
agrees with the IR pipeline's; any difference is a bug in the pipeline.

**Re-homed to Stage L (2026-08-23).** The research plan asks this question
in the form it deserves — *which representation is easiest for an LLM to
synthesise, which produces the most useful repair diagnostics, which detects
semantic incompleteness, which yields the highest verified-answer precision,
which needs the least generated text* — with Ein, an SMT encoding, Datalog,
general-purpose code and a proof assistant as the candidates. That is
[P2.8](p2.8_representations/README.md), and it is an experiment about
*targets*, not a back-end: the no-solver-back-end decision
([Q2](../open_questions.md#q2--when-does-the-graph-engine-hand-off), M3
dropped) stands, and [M10](../m10_external_benchmarks/README.md)'s
hand-written encodings are the seed. The old working answer's *diagnostic*
use — agreement between two targets as a check on either — is one of P2.8's
readings.

## Q11 — Link-grammar value

The user's open question: *"does feeding link-grammar output to
the LLM enrich the input usefully?"*

**Working answer**: unknown. The experiment runs with a small
benchmark, with metric = correctness of generated IR on a gold set.
Default deployment is no-link-grammar unless it ships a measurable
improvement.

**Where it runs now:** [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md),
as one arm of the representation ablation (the plan's G8 asks the same
shape of question about the *output* side — direct Ein vs a decomposed
intermediate — and this is the *input* side), with the pre-registered
decision rule the old S2.5.2 wrote. The metric is no longer "IR-F1 on five
puzzles": it is constraint precision / recall on the benchmark's synthetic
families, where the gold theory is exact. The submodule note does not stand: no
submodule is registered and none is needed, since [`nl2smt/`](../../nl2smt/README.md) drives the installed
`link-parser` by its command line, and the stage does the same.

## Q-M2.3 — What is the unit of faithfulness without a gold theory?

**Arrived 2026-08-23 with the reshape.** On a synthetic instance the
generator wrote the canonical theory, so faithfulness is arithmetic:
constraint precision and recall against it (H2), an invented constraint is a
false positive, a dropped one a false negative. On an external instance —
BBH, CLUTRR, FOLIO — there is a gold *answer* and no gold theory, and a
theory that reaches the gold answer by an invented constraint is exactly the
E3 failure the plan warns about and exactly what answer accuracy cannot see.

**The candidates:** (a) the B2 provenance categories make it partly
mechanical — a fact whose `:source` span does not exist in the text, or a
rule whose category is *generated assumption*, is flagged regardless of the
answer; (b) a human-judged sample, sized in [P2.7](p2.7_failure_scaling_generalization/README.md);
(c) an LLM-as-judge with the source and the theory, validated against (b)
on the sample before it is trusted on the rest; (d) back-translation of the
theory to NL and an NLI check against the source, which the autoformalization
literature uses ([F16](../followups/f16_autoformalization/ideas.md) cites
Lean Workbook doing exactly this). **[S2.4.4](p2.4_loop/s2.4.4_faithfulness.md)
decides**, and the recommendation is (a) always, (b) as the ground truth for
the sample, and (c) or (d) only with their agreement to (b) reported.

## Q-M2.4 — Is the fixed point syntactic or semantic?

**Arrived 2026-08-23 with the reshape.** The research plan's E4 terminates
on *repeated identical theory* and detects cycles *by canonicalized program
hashes* — syntactic. The user's own note says the fixed point should be
*semantic stabilisation*, `Cl(Tᵢ₊₁, Pᵢ₊₁) = Cl(Tᵢ, Pᵢ)`
([F13](../followups/f13_puzzles_beyond_zebra/ideas.md)), because an LLM can
rewrite equivalent rules forever without the closure moving.

**The engine can do both, today.** `ein-ir` has a canonical dump, which is the
syntactic hash; and [`06-solution-semantics.md` § 1.5, the state class](../../docs/ein/reasoning/02-kb-evolution/06-solution-semantics.md#15-commitment-node-state)
pins that *canonical state identity is the sorted fact list itself*, which
makes the saturated state's digest — or the `.einb` of a saturated KB — the
semantic one, and the summary's sorted `solutions` the model-set one. So the
question is not which is computable but which the loop *stops on*.
**Recommendation, to be decided in [S2.4.1](p2.4_loop/s2.4.1_state_machine.md):**
the syntactic hash is the cheap first check and catches the literal
resubmission; the semantic digest is the termination criterion; and both are
logged per transition, because the *difference* between them — a theory that
changed its text and not its closure — is itself a measurement of how much
repair is cosmetic.

**Since (the re-plan, 2026-11-25).** Three things this entry leans on are not as it says ([`analysis/audit_p2.4.md`](analysis/audit_p2.4.md)): the reasoning page says a digest never decides a state's
identity, the summary has no `solutions` key, and `.einb` is a private cache, never something to compare by. EinAf
links the library and has a program's canonical dump and a state's facts as values, so the two identities are
compared as texts and lists. [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) keeps the question and gives *semantic*
three candidates: root's fixpoint, the answer, and the answer as printed; its default is the second.

## Q-M2.6 — One GBNF or many?

**Since (the analysis, 2026-11-25).** This was Q24 here. The grammar's source is not the page the plan names:
`01-grammar.md` holds pictures and one table. The text is `ein.l` and `ein.y` (LR(1), 101 rules), or the twenty
generated EBNF diagram sources. Reserved words out of `NAME`, and the ends of tokens, are still hand work.

**Working answer**: many — one GBNF per pass (ontology, theory, facts,
query; the alternatives wrapper if S2.4.5 keeps it). Smaller grammars
decode faster and let the prompt focus the LLM. Decided in
[S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md), which also measures the one
thing that could reverse it — decoding speed under the full ein-lang
grammar.

**Since (the host checks and the user's answers, 2026-11-25).** A grammar is llama.cpp's: `llama-server` honoured
one on the model's file, and ollama takes none ([`analysis/host.md`](analysis/host.md)), so the model runs
through a binding to llama.cpp ([Q-M2.14](#q-m214--what-runs-the-model)). **The grammar is the filter's**: the
tool writes Ein with none ([Q-M2.13](#q-m213--who-calls-the-kernel)), and the user expects that a model may
write Ein without one, which [S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md) measures before a grammar is
built. A grammar over the whole output of a thinking model took the thinking too: where the grammar begins is
part of [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md)'s design.

## Q-M2.10 — What does *Ein certifies a unique model* mean on R?

**Raised 2026-11-25 by the analysis** ([§ 2](analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk),
[§ 5.4](analysis/README.md#5-what-the-standing-rules-now-ask-that-the-plan-does-not-answer);
[`audit_p2.4.md`](analysis/audit_p2.4.md), [`audit_p2.5_p2.6.md`](analysis/audit_p2.5_p2.6.md) B.4). It defines the
milestone's two central numbers, `P(correct | certified)` and `P(faithful | certified)`.

**The user, 2026-11-25:** *"it depends on problem shape, lets make experiments and then decide"*. So it is
**open, and the user decides it after experiments**; the recommendation this entry carried is withdrawn.

The plan writes it `k = 1 ∧ exhausted`. Neither field exists. On R there are three things it could be:

| | the question asked | what it says of a program that states no obligation |
|---|---|---|
| **(a) one state in R** | `ein solve --single` answers `Solution`, decided (7 saturations on `zebra2`) | `Ambiguity`: ∅ and every alive partial commitment are in R (`features/11_expect_ambiguity.ein`: two full seatings, `k_s = 7`) |
| **(b) one maximal state** | `--maximal --single` answers `Solution` | `Solution` where exactly one full assignment survives, whatever the program owes |
| **(c) a description** | a bare solve, `k_s = 1 ∧ exhausted` | as (a), at the cost of describing R (120 saturations on `zebra2`; on the clue-dropped variant about 10 s, stopping at the default budget with `exhausted false` after all 32 states are found) |

Under (a) and (c) a formalizer that writes every clue and no theory of what must be filled is told *several
states*, and has to select the theory that owes (`std.slots`, a `total` relation); under (b) the kernel reads
past the omission.

**What the experiment needs, as the coordinator reads it** (its design is the re-plan's): no table fixes
*certified* before the user decides. Every record carries each reading it can take, per instance, beside the
instance's family and shape: the word `--single` says, the word `--maximal --single` says, and where a description
is taken, `k_s`, `k_c` and `exhausted`. The two conditional probabilities are computed under each reading, per
family, and that table is what the user decides on. A flagged program is certified under none
([Q-M2.11](#q-m211--what-is-a-flagged-program-to-the-loop)).

**Since (the re-plan, 2026-11-25).** Two things the re-plan found belong to this decision, and are kept open with
it. **A fourth candidate reading**: the answer query's classes agree over R, which is what a program with several
states and one answer has ([`boxes12.ein`](../../examples/word_problems/boxes12.ein)); the plan records it beside
the other three. **The answer rule**, which says what short text an outcome is scored as
([S2.5.3](p2.5_harness/s2.5.3_metrics.md)), takes three defaults that set the accuracy of the arms that run Ein on
ambiguous and unsatisfiable instances: several states with one answer are scored as one answer; one state whose
answer query has no row is scored as ambiguous; and `Open` is no answer, not *unsatisfiable*. The rule is a
parameter of the evaluation, so the same records are scored again under another.

## Q-M2.15 — Where does M2 stop for the next iteration of Ein?

**Raised 2026-11-25 by the user's second main idea**
([README](README.md#the-two-main-ideas-the-user-2026-11-25)): *"evaluate ein.rs in this auto-frmalization
application, find/highlight it's weaknesses, strength, gaps, bugs and limits, and then go to next iteration of
ein/ein.rs development with this feedback"*. **The user's, when there is something to decide on.**

The plan reaches Level D in ten phases. The idea says the milestone is also a measurement of Ein, after which Ein
is developed again; it does not say after which phase. What the plan does by default:

- [the evaluation of Ein](ein_evaluation.md) is kept from the first stage on, every stage adding what it found,
  and is shown at each phase's close;
- **each level's gate is a stop**: at Level B ([P2.3](p2.3_benchmark/README.md)), Level C
  ([P2.7](p2.7_failure_scaling_generalization/README.md)) and Level D
  ([P2.10](p2.10_result_artifact_demo/README.md)) the evaluation is written up, and the user says whether EinAf
  goes on or an iteration of Ein comes first;
- between gates the kernel gains what the application needs of it and a bug is fixed, where Ein's syntax and
  semantics stay as they are ([Q-M2.16](#q-m216--what-of-the-kernel-may-change-inside-m2)); what would change the
  language waits for the iteration of Ein. A bug that is a wrong answer is filed in the wrong-answer register as
  it is found, whoever fixes it.

Nothing waits for this question: the first gate is the first time it has an answer to give.

---

## Q-M2.1 — When is the kernel frozen?

**Ruled by the user, 2026-11-25:** *"use current state - after M1f closed"*.

**Read as:** the instrument is the kernel as it stands now, after M1f's close (`458bf90b`). M2 is planned and run
against it and does not wait for a kernel milestone that is queued, unless the user says so
([Q-M2.12](#q-m212--what-prints-the-answer) asks about one). The dependencies this entry named are spent: M1c and
M1d are closed, and the engine says *not decided* in its own words, so the loop derives no `unknown`.

**What stays of the entry**, for [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md)'s re-plan: a record carries the
commit of Ein it was taken on (from git: `ein --version` prints none), the stdlib manifest's SHA and the resolved
configuration, so that a number says which kernel it came from. **What changes the kernel inside M2** is what a
ruling here asks for and nothing else: today that is the machine surface for diagnostics
([Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics)), which changes no answer.

## Q-M2.2 — Must the model be local?

**Ruled by the user, 2026-11-25:** *"yes"*.

**Read as:** the model is local, which keeps the three things this entry listed: a grammar at decoding, a model
file with a SHA, and a budget counted in the same tokens for every arm. The entry's recommendation also kept a
hosted model as a replication arm at the end of Level C. The question ruled is *must*, so that arm is not in the
plan, and adding one would be the user's decision. Which model: [Q-M2.5](#q-m25--local-model-choice). What *local*
is on the host today: [`analysis/host.md`](analysis/host.md).

## Q-M2.5 — Local model choice

**Ruled by the user, 2026-11-25:** *"Qweb3.5:4b"*.

**Read as `qwen3.5:4b`**, the ollama tag, which is in the host's system ollama store (pulled 2026-11-20):
architecture `qwen35`, 4.2B parameters, quantization Q4_K_M, a 2.47 GiB GGUF file (`sha256:a9d0a3e1d8c9…`) with a
0.63 GiB vision projector, context length 262 144, capabilities `completion`, `vision`, `tools` and `thinking` (on
by default), default parameters `temperature 1`, `top_k 20`, `top_p 0.95`, `presence_penalty 1.5`. This was Q23
here; its working answer (Qwen3-30B-Instruct, Phi-4 14B as the fallback) and its list of candidates are gone.

**What stays of the entry:** the model is an **experimental constant**, not a product choice. It is pinned by its
file's SHA for the whole of Level C; a second model would be a replication, not a fallback, and none is named.

**The user since, 2026-11-25:** *"qwen model (or other suitable models)"*. The model of the plan stays
`qwen3.5:4b`; another suitable model is a second value of the record's model field, taken when an experiment asks
for one, and the comparison of two models is then a replication.

**On the host** ([`analysis/host.md`](analysis/host.md), checked twice 2026-11-25): the tag names a GGUF file in
ollama's system store, which llama.cpp reads as it is. With the GPU backend the user installed, `llama-server`
loaded the whole model on the device (2 771 MiB at a context of 4 096 tokens, without the projector) and
generated about 66 tokens a second. What runs it is [Q-M2.14](#q-m214--what-runs-the-model).

## Q-M2.7 — What language is the frontend written in?

**Ruled by the user, 2026-11-25:** *"Rust"*.

**Read as:** the harness and the loop are Rust and reach the engine as crates, `ein-ir` to load, `ein-infer` to
solve, `ein-render` to explain ([`04-api.md`](../../docs/ein/ein.rs/04-api.md)): the answer as values (`ReadOut`,
`Verdict`, root's debts and core), a parse error with every error, its notes and its repairs, and
`ein_ir::dump_canonical`, which no command prints and five tasks of P2.2 and P2.4 key on. This was Q25 here.
The stage that was to decide it, *the boundary*, is gone from [P2.1](p2.1_kernel_as_instrumentation/README.md),
and no stage file writes a Python path.

**What it left to the re-plan, and what the re-plan took** (each a default, the user's to correct):

- **Four load-side errors are `String` newtypes** (`KbLoadError`, `LoadError`, `MacroError`, `CompileError`), so a
  Rust loop reads strings there until they carry a kind and a place: the same ground as
  [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics), and S2.1.2's first task.
- **Where the package sits**: a Cargo package of its own under `einaf/`
  ([Q-M2.8](#q-m28--where-does-einaf-live)) that depends on the engine's crates by path, outside the engine's
  workspace, so that nothing under `ein.rs/` names it
  ([S2.1.1](p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md)).
- **The benchmark's generators are Rust too**, in the same package, so that EinAf has one toolchain; the ruling
  names the frontend, and this goes one step past it.

**Kept from the entry, because it is cheap to write down and expensive to rediscover:** PyO3 is not a boundary
([Q-M1a.23](../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)); there is no socket
and no server, the engine ships as a library and a CLI; and `ollama` is no runtime where a grammar is the
mechanism, which the host checks confirmed on two versions ([Q-M2.14](#q-m214--what-runs-the-model)).

## Q-M2.8 — Where does EinAf live?

**Ruled by the user, 2026-11-25, in two answers.** The first: *"./einaf/, ./docs/einaf/, etc..."*. The second, to
the coordinator's default that those trees were external to both the project and the process: *"yes, own tree, in
this repository, outside of ein, because ein.rs is an implementation of ein and is a dependency of einaf, but
inside the development process, make clear projects hierachy in root readme and AGENTS.md"*.

**Read as:** the repository holds two projects, one over the other, and one development process over both.

| | what it is | its trees | depends on |
|---|---|---|---|
| **Ein** | the language, what a program means and what its answer is, and **ein.rs**, an implementation of it | `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/ein/`, `docs/decisions/`; the implementation in `ein.rs/` | nothing in the repository |
| **EinAf** | iterative autoformalization, an application of Ein | `einaf/`, `docs/einaf/` | ein.rs, as a library |
| the process | how both are developed | `plans/`, `utils/`, the gate and CI, `AGENTS.md` | reads both |

- **The dependency runs one way.** EinAf reads Ein and links ein.rs. Nothing of Ein reads, links or names EinAf:
  no crate, no page of `docs/ein/`, no decision, no comment, no test, no module of the library, no corpus entry.
  A change the kernel takes for EinAf's sake ([Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics))
  is stated in Ein's terms. The root `README.md` is the repository's page and names both.
- **Inside the development process**, which corrects the default: EinAf is planned under `plans/`, its questions
  are this ledger's, `AGENTS.md`'s standing rules hold for it, and its build and tests are steps of the gate once
  they exist. It is not *external to both*; `c/`, `nl2smt/` and `docs/lib/` stay that.

**Done with the ruling:** the root `README.md` and `AGENTS.md` state the hierarchy. No tree is created by it:
[S2.1.1](p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md) creates them.

**Ruled further, 2026-11-25**, to the first of the defaults below: *"One tree - yes, preferred, docs/einaf/ -
separately"*. So item 1 is the user's; the others stay defaults.

**What the first two answers did not say, each taken by default and the user's to correct, but the first:**

1. **One tree a project at the root** (ruled). The plan's other root directories (`grammars/`, `benchmark/`,
   `experiments/`, `packaging/`) are folders of `einaf/`, so that the root shows the hierarchy, and EinAf's
   documentation is `docs/einaf/`, apart from it.
2. **The checks of Ein's prose are Ein's.** The release-voice check and the rules of `docs/ein/` (the current
   state only, no date, no measurement) are scoped to Ein: `einaf/` and `docs/einaf/` are named in
   `utils/check_release_voice.py`'s `NOT_PROJECT` when they are created. EinAf's own pages use the research
   plan's vocabulary (F0 to F8, B0 to B6, G1 to G9), which that check reads as followup ids, and its records are
   dated measurements by nature. What EinAf's pages hold themselves to is decided with its first page.
3. **A gate step of its own.** EinAf is a Cargo package outside the engine's workspace, built and tested by its
   own step of `run_tests.sh` and of CI. What needs the model file and the GPU is an experiment, run by hand with
   its record, in no tier: the runner has neither.
4. **The benchmark's programs stay out of the corpus roots** (`examples/`, `stdlib/`, `tests/`), where each would
   need a `corpus.toml` entry and would join every sweep of Ein's gate; EinAf's own command checks their claims,
   by the library's functions or by the binary's `ein test`.
5. **Where an answer about EinAf lands**: in this ledger while EinAf is planned, and in `docs/einaf/` once the
   thing it decides is built. `docs/decisions/` is about Ein
   ([Q-M1f.346](../../docs/decisions/docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).

**Proposed by [S2.1.1](p2.1_kernel_as_instrumentation/s2.1.1_tree_and_outcomes.md), with EinAf's first pages
(2026-11-25): what EinAf's pages hold themselves to.** A proposal, the user's to correct; the pages written so far
([`docs/einaf/`](../../docs/einaf/README.md)) follow it.

1. **The current state, as Ein's pages do.** A page that stops being true is rewritten or deleted; no page names a
   plan, a phase, a stage, a task or a date, and none links into `plans/`: EinAf is a project, and the process
   reads it as it reads Ein. An answer about EinAf is written into the page of the thing it decides, once that is
   built; until then it is this ledger's, and a page does not cite it by id.
2. **A dated measurement is a record, not a page**: one run, immutable, under `einaf/experiments/`. A number a page
   states carries the command or the test that re-takes it, as Ein's do
   ([Q-M1e.4](../../docs/decisions/docs/Q-M1e.4-an-exact-number-in-a-page-carries-the-command.md)); the corpus counts
   of S2.1.1's sweep are in its record, and the page names the test that prints them.
3. **The research plan's labels are EinAf's vocabulary**: F0 to F8 (what the kernel is allowed to say), B0 to B6
   (the arms), G1 to G9, written in its pages and its code as they are in [`EinAf.md`](EinAf.md). This is why the
   release-voice check does not read EinAf's trees: it would read `F2` as a followup id.
4. **Named as Ein's pages are**: `NN-kebab-name` in reading order with a `README.md` index; a diagram is a file
   with its picture beside the page, under `diagrams/`. `utils/check_doc_blocks.py` reads `docs/ein/` only, and
   reads a `diagrams/` folder under `docs/einaf/` without the page that shows it, so EinAf's first diagram needs the
   check to read EinAf's pages for its rule 1.
5. **What they may link**: Ein's pages, tests and decisions, and EinAf's own code and tests. A decision of Ein's
   they cite resolves to its file: `utils/doc_audit.py --links --check --tracked` reads EinAf's pages for links,
   anchors and decision ids as it reads Ein's.
6. **What enforces it today**: the link check alone. Items 1 to 4 have no check yet; one is proposed for when the
   pages grow, either a check of EinAf's own (no plan id, no date, no path into the process; the labels allowed) or
   the release-voice check with EinAf's labels allowed.

## Q-M2.9 — Does the kernel grow a machine surface for diagnostics?

**Ruled by the user, 2026-11-25:** *"(b)"*: the kernel grows the surface, in its own terms.

**What (b) was:** a refusal, an abort and a run's notes reach a machine surface, each with a kind, and with a place
where the source has one, as a change to the project made under the project's rules; no page or comment of the
kernel names the loop; and the feedback object stays the harness's, a view over that surface and `ein-summary/2`.
Not taken: (a), the harness reads stderr, and (c), the plan's `ein-feedback/1` written by `ein-cli`.

**What is on no machine surface today** ([`analysis/audit_p2.1.md`](analysis/audit_p2.1.md) § 0): a parse or load
refusal (stderr text and no summary; `ein test --json-report` says `outcome: "error"` and no message); a `-T`
abort (one stderr line, exit 2, no summary); a load's `note:` lines; a core's `:source` quote; the core under
`--single`.

**What it sets in motion:**

- [F23](../followups/f23_diagnostics_in_eins_words.md)'s D-18 and L-125 are this change, and P2.1's re-plan carries
  them; F23 says so.
- The change is the kernel's: `07-diagnostics.md` and `08-read-out.md` in the same commit, `help_shape.txt` and
  `06-config.md`'s census if it is an option, the `.expected` fixtures if places are added.
- With a Rust harness ([Q-M2.7](#q-m27--what-language-is-the-frontend-written-in)) the same content is wanted as
  values of the library, where four load-side errors are strings today; whether the JSON is written from those
  values is the design's.
- **Its decision file is written with the change**, under this id and in the project's words: a decision states
  what the project does, and today the project does not do this.

## Q-M2.11 — What is a flagged program to the loop?

**Ruled by the user, 2026-11-25:** *"(a)"*: an outcome of its own.

**What (a) was:** a program the static check flags as not monotone
([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)) is **never
certified**; where it is flagged (`monotonicity.sites`) is fed back as a diagnostic at the feedback levels that
carry one; every table counts it in a column of its own; and the benchmark's canonical programs are never flagged,
as no program under `tests/` is. Not taken: (b), treating it as a refusal, and (c), reading its answer like any
other.

**For the re-plan:** it is a seventh outcome beside Stage A1's six, in the state machine
([S2.4.1](p2.4_loop/s2.4.1_state_machine.md)), in *certified*
([Q-M2.10](#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r)) and in the metrics
([S2.5.3](p2.5_harness/s2.5.3_metrics.md)). A formalizer will write such programs, since a closed-world reading
(*the youngest is the one nobody is younger than*) is an `absent` over a relation a commitment can fill, and how
often it does is itself a finding. What a flagged program's answer is stays
[M1n](../m1n_non_monotone_programs/README.md)'s.

## Q-M2.12 — What prints the answer?

**Ruled by the user, 2026-11-25:** *"don't run M1p now, use as much of existing ein NL features, e.g. query
:goal-text, for numbers and other compound facts use additional external formatters, they will be later starting
point for M1p"*.

The question was raised by the user's statement of the core idea
([README](README.md#the-two-main-ideas-the-user-2026-11-25)): *"ideally the final test problem answer must be
short and must be produced by ein program NL output"*. Of the three ways it weighed (the answer as Ein prints it
today; the part of M1p an answer needs run first, which was recommended; the harness formats values) the ruling is
the third, with as much of the first as there is. `git show 6a3c9a75:plans/m2_nl_to_ir/open_questions.md` has the
table.

**Read as:**

- **[M1p](../m1p_ein_to_natural_language/README.md) does not run**, and the kernel is not changed for the
  answer's words.
- **The answer of a test problem is what its Ein program says, through what Ein has today**: the query's
  `:goal-text`, filled with a row's bindings; and, where an explanation is shown, a rule's and a relation's `:why`
  and a fact's `:source`
  ([`14-natural-language.md`](../../docs/ein/user/03-syntax-and-semantics/14-natural-language.md)). So the program
  the formalizer writes carries its own answer sentence: the query's `:goal-text` is part of the contract
  ([S2.2.1](p2.2_formalizer/s2.2.1_contract.md)).
- **Where a value does not read as text, EinAf formats it, outside the kernel.** Today
  [`bob1.ein`](../../examples/word_problems/bob1.ein) answers `“(amount Bob apples t1): (n z 3)”`: a number is its
  term, `(n z 3)`, and so is a fact that stands as a value. The formatters are EinAf's, in `einaf/`, over the
  library's values: a numeral of the calculator's first, then the other compound values a benchmark family
  prints.
- **They are written to be M1p's starting point**: a formatter a kind of value, its cases as data, its tests
  beside it, and no knowledge of the loop in it, so that the phase of M1p that gives a value its own format
  ([P1p.1](../m1p_ein_to_natural_language/p1p.1_custom_formatting/README.md)) can take them as they are.

**What it changes in the plan:** the formatters are a stage of the formalizer
([S2.2.5](p2.2_formalizer/s2.2.5_from_text.md)); a benchmark instance's gold answer is a short text beside its
`:expect` claim ([P2.3](p2.3_benchmark/README.md)); and what scores an answer compares that text across every arm
([S2.5.3](p2.5_harness/s2.5.3_metrics.md)). What Ein lacks here (the answer's line is on the read-out and in no
key of `ein-summary/2`; a value has no text of its own) is recorded in
[the evaluation of Ein](ein_evaluation.md), for the iteration of Ein that follows.

**Ruled further, 2026-11-25**, to two defaults of the re-plan:

- **The answer is a sentence.** To the default that every arm is scored on a bare value (*3*, *Cy*): *"no, I meant
  a sentence, like zebra's “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”"*. That
  sentence is what [`zebra.ein`](../../examples/zebra.ein)'s query prints through its `:goal-text`. **Read as:**
  *short* means a sentence or two in the problem's own words, not a bare value; the answer query's `:goal-text` is
  such a sentence; every arm is asked for one; and a gold answer is the sentence its canonical program prints.
  **What it changes:** an answer is no longer scored by equality of a normalised value. How a sentence is scored
  is [S2.5.3](p2.5_harness/s2.5.3_metrics.md)'s to design, by default **by what it states**: the gold holds, beside
  its sentence, the values the sentence binds (the answer query's rows), and an answer is right when it states
  each of them and no other value in its place; for an arm that runs Ein the rows are the program's own and the
  gold claim is checked by the kernel where it is comparable.
- **EinAf's fixed texts, where no program prints the answer:** *"OK"*. The two texts
  ([S2.2.5](p2.2_formalizer/s2.2.5_from_text.md): *The statements do not determine the answer.* and *The
  statements contradict each other.*) are EinAf's and stand, said as such.

## Q-M2.13 — Who calls the kernel?

**Ruled by the user, 2026-11-25**, to the coordinator's reading that both are arms of one experiment, compared
with a chain of thought: *"yes, it is correct, "tool for the LLM" - means LLM can decide to use it on it's own, it
is recommened but optional, also qwen model (or other suitable models) most probably not aware about this
language, was not trained for synthesis of ein programs, newertheless ein semantic is very simalar to Datalog,
syntax is Lisp/SMT style, so writing ein program may be not impossible without GBNF; "filter of thoughts" - means
it is forced to write ein code and can not avoid it"*.

The question was raised by the user's statement of the core idea
([README](README.md#the-two-main-ideas-the-user-2026-11-25)): *"On one hand ein works like a tool for LLM, on
other hand it works like filter of thoughts"*.

**Read as:** two arrangements of one model and one kernel, both arms of the experiment, each compared with the
model reasoning in text ([Stage D](EinAf.md#stage-d--establish-baselines)'s B1).

| | **the tool** | **the filter** |
|---|---|---|
| who decides that Ein runs | the model, when it likes; the prompt recommends it, and the model may answer without it | the harness, on every generation |
| what the model writes | text, and an Ein program where it chooses to call | an Ein program and nothing else: it cannot avoid it |
| what holds the syntax | nothing: the model writes Ein as it can, and a parse error is what the tool returns | the harness accepts a program only; a grammar over the output makes that hold token by token ([Q-M2.6](#q-m26--one-gbnf-or-many)) |
| the answer | the model's last message; whether it is what its last run of Ein printed is recorded | what the Ein program prints ([Q-M2.12](#q-m212--what-prints-the-answer)); with no program the kernel passes there is no answer |
| in the plan | new: [S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md), and B6 of [S2.5.2](p2.5_harness/s2.5.2_baselines.md) | the plan's loop: [S2.4.1](p2.4_loop/s2.4.1_state_machine.md), B2 to B5 |

**The user's remark on the model is a hypothesis the plan measures, not a premise it takes**: a model not trained
on Ein may still write it, the semantics being near Datalog's and the syntax Lisp's and SMT-LIB's. So *the model
writes Ein with no grammar* is measured where it is cheapest, as the first thing the formalizer's phase does
([S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md)), and the filter with its grammar off is an ablation
([P2.6](p2.6_ablations/README.md)).

**What the ruling leaves to the plan, each a default:** what the tool replies is a level of the feedback ladder,
the same object the filter renders ([S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md)); a tool episode is priced in
generated tokens with its calls counted, so that it compares with the others under
[Stage D](EinAf.md#stage-d--establish-baselines)'s *same model, similar budget*; the tool is a function the
harness declares to the model in its own process, and nothing of [M15](../m15_mcp_server/README.md), an MCP
server, starts by it.

## Q-M2.14 — What runs the model?

**Ruled by the user, 2026-11-25:** *"I expect to use llama-cpp with rust wrapper, not ollama, because ollama does
not allow full LLM control and GBNF, does it?"*.

**The question in it, answered by the host check** ([`analysis/host.md`](analysis/host.md)): it does not. Ollama
0.40.16, as 0.35.1 before it, takes a JSON schema as `format` and no grammar: a GBNF `grammar` at the top level
and inside `options` came back as free prose, and a GBNF string as `format` was refused. On the same file
`llama-server`, of llama.cpp 0.9.0, honoured the grammar against the prompt, with the whole model on the GPU.

**Read as:** the model runs **inside EinAf's process, through a Rust binding to llama.cpp**: no server, no
container and no HTTP between the harness and the model. Ollama is no runtime of the plan; its store is where the
file was pulled to. What *full control* buys, and the plan uses:

- the sampler chain, with the grammar in it, the seed and every parameter a field of the record;
- thinking on or off a call, and a grammar that begins where the thinking ends (in the host check a grammar over
  the whole output of a thinking model took the thinking too);
- each token's log-probabilities;
- the context kept between the turns of one loop, so that a repair pays for the new tokens only;
- the chat template as a file of the record, not a server's default.

**Left to [S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md), each a default:**

- **Which binding.** On crates.io at the check: `llama-cpp-2`, `llama-cpp-4`, `llama_cpp`, `llama-cpp-bindings`;
  none is tried. Whether it builds a llama.cpp of its own or links the host's `libllama.so.0` is part of the
  choice, and the first thing it is held to is that the llama.cpp it gets knows the model's architecture,
  `qwen35`.
- **`llama-server`, of the same package, is the reference** a binding is compared with (one file, one grammar,
  one seed) and what the plan falls back to if no binding loads the model. The Compose service of the old plan,
  and `packaging/` with it, are dropped.

## Q-M2.16 — What of the kernel may change inside M2?

**Ruled by the user, 2026-11-25:** *"we can change kernel without changing ein language syntax and semantics, any
additional kernel features (logs, traces, measurement, timeouts, limits, etc) can be added; bug fixes allowed"*.

The question was raised by the re-plan as a narrower one, *is a saturation bounded inside the kernel?*: nothing in
a solve stops a saturation that has no fixpoint ([`grow.ein`](analysis/probes/p2.7/grow.ein), E-45 of
[the evaluation](ein_evaluation.md)), and the re-plan's default was to change nothing in the kernel and have EinAf
stop it from outside. `git show cce0f0ff:plans/m2_nl_to_ir/open_questions.md` has the two options.

**Read as:** inside M2 the kernel may be changed wherever **Ein's syntax and semantics stay as they are**.

- **Allowed, with no further ruling:** a feature that makes the kernel say, measure or bound more (a log, a trace,
  a measurement, a timeout, a limit, a key of a machine surface, a value of the library), and **a bug fix**. Each
  is made under Ein's rules, as [S2.1.2](p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md) is: its
  pages and tests in the same commit, the gate, a decision file where it decides something, and no word of EinAf
  in Ein.
- **Not allowed by this:** a change to the language (a form, a keyword, a name the kernel reads) or to what a
  program means and what its answer is. That waits for the iteration of Ein that follows
  ([Q-M2.15](#q-m215--where-does-m2-stop-for-the-next-iteration-of-ein)), or for a ruling of its own.
- **It supersedes** the re-plan's default that the kernel changes inside M2 *only where a ruling says so*, and the
  sentences of the plan that called S2.1.2 the milestone's one kernel change.

**What it changes in the plan:**

- **A saturation is bounded by the kernel.** The clock (`-T`) and the budget the kernel already has reach inside a
  saturation, and a run they stop says what a stopped run says today: no new word and no new outcome. It is a task
  of [S2.1.2](p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md). The word of its own for *did not
  stop*, a bound that is the saturation's own, and a static check are
  [M1g](../m1g_logic/README.md)'s, in its stage on a saturation that does not stop, with the user's question
  there.
- **EinAf calls the kernel in its own process**, as ruled
  ([Q-M2.7](#q-m27--what-language-is-the-frontend-written-in)). A worker process is built only if a probe shows a
  case the kernel's own bounds do not stop ([S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_feedback_object.md)).
- **An entry of the evaluation that is a gap or a bug may be closed inside M2**, by the stage that needs it, and
  the entry then says so. An entry nothing in M2 needs stays for the iteration of Ein.

## Q-M2.17 — What is the milestone's minimal goal?

**Ruled by the user, 2026-11-25**, to the re-plan's note that a 4B model may not formalize the Zebra puzzle in one
shot: *"I just checked in ollama Qwen 3.5 started "Step-by-Step Deduction" in default thinking mode, but didn't
manage to find the solution and stopped (probably on some limit, I'm not aware about). So if this model with help
of Ein or through Ein will became able to solve the project's signature problem - It would be already success. So
this is a minimal goal."*

**Read as:** the milestone's minimal goal is that **the chosen model, which does not solve the Zebra puzzle by
reasoning in text, solves it with Ein**: *with the help of Ein*, the tool, or *through Ein*, the filter
([Q-M2.13](#q-m213--who-calls-the-kernel)). Reaching it is already a success, whatever the tables then say. It is
not one shot: any arm that uses Ein counts, the loop included.

**What it changes in the plan:**

- **It is the first line of the milestone's acceptance**, before the levels, and the Level B criterion that asked
  for the Zebra puzzle in one shot is that goal's first attempt, not a bar of its own
  ([README](README.md#acceptance)).
- **The other side is taken as a record, not as an observation.** The user's run was one, through ollama, cut by
  a limit nobody set on purpose; in the host check ollama had loaded the model with a context of 4 096 tokens
  ([`analysis/host.md`](analysis/host.md)), which a step-by-step deduction of this puzzle may well exceed. So the
  model alone on the Zebra text is run by EinAf, thinking on, with the context and the budget stated in the
  record, first in [S2.2.2](p2.2_formalizer/s2.2.2_model_runtime.md)'s probe and again as arm B1 on the puzzle
  ([S2.5.2](p2.5_harness/s2.5.2_baselines.md)): *the model alone does not solve it* is then a claim with its
  evidence, and the goal a difference between two records.
- **Each stage that runs the model on the Zebra text says where the goal stands**: one shot
  ([S2.2.5](p2.2_formalizer/s2.2.5_from_text.md)), the filter's loop ([S2.4.1](p2.4_loop/s2.4.1_state_machine.md)),
  the tool ([S2.4.6](p2.4_loop/s2.4.6_ein_as_a_tool.md)).
