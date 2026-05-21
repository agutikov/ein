# P1.7 — Bootstrapping: Zebra puzzle end-to-end

**Estimate:** 1-2 weeks.
**Depends on:** P1.1, P1.2, P1.3, P1.4, P1.5, P1.6.
**Blocks:** M1 done.

## Goal

Make the Zebra puzzle the engine's *first real test* — and the
M1 acceptance gate. Encode it in IR; solve it; produce a trace
that matches the human walkthrough from
[`docs/ideas/08`](../../../docs/ideas/08-human-style-deductive-trace.md);
audit which knowledge is hardcoded in code vs declared in IR;
document the decisions for future puzzles.

This is also where we discover all the gaps that the earlier
phases optimistically deferred. Reserve a third of the phase for
that.

## Stages

| ID      | Title                                  | Duration |
|---------|----------------------------------------|----------|
| S1.7.1  | Zebra IR + ontology                    | 2-3 days |
| S1.7.2  | Dynamic-vs-hardcoded audit + decision + proto-library | 4-6 days |
| S1.7.3  | Trace-matches-human acceptance         | 3-4 days |

## Acceptance

This phase *is* the M1 acceptance. The full set:

1. `ein-bot solve examples/zebra.ein --trace=zebra.md
    --diagrams=zebra-out/` exits 0 with the canonical answer:

   > The **Japanese** keeps the **zebra**. The **Norwegian**
   > drinks **water**.

2. The markdown trace satisfies the *named-rule-firing checklist*
   from [idea 08](../../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
   — every human move (direct fact, composition, elimination,
   forward chaining, case analysis, reductio, symmetry) appears
   as a named rule firing with provenance.

3. `ein-bot query examples/zebra.ein --mode=gaps`, after deleting
   condition (15), returns at least one diverging goal node.

4. `ein-bot query examples/zebra-bad.ein --mode=contradictions`,
   where `zebra-bad.ein` adds `(rel has-color House_1 Green)` to
   the original, returns a 2- or 3-edge unsat core that includes
   the offending fact and condition (5).

5. Pytest suite total ≥ 100 tests; `ruff check .` green; `pytest`
   wall time < 30 s.

## Connections

- The whole P1.7 is the operational check that
  [idea 05](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md) and
  [idea 06](../../../docs/ideas/06-inference-rules-completeness.md)
  produced an implementable design.
- [Idea 08 §The target trace](../../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
  is the canonical regression target.
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — all three
  task classes get exercised here.
