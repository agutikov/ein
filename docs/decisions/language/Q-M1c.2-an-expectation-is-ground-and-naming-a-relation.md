# Q-M1c.2 — An expectation is ground, and naming a relation closes its held extent

A claim is an answer, never a pattern: its facts are ground, and a variable in one is refused at the parse. Its
vocabulary is the answer's own: `(model <fact>*)`, several under `(or …)`, compared as a set in any order with the
facts in each compared as a set, and `(false)`, the kernel's ⊥, for no answer at all. A claim says which facts
hold, never which rule derived them.

- In a query's `:expect`, each model is one class of the goal's rows over the states the query asks about, `(model)`
  the class with no row, and the claim lists every class.
- In the `(solve …)` form's `:expect`, each model is a state of the region, root's facts included, and **naming a
  relation closes its held extent**: every relation the claim names positively is listed at its complete held extent
  in every model, every fact of it that the state holds, and a relation it never names is unconstrained. Stored
  negatives are not closed: a listed `(not …)` is checked for presence like any other fact.

**Why.** A per-fact assertion cannot catch a surplus fact, and a whole-state copy pins every fact no test means to
assert; closing the held extent of exactly the relations named sits between the two. A variable in a claim would
match whatever the engine derived, a test that cannot fail.

**Held by.** `ein-infer/tests/claim_semantics.rs`: `naming_a_relation_closes_it_in_every_model`,
`a_listed_negative_is_checked_for_presence`, `the_empty_model_is_root_and_not_a_wildcard`,
`false_is_every_node_dead`. `ein-infer/tests/expect_semantics.rs`:
`the_classes_over_all_of_r_hold_when_listed_whole`, `the_classes_and_their_facts_compare_as_sets`,
`false_is_no_class`. The grammar, `ein-ir/src/ein.y` (`Claim`, `ClaimFact`, `RegionClaim`), and its table,
`ein-ir/tests/grammar_pages.rs`.
