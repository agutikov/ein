# Region trace

```text
  asked             the maximal line — every state on it
  maximal line      2 states — all of them
  exhausted         yes — this is all of the maximal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          46 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              141 saturations · 794 facts beyond root
```

## Root — 16 steps

### Step 1 — `symmetric`

> co-located sym

Premises: `co-located(Red, H1)` — from (1)

Derives `co-located(H1, Red)` — H1 is co-located with Red

<dot>

### Step 2 — `sibling-exclusive`

> sib T

Premises: `is-a(Color, T)`, `is-a(House, T)`

Derives `not(co-located(Color, House))`.

<dot>

### Step 3 — `sibling-exclusive`

> sib T

Premises: `is-a(House, T)`, `is-a(Color, T)`

Derives `not(co-located(House, Color))`.

<dot>

### Step 4 — `sibling-exclusive`

> sib Color

Premises: `is-a(Red, Color)`, `is-a(Blue, Color)`

Derives `not(co-located(Red, Blue))`.

<dot>

### Step 5 — `sibling-exclusive`

> sib Color

Premises: `is-a(Red, Color)`, `is-a(Green, Color)`

Derives `not(co-located(Red, Green))`.

<dot>

### Step 6 — `sibling-exclusive`

> sib Color

Premises: `is-a(Blue, Color)`, `is-a(Red, Color)`

Derives `not(co-located(Blue, Red))`.

<dot>

### Step 7 — `sibling-exclusive`

> sib Color

Premises: `is-a(Blue, Color)`, `is-a(Green, Color)`

Derives `not(co-located(Blue, Green))`.

<dot>

### Step 8 — `sibling-exclusive`

> sib Color

Premises: `is-a(Green, Color)`, `is-a(Red, Color)`

Derives `not(co-located(Green, Red))`.

<dot>

### Step 9 — `sibling-exclusive`

> sib Color

Premises: `is-a(Green, Color)`, `is-a(Blue, Color)`

Derives `not(co-located(Green, Blue))`.

<dot>

### Step 10 — `sibling-exclusive`

> sib House

Premises: `is-a(H1, House)`, `is-a(H2, House)`

Derives `not(co-located(H1, H2))`.

<dot>

### Step 11 — `sibling-exclusive`

> sib House

Premises: `is-a(H1, House)`, `is-a(H3, House)`

Derives `not(co-located(H1, H3))`.

<dot>

### Step 12 — `sibling-exclusive`

> sib House

Premises: `is-a(H2, House)`, `is-a(H1, House)`

Derives `not(co-located(H2, H1))`.

<dot>

### Step 13 — `sibling-exclusive`

> sib House

Premises: `is-a(H2, House)`, `is-a(H3, House)`

Derives `not(co-located(H2, H3))`.

<dot>

### Step 14 — `sibling-exclusive`

> sib House

Premises: `is-a(H3, House)`, `is-a(H1, House)`

Derives `not(co-located(H3, H1))`.

<dot>

### Step 15 — `sibling-exclusive`

> sib House

Premises: `is-a(H3, House)`, `is-a(H2, House)`

Derives `not(co-located(H3, H2))`.

<dot>

### Step 16 — `symmetric`

> co-located sym

Premises: `co-located(H1, Red)`

Derives `co-located(Red, H1)` — Red is co-located with H1

<dot>

## State 1 of 2 — 4 candidates · maximal · from {(co-located Blue H2), (co-located Green H3), (co-located H2 Blue), (co-located H3 Green)} — 4 steps

Assuming **(co-located Blue H2), (co-located Green H3), (co-located H2 Blue), (co-located H3 Green)**.

### Step 1 — `symmetric`

> co-located sym

Premises: `co-located(Blue, H2)`

Derives `co-located(H2, Blue)` — H2 is co-located with Blue

<dot>

### Step 2 — `symmetric`

> co-located sym

Premises: `co-located(Green, H3)`

Derives `co-located(H3, Green)` — H3 is co-located with Green

<dot>

### Step 3 — `symmetric`

> co-located sym

Premises: `co-located(H2, Blue)`

Derives `co-located(Blue, H2)` — Blue is co-located with H2

<dot>

### Step 4 — `symmetric`

> co-located sym

Premises: `co-located(H3, Green)`

Derives `co-located(Green, H3)` — Green is co-located with H3

<dot>

## State 2 of 2 — 4 candidates · maximal · from {(co-located Blue H3), (co-located Green H2), (co-located H2 Green), (co-located H3 Blue)} — 4 steps

Assuming **(co-located Blue H3), (co-located Green H2), (co-located H2 Green), (co-located H3 Blue)**.

### Step 1 — `symmetric`

> co-located sym

Premises: `co-located(Blue, H3)`

Derives `co-located(H3, Blue)` — H3 is co-located with Blue

<dot>

### Step 2 — `symmetric`

> co-located sym

Premises: `co-located(Green, H2)`

Derives `co-located(H2, Green)` — H2 is co-located with Green

<dot>

### Step 3 — `symmetric`

> co-located sym

Premises: `co-located(H2, Green)`

Derives `co-located(Green, H2)` — Green is co-located with H2

<dot>

### Step 4 — `symmetric`

> co-located sym

Premises: `co-located(H3, Blue)`

Derives `co-located(Blue, H3)` — Blue is co-located with H3

<dot>
