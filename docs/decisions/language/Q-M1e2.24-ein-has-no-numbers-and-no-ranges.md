# Q-M1e2.24 — Ein has no numbers and no ranges

Ein has no numbers, no ranges and no int kind, and none is planned. A run of name characters is a name, digits
included: `1` and `01` are two names, `-7` is a name, and `1..5` is the name it spells. A name of digits is a
candidate like any other name. A name is spelt from an allow-list of ASCII characters, letters, digits, the
connectors `_ - .` and the decorators, and stays ASCII; `\` is never one.

**Why.** No operation of the engine reads a value as a quantity: its only relations of values are `eq` and `neq`,
fixed by name identity. Arithmetic is a theory of the stdlib, computed by saturation over numerals written in digit
names, so a kernel number would be a second way to say what a rule already says, and one no rule could explain.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_run_of_digits_is_a_name_in_every_place`, `1`, `01`, `7`, `-7`
and `07` five names in every place a name is written; `a_range_is_the_name_it_spells`. The characters of a name:
the grammar, `ein-ir/src/ein.l`, and its table, `ein-ir/tests/grammar_pages.rs`.
