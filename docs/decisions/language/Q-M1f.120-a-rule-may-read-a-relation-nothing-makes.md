# Q-M1f.120 — A rule may read a relation nothing makes

A rule's `:match` may read a relation that no declaration, fact or conclusion of the program makes. The program
loads, the rule never fires, and no surface reports it. A goal over such a relation loads too, and has no row
([Q-M1f.119](Q-M1f.119-a-goal-and-a-claim-ask-only-about-relations.md)).

**Why.** A library rule reads relations its importer supplies, and the stdlib's rules read marker relations whose
absence means only that the rule does not apply: `std.algebra`'s `derive-join` reads `(join …)`,
`symmetric-is-self-converse` reads `(symmetric …)`. An import brings its module whole, so refusing a rule over a
relation nothing makes would refuse every program that uses part of a module.

**Held by.** The loader's checks of relation names, `ein_ir::from_ir`'s `program_relations` and
`refuse_solve_names`, read goals, claims and `(solve …)` lists, and no rule's `:match`;
`ein-cli/tests/goal_premises.rs`: `a_goal_over_a_relation_of_the_program_loads`, whose
`an-imported-rules-conclusion` program imports `std.algebra` and states no `join`.
