# Solution trace

> No solution — 12 commitments refuted (12 dead).

## Before any assumption — 6 steps

## Step 1 — `type-exclusivity`

Premises: `is-a(Dog, Pet)`, `is-a(Cat, Pet)`

Derives `not(co-located(Dog, Cat))`.

<dot>

## Step 2 — `type-exclusivity`

Premises: `is-a(Dog, Pet)`, `is-a(Bird, Pet)`

Derives `not(co-located(Dog, Bird))`.

<dot>

## Step 3 — `type-exclusivity`

Premises: `is-a(Cat, Pet)`, `is-a(Dog, Pet)`

Derives `not(co-located(Cat, Dog))`.

<dot>

## Step 4 — `type-exclusivity`

Premises: `is-a(Cat, Pet)`, `is-a(Bird, Pet)`

Derives `not(co-located(Cat, Bird))`.

<dot>

## Step 5 — `type-exclusivity`

Premises: `is-a(Bird, Pet)`, `is-a(Dog, Pet)`

Derives `not(co-located(Bird, Dog))`.

<dot>

## Step 6 — `type-exclusivity`

Premises: `is-a(Bird, Pet)`, `is-a(Cat, Pet)`

Derives `not(co-located(Bird, Cat))`.

<dot>

_(no surviving derivation — see the refuted branches below.)_

## Refuted hypotheses

<details>
<summary>Assumed {co-located(Bird, Pet), is-a(Bird, Cat), is-a(Pet, Cat)} — refuted (dead-post)</summary>

Assumed **{co-located(Bird, Pet), is-a(Bird, Cat), is-a(Pet, Cat)}**; the branch derives ⊥.

Lifted no-good: `co-located(Bird, Pet), is-a(Bird, Cat), is-a(Pet, Cat)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Bird, Pet), is-a(Bird, Dog), is-a(Pet, Dog)} — refuted (dead-post)</summary>

Assumed **{co-located(Bird, Pet), is-a(Bird, Dog), is-a(Pet, Dog)}**; the branch derives ⊥.

Lifted no-good: `co-located(Bird, Pet), is-a(Bird, Dog), is-a(Pet, Dog)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Cat, Pet), is-a(Cat, Bird), is-a(Pet, Bird)} — refuted (dead-post)</summary>

Assumed **{co-located(Cat, Pet), is-a(Cat, Bird), is-a(Pet, Bird)}**; the branch derives ⊥.

Lifted no-good: `co-located(Cat, Pet), is-a(Cat, Bird), is-a(Pet, Bird)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Cat, Pet), is-a(Cat, Dog), is-a(Pet, Dog)} — refuted (dead-post)</summary>

Assumed **{co-located(Cat, Pet), is-a(Cat, Dog), is-a(Pet, Dog)}**; the branch derives ⊥.

Lifted no-good: `co-located(Cat, Pet), is-a(Cat, Dog), is-a(Pet, Dog)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Dog, Pet), is-a(Dog, Bird), is-a(Pet, Bird)} — refuted (dead-post)</summary>

Assumed **{co-located(Dog, Pet), is-a(Dog, Bird), is-a(Pet, Bird)}**; the branch derives ⊥.

Lifted no-good: `co-located(Dog, Pet), is-a(Dog, Bird), is-a(Pet, Bird)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Dog, Pet), is-a(Dog, Cat), is-a(Pet, Cat)} — refuted (dead-post)</summary>

Assumed **{co-located(Dog, Pet), is-a(Dog, Cat), is-a(Pet, Cat)}**; the branch derives ⊥.

Lifted no-good: `co-located(Dog, Pet), is-a(Dog, Cat), is-a(Pet, Cat)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Bird), is-a(Bird, Cat), is-a(Pet, Cat)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Bird), is-a(Bird, Cat), is-a(Pet, Cat)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Bird), is-a(Bird, Cat), is-a(Pet, Cat)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Bird), is-a(Bird, Dog), is-a(Pet, Dog)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Bird), is-a(Bird, Dog), is-a(Pet, Dog)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Bird), is-a(Bird, Dog), is-a(Pet, Dog)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Cat), is-a(Cat, Bird), is-a(Pet, Bird)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Cat), is-a(Cat, Bird), is-a(Pet, Bird)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Cat), is-a(Cat, Bird), is-a(Pet, Bird)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Cat), is-a(Cat, Dog), is-a(Pet, Dog)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Cat), is-a(Cat, Dog), is-a(Pet, Dog)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Cat), is-a(Cat, Dog), is-a(Pet, Dog)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Dog), is-a(Dog, Bird), is-a(Pet, Bird)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Dog), is-a(Dog, Bird), is-a(Pet, Bird)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Dog), is-a(Dog, Bird), is-a(Pet, Bird)`.

<dot>

</details>

<details>
<summary>Assumed {co-located(Pet, Dog), is-a(Dog, Cat), is-a(Pet, Cat)} — refuted (dead-post)</summary>

Assumed **{co-located(Pet, Dog), is-a(Dog, Cat), is-a(Pet, Cat)}**; the branch derives ⊥.

Lifted no-good: `co-located(Pet, Dog), is-a(Dog, Cat), is-a(Pet, Cat)`.

<dot>

</details>

## Commitment lattice

<dot>
