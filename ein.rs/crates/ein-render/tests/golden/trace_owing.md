# Region trace

```text
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers
```

## Root — 2 steps

### Step 1 — `infer-closure`

> r is functional + total → its extension is fixed (operational closure)

Premises: `functional(r)`, `total(r)`

Derives `__closed__(r)`.

<dot>

### Step 2 — `closed-observed`

> r's extension is closed

Premises: `__closed__(r)`

Derives `closed-here(r)`.

<dot>

## Owing — 1 node with no alive node one fact up

- **∅** owes
  - r owes a1 a B.
