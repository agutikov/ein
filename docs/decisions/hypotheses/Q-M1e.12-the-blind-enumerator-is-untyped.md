# Q-M1e.12 — The blind enumerator is untyped

The blind enumerator commits to no type system: it fills every argument's place of every relation whose held extent
is not closed with every name of the program that no declaration writes as a label. An atom a declaration writes as
a label is not guessable. So a program whose declaration admits in an argument's place an atom that an `is-a` fact
points at, `(relation co-located T T)` beside `(is-a Red Color)`, is under-specified, and Ein answers it as written:
`(co-located Blue Color)` is a candidate like `(co-located Blue H5)`, and a state of R may hold it. `is-a` is a
relation like any other, and nothing that reports the universe reads it by its name: the program with the relation
spelt otherwise prints the same, name for name.

**Why.** An atom that stands on the right of an `is-a` is an atom like any other. A rule that excluded such atoms
would be a type system arriving in the kernel, and a report that named them would read one convention of membership
by its name where programs write others, `type` and `instance` among them. The program's way out is its declaration.

**Held by.** `ein_infer::hypgen::candidate_objects`, which leaves out a name a declaration writes as a label, and its
test `candidate_objects_drop_signature_atoms_and_primitives`; `ein-cli/tests/read_out.rs`:
`is_a_reads_as_any_other_relation_on_every_surface`.
