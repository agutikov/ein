# Q-M1f.280 — A program does not write `written`

A fact of `written`, a stored negation of one, and a rule's or an hrule's conclusion of one are refused at load, on
every command, one line each placed at the form: *(written (p a)) — written is the loader's: it states (written T)
for each fact nested in a fact a file writes, and a program does not write it*. `written` is reserved, so no
declarator binds it. A rule and a goal may read it.

**Why.** Only the loader states `written`, so it holds what the files wrote and is decided at root: a check that
reads it, a representation module's of its terms
([Q-M1f.272](Q-M1f.272-a-representation-module-reads-what-a-file-writes.md)), answers before any commitment. A fact
a program or a conclusion could add would be one more statement no file wrote, and one a commitment could extend.

**Held by.** `ein-cli/tests/written_facts.rs`: `a_program_does_not_write_written`: a fact, a negation, a rule's and
an hrule's conclusion, each refused on the nine commands it runs; a declaration refused; a rule and a goal that
read `written` answered. The reserved name: `ein_core::RESERVED`.
