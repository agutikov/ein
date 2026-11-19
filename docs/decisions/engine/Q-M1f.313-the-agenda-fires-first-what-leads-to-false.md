# Q-M1f.313 — The agenda fires first what leads to `(false)`

Every saturation, root's, each route's node's, the oracle's and every reader's, fires before every other candidate
the application of a rule that states `:ein inconsistent`, which ends the state
([Q-M1f.399](../language/Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md)), and of a rule that refutes a
fact beside one holding it alone, `std.contradiction`'s `clash-N`, so a state ends as soon as it holds the pair, and
ranks the rest in two
halves: first what leads to the end of a state, a rule concluding a fact such a rule matches, or concluding either
side of a pair a rule refutes, a fact beside a fact that holds it as its one argument; then the rest. Within each half the order is FIFO, the written and arrival
order. The boundary walks its parked candidates in the same order, and a resumed saturation keeps it.

What such a rule matches is read off the rules, with no name: in every program it is `(false)`, which the rule every
program gets matches ([Q-M1f.551](../stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)), so a rule that
concludes `(false)` is among the first, and with it a rule that concludes what a program's own such rule matches.
A pair is read off the rules as well: a rule whose match is a fact and one fact holding it, and whose firing ends a
state, `std.contradiction`'s `clash-N` where a program imports it
([Q-M1f.557](../stdlib/Q-M1f.557-the-contradiction-is-a-theory-a-program-imports.md)). Under it a rule that
concludes the holding side, a stored negation, is among the first, and so is one that concludes a fact of a
relation the holding side of which a rule concludes or the state holds. A program with no such rule has no such
pair, and a stored negation is a fact of the second half. A
rule that states the key fires first under the written order alone too: a route's node stops at the firing that
ends its state, one firing after the one that concluded what it matched.

**Why.** On a monotone program the fixpoint does not depend on the order, so the order decides no answer, and
firing first what leads to the end of a state finds a dead node's contradiction sooner. Of a program the static
check flags as not monotone, the engine claims nothing under any order.

**Held by.** `ein_infer::saturator::AGENDA`, the one order every saturation takes, and
`ein_infer::monotone::leads_to_an_end` and `ein_infer::contradiction::Pairs`, what it reads of the rules;
`ein_infer::saturator`'s `a_fail_first_agenda_moves_the_order_and_not_the_fixpoint`, `ein_infer::saturator`'s
`a_rule_that_states_the_key_ends_the_state_and_writes_no_fact` and
`a_stopped_saturation_stops_at_the_firing_that_ends_the_state`; `ein-infer/tests/agenda.rs`:
`every_question_and_claim_answers_alike_under_the_agenda_and_the_written_order`, which holds every question and
every claim of every monotone corpus program alike under this order and under the written order alone. A core is an
explanation whatever the order: [Q-M1f.6](../answer/Q-M1f.6-a-core-is-an-explanation.md).
