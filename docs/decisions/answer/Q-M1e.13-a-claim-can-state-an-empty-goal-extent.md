# Q-M1e.13 — A claim can state an empty goal extent

An answer in which the goal's relation has no fact is a claim like any other:

- **A query's claim** lists projection classes, and `(model)` is the class of the states where the goal has no
  row. `(query :goal (p ?x) :expect (model))` holds of a program whose states hold no fact of `p`, and `(or
  (model) (model (p A)))` of one where some do and some do not. The classes compare as a set, so a claim fails
  when the answer gains a class and when it loses one: a claim of two classes refutes an answer of one.
- **A region claim** lists states, and a relation it names is closed in every model it lists, so states that hold
  facts of different relations are stated together: `(solve :expect (region (model (q A)) (model (p A) (p B))))`
  says that the first state holds no `p` and the second no `q`.

A query's claim lists only the goal's atoms. A fact of another relation in it is refused at load, with the
reason: `:expect lists (q A), which no atom of the query's :goal matches — a query's claim lists its projection
classes; a claim about the state is (solve … :expect (region …))'s`.

**Why.** A claim is what turns a wrong answer into a failed exit code, so an answer no claim can state is one no
test can hold against a change. The goal's having no row is an answer, often the one a fixture is about, and the
empty class states it without inventing a fact.

**Held by.** `ein-infer/tests/expect_semantics.rs`: `an_undetermined_goal_is_the_empty_class`,
`the_classes_and_their_facts_compare_as_sets`, `over_selects_the_states_and_a_missing_class_is_named` and
`an_extra_class_is_named`; `ein-infer/tests/claim_round_trip.rs`: `every_printed_region_holds_as_its_own_claim`,
every corpus region printed as a claim and held as one. The page: [`syntax.md`](../../ein/user/reference/syntax.md) § The expected
region and § Query. That a relation is empty: [Q-M1c.6](Q-M1c.6-a-claim-can-say-that-a-relation-is-empty.md).
