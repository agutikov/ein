# P1.8 — Improvements

**Estimate:** TBD.
**Status:** **placeholder** — three themes parked here for
post-P1.7 attention. Originally the home of the modules/imports
deferral from S1.3.0 R8 (2026-05-20); broadened 2026-05-21 to
also collect runtime optimisations the M1 implementation surfaced.
**Depends on:** varies per theme; see each §.
**Blocks:** nothing within M1 acceptance — P1.7 still gates "M1
done" regardless. P1.8 ships when one of its themes acquires
enough motivation (puzzle authoring grows past one file, perf
hits ergonomic thresholds, or a future puzzle's branching cost
overruns the laptop).

Directory name `p1.8_ein_lang_modules` is historical from when the
phase was modules-only; the file inside this directory is the
authoritative scope statement (rename deferred to avoid a churn
of cross-link breakage across S1.3.0, P1.3 README, etc.).

## Themes

### Theme A — Ein-lang modules + imports

The original P1.8 deferral from S1.3.0 R8.

P1.8 designs the **module / import / include mechanism** for
ein-lang and decides where rule libraries live across files. The
phase is the canonical home for:

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  — the question P1.3 deferred here.
- Any related decisions about cross-file references in `.ein`
  files (relation / type / instance imports, namespace scoping,
  re-exports, conflict resolution).

The P1.3 review surfaced three coupled questions:

1. Where do rule definitions live — inline per puzzle, in a
   universal library, or hybrid?
2. How does one `.ein` file reference rules in another?
3. How are conflicts resolved when a puzzle redefines a library
   rule?

These collectively form a *module-system design* too large for
P1.3 (rule engine) to absorb. P1.3 proceeds under the working
assumption that **rules are inline per puzzle** (current zebra.ein
convention); P1.8 revisits and decides.

Likely stages once activated:

- **S1.8.A1** — module-system design (file-level vs symbol-level
  imports; namespace scoping; conflict policy).
- **S1.8.A2** — grammar / parser extensions for `(import …)` /
  `(use …)` (or analogous) forms.
- **S1.8.A3** — loader changes: resolve imports, merge modules
  into a single `KnowledgeBase`, detect conflicts.
- **S1.8.A4** — rule library location (Q30 options a / b / c →
  final call). If (a) or (c): ship `examples/rules.ein` (or
  equivalent) with the kernel rule core.
- **S1.8.A5** — migration: zebra.ein adopts imports for any rules
  promoted to the universal library. If Q30 lands on (b), this
  stage is empty.

### Theme B — Copy-on-write hypothesis branching

P1.2's `KnowledgeBase.fork()` shallow-copies the `facts` list +
the four reverse-index dicts (O(|facts|) cost). For P1.5's
hypothesis loop, where each branch potentially forks many sub-
branches, that copy cost compounds.

**Insight (user direction 2026-05-21):** the M1 KB is **append-
only** — no deletes, no in-place writes, only appends. That makes
**true copy-on-write** trivially correct: a fork inherits the
parent's facts + indexes by reference; the fork's own appends
write to per-fork overlays; lookups consult the fork's overlays
first, then fall through to the parent.

Likely stages once activated:

- **S1.8.B1** — `ForkedKnowledgeBase` overlay class (or rework
  `KnowledgeBase` itself with a layered backing store).
- **S1.8.B2** — adjust the engine's `_facts_by_relation` lookups
  to traverse the parent chain transparently.
- **S1.8.B3** — saturator + contradiction-detector behaviour
  preserved (same `Firing` outputs, same `Contradiction` records).
- **S1.8.B4** — perf benchmark vs the shallow-copy fork (target:
  O(1) fork; lookup overhead ≤ depth × constant).

Out of scope: garbage-collecting collapsed branches (M1's append-
only model says we don't free anything). Out of M1 entirely.

### Theme C — Negative-fact volume reduction

P1.3 saturation produces **lots** of negative facts. Zebra.ein's
`(type-exclusivity co-located)` alone derives 120 `(not (co-located
A B))` facts; over the full saturation the REASONING layer holds
~120 not-facts vs ~25 positive derivations. The volume is
correct (each negative is a real consequence of the rule), but
much of it is *load-bearing only via the contradiction detector* —
the negatives are scanned for matching positives, never queried
directly.

**Possible optimisations** (each is a research direction, not a
quick win):

- **Lazy materialisation**: derive `(not X)` *on demand* when the
  contradiction detector asks rather than during saturation. Trade
  fact-volume for detector-time; the detector becomes a pull-style
  matcher.
- **Layer-aware filtering**: a `(not X)` derived from
  type-exclusivity is mechanically true given the puzzle's type
  declarations — it doesn't need to live in REASONING at all if no
  consumer reads it. Move to a virtual layer.
- **Goal-driven pruning**: combined with F7 §C (rule-set
  sufficiency), suppress type-exclusivity firings whose
  conclusions are demonstrably unconsumed by any subsequent rule
  *and* unconsumed by the query.
- **Compressed representation**: a *single* `(not (co-located ?
  ?_T))` fact with `?_T` denoting "any pair distinct under T",
  expanded lazily. Needs compound-node-kind support (Q26).

Likely stages once activated:

- **S1.8.C1** — measure: how much of the (not …) volume is
  actually consumed? Empirical study on Zebra + the demos.
- **S1.8.C2** — pick a representation (lazy / virtual layer /
  compressed) based on the measurement.
- **S1.8.C3** — refactor the saturator + detector + trace
  renderer to honour it.

Cross-cuts F7 §C (rule-set sufficiency) — the deepest win likely
combines volume reduction with activator-selection pruning.

## Acceptance (TBD per theme)

Each theme drafts its own when activated. Skeleton:

- **A:** a multi-file `.ein` project loads end-to-end; conflict
  policy documented; zebra.ein continues to work.
- **B:** `fork()` is O(1); zebra.ein saturation through a fork
  matches the non-fork baseline; perf benchmark green.
- **C:** Zebra REASONING-layer fact count drops by ≥ 50% with no
  loss of contradiction-detection power; saturation time within
  20% of pre-optimisation.

## Open questions

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  (Theme A).
- *(Theme B and Theme C have not yet surfaced specific Q-numbered
  questions; activation will probably introduce some.)*

## Cross-links

- Origin of Theme A (modules deferral):
  [`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30](../p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).
- Related followups:
  [F2 self-modifying language](../../followups/f2_self_modifying_language.md),
  [F5 rules as data](../../followups/f5_rules_as_data.md) (both
  would consume Theme A's module mechanism if it lands).
- Theme B context: P1.2's `KnowledgeBase.fork()` shallow-copy,
  P1.5 hypothesis branching's per-fork cost.
- Theme C context:
  [F7 — Rule taxonomy + rule induction](../../followups/f7_rule_induction.md) §C
  (rule-set sufficiency) is the upstream design partner.
