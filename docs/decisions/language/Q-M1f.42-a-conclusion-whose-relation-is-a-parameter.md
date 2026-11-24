# Q-M1f.42 — A conclusion whose relation is a parameter writes into every rule the program names

A conclusion written `(?T …)` writes into whichever relation its activator names. The load's reading of which
positions hold relation names ([Q-M1f.41](Q-M1f.41-a-position-holds-a-fact-unless-the-program.md)) reads it as
writing into every rule of the conclusion's arity whose name the program writes as an argument: in a fact at any
depth, in a rule's clause, or in the `(solve …)` form. So `(rule copy (?T) :match (marker ?x) :assert (?T ?x))`
with `(copy r)` passes what `marker`'s argument holds to `r`'s parameter, and where that parameter names a
relation, `marker`'s argument holds relation names.

**Why.** A rule's name reaches a parameter only from a place the program writes it, and no hypothesis proposes a
rule's name, so every rule the conclusion can activate is among those the program writes: the reading is sound, and
it reads only what a run could reach.

**Held by.** `ein_ir::from_ir`'s `read_relation_places`, and its unit test
`a_derived_activator_binding_a_relation_parameter_to_a_fact_loads`, whose `copy` case is this shape: the name a
marker holds is a relation of the program.
