# Q-M1f.67 — A macro's body is checked where it is called, and an unused macro loads

A macro's body is a template, and what its kernel primitives mean depends on what a call puts in it, so a body is
not refused where it is defined: `(macro one (?a) (eq ?a))` loads, and a macro no rule, hrule, obligation or goal
calls loads whatever its body. What is required is that no path executes a malformed primitive: a call's expansion
is held to the grammar's shapes at load, before anything runs
([Q-M1f.80](Q-M1f.80-a-malformed-primitive-a-macros-expansion-writes.md),
[Q-M1f.81](Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)). A word the grammar refuses wherever it
stands, such as `open`, `_` or a string, is refused in a body whether it is called or not
([Q-M1f.86](Q-M1f.86-a-macros-body-never-writes-open.md)).

**Why.** A primitive's shape is decided by the call: the same body is well formed under one argument and malformed
under another. Refusing at the definition would refuse bodies no call makes malformed; refusing at the call refuses
exactly what would run.

**Held by.** `ein-cli/tests/macro_bodies.rs`: `an_unused_malformed_macro_loads`, every malformed macro called by no
rule answered by every command; `ein-cli/tests/goal_premises.rs`: `a_goal_over_a_relation_of_the_program_loads`,
whose `an-unused-macro` program loads.
