# Q-M1f.226 — An obligation's name is no candidate object

An obligation rule is a rule, and the KB's names index holds its name as it holds every rule's and every relation's.
So a rule that names an obligation as a value, as a module's closing rule names each rule of the module, seeds no new
name, and the blind enumerator, which leaves out every name the registry calls a rule, proposes nothing about it.

**Why.** One category, read one way: were an obligation's name outside the index, the same word would be an object
where a rule's name is not, and the names of the program and the enumerator's objects would part on it.

**Held by.** `ein-core/src/kb.rs`, where the names index unions the registry's relations, rules and obligations;
`ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`, which holds every name the
program seeds to the enumerator's objects, with no exception.
