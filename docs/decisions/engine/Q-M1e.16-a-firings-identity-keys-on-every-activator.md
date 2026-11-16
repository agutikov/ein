# Q-M1e.16 — A firing's identity keys on every activator argument

A rule application is identified by its rule, its activator and the values of its register file, and the activator
half is the compile key's list: every argument of the activator fact, as values, names and nested facts alike. Two
plans of one rule therefore share an identity only where they share a compile key, and so a register layout; two
activators that differ anywhere, a nested fact included, each fire.

**Why.** An identity that compared two register layouts as one would drop a valid application as a repeat of
another, and lose a derivation depending on which activator was written first. With the compile key and the binding
key one list, each plan has its own space and the order activators are written in decides nothing.

**Held by.** `ein_infer::firing::BindingKey` and `ein_infer::compile::PlanKey`; `Engine::check_layout`, a debug
build's guard that the two keys have not parted; `ein-infer/tests/rule_semantics.rs`:
`an_int_beside_a_nested_fact_in_one_position_loses_no_derivation`,
`activators_differing_only_by_a_nested_fact_argument_both_fire`. A plan's activator, as events print it:
[Q-M1f.43](Q-M1f.43-a-plans-activator-is-every-argument.md).
