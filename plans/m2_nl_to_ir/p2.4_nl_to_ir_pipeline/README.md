# P2.4 — NL → IR pipeline

**Estimate:** 2-3 weeks.
**Depends on:** P2.1 (decisions), P2.2 (LLM client), P2.3 (GBNFs).
**Blocks:** P2.6 (the eval harness benchmarks this pipeline).

## Goal

Realise the pipeline sketched in
[`docs/ideas/04 §Sketch of the pipeline`](../../ideas/04-nlp-to-graph-to-solver-pipeline.md#sketch-of-the-pipeline)
end-to-end:

```
puzzle text
   │
   ▼
LLM front-end (constrained by ontology/facts GBNFs)
   │
   │  per-sentence structured facts (IR fragments)
   ▼
typed-ontology validator   ──→  reject / disambiguate
   │
   ▼
constraint hypergraph IR (with provenance edges + parse-hypotheses)
   │
   └──→  M1 engine (solve / gaps / contradictions)
```

The user's open question — *"can multi-pass with verifier help?"* —
is answered yes: every LLM-emitted fact is *checked* against the
ontology before it lands in the IR. Verification failures become
re-prompts.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S2.4.1  | Two-stage extraction (ontology + facts) | 4-5 days |
| S2.4.2  | Validator + re-prompt                  | 3-4 days |
| S2.4.3  | Ambiguity-as-hypothesis emission       | 3-4 days |
| S2.4.4  | End-to-end CLI: `Ein from-text`    | 2-3 days |

## Acceptance

- `Ein from-text examples/zebra.txt > examples/zebra.ein`
  emits IR that the M1 engine solves to the canonical answer.
- Same for Einstein puzzle text + one logic-grid puzzle text.
- Removing one Zebra sentence yields a GAPS-mode answer (engine
  reports ambiguity, not parse failure).
- Adding an explicit contradiction phrase in the text yields an
  IR that triggers CONTRADICTIONS mode.

## Connections

- [Idea 04](../../ideas/04-nlp-to-graph-to-solver-pipeline.md) — the architecture.
- [Idea 03](../../ideas/03-three-task-classes.md) — the three task classes get exercised at the NL boundary.
