# Q-M1e2.27 — In Ein's JSON, `[]` is the empty set and `null` what does not exist

On every field of every JSON surface, `[]` is a set known to be empty, and `null` something that does not exist or
that the run does not state. A core is one thing, the set of facts a dead node's contradiction rests on, so a dead
node's `core` and `refuted_by` are lists, and a node that is not dead, whose core does not exist, has `null`: in the
summary's root and minimal holes, `nogoods.json`, `region.json`, `nodes.json` and `--events`' `enter` lines alike.
`core: []` is a core that exists and holds no fact, *the empty set suffices*, as the dead root of a rule `:match ()
:assert (false)` has.

**Why.** An empty core and no core are two answers: the first says the refutation rests on nothing, the second
that there is no refutation. Written alike, a reader could not tell an alive node from one dead for no reason.

**Held by.** `ein-cli/tests/answer_round_trip.rs`: `no_core_unless_dead`, over every corpus program's dump and the
oracle's; `ein-infer/tests/corpus_cores.rs`: `a_rule_with_no_premise_refutes_root_with_the_empty_core`. The rule:
[08-read-out.md](../../ein/user/04-reference/08-read-out.md#11---json-summary-the-answer-blocks) § 11. A text surface writes the empty core `∅`
([Q-M1f.48](Q-M1f.48-a-text-surface-writes-the-empty-core.md)).
