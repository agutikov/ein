# Q-M1f.1 — A rule that could derive an activator binding a relation parameter to a fact is warned of at load, and such an activator activates nothing

A relation parameter names a relation, and a fact names none. An activator that binds a rule's relation parameter
to a fact is held as the fact it is and activates nothing: no plan of the rule is compiled for it, and the rule
never fires on it. A program in which some rule could derive such an activator loads, and the load says so whether
or not any run derives it: one `warning:` line on stderr, naming the deriving rule, the conclusion, the activated
rule and its parameter, by every command that loads the program. Where a state does derive one, the run says so too,
one `warning:` line placed at the rule, naming it and the activator. Where the activator is written rather than
derived, the load's line is [Q-M1e2.18](Q-M1e2.18-a-keyword-pair-is-no-guards-argument-and-a-fact.md)'s. Which
positions can hold a fact is read off the program's text
([Q-M1f.41](Q-M1f.41-a-position-holds-a-fact-unless-the-program.md),
[Q-M1f.42](Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)). No answer and no exit code moves for a warning.

**Why.** Nothing is refused at load for what a rule could derive. A pattern whose relation is a fact matches
nothing, and a conclusion whose relation is a fact is no object of the knowledge base, so the one reading that
compiles nothing it cannot compile is that the activator activates nothing; and a run that stopped where such a plan
is compiled would stop one command and let another, which never derives it, answer. The load's warning is read off
the program, so it is the same on every command and every run; the run's names what a state did derive.

**Held by.** `ein_ir::from_ir`'s `warn_derived_relation_activators`; `ein_infer::activators_for`, which hands no
plan's driver such an activator, and `ein_infer::compile::note_inert_activators`, which records the run's line.
`ein-cli/tests/cli_semantics.rs`: `a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load`,
on every command, and `ein-ir/src/from_ir.rs`' unit test of the same name;
`ein-infer/tests/compile_semantics.rs`: `a_relation_parameter_a_rule_binds_to_a_nested_fact_activates_nothing`;
`examples/features/19_derived_activator.ein` and `20_derived_activator_never.ein`.
