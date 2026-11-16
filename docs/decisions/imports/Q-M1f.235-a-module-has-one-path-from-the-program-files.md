# Q-M1f.235 — A module has one path, from the program file's directory

A module's name is its path. `std.x.y` is the standard library's, found under its one root; any other module's name
is a path from the program file's directory, a dot per directory, so `(import word-theory.gaps :as gaps)` reads
`word-theory/gaps.ein` beside the program, and that module's names are `word-theory.gaps.gaps` and so on. A
module's own imports are found from the program file's directory too, never from the module's own. No name reaches
outside that directory: `..` is refused, and a standard-library module imports only `std.*` modules. So every
module has one name and one path, and its full names read one way.

**Why.** A full name is the module's name and the short name, the same through every path, which holds only if one
module cannot be reached under two names. Links to the same file under two paths are outside what the resolver
reads.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `a_module_is_found_from_the_programs_directory`, and
`the_refusals`, `../m` among the names refused; `ein-ir/tests/imports_semantics.rs`:
`module_names_mangle_into_these_file_paths`, the file each name reads, banked in
`ein-ir/tests/golden/module_paths.txt`, and `a_stdlib_module_may_not_import_a_file_relative_one`. The spelling of a
name: [Q-M1f.252](Q-M1f.252-a-modules-name-has-no-empty-segment-and-no-slash.md).
