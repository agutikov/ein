# E-116 — A query with no row does not say which of its premises matched nothing

**Kind:** gap
**Status:** open
**Filed:** by S2.2.1, 2026-11-26, on `98125f61`
**Owner:** none
**Since P2.2's close (2026-11-27):** none; [S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md) is the stage that needs it

## What

**A query with no row does not say which of its premises matched nothing.** The Zebra puzzle's step 2 is solved in
the theory's relation, one state with every value placed in `same-house`, and its answer query, `(and (drinks
?who_water Water) (owns ?who_zebra Zebra))`, has no row: the read-out says *"the goal has no row: undetermined
here"*, and the summary's `queries[0].classes[0].rows` is `[]`. Nothing says that `(drinks ?who_water Water)`
matched nothing because no rule concludes `drinks` beyond the two sentences that state it, nor that `(owns ?who_zebra
Zebra)` matched nothing either. *Undetermined here* reads as an open question where the goal is unanswerable in
every state. The word problem *together*'s step 3, whose text does not give Alice's apples, says the same, and there
the premise is answered nowhere because a rule's premise, the receiver's count at the start, is stated by no
sentence.

## Evidence

`ein solve einaf/contract/zebra/step2.ein --single` and its `--json-summary`'s `queries`; `einaf feedback
einaf/contract/together/step3.ein --single`, whose `queries[0].classes[0].rows` is `[]`.

## What it costs the application

Between steps and in a repair, *your question's relation is never derived* is the first thing to say, and *this
rule's premise is stated by no sentence* the second. The feedback object has no part that says either, and EinAf
cannot compute it: `einaf::kernel` asks questions about R, not about a premise, and no value of the library says
which conjunct of a goal matched nothing.
