# Q-M1e2.9 — A program's names are the constants its facts and rules write

The names of a program are every constant written in an argument position of a fact, at any depth, a nested fact's
arguments included, and every constant written anywhere in a rule, its `:match` and its `:assert`. Each is an atom a
hypothesis may be about, a candidate for the guessable atoms
([Q-M1f.306](../hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md)). A constant written only in
a `(query …)` is not a name of the program.

**Why.** The region R is the program's, and a query asks about it: adding a query never changes R. A name a model
needs that the candidates lacked would be a wrong answer, so every name the program's facts and rules write is one.

**Held by.** `ein_infer::invariant::program_names`, the one list of the program's names, which reads no query.
`ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`,
`no_run_reaches_a_name_the_program_does_not_have`; `ein-infer/tests/rule_semantics.rs`:
`a_guards_free_variable_ranges_over_the_programs_names`.
