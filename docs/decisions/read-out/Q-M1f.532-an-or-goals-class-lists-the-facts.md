# Q-M1f.532 — An `(or …)` goal's class lists the facts of the disjuncts that hold

The class of a query whose goal is an `(or …)` lists, under each binding row, the instances of the patterns of the
disjunct that holds under it, each disjunct by its own bindings, and nothing else: no `(or …)` form, and no instance of
a disjunct that does not hold. Asked of `zebra2`, `(or (symmetric ?r) (bijective ?r) (transitive ?r))` has the class
`(model (bijective color-loc) … (symmetric next-to) (transitive is-a*))`, never `(symmetric color-loc)`. Its `as
:expect` is a claim the query states and the loader takes back; a fact of `relation` the goal reads, `(relation
next-to)`, is one a claim may list, as a goal may ask about it
([Q-M1f.119](../language/Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)). A goal with no `(or …)` is read
as before.

**Why.** A class is the facts a state gives the goal, which a claim lists; an `(or …)` is no fact a state holds, and
a class that wrote one stated disjuncts no state holds and could not be pasted back as the query's claim.

**Held by.** `ein_infer::read_out`, each match's disjunct (`ein_infer::verdict::goal_matches`); `ein_ir::expect`'s
`goal_disjuncts` and `goal_atoms`, which a claim's facts are held to; `ein-cli/tests/goal_premises.rs`:
`a-disjunctions-class` and `a-disjunctions-class-as-its-claim`. The pages are
[queries](../../ein/user/03-syntax-and-semantics/12-query.md#in-mathematics) and
[the read-out](../../ein/user/04-reference/08-read-out.md).
