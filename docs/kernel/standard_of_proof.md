# The standard of proof

**Two rules, ratified 2026-08-28.** They say what it takes to call a claim
about this repo *settled*, and they exist because the tree they live in is
load-bearing: since M1a [P1a.10](../history/m1a_rust/README.md#p1a10--one-implementation)
`docs/kernel/` is the only statement of intent that is not also the
implementation, so a claim here is checked by `cargo test --workspace` and by
nothing else.

They were written as [M1e](../../plans/m1e_review_processing/README.md)'s
[Q-M1e.1](../../plans/m1e_review_processing/open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
and
[Q-M1e.2](../../plans/m1e_review_processing/open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
— *what counts as refuted*, and *when an argument is enough* — and ratified
together because they are one rule read from two ends. They apply to any claim
about the engine, whether it comes from a review, a probe, or a page like this
one.

## Rule 1 — what counts as **refuted**

| the claim is about… | it is refuted by… |
|---|---|
| a **behaviour** — *the engine does X* | an **executed probe, banked as a test**. Not a reading of the source, not an argument from the design |
| an **absence** — *nothing checks Y* | **naming the thing** that checks it |
| a **risk** — *X could happen* | **nothing.** A risk is not refutable by argument |

The third is the one that will be argued with, so it is stated as a closure
rule: a risk has exactly three honest outcomes — **fixed**, **accepted** with
the argument written at the site, or **deferred** to a named owner. *"I could
not build it"* is not a refutation; it is a probe that did not reproduce, and
it is banked as one.

## Rule 2 — when an **argument** is enough

> **An argument suffices when its premise is itself enforced.**

`accepted` is a first-class outcome, and a written argument is often the right
one. What makes it acceptable is not its plausibility but whether the thing it
rests on is checked by something that fails when it stops being true.

| | premise | enforced? | verdict |
|---|---|---|---|
| [design/02](../history/m1a_rust/design/02_determinism_and_order.md)'s determinism argument | canonical ordering everywhere a traversal reads | yes, by the ordering tests | the argument is enough |
| `EqClasses` auto-vivification | *nothing fires equality propagation* | **the named test does not enforce it — 2026-09-01** | the row was **misattributed**, and the hazard was removed instead (M1e [S1e.4.3](../../plans/m1e_review_processing/p1e.4_low/s1e.4.3_state_model.md)) |
| the alive-set invariant | *rules assert no new objects or relations* | **checked since 2026-08-31, and it fails** | the check was built (M1e [S1e.3.3](../../plans/m1e_review_processing/p1e.3_medium/s1e.3.3_state_model.md)) and found the premise **false on two corpus programs**. So the argument was never available: what stands in its place is a named breach set, a measurement that neither breach costs anything, and an eleven-line fixture on which one does — `k = 0, exhausted = true` where a model exists. § 3.3 of [`defined_behaviour.md`](defined_behaviour.md#33-the-m1-alive-set-invariant-operationally) |
| [design/08](../history/m1a_rust/design/08_parallelism.md)'s `dead` is monotone | *the KB is append-only, so nothing retracts* | **no** | broken by a twenty-line program — see below |

One of the four is the pattern working; three are the pattern's absence, and
one of those is what settled the rule. **All three of the absent rows have
since been probed and all three premises failed** — `dead` is monotone by a
twenty-line program (Q-M1e.9, 2026-08-28), the alive-set invariant by an
eleven-line one (Q-M1e.21, 2026-08-31), and `EqClasses` by a mutation
(S1e.4.3, 2026-09-01). Three for three is not a sample, but it is the only
evidence there is about what an unenforced premise is worth.

**And the third failed in a way the first two did not, which is why it is worth
its own sentence.** The `EqClasses` row's premise may well have been *true*;
what was false was the **citation**. `naf_semantics::matching_does_not_resolve_equality_classes`
unions by hand and then asserts the *matcher* ignores the class — a different
claim, and a probe that made the engine union on every stored fact left it
green. The test that would actually have caught that probe is
[`fork_cost.rs`](../../ein.rs/crates/ein-core/tests/fork_cost.rs), and only
because a growing map breaks an O(1) fork, so what the tree really enforced was
the weaker *propagation does not scale with the fact count* — a bounded
propagation would have passed both.

So Rule 2 has a second question, and it is the one that is easy to skip:
**does the named test enforce the premise, or something adjacent to it?** An
argument citing a real, green, well-named test can still be unsupported, and
that failure mode is harder to see than an uncited one — the citation is what
stops anybody looking. When the answer is *adjacent*, the row is not evidence
that the argument is enough; it is evidence that nobody has checked.

## Where an argument goes

**Beside the code, not in a plan file.** A plan is read once by whoever
executes it; the next reader of the code has the same question and no answer at
the site. `design/02` is the repo's own precedent — an argument that lives
where a reader of the determinism rules finds it.

The same applies to a rule *about* claims, which is why this page is in
`docs/kernel/` rather than in the milestone that ratified it, and why it is not
in [`defined_behaviour.md`](defined_behaviour.md): that page states what the
**engine** does, and this one states what it takes to know it.

## The worked example, and it is not hypothetical

`design/08` § The objects states, as a definition:

> `dead(X)` — `X` holds a contradiction. **Monotone**: `X ⊆ Y ∧ dead(X) ⇒
> dead(Y)`, because the KB is append-only and nothing retracts.

Nothing enforced it. Three shipped mechanisms read it — the lookahead kill
cache, the singleton writeback, and the no-good store's width-1 clause — and on
2026-08-28 a twenty-line program broke it: with a rule that refutes `(p A)`
only while `(q A)` is missing, `{(p A)}` is dead and `{(p A), (q A)}` is alive,
so `dead` is not upward-closed under `absent`
([Q-M1e.9](../../plans/m1e_review_processing/open_questions.md#q-m1e9--is-dead-really-upward-closed-under-absent);
[`absent_semantics.md`](inference/absent_semantics.md) C3 had stated the
caveat, unreconciled, the whole time).

The premise was written down, was believed, was load-bearing, and was wrong for
a year. That is what Rule 2 is for.

## See also

- [`defined_behaviour.md`](defined_behaviour.md) — what the engine does, as
  opposed to what it takes to know it.
- [`inference/solution_semantics.md`](inference/solution_semantics.md) § 6 —
  a page applying Rule 1 to itself: a soundness row that said *yes* was
  withdrawn on 2026-08-28 when the probe that tested it came back the other
  way.
- [`inference/absent_semantics.md`](inference/absent_semantics.md) — C1–C6,
  the corollaries the `dead`-monotonicity premise has to be read beside.
- [M1e's open questions](../../plans/m1e_review_processing/open_questions.md)
  — where the two rules were argued, and the ledger that records what each
  finding's disposition was.
