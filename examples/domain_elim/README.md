# `domain_elim/` — S1.5b.32 measurement fixtures

Minimal exactly-one puzzle (one bijective `color-loc` between
`{Red,Green,Blue}` and `{H1,H2,H3}`, anchors `Red@H2` + `Green@H3`,
goal `(color-loc Blue ?h)` → `H1`) used to compare the two ways the
engine reaches the same positive:

- **A** — the `domain-/range-elimination` saturation rule (fires at
  d=0 once alternatives are excluded).
- **B** — explicit hypothesis exploration (fork on each candidate,
  refute the rest).

All three fixtures hold the hypothesis generator constant
(`:hrules (guess (color-loc Color House))`) and vary only the rule
library, so each comparison moves a single variable:

| fixture | elimination rules (A) | negative-completion | solved by |
|---------|:---:|:---:|-----------|
| `ab.ein` | ✓ | ✓ | pathway A, at root saturation |
| `b_only.ein` | — | ✓ | forced-positive promotion |
| `b_branch.ein` | — | — | real forking + refutation (lookahead off) |

Run the measurement:

```sh
python3 utils/s1_5b_32_measure.py
```

Full analysis + results tables:
[`docs/kernel/inference/domain_elim_vs_hypothesis.md`](../../docs/kernel/inference/domain_elim_vs_hypothesis.md).
