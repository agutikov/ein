# Q-M1e.12 — The blind enumerator is untyped, and an atom on the right of an `is-a` that it guesses about is named

The blind enumerator commits to no type system: it fills every argument's place of every relation whose held extent
is not closed with every name of the program that no declaration writes as a label. An atom a declaration writes as
a label is not guessable. So a program whose declaration admits in an argument's place an atom that an `is-a` fact
points at, `(relation co-located T T)` beside `(is-a Color T)`, is under-specified, and Ein answers it as written:
`(co-located Blue Color)` is a candidate like `(co-located Blue H5)`. Where the blind enumerator built H and a
candidate names the second argument of an `is-a` fact, every question prints a `types` row under `universe`, and the
summary carries `root.hypgen.types_as_objects`; *type* and *object* are the read-out's words there, for such an atom
and for a guessable atom. The row decides nothing: H, R, every count and every exit code are what they are without
it.

**Why.** An atom that stands on the right of an `is-a` is an atom like any other; a rule that excluded such atoms
would be a type system arriving in the kernel. The program's way out is its declaration, and the row tells its
author which relation admits the binding.

**Held by.** `ein-cli/tests/read_out.rs`: `a_type_bound_as_an_object_is_one_sentence_on_every_surface`;
`ein_infer::types_as_objects`; `ein_infer::hypgen::candidate_objects`, which leaves out a name a declaration writes
as a label.
