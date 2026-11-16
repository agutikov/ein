# Q-M1f.319 — No module reserves its names

A module has no way to keep a program from stating facts of its relations. A program that lists a module's relation
may write facts of it, a theory's results and its circuit's relations as well as its requests: written beside the
calculator, `(sum-of (n z 2) (n z 2) (n z 5))` loads, and the theory computes with it as with a derived fact, an
answer nobody computed. A file that lists a module's name and declares it too is refused
([Q-M1f.234](Q-M1f.234-one-short-name-bound-to-two-full-names-in-one.md)); a written fact of it is not. Which
relations a program states and which it reads is the theory's page to say: for `std.math.decimal-arith`, a program
states the requests and `let`.

**Why.** An imported module is part of the program, and Ein has one kind of fact: a fact the main file writes is a
fact like one a module writes or a rule derives. A reservation would be a second kind, a fact some file may not
write.

**Held by.** `ein-ir/tests/imports_semantics.rs`: `a_diamond_of_flat_imports_yields_each_fact_once`, a program's
own statement of a fact of a module's relation kept; `stdlib/README.md`, the relations each theory of the
calculator leaves to a program.
