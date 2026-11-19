# Q-M1f.550 — The engine's word is `inconsistent`

The engine has one word a rule may state under `:ein`
([Q-M1f.399](Q-M1f.399-a-rule-states-facts-or-a-word-to-the-engine.md)): `inconsistent`. Where a rule that states it
fires, the saturation of the state ends and the state is dead. The firing is recorded with what it rests on, the
premises its match read, the absences it passed and what its universals read, as a derived fact's derivation is, so
a core is taken over it; no fact is written.

**Why.** *Inconsistent* is the property of a set of premises from which a contradiction follows, and a state is such
a set: its facts. *False* is the property of a formula in a model, and `(false)` keeps that word for the proposition.
Naming the engine's act by the property of the state keeps the two from being mixed, and reads as a statement about
the state where an imperative would read as a command to the search, which never sees the rule.

**Held by.** `ein_core::EngineWord`, the list of the engine's words, which the loader's refusal prints;
`ein_core::Kb::end` and `ein_core::Kb::ended`, the record of the firings that ended a state;
`ein_infer::contradiction::detect`, which reads them; `ein_infer::saturator`'s
`a_rule_that_states_the_key_ends_the_state_and_writes_no_fact` and
`a_stopped_saturation_stops_at_the_firing_that_ends_the_state`.
