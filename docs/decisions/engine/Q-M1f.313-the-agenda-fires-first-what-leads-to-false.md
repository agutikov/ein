# Q-M1f.313 — The agenda fires first what leads to `(false)`

Every saturation, root's, each route's node's, the oracle's and every reader's, fires before every other candidate
the application of a rule that states `:ein inconsistent`, which ends the state
([Q-M1f.399](../language/Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md)), and ranks the rest in two
halves: first what leads to the end of a state, a rule concluding a fact such a rule matches, a stored negation, or a
fact whose negation can be concluded; then the rest. Within each half the order is FIFO, the written and arrival
order. The boundary walks its parked candidates in the same order, and a resumed saturation keeps it.

What such a rule matches is read off the rules, with no name: in every program it is `(false)`, which the rule every
program gets matches ([Q-M1f.551](../stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)), so a rule that
concludes `(false)` is among the first, and with it a rule that concludes what a program's own such rule matches. A
rule that states the key fires first under the written order alone too: a route's node stops at the firing that
ends its state, one firing after the one that concluded what it matched.

**Why.** On a monotone program the fixpoint does not depend on the order, so the order decides no answer, and
firing first what leads to the end of a state finds a dead node's contradiction sooner. Of a program the static
check flags as not monotone, the engine claims nothing under any order.

**Held by.** `ein_infer::saturator::AGENDA`, the one order every saturation takes, and
`ein_infer::monotone::leads_to_an_end`, what it reads of the rules; `ein_infer::saturator`'s
`a_rule_that_states_the_key_ends_the_state_and_writes_no_fact` and
`a_stopped_saturation_stops_at_the_firing_that_ends_the_state`; `ein-infer/tests/agenda.rs`:
`every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order`, which holds every question and
every claim of every monotone corpus program alike under this order and under the written order alone. A core is an
explanation whatever the order: [Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md).
