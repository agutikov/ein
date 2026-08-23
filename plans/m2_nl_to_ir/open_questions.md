# M2 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

## Index

| Q   | Title                                                                       | Resolved in |
|-----|-----------------------------------------------------------------------------|-------------|
| Q7  | Is the surface generator (NL output) allowed to be an LLM?                  | P2.1 S2.1.4 |
| Q8  | Where do ambiguous NL parses go — branched on the IR, or rejected?          | P2.1 S2.1.2 |
| Q9  | Per-puzzle declared ontology vs ontology inferred from text                 | P2.1 S2.1.3 |
| Q10 | When is direct LLM → constraint emission acceptable (skip the IR)?          | P2.1 S2.1.5 |
| Q11 | Does link-grammar enrich LLM input usefully, or is it dead weight?          | P2.5 (experiment) |
| Q23 | Which local model — Qwen3, Mistral, Phi-4, Gemma3, GLM?                     | P2.2 S2.2.2 |
| Q24 | One GBNF grammar per task, or one grammar for everything?                   | P2.3 |

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
prompt that cannot change the rule firings). C never. Decided in
P2.1 S2.1.4.

## Q8 — Ambiguous parses

Per [idea 04 §Multiple-variant complication](../ideas/04-nlp-to-graph-to-solver-pipeline.md#multiple-variant-complication)
and [idea 04 §Open questions point 3](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: branched on the IR. The NL frontend emits all
plausible parses, each guarded by a `(hypothesis-parse ?id …)`
wrapper that the M1 engine treats like any other hypothesis.
Hypothesis level 0 = parse choices; level 1+ = puzzle hypotheses.
Decided in P2.1 S2.1.2.

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
as escape hatch. Decided in P2.1 S2.1.3.

## Q10 — Direct LLM → constraint?

Per [idea 04 §Open questions point 4](../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: never the *default*, and with the SMT milestone
dropped (2026-08-18) there is no in-repo solver path to compare against
either. If the question is revisited it is as a *diagnostic*: prompt the
LLM to emit constraints for an external solver and check that its answer
agrees with the IR pipeline's; any difference is a bug in the pipeline.
Decided in P2.1 S2.1.5.

## Q11 — Link-grammar value

The user's open question: *"does feeding link-grammar output to
the LLM enrich the input usefully?"*

**Working answer**: unknown. P2.5 runs the experiment with a small
benchmark (~10 puzzles), with metric = correctness of generated IR
on a gold set. Default deployment is no-link-grammar unless P2.5
ships a measurable improvement.

## Q23 — Local model choice

**Working answer**: Qwen3-30B-Instruct as the primary; Phi-4 14B as
the fallback. Both are GBNF-friendly and bilingual EN/RU. Decided
in P2.2 S2.2.2 with a benchmark on the gold IR set.

## Q24 — One GBNF or many?

**Working answer**: many — one GBNF per task class (ontology
extraction, fact extraction, ambiguity-flag, definite description
resolution). Smaller grammars decode faster and let the prompt
focus the LLM. Decided in P2.3.

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
  [P2.2](p2.2_llm_infra/README.md) is a `llama-server` container and a thin
  HTTP client. The pattern it mirrors is acva's, and acva's client is **C++**.

**The two live options.**

| | how it reaches the engine | what it gets |
|---|---|---|
| **Rust** | links `ein-ir` / `ein-infer` as crates ([`docs/api/rust.md`](../../docs/api/rust.md)) | structured diagnostics as *values* — `ParseError` with its location, `KbLoadError` with the accumulated problems. No boundary, no mirrored data model, no exception hierarchy to design |
| **Python** | drives the `ein` binary — `--json-summary` for the verdict and counters, `--events` for the narration | the ecosystem, and a frontend that can be rewritten without touching the engine. Diagnostics are **strings**: [`defined_behaviour.md` §1/§4](../../docs/kernel/defined_behaviour.md) pins them as such, and the CLI cannot grow a structured surface without breaking that |

**Why it is not decided here.** The strongest argument for one option is
[S2.4.2](p2.4_nl_to_ir_pipeline/s2.4.2_validator_reprompt.md)'s
validator/repair loop, written as `validate(facts, ontology)` and needing
*why* a load failed rather than the message text — which is the Rust column,
exactly. The strongest argument for the other is that everything *around* the
engine in this milestone is HTTP, JSON and prompt templates, where Python is
the shorter road. That is a trade this milestone owns, and
[P2.1](p2.1_investigations/README.md) is where it is made. **What S1a.9.4 fixed
is only that the plan no longer asserts an answer.**

**Also settled while passing through, because it is cheap to write down and
expensive to rediscover:** `ollama` is **not** an alternative to
`llama-server` for this milestone. GBNF is the mechanism
([P2.3](p2.3_gbnf_for_ir/README.md),
[idea 01](../ideas/01-self-modifying-constraint-language.md)); llama.cpp's
server takes a `grammar` field, and ollama's API exposes only JSON-schema
`format`.
