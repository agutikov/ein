# Q-M1f.91 — An imported module is Ein text, and a `.einb` container is refused as one

A module an import names is read as Ein text, through the same read every command makes of the program it is
given. A module whose file is a `.einb` container is refused at load, on every command, with a line of its own:
`kb load error: m.ein: a .einb container, and an imported module is Ein text: import the .ein it was saved from`.

**Why.** A `.einb` is a private cache of one program, never an interchange format: what a program shares with
another is its text. The resolver also stands below the crate that opens a container, so a module it reads is
text by construction.

**Held by.** `ein_ir::read::container_module_line`, the line; `ein-cli/tests/unicode_only_in_a_string.rs`:
`an_imported_module_is_read_as_the_program_is`, a module that is a container among the ways a module is read, on
every command.
