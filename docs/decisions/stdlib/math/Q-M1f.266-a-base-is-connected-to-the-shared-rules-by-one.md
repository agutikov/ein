# Q-M1f.266 — A base is connected to the shared rules by one call of a macro

The rules every positional base shares are `std.math.positional`'s, each relation they name a parameter. A base
connects them by one call of the module's macro, `(positional operand operand-is n …)`, its own relations in the
macro's order, inside its own closing rule's conclusion; the call stands for one activator a shared rule, derived
in the firing that closes the base. The base's module is the file that combines: a program imports
`std.math.decimal-arith` or `std.math.binary-arith`, never `std.math.positional`.

**Why.** A rule with parameters fires for a fact of its own name, so each shared rule needs an activator, and a
macro's call is the one hand-written form that stands for all of them. Nothing uses a base's relations without the
shared rules, so a file apart that combined them would buy nothing.

**Held by.** `ein-infer/tests/arith_theory.rs`: `one_call_of_the_macro_connects_each_base`; the closing rules
`decimal-arith` and `binary-arith`.
