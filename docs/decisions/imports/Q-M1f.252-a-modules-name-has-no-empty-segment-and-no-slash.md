# Q-M1f.252 — A module's name has no empty segment and no slash

A module's name is segments joined by dots, a name per directory and the file's, none empty and none holding a `/`.
`(import ..m)`, `(import m.)`, `(import a/b)` and `(import ../m)` are each refused at load, on every command, by a
line that says so, *a module's name is its path from the program's directory, a name per directory joined by dots,
none empty and none holding a '/': no '..', no path*. Nothing is dropped or normalised.

**Why.** A module's name is its identity: its declarations' full names are built from it. Normalised, `..succ` and
`succ` would be one file under two names, and a `/` would let a name reach outside the program's directory
([Q-M1f.235](Q-M1f.235-a-module-has-one-path-from-the-program-files.md)).

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`, the four names;
`ein-ir/tests/imports_semantics.rs`: `module_names_mangle_into_these_file_paths`, the doubled and trailing dots
among its rows.
