# Q23 — A premise says what a variable ranges over, and nothing has a type slot

No atom, variable, relation, rule or fact carries a type slot, and the kernel imposes no type system. What a
variable ranges over is said by the premises that bind it, `(is-a ?h House)`. A relation's declaration names a label
for each argument: the kernel reads how many there are and never reads a label as a type, and a rule reads the
labels through the declaration's stored fact `(relation R A B)`; no form is checked against it. What an atom is
classified under is a fact of the program's own membership relation, such as `is-a`.

**Why.** A label and a class are atoms of the graph and membership is a fact, so a rule reads, derives and explains
what a thing is as it does any fact; a slot would be a second record of what a fact already says.

**Held by.** `ein-cli/tests/declared_counts.rs`: `what_is_not_a_declared_relations_form_loads`, the case
`not-a-type-check`; `tests/stdlib/typing/03_type_hierarchy_welltyped.ein`, a hierarchy of labels checked by rules.
The declaration's two readings: `02-syntax.md` § Relation declarator.
