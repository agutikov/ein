# Q-M1f.40 — No firing meets an unbound variable

A firing builds its conclusion only from registers its match bound. The loader refuses a conclusion variable that
some way of satisfying the `:match` leaves unbound
([Q-M1e2.37](../language/Q-M1e2.37-a-conclusions-variable-is-bound-by-every-way.md)), and the compiler seeds every
parameter from its activator, a nested fact among the objects it binds, so no state's saturation, at root or under
a commitment, meets a conclusion it cannot build. A rule's firing and a hypothesis rule's are alike, and so is the
plan of a rule no state ever fires. A conclusion that read an unbound register would be a defect of the engine, not
an answer: `ein_infer::firing::FireError` has no variant for it, and the firing asserts it.

**Why.** A run that stopped at a firing would answer on a command that never compiled the rule and stop on one that
did, and a hypothesis rule's template would propose nothing in silence. Held at load, the refusal reaches every
command alike ([Q-M1f.84](Q-M1f.84-a-compiler-arm-the-loader-covers-is-an-invariant.md)); what the compiler builds
must then agree with what the loader read as bound.

**Held by.** `ein-ir/tests/fuzz_properties.rs`: `no_firing_meets_an_unbound_variable`, every plan the compiler
builds, one per activator, for each rule and hypothesis rule of every generated program the loader accepts, read for
a conclusion register no disjunct binds; `ein_infer::firing::resolve`'s assertion;
`ein-infer/tests/compile_semantics.rs`: `a_value_parameter_an_activator_binds_to_a_nested_fact_is_bound`;
`ein-cli/tests/cli_semantics.rs`: `a_conclusion_variable_nothing_binds_is_refused_at_load`, which runs
`examples/ein-bugs/wrong-answers/parameter-nested-assert.ein`, a parameter bound to a nested fact, through
`saturate`, `solve`, `solve --minimal` and `oracle`, each answering.
