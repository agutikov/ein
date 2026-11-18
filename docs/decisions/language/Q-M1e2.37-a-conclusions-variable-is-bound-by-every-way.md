# Q-M1e2.37 — A conclusion's variable is bound by every way of satisfying the `:match`

Every variable a rule's or an hrule's conclusion writes must be bound by a premise or a parameter on every way of
satisfying its `:match`, each disjunct of an `(or …)` included. A conclusion variable nothing binds, `(rule r ()
:match (p ?x) :assert (q ?y))`, is refused at load, by every command, one line a variable. What it would mean, the
conclusion of every name, is written explicitly: a premise of any relation that binds the variable over the objects
the relation holds of, `(R ?y)`, or `(eq ?y ?y)`, which binds it over the program's names
([Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)). A parameter its activator binds is
bound, a nested fact among the objects it may be bound to. The refusal's line:
[Q-M1f.38](../diagnostics/Q-M1f.38-a-refusals-line-names-the-fault-then-what.md).

**Why.** Logically the implicit form concludes its fact of every name, which is almost never what was meant and is
most often a misspelt variable. Refusing it forbids nothing that can be meant, since the explicit spellings say the
same, and catches the slip where it is written.

**Held by.** `ein-cli/tests/cli_semantics.rs`: `a_conclusion_variable_nothing_binds_is_refused_at_load`, on every
command, and what loads beside it: a variable a relation premise, an `eq` guard or a parameter binds.