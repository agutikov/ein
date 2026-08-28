# D5 — Does T1 ratify Q-M1e.2 as well as Q-M1e.1?

**Touches:** [T1e.1.1.1](README.md#task-t1e111--ratify-the-standard-of-proof).
**Cheapest to settle up front**, because Q4's likeliest disposition depends on
it and would otherwise be decided twice.

## The two rules

[**Q-M1e.1**](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)
— *what counts as **refuted***. Three clauses: a **behaviour** is refuted only
by an executed probe banked as a test; an **absence** is refuted by naming the
thing; a **risk** is not refutable by argument at all — only `fixed`,
`accepted` with the argument written at the site, or `deferred`. T1's job is
to write it into the ledger as decided.

[**Q-M1e.2**](../../open_questions.md#q-m1e2--may-a-review-finding-be-closed-by-a-comment)
— *when a written argument is enough*. Proposed rule: **an argument suffices
when its premise is itself enforced.** It has **no owning stage**.

They are one rule read from two ends. Q-M1e.1 says a risk cannot be argued
away; Q-M1e.2 says what an acceptable argument looks like when you `accept`
one anyway.

## Why Q4 needs both

Suppose [D1](d1_q4_which_route_reaches_the_site.md) lands on option C and the
alive-∅ site turns out unreachable at stock configuration. The disposition is
`accepted`, and the argument written at `solve.rs:1544` would be:

> `compute_alive` cannot return ∅ without passing through a singleton, and the
> singleton path re-saturates and re-checks.

Apply Q-M1e.2's test: **is that premise enforced?** Nothing asserts it. No
`debug_assert`, no test, no invariant check. So by Q-M1e.2's own rule the
argument is *not* sufficient and Q4 needs the check rather than the comment —
which is the opposite of what `accepted` would have concluded.

The repo's two precedents are the calibration:

| | premise | enforced? | verdict |
|---|---|---|---|
| `design/02`'s determinism argument | canonical ordering everywhere a traversal reads | yes, by the ordering tests | argument is enough |
| [ST-M1](../../README.md#the-findings)'s alive-set invariant | *rules assert no new objects or relations* | **no** | needs the cheap post-fixpoint check |
| [ST-L1](../../README.md#the-findings)'s `EqClasses` auto-vivification | *nothing fires equality propagation* | yes — `naf_semantics::matching_does_not_resolve_equality_classes` | a comment is enough |

And note [D4](d4_q_m1e9_upward_closure.md) has just supplied a fourth row of
the same kind, in the failing direction: design/08's `dead`-is-monotone
premise was **written down and unchecked**, and a twenty-line program broke
it. That is the strongest argument available for adopting Q-M1e.2's rule now
rather than at S1e.2.2.

## Options

| | T1 does | consequence |
|---|---|---|
| **A** | ratify Q-M1e.1 **and** Q-M1e.2 together, both as decided, both written into `docs/kernel/` prose rather than only the ledger | half a day instead of a quarter. Every later `accepted` is then mechanical, and Q4 is decided once |
| **B** | ratify Q-M1e.1 only; leave Q-M1e.2 to whoever reaches it | status quo. Q-M1e.2 has **no owning stage**, so "whoever reaches it" is currently nobody, and the first `accepted` disposition decides it implicitly |
| **C** | ratify Q-M1e.1, and give Q-M1e.2 an explicit owner ([S1e.2.2](../../p1e.2_high/s1e.2.2_code_doc_consistency.md) or [S1e.3.1](../../p1e.3_medium/s1e.3.1_correctness.md)) without deciding it here | keeps T1 small and stops the drift, at the cost of Q4 possibly being re-opened later |

**Recommended: A.** It is the difference between a quarter-day and a half-day,
and D4 has just demonstrated the failure mode the rule exists to prevent.

## Where the ratified rules live

Not in the plan. [Q-M1e.1](../../open_questions.md#q-m1e1--what-is-the-standard-of-proof-for-refuted)'s
own third clause says an argument *"goes beside the code, not into a plan
file"*, and the same applies to the rule itself. Candidates:
[`defined_behaviour.md`](../../../../docs/kernel/defined_behaviour.md) (which
P1e.1 Q3 is amending anyway) or a `design/`-style page. Decide the file when
T1 runs; the ledger keeps the pointer.
