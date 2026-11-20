# Region trace

```text
  asked             all of R — describe
  states (k_s)      3
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      2 states
  components        1
  sub-cubes         2, each of dimension 2
  merged            2 states reached by more than one commitment, with 4 minimal
  N                 51 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          51 candidates, 47 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L8 what a state denies is dead above it
  work              52 saturations · 344 facts beyond root · 3 layers
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

## State 1 of 3 — ∅, root itself · minimal — 0 steps

_Nothing beyond root and the commitment._

## State 2 of 3 — 2 candidates · maximal · 3 commitments, 2 minimal — 10 steps

Assuming **(color-of H2 Blue)**.

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

### Step 5 — `domain-elimination`

> every Color except Blue excluded for H2 → (color-of H2 Blue)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H2, House)`, `is-a(Blue, Color)`

Derives `color-of(H2, Blue)`.

<dot>

### Step 6 — `domain-elimination`

> every Color except Green excluded for H3 → (color-of H3 Green)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H3, House)`, `is-a(Green, Color)`

Derives `color-of(H3, Green)`.

<dot>

### Step 7 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H3, Red))`.

<dot>

### Step 8 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Blue)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Blue, Color)`

Derives `not(color-of(H3, Blue))`.

<dot>

### Step 9 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Green))`.

<dot>

### Step 10 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H2 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H2, House)`

Derives `not(color-of(H2, Green))`.

<dot>

## State 3 of 3 — 2 candidates · maximal · 3 commitments, 2 minimal — 10 steps

Assuming **(color-of H2 Green)**.

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

### Step 5 — `domain-elimination`

> every Color except Green excluded for H2 → (color-of H2 Green)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H2, House)`, `is-a(Green, Color)`

Derives `color-of(H2, Green)`.

<dot>

### Step 6 — `domain-elimination`

> every Color except Blue excluded for H3 → (color-of H3 Blue)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H3, House)`, `is-a(Blue, Color)`

Derives `color-of(H3, Blue)`.

<dot>

### Step 7 — `functional-negative`

> color-of functional: (color-of H3 Blue) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Red, Color)`

Derives `not(color-of(H3, Red))`.

<dot>

### Step 8 — `functional-negative`

> color-of functional: (color-of H3 Blue) ⟹ (not (color-of H3 Green)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(Green, Color)`

Derives `not(color-of(H3, Green))`.

<dot>

### Step 9 — `injective-negative`

> color-of injective: (color-of H3 Blue) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H1, House)`

Derives `not(color-of(H1, Blue))`.

<dot>

### Step 10 — `injective-negative`

> color-of injective: (color-of H3 Blue) ⟹ (not (color-of H2 Blue)).

Premises: `color-of(H3, Blue)`, `relation(color-of, House, Color)` — the colour of House is Color, `is-a(H2, House)`

Derives `not(color-of(H2, Blue))`.

<dot>

## N — 51 minimal dead sets

<details>
<summary>{(color-of 0 1)} — dead</summary>

Core: `(color-of 0 1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 Blue)} — dead</summary>

Core: `(color-of 0 Blue)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 Green)} — dead</summary>

Core: `(color-of 0 Green)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 H1)} — dead</summary>

Core: `(color-of 0 H1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 H2)} — dead</summary>

Core: `(color-of 0 H2)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 H3)} — dead</summary>

Core: `(color-of 0 H3)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 0 Red)} — dead</summary>

Core: `(color-of 0 Red)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 0)} — dead</summary>

Core: `(color-of 1 0)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 Blue)} — dead</summary>

Core: `(color-of 1 Blue)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 Green)} — dead</summary>

Core: `(color-of 1 Green)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 H1)} — dead</summary>

Core: `(color-of 1 H1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 H2)} — dead</summary>

Core: `(color-of 1 H2)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 H3)} — dead</summary>

Core: `(color-of 1 H3)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of 1 Red)} — dead</summary>

Core: `(color-of 1 Red)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue 0)} — dead</summary>

Core: `(color-of Blue 0)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue 1)} — dead</summary>

Core: `(color-of Blue 1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue Green)} — dead</summary>

Core: `(color-of Blue Green)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue H1)} — dead</summary>

Core: `(color-of Blue H1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue H2)} — dead</summary>

Core: `(color-of Blue H2)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue H3)} — dead</summary>

Core: `(color-of Blue H3)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Blue Red)} — dead</summary>

Core: `(color-of Blue Red)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green 0)} — dead</summary>

Core: `(color-of Green 0)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green 1)} — dead</summary>

Core: `(color-of Green 1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green Blue)} — dead</summary>

Core: `(color-of Green Blue)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green H1)} — dead</summary>

Core: `(color-of Green H1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green H2)} — dead</summary>

Core: `(color-of Green H2)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green H3)} — dead</summary>

Core: `(color-of Green H3)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Green Red)} — dead</summary>

Core: `(color-of Green Red)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of H1 0)} — dead</summary>

Core: `(color-of H1 0)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H1 1)} — dead</summary>

Core: `(color-of H1 1)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H1 H2)} — dead</summary>

Core: `(color-of H1 H2)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H1 H3)} — dead</summary>

Core: `(color-of H1 H3)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H2 0)} — dead</summary>

Core: `(color-of H2 0)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H2 1)} — dead</summary>

Core: `(color-of H2 1)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H2 H1)} — dead</summary>

Core: `(color-of H2 H1)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H2 H3)} — dead</summary>

Core: `(color-of H2 H3)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H3 0)} — dead</summary>

Core: `(color-of H3 0)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H3 1)} — dead</summary>

Core: `(color-of H3 1)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H3 H1)} — dead</summary>

Core: `(color-of H3 H1)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of H3 H2)} — dead</summary>

Core: `(color-of H3 H2)`

Refuted by `typecheck-arg-1`.

</details>

<details>
<summary>{(color-of Red 0)} — dead</summary>

Core: `(color-of Red 0)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red 1)} — dead</summary>

Core: `(color-of Red 1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red Blue)} — dead</summary>

Core: `(color-of Red Blue)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red Green)} — dead</summary>

Core: `(color-of Red Green)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red H1)} — dead</summary>

Core: `(color-of Red H1)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red H2)} — dead</summary>

Core: `(color-of Red H2)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of Red H3)} — dead</summary>

Core: `(color-of Red H3)`

Refuted by `typecheck-arg-0`.

</details>

<details>
<summary>{(color-of H2 Blue), (color-of H2 Green)} — dead</summary>

Core: `(color-of H2 Blue)`, `(color-of H2 Green)`

Refuted by `functional`.

</details>

<details>
<summary>{(color-of H2 Blue), (color-of H3 Blue)} — dead</summary>

Core: `(color-of H2 Blue)`, `(color-of H3 Blue)`

Refuted by `injective`.

</details>

<details>
<summary>{(color-of H2 Green), (color-of H3 Green)} — dead</summary>

Core: `(color-of H2 Green)`, `(color-of H3 Green)`

Refuted by `injective`.

</details>

<details>
<summary>{(color-of H3 Blue), (color-of H3 Green)} — dead</summary>

Core: `(color-of H3 Blue)`, `(color-of H3 Green)`

Refuted by `functional`.

</details>
