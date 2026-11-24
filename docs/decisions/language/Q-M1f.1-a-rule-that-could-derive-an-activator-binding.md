# Q-M1f.1 — A rule that could derive an activator binding a relation parameter to a fact loads, and such an activator activates nothing in silence

A relation parameter names a relation, and a fact names none. An activator that binds a rule's relation parameter
to a fact is held as the fact it is and activates nothing: no plan of the rule is compiled for it, and the rule
never fires on it, whether the program writes it or a state derives it. A program that writes one, or in which a
rule could derive one, loads, and nothing says so: no command prints a line of it at the load or at a run, as none
prints one of an activator of another count
([Q-M1f.115](Q-M1f.115-an-activator-is-a-fact-of-its-rules-name-and-one.md)). No answer and no exit code moves.

**Why.** A relation's name is an argument a program writes, `(imply1 equivalence symmetric)` among them, and a
program may import rules it never activates, so an activator that names no relation is no fault of the program,
and a line read off the program's text for one would be printed of programs that mean what they say. Nothing is
refused for it either: a pattern whose relation is a fact matches nothing, and a conclusion whose relation is a fact
is no object of the knowledge base, so the one reading that compiles nothing it cannot compile is that the activator
activates nothing; and a run that stopped where such a plan is compiled would stop one command and let another,
which never derives it, answer.

**Held by.** `ein_infer::activators_for`, which hands no plan's driver such an activator.
`ein-cli/tests/cli_semantics.rs`: `an_activator_binding_a_relation_parameter_to_a_fact_activates_nothing_in_silence`,
on every command, and `a_rule_shape_that_reads_nothing_is_refused_at_load`, over the written activator of
`examples/ein-bugs/wrong-answers/nested-fact-activator.ein`; `ein-ir/src/from_ir.rs`' unit test
`a_derived_activator_binding_a_relation_parameter_to_a_fact_loads`;
`ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`;
`examples/features/19_derived_activator.ein` and `20_derived_activator_never.ein`.
