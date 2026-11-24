# 86 · 2 — a read under `absent` flags nothing (default 2, Q-M1f.686)

[← item 86](../README.md) · [← part 1](../1-the-absent-is-read/README.md) ·
[part 3 →](../3-the-oracles-listing/README.md) · [part 4 →](../4-the-witness-and-its-decision/README.md)

**This is the default with a consequence.**

## The question

Once a hypothesis rule reads its `absent`, what it proposes can depend on the state. Take a rule that reads, under
an `absent`, a relation **a commitment can change**: the alibis, when the alibis are themselves hypotheses. At root
nobody has an alibi, so it proposes every culprit. At a state above root that commits `(alibi Ann)`, its `absent`
fails for Ann, and that state's own pass proposes **fewer** culprits. Is that a site of the static check, which
flags the program and claims nothing of its answer?

- **The default: no.** Such a read can only take a hypothesis away above root, never add one. So nothing is first
  held above root, which is what Q-M1f.574 flags. $R$ is over root's $H$ and reads nothing a state above root
  proposes. The states above root hold fewer of root's hypotheses among their `open` facts, and that is all.
- **The other reading: yes**, a site at every depth: a program whose states above root propose other than root did
  is flagged, and nothing is claimed of its answer.

**The contrast is fixed either way.** A rule that reads such a relation **positively** can propose, above root, a
hypothesis root did not: that is a site already (Q-M1f.574), and the program is flagged.

<img src="runs/concept.svg" alt="alibi-supposed: root proposes alibi Ann, alibi Bob, culprit Ann, culprit Bob; the state committing alibi Ann proposes culprit Bob but not culprit Ann, taking one away and adding none: no site, R over root's H, so {alibi Ann, culprit Ann} is in R. alibi-then-witness: root proposes the two alibis; the state committing alibi Ann proposes witness Ann, first held above root: a site, the program flagged">

*Drawn by hand (`../tools/concept-2.dot` → `runs/concept.svg`).*

## The example: the alibis are supposed too

[`programs/alibi-supposed.ein`](programs/alibi-supposed.ein):

```lisp
(present Ann) (present Bob)
(rule maybe-alibi () :match (present ?x) :assert (open (alibi ?x)) …)            ; alibis are guesses
(rule suspect ()
  :match  (and (present ?x) (absent (alibi ?x)))                                  ; reads them under absent
  :assert (open (culprit ?x)) …)
```

On the final binary: `ein solve --monotone` says *yes — no site*; $H$ is 4 candidates, two alibis and two culprits;
$R$ is all 16 states. The `open` facts each state of the cube holds (`ein oracle --states`,
`runs/alibi-supposed.new.nodes.txt`), the first eight:

| the commitment | the `open` facts its own pass writes |
|---|---|
| ∅ (root) | alibi Ann, alibi Bob, **culprit Ann**, **culprit Bob** |
| `{(alibi Ann)}` | alibi Ann, alibi Bob, culprit Bob: **culprit Ann taken away** |
| `{(alibi Bob)}` | alibi Ann, alibi Bob, culprit Ann: **culprit Bob taken away** |
| `{(alibi Ann), (alibi Bob)}` | alibi Ann, alibi Bob: **both taken away** |
| `{(culprit Ann)}` | all four |
| **`{(alibi Ann), (culprit Ann)}`** | alibi Ann, alibi Bob, culprit Bob |
| `{(alibi Bob), (culprit Ann)}` | alibi Ann, alibi Bob, culprit Ann |
| `{(alibi Ann), (alibi Bob), (culprit Ann)}` | alibi Ann, alibi Bob |

**What the default means here**: `{(alibi Ann), (culprit Ann)}` is a member of $R$. Ann has an alibi and is the
culprit, though at that state `suspect` would not propose her. $R$ is over root's $H$, and no state's own pass
removes a candidate from it. Under the other reading the program would be flagged, *THIS PROGRAM IS NOT MONOTONE*,
and nothing claimed of these 16 states.

## The contrast: a positive read adds a hypothesis, and is flagged

[`programs/alibi-then-witness.ein`](programs/alibi-then-witness.ein): `ask` proposes a witness among those with an
alibi, reading the supposed alibi **positively**.

```lisp
(rule ask () :match (and (present ?x) (alibi ?x)) :assert (open (witness ?x)) …)
```

At root nobody has an alibi, so `ask` proposes nothing; committing `(alibi Ann)` makes that state propose
`(witness Ann)`, a hypothesis root does not hold. Both engines flag it (`runs/alibi-then-witness.*.monotone.txt`):

<!-- output -->
```text
  monotone          no — 1 site
    site            ask: hypothesis reads alibi at absent depth 0, concludes (open …) — at stake: the hypotheses: a state can hold one root does not
```

and every read-out of it opens with the `WARNING` row.

| `alibi-supposed`, its region: all 16 states in $R$ | `alibi-then-witness`, its region, flagged |
|---|---|
| <img src="runs/alibi-supposed.new.region.svg" alt="the cube of four candidates, every node in R"> | <img src="runs/alibi-then-witness.new.region.svg" alt="the cube of the two alibis, drawn for a program the static check flags"> |

## Before the fix

The engine before the fix answers `alibi-supposed` the same: `absent` unread, every state proposes both culprits, and
the check calls it monotone. Its listing showed no `open` fact at any node, which is part 3. The fix changed what each
state above root holds among its `open` facts, not $H$ and not $R$.
