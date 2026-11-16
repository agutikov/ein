# Q-M1e2.26 — `eq` and `neq` are the relations of equality and non-equality

`eq` and `neq` are relations whose facts are fixed by name identity, computed and never stored. They read the same
in every place a relation is written:

- **In a goal**, a query over them is answered: `(query :goal (neq w ?y))` gives a row for every name of the
  program but `w`, and `(eq w ?y)` binds `?y` to `w`. A claim may name either.
- **In a premise**, a rule's `:match` means the same in any order of its conjuncts: a guard whose variable a
  relation premise of the conjunction binds waits for it, and one whose variable nothing else binds ranges over the
  program's names ([Q-M1e2.9](Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)). A fact that holds no
  variable is a value a guard may compare.
- **In a conclusion**, a rule may conclude either: a true one adds nothing, and a false one is a contradiction, as
  `(false)` is.

`_` and a keyword pair stay no guard's argument ([Q-M1e2.22](Q-M1e2.22-a-bare-is-reserved-and-refused.md),
[Q-M1e2.18](Q-M1e2.18-a-keyword-pair-is-no-guards-argument-and-a-fact.md)).

**Why.** A guard looks like a fact of a relation and means one, so it answers as one: refusing a query over it, or
giving a premise a meaning that depends on where it is written, would be a restriction with no logical reason. A
conclusion of `eq` or `neq` changes nothing a guard computes, and a false one is a contradiction like any other.

**Held by.** `ein-infer/tests/rule_semantics.rs`: `eq_admits_exactly_the_bindings_neq_rejects`,
`a_guards_free_variable_ranges_over_the_programs_names`, `an_eq_or_neq_conclusion_is_checked_never_stored`;
`ein-cli/tests/cli_semantics.rs`: `a_goals_empty_row_says_whether_the_goal_has_a_variable`, which answers `(neq w
?y)` with a row for each name. The rest: [Q-M1f.30](Q-M1f.30-a-guard-argument-holds-no-variable-inside-a-fact.md),
[Q-M1f.31](Q-M1f.31-not-eq-x-y-is-neq-x-y-in-every-place.md),
[Q-M1f.33](Q-M1f.33-a-written-fact-of-eq-is-a-conclusion-with-no.md),
[Q-M1f.34](Q-M1f.34-a-guard-that-binds-its-variable-binds-it-where.md).
