# Q-M1f.93 — A module's read line names the path its import resolves to

When an imported module cannot be read, the line names the path the import resolves to, `<the importing file's
directory>/<module>.ein`, relative where the program was given relatively: `ein solve diag/main.ein` names
`diag/m.ein`. It is the path `module not found at …` names for a module that is missing. A module that is read and
then refused by the parser names its canonical path, as every position in it does.

**Why.** The read is of the path the import names, and the line is the one that path prints when handed to a
command as the program.

**Held by.** `ein-cli/tests/unicode_only_in_a_string.rs`: `an_imported_module_is_read_as_the_program_is`, a
directory's, an unreadable file's and a missing module's lines naming the relative path, a parse error the
canonical one.
