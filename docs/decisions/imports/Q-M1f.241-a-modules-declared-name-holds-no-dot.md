# Q-M1f.241 — A module's declared name holds no dot

A name a module declares holds no dot. A module that declares one is refused at load, on every command:
`d.ein:2:11: d declares 'y.z', a name with a dot — a module's declared name holds no dot, so that a full name such
as d.x reads one way`. The main file may declare a dotted name, which is how a state file declares the full names
its program was loaded with ([Q-M1f.239](Q-M1f.239-a-program-declares-no-name-under-the-full-name.md)).

**Why.** A full name is the module's name, a dot per directory, and the short name. If module `word-theory` could
declare `gaps.x`, then `word-theory.gaps.x` would be the full name of two declarations, its own and module
`word-theory.gaps`'s `x`.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`.
