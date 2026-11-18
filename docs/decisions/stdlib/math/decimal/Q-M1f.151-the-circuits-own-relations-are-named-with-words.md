# Q-M1f.151 — The circuit's own relations are named with words

The relations a computation's steps are facts of are named with words, `add-run`, `strip`, `turn`, `done`,
`short`, `operand`, `add-first`, and the module's header lists them as the circuit's own: in the state for a query
or a `--trace` to see, and no promise. Each has a full name, `std.math.decimal-arith.strip`, which a state prints
where the program gives it no spelling of its own, so a program may declare a `done` or a `b` of its own beside the
calculator ([Q-M1f.231](../../../imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)).

**Why.** A trace and a state print these names, and a word reads as the step it is. A clash of names is the
import mechanism's to resolve, by full names, and not each module's by a prefix.

**Held by.** `ein-cli/tests/imports_by_full_name.rs`: `a_programs_own_names_beside_the_calculator`.
