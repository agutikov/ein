# P1.1 — IR language

**Estimate:** 1-2 weeks.
**Depends on:** nothing.
**Blocks:** every other M1 phase.

## Goal

Design a small homoiconic IR — S-expressions with semantic load on
atoms — for problems, ontologies, rules, constraints, queries, and
*trace steps*. The IR is the single substrate that the parser, the
graph engine, the rule registry, the trace renderer, and (later) the
NL frontend and the SMT emitter all read and write.

The design is locked here so the rest of M1 has something stable to
build against. Per
[`docs/ideas/02-graph-as-formal-substrate.md`](../../../docs/ideas/02-graph-as-formal-substrate.md)
the IR is *also* the working memory of the reasoner — not a discarded
intermediate.

## Stages

| ID      | Title                       | Duration  |
|---------|-----------------------------|-----------|
| S1.1.1  | Grammar design              | 3-4 days  |
| S1.1.2  | Parser + serialiser         | 3-4 days  |
| S1.1.3  | Round-trip tests + golden   | 1-2 days  |

## Acceptance

- A `docs/ir.md` spec listing every form with a one-line description
  and a worked example.
- `ein_bot.ir.parse(text) -> IRNode`,
  `ein_bot.ir.dump(node) -> str`, idempotent under
  `dump ∘ parse ∘ dump`.
- The Zebra puzzle is expressible in `examples/zebra.ein`; the parser
  accepts it (the puzzle is *expressed*, not solved yet).
- Pytest suite covers every form + at least 5 syntactic error
  fixtures.

## Connections

- [Idea 01](../../../docs/ideas/01-self-modifying-constraint-language.md) §point 3:
  Lisp / SMT-LIB / miniKanren reference templates.
- [Idea 04](../../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md):
  the IR is the *target* of the NL frontend (M2).
- The SMT-LIB analogy is intentional — M3's translator stays close
  to a structural rewrite.
