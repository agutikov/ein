# S1e.1b.8 — May a refutation rest on an `absent`?

**Phase:** [P1e.1b](README.md) (The structure of the hypothesis set)
**Estimate:** 3 days
**Depends on:** [S1e.1b.1](s1e.1b.1_exclusion_census.md) — this stage is that
census's **soundness precondition**, and the census is what measures it.
**Blocks:** [S1e.1b.3](s1e.1b.3_the_restricted_join.md), which spends the
exclusion relation, and [S1e.1b.6](s1e.1b.6_obligations_under_hypothesis.md),
whose domain sentence has to say whether this shape is inside it.
**Answers:** the **language** half of
[Q-M1e.9](../open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent)
— the engine half stays
[D4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)'s.
**Source:** the user's note of 2026-08-28, recorded in
[D4 § The user's reading](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md):
*"maybe emitting `(false)` or `(not …)` under `absent` is not a good idea. It
also worth investigation stage."*

## Why it is this phase's, and not a patch to the search

The phase's first rung is one predicate:

```
excludes(h₁, h₂)  ≡  saturate(rules ∪ ontology ∪ {h₁, h₂})  derives (false)
```

and the claim built on it is that this is a property of the **program**,
computed once at load and valid at every node. **A `(false)` that rests on an
`absent` is precisely the mechanism by which that claim would be false.** The
minimal KB has fewer facts than any node below it, so an `absent` guard passes
there and fails deeper: the exclusion is computed on the smallest world and
spent on larger ones. [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
C3 states the direction that bites — *removing a fact can flip an absent and
**fabricate** a contradiction the full KB never had* — and a minimal KB is
nothing but a full KB with facts removed.

S1e.1b.1 already predicts the symptom and calls it *"a bug in the minimal KB's
definition, and finding one is this stage's most valuable outcome"*. This stage
says it may instead be a bug in the **rule**, and that the two have different
fixes: one narrows what *minimal* means, the other narrows what a rule may
conclude.

## What is already established

- **The mechanism.** D4's probe, and the matrix in which five of six shipped
  configurations answer a twenty-line program wrongly. `{(p A)}` dies, the
  writeback stores `(not (p A))`, and L2 never sees `{(p A), (q A)}`.
- **The probe is a mis-encoded obligation**, per the user's reading: it says
  *a world with `p` and without `q` is false* and never says *`q` is required*,
  which is what `(open ?R)` — form **G** of
  [`obligation_forms.md`](../../../docs/history/m1d_satisfiability/obligation_forms.md)'s
  menu — exists to say. So the engine's complaint about it is not wrong; the
  **word** may be.
- **The shape is in the stdlib.** A syntactic census (2026-08-28) of
  `stdlib/`, `examples/` and `tests/` finds **60 rules** whose `:match` carries
  an `(absent …)` and whose `:assert` is `(false)` (14) or a `(not …)` (46).
  The discriminator is whether the `absent` reads a relation the search can
  still extend:

  | family | the `absent` reads | exposed? |
  |---|---|---|
  | `std.slots`' `slot-prune-*` / `slot-endpoint-*` / `slot-adjacent-*-neg`, and the six inline twins in every `zebra2*` | `?S`, the **given** adjacency structure over positions | no — and this is why the corpus is quiet |
  | `std.algebra`'s **`connex`** — `(absent (?R ?a ?b)) (absent (?R ?b ?a)) ⟹ (false)` | `?R`, the **subject** relation | **yes by shape**: `(connex color-loc)` on a puzzle whose `*-loc` the rung proposes is D4's probe with a stdlib rule in place of `bad` |

- **And the repo has already written the distinction down** — in a fixture
  header, not in a rule.
  [`tests/stdlib/closure/03_closed_and_owing.ein`](../../../tests/stdlib/closure/03_closed_and_owing.ein):

  > `total`'s stored-negative discipline is what stops it firing `(false)` on
  > every empty-yet state, and is the **right** way to write it — **`connex` is
  > what the other way looks like**.

  `total` demands a stored negative for every candidate
  (`(forall ?b (?isa ?b ?B) (not (?R ?a ?b)))`) before concluding; `connex`
  concludes from an absence. Two stdlib rules, one idiom apart, and the
  difference has never been stated as a rule anyone must follow.

## The question, in three parts

1. **What counts as *still extendable*?** The union of the hypothesis-eligible
   relations (the generator's `allowed` minus `excluded` minus `__closed__`)
   and the rule-derived ones — which is `warn-derived-naf`'s existing subject.
   The compiler knows both already; that is what makes a load-time answer
   possible at all.
2. **What replaces the pattern?** Both replacements exist and neither is
   documented as *the* way: `total`'s stored-negative scan for a refutation,
   `(open ?R)` for a requirement. The constructive half of this stage is
   writing that down.
3. **What happens to a program that does it anyway?** Refuse, warn, scope, or
   accept-and-say-so.

## Options

| | ruling | consequence |
|---|---|---|
| **A — forbid** | a `(false)` or `(not …)` conclusion may not rest on an `(absent P)` whose relation is still extendable; load error naming the replacement | strongest, and it makes [D4](../p1e.1_open_questions/s1e.1.1_search_soundness_probes/d4_q_m1e9_upward_closure.md)'s option C **unnecessary** — nothing can write a negative whose justification a later fact invalidates, so the no-good store needs no world-awareness. Costs `connex` a rewrite and a `broken/load/` fixture |
| **B — warn** | extend `warn-derived-naf` to the hypothesis-eligible case, default off | cheapest, changes nothing, and leaves the wrong answer shipping. It is D4's option B with a wider watch list |
| **C — scope** | keep it legal, tag the conclusion with its `absent` premises (`Prov::absent` **already records them**) and refuse to store it beyond the world it was derived in | D4's option C. The real fix *if* the pattern must stay legal, and the first walk that interprets what `Prov::absent` has been recording since S1.21.8 |
| **D — allow, and fix the word** | the shape is legal; the answer to such a program is *your program is ill-formed*, which is **not** *your constraints are unsatisfiable* | the user's own reading taken to its end. It needs `Q-M1d.1`'s verdict vocabulary, which M1e does not own — so it is a companion to A/B/C, not an alternative |

**Recommended: A, with `connex` rewritten as the proof that A is affordable —
and D's distinction recorded whatever else is chosen.** A is decidable at load
from information the compiler already has, it removes the premise rather than
compensating for it, and the census says the corpus survives it. What would
overturn it is a second legitimate user found by T1: one rule that needs to
refute from an absence over an extendable relation and cannot be written
`total`-style. If T1 finds one, A becomes C.

## Acceptance

- **The census exists, is re-takable, and names the exposed set per corpus
  entry** — not the 60 syntactic sites but the subset whose `absent` reads an
  extendable relation under that entry's declared runs. It is a column of
  [S1e.1b.1](s1e.1b.1_exclusion_census.md)'s script if it fits there and a
  small one of its own if it does not ([AR-M1](../README.md#the-findings): no
  fourth copy).
- **`connex` has a disposition** — rewritten, restricted to closed relations,
  or kept with the reason written at the rule. It is the stdlib's only exposure
  and the ruling has to survive contact with it.
- **The ruling is written in
  [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
  as a numbered corollary**, beside C3 and C6, which are what it follows from —
  not in a plan file
  ([Q-M1e.1](../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
  third rule).
- **If A: the diagnostic names the replacement**, has a `broken/load/` fixture
  with a positioned message, and D4's option B is the same code rather than a
  second check.
- **Not one answer moves** on any program that survives the ruling — the
  phase's standing acceptance, and here it is also the test that the ruling is
  narrow enough.

## Tasks

### Task T1e.1b.8.1 — The census, from syntax to exposure

One day. Start from the 60 syntactic sites, then compute per corpus entry
which of them have an `absent` over a relation that entry's runs can extend.
The two-line answer to look for: *how many rules are exposed, and does any
program actually reach one?* Today's expectation is **one rule and no
program** — `connex`, unexercised on a hypothesis-eligible relation — and if
that holds, the ruling is cheap.

### Task T1e.1b.8.2 — Exposed by shape, or in fact?

Half a day. Declare `(connex R)` on a relation the rung proposes and run it. If
the state that a hypothesis would have repaired is refuted, `connex` is D4's
probe wearing a stdlib name, it is a **stdlib defect**, and it gets a
`tests/stdlib/algebra/` fixture whether or not the ruling lands.

### Task T1e.1b.8.3 — Price the replacement

Half a day. Rewrite `connex` in `total`'s stored-negative style and check it
still does its job on the fixture that activates it. This is what makes A a
measurement rather than a preference: *the same constraint, written the way the
repo already calls right.*

### Task T1e.1b.8.4 — Rule, and write it where rules are written

Half a day. Pick from A–D, write the corollary into `absent_semantics.md`, and
say what D4's B becomes. If A, T5 is the code.

### Task T1e.1b.8.5 — The check, if A

Half a day. The compiler walks each rule's guards for `absent` heads and its
asserts for `(false)` / `not`, intersects with the extendable set, and refuses
with a message naming `total`'s form and `(open ?R)`. One `broken/load/`
fixture, one corpus entry.

## Notes

**Why not P1e.2.** It is not one of the 63 findings and it is not a defect in a
*surface*. It is a question about what a rule may conclude — the language, not
the read-out — and the phase that is already reasoning about what a hypothesis
set means is where it can be answered without inventing a context for it.

**The honest risk.** A forbids a shape the language has always allowed, on the
evidence that one stdlib rule uses it and no program exercises that rule
dangerously. That is a thin margin, and T1 is what widens or closes it. A
ruling made before the census is a ruling made on the shape of the argument
rather than on its extent — which is the failure this phase's own
[S1e.1b.1](s1e.1b.1_exclusion_census.md) exists to avoid one rung lower.
