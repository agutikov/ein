# 53 · Q-M1f.575: what one `:assert` holds beside `(open R)` and beside `(open F)`

[← part 2](../review/part-2/README.md) · [← the phase](../README.md) ·
[the ledger](../../open_questions.md#q-m1f575--what-one-assert-holds-beside-open-r-and-beside-open-f)

**Ruled 2026-11-20, and out of the review bundle**, where it was item 53 of part 2. Your answer, in the part's
answers' file:

> (a)

It is filed under the question in the ledger and in [the phase's README](../README.md#the-answers-to-items-52-to-55).
It leaves nothing open. The page is kept, with its programs, as the survey the second part's stages cite; where it says
*the default*, read it as it stood before the answer. Its outputs and its tools (`runs/` and `tools/` below) are
on this machine under `build/review-archive/p1f21-part-2/53-Q-M1f.575/`, as they were run.

**From your answer to item 48**, 2026-11-20: *"Rule can assert obligations (which imply hypotheses) and pure
hypotheses independently. It can assert (open R) - and then it works as before, plus it cant assert (open (fact))
with whatever other fact, in the same assert. Identical open facts deduplicate naturally. Exactly (b)."*

**The gap.** *cant* reads two ways, and the two are opposite: that a rule *can* conclude `(open (fact))` beside any
other fact in one `:assert`, or that the rule which concludes `(open R)` *can't* conclude anything else beside it.

Nothing is built: every run is today's engine, the gate build of master at `edbe6c3b`
(`build/review-bin/ein-head-edbe6c3b`), on the programs in [`programs/`](programs/). `tools/run.sh` re-takes them
into `runs/` (both ignored); `runs/cells.txt` has one block a program.

## 1. What today's engine does

| program | today |
|---|---|
| [`open-r-with-a-fact.target.ein`](programs/open-r-with-a-fact.target.ein): `:assert (and (open seats) (waiting ?a))` | refused: `(open R)` is a rule's whole `:assert` |
| [`hrule-two-facts.target.ein`](programs/hrule-two-facts.target.ein): a hypothesis rule with `:assert (and (p ?x) (q ?x))` | refused: *an hrule proposes one fact per match, each a commitment of its own* |
| [`question-with-a-fact.ein`](programs/question-with-a-fact.ein): an ordinary rule with `:assert (and (question (p ?x)) (seen ?x))`, `question` standing for `open` | loads: root holds `(seen A)` `(seen B)`, and $H$ is `(p A)` `(p B)` |

## 2. The options

- **(a)** `(open R)` is a rule's whole `:assert`, as today: the obligation *works as before*. `(open F)` is a
  conclusion like any fact: it stands in a conjunction with facts and with other `(open F)`. **Default.**
- **(b)** Both stand in any conjunction: one rule concludes `(open R)`, `(open F)` and facts together. An obligation
  is read by a pass at the fixpoint and derives nothing in saturation, so the facts beside its verdict would be
  written by that pass too.
- **(c)** Each is a rule's whole `:assert`: a rule that concludes `(open F)` concludes that one hypothesis and
  nothing else, as a hypothesis rule concludes one fact today.

## 3. The ruling

Ruled: the answer at the head of this page.
