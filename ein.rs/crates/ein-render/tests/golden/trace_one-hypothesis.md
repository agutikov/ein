# Region trace

```text
  asked             the maximal line — every state on it
  maximal line      2 states — all of them
  exhausted         yes — this is all of the maximal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          51 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L8 what a state denies is dead above it
  work              80 saturations · 552 facts beyond root
```

## Root — 11 steps

### Step 1 — `std.contradiction.clash-of-0`

> color-of is declared.

Premises: `relation(color-of)`

Derives `std.contradiction.clash-0(color-of)`.

<dot>

### Step 2 — `std.contradiction.clash-of-0`

> is-a is declared.

Premises: `relation(is-a)`

Derives `std.contradiction.clash-0(is-a)`.

<dot>

### Step 3 — `std.contradiction.clash-of-2`

> color-of is declared with two arguments.

Premises: `relation(color-of, House, Color)` — the colour of House is Color

Derives `std.contradiction.clash-2(color-of)`.

<dot>

### Step 4 — `std.contradiction.clash-of-2`

> is-a is declared with two arguments.

Premises: `relation(is-a, T, T)` — T is a T

Derives `std.contradiction.clash-2(is-a)`.

<dot>

### Step 5 — `derive-functional-negative`

> color-of functional ⟹ functional-negative active.

Premises: `functional(color-of)`

Derives `functional-negative(color-of)`.

<dot>

### Step 6 — `functional-negative`

> color-of functional: (color-of H1 Red) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Green, Color)` — from (1) H1 is Red

Derives `not(color-of(H1, Green))`.

<dot>

### Step 7 — `functional-negative`

> color-of functional: (color-of H1 Red) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Blue, Color)` — from (1) H1 is Red

Derives `not(color-of(H1, Blue))`.

<dot>

### Step 8 — `derive-injective-negative`

> color-of injective ⟹ injective-negative active.

Premises: `injective(color-of)`

Derives `injective-negative(color-of)`.

<dot>

### Step 9 — `injective-negative`

> color-of injective: (color-of H1 Red) ⟹ (not (color-of H2 Red)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H2, House)` — from (1) H1 is Red

Derives `not(color-of(H2, Red))`.

<dot>

### Step 10 — `injective-negative`

> color-of injective: (color-of H1 Red) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H3, House)` — from (1) H1 is Red

Derives `not(color-of(H3, Red))`.

<dot>

### Step 11 — `domain-elimination`

> every Color except Red excluded for H1 → (color-of H1 Red)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H1, House)`, `is-a(Red, Color)`

Derives `color-of(H1, Red)`.

<dot>

## State 1 of 2 — 2 candidates · maximal · from {(color-of H2 Blue), (color-of H3 Green)} — 10 steps

Assuming **(color-of H2 Blue), (color-of H3 Green)**.

### Step 1 — `functional-negative`

> color-of functional: (color-of H2 Blue) ⟹ (not (color-of H2 Red)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H2, Red))`.

<dot>

### Step 2 — `functional-negative`

> color-of functional: (color-of H2 Blue) ⟹ (not (color-of H2 Green)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Green, Color)`

Derives `not(color-of(H2, Green))`.

<dot>

### Step 3 — `injective-negative`

> color-of injective: (color-of H2 Blue) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Blue))`.

<dot>

### Step 4 — `injective-negative`

> color-of injective: (color-of H2 Blue) ⟹ (not (color-of H3 Blue)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H3, House)`

Derives `not(color-of(H3, Blue))`.

<dot>

### Step 5 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H3, Red))`.

<dot>

### Step 6 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Blue)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Blue, Color)`

Derives `not(color-of(H3, Blue))`.

<dot>

### Step 7 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Green))`.

<dot>

### Step 8 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H2 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H2, House)`

Derives `not(color-of(H2, Green))`.

<dot>

### Step 9 — `domain-elimination`

> every Color except Blue excluded for H2 → (color-of H2 Blue)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H2, House)`, `is-a(Blue, Color)`

Derives `color-of(H2, Blue)`.

<dot>

### Step 10 — `domain-elimination`

> every Color except Green excluded for H3 → (color-of H3 Green)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H3, House)`, `is-a(Green, Color)`

Derives `color-of(H3, Green)`.

<dot>

## State 2 of 2 — 2 candidates · maximal · from {(color-of H2 Green), (color-of H3 Blue)} — 10 steps

Assuming **(color-of H2 Green), (color-of H3 Blue)**.

### Step 1 — `functional-negative`

> color-of functional: (color-of H2 Green) ⟹ (not (color-of H2 Red)).

Premises: `color-of(H2, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H2, Red))`.

<dot>

### Step 2 — `functional-negative`

> color-of functional: (color-of H2 Green) ⟹ (not (color-of H2 Blue)).

Premises: `color-of(H2, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Blue, Color)`

Derives `not(color-of(H2, Blue))`.

<dot>

### Step 3 — `injective-negative`

> color-of injective: (color-of H2 Green) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H2, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Green))`.

<dot>

### Step 4 — `injective-negative`

> color-of injective: (color-of H2 Green) ⟹ (not (color-of H3 Green)).

Premises: `color-of(H2, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H3, House)`

Derives `not(color-of(H3, Green))`.

<dot>

### Step 5 — `functional-negative`

> color-of functional: (color-of H3 Blue) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H3, Red))`.

<dot>

### Step 6 — `functional-negative`

> color-of functional: (color-of H3 Blue) ⟹ (not (color-of H3 Green)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Green, Color)`

Derives `not(color-of(H3, Green))`.

<dot>

### Step 7 — `injective-negative`

> color-of injective: (color-of H3 Blue) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Blue))`.

<dot>

### Step 8 — `injective-negative`

> color-of injective: (color-of H3 Blue) ⟹ (not (color-of H2 Blue)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H2, House)`

Derives `not(color-of(H2, Blue))`.

<dot>

### Step 9 — `domain-elimination`

> every Color except Green excluded for H2 → (color-of H2 Green)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H2, House)`, `is-a(Green, Color)`

Derives `color-of(H2, Green)`.

<dot>

### Step 10 — `domain-elimination`

> every Color except Blue excluded for H3 → (color-of H3 Blue)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H3, House)`, `is-a(Blue, Color)`

Derives `color-of(H3, Blue)`.

<dot>
