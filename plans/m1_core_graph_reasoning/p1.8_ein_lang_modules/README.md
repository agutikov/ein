# P1.8 — Ein-lang modules + imports

**Estimate:** TBD.
**Status:** **placeholder** — scope deferred from P1.3 review
([`s1.3.0_review_and_revisions.md`](../p1.3_inference_rules/s1.3.0_review_and_revisions.md))
on 2026-05-20.
**Depends on:** P1.1 (IR), P1.3 (rule engine).
**Blocks:** nothing within M1 acceptance — P1.7 still gates "M1
done" regardless. P1.8 ships when ein-lang authoring grows past a
single file per puzzle.

## What this phase owns

P1.8 designs the **module / import / include mechanism** for
ein-lang and decides where rule libraries live across files. The
phase is the canonical home for:

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  — the question P1.3 deferred here.
- Any related decisions about cross-file references in `.ein`
  files (relation / type / instance imports, namespace scoping,
  re-exports, conflict resolution).

## Why this is a separate phase

The P1.3 review surfaced three coupled questions:

1. Where do rule definitions live — inline per puzzle, in a
   universal library, or hybrid?
2. How does one `.ein` file reference rules in another?
3. How are conflicts resolved when a puzzle redefines a library
   rule?

These collectively form a *module-system design* that's too large
for P1.3 (rule engine) to absorb. P1.3 proceeds under the working
assumption that **rules are inline per puzzle** (current zebra.ein
convention); P1.8 revisits and decides.

## Scope (TBD)

To be drafted when P1.8 is activated. Likely contents:

- **Stage S1.8.1** — module-system design (file-level vs symbol-
  level imports; namespace scoping; conflict policy).
- **Stage S1.8.2** — grammar / parser extensions for `(import …)`
  / `(use …)` (or analogous) forms.
- **Stage S1.8.3** — loader changes: resolve imports, merge modules
  into a single `KnowledgeBase`, detect conflicts.
- **Stage S1.8.4** — rule library location (Q30 options a / b / c
  → final call). If (a) or (c): ship `examples/rules.ein` (or
  equivalent) with the kernel rule core.
- **Stage S1.8.5** — migration: zebra.ein adopts imports for any
  rules promoted to the universal library. If Q30 lands on (b),
  this stage is empty.

## Acceptance (TBD)

To be drafted when scope is finalised. Skeleton:

- A multi-file `.ein` project loads end-to-end.
- Conflict policy is documented and tested.
- Zebra.ein continues to work (either inline or via imports).
- `pytest` coverage on the new loader paths.

## Open questions

- [Q30 — Universal rule library + import mechanism](../open_questions.md#q30--universal-rule-library--import-mechanism)
  — the deferral that spawned this phase.

## Cross-links

- Origin of the deferral:
  [`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30](../p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).
- Related followups:
  [F2 self-modifying language](../../followups/f2_self_modifying_language.md),
  [F5 rules as data](../../followups/f5_rules_as_data.md) (both
  would consume the module mechanism if it lands).
