# Q-M1f.128 — A firing that meets the negation of a name concludes nothing for it

Where the loader does not read a binding
([Q-M1f.127](Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)), such as `(rule deny (?R) :match (?R
?g) :assert (not ?g))` activated through a rule that derives `(deny paid)`, a run may meet `(not ?g)` with `?g` a
name. There is no fact to conclude: the firing stores nothing for that conclusion and stores the rule's other
conclusions. No command stops, no `(false)` is concluded, and no state holds
the negation of a name.

**Why.** `not` of a name is the negation of no fact
([Q-M1f.113](Q-M1f.113-an-object-is-an-atom-or-a-fact-a-state-could.md)), so there is nothing to state, as a true
`eq` conclusion adds nothing. Concluding `(false)` would kill a state for a conclusion that says nothing, and a
stop would make an answer exit non-zero for a binding only some states meet.

**Held by.** `ein_infer::firing::build_fact`, the one place every conclusion is built, which returns no fact for
the negation of a name; `ein-cli/tests/negated_names.rs`: `a_name_only_a_run_meets_is_not_negated`, each program
storing and proposing no such negation, `ein solve` and `ein oracle` counting the same states, as text and through
a `.einb`.
