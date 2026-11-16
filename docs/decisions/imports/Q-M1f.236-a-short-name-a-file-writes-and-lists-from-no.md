# Q-M1f.236 — A short name a file writes and lists from no import is its own

A short name a file writes and lists from none of its imports is the file's own name, whatever a module it imports
declares under the same short name: the file's `done` beside the calculator's is another relation, and nothing
warns. Two modules that declare one short name are imported together, each declaration under its own full name;
only a file that binds that short name to both is refused
([Q-M1f.234](Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)).

**Why.** A short name is local to its file, as a variable is local to its scope: a file's own name that shadows a
module's is a mistake the file can make, as shadowing is, and not a fault of the program's form. Full names keep
two modules' declarations apart, so neither reads the other's facts.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `a_short_name_is_local_to_its_file`, a name written and not
listed kept beside the module's; `what_prints`, a module's listed name and another file's own name of the same
spelling never printed as one; `ein-cli/tests/imports_by_full_name.rs`:
`a_programs_own_names_beside_the_calculator`.
