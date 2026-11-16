# Q-M1f.251 — A prefixed name that names nothing its module declares is refused

A prefixed name, by an alias, `x.nope` under `(import m :as x)`, or by a full name, `m.nope` beside an import of
`m`, is a full name of a declaration of the module it names. One that names nothing the module declares is refused
at load, on every command: `'x.nope' names nothing: m declares no 'nope' — a prefixed name is a full name of a
declaration of the module it names`. So `i.n` under `(import std.math.int :as i)` is refused: `n` is
`std.math.decimal`'s.

**Why.** A prefix says whose name it is; a name the prefix does not hold would otherwise be a fresh name of the
program, answered by silence, where the file meant the module's.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`, by alias and by full name.
