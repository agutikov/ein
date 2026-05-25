# P1.3 — Inference rules: matcher, predicates, saturation engine

**Estimate:** 2-3 weeks.
**Depends on:** P1.1 (IR), P1.2 (graph + provenance + R9's
`Fact.args` widening).
**Blocks:** P1.5 (hypothesis loop uses these rules + Q40's
synthetic-fact emission), P1.6 (trace needs named rule firings).

## Goal

Implement the rule matcher + saturation engine + built-in
predicate registry. Rule firings produce new reasoning-layer
facts with full provenance (P1.2 S1.2.3).

P1.3 was substantially **rescoped 2026-05-20** by the
[S1.3.0 review](s1.3.0_review_and_revisions.md) against the
shipped P1.1 + P1.2:

- **Scope narrowed** to the 6 zebra.ein rules + `hypothesis-contradiction`
  (promoted by [Q40 Option A](s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13))
  = **7 rules total**. The other 4 from the original 10-rule
  taxonomy ([idea 06](../../../docs/ideas/06-inference-rules-completeness.md))
  are deferred to followups by [Q33](../open_questions.md#q33--predicate-primitives--minimal-set-for-m1):
  M1 ships only `eq` + `neq` predicates; no numeric / set /
  variadic primitives.
- **No separate Pattern / Rule / Registry classes** — `kb.Pattern`,
  `kb.Rule`, `kb.rules` are the canonical entities (P1.2).
- **No `examples/rules.ein` universal library in M1** — Q30
  deferred to [P1.8](../p1.8_ein_lang_modules/); rules live inline
  in each puzzle's `.ein` file (current zebra.ein convention).
- **No `:where` keyword** — Q32 dropped it; the predicate registry
  is the single source of truth for built-ins.
- **Module path**: flat `src/ein_bot/inference/` (Q39); tests at
  `tests/inference/`.

The acceptance criterion remains the
[idea 08](../../../docs/ideas/08-human-style-deductive-trace.md)
target: every named reasoning move in the human Zebra walkthrough
has a corresponding rule in zebra.ein and a corresponding firing
in the trace.

## Stages

| ID      | Title                                                          | Duration  |
|---------|----------------------------------------------------------------|-----------|
| S1.3.0  | Review of P1.3 against shipped P1.1/P1.2 + resolution plan      | meta — closed 2026-05-20 |
| S1.3.1  | Rule DSL + compiled matcher + predicate registry               | 4-5 days  |
| S1.3.2  | Rule families (6 zebra core + `hypothesis-contradiction`)       | 1-1.5 wk  |
| S1.3.3  | Saturation engine + banded priorities                          | 3-4 days  |

Total ≈ 2-2.5 wk; slightly shorter than the original estimate
because the 4 deferred rule families absorbed maybe half a week of
scope.

## Acceptance

- The **7 M1 rules** ship inline in `examples/zebra.ein`:
  `symmetric`, `transitive`, `implies`, `square-fwd`, `square-bwd`,
  `type-exclusivity`, `hypothesis-contradiction`. Each carries
  `:priority` (Q41 banded 100/200/300/900) and `:why` (Q31
  `{?var}` template).
- `pytest tests/inference/` — at least one positive + one
  negative test per rule family (≥ 25 tests).
- Running saturation on the Zebra `fact` layer fires the
  applicable rules in priority-band order
  (`propagate → derive → eliminate`); the trace shows the firings.
- Q40 Option A: a synthetic `(hypothesis (co-located Norwegian
  House-2))` fact is matched by `hypothesis-contradiction`'s
  pattern (nested-fact unification works end-to-end).
- Built-in predicate registry contains exactly `eq` + `neq` (Q33);
  no phantom `eq` / `neq` relations auto-vivified in
  `kb.relations` (loader consulted the registry).
- Per-puzzle rule-demo problems live at
  `examples/zebra/demos/<rule-name>/{a,b,c}.ein` — one rule fires
  per demo; documented in `examples/zebra/demos/README.md`
  (replaces the original `docs/rules/demos.md` location, which
  would have lived next to a non-existent universal library).
- `ruff check src/ein_bot/inference/ tests/inference/` green.
- 144 + 10 (R9) + new (P1.3) kb/inference tests all pass.

## Connections

- [S1.3.0 review](s1.3.0_review_and_revisions.md) — the audit +
  9 open-question resolutions that reshaped this phase.
- [Idea 06](../../../docs/ideas/06-inference-rules-completeness.md) —
  the original 10-rule taxonomy. M1 ships 6+1; the other 4 land
  in a followup adjacent to F4.
- [Idea 07 Reading C](../../../docs/ideas/07-categorical-formulation.md) —
  the DSL is a typed pattern → pattern rewrite; in CT terms a DPO
  rewrite. We don't *implement* DPO; we implement enough that an
  F1 formalisation can recognise it later.
- [P1.5 hypothesis loop](../p1.5_hypothesis_loop/) — Q40 Option A
  splits responsibility: P1.3 ships the `hypothesis-contradiction`
  rule + nested-fact matching; P1.5 emits the synthetic
  `(hypothesis …)` / `(contradiction-under …)` facts at
  fork / contradict moments and handles branch retraction.
- [`plans/ideas.md`](../../ideas.md) — live entry "P1.4
  constraints — collapse into rules with `(not …)` in `:assert`?"
  surfaced during S1.3.0; revisit P1.4 after P1.3 ships.
