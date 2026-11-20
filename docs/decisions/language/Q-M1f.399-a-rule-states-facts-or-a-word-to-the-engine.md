# Q-M1f.399 — A rule states facts, or a word to the engine

A rule states what it concludes under `:assert`, facts, or under `:ein`, one word addressed to the engine. The word is
no fact: no knowledge base holds it, no rule matches it, and it stands under no arrow. Where a rule that states it
fires, the engine acts: the application writes no fact. No name is reserved for the engine and no prefix: `(false)`
stays the word a program writes for a contradiction, a proposition a rule concludes, a program states or another fact
holds under an arrow, and what makes a state that holds it dead is a rule from it to the engine's word
([Q-M1f.551](../stdlib/Q-M1f.551-std-core-is-loaded-into-every-program.md)).

A rule states one or the other, never both. The grammar takes
one name after `:ein`; the loader holds it to the engine's words
([Q-M1f.550](Q-M1f.550-the-engines-word-is-inconsistent.md)) and refuses any other with a line that lists them, so a
misspelt word does not load in silence. A program's own rule may state the word, from whatever it matches, with no
`(false)` between.

**Why.** What ends a state is an act of the engine, not a proposition. A fact that carried it would be a fact a rule
could match, a state could hold under an arrow and a program could state, and each of those would read as a command
where a proposition stands. A key of its own keeps the two apart: what a program says is facts, and what it asks of
the engine is written where no fact goes.

**Held by.** `ein-ir/src/ein.y`'s `RuleClause`, the one production that takes `:ein`; `ein_ir::from_ir`'s
`engine_word`, with `a_rule_states_a_word_to_the_engine_in_the_place_of_an_assert`;
`ein_core::EngineWord` and `ein_core::Rule::ein`; `ein_infer::firing::end`, which records the application and writes
no fact, and `ein_infer::saturator`'s tests of it; `ein-ir/tests/grammar_pages.rs`, a row of the grammar's table per
shape the parse refuses; `examples/features/18_ein_inconsistent.ein`, and the two refusals under
`examples/broken/load/`, `ein_word_unknown` and `ein_beside_assert`.
