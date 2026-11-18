# Q-M1f.127 — A conclusion `(not ?x)` is refused where the program shows that `?x` can hold a name

A conclusion `(not ?x)` is of a variable bound to a fact; a name has no negation. The loader refuses the
conclusion, one line at the variable, wherever the program's own text shows that `?x` can hold a name: where, in
one disjunct of the `:match`, every relation premise that binds `?x` binds it at a position that holds a name. A
position holds a name where the program shows one there:

- a written fact, at any depth and under a `(not …)`;
- a `(relation …)` declaration, matched as a fact;
- a rule's or an hrule's conclusion that writes a name, or a variable that can hold one, to a fixpoint;
- the rule's own activator, or an hrule's in `(solve :hrules …)`;
- a declared relation the hypotheses are taken from: no hrule declared, the `(solve …)` lists leaving it in, and
  nothing writing the closure marker;
- a guard that alone binds the variable.

A position that holds both a name and a fact is refused. A pattern whose relation is a parameter, `(?R ?x)`, is
read as one of each relation a written activator names there, and is not read where the activator is derived
through a variable; what the loader does not read is the firing's
([Q-M1f.128](Q-M1f.128-a-firing-that-meets-the-negation-of-a-name.md)).

**Why.** Each source is one the program's text shows without running it, so the refusal is decided at load on every
command. Reading less would load a program whose states past root negate names.

**Held by.** `ein_ir::from_ir`'s `refuse_negated_names`; `ein-ir`'s
`from_ir::tests::a_conclusion_negating_a_name_is_refused_at_load`; `ein-cli/tests/negated_names.rs`:
`a_conclusion_negating_a_name_is_refused_on_every_command` and `the_negation_of_a_bound_fact_loads`.
