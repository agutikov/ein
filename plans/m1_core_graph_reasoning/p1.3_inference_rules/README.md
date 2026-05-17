# P1.3 — Inference rules: registry + ten core families

**Estimate:** 2-3 weeks.
**Depends on:** P1.1 (IR), P1.2 (graph + provenance).
**Blocks:** P1.5 (hypothesis loop uses these rules), P1.6 (trace
needs named rule firings).

## Goal

Implement the rule registry and the ten rule families catalogued in
[`docs/ideas/06-inference-rules-completeness.md`](../../../docs/ideas/06-inference-rules-completeness.md).
Each rule is a structural pattern → conclusion declaration in the
IR's rule sub-language (P1.1 S1.1.1 T1.1.1.2). Rule firings produce
new reasoning-layer edges with provenance (P1.2 S1.2.3).

The acceptance criterion is set by
[`docs/ideas/08-human-style-deductive-trace.md`](../../../docs/ideas/08-human-style-deductive-trace.md):
every named reasoning move in the human Zebra walkthrough has a
corresponding rule in the registry.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S1.3.1  | Rule DSL + pattern matcher             | 4-5 days |
| S1.3.2  | Ten rule families                      | 1-2 wk   |
| S1.3.3  | Rule ordering + saturation engine      | 3-4 days |

## Acceptance

- `examples/rules.ein` holds the ten core rule definitions, each
  with `:why` template + worked example.
- `pytest tests/rules/` — at least one positive + one negative test
  per rule family (≥ 20 tests).
- Running saturation on the Zebra `fact` layer fires all ten rules
  at least once across the trace.

## Connections

- [Idea 06](../../../docs/ideas/06-inference-rules-completeness.md) —
  the rule taxonomy; the table there is the source of truth for
  S1.3.2.
- [Idea 07 Reading C](../../../docs/ideas/07-categorical-formulation.md) —
  the DSL is a typed pattern → pattern rewrite; in CT terms a DPO
  rewrite. We don't *implement* DPO; we implement enough that an F1
  formalisation can recognise it later.
