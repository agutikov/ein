# Q-M1f.42 — A conclusion whose relation is a parameter writes into every rule the program names

A conclusion written `(?T …)` writes into whichever relation its activator names. The load check of
[Q-M1f.1](Q-M1f.1-a-rule-that-could-derive-an-activator-binding.md) reads it as writing into every rule of the
conclusion's arity whose name the program writes as an argument: in a fact at any depth, in a rule's clause, or in
the `(solve …)` form. So `(rule copy (?S ?T) :match (?S ?x) :assert (?T ?x))` with `(copy item r)` is warned of
where `item`'s argument can be a fact and `r`'s parameter names a relation.

**Why.** A rule's name reaches a parameter only from a place the program writes it, and no hypothesis proposes a
rule's name, so every rule the conclusion can activate is among those the program writes: the reading is sound, and
it warns only of what a run could reach.

**Held by.** `ein_ir::from_ir`'s `warn_derived_relation_activators`, and its unit test
`a_derived_activator_binding_a_relation_parameter_to_a_fact_is_warned_of_at_load`, whose `copy` case is this shape.
