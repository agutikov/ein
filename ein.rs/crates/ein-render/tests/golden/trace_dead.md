# Region trace

```text
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             1, the alive down-set — L1 dead is upward-closed
  work              1 saturation · 0 facts beyond root · 1 layer
```

## Root — 1 steps

### Step 1 — `irreflexive`

> (irreflexive loops) violated: self-loop loops(A,A).

Premises: `loops(A, A)`

Derives `false`.

<dot>

## N — 1 minimal dead sets

<details>
<summary>∅ — dead</summary>

Core: `(loops A A)`

Refuted by `irreflexive`.

</details>
