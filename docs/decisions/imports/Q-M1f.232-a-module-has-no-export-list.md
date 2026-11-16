# Q-M1f.232 — A module has no export list

A module says nothing of what it exports: there is no `(export …)` form and no entry list. Every name it declares
has a full name any importer may write, and an importer lists from it the short names it writes. A module does not
hide a name: what a theory promises to a program, and what is its own circuit, is its documentation's to say.

**Why.** A full name per declaration already keeps two modules' names apart
([Q-M1f.231](Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)), so an export list would add a second list
of the same names for every module to keep, and a state would still have to print the hidden names.

**Held by.** `ein-ir/tests/imports_semantics.rs`: `what_each_import_form_brings_of_a_module`, every declaration of
the module kept under each form, a name the program does not list under its full name;
`ein-ir/tests/imports_full_names.rs`: `a_short_name_is_local_to_its_file`.
