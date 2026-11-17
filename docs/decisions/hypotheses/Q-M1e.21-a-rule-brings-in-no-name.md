# Q-M1e.21 — A rule brings in no name

A constant a rule writes, in its `:assert` or its `:match`, is a name of the program, as a constant a fact writes at
any depth is ([Q-M1e2.9](../language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)). The guessable atoms
are seeded with every such name, `ein_infer::invariant::program_names`, beside the names index at root's fixpoint,
and then lose only what [Q-M1f.306](Q-M1f.306-the-guessable-atoms-are-the-programs-names-never.md) takes out. So a
model whose facts name a constant only a rule writes is in R, and the alive-set invariant, *no rule asserts a name
the program lacks*, holds of every program by construction.

**Why.** Rules are part of root. A name a rule could bring in mid-search would grow the candidates inside a fork, in
a part of the lattice H was not built for, and R would lose the model that names it.

**Held by.** `ein-infer/tests/alive_invariant.rs`: `every_name_of_the_program_is_a_candidate_object`,
`no_run_reaches_a_name_the_program_does_not_have`, `the_pair_answers_one_region`;
`ein_infer::hypgen::candidate_objects`, which reads `program_names`.
