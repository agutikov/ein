# Q-M1f.31 — `(not (eq x y))` is `(neq x y)` in every place

The negation of an `eq` fact is computed, as `eq`'s facts are: `(not (eq x y))` reads as `(neq x y)` wherever the
grammar takes it. In a premise and in a goal it is the `neq` guard; in a conclusion it is checked as `neq`'s is, a
true one adding nothing and a false one a contradiction; a written one is a fact of `neq`, checked where it stands
([Q-M1f.33](Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md)). It is never a stored negation, and no state
holds one.

**Why.** `eq` and `neq` are relations whose truth is computed by name identity
([Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)), so the negation of one is computed
too. Read as a stored negation it would match only stored facts, which no state holds, and a stored `(not (eq a
a))` would sit in a state and contradict nothing: one relation with two readings.

**Held by.** The grammar, `ein-ir/src/ein.y` (`Negated`). `ein-infer/tests/rule_semantics.rs`:
`not_eq_and_a_relation_bound_to_eq_are_the_guards`, `a_written_eq_fact_is_checked_never_stored`.
