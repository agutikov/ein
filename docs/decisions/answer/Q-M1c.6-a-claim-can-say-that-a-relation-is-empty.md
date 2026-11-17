# Q-M1c.6 — A claim can say that a state holds no fact of a relation

That the held extent of a relation is empty, that a state holds no fact of it, is said in one of two ways.

- **A query's claim**, over every state the query reads: `(query :goal (r ?x) :expect (model))`. A query's claim
  lists its projection classes, and `(model)` is the class of the states where the goal has no row; listed alone, it
  says that no state read holds a fact of `r`. A state that does fails the claim, naming the class it found.
- **A region claim**, of one state: a relation a `(solve … :expect (region …))` names in any of its models is listed
  at its complete held extent in every model, so a model that lists no fact of it says that the state holds none. A
  relation the claim names nowhere is not constrained by it.

**Why.** Naming a relation is what closes its held extent, and a relation is named by a fact of it, so an empty held
extent needs a form that names a relation without a fact. The query names it by its goal, which is what a reader
asks about; the region claim names it by another state that holds one.

**Held by.** `ein-infer/tests/expect_semantics.rs`: `an_undetermined_goal_is_the_empty_class`, and
`over_selects_the_states_and_a_missing_class_is_named` for the failing direction. The page:
[`02-syntax.md`](../../ein/user/04-reference/02-syntax.md) § The expected region and § Query. An answer whose goal
extent is empty: [Q-M1e.13](Q-M1e.13-a-claim-can-state-an-empty-goal-extent.md).
