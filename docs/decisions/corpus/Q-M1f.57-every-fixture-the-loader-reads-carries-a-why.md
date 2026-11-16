# Q-M1f.57 — Every fixture the loader reads carries a `:why`

Every `.ein` file of the project that the loader reads carries a `:why` on each relation, rule, hrule and macro it
declares, as every program must under the default options. A fixture the loader refuses carries them too, each
`:why` on its form's last line, so that no line or column its refusal prints moves. A fixture refused at the parse
carries none: the parse stops before the loader reads a `:why`, and one written there would be read by nothing and
would move the positions the refusal prints.

**Why.** A fixture that loads only with the requirement off tests a program no user can write without the flag; and
a fixture refused at load must be refused for the fault it is about, not for a missing `:why`.

**Held by.** `ein-ir/tests/load_semantics.rs`: `the_load_negative_fixtures_are_byte_identical`, every refused
fixture's message against its `.expected`, and `the_default_requires_a_why_and_false_loads_without`;
`ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code`, every corpus run under the default options.
The option that loads a program without a `:why`:
[Q-M1f.62](../cli/Q-M1f.62-no-require-why-loads-a-program-without-a-why.md).
