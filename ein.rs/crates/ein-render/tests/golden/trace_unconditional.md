# Solution trace

> Solved in 0 steps after 23 unconditional; commitment ∅ (unconditional); 1 solution(s), 0 refuted.

## Before any assumption — 23 steps

## Step 1 — `bijective-properties`

> color-loc is bijective ⟹ functional ∧ injective ∧ total ∧ surjective.

Premises: `bijective(color-loc)`

Derives `functional(color-loc)`.

<dot>

## Step 2 — `derive-functional-negative`

> color-loc functional ⟹ functional-negative active.

Premises: `functional(color-loc)`

Derives `functional-negative(color-loc)`.

<dot>

## Step 3 — `derive-domain-elimination`

> color-loc functional + total ⟹ domain-elimination active.

Premises: `functional(color-loc)`, `total(color-loc)`

Derives `domain-elimination(color-loc)`.

<dot>

## Step 4 — `derive-injective-negative`

> color-loc injective ⟹ injective-negative active.

Premises: `injective(color-loc)`

Derives `injective-negative(color-loc)`.

<dot>

## Step 5 — `derive-range-elimination`

> color-loc injective + surjective ⟹ range-elimination active.

Premises: `injective(color-loc)`, `surjective(color-loc)`

Derives `range-elimination(color-loc)`.

<dot>

## Step 6 — `functional-negative`

> color-loc functional: (color-loc Red H2) ⟹ (not (color-loc Red H1)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(H1, House)` — from (1) Red is in house 2

Derives `not(color-loc(Red, H1))`.

<dot>

## Step 7 — `functional-negative`

> color-loc functional: (color-loc Red H2) ⟹ (not (color-loc Red H3)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(H3, House)` — from (1) Red is in house 2

Derives `not(color-loc(Red, H3))`.

<dot>

## Step 8 — `functional-negative`

> color-loc functional: (color-loc Green H3) ⟹ (not (color-loc Green H1)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(H1, House)` — from (2) Green is in house 3

Derives `not(color-loc(Green, H1))`.

<dot>

## Step 9 — `functional-negative`

> color-loc functional: (color-loc Green H3) ⟹ (not (color-loc Green H2)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(H2, House)` — from (2) Green is in house 3

Derives `not(color-loc(Green, H2))`.

<dot>

## Step 10 — `injective-negative`

> color-loc injective: (color-loc Red H2) ⟹ (not (color-loc Green H2)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)` — from (1) Red is in house 2

Derives `not(color-loc(Green, H2))`.

<dot>

## Step 11 — `injective-negative`

> color-loc injective: (color-loc Red H2) ⟹ (not (color-loc Blue H2)).

Premises: `color-loc(Red, H2)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)` — from (1) Red is in house 2

Derives `not(color-loc(Blue, H2))`.

<dot>

## Step 12 — `injective-negative`

> color-loc injective: (color-loc Green H3) ⟹ (not (color-loc Red H3)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)` — from (2) Green is in house 3

Derives `not(color-loc(Red, H3))`.

<dot>

## Step 13 — `injective-negative`

> color-loc injective: (color-loc Green H3) ⟹ (not (color-loc Blue H3)).

Premises: `color-loc(Green, H3)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)` — from (2) Green is in house 3

Derives `not(color-loc(Blue, H3))`.

<dot>

## Step 14 — `domain-elimination`

> every House except H2 excluded for Red → (color-loc Red H2).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)`, `is-a(H2, House)`

Derives `color-loc(Red, H2)`.

<dot>

## Step 15 — `domain-elimination`

> every House except H3 excluded for Green → (color-loc Green H3).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)`, `is-a(H3, House)`

Derives `color-loc(Green, H3)`.

<dot>

## Step 16 — `domain-elimination`

> every House except H1 excluded for Blue → (color-loc Blue H1).

Premises: `functional(color-loc)`, `total(color-loc)`, `relation(color-loc, Color, House)`, `is-a(Blue, Color)`, `is-a(H1, House)`

Derives `color-loc(Blue, H1)`.

<dot>

## Step 17 — `functional-negative`

> color-loc functional: (color-loc Blue H1) ⟹ (not (color-loc Blue H2)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(H2, House)`

Derives `not(color-loc(Blue, H2))`.

<dot>

## Step 18 — `functional-negative`

> color-loc functional: (color-loc Blue H1) ⟹ (not (color-loc Blue H3)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(H3, House)`

Derives `not(color-loc(Blue, H3))`.

<dot>

## Step 19 — `injective-negative`

> color-loc injective: (color-loc Blue H1) ⟹ (not (color-loc Red H1)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(Red, Color)`

Derives `not(color-loc(Red, H1))`.

<dot>

## Step 20 — `injective-negative`

> color-loc injective: (color-loc Blue H1) ⟹ (not (color-loc Green H1)).

Premises: `color-loc(Blue, H1)`, `relation(color-loc, Color, House)`, `is-a(Green, Color)`

Derives `not(color-loc(Green, H1))`.

<dot>

## Step 21 — `range-elimination`

> every Color except Blue excluded for H1 → (color-loc Blue H1).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H1, House)`, `is-a(Blue, Color)`

Derives `color-loc(Blue, H1)`.

<dot>

## Step 22 — `range-elimination`

> every Color except Red excluded for H2 → (color-loc Red H2).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H2, House)`, `is-a(Red, Color)`

Derives `color-loc(Red, H2)`.

<dot>

## Step 23 — `range-elimination`

> every Color except Green excluded for H3 → (color-loc Green H3).

Premises: `injective(color-loc)`, `surjective(color-loc)`, `relation(color-loc, Color, House)`, `is-a(H3, House)`, `is-a(Green, Color)`

Derives `color-loc(Green, H3)`.

<dot>

_(no surviving derivation — see the refuted branches below.)_

## Commitment lattice

<dot>

## Solution

<dot>
