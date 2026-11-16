# Q-M1f.138 — An imported fact keeps its own provenance

An imported fact's provenance is the one its module writes: its `:source` where the module writes one, and
otherwise none, a given as if the program had written it. No import stamps a fact with the name of the module it
came from.

**Why.** An imported fact is a fact of the program, and a program answers as the same program with the module
written in place. A provenance says what a written fact claims to rest on, and a module's name claims nothing.

**Held by.** `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`, a fact with a
`:source` kept as the module writes it under each form; `ein-cli/tests/import_facts.rs`:
`a_flat_import_of_facts_answers_as_the_facts_written_in_place_on_every_command`, `ein render kb` and `ein saturate
--dump` among the commands compared.
