# Region trace

```text
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: the program states :no-search, so R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers
```

## Root — 4 steps

### Step 1 — `std.contradiction.clash-of-0`

> is-a is declared.

Premises: `relation(is-a)`

Derives `std.contradiction.clash-0(is-a)`.

<dot>

### Step 2 — `std.contradiction.clash-of-0`

> likes is declared.

Premises: `relation(likes)`

Derives `std.contradiction.clash-0(likes)`.

<dot>

### Step 3 — `std.contradiction.clash-of-2`

> is-a is declared with two arguments.

Premises: `relation(is-a, T, T)` — T is a T

Derives `std.contradiction.clash-2(is-a)`.

<dot>

### Step 4 — `std.contradiction.clash-of-2`

> likes is declared with two arguments.

Premises: `relation(likes, Person, Food)` — Person likes Food

Derives `std.contradiction.clash-2(likes)`.

<dot>

## Owing — 1 node with no alive node one fact up

- **∅** owes
  - likes owes Ann a Food.
