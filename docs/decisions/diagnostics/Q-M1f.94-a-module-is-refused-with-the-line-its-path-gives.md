# Q-M1f.94 — A module is refused with the line its path gives as a program

An imported module is read by the read every command makes of the program it is given, so a module that is a
directory, that cannot be read, or that is not UTF-8 is refused with the line its path would give as that program,
behind `kb load error: `: for a directory, `kb load error: IsADirectoryError: [Errno 21] Is a directory:
'm.ein'`. A module that does not exist has a line of its own, `(import m) — module not found at m.ein …`, which no
read reaches.

**Why.** One read, one set of lines: a module and a program on the same path are refused the same way, and a
change to how a read failure is spelt moves both.

**Held by.** `ein_ir::read::read_program`; `ein-cli/tests/unicode_only_in_a_string.rs`:
`an_imported_module_is_read_as_the_program_is`, each case on every command; `ein-cli/tests/program_read.rs`:
`every_command_refuses_a_file_that_is_not_utf8_with_one_line`.
