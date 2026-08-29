# S1f.10.4 — Bounded groups: rediscovering the bijection

**Phase:** [P1f.10](README.md)
**Estimate:** 3 days
**Depends on:** [S1f.10.2](s1f.10.2_groups.md) — the cover and its overlap.
**Blocks:** nothing. It is the phase's most speculative rung and the one most
likely to end in a written *no*.

## Context

The instruction's second half:

> Next step is somehow functionally find that there are bounded hyp groups,
> e.g. permutations of pets, permutations of nationalities.

A group from [S1f.10.2](s1f.10.2_groups.md) says *at most one*. A **bounded**
group says *exactly one* — and that is a different and much stronger claim,
because *exactly one* is what makes a branch set **jointly exhaustive**, which
is what licenses committing to one alternative and discarding its siblings
with nothing to refute
([`completeness.md`](../../../docs/history/m1d_satisfiability/completeness.md)).

The engine already has this, declared: `(bijective nation-loc)` fans out into
`total` / `surjective` / `functional` / `injective`, and since M1d S1d.2.5 the
`total-owed` / `surjective-owed` obligations make the *exactly one* into a
choice point the search branches on
([`oblgen.rs`](../../../ein.rs/crates/ein-infer/src/oblgen.rs)). The tree
traversal's 86-against-17 204 592 is what that buys.

This stage asks whether it can be **derived** where it is not declared, and
the target is [I-Z1](README.md#the-instances) — `examples/zebra.ein`, whose
one `co-located` equivalence relation carries the same structure with no
`(bijective …)` anywhere.

## What *exactly one* would have to rest on

*At most one* is a negative and comes free from the exclusion graph. *Exactly
one* is a positive: it says some member of the group **must** hold, and no
amount of pairwise refutation implies that. Three routes, and the stage's real
work is choosing between them:

| route | the claim | where the *must* comes from |
|---|---|---|
| **(a) counting** | group of size *n* over *n* objects, and the two clique families cover the same members ⇒ a perfect matching ⇒ exactly one per row and per column | the pigeonhole, applied to the cover's own shape. Needs **closure**: that the object list is complete |
| **(b) the declaration, generalised** | recognise `functional ∧ total` and read the group off it, without requiring the word `bijective` | already half-shipped — `std.closure` is exactly `functional ∧ total ⇒ (__closed__ R)`, opt-in |
| **(c) the obligation** | the group *is* an obligation instance's candidate set, so state one and let the existing rung do the work | zero new machinery; the question becomes *may the loader synthesise an `(open …)` rule?* |

Route (a) is the instruction's ("functionally find"), and it is the one that
needs a closure assumption the kernel does not otherwise make — C1 of
[`domain_contract.md`](../../../docs/history/m1d_satisfiability/domain_contract.md)
is emphatic that *stating* an obligation asks nothing about closure. Route (c)
is the cheapest and hands the whole problem to machinery that already has a
completeness argument written for it.

## Acceptance

- The three routes are each tried on [I-Z1](README.md#the-instances) and
  [I-B06](README.md#the-instances), and the stage **names which it took and
  why**, with the other two's failure written where a reader will look for it.
- Whichever route: `examples/zebra.ein` gets the same branch structure the
  `-obligations` variants of `zebra2` get, **or** the stage states exactly
  what is missing from the program text to make that derivable. A written
  *"not without a closure declaration"* is a full result.
- **The closure assumption, if taken, is stated as a premise and checked.**
  It is `(__closed__ R)` under another name, and this repo already has
  [ST-M1](../../m1e_review_processing/README.md#the-findings) — *an invariant that is the warrant for a
  dedup and is enforced nowhere*. A second one is not acceptable.
- Not one model moves. Same acceptance as
  [S1f.10.3](s1f.10.3_the_restricted_join.md), and it matters more here:
  *exactly one* is a claim that can **remove** models if it is wrong, which is
  the failure class the project treats as worst.
- If the derived structure reaches the **tree traversal** — a program with no
  `(open …)` getting jointly exhaustive branches — then
  [Q6](../../m1e_review_processing/p1e.1_open_questions/s1e.1.1_search_soundness_probes/README.md)'s answer
  applies to it too, and the stage says so rather than inheriting the tree's
  root-only probe by accident.

## Tasks

### Task T1f.10.4.1 — Route (c) first, because it is falsifiable in a day

Hand-write the `(open …)` rule that the group structure of
`examples/zebra.ein` would correspond to, add it to a copy of the file, and
run it under `EIN_TRAVERSAL=tree` against the lattice. If the model sets agree
fact for fact, route (c) is *sound for this program* and the remaining
question is purely whether a loader may synthesise the rule. If they disagree,
the group structure is not an obligation's candidate set and routes (a)/(b)
are the only ones left — a much more valuable finding than a day's work
usually buys.

The `-obligations` fixture pair is the precedent and the shape:
[`examples/zebra2-obligations.ein`](../../../examples/zebra2-obligations.ein)
is `zebra2` with the `(hrule guess …)` and the `:hrules` clause deleted and
nothing else, and it *"solves to the same models in the same number of
enterings as the hrule path it dropped"*.

### Task T1f.10.4.2 — Route (a): the counting argument, and its premise

Two clique families over the same member set, `|rows| = |cols| = n`, every
member in exactly one of each ⇒ the grid. State the premise this needs
(**the object list is complete** — no rule may introduce a new `Color`) and
check it, because it is the same premise
[ST-M1](../../m1e_review_processing/README.md#the-findings) says nothing enforces. The check is cheap
and post-fixpoint: did saturation introduce an object the load-time KB did not
have?

### Task T1f.10.4.3 — Route (b): what `std.closure` already gives

`functional ∧ total ⇒ (__closed__ R)` is opt-in and in the stdlib. Measure how
many corpus entries would satisfy it if imported, and whether `(__closed__ R)`
is already enough for the group to be *exactly one* — because if it is, this
stage's answer is *"import `std.closure`"* and the rest is documentation.

### Task T1f.10.4.4 — Say what it is worth

On [I-Z2M](README.md#the-instances) — 32 models, 23 varying variables in one
coupling component — compare enterings under the lattice, under the tree, and
under whichever route this stage took. The tree's 86-vs-17 204 592 is the bar,
and the honest outcome is that a derived structure reaches some fraction of
it. Name the fraction.

## Notes

The stage is deliberately last-but-one and deliberately allowed to fail. The
phase's shipped value is [S1f.10.3](s1f.10.3_the_restricted_join.md) — *at
most one*, which is free and answer-preserving. *Exactly one* is a semantic
claim about the puzzle's intent, and a milestone that processes a review is a
strange place to make one; if the routes all need a declaration the program
does not carry, the right result is a `Q-M1e.<n>` and a pointer at M1d's
unfinished traversal work, not a heuristic.
