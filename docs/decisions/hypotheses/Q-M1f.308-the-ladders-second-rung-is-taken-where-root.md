# Q-M1f.308 — The ladder's second rung is taken where root activates an obligation

Hypothesis generation is a ladder, decided once at root by one function that the generator and
`ein_infer::hypgen::eligible_relations` both read: the program's `(hrule …)` if it declares one; else, where root's
activators compile a plan of an obligation rule (one with no parameters always), the obligations rung; else the blind
enumerator.
`(solve :no-search)` runs none. An obligation rule the program loads and nothing activates chooses no rung, so a module
imported for another of its names leaves the program's own open relations to the blind enumerator. A root that owes
nothing under an activated obligation proposes nothing, and is complete because it is discharged.

**Why.** Where a program states an obligation, a state is judged by whether it is paid, and that judgement needs a
rung that proposes nothing once nothing is owed. Whether a program states one is a matter of what root activates:
every import brings its module whole, and importing one name must not change what is guessed.

**Held by.** `ein-cli/tests/import_invariance.rs`: `a_module_imported_for_one_rule_takes_the_rung_of_the_rule_written_out`;
`ein-infer/tests/obligation_rung.rs`: `the_ladder_dispatches_by_what_the_program_declares`,
`a_discharged_state_owes_nothing_and_declines_nothing`; `ein_infer::hypgen`, its `rung`;
[`solution_semantics.md` § 1.4](../../kernel/inference/solution_semantics.md).
