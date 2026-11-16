# Q-M1f.231 — Every name a module declares has a full name, and a file lists the short names it writes

Every name a module declares, a relation, a rule, an hrule or a macro, has one full name: the module's name, a dot,
and the name, `std.math.decimal-arith.add`. A full name is the same whoever imports the module and through whatever
path, so a module is one copy per program. A short name is an alias local to the file that binds it: a module's own
file writes its declarations short, and an importer writes short only the names it lists, `(import M :symbols (add
sum-of))`, or prefixes with its alias, `(import M :as a)` and `a.add`. A file lists every short name it writes from
the module that declares it; nothing is re-exported, so a module's own imports give its importer no short name.

**Why.** Theories that share a name are imported together without one reading the other's facts, since the two
names are two full names; and a program still writes the theory it uses in its own short words. One spelling per
declaration across every path is what makes a module one copy rather than one per import.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_users_dependency_graph`, a graph of modules imported
through several paths, one copy each; `a_short_name_is_local_to_its_file`, no re-export;
`ein-ir/tests/imports_semantics.rs`: `one_module_under_two_spellings_is_one_copy`;
`ein-cli/tests/imports_by_full_name.rs`: `the_calculator_by_the_names_a_program_writes` and
`a_programs_own_names_beside_the_calculator`. What the forms bring:
[Q-M1f.247](Q-M1f.247-every-import-brings-its-module-whole.md).
