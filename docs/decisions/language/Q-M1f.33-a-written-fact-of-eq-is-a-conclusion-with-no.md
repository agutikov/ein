# Q-M1f.33 — A written fact of `eq` is a conclusion with no premise

A top-level fact `(eq a b)`, or its negation `(not (eq a b))`, is read as a rule's conclusion with no premise: a
true one is not stored, and a false one is `(false)`, written where it stands, so root is a contradiction refuted
by `(given)`. `eq`'s facts are never stored.

**Why.** A fact is a rule with no premise, and a rule's false conclusion of `eq` is a contradiction
([Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)); a stored `(eq a b)` would change a
relation whose facts are fixed by name identity, and no guard would read it.

**Held by.** `ein-infer/tests/rule_semantics.rs`: `a_written_eq_fact_is_checked_never_stored`. A written fact of
`eq` at another arity is refused at load: `ein-cli/tests/cli_semantics.rs`:
`eq_at_another_arity_is_refused_at_load`.
