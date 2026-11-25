# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

Two id forms share this file. **Q7–Q11** are sticky ids from the cross-milestone pool, assigned when the
milestone was *NL → IR*; they keep their numbers. **Q-M2.n** is the milestone-scoped form, which
[M10](../m10_external_benchmarks/open_questions.md) uses too: Q-M2.1–4 arrived with the 2026-08-23 reshape around
[`EinAf.md`](EinAf.md), and Q-M2.8–11 with the analysis M2 started from, 2026-11-25
([`analysis/`](analysis/README.md)).

**Q-M2.5, Q-M2.6 and Q-M2.7 were this file's Q23, Q24 and Q25 until 2026-11-25.** Those three ids were already the
root ledger's, given to questions of M1's, and two of them are decision files under `docs/decisions/` (a premise
says what a variable ranges over; Ein has no ordinal form). An id is never reused, so M2's three took the next free
ids of the milestone, and every file of this folder that cited them cites the new ones.

**Where an answer lands.** An answer about Ein is a file under `docs/decisions/`, under the id of its question;
an answer about EinAf or about how the work is done stays here, beside its question. Which of the two an answer
about EinAf's harness is follows from [Q-M2.8](#q-m28--where-does-einaf-live).

## Index

| Q | title | decided in |
|---|---|---|
| [Q7](#q7--llm-as-surface-generator) | Is the surface generator (NL output) allowed to be an LLM? | **outside the research question** — the NL explanation side is [M5](../m5_presentation/README.md)'s demo; parked here, working answer kept |
| [Q8](#q8--ambiguous-parses) | Where do ambiguous NL parses go — branched on the IR, or rejected? | [S2.4.5](p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) |
| [Q9](#q9--ontology-provenance) | Per-puzzle declared ontology vs ontology inferred from text | [S2.2.4](p2.2_formalizer/s2.2.4_passes.md) — reframed as theory *selection* |
| [Q10](#q10--direct-llm--constraint) | When is direct LLM → constraint emission acceptable (skip the IR)? | [P2.8](p2.8_representations/README.md) — Stage L is this question asked properly |
| [Q11](#q11--link-grammar-value) | Does link-grammar enrich LLM input usefully, or is it dead weight? | [S2.6.4](p2.6_ablations/s2.6.4_representation_ablations.md), by experiment |
| [Q-M2.5](#q-m25--local-model-choice) | Which local model — Qwen3, Mistral, Phi-4, Gemma3, GLM? | [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) |
| [Q-M2.6](#q-m26--one-gbnf-or-many) | One GBNF grammar per task, or one grammar for everything? | [S2.2.3](p2.2_formalizer/s2.2.3_gbnf.md) |
| [Q-M2.7](#q-m27--what-language-is-the-frontend-written-in) | What language is the frontend written in? | [S2.1.3](p2.1_kernel_as_instrumentation/s2.1.3_boundary.md) |
| [Q-M2.1](#q-m21--when-is-the-kernel-frozen) | When is the kernel frozen? | [S2.5.1](p2.5_harness/s2.5.1_experiment_record.md) |
| [Q-M2.2](#q-m22--must-the-model-be-local) | Must the model be local? | [S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md) |
| [Q-M2.3](#q-m23--what-is-the-unit-of-faithfulness-without-a-gold-theory) | What is the unit of faithfulness without a gold theory? | [S2.4.4](p2.4_loop/s2.4.4_faithfulness.md) |
| [Q-M2.4](#q-m24--is-the-fixed-point-syntactic-or-semantic) | Is the fixed point syntactic or semantic? | [S2.4.1](p2.4_loop/s2.4.1_state_machine.md) |
| [Q-M2.8](#q-m28--where-does-einaf-live) | Where does EinAf live? | **the user**; raised by the analysis, § 5.1 |
| [Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics) | Does the kernel grow a machine surface for diagnostics? | **the user**; raised by the analysis, § 5.2 |
| [Q-M2.10](#q-m210--what-does-ein-certifies-a-unique-model-mean-on-r) | What does *Ein certifies a unique model* mean on R? | **the user**; raised by the analysis, § 5.4 |
| [Q-M2.11](#q-m211--what-is-a-flagged-program-to-the-loop) | What is a flagged program to the loop? | **the user**; raised by the analysis, § 5.3 |

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

## Q-M2.5 — Local model choice

**Since (the analysis, 2026-11-25).** This was Q23 here. The host the work runs on has an RTX 4060 Laptop GPU
with 8 GB, 31 GB of RAM, Docker and Compose, `ollama` installed, no `llama-server` and no model file. A 30B model
does not fit that GPU, so the working answer below is not runnable there as written; the candidates are a list of
May 2026 and were not re-checked. The audit: [`analysis/audit_p2.2.md`](analysis/audit_p2.2.md).

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

## Q-M2.7 — What language is the frontend written in?

**Since (the analysis, 2026-11-25).** This was Q25 here. Still open, and both columns of the table below grew.
A Rust caller gets the whole answer as values (`ReadOut`, `Verdict`, root's debts and core;
[`04-api.md`](../../docs/ein/ein.rs/04-api.md)) and a parse error with every error, its notes and its repairs;
the four load-side errors are still `String` newtypes. A caller that drives the binary gets `ein-summary/2` (the
whole read-out), `ein-answer/3` (each state as a loadable file), `ein-lattice/2`, `ein-test-report/6`,
`ein-events/1` and `h-space.json`, and **no canonical dump of a program**: `ein_ir::dump_canonical` is
library-only, and five tasks of P2.2 and P2.4 key on it. The stage files of P2.2 and P2.5 already write Python
paths while this is open. The text below is as written 2026-08-23: `--json-summary` there is `ein-summary/1`,
`07-diagnostics.md`'s § 4 is its § 2, and whether the protocol of S2.1.2 is built at all is
[Q-M2.9](#q-m29--does-the-kernel-grow-a-machine-surface-for-diagnostics).
The audit: [`analysis/audit_p2.1.md`](analysis/audit_p2.1.md).

**Raised 2026-08-23 by M1a S1a.9.4
T1a.9.4.6**, which found this milestone's plan asserting a boundary that had
been deferred out from under it.

**The premise that is gone.** M2's documents assumed a frontend calling the
engine in-process, and that needed no argument. It is now open:

- **PyO3 is not the boundary**: the binding was deferred 2026-08-21 for want
  of a consumer
  ([Q-M1a.23](../../docs/decisions/engine/Q-M1a.23-ein-has-no-binding-for-another-language.md)),
  and *this milestone was the last candidate*. There is also **no socket** —
  the server was dropped 2026-08-18; the engine ships as a library plus a CLI.
- **The llama.cpp argument was never a language argument.**
  The LLM infra ([S2.2.2](p2.2_formalizer/s2.2.2_llama_server_and_client.md))
  is a `llama-server` container and a thin HTTP client. The pattern it mirrors
  is acva's, and acva's client is **C++**.

**The two live options.**

| | how it reaches the engine | what it gets |
|---|---|---|
| **Rust** | links `ein-ir` / `ein-infer` as crates ([`docs/ein/ein.rs/04-api.md`](../../docs/ein/ein.rs/04-api.md)) | structured diagnostics as *values* — `ParseError` with its location, `KbLoadError` with the accumulated problems. No boundary, no mirrored data model, no exception hierarchy to design |
| **Python** | drives the `ein` binary — `--json-summary` for the verdict and counters, `--events` for the narration | the ecosystem, and a frontend that can be rewritten without touching the engine. Diagnostics are **strings**: [`07-diagnostics.md` §1/§4](../../docs/ein/user/04-reference/07-diagnostics.md#1-parse-and-load-diagnostics) pins them as such, and the CLI cannot grow a structured surface without breaking that |

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
queued: M1d changes what the engine says
at a depth cap ([Q-M1d.6](../../docs/decisions/answer/Q-M1d.6-contradiction-is-said-only-where-no-node-is.md))
and may change the traversal (Q-M1d.4),
and M1c may find a stdlib rule wrong.
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

**Since (the analysis, 2026-11-25).** One new fact: the host's GPU has 8 GB
([Q-M2.5](#q-m25--local-model-choice)), which bounds what *local* can mean there.

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

## Q-M2.8 — Where does EinAf live?

**Raised 2026-11-25 by the analysis** ([§ 5.1](analysis/README.md#5-what-the-standing-rules-now-ask-that-the-plan-does-not-answer);
[`audit_p2.5_p2.6.md`](analysis/audit_p2.5_p2.6.md) C.1 has the check run on sample paths). **The user's.**

The plan puts a harness in `einaf/`, its pages in `docs/einaf/`, grammars in `grammars/`, a benchmark in
`benchmark/`, records in `experiments/` and a Compose file in `packaging/`. Since it was written the repository
became two things with a one-way dependency
([Q-M1f.345](../../docs/decisions/docs/Q-M1f.345-nothing-in-the-project-reads-links-or-names.md)): **the project**
(`README.md`, `ein.rs/`, `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/` but `docs/lib/`), **the process**
(`plans/`, `utils/`, the gate, CI, `AGENTS.md`), and three trees **external to both** (`c/`, `nl2smt/`,
`docs/lib/`). None of EinAf's trees is in any list, and `utils/check_release_voice.py` takes a new top-level
directory as the project's.

| | what it means | what it costs |
|---|---|---|
| **(a) external to both** | one tree at the root, `einaf/`, holding the harness, prompts, grammars, the benchmark and the records; named in `AGENTS.md`'s external list and in the check's `NOT_PROJECT`; its own tests, which the gate does not run | the project never names it, so a kernel page cannot cite a record or the loop; *re-certified by the gate* becomes *checked by EinAf's own command*, which runs `ein test` over the benchmark's programs |
| **(b) the project** | under `docs/`, `examples/`, a crate | release voice (no date, no stage id, and **the research plan's F0 to F8 match the followup-id pattern**), no measurement in a page, current state only: records cannot live there. Under a corpus root every benchmark program needs a `corpus.toml` entry (the plan counts 4 080) and joins every sweep |
| **(c) a repository of its own** | EinAf beside Ein, pinning an Ein commit | the cleanest separation and the freeze of Q-M2.1 for free; two repositories for one user, and the plans stay here |
| **(d) the process** | code under `utils/`, records under `plans/` | records are deleted with the folder when the milestone closes; `utils/` is the measurement set, not a system |

**Recommendation: (a).** `nl2smt/` and `c/` are the precedent for code about Ein that the gate does not run; the
records and the benchmark are data that must outlive the plan's folder; the kernel stays free of any name of the
loop; and the feedback levels keep the research plan's names. What the kernel itself must grow is then a project
change made in the project's own terms (Q-M2.9). The benchmark's canonical programs stay out of the corpus roots,
and a handful of them may be copied into `examples/` as examples in their own right.

## Q-M2.9 — Does the kernel grow a machine surface for diagnostics?

**Raised 2026-11-25 by the analysis** ([§ 3, P2.1](analysis/README.md#p21--the-kernel-as-instrumentation-audit);
[`audit_p2.1.md`](analysis/audit_p2.1.md) § 0 and C.6). **The user's**, because it decides whether M2 changes the
kernel.

Stage A3 says *human-readable CLI output should not be used by the experimental system*. Today most of what the
loop reads is on a machine surface already, `ein-summary/2`. **One thing is not: a diagnostic.** A parse or load
refusal is text on stderr and writes no summary (`ein test --json-report` says `outcome: "error"` and no message);
a `-T` abort is one stderr line, exit 2, no summary; a load's `note:` lines are stderr only; a core's `:source`
quote is in no key; under `--single` the core is in no key. The same gap is
[F23](../followups/f23_diagnostics_in_eins_words.md)'s D-18 and L-125.

| | what it means | what it costs |
|---|---|---|
| **(a) nothing in the kernel changes** | the harness reads the exit code, stderr's lines (pinned as normative by `07-diagnostics.md`) and the summary, and builds the feedback object from the three | the experiment parses prose, which A3 forbids; exit 1 is four things and exit 2 two; a load refusal is one line of findings joined by `; `, some placed, some `at None` |
| **(b) the kernel grows the surface, in its own terms** | F23's items taken up as a change to the project: a refusal, an abort and a run's notes reach JSON, with a kind and a place where the source has one; no page or comment names the loop. The feedback object stays the harness's, a view over that and `ein-summary/2` | a kernel change under the kernel's rules: `help_shape.txt` and `06-config.md`'s census if it is an option, `07-diagnostics.md`, `08-read-out.md`, the `.expected` fixtures if places are added; and the kernel is no longer frozen at M2's start |
| **(c) the plan's `--feedback` in `ein-cli`** | `ein-feedback/1` produced by the binary, with the outcome vocabulary inside it | everything of (b), and the loop's vocabulary (the six outcomes, the ladder's blocks) lives in the project, which may not name it |

**Recommendation: (b)**, small: the three stderr-only things on a JSON surface, and nothing of the loop's
vocabulary in the kernel. The plan's own acceptance line, *nothing in `ein-infer`'s semantics changes*, still
holds under it. If (a) is ruled, the harness's reading of stderr is the documented exception to A3 and is tested
against the corpus's negative fixtures.

## Q-M2.10 — What does *Ein certifies a unique model* mean on R?

**Raised 2026-11-25 by the analysis** ([§ 2](analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk),
[§ 5.4](analysis/README.md#5-what-the-standing-rules-now-ask-that-the-plan-does-not-answer);
[`audit_p2.4.md`](analysis/audit_p2.4.md), [`audit_p2.5_p2.6.md`](analysis/audit_p2.5_p2.6.md) B.4). **The
user's**: it defines the milestone's two central numbers, `P(correct | certified)` and `P(faithful | certified)`.

The plan writes it `k = 1 ∧ exhausted`. Neither field exists. On R there are three things it could be:

| | the question asked | what it says of a program that states no obligation |
|---|---|---|
| **(a) one state in R** | `ein solve --single` answers `Solution`, decided (7 saturations on `zebra2`) | `Ambiguity`: ∅ and every alive partial commitment are in R (`features/11_expect_ambiguity.ein`: two full seatings, `k_s = 7`) |
| **(b) one maximal state** | `--maximal --single` answers `Solution` | `Solution` where exactly one full assignment survives, whatever the program owes |
| **(c) a description** | a bare solve, `k_s = 1 ∧ exhausted` | as (a), at the cost of describing R (120 saturations on `zebra2`; on the clue-dropped variant about 10 s, stopping at the default budget with `exhausted false` after all 32 states are found) |

Under (a) and (c) a formalizer that writes every clue and no theory of what must be filled is told *several
states*, and has to select the theory that owes (`std.slots`, a `total` relation); under (b) the kernel reads
past the omission.

**Recommendation: (a) as the definition, (b) reported beside it.** A program that does not say what it owes has
not said the puzzle, and the kernel's reply is the feedback the loop exists to give. Every canonical program of
the benchmark states its obligations and its gold as `:expect`, so `ein test` checks the ground truth. The loop
asks `--single` first and describes only where a feedback level needs the states or the core. A flagged program
is never certified (Q-M2.11). Reporting (b) beside (a) measures how often the difference is the missing theory.

## Q-M2.11 — What is a flagged program to the loop?

**Raised 2026-11-25 by the analysis** ([§ 5.3](analysis/README.md#5-what-the-standing-rules-now-ask-that-the-plan-does-not-answer)).
**The user's.**

A program the static check flags as not monotone loads, is answered, and **nothing is claimed of its answer**
([Q-M1f.564](../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)): the read-out's
first row, stderr's first line and `monotonicity.warning` say so, and `monotonicity.sites` names where. A
formalizer will write such programs: a closed-world reading (*the youngest is the one nobody is younger than*,
a *can* question, ProofWriter's closed-world assumption) is an `absent` over a relation a commitment can fill.
The plan has no case for it, in Stage A1's six outcomes, the state machine, *certified*, or the benchmark's gold.
What a flagged program's answer is belongs to [M1n](../m1n_non_monotone_programs/README.md).

- **(a) An outcome of its own.** Never certified; the sites are fed back as a diagnostic at the levels that carry
  one; counted in every table as its own column. The benchmark's canonical programs are never flagged, as no
  program under `tests/` is.
- **(b) Invalid.** Treated as a refusal: the loop repairs or stops.
- **(c) Its answer is read like any other**, with the warning recorded.

**Recommendation: (a).** (c) claims what the kernel declines to claim. (b) throws away the one thing the kernel
does say, where the absence is read; and how often a formalizer reaches for a closed-world reading is itself a
finding about the families that need one.
