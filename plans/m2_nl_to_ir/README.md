# M2 — NL → IR

**Estimate:** ~2 months (~8 weeks).
**Status:** next.
**Depends on:** [M1](../m1_core_graph_reasoning/README.md) closed —
the IR (P1.1) and the engine acceptance (P1.7) are M2's preconditions.
**Blocks:** none (M3 only needs the IR, not the NL pipeline).

## Goal

Take a natural-language puzzle statement and emit M1's IR, so the
graph engine can solve, find gaps, or find contradictions in
problems stated in English (or Russian — the user is bilingual).

Per [`docs/ideas/04-nlp-to-graph-to-solver-pipeline.md`](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md):
**do NOT go directly from NLP-tree to SMT**. The IR is the *third*
artefact in the pipeline:

```
puzzle text → (LLM + optional link-grammar) → IR → M1 engine
```

Use a **local LLM via llama.cpp + GBNF**, mirroring the runtime
pattern from `/home/user/work/acva` (separate `llama-server`
container; thin HTTP client; GBNF for output structure).

The NL frontend may keep alternative parses *alive* — ambiguous
sentences become hypotheses on the IR (which the M1 engine handles
exactly as it handles puzzle-level hypotheses).

## Phases

| ID    | Title                              | Duration | Folder |
|-------|------------------------------------|----------|--------|
| P2.1  | Investigations + decisions         | 2 wk     | [`p2.1_investigations/`](p2.1_investigations/) |
| P2.2  | LLM infra (acva-mirrored)          | 1-2 wk   | [`p2.2_llm_infra/`](p2.2_llm_infra/) |
| P2.3  | GBNF grammar for IR                | 1 wk     | [`p2.3_gbnf_for_ir/`](p2.3_gbnf_for_ir/) |
| P2.4  | NL → IR pipeline                   | 2-3 wk   | [`p2.4_nl_to_ir_pipeline/`](p2.4_nl_to_ir_pipeline/) |
| P2.5  | Link-grammar experiment            | 1-2 wk   | [`p2.5_link_grammar_experiment/`](p2.5_link_grammar_experiment/) |
| P2.6  | Evaluation harness                 | 1-2 wk   | [`p2.6_evaluation_harness/`](p2.6_evaluation_harness/) |

P2.1 is *literally* a phase of investigation — it gates the work in
P2.2-P2.5. Decision artefacts: a written verdict per major open
question, committed under `docs/decisions/M2-*.md`.

## Acceptance

M2 ships when:

1. `ein-bot from-text zebra.txt > zebra.ein` produces valid IR that
   the M1 engine solves to the canonical Zebra answer.
2. End-to-end on at least three puzzles: classical Zebra, Einstein
   variant, one logic-grid puzzle with a different ontology.
3. Ambiguous parses propagate as hypotheses (deletion of a critical
   word in the text yields M1 GAPS-mode output, not parse failure).
4. The pipeline is reproducible: model file SHA, prompt template,
   GBNF version, seed are all recorded in the trace.
5. Decision artefacts committed for each P2.1 question.

## Open questions

See [`open_questions.md`](open_questions.md) for the M2-scoped set
(Q7-Q11). Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

## Connections

- [Idea 04](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md) —
  the architecture sketch this milestone realises.
- [Idea 01](../../docs/ideas/01-self-modifying-constraint-language.md) —
  GBNF; the *self-modifying* loop is deferred to
  [followup F2](../followups/f2_self_modifying_language.md).
- [`/home/user/work/acva/`](../../../acva/) — runtime pattern;
  P2.2 mirrors its `llama-server` container layout.
