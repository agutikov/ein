# Solution trace

> Solved in 4 steps after 16 unconditional; commitment {co-located(Blue, H2), co-located(Green, H3)}; 1 solution(s), 2 refuted.

## Before any assumption — 16 steps

## Step 1 — `symmetric`

> co-located sym

Premises: `co-located(Red, H1)` — from (1)

Derives `co-located(H1, Red)`.

<dot>

## Step 2 — `symmetric`

> co-located sym

Premises: `co-located(H1, Red)`

Derives `co-located(Red, H1)`.

<dot>

## Step 3 — `sibling-exclusive`

> sib T

Premises: `is-a(Color, T)`, `is-a(House, T)`

Derives `not(co-located(Color, House))`.

<dot>

## Step 4 — `sibling-exclusive`

> sib T

Premises: `is-a(House, T)`, `is-a(Color, T)`

Derives `not(co-located(House, Color))`.

<dot>

## Step 5 — `sibling-exclusive`

> sib Color

Premises: `is-a(Red, Color)`, `is-a(Blue, Color)`

Derives `not(co-located(Red, Blue))`.

<dot>

## Step 6 — `sibling-exclusive`

> sib Color

Premises: `is-a(Red, Color)`, `is-a(Green, Color)`

Derives `not(co-located(Red, Green))`.

<dot>

## Step 7 — `sibling-exclusive`

> sib Color

Premises: `is-a(Blue, Color)`, `is-a(Red, Color)`

Derives `not(co-located(Blue, Red))`.

<dot>

## Step 8 — `sibling-exclusive`

> sib Color

Premises: `is-a(Blue, Color)`, `is-a(Green, Color)`

Derives `not(co-located(Blue, Green))`.

<dot>

## Step 9 — `sibling-exclusive`

> sib Color

Premises: `is-a(Green, Color)`, `is-a(Red, Color)`

Derives `not(co-located(Green, Red))`.

<dot>

## Step 10 — `sibling-exclusive`

> sib Color

Premises: `is-a(Green, Color)`, `is-a(Blue, Color)`

Derives `not(co-located(Green, Blue))`.

<dot>

## Step 11 — `sibling-exclusive`

> sib House

Premises: `is-a(H1, House)`, `is-a(H2, House)`

Derives `not(co-located(H1, H2))`.

<dot>

## Step 12 — `sibling-exclusive`

> sib House

Premises: `is-a(H1, House)`, `is-a(H3, House)`

Derives `not(co-located(H1, H3))`.

<dot>

## Step 13 — `sibling-exclusive`

> sib House

Premises: `is-a(H2, House)`, `is-a(H1, House)`

Derives `not(co-located(H2, H1))`.

<dot>

## Step 14 — `sibling-exclusive`

> sib House

Premises: `is-a(H2, House)`, `is-a(H3, House)`

Derives `not(co-located(H2, H3))`.

<dot>

## Step 15 — `sibling-exclusive`

> sib House

Premises: `is-a(H3, House)`, `is-a(H1, House)`

Derives `not(co-located(H3, H1))`.

<dot>

## Step 16 — `sibling-exclusive`

> sib House

Premises: `is-a(H3, House)`, `is-a(H2, House)`

Derives `not(co-located(H3, H2))`.

<dot>

Assuming **{co-located(Blue, H2), co-located(Green, H3)}**.

## Step 17 — `symmetric`

> co-located sym

Premises: `co-located(Blue, H2)`

Derives `co-located(H2, Blue)`.

<dot>

## Step 18 — `symmetric`

> co-located sym

Premises: `co-located(Green, H3)`

Derives `co-located(H3, Green)`.

<dot>

## Step 19 — `symmetric`

> co-located sym

Premises: `co-located(H2, Blue)`

Derives `co-located(Blue, H2)`.

<dot>

## Step 20 — `symmetric`

> co-located sym

Premises: `co-located(H3, Green)`

Derives `co-located(Green, H3)`.

<dot>

## Refuted hypotheses

<details>
<summary>Assumed {co-located(Blue, H2), co-located(Blue, H3)} — refuted (dead-post)</summary>

Assumed **{co-located(Blue, H2), co-located(Blue, H3)}**; the branch derives ⊥.

Lifted no-good: `co-located(Blue, H2), co-located(Blue, H3)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Blue, H2), co-located(Green, H2)} — refuted (dead-post)</summary>

Assumed **{co-located(Blue, H2), co-located(Green, H2)}**; the branch derives ⊥.

Lifted no-good: `co-located(Blue, H2), co-located(Green, H2)`.

<dot>

</details>

## Commitment lattice

<dot>

## Solution

<dot>
