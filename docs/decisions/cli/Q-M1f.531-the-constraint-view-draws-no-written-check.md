# Q-M1f.531 — The constraint view draws no written check

A check a program writes where a fact stands, `(eq a w)` or `(not (eq a a))`, is no fact a knowledge base holds,
`eq` and `neq` being computed and never stored, and `ein render constraints` draws a knowledge base
([Q-M1f.530](Q-M1f.530-ein-render-constraints-draws-roots-facts-about.md)): no written check is drawn, as `ein render
program` draws none.

**Why.** A drawing of a check as a fact draws a fact no state holds, and two views of one program would disagree
about what it states.

**Held by.** `ein-render/tests/every_object_once.rs`: `the_constraint_view_draws_no_written_check`;
`ein-cli/tests/render_doors.rs`: `render_constraints_draws_roots_facts_about_relations`, on
`examples/ein-bugs/wrong-answers/fact-eq-false.ein` and `not-eq-fact.ein`.
