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

Per [idea 08 §Open questions point 4](../../docs/ideas/08-human-style-deductive-trace.md#open-questions).

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

Per [idea 04 §Multiple-variant complication](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md#multiple-variant-complication)
and [idea 04 §Open questions point 3](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: branched on the IR. The NL frontend emits all
plausible parses, each guarded by a `(hypothesis-parse ?id …)`
wrapper that the M1 engine treats like any other hypothesis.
Hypothesis level 0 = parse choices; level 1+ = puzzle hypotheses.
Decided in P2.1 S2.1.2.

## Q9 — Ontology provenance

Per [idea 04 §Open questions point 2](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

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

Per [idea 04 §Open questions point 4](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: never the *default*. Allow as a
`--no-ir` debugging flag that prompts the LLM to emit SMT-LIB
directly and compares the solver answer to the IR pipeline's
answer; differences are bugs in the pipeline. Decided in P2.1 S2.1.5.

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
