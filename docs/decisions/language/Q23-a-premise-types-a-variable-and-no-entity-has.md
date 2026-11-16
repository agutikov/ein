# Q23 — A premise types a variable, and no entity has a type slot

No variable, relation, rule or fact carries a type slot, and the kernel imposes no type system. A variable is typed
by the premises that bind it, `(is-a ?h House)`. A relation's declaration names a type for each argument, its
signature: the kernel reads its shape and never reads an atom as a type, and a rule reads its atoms as types
through the declaration's stored fact `(relation R A B)`; no form is checked against it. An object's type is a fact
of the program's own membership relation, such as `is-a`.

**Why.** Types are nodes of the graph and membership is an edge, so a rule reads, derives and explains a type as it
does any fact; a slot would be a second record of what a fact already says.

**Held by.** `ein-cli/tests/declared_counts.rs`: `what_is_not_a_declared_relations_form_loads`, the case
`not-a-type-check`; `tests/stdlib/typing/03_type_hierarchy_welltyped.ein`, a type hierarchy checked by rules. The
signature's two readings: `syntax.md` § Relation declarator.
