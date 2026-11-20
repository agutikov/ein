# Q-M1f.130 — A macro's call in a conclusion is expanded

A macro is called where the loader expands it: a rule's `:match` and `:assert`, at any depth, and a
query's `:goal`. A call in a conclusion is replaced by the macro's body as a premise's is, so `:assert (settled
?g)` beside `(macro settled (?g) (paid ?g))` stores `(paid Ann)`, and `:assert (says (settled ?g))` stores `(says
(paid Ann))`. The expansion is held to what a written conclusion is held to, and one that is no conclusion is
refused at load ([Q-M1f.81](Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)). Nowhere else is a form
headed by a macro's name a call.

**Why.** A macro's body is checked where a rule is, on its expansion, so a conclusion's call reads as the rule its
expansion writes, with no second meaning for a macro's name in a conclusion.

**Held by.** `ein-cli/tests/macro_facts.rs`: `a_macro_called_where_it_is_expanded_loads`, a call in a rule's
`:assert`, inside a conclusion and inside a hypothesis, `(open (settled ?g))`, each storing what the body spells and no fact of the
macro's name.
