# Q-M1f.93 — A module's read line names the path its import resolves to

When an imported module cannot be read, is missing, does not parse, or is an end of an import cycle, the line names
it by the path the import resolves to, the program's directory as it was given joined with the module's name as
written, relative where the program was given relatively: `ein solve diag/main.ein` names `diag/m.ein`, and every
position in the module names the same path. It is the path `module not found at …` names for a module that is
missing. A `std.*` module is named under the standard library's root as it was found. No path is made absolute or
resolved further: a module is told from another by its name, not by its path.

**Why.** The read is of the path the import names, and the line is the one that path prints when handed to a
command as the program. A path the program did not give carries the working directory into a message, a place and
a summary, so that one program run from two directories would say two things.

**Held by.** `ein-cli/tests/unicode_only_in_a_string.rs`: `an_imported_module_is_read_as_the_program_is`, a
directory's, an unreadable file's, a missing module's and a parse error's lines naming the relative path;
`ein-cli/tests/module_paths.rs`: `a_module_is_named_as_the_program_names_it`, every refused corpus program and every
one that imports a module by its file-relative name, named three ways from two working directories.
