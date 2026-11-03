# Region trace

```text
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 1
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      1 state
  components        1
  sub-cubes         1, of dimension 0
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 0 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers
```

## Root — 23 steps

### Step 1 — `bijective-properties`

> color-loc is bijective ⟹ functional ∧ injective ∧ total ∧ surjective.

Premises: `bijective(color-loc)`

Derives `functional(color-loc)`.

<dot>

### Step 2 — `derive-functional-negative`

> color-loc functional ⟹ functional-negative active.

Premises: `functional(color-loc)`

Derives `functional-negative(color-loc)`.

<dot>

### Step 3 — `derive-domain-elimination`

> color-loc functional + total ⟹ domain-elimination active.

Premises: `functional(color-loc)`, `total(color-loc)`

Derives `domain-elimination(color-loc)`.

<dot>

### Step 4 — `derive-injective-negative`

> color-loc injective ⟹ injective-negative active.

Premises: `injective(color-loc)`

Derives `injective-negative(color-loc)`.

<dot>

### Step 5 — `derive-range-elimination`

> color-loc injective + surjective ⟹ range-elimination active.

Premises: `injective(color-loc)`, `surjective(color-loc)`

Derives `range-elimination(color-loc)`.

<dot>

### Step 6 — `functional-negative`

> color-loc functional: (color-loc Red H2) ⟹ (not (color-loc Red H1)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(H1, House)` — from (1) Red is in house 2

Derives `not(color-loc(Red, H1))`.

<dot>

### Step 7 — `functional-negative`

> color-loc functional: (color-loc Red H2) ⟹ (not (color-loc Red H3)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(H3, House)` — from (1) Red is in house 2

Derives `not(color-loc(Red, H3))`.

<dot>

### Step 8 — `functional-negative`

> color-loc functional: (color-loc Green H3) ⟹ (not (color-loc Green H1)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(H1, House)` — from (2) Green is in house 3

Derives `not(color-loc(Green, H1))`.

<dot>

### Step 9 — `functional-negative`

> color-loc functional: (color-loc Green H3) ⟹ (not (color-loc Green H2)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(H2, House)` — from (2) Green is in house 3

Derives `not(color-loc(Green, H2))`.

<dot>

### Step 10 — `injective-negative`

> color-loc injective: (color-loc Red H2) ⟹ (not (color-loc Green H2)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)` — from (1) Red is in house 2

Derives `not(color-loc(Green, H2))`.

<dot>

### Step 11 — `injective-negative`

> color-loc injective: (color-loc Red H2) ⟹ (not (color-loc Blue H2)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)` — from (1) Red is in house 2

Derives `not(color-loc(Blue, H2))`.

<dot>

### Step 12 — `injective-negative`

> color-loc injective: (color-loc Green H3) ⟹ (not (color-loc Red H3)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)` — from (2) Green is in house 3

Derives `not(color-loc(Red, H3))`.

<dot>

### Step 13 — `injective-negative`

> color-loc injective: (color-loc Green H3) ⟹ (not (color-loc Blue H3)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)` — from (2) Green is in house 3

Derives `not(color-loc(Blue, H3))`.

<dot>

### Step 14 — `domain-elimination`

> every House except H2 excluded for Red → (color-loc Red H2).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)`, `is-a(H2, House)`

Derives `color-loc(Red, H2)`.

<dot>

### Step 15 — `domain-elimination`

> every House except H3 excluded for Green → (color-loc Green H3).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)`, `is-a(H3, House)`

Derives `color-loc(Green, H3)`.

<dot>

### Step 16 — `domain-elimination`

> every House except H1 excluded for Blue → (color-loc Blue H1).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)`, `is-a(H1, House)`

Derives `color-loc(Blue, H1)`.

<dot>

### Step 17 — `functional-negative`

> color-loc functional: (color-loc Blue H1) ⟹ (not (color-loc Blue H2)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(H2, House)`

Derives `not(color-loc(Blue, H2))`.

<dot>

### Step 18 — `functional-negative`

> color-loc functional: (color-loc Blue H1) ⟹ (not (color-loc Blue H3)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(H3, House)`

Derives `not(color-loc(Blue, H3))`.

<dot>

### Step 19 — `injective-negative`

> color-loc injective: (color-loc Blue H1) ⟹ (not (color-loc Red H1)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)`

Derives `not(color-loc(Red, H1))`.

<dot>

### Step 20 — `injective-negative`

> color-loc injective: (color-loc Blue H1) ⟹ (not (color-loc Green H1)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)`

Derives `not(color-loc(Green, H1))`.

<dot>

### Step 21 — `range-elimination`

> every Color except Blue excluded for H1 → (color-loc Blue H1).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H1, House)`, `is-a(Blue, Color)`

Derives `color-loc(Blue, H1)`.

<dot>

### Step 22 — `range-elimination`

> every Color except Red excluded for H2 → (color-loc Red H2).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H2, House)`, `is-a(Red, Color)`

Derives `color-loc(Red, H2)`.

<dot>

### Step 23 — `range-elimination`

> every Color except Green excluded for H3 → (color-loc Green H3).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H3, House)`, `is-a(Green, Color)`

Derives `color-loc(Green, H3)`.

<dot>

## State 1 of 1 — ∅, root itself · minimal and maximal — 0 steps

_Nothing beyond root and the commitment._
