# Q-M1f.121 — A goal's relation variable is refused with the goal's line at every depth

A pattern in a query's `:goal` whose relation is a variable is refused at load, at the top of the goal, under
`and`, `or` and `absent`, and inside another pattern at any depth, with one line: `query: its :goal writes (?R ?g)
with ?R as its relation, which nothing binds — a relation is named by a name, or by a rule's parameter its
activator binds, and a query has no activator`.

**Why.** One shape in one clause reads one way at every depth. A rule's line for the same shape speaks of the
rule's parameters, and a query has none.

**Held by.** `ein-cli/tests/goal_premises.rs`: `a_goal_a_rule_would_be_refused_for_is_refused_on_every_command`,
the variable at the top, inside a pattern, under `absent` and two deep; `ein-ir`'s
`from_ir::tests::the_compilers_refusals_a_program_reached_are_refused_at_load`.
