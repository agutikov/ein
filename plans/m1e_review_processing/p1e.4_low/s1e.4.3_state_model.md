# S1e.4.3 — State model (Low)

**Phase:** [P1e.4](README.md) (Low)
**Estimate:** 0.5 days
**Depends on:** nothing.
**Findings:** [`ST-L1`](../review/state-model/low.md).
**Related:**
[Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
— this is the finding the *comment is enough* side of that rule was written
for.

## Context

`EqClasses` **auto-vivifies on read**. Merely asking
`kb.classes().equivalent(a, c)` inserts `c` into the parent map
([`kb.rs:415-481`](../../../ein.rs/crates/ein-core/src/kb.rs); the behaviour
is documented by the type's own test at `:1909-1923`), so `classes()` output
order depends on **query history**, and `fork()` copies whatever the queries
left behind.

It is faithful to `ein.py` and it is inert today: nothing fires equality
propagation — O4 is a stub, and
`naf_semantics::matching_does_not_resolve_equality_classes` pins that.

What makes it a finding rather than a curiosity is where it sits. A
query-mutates-state API is exactly the shape this project's determinism rules
exist to keep away from observables — the rules that produced no `Ord` on
`Symbol`/`Value`, the rank tables, the content-based candidate order and a CI
lint against hash-map iteration. And the first real consumer, the F4 e-graph
seam, would inherit it **silently**: at that point `classes()` output becomes
an observable and its order becomes a function of what someone asked earlier.

## Acceptance

- Either `find` is non-vivifying (a lookup, not a union-find insert), or the
  hazard is stated at the point a future consumer would wire it — and *stated*
  means where the wiring happens, not only where the map is.
- The existing test that documents the vivification stays, with its comment
  updated to say whether the behaviour is intended or tolerated.
- The disposition is recorded against
  [Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)'s
  rule, since this is the clean case for it: the argument's premise —
  *nothing fires equality propagation* — is itself enforced by a named test.

## Tasks

### Task T1e.4.3.1 — Decide: non-vivifying find, or a loud comment

**Non-vivifying find** is the stronger fix and is probably a few lines: a
lookup that returns the representative if present and the element itself if
absent, without inserting. The risk is parity — `ein.py`'s union-find
vivified, and if any golden or behaviour depends on the vivification's side
effect, changing it is a semantic change in a dormant subsystem, which is a
strange place to spend risk.

**The loud comment** is the honest cheap path and the one
[Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
licenses: the premise (*nothing consumes `classes()` as an observable*) is
enforced by a named test, so the argument is sufficient. The comment must sit
where the future consumer arrives — at `classes()`'s public accessor and in
whatever F4-seam note exists — and must say the specific thing, not the
general one: *this is a union-find `find`; reading it inserts; if you make
`classes()` output observable, make `find` non-vivifying first.*

Recommendation: take the comment now and file the change with F4, **unless**
the non-vivifying version turns out to break nothing in ten minutes of trying
— in which case take it, since a hazard removed beats a hazard documented.

### Task T1e.4.3.2 — Check the fork copy

One thing the review notes in passing and does not pursue: `fork()` copies the
parent map, so vivification in a parent is inherited by every fork made
afterwards, and vivification in a fork is not seen by siblings. If anything
ever *does* read `classes()`, that asymmetry is a second-order determinism
hazard on top of the first — the order would depend not only on query history
but on **when** in the search the query happened.

Confirm the copy semantics, and add the sentence to whichever fix T1e.4.3.1
takes. It costs a line and it is the part a future consumer is least likely to
work out for themselves.

---

## ✅ Done 2026-09-01 — the licence to leave it alone did not exist

**Disposition: fixed** — a non-vivifying `find`, ten net lines — and **not**
the comment this stage and
[Q-M1e.2](../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
both expected. The reason is the interesting part of the stage.

### The premise was enforced by something else

`docs/kernel/standard_of_proof.md`'s Rule 2 admits an argument only while its
premise *"is itself enforced by something that fails when it stops being
true"*, and its calibration table named this finding as the clean case:
premise *nothing fires equality propagation*, enforced by
`naf_semantics::matching_does_not_resolve_equality_classes`.

**It is not.** That test unions **by hand** and then asserts the *matcher*
ignores the class — a claim about the boundary's extent-size stamp, which is
what its own doc comment says. Probed by making `Kb::add_fact` union the
relation with every symbol argument of every stored fact — equality
propagation firing on every derivation — the named test stays **green**.

What *would* have caught that probe is
[`fork_cost.rs`](../../../ein.rs/crates/ein-core/tests/fork_cost.rs), and only
incidentally: a growing parent map breaks the O(1) fork, so
`a_fork_costs_the_same_at_ten_facts_and_at_ten_thousand` fails with 228 172
bytes per fork against 940. That enforces the **weaker** *propagation does not
scale with the fact count* — a bounded propagation would leave it green.

So the row was **misattributed** rather than unenforced, and that is a third
failure mode neither Q-M1e.1 nor Q-M1e.2 had named: **an argument citing a
real, green, well-named test can still be unsupported, and it is harder to see
than an uncited one, because the citation is what stops anybody looking.**
Rule 2 carries the second question now — *does the named test enforce the
premise, or something adjacent to it?* — and the table's summary went from
*two of four are the pattern's absence* to three of four, all three probed,
all three failed.

### Two hazards, not one

The stage and the review both framed this as determinism. It is also **fork
cost**: `Kb::branch` deep-copies the map, so every name ever merely *asked
about* became permanent per-fork bytes on the path P1a.7 wants hundreds of live
copies of. The comment route would have left that standing.

### T1e.4.3.1 — the ten minutes

The stage's escape clause is *"unless the non-vivifying version turns out to
break nothing in ten minutes of trying"*. It broke **exactly one assertion**,
and that assertion is the one documenting the defect (`kb.rs`'s
`equality_classes_are_copied_and_stay_inert`, whose last line used to read
`[(a, [a, b]), (c, [c])]`). The parity risk the stage priced is empty:
[design/03 §8](../../../docs/history/m1a_rust/design/03_data_model.md) names
**path compression** and **union-by-first-argument** as the ported behaviours
and both are kept; auto-vivification is in no contract; `EqClasses` reaches no
golden, no `Kb::diff`, no `Layer` and no `.einb` section; and `ein.py` is gone,
so *faithful to ein.py* is a claim nothing can re-measure. The divergence
ledger is closed by its own header, so no entry is owed.

`find` is a lookup; `record` is the vivifying half, private, and called only by
`union` — which is the shape the finding asked for: *a write is the only
operation entitled to grow the map*.

### T1e.4.3.2 — the fork copy

Confirmed and kept in the test rather than only in prose: a merge in a parent
is inherited by every fork made afterwards, a merge in a fork is invisible to
its siblings. What no longer holds is that a **question** can do it.

### What holds it

- `kb::tests::asking_a_question_does_not_move_the_answer` — the property, not
  the example: the same unions produce the same `classes()` whether or not
  anything was asked first. It fails on the pre-S1e.4.3 engine in both length
  and order.
- `equality_classes_are_copied_and_stay_inert`, amended, still holds the fork
  asymmetry.
- The argument is at the site — `EqClasses`'s own doc comment and
  `Kb::classes()`, the accessor an F4 consumer would call. The stage's
  acceptance asked for it *"where the wiring happens"*; there is no wiring
  point (`grep -rn F4 ein.rs/crates` finds two doc comments and no code), which
  is itself the reason the hazard had to go rather than be documented for a
  reader who has nowhere to read.

**Gate:** `cargo test --workspace` — **808 tests, 0 failures**. No golden
moved: `EqClasses` is absent from `Kb::diff`, from `materialise`, and from the
`.einb` format, which has no section for it and no way to make one.
