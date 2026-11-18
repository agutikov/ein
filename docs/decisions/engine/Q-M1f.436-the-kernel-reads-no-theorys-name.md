# Q-M1f.436 — The kernel reads no theory's name

The kernel, the loader, the saturation and the search, expects the minimal shape of a program, its relations and
rules, and gives a fixed meaning to its own names alone: those `ein_core::terms::Kernel` holds, which `Terms::new` interns before a program arrives, the words of
the grammar, and the three names `:over` takes (`ein_ir::expect::Over::NAMES`). Every other name, `is-a`, `type`,
`instance`, `symmetric`, `bijective`, `slot-partition` and a program's own, is a theory's or a program's, and means
what the rules that read it make it mean. The kernel mirrors no relation: a relation closed under swapping its two
arguments is `std.algebra`'s rule `(symmetric R)`, and `__symmetric__` is an ordinary name, which a program may bind
and a fact of which derives nothing. The relations every program has, beside those it writes, are the kernel's
computed `eq` and `neq`; a goal or a claim over `is-a` in a program that writes none is refused as one over any
relation the program lacks. No drawing reads a theory's name either
([Q-M1f.421](../cli/Q-M1f.421-no-view-draws-a-relation-otherwise-than-another.md)).

**Why.** A name such as `is-a` or `symmetric` belongs to one theory, and a kernel that reads it gives that theory's
reading to every program, which the program cannot change by writing other rules. Ein has no types
([Q-M1f.372](../language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)); a relation's properties are
facts that rules read, so the standard library states them and the kernel stays the same for every theory. The
native mirror was the one such reading built into the saturator, and `std.algebra`'s rule answers alike on every
program that used it.

**Held by.** `ein-render/tests/renaming_invariance.rs`: every corpus program, with every name that is not the
kernel's renamed by one order-preserving bijection, in its text, in every local module it imports and in the whole
standard library, gives the same load, saturation, plans, explanation, drawings and answer, its claims' verdicts
among them, apart from the names; per commit outside `tests/`, under `EIN_CORPUS_SLOW=1` every program.
`ein-infer/tests/stdlib_semantics.rs`' `the_kernel_mirrors_no_relation_and_symmetric_is_a_rule`;
`ein-ir/tests/engine_names.rs`' `a_name_the_kernel_does_not_read_binds_as_any_name_does`;
`ein-cli/tests/goal_premises.rs`' `a_goal_over_is_a_in_a_program_that_writes_none_is_refused`. The page is
[reserved engine strings](../../ein/ein.rs/07-reserved-engine-strings.md#no-theorys-name).
