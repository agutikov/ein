# Q-M1f.113 — A value is a name, or a fact a state could hold

A value is a name, or a fact a state could hold: an atom of an ordinary relation, or its negation. A fact inside a
fact, a pattern inside a pattern, and a conclusion's or a guard's argument are values. So `eq`, `neq`, `absent`,
`and`, `or`, and `false` with an argument, never head a nested form; `not` does, as the stored negation of a fact.
The readings of the rule:

- `absent` heads a premise and a claim's class atom, and nothing else
  ([Q-M1f.122](Q-M1f.122-absent-heads-a-premise-and-a-claims-class-atom.md));
- `(false)` with no argument is a value ([Q-M1f.123](Q-M1f.123-false-with-no-argument-is-a-value.md));
- `(eq a b)` is written only as a form of its own
  ([Q-M1f.124](Q-M1f.124-eq-a-b-is-written-only-as-a-form-of-its-own.md));
- a connective as a pattern's argument is refused after expansion
  ([Q-M1f.125](Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md));
- a relation parameter bound to one of these words is held to the same rule
  ([Q-M1f.126](Q-M1f.126-a-relation-parameter-bound-to-absent-false-or-eq.md));
- `not` of a name is the negation of no fact
  ([Q-M1f.127](Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md),
  [Q-M1f.128](Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)).

**Why.** A value is what a state may hold or a rule may match by shape. An equality is computed and never stored, a
negation as failure is a test of a state, and a connective joins premises: none is a fact any state holds, so a
value headed by one could be matched only by its spelling, deriving what no state supports.

**Held by.** The grammar, `ein-ir/src/ein.y`'s `FactHead`, `ValueHead`, `NestedFact`, `Premise`, `Pattern` and
`Negated`, with a note on each refusal, and the loader for what only expansion shows;
`ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command` and
`what_a_state_could_hold_loads`; `ein-ir/tests/grammar_pages.rs`, a row of the grammar's table per place.
