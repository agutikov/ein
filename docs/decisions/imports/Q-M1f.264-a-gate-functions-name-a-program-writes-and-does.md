# Q-M1f.264 — A gate function's name a program writes and does not list is its own

Each of the sixteen functions of two inputs is declared by `std.bool.gates` as a relation, so its name has one full
name, `std.bool.gates.Xor`, as the relation and as the value in the module's table. A program that writes a
function's name, as a gate's function too, `(gate g1 Xor a b o)`, lists it, `(import std.bool.gates :symbols (Xor
And))`, or writes it in full or by an alias. A name it writes and does not list is its own, as every short name is
([Q-M1f.236](Q-M1f.236-a-short-name-a-file-writes-and-lists-from-no.md)): a name no row of the table holds, so a
gate of that function computes nothing, and nothing says so. No rule of `std.bool.circuit` guards against it, and
no loader line warns.

**Why.** A short name binds one way in every position, a relation's and a value's alike; a module that guarded its
own names against a program's would be a second rule for what the name rule already decides. The module's header
and its page say that a program lists every function's name it writes.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `a_short_name_is_local_to_its_file`;
`ein-infer/tests/bool_theory.rs`: `the_modules_write_no_negation`, no `absent` in any rule of the four modules, so
no guard; `stdlib/bool/gates.ein`'s header and `stdlib/README.md`, which state the rule where a program's author
reads first.
