# Q-M1f.247 — Every import brings its module whole

Every import brings every declaration and every fact of its module, with the module's own imports, all the way
down, one copy per program, every declared name under its full name. The three shapes differ only in the names the
file may write short: `(import M :symbols (…))` lets it write the listed names short, `(import M :as a)` writes
`a.name`, and a bare `(import M)` writes full names. A list selects nothing: no import leaves a rule, a fact or an
obligation of its module behind, and nothing follows a closure of names.

**Why.** A name a program writes means exactly what its full name means, however the module was imported: a
selection that brought part of a module could make a listed name mean less, and the same program would answer
differently under two import forms ([Q-M1f.249](Q-M1f.249-the-answer-does-not-depend-on-the-import-form.md)).

**Held by.** `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`, a case per form;
`a_selection_names_and_the_module_comes_whole`; `an_import_brings_its_module_whole_and_the_inlined_program_is_it`;
`ein-cli/tests/imports_by_full_name.rs`: `nothing_is_left_behind`. Names:
[Q-M1f.231](Q-M1f.231-every-name-a-module-declares-has-a-full-name.md).
