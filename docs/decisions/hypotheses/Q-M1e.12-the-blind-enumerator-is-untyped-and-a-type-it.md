# Q-M1e.12 — The blind enumerator is untyped, and a type it binds is named

The blind enumerator commits to no type system: it fills every slot of every open relation with every name of the
program that no relation signature declares. A name a signature declares as a type is no candidate object. So a
program whose signature admits a type in a slot, `(relation co-located T T)` beside `(is-a Color T)`, is
under-specified, and Ein answers it as written: `(co-located Blue Color)` is a candidate like `(co-located Blue H5)`.
Where the blind enumerator built H and a candidate names the second argument of an `is-a` fact, every question prints
a `types` row under `universe`, and the summary carries `root.hypgen.types_as_objects`. The row decides nothing: H, R,
every count and every exit code are what they are without it.

**Why.** A type is an object that happens to stand on the right of an `is-a`; a rule that excluded types would be a
type system arriving in the kernel. The program's way out is its signature, and the row tells its author which
relation admits the binding.

**Held by.** `ein-cli/tests/read_out.rs`: `a_type_bound_as_an_object_is_one_sentence_on_every_surface`;
`ein_infer::types_as_objects`; `ein_infer::hypgen::candidate_objects`, which leaves out a name a signature declares.
