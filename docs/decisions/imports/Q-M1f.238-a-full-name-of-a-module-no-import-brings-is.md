# Q-M1f.238 — A full name of a module no import brings is refused

A file may write any name by its full name, but only of a module an import of the program brings. A full name of
a module no import brings is refused at load, on every command, naming the module and what to write:
`'std.math.decimal-arith.add' is a full name of std.math.decimal-arith, which no import of the program brings — a
module enters a program by an import of it: write (import std.math.decimal-arith …)`. Writing a full name imports
nothing.

**Why.** An import is the one way a module enters a program, so what a program holds can be read off its imports.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`, with an import of another module and with none;
`ein-cli/tests/imports_by_full_name.rs`: `a_refusal_is_one_line`, the line's form on every command.
