# P2.1 — Investigations + decisions

**Estimate:** 2 weeks.
**Depends on:** M1 closed.
**Blocks:** P2.2-P2.5 (each depends on a decision made here).

## Goal

Resolve M2's open questions [Q7](../open_questions.md#q7--llm-as-surface-generator),
[Q8](../open_questions.md#q8--ambiguous-parses),
[Q9](../open_questions.md#q9--ontology-provenance),
[Q10](../open_questions.md#q10--direct-llm--constraint),
[Q23](../open_questions.md#q23--local-model-choice) — before
committing engineering work to them. Each resolution is a written
verdict under `docs/decisions/M2-*.md` and edits to the relevant
question.

## Stages

| ID       | Title                                          | Duration |
|----------|------------------------------------------------|----------|
| S2.1.1   | Survey + pin the open questions                | 1-2 days |
| S2.1.2   | Decision: ambiguous parses                     | 1 day    |
| S2.1.3   | Decision: ontology provenance                  | 1-2 days |
| S2.1.4   | Decision: NL-output via LLM?                   | 1 day    |
| S2.1.5   | Decision: when (if ever) direct LLM → SMT      | 1 day    |
| S2.1.6   | Decision: model + GBNF strategy                | 2-3 days |

These are *investigations*, not implementations — each ends with a
write-up. Pin the answer in the milestone's `open_questions.md`
and the relevant decision file.

## Acceptance

- Six decision documents committed under `docs/decisions/M2-*.md`.
- Each open question above marked resolved.
- A summary in [README.md](README.md) §Acceptance — done date + per-question outcome.

## Connections

- All four required ideas inform this phase, but specifically:
  [idea 04 §Open questions](../../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions),
  [idea 01](../../../docs/ideas/01-self-modifying-constraint-language.md).
