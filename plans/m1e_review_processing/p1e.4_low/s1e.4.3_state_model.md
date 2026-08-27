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
