# S1a.6.9 — the trace, before and after

**Task:** [T1a.6.9.3](s1a.6.9_fork_entry_delta.md) — *answer
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
with a diff, not an argument.* The question asks whether a fork may stop
re-narrating the root's fixpoint, and the plan says to decide it **against a
rendered human trace** rather than against a line count. This file is that
trace.

Produced by one binary, twice:

```sh
cd ein.rs && cargo build --release --features fork-delta --target-dir target-fd
EIN_FORK_DELTA=0 \
ein.rs/target-fd/release/ein solve examples/zebra2.ein --trace before.md
ein.rs/target-fd/release/ein solve examples/zebra2.ein --trace after.md
```

The solution is the same, its commitment is the same, and every fact in it
has the same primary justification. What changes is how the **step list** is
arranged — and, once the renderer caught up, *only* how it is arranged.

| | before | after |
|---|---:|---:|
| steps under the hypothesis | 561 | **240** |
| steps before it, in their own section | 0 | **321** |
| distinct rules in the proof | 24 | **24** |

The middle row is [T1a.6.9.4](s1a.6.9_fork_entry_delta.md)'s renderer change
and it is not cosmetic: without it the "after" column reads 240 / 0 / **12**,
and the twelve missing rules include `symmetric`, which closes `next-to` at
root and nowhere else.

## The first six steps

The header is identical in both — same verdict, same commitment, same
"1 solution(s), 2 refuted".

> Assuming **{drink-loc(Juice, House-4)}**.

### before — the fresh saturator

```
## Step 1 — `symmetric`
> next-to is symmetric: House-2 ↔ House-1.
Premises: next-to(House-2, House-1)  →  next-to(House-1, House-2)

## Step 2 — `symmetric`
> next-to is symmetric: House-3 ↔ House-2.
Premises: next-to(House-3, House-2)  →  next-to(House-2, House-3)

## Step 3 — `symmetric`
> next-to is symmetric: House-4 ↔ House-3.
## Step 4 — `symmetric`
> next-to is symmetric: House-5 ↔ House-4.
## Step 5 — `symmetric`
> next-to is symmetric: House-1 ↔ House-2.
## Step 6 — `symmetric`
> next-to is symmetric: House-2 ↔ House-3.
```

Steps 1–8 close `next-to` under symmetry. Steps 9–~40 close `is-a*` under
`includes`. Not one of them has anything to do with the hypothesis: they are
the ontology's own closure, true before the assumption was made and already
derived at root — re-derived here so the fork's saturator can discover that
each conclusion is already present. Note also that this is not even root's own
derivation order: a fork rediscovers the closure plan by plan, which is why
`symmetric` comes first here and `includes` comes first in the root section
below.

### after — the resumed saturator, with the root section

The trace now opens with **`## Before any assumption — 321 steps`** — the
ontology's own closure, told once, in the order root actually derived it
rather than in the order a fork happened to rediscover it:

```
## Before any assumption — 321 steps

## Step 1 — `includes`
> is-a* includes is-a: from Attribute →is-a→ T, derive Attribute →is-a*→ T.
Premises: is-a(Attribute, T)  →  is-a*(Attribute, T)

## Step 2 — `includes`
> is-a* includes is-a: from House →is-a→ Attribute, derive House →is-a*→ Attribute.
…
## Step 321 — `range-elimination`
```

and then, 321 steps later:

```
Assuming **{drink-loc(Juice, House-4)}**.

## Step 322 — `co-located-negative`
> co-located: (not (color-loc Red House-4)) ⟹ (not (nation-loc Englishman House-4)).
Premises: not(color-loc(Red, House-4))  →  not(nation-loc(Englishman, House-4))

## Step 323 — `co-located`
> co-located: (drink-loc Juice House-4) ⟹ (smoke-loc Lucky_Strike House-4).
Premises: drink-loc(Juice, House-4)  →  smoke-loc(Lucky_Strike, House-4)

## Step 324 — `co-located-negative`
> co-located: (not (drink-loc Juice House-2)) ⟹ (not (smoke-loc Lucky_Strike House-2)).

## Step 325 — `co-located-negative`
> co-located: (not (nation-loc Englishman House-4)) ⟹ (not (color-loc Red House-4)).
## Step 326 — `co-located`
> co-located: (smoke-loc Lucky_Strike House-4) ⟹ (drink-loc Juice House-4).
## Step 327 — `co-located-negative`
> co-located: (not (smoke-loc Lucky_Strike House-2)) ⟹ (not (smoke-loc …)).
```

**Step 323 is the hypothesis's own first consequence** — *Juice is drunk in
House-4, therefore Lucky Strike is smoked there* — which is where
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
starts and what
[`08-human-style-deductive-trace`](../../ideas/08-human-style-deductive-trace.md)
asks a hypothesis's proof to show.

## What the "after" trace still narrates that a human would not

Being honest about the 240 that remain, because they are the argument for
*not* stopping here:

- **Steps 325–327 are the closure ping-ponging.** `co-located` is symmetric in
  its own rule set, so having derived `A ⟹ B` the engine derives `B ⟹ A` and
  re-derives `A`. Step 5 re-derives the hypothesis itself. That redundancy is
  the *rule set's*, not the fork boundary's, and this change does not touch
  it: 82.7 % of what a resumed `zebra2 -e` fork still narrates is redundant,
  against 95.6 % before.
- **Step 322 is the root's own unpropagated delta.** A singleton death writes
  `(not h)` at root and root is *not* re-saturated, so the first fork after it
  is the first thing to propagate it — 32 such writebacks stand between root
  saturation and this solution node. Re-saturating root after a writeback
  would move those steps out of the fork; it is a separate change with its own
  observable, and it is not part of this stage.

## What is unchanged, and what is not

Verified over the whole corpus by `utils/fork_delta_verify.py` — one binary,
two arms, 1.08 M enterings compared fact by fact:

- **unchanged**: the verdict, `k`, the models, the query bindings, the printed
  unsat core, the entering count, each entering's `kind`, every **alive**
  fork's fixpoint fact for fact, and all 85 fields of `summary.json` —
  **T0 and T1 in full**;
- **changed**: the firing lists themselves (T2 and T3), and — the finding this
  stage did not expect — the **proof structure**: which of several equally
  valid derivations is recorded first, for 267 529 facts. See
  [baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured) for
  the numbers,
  [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
  for the decision, and [D3](../divergences.md) for what was accepted.
