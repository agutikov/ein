# Q-M1f.279 — A `written` fact is derived by the loader's step `#written`

Each `(written T)` is derived by the loader's step `#written` (`ein_core::terms::WRITTEN_STEP`, a name the loader
makes up), from the fact it was read in, its one premise, or from no fact where a rule's text writes it. So a core
that rests on one names the fact as written, and `refuted by` names the rule that read it; one stated from a rule's
text rests on no fact, and its core is ∅. It shows where a declaration's mirror fact shows: `ein saturate --dump`
lists and counts it among the derived facts, `(written (n z 1) :rule "#written")`, a KB view draws it and a `.einb`
stores it. A `--dump-answer` state file, `ein render`'s KB text, `-P`'s blocks, a `(model …)`'s comparison and the
read-out leave it out, since the loader states it again from the facts a file holds.

**Why.** A derivation with a premise is what every walk of an explanation already follows, so a core reaches the
written fact with no case of its own.

**Held by.** `ein-cli/tests/written_facts.rs`: `a_core_names_the_fact_as_written`, both cores;
`ein-cli/tests/answer_round_trip.rs`, which reloads every corpus program's dumped states and finds their `written`
facts stated again.
