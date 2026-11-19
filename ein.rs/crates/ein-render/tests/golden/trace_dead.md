# Region trace

```text
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint holds a contradiction
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer
```

## Root — 2 steps

### Step 1 — `irreflexive`

> (irreflexive loops) violated: self-loop loops(A,A).

Premises: `loops(A, A)`

Derives `false`.

<dot>

### Step 2 — `std.core.inconsistent`

> a state that derives a contradiction is inconsistent

Premises: `false`

States `:ein inconsistent`: the state is inconsistent, and the step derives no fact.

<dot>

## N — 1 minimal dead sets

<details>
<summary>∅ — dead</summary>

Core: `(loops A A)`

Refuted by `irreflexive`.

</details>
