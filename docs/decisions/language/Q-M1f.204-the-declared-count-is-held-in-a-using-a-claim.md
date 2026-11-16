# Q-M1f.204 — The declared count is held in a `:using`, a claim and a called macro's body

Beyond a fact, a rule and a goal, the declared count
([Q-M1f.315](Q-M1f.315-a-form-of-a-declared-relation-has-as-many.md)) is held in:

- **a fact's `:using`**, whose premises are facts, written as a nested fact is;
- **a fact a claim lists**, a query's `:expect` and a `(solve … :expect …)`'s; the claim's own words, `model`,
  `region`, `merged`, `owing` and `or`, are its shape and no relation's form;
- **a macro's body where a rule or a goal calls it**, on the expansion; a macro nothing calls is not read.

A `(solve …)` list is not read: `:hrules (guess (at Color House))` and `:hypothesis-relations (likes fan)` are
lists of names, and an `:hrules` activator is held to its hrule's parameters
([Q-M1f.115](Q-M1f.115-an-hrules-list-names-declared-hrules.md)).

**Why.** Each held place writes a fact or a pattern of one, which a wrong count leaves matching nothing; a list of
names writes no form.

**Held by.** `ein-cli/tests/declared_counts.rs`: `a_form_of_another_count_is_refused_on_every_command`, the rows
`a-facts-using`, `a-queries-claim`, `a-region-claim`, `a-macros-body-where-a-rule-calls-it` and
`a-macros-body-where-a-goal-calls-it`; `what_is_not_a_declared_relations_form_loads`, the cases `a-solve-list`,
`a-solve-list-of-relations` and `an-unused-macro`.
