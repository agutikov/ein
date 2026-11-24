# 86 · 1 — the `absent` is read: the wrong answer and its fix (default 1, Q-M1f.685)

[← item 86](../README.md) · [part 2 →](../2-a-read-under-absent-flags-nothing/README.md) ·
[part 3 →](../3-the-oracles-listing/README.md) · [part 4 →](../4-the-witness-and-its-decision/README.md)

## What was wrong

A hypothesis rule is a rule that concludes `(open F)` (Q-M1f.567): at root its conclusions are $H$, the candidates
the search commits. Its `:match` was read **without its `absent` premises**: every binding of the positive premises
proposed a hypothesis, whatever the `absent` said. A rule that concludes a fact, over the same match, read them. The
program is monotone by the static check, so the engine claimed the answer, and the oracle agreed: it builds $H$ by
the same function.

**Since the fix** (`21115a70`) the `absent` is read, judged against the state, as every rule's is: the default taken
(Q-M1f.685). The other reading: a hypothesis rule's `absent` is no part of what it proposes, and the pages say so.

<img src="runs/concept.svg" alt="the match (and (present ?x) (absent (alibi ?x))): for Ann the absent fails, since (alibi Ann) holds, so since the fix nothing is proposed, where before the fix (open (culprit Ann)) was written anyway; for Bob the absent holds and (open (culprit Bob)) is proposed; before, H was both culprits and R four states; since, H is culprit Bob and R two states">

*Drawn by hand (`../tools/concept-1.dot` → `runs/concept.svg`).*

## The simple example: no alibi

[`programs/no-alibi.ein`](programs/no-alibi.ein):

```lisp
(relation present Person :why "{?3} was there")
(relation alibi   Person :why "{?3} has an alibi")
(relation culprit Person :why "{?3} did it")
(present Ann) (present Bob)
(alibi Ann)
(rule suspect ()
  :match  (and (present ?x) (absent (alibi ?x)))
  :assert (open (culprit ?x))
  :why    "suppose {?x} did it: {?x} was there and has no alibi")
```

| | the engine before the fix | since |
|---|---|---|
| `ein show hyps` | `(culprit Ann)`, `(culprit Bob)`: **Ann, who has an alibi, a suspect** | `(culprit Bob)` |
| $R$ | 4 states: ∅, `{Ann}`, `{Bob}`, `{Ann, Bob}` | 2 states: ∅, `{Bob}` |
| the static check | monotone | monotone |

From `runs/no-alibi.pre.*` and `runs/no-alibi.new.*`.

| $R$ before the fix (`ein oracle`, drawn) | $R$ since |
|---|---|
| <img src="runs/no-alibi.pre.region.svg" alt="before the fix: the cube of two candidates, culprit Ann and culprit Bob, all four nodes in R"> | <img src="runs/no-alibi.new.region.svg" alt="since the fix: one candidate, culprit Bob: the empty commitment and culprit Bob, both in R"> |

Root's knowledge base, with the hypotheses root holds drawn under `open`: `runs/no-alibi.pre.kb.svg` and
`runs/no-alibi.new.kb.svg`; the lattices beside them.

## The corpus witness

[`examples/ein-bugs/wrong-answers/hypothesis-rule-absent.ein`](../../../../../../../examples/ein-bugs/wrong-answers/hypothesis-rule-absent.ein),
the probe of item 62 moved into the corpus: one match, `(and (p ?x) (absent (q ?x)))`, `(q a)` written, under a rule
that concludes `(s ?x)` and one that concludes `(open (r ?x))`. Its `:expect` is the right answer, `(region (model)
(model (r b)))`. `ein test` on each engine (`runs/witness.pre.test.txt`, `runs/witness.new.test.txt`):

<!-- output -->
```text
before the fix:  FAILED  — the claim lists 2 states and R has 4
                             a state of R no model lists: (model (r a))
                             a state of R no model lists: (model (r a) (r b))
since:           ok      — 1 file, 1 expectation: 1 held
```

`plain`, the rule that concludes a fact, concluded `(s b)` alone on both engines: the two rules disagreed over one
match before the fix, and agree since.

## What holds it

`ein-infer/tests/hypothesis_rule_absent.rs` holds $H$, and each node's `open` facts, to what a rule that concludes a
fact concludes over the same match, on the witness and six shapes of match. It fails on the function as it was. The
oracle could not be the check: it builds $H$ by the same function. $H$ and $R$ moved on no corpus program, and none
changed flag (931 run before and after).
