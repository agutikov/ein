# Q-M1f.253 — Under an alias, a name prints as the main file spells it

Under `(import std.math.decimal-arith :as a)` the main file writes `a.sum-of`, and that spelling is its name for
the declaration: every surface prints `a.sum-of`. Where the main file also lists the name, the listed short name
prints; where it aliases one module twice, the first alias's spelling. A module that imports under an alias of its
own gives the program no spelling, and its names print in full.

**Why.** A program's facts print as the program writes them
([Q-M1f.237](Q-M1f.237-a-name-prints-as-the-main-file-writes-it-else.md)), and an alias is how that program writes
them.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `what_prints`, a listed name and an aliased one in one program;
`ein-ir/tests/imports_semantics.rs`: `one_module_under_two_spellings_is_one_copy` and
`no_import_brings_a_modules_query_solve_or_config`, the first of two aliases and a module's own alias.
