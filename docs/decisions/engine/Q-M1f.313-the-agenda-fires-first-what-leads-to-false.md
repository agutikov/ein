# Q-M1f.313 — The agenda fires first what leads to `(false)`

Every saturation, root's, each route's node's, the oracle's and every reader's, ranks its candidates in two
halves: first what leads to `(false)`, a rule concluding `(false)`, a stored negation, or a fact whose negation can
be concluded; then the rest. Within each half the order is FIFO, the written and arrival order. The boundary walks
its parked candidates in the same order, and a resumed saturation keeps it.

**Why.** On a monotone program the fixpoint does not depend on the order, so the order decides no answer, and
firing first what leads to `(false)` finds a dead node's contradiction sooner. Of a program the static check flags
as not monotone, the engine claims nothing under any order.

**Held by.** `ein_infer::saturator::AGENDA`, the one order every saturation takes; `ein-infer/tests/agenda.rs`:
`every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order`, which holds every question and
every claim of every monotone corpus program alike under this order and under the written order alone. A core is an
explanation whatever the order: [Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md).
