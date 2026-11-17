# Q-M1f.250 — A listed name a module states facts of names the relation those facts are of

A file may list from a module a name the module declares or states a fact of. A listed name is a name only, since
the module comes whole whatever is listed: one the module states facts of and does not declare is bound to the
relation those facts are of, as the module's own text reads it. A module that imports `circuit` and states `(gate
…)` facts lets its importer list `gate` and mean `circuit`'s `gate`; a relation no file declares, `likes`, is one
name everywhere. A listed name the module neither declares nor states a fact of, one it writes only in a rule's
conclusion or only as an argument, is refused: `(import M :symbols …) — not provided by the module: thing`.

**Why.** A library of facts may be of another module's relation or of none, and a program that imports it writes
those facts' relation by the name the library writes; binding the name as the module reads it keeps one relation
for both.

**Held by.** `ein_ir::imports`, the resolver's binding of a listed name; `ein-ir/tests/imports_semantics.rs`:
`what_each_import_form_brings_of_a_module`, an undeclared relation listed and names that are not provided refused.
