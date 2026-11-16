# Q-M1f.1 — A rule that could derive an activator binding a relation parameter to a fact is refused at load

A relation parameter names a relation, and a fact names none. A program in which some rule could derive an
activator that binds a rule's relation parameter to a fact is refused at load, whether or not any run derives it:
one line, naming the deriving rule, the conclusion, the activated rule and its parameter, by every command that
loads the program. Where the activator is written rather than derived, the refusal is
[Q-M1e2.18](Q-M1e2.18-a-keyword-pair-is-no-guards-argument-and-a-fact.md)'s. Which positions can hold a fact is
read off the program's text ([Q-M1f.41](Q-M1f.41-a-position-holds-a-fact-unless-the-program.md),
[Q-M1f.42](Q-M1f.42-a-conclusion-whose-relation-is-a-parameter.md)).

**Why.** A rule whose bad conclusion fires only on some runs would stop those runs where its plan is compiled and
answer the others, so two commands, or two orders of one search, could disagree. A refusal read off the program is
the same on every command and every run.

**Held by.** `ein_ir::from_ir`'s `refuse_derived_relation_activators`. `ein-cli/tests/cli_semantics.rs`:
`a_derived_activator_binding_a_relation_parameter_to_a_fact_is_refused_at_load`, on every command, and
`ein-ir/src/from_ir.rs`' unit test of the same name; `examples/broken/load/derived_activator.ein` and
`derived_activator_never.ein`.
