# Q29 — A rule compiles once per activator

A rule is not interpreted: it is compiled to a plan, once for each activator that binds its parameters, and the
plan is cached under its compile key, the rule's name and every argument of the activator. Compiling binds the
parameters before any match, so the relation names an activator supplies are written into the plan, and a
saturation runs the plans its activators have. A rule with no parameters has one plan; an activator a rule derives
at run time, in root or in a node, gets its plan when it arrives.

**Why.** An activator binds a rule's parameters once, and the plan is a pure function of that binding; a matcher
that walked the rule's tree would rediscover the binding at every match. The compiled form lives in the engine and
not in the store, so the store depends on nothing the engine computes.

**Held by.** `ein_infer::compile::PlanKey` and `plan_key`; `ein_infer::Engine::compile_all`;
`ein-infer/tests/explain_semantics.rs`: `a_runtime_derived_activator_gets_a_plan_on_a_fork`;
`ein-infer/tests/rule_semantics.rs`: `an_empty_match_fires_once_or_once_per_activator`. The key's arguments are a
firing's identity too: [Q-M1e.16](Q-M1e.16-a-firings-identity-keys-on-every-activator.md).
