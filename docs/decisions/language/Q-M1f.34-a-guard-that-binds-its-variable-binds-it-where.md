# Q-M1f.34 — A guard that binds its variable binds it where it is written

A guard whose variable a relation premise of its conjunction binds waits for that premise, whatever the order. A
guard whose variable nothing else in the conjunction binds binds it itself, over the program's names, and does so
where it is written, as a relation premise in its place would. So an `(absent …)` written after it reads the
variable, and one written before it does not: `(and (neq ?x a) (absent (q ?x)))` is *the names other than `a` with
no `q`*, and `(and (absent (q ?x)) (neq ?x a))` is *the names other than `a`, if there is no `q` at all*.

**Why.** A binding guard is a premise that binds, the relation it is
([Q-M1e2.26](Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)), so the order rule an `(absent …)`
already has, that it reads the variables the premises before it bind, is the only one needed; a guard moved past an
`(absent …)` would bind where a relation premise in its place could not.

**Held by.** `ein-infer/tests/rule_semantics.rs`: `a_binding_guard_binds_where_it_is_written`, both orders.
