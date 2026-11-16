# Q-M1f.131 — No relation is declared under a macro's name

A `(relation settled …)` beside `(macro settled (?g) (paid ?g))` is refused at load, on every command, one line at
the declaration: *relation 'settled': settled is a macro too, (macro settled (?g) (paid ?g)) — a premise (settled
…) is the macro's call, replaced by its body, and a fact (settled …) is refused, so nothing could be of the
relation; give one of the two another name*.

**Why.** A premise headed by the name is the macro's call and a fact headed by it is refused, so no fact could be
of the relation and no premise could read one. A relation nothing can hold is a mistake in a name, and the
declaration is where its author reads about it.

**Held by.** `ein-cli/tests/macro_facts.rs`: `a_macros_call_written_as_a_fact_is_refused_on_every_command`, the row
`a-relation-of-a-macros-name`. The code: `ein_ir::from_ir`'s `refuse_macro_facts`.
