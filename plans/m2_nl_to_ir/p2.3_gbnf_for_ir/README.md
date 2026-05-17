# P2.3 — GBNF grammar for IR

**Estimate:** ~1 week.
**Depends on:** P1.1 (IR locked), P2.2 (llama-server up).
**Blocks:** P2.4 (pipeline uses these grammars).

## Goal

Lift the M1 IR (P1.1) into a GBNF grammar so the LLM can only emit
*valid* IR by construction. Then build a small library of
task-specific GBNFs (extracting ontology, extracting facts,
resolving definite descriptions, flagging ambiguity) that the
pipeline switches between.

Per [idea 01](../../../docs/ideas/01-self-modifying-constraint-language.md)
GBNF is the *syntactic firewall*. Semantic validity (type-correctness,
ontology coverage) is checked in P2.4.

## Stages

| ID      | Title                            | Duration |
|---------|----------------------------------|----------|
| S2.3.1  | IR-grammar generator             | 3-4 days |
| S2.3.2  | Task-specific GBNFs              | 2-3 days |

## Acceptance

- `python -m ein_bot.ir.to_gbnf > grammars/ir.gbnf` produces a
  grammar accepted by llama-server.
- A round-trip experiment: take the engine's
  `examples/zebra.ein`, dump-canonical it, feed it through the
  LLM constrained by `grammars/ir.gbnf` (with a no-op "echo" prompt)
  — the LLM emits a byte-identical (modulo whitespace) IR.
- Four task GBNFs in `grammars/`: `ontology.gbnf`, `facts.gbnf`,
  `definite-description.gbnf`, `ambiguity.gbnf`.

## Connections

- [Idea 01](../../../docs/ideas/01-self-modifying-constraint-language.md) —
  GBNF as syntactic substrate; the self-modifying loop ([F2](../../followups/f2_self_modifying_language.md))
  builds on this.
- [Idea 04](../../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md) —
  the GBNFs are the *constrained output* step of the pipeline.
