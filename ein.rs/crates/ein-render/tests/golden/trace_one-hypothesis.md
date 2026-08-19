# Solution trace

> Solved in 10 steps after 7 unconditional; commitment {color-of(H2, Blue)}; 1 solution(s), 0 refuted.

## Before any assumption — 7 steps

## Step 1 — `derive-functional-negative`

> color-of functional ⟹ functional-negative active.

Premises: `functional(color-of)`

Derives `functional-negative(color-of)`.

<dot>

## Step 2 — `derive-injective-negative`

> color-of injective ⟹ injective-negative active.

Premises: `injective(color-of)`

Derives `injective-negative(color-of)`.

<dot>

## Step 3 — `functional-negative`

> color-of functional: (color-of H1 Red) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)`, `is-a(Green, Color)` — from (1) H1 is Red

Derives `not(color-of(H1, Green))`.

<dot>

## Step 4 — `functional-negative`

> color-of functional: (color-of H1 Red) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)`, `is-a(Blue, Color)` — from (1) H1 is Red

Derives `not(color-of(H1, Blue))`.

<dot>

## Step 5 — `injective-negative`

> color-of injective: (color-of H1 Red) ⟹ (not (color-of H2 Red)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)`, `is-a(H2, House)` — from (1) H1 is Red

Derives `not(color-of(H2, Red))`.

<dot>

## Step 6 — `injective-negative`

> color-of injective: (color-of H1 Red) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H1, Red)`, `relation(color-of, House, Color)`, `is-a(H3, House)` — from (1) H1 is Red

Derives `not(color-of(H3, Red))`.

<dot>

## Step 7 — `domain-elimination`

> every Color except Red excluded for H1 → (color-of H1 Red)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H1, House)`, `is-a(Red, Color)`

Derives `color-of(H1, Red)`.

<dot>

Assuming **{color-of(H2, Blue)}**.

## Step 8 — `functional-negative`

> color-of functional: (color-of H2 Blue) ⟹ (not (color-of H2 Red)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)`, `is-a(Red, Color)`

Derives `not(color-of(H2, Red))`.

<dot>

## Step 9 — `functional-negative`

> color-of functional: (color-of H2 Blue) ⟹ (not (color-of H2 Green)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)`, `is-a(Green, Color)`

Derives `not(color-of(H2, Green))`.

<dot>

## Step 10 — `injective-negative`

> color-of injective: (color-of H2 Blue) ⟹ (not (color-of H1 Blue)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)`, `is-a(H1, House)`

Derives `not(color-of(H1, Blue))`.

<dot>

## Step 11 — `injective-negative`

> color-of injective: (color-of H2 Blue) ⟹ (not (color-of H3 Blue)).

Premises: `color-of(H2, Blue)`, `relation(color-of, House, Color)`, `is-a(H3, House)`

Derives `not(color-of(H3, Blue))`.

<dot>

## Step 12 — `domain-elimination`

> every Color except Blue excluded for H2 → (color-of H2 Blue)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H2, House)`, `is-a(Blue, Color)`

Derives `color-of(H2, Blue)`.

<dot>

## Step 13 — `domain-elimination`

> every Color except Green excluded for H3 → (color-of H3 Green)

Premises: `functional(color-of, 0, 1)`, `total(color-of, 0)`, `is-a(H3, House)`, `is-a(Green, Color)`

Derives `color-of(H3, Green)`.

<dot>

## Step 14 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Red)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)`, `is-a(Red, Color)`

Derives `not(color-of(H3, Red))`.

<dot>

## Step 15 — `functional-negative`

> color-of functional: (color-of H3 Green) ⟹ (not (color-of H3 Blue)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)`, `is-a(Blue, Color)`

Derives `not(color-of(H3, Blue))`.

<dot>

## Step 16 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H1 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)`, `is-a(H1, House)`

Derives `not(color-of(H1, Green))`.

<dot>

## Step 17 — `injective-negative`

> color-of injective: (color-of H3 Green) ⟹ (not (color-of H2 Green)).

Premises: `color-of(H3, Green)`, `relation(color-of, House, Color)`, `is-a(H2, House)`

Derives `not(color-of(H2, Green))`.

<dot>

## Commitment lattice

<dot>

## Solution

<dot>
