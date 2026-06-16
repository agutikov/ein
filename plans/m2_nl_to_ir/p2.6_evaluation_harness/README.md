# P2.6 — Evaluation harness

**Estimate:** 1-2 weeks.
**Depends on:** P2.4 (pipeline) + P2.5 (decision).
**Blocks:** M2 done.

## Goal

A reproducible benchmark over a small corpus of NL puzzles that
measures:

- IR extraction quality (structural F1 vs gold);
- end-to-end correctness (the M1 engine's answer matches the
  known-correct answer);
- trace quality (human review checklist, plus a structural
  audit against the gold trace where one exists).

Mirrors [idea 08 §Open questions point 1](../../ideas/08-human-style-deductive-trace.md#open-questions):
*"What evaluation harness? Compare generated traces against a
small corpus of human walkthroughs."*

## Stages

| ID      | Title                                | Duration |
|---------|--------------------------------------|----------|
| S2.6.1  | Benchmark corpus + gold IR/trace     | 4-5 days |
| S2.6.2  | Harness + CI integration             | 3-4 days |

## Acceptance

- `pytest tests/nl/benchmark/` runs the full corpus when
  `LLAMA_HOST` is set; reports a table of per-puzzle scores.
- CI smoke-checks the harness on a mocked LLM (no real model)
  so the harness itself doesn't break silently.
- Aggregate metrics committed in `docs/decisions/M2-final.md`
  alongside the milestone-done note.

## Connections

- [Idea 08](../../ideas/08-human-style-deductive-trace.md) —
  the harness operationalises the evaluation question.
