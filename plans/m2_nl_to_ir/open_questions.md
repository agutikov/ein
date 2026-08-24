# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

Two id forms share this file. **Q7–Q11 and Q23–Q25** are sticky ids from the
cross-milestone pool, assigned when the milestone was *NL → IR*; they keep
their numbers. **Q-M2.1–4** arrived with the 2026-08-23 reshape around
[`EinAf.md`](EinAf.md) and use the milestone-scoped form that
[M1d](../m1d_satisfiability/open_questions.md) and
[M10](../m10_external_benchmarks/open_questions.md) use. A verdict lands
**here, beside its question** — the old plan's `docs/decisions/M2-*.md` never
existed and is not coming.

## Index

| Q | title | decided in |
|---|---|---|
| [Q7](#q7--llm-as-surface-generator) | Is the surface generator (NL output) allowed to be an LLM? | **outside the research question** — the NL explanation side is [M5](../m5_presentation/README.md)'s demo; parked here, working answer kept |
| [Q8](#q8--ambiguous-parses) | Where do ambiguous NL parses go — branched on the IR, or rejected? | [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) |
| [Q9](#q9--ontology-provenance) | Per-puzzle declared ontology vs ontology inferred from text | [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) — reframed as theory *selection* |
| [Q10](#q10--direct-llm--constraint) | When is direct LLM → constraint emission acceptable (skip the IR)? | [P2.8](p2.8_representations/README.md) — Stage L is this question asked properly |
| [Q11](#q11--link-grammar-value) | Does link-grammar enrich LLM input usefully, or is it dead weight? | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), by experiment |
| [Q23](#q23--local-model-choice) | Which local model — Qwen3, Mistral, Phi-4, Gemma3, GLM? | [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) |
| [Q24](#q24--one-gbnf-or-many) | One GBNF grammar per task, or one grammar for everything? | [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) |
| [Q25](#q25--what-language-is-the-frontend-written-in) | What language is the frontend written in? | [S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md) |
| [Q-M2.1](#q-m21--when-is-the-kernel-frozen) | When is the kernel frozen? | [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) |
| [Q-M2.2](#q-m22--must-the-model-be-local) | Must the model be local? | [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) |
| [Q-M2.3](#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory) | What is the unit of faithfulness without a gold theory? | [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) |
| [Q-M2.4](#q-m24--is-the-fixed-point-syntactic-or-semantic) | Is the fixed point syntactic or semantic? | [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) |

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

## Q8 — Ambiguous parses

Per [idea 04 §Multiple-variant complication](../ideas/04-nlp-to-graph-to-solver-pipeline.md#multiple-variant-complication)
and [idea 04 §Open questions point 3](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: branched on the IR. The NL frontend emits all
plausible parses, each guarded by a `(hypothesis-parse ?id …)`
wrapper that the M1 engine treats like any other hypothesis.
Hypothesis level 0 = parse choices; level 1+ = puzzle hypotheses.

**What the reshape adds.** The research plan has a second place for
ambiguity that the old plan did not: the *verdict*. A reading the formalizer
could not resolve may be emitted as alternatives and pruned by the kernel
(the working answer), **or** committed one way and caught by the loop when
the kernel reports `k > 1` / `k = 0` and the feedback names the sentence
([S2.4.3](p2.4_loop/s2.4.3_feedback_ladder.md)). These are two strategies
for the same sentence, and which one the loop should prefer is an
experiment, not a decision — [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md)
is the stage and [P2.6](p2.6_ablations/README.md) is where the two are
compared. Two facts constrain the working answer: **there is no
`hypothesis-parse` form in ein-lang** — the keyword was to be reserved by the
old S2.1.2 and never was — and the engine's existing `hrule` is the form that
already makes a choice point out of alternatives. S2.4.5 decides whether a
new form is needed or `hrule` is it.

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

## Q23 — Local model choice

**Working answer**: Qwen3-30B-Instruct as the primary; Phi-4 14B as
the fallback. Both are GBNF-friendly and bilingual EN/RU. Decided in
[S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) with a
benchmark on the seed set.

**What the reshape changes:** the model is an **experimental constant**,
not a product choice. Stage D's comparison is *same model, similar budget,
different feedback*, so one model is pinned by file SHA for the whole of
Level C and a second model is a replication, not a fallback. Which one is
still S2.2.2's to measure; see [Q-M2.2](#q-m22--must-the-model-be-local) for
whether "local" is part of the question.

## Q24 — One GBNF or many?

**Working answer**: many — one GBNF per pass (ontology, theory, facts,
query; the alternatives wrapper if S2.4.5 keeps it). Smaller grammars
decode faster and let the prompt focus the LLM. Decided in
[S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md), which also measures the one
thing that could reverse it — decoding speed under the full ein-lang
grammar.

## Q25 — What language is the frontend written in?

**Raised 2026-08-23 by M1a [S1a.9.4](../../docs/history/m1a_rust/README.md#s1a94--documentation)
T1a.9.4.6**, which found this milestone's plan asserting a boundary that had
been deferred out from under it.

**The premise that is gone.** M2's documents were written when ein was Python,
so "the frontend is Python, calling the engine in-process" needed no argument.
Both halves of that are now open:

- `ein.py/` was deleted at M1a
  [S1a.10.5](../../docs/history/m1a_rust/README.md#s1a105--the-removal).
- **PyO3 is not the boundary**: the binding was deferred 2026-08-21 for want
  of a consumer
  ([Q-M1a.23](../../docs/history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)),
  and *this milestone was the last candidate*. There is also **no socket** —
  the server was dropped 2026-08-18; the engine ships as a library plus a CLI.
- **The llama.cpp argument was never a CPython argument.**
  The LLM infra ([S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md))
  is a `llama-server` container and a thin HTTP client. The pattern it mirrors
  is acva's, and acva's client is **C++**.

**The two live options.**

| | how it reaches the engine | what it gets |
|---|---|---|
| **Rust** | links `ein-ir` / `ein-infer` as crates ([`docs/api/rust.md`](../../docs/api/rust.md)) | structured diagnostics as *values* — `ParseError` with its location, `KbLoadError` with the accumulated problems. No boundary, no mirrored data model, no exception hierarchy to design |
| **Python** | drives the `ein` binary — `--json-summary` for the verdict and counters, `--events` for the narration | the ecosystem, and a frontend that can be rewritten without touching the engine. Diagnostics are **strings**: [`defined_behaviour.md` §1/§4](../../docs/kernel/defined_behaviour.md) pins them as such, and the CLI cannot grow a structured surface without breaking that |

**What the reshape changes in the trade (2026-08-23).** The research plan's
Stage A3 says *human-readable CLI output should not be used by the
experimental system* and asks for a versioned structured protocol — which
[S2.1.2](p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md) builds
either way, as `ein-feedback/1`. Once it exists, the Python column's "diagnostics
are strings" cost is gone: the loop reads the feedback object, never the
table, and the table's strings stay pinned. And the Rust column's "values"
advantage is smaller than it looked: the census ([S2.1.1](p2.1_kernel_as_instrumentation/s2.1.1_census.md))
found that only `ParseError` carries fields — `KbLoadError`, `LoadError`,
`MacroError` and `CompileError` are `String` newtypes — so a Rust loop would
be parsing strings too until the vocabulary of A4 is given to them. **The
decision is [S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md)'s**,
taken after S2.1.2 has made the protocol the boundary and the language a
question of who writes the renderers, the harness and the benchmark
generators. What S1a.9.4 fixed is only that the plan no longer asserts an
answer; what the reshape fixed is that the answer no longer decides the
diagnostics.

**Also settled while passing through, because it is cheap to write down and
expensive to rediscover:** `ollama` is **not** an alternative to
`llama-server` for this milestone. GBNF is the mechanism
([S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md),
[idea 01](../ideas/01-self-modifying-constraint-language.md)); llama.cpp's
server takes a `grammar` field, and ollama's API exposes only JSON-schema
`format`.

## Q-M2.1 — When is the kernel frozen?

**Arrived 2026-08-23 with the reshape.** The research plan's Stage A ends
with the sentence *the symbolic component should be treated as experimental
instrumentation, not an evolving prototype* — and two kernel milestones are
queued: [M1d](../m1d_satisfiability/README.md) changes what the engine says
at a depth cap ([Q-M1d.6](../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false))
and may change the traversal ([Q-M1d.4](../m1d_satisfiability/open_questions.md#q-m1d4--may-an-obligation-driven-generator-change-the-traversal)),
and [M1c](../../docs/history/m1c_external_validation/README.md) may find a stdlib rule wrong.
An instrument that changes between two rows of a table makes the table
uninterpretable; an instrument that cannot change for six months blocks two
milestones.

**The shape of the answer** (to be taken in [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md),
where the record is designed): the kernel is frozen **per experiment, by
commit**, and the Ein commit, the stdlib manifest SHA and the resolved
`SolverConfig` are fields of every record — which is what Stage N asks for
anyway. A kernel change between experiments is then a *replication question*:
the baselines re-run on the new commit before any ablation does, and the
record says which commit every number came from. What this does **not**
permit is a kernel change *inside* a phase's table. The specific dependency
on M1d: until it lands, the loop computes `unknown` itself from
`exhausted = false` and `verdict.type` ([S2.1.2](p2.1_kernel_as_instrumentation/s2.1.2_feedback_object.md)),
and the day M1d gives the engine a word for it the feedback object's field
stays and its source changes.

## Q-M2.2 — Must the model be local?

**Arrived 2026-08-23 with the reshape.** The old plan took *local* for
granted — `llama-server`, a pinned model file, GBNF — because
[idea 01](../ideas/01-self-modifying-constraint-language.md) is about
grammars and because a model file with a SHA is the strongest reproducibility
claim there is. The research plan does not say local; it says *model, model
version, generation parameters* recorded per experiment (B4), and it says
*same model* across the baselines (Stage D).

**What hangs on it.** A hosted frontier model would likely move every number
up and would make the main table more interesting to a reader; it would also
break three things the plan holds: GBNF (hosted APIs expose JSON-schema
constrained decoding, not a grammar), the model-file SHA (a hosted model
version is a name the provider can retire), and the inference-budget match
(token accounting differs). A local model keeps all three and limits the
formalizer's strength — which is **not obviously a weakness**: the hypothesis
is that *feedback* lifts a formalizer, and a weaker formalizer leaves more
room to lift.

**Recommendation, to be decided in [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md):**
local is the primary condition, for the three reasons above; a hosted model
is a **replication arm** run once at the end of Level C on the same split
with the same prompts, reported as a separate table whose caveats are
stated where the numbers are. The two must not be mixed in one table.

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
syntactic hash; and [`defined_behaviour.md` § 2.4](../../docs/kernel/defined_behaviour.md)
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
