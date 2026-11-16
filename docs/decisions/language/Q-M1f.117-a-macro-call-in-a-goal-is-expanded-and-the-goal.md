# Q-M1f.117 — A macro call in a goal is expanded, and the goal is printed as written

A query's `:goal` is a premise, and a macro call in it is expanded at load as a rule's `:match` is:
`(unknown (paid ?g))` is `(and (absent (paid ?g)) (absent (not (paid ?g))))`. The expansion is what is matched,
what a claim's facts are held to and what a class lists, an `(and …)` inside an `(and …)` read flat. The read-out's
`query` row, the JSON summary's `goal` and `ein test`'s report print the goal as the program wrote it, and the
read-out's `as :expect` row shows what it means.

**Why.** A goal is compiled as a rule's `:match` is, so a call means in a goal what it means in a rule. The author
reads back the question as asked, and the row below it shows the claim the expansion supports.

**Held by.** `ein_ir::from_ir`'s `expand_goal`, and `ein_core::Query`'s `goal_written`, the goal as written where
the loader expanded one; `ein-cli/tests/goal_premises.rs`: `a_goal_is_answered_as_a_rules_match_would_be`, each
program answered with its goal printed as written.
