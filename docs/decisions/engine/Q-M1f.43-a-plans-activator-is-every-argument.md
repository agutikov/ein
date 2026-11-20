# Q-M1f.43 — A plan's activator is every argument of its activator fact

`Plan::activator_args` holds every argument of the fact the plan was compiled from, as objects (the engine's
`Value`), the compile key's list; each argument binds its parameter, a nested fact included. It is what the
`enqueue`, `fire` and `owe` events print as the activator, what the static checks print as a label, and a plan's
identity wherever an analysis keys on one: the obligations' owed instances, the region's, the detector's root
plans and the NAF dependencies.

**Why.** One list for a plan's activator everywhere: two plans that differ only in a nested fact bind two objects,
and every key that reads the list tells them apart, as the compile key and the binding key do.

**Held by.** `ein_infer::plan::Plan::activator_args`; `ein-infer/tests/compile_semantics.rs`:
`a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound`; the binding key that shares the list:
[Q-M1e.16](Q-M1e.16-a-firings-identity-keys-on-every-activator.md).
