# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

Two id forms share this file. **Q7–Q11** are sticky ids from the cross-milestone pool, assigned when the
milestone was *NL → IR*; they keep their numbers. **Q-M2.n** is the milestone-scoped form, which
[M10](../m10_external_benchmarks/open_questions.md) uses too: Q-M2.1–4 arrived with the 2026-08-23 reshape around
[`EinAf.md`](EinAf.md), Q-M2.8–11 with the analysis M2 started from, 2026-11-25
([`analysis/`](analysis/README.md)), and Q-M2.12–13 with the user's answers of that day.

**Q-M2.5, Q-M2.6 and Q-M2.7 were this file's Q23, Q24 and Q25 until 2026-11-25.** Those three ids were already the
root ledger's, given to questions of M1's, and two of them are decision files under `docs/decisions/` (a premise
says what a variable ranges over; Ein has no ordinal form). An id is never reused, so M2's three took the next free
ids of the milestone, and every file of this folder that cited them cites the new ones.

**How it is kept.** The open questions come first and the user's rulings after them. A ruling is the user's words
as given, then what the coordinator reads them to mean, **which the user may correct**; the options a ruled
question weighed are not kept here (`git show 5d678042:plans/m2_nl_to_ir/open_questions.md` has them all).

**Where an answer lands.** An answer about Ein is a file under `docs/decisions/`, under the id of its question,
written with the change that makes it true; an answer about EinAf or about how the work is done stays here, beside
its question. EinAf's trees are not the project's ([Q-M2.8](#q-m28--where-does-einaf-live)), so an answer about
the harness is the second kind.

## Index

### Open

| Q | title | decided |
|---|---|---|
| [Q7](#q7--llm-as-surface-generator) | Is the surface generator (NL output) allowed to be an LLM? | **outside the research question**, parked with its working answer; the answer's own words are [Q-M2.12](#q-m212--what-prints-the-answer) |
| [Q8](#q8--ambiguous-parses) | Where do ambiguous NL parses go — branched on the IR, or rejected? | [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md), by experiment; the form is settled, a hypothesis `(open F)` (the user, 2026-11-25) |
| [Q9](#q9--ontology-provenance) | Per-puzzle declared ontology vs ontology inferred from text | [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) — reframed as theory *selection* |
| [Q10](#q10--direct-llm--constraint) | When is direct LLM → constraint emission acceptable (skip the IR)? | [P2.8](p2.8_representations/README.md) — Stage L is this question asked properly |
| [Q11](#q11--link-grammar-value) | Does link-grammar enrich LLM input usefully, or is it dead weight? | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), by experiment |
| [Q-M2.3](#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory) | What is the unit of faithfulness without a gold theory? | [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) |
| [Q-M2.4](#q-m24--is-the-fixed-point-syntactic-or-semantic) | Is the fixed point syntactic or semantic? | [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) |
| [Q-M2.6](#q-m26--one-gbnf-or-many) | One GBNF grammar per task, or one grammar for everything? | [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) |
| [Q-M2.10](#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) | What does *Ein certifies a unique model* mean on R? | **the user, after experiments** (*"it depends on problem shape"*) |
| [Q-M2.12](#q-m212--what-prints-the-answer) | What prints the answer? | **the user**; raised by the answers; it orders M2 and [M1p](../m1p_ein_to_natural_language/README.md) |
| [Q-M2.13](#q-m213--who-calls-the-kernel) | Who calls the kernel: the harness, the model, or both? | the re-plan of [P2.4](p2.4_loop/README.md) and [S2.5.2](p2.5_harness/s2.5.2_baselines.md); the reading is the coordinator's |

### Ruled by the user

| Q | title | the user, 2026-11-25 |
|---|---|---|
| [Q-M2.1](#q-m21--when-is-the-kernel-frozen) | When is the kernel frozen? | *"use current state - after M1f closed"* |
| [Q-M2.2](#q-m22--must-the-model-be-local) | Must the model be local? | *"yes"* |
| [Q-M2.5](#q-m25--local-model-choice) | Which local model? | *"Qweb3.5:4b"*, read as `qwen3.5:4b` |
| [Q-M2.7](#q-m27--what-language-is-the-frontend-written-in) | What language is the frontend written in? | *"Rust"* |
| [Q-M2.8](#q-m28--where-does-einaf-live) | Where does EinAf live? | *"./einaf/, ./docs/einaf/, etc..."* |
| [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics) | Does the kernel grow a machine surface for diagnostics? | *"(b)"*: it does, in its own terms |
| [Q-M2.11](#q-m211--what-is-a-flagged-program-to-the-loop) | What is a flagged program to the loop? | *"(a)"*: an outcome of its own, never certified |

The stage each ruled question was to be decided in ([S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md)
for Q-M2.7, [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) for Q-M2.2 and Q-M2.5,
[S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) for Q-M2.1) has that task no longer; the stage files say so
when their phases are re-planned.

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
([README](README.md#the-core-idea-the-user-2026-11-25)) asks that *"the final test problem answer must be short and
must be produced by ein program NL output"*. That is rendering A, the engine's own templates, and it puts the
answer's words inside the research question; what stays parked here is B, an LLM's polish. Ein to natural language
is [M1p](../m1p_ein_to_natural_language/README.md)'s, and what M2 needs of it is
[Q-M2.12](#q-m212--what-prints-the-answer).

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
families, where the gold theory is exact. The submodule note stands: it is
deinitialised, and registering it is the stage's first act.

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

**Since (the host check, 2026-11-25).** A GBNF grammar needs `llama-server`: ollama 0.35.1, whose store holds the
model the user chose ([Q-M2.5](#q-m25--local-model-choice)), takes a JSON schema and does not honour a grammar
([`analysis/host.md`](analysis/host.md)).

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

## Q-M2.12 — What prints the answer?

**Raised 2026-11-25 by the user's answers.** The user, of the core idea
([README](README.md#the-core-idea-the-user-2026-11-25)): *"ideally the final test problem answer must be short and
must be produced by ein program NL output"*. **The user's**, because it orders two milestones.

**What an Ein program prints as an answer today.** A query's `:goal-text`, filled with each row's bindings, is the
one text an answer has. [`bob1.ein`](../../examples/word_problems/bob1.ein) (*Bob has 5 apples. He gave 2 to
Alice. How many apples does Bob have?*, with `:goal-text "{?q}: {?n}"`) answers

```text
“(amount Bob apples t1): (n z 3)”
```

Two things stand between that line and a short answer a benchmark can score:

- **A value is written as its term**: the number 3 is `(n z 3)`, and what was asked about is `(amount Bob apples
  t1)`. How a value is written as text is [M1p](../m1p_ein_to_natural_language/README.md)'s
  [P1p.1](../m1p_ein_to_natural_language/p1p.1_custom_formatting/README.md) (a decimal number first), and an answer
  that reads in natural language is its [P1p.3](../m1p_ein_to_natural_language/p1p.3_reasoning_formatting/README.md).
  M1p is not started, and each of its phases runs only on the user's word.
- **The line is on the read-out and in no key of `ein-summary/2`**: `queries[].classes[].rows` holds the bindings
  (`{"n": "(n z 3)", "q": "(amount Bob apples t1)"}`) and not the text. A Rust harness
  ([Q-M2.7](#q-m27--what-language-is-the-frontend-written-in)) can fill the template from the library's values; a
  reader of the summary cannot.

| | what it means | what it costs |
|---|---|---|
| **(a) M2 takes the answer as Ein prints it today** | a gold answer is a set of bindings, stated as an `:expect` claim; the `:goal-text` line is shown and not scored | the answer is neither short nor words (`(n z 3)`): the statement's *ideally* is not met until M1p runs |
| **(b) the part of M1p an answer needs runs first** | P1p.1, a value's own format, and what of P1p.3 makes a query's answer a sentence, before the benchmark's gold answers are written ([P2.3](p2.3_benchmark/README.md)) | phases of a kernel milestone before or inside M2, on the user's word; the kernel M2 reads is then not the one M1f left ([Q-M2.1](#q-m21--when-is-the-kernel-frozen)) |
| **(c) the harness formats values** | EinAf turns `(n z 3)` into `3` | the answer is then not *produced by ein program NL output*, and there is a second formatter beside M1p's |

**Recommendation: (b), narrowly**: P1p.1 and the answer's sentence, with the sentence on the summary beside its
row, which is a kernel change of the kind [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics)
ruled. Whatever is ruled, a gold answer can be written as an `:expect` claim, which holds under all three.

## Q-M2.13 — Who calls the kernel?

**Raised 2026-11-25 by the user's answers.** The user, of the core idea
([README](README.md#the-core-idea-the-user-2026-11-25)): *"the LLM that synthesize problem formalization
(facts+ontology+theory) checks if it's formalization works"*, and *"On one hand ein works like a tool for LLM, on
other hand it works like filter of thoughts"*.

The plan has one arrangement, the second: **a filter**. A harness runs a state machine
([S2.4.1](p2.4_loop/s2.4.1_state_machine.md)); after each generation it runs the kernel and renders what the kernel
said into the next prompt, at a feedback level, and a program the kernel does not pass is never an answer. The
model never asks.

**A tool** is the other: the model decides, inside one episode, when to run the kernel on what it has written, and
reads the reply as a tool's result. What bears on it:

- `qwen3.5:4b` ([Q-M2.5](#q-m25--local-model-choice)) declares the `tools` capability, and in the host check it
  emitted a well-formed call of a declared function ([`analysis/host.md`](analysis/host.md));
- [M15](../m15_mcp_server/README.md), an MCP server for Ein, is the unstarted neighbour that would be one transport
  of it;
- [Stage D](EinAf.md#stage-d--establish-baselines)'s comparison is *same model, similar budget*: a harness loop is
  priced per iteration, and a model that calls a tool when it likes needs a budget defined for it;
- the feedback ladder F0 to F8 is what a harness chooses to say; a tool's reply is one level of it, or the level
  the model asks for.

**The coordinator's reading, which the user may correct:** both are arms of one experiment. The filter is the
plan's loop (B3 to B5); the tool is an arm the plan does not have; and *"replacing chain of thought with chain of
formal reasonings"* is the comparison of both with B1, the model reasoning in text. **Decided in** the re-plan of
[P2.4](p2.4_loop/README.md) (the state machine) and of [S2.5.2](p2.5_harness/s2.5.2_baselines.md) (the baselines).
Nothing of M15 starts by this.

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

**What [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md)'s re-plan meets on the host**
([`analysis/host.md`](analysis/host.md), checked 2026-11-25 after the user's installs):

- **The tag names a model in ollama's store, and ollama does not take a grammar.** Ollama 0.35.1 honours a JSON
  schema as `format`; a GBNF `grammar` field, at the top level and in `options`, came back as free prose. The file
  is a GGUF, which `llama-server` reads as it is, so the plan's server and its GBNF
  ([Q-M2.6](#q-m26--one-gbnf-or-many)) stand over the same file.
- **`llama-server` is installed and does not start** (the `llama-cpp` package, 2026-11-25): it needs
  `libssl.so.4`, and the host has OpenSSL 3.6.3, with 1 239 packages waiting for an upgrade.
- **Both runtimes are CPU-only as installed** (`ollama` without `ollama-cuda`, `ggml` without `ggml-cuda`): the
  model ran at about 20 tokens a second on the CPU, the GPU idle.

The first is S2.2.2's to design. The other two need the host changed, which is the user's hand.

## Q-M2.7 — What language is the frontend written in?

**Ruled by the user, 2026-11-25:** *"Rust"*.

**Read as:** the harness and the loop are Rust and reach the engine as crates, `ein-ir` to load, `ein-infer` to
solve, `ein-render` to explain ([`04-api.md`](../../docs/ein/ein.rs/04-api.md)): the answer as values (`ReadOut`,
`Verdict`, root's debts and core), a parse error with every error, its notes and its repairs, and
`ein_ir::dump_canonical`, which no command prints and five tasks of P2.2 and P2.4 key on. This was Q25 here.
[S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md) has no boundary left to decide, and the stage files of
P2.2 and P2.5 that write Python paths are re-aimed when their phases are re-planned.

**What it leaves to the re-plan:**

- **Four load-side errors are `String` newtypes** (`KbLoadError`, `LoadError`, `MacroError`, `CompileError`), so a
  Rust loop reads strings there until they carry a kind and a place: the same ground as
  [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics).
- **Where the package sits**: by default a Cargo package of its own under `einaf/`
  ([Q-M2.8](#q-m28--where-does-einaf-live)) that depends on the engine's crates by path, outside the engine's
  workspace, so that nothing under `ein.rs/` names it.
- Whether the benchmark's generators are Rust too: the plan's are Python, and the ruling names the frontend.

**Kept from the entry, because it is cheap to write down and expensive to rediscover:** PyO3 is not a boundary
([Q-M1a.23](../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)); there is no socket
and no server, the engine ships as a library and a CLI; and `ollama` is not an alternative to `llama-server` where
GBNF is the mechanism, which the host check confirmed on 0.35.1 ([Q-M2.5](#q-m25--local-model-choice)).

## Q-M2.8 — Where does EinAf live?

**Ruled by the user, 2026-11-25:** *"./einaf/, ./docs/einaf/, etc..."*.

**Read as:** in this repository, in the trees the plan already names: the harness in `einaf/`, its pages in
`docs/einaf/`, and the rest where the plan puts them (`grammars/`, `benchmark/`, `experiments/`, `packaging/`).
Not the single tree this entry recommended, and not a repository of its own.

**What the ruling does not say, taken by default and the user's to correct: these trees are external to both the
project and the process**, as `c/`, `nl2smt/` and `docs/lib/` are, the last being the precedent for a tree under
`docs/` that is not the project's
([Q-M1f.345](../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)). Neither other
side can hold them. As the project, a page carries no date, no measurement and no id that reads as a followup's,
which the research plan's own F0 to F8 do, so the records and the feedback ladder could not be written there. As
the process, they would be deleted with this folder at the milestone's close.

**What the first stage that creates one of them does** (nothing is created by this ruling):

- names each tree in `AGENTS.md`'s external list and in `utils/check_release_voice.py`'s `NOT_PROJECT`;
- rewrites the root `README.md`'s sentence that the loop *is not in this repository*, and names EinAf where the
  README names the other external trees, as a reference outside the project; no other page, comment or test of the
  project names it;
- gives EinAf a test command of its own: the gate runs none of it, and the plan's *re-certified by the gate*
  becomes *checked by EinAf's command*, which runs `ein test` over the benchmark's programs;
- keeps the benchmark's programs out of the corpus roots (`examples/`, `stdlib/`, `tests/`), where each would need
  a `corpus.toml` entry and would join every sweep.

An answer about EinAf therefore stays in this ledger: `docs/decisions/` is about Ein
([Q-M1f.346](../../docs/decisions/docs/Q-M1f.346-the-decisions-are-about-ein-not-about-how.md)).

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
