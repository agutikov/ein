# `domain_elim/` — one answer, reached two ways

Minimal exactly-one puzzle (one bijective `color-loc` between
`{Red,Green,Blue}` and `{H1,H2,H3}`, anchors `Red@H2` + `Green@H3`,
goal `(color-loc Blue ?h)` → `H1`), written three times to compare the two
ways the engine reaches the same positive:

- **A** — the `domain-/range-elimination` saturation rule (fires at
  root once every alternative is excluded).
- **B** — the hypothesis layer: `(color-loc Blue H1)` is a candidate, and
  each wrong house a commitment refutes.

All three fixtures hold the hypothesis generator constant (the rule
`guess` and its activator `(guess color-loc Color House)`) and vary only the rule
library, so each comparison moves a single variable:

| fixture | elimination rules (A) | negative-completion | what `ein solve` finds |
|---------|:---:|:---:|-----------|
| `ab.ein` | ✓ | ✓ | the answer at root: no candidate is left, R is {∅}, and `?h = H1` there |
| `b_only.ein` | — | ✓ | one candidate, `(color-loc Blue H1)`; R is ∅ and that commitment |
| `b_branch.ein` | — | — | seven candidates, six of them dead alone; R is ∅ and `{(color-loc Blue H1)}` |

Under B, root alone does not decide the goal: `?h = H1` holds in the state
that commits to it, and ∅ leaves it undetermined. Pathway A is what lets a
program answer at root with no search at all.
